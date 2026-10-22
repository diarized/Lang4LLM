//! Expression pretty-printing with precedence-aware parenthesization.

use lang4llm_ast::{BinOp, Expr, Literal, RangeOp, UnOp};

use crate::context::FmtContext;
use crate::patterns::fmt_pattern;
use crate::stmts::fmt_stmt;
use crate::types::fmt_type;

/// Operator precedence (higher = tighter binding).
fn binop_prec(op: BinOp) -> u8 {
    match op {
        BinOp::Or => 1,
        BinOp::And => 2,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => 3,
        BinOp::Add | BinOp::Sub => 4,
        BinOp::Mul | BinOp::Div | BinOp::Mod => 5,
    }
}

fn binop_str(op: BinOp) -> &'static str {
    match op {
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
        BinOp::And => "and",
        BinOp::Or => "or",
    }
}

/// Format an expression, parenthesizing if its precedence is lower than `min_prec`.
pub fn fmt_expr_prec(expr: &Expr, min_prec: u8, ctx: &mut FmtContext) {
    if let Expr::Binary(b) = expr {
        let prec = binop_prec(b.op);
        if prec < min_prec {
            ctx.write("(");
            fmt_expr_prec(expr, 0, ctx);
            ctx.write(")");
            return;
        }
    }
    fmt_expr(expr, ctx);
}

/// Format an expression.
pub fn fmt_expr(expr: &Expr, ctx: &mut FmtContext) {
    match expr {
        Expr::Literal(lit, _) => fmt_literal(lit, ctx),
        Expr::Variable(ident, _) => ctx.write(&ident.name),
        Expr::Paren(inner, _) => {
            ctx.write("(");
            fmt_expr(inner, ctx);
            ctx.write(")");
        }
        Expr::Binary(b) => {
            let prec = binop_prec(b.op);
            fmt_expr_prec(&b.lhs, prec, ctx);
            ctx.write(" ");
            ctx.write(binop_str(b.op));
            ctx.write(" ");
            // Right operand: for left-associative ops, use prec+1 to force parens
            fmt_expr_prec(&b.rhs, prec + 1, ctx);
        }
        Expr::Unary(u) => {
            let op = match u.op {
                UnOp::Not => "not ",
                UnOp::Neg => "-",
            };
            ctx.write(op);
            fmt_expr(&u.operand, ctx);
        }
        Expr::Call(call) => {
            fmt_expr(&call.func, ctx);
            ctx.write("(");
            for (i, arg) in call.args.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_expr(arg, ctx);
            }
            ctx.write(")");
        }
        Expr::Array(arr) => {
            ctx.write("[");
            for (i, elem) in arr.elements.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_expr(elem, ctx);
            }
            ctx.write("]");
        }
        Expr::Tuple(tup) => {
            ctx.write("(");
            for (i, elem) in tup.elements.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_expr(elem, ctx);
            }
            if tup.elements.len() == 1 {
                ctx.write(",");
            }
            ctx.write(")");
        }
        Expr::Struct(s) => {
            ctx.write(&s.name.name);
            ctx.write(" { ");
            for (i, fi) in s.fields.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                ctx.write(&fi.name.name);
                ctx.write(": ");
                fmt_expr(&fi.value, ctx);
            }
            ctx.write(" }");
        }
        Expr::ArrayAccess(aa) => {
            fmt_expr(&aa.array, ctx);
            ctx.write("[");
            fmt_expr(&aa.index, ctx);
            ctx.write("]");
        }
        Expr::FieldAccess(fa) => {
            fmt_expr(&fa.object, ctx);
            ctx.write(".");
            ctx.write(&fa.field.name);
        }
        Expr::If(ie) => {
            ctx.write("if ");
            fmt_expr(&ie.condition, ctx);
            ctx.write(" ");
            fmt_block_inline(&ie.then_branch.stmts, ie.then_branch.expr.as_deref(), ctx);
            if let Some(else_block) = &ie.else_branch {
                ctx.write(" else ");
                fmt_block_inline(&else_block.stmts, else_block.expr.as_deref(), ctx);
            }
        }
        Expr::Match(me) => {
            ctx.write("match ");
            fmt_expr(&me.expr, ctx);
            ctx.writeln(" {");
            ctx.indent();
            for arm in &me.arms {
                ctx.emit_indent();
                fmt_pattern(&arm.pattern, ctx);
                ctx.write(" => ");
                fmt_expr(&arm.body, ctx);
                ctx.writeln(",");
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.write("}");
        }
        Expr::Block(block) => {
            fmt_block_inline(&block.stmts, block.expr.as_deref(), ctx);
        }
        Expr::Range(re) => {
            if let Some(start) = &re.start {
                fmt_expr(start, ctx);
            }
            match re.op {
                RangeOp::Exclusive => ctx.write(".."),
                RangeOp::Inclusive => ctx.write("..="),
            }
            if let Some(end) = &re.end {
                fmt_expr(end, ctx);
            }
        }
        Expr::Closure(ce) => {
            ctx.write("|");
            for (i, p) in ce.params.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_closure_param(p, ctx);
            }
            ctx.write("| ");
            fmt_expr(&ce.body, ctx);
        }
        Expr::EnumVariant(ev) => {
            ctx.write(&ev.enum_name.name);
            ctx.write("::");
            ctx.write(&ev.variant_name.name);
            if let Some(args) = &ev.args {
                ctx.write("(");
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        ctx.write(", ");
                    }
                    fmt_expr(a, ctx);
                }
                ctx.write(")");
            }
        }
    }
}

/// Format a block expression inline (used for if/else/match bodies).
pub fn fmt_block_inline(stmts: &[lang4llm_ast::Stmt], expr: Option<&Expr>, ctx: &mut FmtContext) {
    ctx.writeln("{");
    ctx.indent();
    for s in stmts {
        ctx.emit_indent();
        fmt_stmt(s, ctx);
    }
    if let Some(e) = expr {
        ctx.emit_indent();
        fmt_expr(e, ctx);
        ctx.newline();
    }
    ctx.dedent();
    ctx.emit_indent();
    ctx.write("}");
}

fn fmt_closure_param(p: &lang4llm_ast::Param, ctx: &mut FmtContext) {
    match &p.name {
        lang4llm_ast::ParamName::SelfParam => ctx.write("self"),
        lang4llm_ast::ParamName::Ident(id) => ctx.write(&id.name),
        lang4llm_ast::ParamName::GenericBound(id, _) => ctx.write(&id.name),
    }
    // Closure params may have inferred types — only emit annotation if non-inferred
    if !matches!(p.ty, lang4llm_ast::Type::Inferred(_)) {
        ctx.write(": ");
        fmt_type(&p.ty, ctx);
    }
}

/// Re-encode a decoded string value back to its source representation.
/// The lexer decodes `\n` → '\n', `\\` → '\', `\"` → '"', etc.
/// We must reverse this for the formatter output.
fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

fn fmt_literal(lit: &Literal, ctx: &mut FmtContext) {
    match lit {
        Literal::Int(n) => ctx.write(&n.to_string()),
        Literal::Float(f) => {
            let s = format!("{f}");
            // Ensure float literals always have a decimal point
            if s.contains('.') || s.contains('e') || s.contains('E') {
                ctx.write(&s);
            } else {
                ctx.write(&s);
                ctx.write(".0");
            }
        }
        Literal::Bool(b) => ctx.write(if *b { "true" } else { "false" }),
        Literal::String(s) => {
            ctx.write("\"");
            ctx.write(&escape_string(s));
            ctx.write("\"");
        }
        Literal::None => ctx.write("none"),
    }
}
