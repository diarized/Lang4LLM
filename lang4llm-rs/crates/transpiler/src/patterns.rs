//! Pattern code generation for match arms and destructuring.

use lang4llm_ast::{Literal, Pattern};

use crate::context::CodegenContext;

/// Emit a pattern as Rust source.
pub fn emit_pattern(pat: &Pattern, ctx: &mut CodegenContext) {
    match pat {
        Pattern::Literal(lit, _) => emit_literal_pattern(lit, ctx),
        Pattern::Ident(ident, _) => ctx.write(&ident.name),
        Pattern::Wildcard(_) => ctx.write("_"),
        Pattern::Tuple(pats, _) => {
            ctx.write("(");
            for (i, p) in pats.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_pattern(p, ctx);
            }
            ctx.write(")");
        }
        Pattern::Struct(sp) => {
            ctx.write(&sp.name.name);
            ctx.write(" { ");
            for (i, fp) in sp.fields.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                ctx.write(&fp.name.name);
                ctx.write(": ");
                emit_pattern(&fp.pattern, ctx);
            }
            ctx.write(" }");
        }
        Pattern::Enum(ep) => {
            if let Some(enum_name) = &ep.enum_name {
                ctx.write(&enum_name.name);
                ctx.write("::");
            }
            ctx.write(&ep.variant_name.name);
            if let Some(pats) = &ep.patterns {
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
}

fn emit_literal_pattern(lit: &Literal, ctx: &mut CodegenContext) {
    match lit {
        Literal::Int(n) => ctx.write(&format!("{}_i64", n)),
        Literal::Float(f) => ctx.write(&format!("{:?}", f)),
        Literal::Bool(b) => ctx.write(if *b { "true" } else { "false" }),
        Literal::String(s) => ctx.write(&format!("\"{}\"", s.escape_default())),
        Literal::None => ctx.write("()"),
    }
}
