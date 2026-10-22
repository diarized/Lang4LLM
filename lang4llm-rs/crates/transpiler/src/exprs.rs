//! Expression code generation.

use lang4llm_ast::{
    BinOp, BinaryExpr, Expr, FieldAccess, Literal, ParamName, RangeOp, Type, UnOp,
};

use crate::builtins::{emit_builtin_call, is_builtin};
use crate::context::CodegenContext;
use crate::patterns::emit_pattern;
use crate::types::emit_type;

/// Emit an expression as Rust source.
pub fn emit_expr(expr: &Expr, ctx: &mut CodegenContext) {
    match expr {
        Expr::Literal(lit, _) => emit_literal(lit, ctx),
        Expr::Variable(ident, _) => ctx.write(&ident.name),
        Expr::Paren(inner, _) => {
            ctx.write("(");
            emit_expr(inner, ctx);
            ctx.write(")");
        }
        Expr::Block(block) => {
            ctx.writeln("{");
            ctx.indent();
            for stmt in &block.stmts {
                crate::stmts::emit_stmt(stmt, ctx);
            }
            if let Some(e) = &block.expr {
                ctx.emit_indent();
                emit_expr(e, ctx);
                ctx.newline();
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.write("}");
        }
        Expr::Binary(b) => emit_binary(b, ctx),
        Expr::Unary(u) => {
            let op = match u.op {
                UnOp::Not => "!",
                UnOp::Neg => "-",
            };
            ctx.write(op);
            emit_expr(&u.operand, ctx);
        }
        Expr::Call(call) => {
            // Check if this is a builtin call via a simple Variable func
            if let Expr::Variable(ident, _) = call.func.as_ref() {
                if is_builtin(&ident.name) {
                    emit_builtin_call(&ident.name, &call.args, ctx, &mut emit_expr);
                    return;
                }
            }
            // Check if this is a method call: FieldAccess(obj, method)(args)
            if let Expr::FieldAccess(fa) = call.func.as_ref() {
                emit_expr(&fa.object, ctx);
                ctx.write(".");
                ctx.write(&fa.field.name);
                ctx.write("(");
                for (i, arg) in call.args.iter().enumerate() {
                    if i > 0 {
                        ctx.write(", ");
                    }
                    emit_expr(arg, ctx);
                }
                ctx.write(")");
                return;
            }
            // Regular function call
            emit_expr(&call.func, ctx);
            ctx.write("(");
            for (i, arg) in call.args.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_call_arg(arg, ctx);
            }
            ctx.write(")");
        }
        Expr::Array(arr) => {
            ctx.write("vec![");
            for (i, elem) in arr.elements.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_expr(elem, ctx);
            }
            ctx.write("]");
        }
        Expr::Tuple(tup) => {
            ctx.write("(");
            for (i, elem) in tup.elements.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_expr(elem, ctx);
            }
            if tup.elements.len() == 1 {
                ctx.write(","); // trailing comma for 1-tuples
            }
            ctx.write(")");
        }
        Expr::Struct(sl) => {
            ctx.write(&sl.name.name);
            ctx.write(" { ");
            for (i, fi) in sl.fields.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                ctx.write(&fi.name.name);
                ctx.write(": ");
                emit_expr(&fi.value, ctx);
            }
            ctx.write(" }");
        }
        Expr::ArrayAccess(aa) => {
            emit_expr(&aa.array, ctx);
            ctx.write("[");
            // Array/slice indices in Rust must be `usize`, not `i64`.
            match aa.index.as_ref() {
                Expr::Range(re) => {
                    // Range slicing: cast each bound to usize (wrap in parens for safety)
                    if let Some(start) = &re.start {
                        ctx.write("(");
                        emit_expr(start, ctx);
                        ctx.write(") as usize");
                    }
                    match re.op {
                        RangeOp::Exclusive => ctx.write(".."),
                        RangeOp::Inclusive => ctx.write("..="),
                    }
                    if let Some(end) = &re.end {
                        ctx.write("(");
                        emit_expr(end, ctx);
                        ctx.write(") as usize");
                    }
                    ctx.write("]");
                    // Vec slicing returns &[T]; call .to_vec() for owned Vec<T>
                    ctx.write(".to_vec()");
                }
                other => {
                    // Wrap in parens before `as usize` so that binary expressions like
                    // `i + 1` emit as `(i + 1) as usize` instead of `i + 1 as usize`
                    // (which Rust parses as `i + (1 as usize)` — type mismatch).
                    // Add `.clone()` so Vec<String>[i] returns an owned String rather than
                    // attempting a move out of the Vec. For Copy types (i64, f64) clone is
                    // free; for String it produces an owned copy — correct semantics.
                    ctx.write("(");
                    emit_expr(other, ctx);
                    ctx.write(") as usize].clone()");
                }
            }
        }
        Expr::FieldAccess(fa) => emit_field_access(fa, ctx),
        Expr::If(if_expr) => {
            ctx.write("if ");
            emit_expr(&if_expr.condition, ctx);
            ctx.write(" {");
            ctx.newline();
            ctx.indent();
            for stmt in &if_expr.then_branch.stmts {
                crate::stmts::emit_stmt(stmt, ctx);
            }
            if let Some(e) = &if_expr.then_branch.expr {
                ctx.emit_indent();
                emit_expr(e, ctx);
                ctx.newline();
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.write("}");
            if let Some(else_branch) = &if_expr.else_branch {
                ctx.write(" else {");
                ctx.newline();
                ctx.indent();
                for stmt in &else_branch.stmts {
                    crate::stmts::emit_stmt(stmt, ctx);
                }
                if let Some(e) = &else_branch.expr {
                    ctx.emit_indent();
                    emit_expr(e, ctx);
                    ctx.newline();
                }
                ctx.dedent();
                ctx.emit_indent();
                ctx.write("}");
            }
        }
        Expr::Match(me) => {
            ctx.write("match ");
            emit_expr(&me.expr, ctx);
            ctx.writeln(" {");
            ctx.indent();
            for arm in &me.arms {
                ctx.emit_indent();
                emit_pattern(&arm.pattern, ctx);
                ctx.write(" => ");
                emit_expr(&arm.body, ctx);
                ctx.writeln(",");
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.write("}");
        }
        Expr::Range(re) => {
            if let Some(start) = &re.start {
                emit_expr(start, ctx);
            }
            match re.op {
                RangeOp::Exclusive => ctx.write(".."),
                RangeOp::Inclusive => ctx.write("..="),
            }
            if let Some(end) = &re.end {
                emit_expr(end, ctx);
            }
        }
        Expr::Closure(ce) => {
            ctx.write("move |");
            for (i, param) in ce.params.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                match &param.name {
                    ParamName::Ident(id) => ctx.write(&id.name),
                    ParamName::SelfParam => ctx.write("self"),
                    ParamName::GenericBound(id, _) => ctx.write(&id.name),
                }
                if !matches!(param.ty, Type::Inferred(_)) {
                    ctx.write(": ");
                    ctx.write(&emit_type(&param.ty));
                }
            }
            ctx.write("| ");
            emit_expr(&ce.body, ctx);
        }
        Expr::EnumVariant(ev) => {
            ctx.write(&ev.enum_name.name);
            ctx.write("::");
            ctx.write(&ev.variant_name.name);
            if let Some(args) = &ev.args {
                ctx.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        ctx.write(", ");
                    }
                    emit_expr(arg, ctx);
                }
                ctx.write(")");
            }
        }
    }
}

