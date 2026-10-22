//! Statement code generation.

use lang4llm_ast::{AssignTarget, Stmt, Type};

use crate::context::CodegenContext;
use crate::exprs::{emit_call_arg, emit_expr};
use crate::patterns::emit_pattern;
use crate::types::emit_type;

/// Emit a statement as Rust source (with trailing newline).
pub fn emit_stmt(stmt: &Stmt, ctx: &mut CodegenContext) {
    match stmt {
        Stmt::Let(let_stmt) => {
            ctx.emit_indent();
            ctx.write("let ");
            if let_stmt.mutable {
                ctx.write("mut ");
            }
            ctx.write(&let_stmt.name.name);
            // Emit type annotation unless it's inferred or a function type.
            // `impl Fn(T) -> U` is not allowed in let binding position in Rust;
            // omit the annotation and let Rust infer the closure type.
            let skip_annotation = matches!(let_stmt.ty, Type::Inferred(_) | Type::Function(_));
            if !skip_annotation {
                ctx.write(": ");
                ctx.write(&emit_type(&let_stmt.ty));
            }
            if let Some(init) = &let_stmt.init {
                ctx.write(" = ");
                // Clone Variable/FieldAccess initialisers to avoid partial moves of structs/Vecs.
                // (e.g. `let d = cb.data` would move Vec<i64> out of `cb`; `.clone()` prevents this.)
                emit_call_arg(init, ctx);
            }
            ctx.writeln(";");
        }
        Stmt::Assign(assign) => {
            ctx.emit_indent();
            emit_assign_target(&assign.lhs, ctx);
            ctx.write(" = ");
            emit_expr(&assign.rhs, ctx);
            ctx.writeln(";");
        }
        Stmt::Expr(expr_stmt) => {
            ctx.emit_indent();
            emit_expr(&expr_stmt.expr, ctx);
            ctx.writeln(";");
        }
        Stmt::Return(ret) => {
            ctx.emit_indent();
            if let Some(val) = &ret.value {
                ctx.write("return ");
                emit_expr(val, ctx);
                ctx.writeln(";");
            } else {
                ctx.writeln("return;");
            }
        }
        Stmt::Break(_) => {
            ctx.emit_indent();
            ctx.writeln("break;");
        }
        Stmt::Continue(_) => {
            ctx.emit_indent();
            ctx.writeln("continue;");
        }
        Stmt::While(w) => {
            ctx.emit_indent();
            ctx.write("while ");
            emit_expr(&w.condition, ctx);
            ctx.writeln(" {");
            ctx.indent();
            for s in &w.body.stmts {
                emit_stmt(s, ctx);
            }
            if let Some(e) = &w.body.expr {
                ctx.emit_indent();
                emit_expr(e, ctx);
                ctx.newline();
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.writeln("}");
        }
        Stmt::For(f) => {
            ctx.emit_indent();
            ctx.write("for ");
            ctx.write(&f.var.name);
            ctx.write(" in ");
            emit_expr(&f.iter, ctx);
            ctx.writeln(" {");
            ctx.indent();
            for s in &f.body.stmts {
                emit_stmt(s, ctx);
            }
            if let Some(e) = &f.body.expr {
                ctx.emit_indent();
                emit_expr(e, ctx);
                ctx.newline();
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.writeln("}");
        }
        Stmt::Loop(l) => {
            ctx.emit_indent();
            ctx.writeln("loop {");
            ctx.indent();
            for s in &l.body.stmts {
                emit_stmt(s, ctx);
            }
            if let Some(e) = &l.body.expr {
                ctx.emit_indent();
                emit_expr(e, ctx);
                ctx.newline();
            }
            ctx.dedent();
            ctx.emit_indent();
            ctx.writeln("}");
        }
        Stmt::Empty(_) => {
            // Omit empty statements — they add no value
        }
    }
}

fn emit_assign_target(target: &AssignTarget, ctx: &mut CodegenContext) {
    match target {
        AssignTarget::Var(ident) => ctx.write(&ident.name),
        AssignTarget::ArrayAccess(arr, idx, _) => {
            emit_expr(arr, ctx);
            ctx.write("[(");
            emit_expr(idx, ctx);
            ctx.write(") as usize]");
        }
        AssignTarget::FieldAccess(obj, field, _) => {
            emit_expr(obj, ctx);
            ctx.write(".");
            ctx.write(&field.name);
        }
        AssignTarget::Tuple(pats, _) => {
            ctx.write("(");
            for (i, p) in pats.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_pattern(p, ctx);
            }
            ctx.write(")");
        }
    }
}
