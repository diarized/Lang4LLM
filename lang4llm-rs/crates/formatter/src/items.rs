//! Top-level item pretty-printing: functions, structs, enums, traits, impls.

use lang4llm_ast::{FunctionDef, FunctionSig, GenericParamDef, GenericParams, ImplBlock, Item,
                   Param, ParamName, StructDef, EnumDef, TraitDef};

use crate::context::FmtContext;
use crate::stmts::fmt_stmt;
use crate::types::fmt_type;

/// Format a top-level item, appending to `ctx`.
pub fn fmt_item(item: &Item, ctx: &mut FmtContext) {
    match item {
        Item::Function(f) => fmt_function(f, ctx),
        Item::Struct(s) => fmt_struct(s, ctx),
        Item::Enum(e) => fmt_enum(e, ctx),
        Item::Trait(t) => fmt_trait(t, ctx),
        Item::Impl(i) => fmt_impl(i, ctx),
    }
}

// ─── Function ────────────────────────────────────────────────────────────────

pub fn fmt_function(f: &FunctionDef, ctx: &mut FmtContext) {
    ctx.write("fn ");
    ctx.write(&f.name.name);
    if let Some(g) = &f.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.write("(");
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        fmt_param(p, ctx);
    }
    ctx.write(") -> ");
    fmt_type(&f.return_type, ctx);
    ctx.write(" ");
    // Body block
    ctx.writeln("{");
    ctx.indent();
    for stmt in &f.body.stmts {
        ctx.emit_indent();
        fmt_stmt(stmt, ctx);
    }
    if let Some(e) = &f.body.expr {
        ctx.emit_indent();
        crate::exprs::fmt_expr(e, ctx);
        ctx.newline();
    }
    ctx.dedent();
    ctx.writeln("}");
}

fn fmt_param(p: &Param, ctx: &mut FmtContext) {
    match &p.name {
        ParamName::SelfParam => ctx.write("self"),
        ParamName::Ident(id) => {
            ctx.write(&id.name);
            ctx.write(": ");
            fmt_type(&p.ty, ctx);
        }
        ParamName::GenericBound(id, bounds) => {
            ctx.write(&id.name);
            ctx.write(": ");
            fmt_type(&p.ty, ctx);
            if !bounds.is_empty() {
                ctx.write(": ");
                for (i, b) in bounds.iter().enumerate() {
                    if i > 0 {
                        ctx.write(" + ");
                    }
                    ctx.write(&b.name);
                }
            }
        }
    }
}

fn fmt_generic_params(g: &GenericParams, ctx: &mut FmtContext) {
    ctx.write("<");
    for (i, gp) in g.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        fmt_generic_param_def(gp, ctx);
    }
    ctx.write(">");
}

fn fmt_generic_param_def(gp: &GenericParamDef, ctx: &mut FmtContext) {
    ctx.write(&gp.name.name);
    if let Some(bound) = &gp.bound {
        ctx.write(": ");
        ctx.write(&bound.name);
    }
}

// ─── Struct ──────────────────────────────────────────────────────────────────

pub fn fmt_struct(s: &StructDef, ctx: &mut FmtContext) {
    ctx.write("struct ");
    ctx.write(&s.name.name);
    if let Some(g) = &s.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for field in &s.fields {
        ctx.emit_indent();
        ctx.write(&field.name.name);
        ctx.write(": ");
        fmt_type(&field.ty, ctx);
        ctx.writeln(",");
    }
    ctx.dedent();
    ctx.writeln("}");
}

// ─── Enum ────────────────────────────────────────────────────────────────────

pub fn fmt_enum(e: &EnumDef, ctx: &mut FmtContext) {
    ctx.write("enum ");
    ctx.write(&e.name.name);
    if let Some(g) = &e.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for variant in &e.variants {
        ctx.emit_indent();
        ctx.write(&variant.name.name);
        if let Some(data) = &variant.data {
            ctx.write("(");
            for (i, t) in data.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_type(t, ctx);
            }
            ctx.write(")");
        }
        ctx.writeln(",");
    }
    ctx.dedent();
    ctx.writeln("}");
}

// ─── Trait ───────────────────────────────────────────────────────────────────

pub fn fmt_trait(t: &TraitDef, ctx: &mut FmtContext) {
    ctx.write("trait ");
    ctx.write(&t.name.name);
    if let Some(g) = &t.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for sig in &t.items {
        ctx.emit_indent();
        fmt_function_sig(sig, ctx);
    }
    ctx.dedent();
    ctx.writeln("}");
}

fn fmt_function_sig(sig: &FunctionSig, ctx: &mut FmtContext) {
    ctx.write("fn ");
    ctx.write(&sig.name.name);
    if let Some(g) = &sig.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.write("(");
    for (i, p) in sig.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        fmt_param(p, ctx);
    }
    ctx.write(") -> ");
    fmt_type(&sig.return_type, ctx);
    ctx.writeln(";");
}

// ─── Impl ────────────────────────────────────────────────────────────────────

pub fn fmt_impl(i: &ImplBlock, ctx: &mut FmtContext) {
    ctx.write("impl");
    if let Some(g) = &i.generics {
        fmt_generic_params(g, ctx);
    }
    ctx.write(" ");
    if let Some(trait_name) = &i.trait_name {
        ctx.write(&trait_name.name.name);
        if let Some(ga) = &trait_name.generics {
            fmt_generic_args_inline(ga, ctx);
        }
        ctx.write(" for ");
    }
    ctx.write(&i.type_name.name.name);
    if let Some(ga) = &i.type_name.generics {
        fmt_generic_args_inline(ga, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for method in &i.items {
        ctx.emit_indent();
        fmt_function(method, ctx);
        ctx.newline();
    }
    ctx.dedent();
    ctx.writeln("}");
}

/// Format `<T, U, ...>` generic args inline.
fn fmt_generic_args_inline(ga: &lang4llm_ast::GenericArgs, ctx: &mut FmtContext) {
    ctx.write("<");
    for (i, t) in ga.args.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        fmt_type(t, ctx);
    }
    ctx.write(">");
}