/// Emit a literal value.
///
/// String literals are emitted as `"...".to_string()` because Lang4LLM's
/// `string` type maps to Rust's owned `String`, not `&str`.
pub fn emit_literal(lit: &Literal, ctx: &mut CodegenContext) {
    match lit {
        // Emit with i64 suffix so Rust always uses i64, not i32.
        // Lang4LLM's `int` type is always i64.
        Literal::Int(n) => ctx.write(&format!("{}_i64", n)),
        Literal::Float(f) => {
            let s = format!("{:?}", f);
            ctx.write(&s);
        }
        Literal::Bool(b) => ctx.write(if *b { "true" } else { "false" }),
        Literal::String(s) => {
            ctx.write(&format!("\"{}\"", s.escape_default()));
            ctx.write(".to_string()");
        }
        Literal::None => ctx.write("()"),
    }
}

/// Emit a string literal as a raw `&str` slice (for use inside format templates).
pub fn emit_literal_raw_str(s: &str, ctx: &mut CodegenContext) {
    ctx.write(&format!("\"{}\"", s.escape_default()));
}

// =============================================================================
// Binary expression with string concat chain flattening
// =============================================================================

/// Operator precedence for binary operators (higher = binds tighter).
fn binop_precedence(op: &BinOp) -> u8 {
    match op {
        BinOp::Or => 1,
        BinOp::And => 2,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => 3,
        BinOp::Add | BinOp::Sub => 4,
        BinOp::Mul | BinOp::Div | BinOp::Mod => 5,
    }
}

/// Emit a function call argument with automatic `.clone()` for non-primitive types.
///
/// In Rust, passing `Vec<T>` or a user-defined struct by value moves it, preventing reuse.
/// Lang4LLM semantics allow reusing a variable after passing it to a function (value semantics
/// without linear types), so we emit `.clone()` on any variable or field-access argument.
/// Primitive types (i64, f64, bool) implement Copy, so `.clone()` is a no-op for them.
pub fn emit_call_arg(arg: &Expr, ctx: &mut CodegenContext) {
    match arg {
        Expr::Variable(_, _) | Expr::FieldAccess(_) => {
            emit_expr(arg, ctx);
            ctx.write(".clone()");
        }
        _ => emit_expr(arg, ctx),
    }
}

