//! Pattern pretty-printing.

use lang4llm_ast::{Literal, Pattern};

use crate::context::FmtContext;

/// Format a pattern, appending to `ctx`.
pub fn fmt_pattern(pat: &Pattern, ctx: &mut FmtContext) {
    match pat {
        Pattern::Literal(lit, _) => fmt_literal_pat(lit, ctx),
        Pattern::Ident(ident, _) => ctx.write(&ident.name),
        Pattern::Wildcard(_) => ctx.write("_"),
        Pattern::Tuple(pats, _) => {
            ctx.write("(");
            for (i, p) in pats.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_pattern(p, ctx);
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
                fmt_pattern(&fp.pattern, ctx);
            }
            ctx.write(" }");
        }
        Pattern::Enum(ep) => {
            if let Some(en) = &ep.enum_name {
                ctx.write(&en.name);
                ctx.write("::");
            }
            ctx.write(&ep.variant_name.name);
            if let Some(pats) = &ep.patterns {
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
}

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

fn fmt_literal_pat(lit: &Literal, ctx: &mut FmtContext) {
    match lit {
        Literal::Int(n) => ctx.write(&n.to_string()),
        Literal::Float(f) => ctx.write(&format!("{f}")),
        Literal::Bool(b) => ctx.write(if *b { "true" } else { "false" }),
        Literal::String(s) => {
            ctx.write("\"");
            ctx.write(&escape_string(s));
            ctx.write("\"");
        }
        Literal::None => ctx.write("none"),
    }
}
