//! Statement pretty-printing.

use lang4llm_ast::{AssignTarget, Stmt};

use crate::context::FmtContext;
use crate::exprs::fmt_expr;
use crate::patterns::fmt_pattern;
use crate::types::fmt_type;

/// Format a single statement, appending to `ctx` (including trailing newline).
pub fn fmt_stmt(stmt: &Stmt, ctx: &mut FmtContext) {
    match stmt {
        Stmt::Let(ls) => {
            if ls.mutable {
                ctx.write("let mut ");
            } else {
                ctx.write("let ");
            }
            ctx.write(&ls.name.name);
            ctx.write(": ");
            fmt_type(&ls.ty, ctx);
            if let Some(init) = &ls.init {
                ctx.write(" = ");
                fmt_expr(init, ctx);
            }
            ctx.writeln(";");
        }
        Stmt::Assign(as_) => {
            fmt_assign_target(&as_.lhs, ctx);
            ctx.write(" = ");
            fmt_expr(&as_.rhs, ctx);
            ctx.writeln(";");
        }
        Stmt::Expr(es) => {
            fmt_expr(&es.expr, ctx);
            ctx.writeln(";");
        }
        Stmt::Return(rs) => {
            if let Some(val) = &rs.value {
                ctx.write("return ");
                fmt_expr(val, ctx);
                ctx.writeln(";");
            } else {
                ctx.writeln("return;");
            }
        }
        Stmt::Break(_) => ctx.writeln("break;"),
        Stmt::Continue(_) => ctx.writeln("continue;"),
        Stmt::While(ws) => {
            ctx.write("while ");
            fmt_expr(&ws.condition, ctx);
            ctx.write(" ");
            fmt_block(&ws.body.stmts, ws.body.expr.as_deref(), ctx);
        }
        Stmt::For(fs) => {
            ctx.write("for ");
            ctx.write(&fs.var.name);
            ctx.write(" in ");
            fmt_expr(&fs.iter, ctx);
            ctx.write(" ");
            fmt_block(&fs.body.stmts, fs.body.expr.as_deref(), ctx);
        }
        Stmt::Loop(ls) => {
            ctx.write("loop ");
            fmt_block(&ls.body.stmts, ls.body.expr.as_deref(), ctx);
        }
        Stmt::Empty(_) => ctx.writeln(";"),
    }
}

/// Format a block as a statement (writes `{\n ... \n}`).
fn fmt_block(stmts: &[Stmt], expr: Option<&lang4llm_ast::Expr>, ctx: &mut FmtContext) {
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
    ctx.writeln("}");
}

fn fmt_assign_target(target: &AssignTarget, ctx: &mut FmtContext) {
    match target {
        AssignTarget::Var(id) => ctx.write(&id.name),
        AssignTarget::ArrayAccess(arr, idx, _) => {
            fmt_expr(arr, ctx);
            ctx.write("[");
            fmt_expr(idx, ctx);
            ctx.write("]");
        }
        AssignTarget::FieldAccess(obj, field, _) => {
            fmt_expr(obj, ctx);
            ctx.write(".");
            ctx.write(&field.name);
        }
        AssignTarget::Tuple(pats, _) => {
            ctx.write("(");
            for (i, p) in pats.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_pattern(p, ctx);
            }
            ctx.write(")");
        }
    }
}