/// Emit a sub-expression, wrapping in parens if its precedence is lower than `parent_prec`.
/// This is needed because the parser strips parentheses (returns inner expr directly),
/// so we must re-insert them based on precedence during code generation.
fn emit_expr_with_prec(expr: &Expr, parent_prec: u8, ctx: &mut CodegenContext) {
    if let Expr::Binary(inner) = expr {
        let inner_prec = binop_precedence(&inner.op);
        if inner_prec < parent_prec {
            ctx.write("(");
            emit_expr(expr, ctx);
            ctx.write(")");
            return;
        }
    }
    emit_expr(expr, ctx);
}

/// Emit a binary expression, flattening string concat chains into `format!`.
fn emit_binary(b: &BinaryExpr, ctx: &mut CodegenContext) {
    if b.op == BinOp::Add && is_string_concat_chain(&Expr::Binary(b.clone()), ctx) {
        let mut parts = Vec::new();
        collect_concat_chain(&Expr::Binary(b.clone()), &mut parts);
        emit_format_concat(&parts, ctx);
        return;
    }

    let op_str = match b.op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Mod => "%",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
        BinOp::And => "&&",
        BinOp::Or => "||",
    };

    let prec = binop_precedence(&b.op);

    // LHS: for left-associative ops, wrap only if strictly lower precedence
    emit_expr_with_prec(&b.lhs, prec, ctx);
    ctx.write(" ");
    ctx.write(op_str);
    ctx.write(" ");
    // RHS: wrap if lower precedence (handles `a - (b - c)` and `a / (b * c)`)
    emit_expr_with_prec(&b.rhs, prec + 1, ctx);
}

/// Returns true if this `+` chain involves at least one string expression.
///
/// "String expression" means: a string literal, a call to a known string-returning
/// builtin (to_string, to_upper, to_lower, trim, join, replace), or a binary `+`
/// chain containing any of the above.
fn is_string_concat_chain(expr: &Expr, ctx: &CodegenContext) -> bool {
    contains_string_expr(expr, ctx)
}

/// Known string-returning builtins for heuristic string-concat detection.
fn is_string_returning_builtin(name: &str) -> bool {
    matches!(
        name,
        "to_string" | "to_upper" | "to_lower" | "trim" | "join" | "replace" | "concat"
    )
}

fn contains_string_expr(expr: &Expr, ctx: &CodegenContext) -> bool {
    match expr {
        // String literals are always string-typed
        Expr::Literal(Literal::String(_), _) => true,
        // Recurse through `+` chains
        Expr::Binary(b) if b.op == BinOp::Add => {
            contains_string_expr(&b.lhs, ctx) || contains_string_expr(&b.rhs, ctx)
        }
        Expr::Paren(inner, _) => contains_string_expr(inner, ctx),
        // A call to a known string-returning builtin (e.g. to_lower, to_string, trim)
        // or a user-defined function that returns string
        Expr::Call(c) => {
            if let Expr::Variable(ident, _) = c.func.as_ref() {
                if is_string_returning_builtin(&ident.name) {
                    return true;
                }
                // Check user-defined function return type
                if let Some(ret_ty) = ctx.fn_return_types.get(&ident.name) {
                    return matches!(
                        ret_ty,
                        lang4llm_ast::Type::Primitive(lang4llm_ast::PrimitiveType::String, _)
                    );
                }
            }
            false
        }
        _ => false,
    }
}

enum ConcatPart {
    Lit(String),
    Expr(Expr),
}

/// Flatten a `+` chain into a list of literal strings and expressions.
fn collect_concat_chain(expr: &Expr, parts: &mut Vec<ConcatPart>) {
    match expr {
        Expr::Binary(b) if b.op == BinOp::Add => {
            collect_concat_chain(&b.lhs, parts);
            collect_concat_chain(&b.rhs, parts);
        }
        Expr::Paren(inner, _) => collect_concat_chain(inner, parts),
        Expr::Literal(Literal::String(s), _) => parts.push(ConcatPart::Lit(s.clone())),
        other => parts.push(ConcatPart::Expr(other.clone())),
    }
}

/// Emit a `format!("template", args...)` macro call from concat parts.
fn emit_format_concat(parts: &[ConcatPart], ctx: &mut CodegenContext) {
    let mut template = String::new();
    let mut arg_exprs: Vec<&Expr> = Vec::new();

    for part in parts {
        match part {
            ConcatPart::Lit(s) => {
                for ch in s.chars() {
                    if ch == '{' {
                        template.push_str("{{");
                    } else if ch == '}' {
                        template.push_str("}}");
                    } else if ch == '"' {
                        // Escape inner quotes so the Rust string literal stays valid
                        template.push_str("\\\"");
                    } else if ch == '\\' {
                        // Escape backslashes
                        template.push_str("\\\\");
                    } else {
                        template.push(ch);
                    }
                }
            }
            ConcatPart::Expr(e) => {
                template.push_str("{}");
                arg_exprs.push(e);
            }
        }
    }

    ctx.write("format!(\"");
    ctx.write(&template);
    ctx.write("\"");
    for arg in &arg_exprs {
        ctx.write(", ");
        emit_expr(arg, ctx);
    }
    ctx.write(")");
}

/// Emit a field access expression.
fn emit_field_access(fa: &FieldAccess, ctx: &mut CodegenContext) {
    emit_expr(&fa.object, ctx);
    ctx.write(".");
    ctx.write(&fa.field.name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang4llm_ast::{BinaryExpr, Expr, Ident, Literal, Span};

    fn dummy_span() -> Span {
        Span::new(0, 0, 0, 0)
    }

    fn ident(name: &str) -> Ident {
        Ident::new(name.to_string(), dummy_span())
    }

    fn str_lit(s: &str) -> Expr {
        Expr::Literal(Literal::String(s.to_string()), dummy_span())
    }

    fn int_lit(n: i64) -> Expr {
        Expr::Literal(Literal::Int(n), dummy_span())
    }

    fn var(name: &str) -> Expr {
        Expr::Variable(ident(name), dummy_span())
    }

    fn add(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary(BinaryExpr {
            lhs: Box::new(lhs),
            op: BinOp::Add,
            rhs: Box::new(rhs),
            span: dummy_span(),
        })
    }

    #[test]
    fn test_int_literal() {
        let mut ctx = CodegenContext::new();
        emit_expr(&int_lit(42), &mut ctx);
        assert_eq!(ctx.finish(), "42_i64");
    }

    #[test]
    fn test_string_literal() {
        let mut ctx = CodegenContext::new();
        emit_expr(&str_lit("hello"), &mut ctx);
        // String literals emit as owned String (not &str) to match Lang4LLM's `string` type
        assert_eq!(ctx.finish(), "\"hello\".to_string()");
    }

    #[test]
    fn test_numeric_add() {
        let mut ctx = CodegenContext::new();
        emit_expr(&add(int_lit(1), int_lit(2)), &mut ctx);
        assert_eq!(ctx.finish(), "1_i64 + 2_i64");
    }

    #[test]
    fn test_string_concat_simple() {
        // "hello" + " world" → format!("hello world")
        let mut ctx = CodegenContext::new();
        emit_expr(&add(str_lit("hello"), str_lit(" world")), &mut ctx);
        assert_eq!(ctx.finish(), "format!(\"hello world\")");
    }

    #[test]
    fn test_string_concat_with_var() {
        // "x = " + x → format!("x = {}", x)
        let mut ctx = CodegenContext::new();
        emit_expr(&add(str_lit("x = "), var("x")), &mut ctx);
        assert_eq!(ctx.finish(), "format!(\"x = {}\", x)");
    }

    #[test]
    fn test_string_concat_chain() {
        // "Point(" + x + ", " + y + ")" → format!("Point({}, {})", x, y)
        let chain = add(
            add(add(add(str_lit("Point("), var("x")), str_lit(", ")), var("y")),
            str_lit(")"),
        );
        let mut ctx = CodegenContext::new();
        emit_expr(&chain, &mut ctx);
        assert_eq!(ctx.finish(), "format!(\"Point({}, {})\", x, y)");
    }

    #[test]
    fn test_brace_escaping_in_string_concat() {
        // "{key}" + v → format!("{{key}}{}", v)
        let mut ctx = CodegenContext::new();
        emit_expr(&add(str_lit("{key}"), var("v")), &mut ctx);
        assert_eq!(ctx.finish(), "format!(\"{{key}}{}\", v)");
    }

    #[test]
    fn test_string_concat_user_fn() {
        use lang4llm_ast::{CallExpr, PrimitiveType, Type};
        use std::collections::HashMap;

        let mut fn_return_types = HashMap::new();
        fn_return_types.insert(
            "greet".to_string(),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        );
        let ctx = CodegenContext::with_fn_types(fn_return_types);

        // "prefix: " + greet() should be detected as string concat
        let call = Expr::Call(CallExpr {
            func: Box::new(Expr::Variable(
                Ident::new("greet".to_string(), Span::dummy()),
                Span::dummy(),
            )),
            args: vec![],
            span: Span::dummy(),
        });
        let binary = add(str_lit("prefix: "), call);
        assert!(is_string_concat_chain(&binary, &ctx));
    }
}
