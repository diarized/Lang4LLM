//! Top-level item code generation: functions, structs, enums, traits, impls.

use lang4llm_ast::{
    EnumDef, FunctionDef, FunctionSig, GenericParams, ImplBlock, Item, ParamName, PrimitiveType,
    StructDef, TraitDef, Type,
};

use crate::context::CodegenContext;
use crate::exprs::emit_expr;
use crate::stmts::emit_stmt;
use crate::types::{emit_type, emit_type_return};

/// Emit a top-level item.
pub fn emit_item(item: &Item, ctx: &mut CodegenContext) {
    match item {
        Item::Function(f) => emit_function(f, ctx),
        Item::Struct(s) => emit_struct(s, ctx),
        Item::Enum(e) => emit_enum(e, ctx),
        Item::Trait(t) => emit_trait(t, ctx),
        Item::Impl(i) => emit_impl(i, ctx),
    }
    ctx.newline();
}

/// Emit a function definition.
pub fn emit_function(f: &FunctionDef, ctx: &mut CodegenContext) {
    ctx.emit_indent();
    ctx.write("fn ");
    ctx.write(&f.name.name);

    // Collect where-clause bounds from GenericParams
    let mut where_clauses: Vec<(String, Vec<String>)> = Vec::new();

    // Generic params
    if let Some(generics) = &f.generics {
        emit_generic_params(generics, ctx);
        for param in &generics.params {
            if let Some(bound) = &param.bound {
                where_clauses.push((param.name.name.clone(), vec![bound.name.clone()]));
            }
        }
    }

    // Ensure every generic param has Clone bound (emit_call_arg always inserts .clone())
    if let Some(generics) = &f.generics {
        for param in &generics.params {
            let name = param.name.name.clone();
            if let Some(entry) = where_clauses.iter_mut().find(|(n, _)| n == &name) {
                if !entry.1.contains(&"Clone".to_string()) {
                    entry.1.push("Clone".to_string());
                }
            } else {
                where_clauses.push((name, vec!["Clone".to_string()]));
            }
        }
    }

    ctx.write("(");
    for (i, param) in f.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        match &param.name {
            ParamName::SelfParam => ctx.write("self"),
            ParamName::Ident(id) => {
                ctx.write(&id.name);
                ctx.write(": ");
                ctx.write(&emit_type(&param.ty));
            }
            ParamName::GenericBound(id, bounds) => {
                // param name: type (e.g., `value: T`)
                ctx.write(&id.name);
                ctx.write(": ");
                ctx.write(&emit_type(&param.ty));
                // Collect bounds for where clause (e.g., T: Display)
                if !bounds.is_empty() {
                    let type_name = emit_type(&param.ty);
                    let bound_names: Vec<String> = bounds.iter().map(|b| b.name.clone()).collect();
                    where_clauses.push((type_name, bound_names));
                }
            }
        }
    }
    ctx.write(")");

    // Return type (omit for unit/main)
    let is_main = f.name.name == "main";
    let is_unit = matches!(f.return_type, Type::Primitive(PrimitiveType::Unit, _));
    if !is_unit && !is_main {
        ctx.write(" -> ");
        ctx.write(&emit_type_return(&f.return_type));
    }

    // Emit where clause or opening brace
    if !where_clauses.is_empty() {
        ctx.newline();
        ctx.emit_indent();
        ctx.write("where");
        ctx.newline();
        for (ty, bounds) in &where_clauses {
            ctx.emit_indent();
            ctx.write("    ");
            ctx.write(ty);
            ctx.write(": ");
            ctx.write(&bounds.join(" + "));
            ctx.writeln(",");
        }
        ctx.emit_indent();
        ctx.writeln("{");
    } else {
        ctx.writeln(" {");
    }
    ctx.indent();
    emit_block_body(&f.body, ctx);
    ctx.dedent();
    ctx.emit_indent();
    ctx.writeln("}");
}

/// Emit a struct definition with derive macros.
pub fn emit_struct(s: &StructDef, ctx: &mut CodegenContext) {
    ctx.emit_line("#[derive(Debug, Clone)]");
    ctx.emit_indent();
    ctx.write("struct ");
    ctx.write(&s.name.name);
    if let Some(generics) = &s.generics {
        emit_generic_params(generics, ctx);
    }
    if s.fields.is_empty() {
        ctx.writeln(";");
    } else {
        ctx.writeln(" {");
        ctx.indent();
        for field in &s.fields {
            ctx.emit_indent();
            ctx.write(&field.name.name);
            ctx.write(": ");
            ctx.write(&emit_type(&field.ty));
            ctx.writeln(",");
        }
        ctx.dedent();
        ctx.emit_indent();
        ctx.writeln("}");
    }
}

/// Emit an enum definition with derive macros.
pub fn emit_enum(e: &EnumDef, ctx: &mut CodegenContext) {
    ctx.emit_line("#[derive(Debug, Clone)]");
    ctx.emit_indent();
    ctx.write("enum ");
    ctx.write(&e.name.name);
    if let Some(generics) = &e.generics {
        emit_generic_params(generics, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for variant in &e.variants {
        ctx.emit_indent();
        ctx.write(&variant.name.name);
        if let Some(data) = &variant.data {
            ctx.write("(");
            for (i, ty) in data.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                ctx.write(&emit_type(ty));
            }
            ctx.write(")");
        }
        ctx.writeln(",");
    }
    ctx.dedent();
    ctx.emit_indent();
    ctx.writeln("}");
}

/// Emit a trait definition.
pub fn emit_trait(t: &TraitDef, ctx: &mut CodegenContext) {
    ctx.emit_indent();
    ctx.write("trait ");
    ctx.write(&t.name.name);
    if let Some(generics) = &t.generics {
        emit_generic_params(generics, ctx);
    }
    ctx.writeln(" {");
    ctx.indent();
    for sig in &t.items {
        emit_function_sig(sig, ctx);
    }
    ctx.dedent();
    ctx.emit_indent();
    ctx.writeln("}");
}

/// Emit an impl block.
pub fn emit_impl(i: &ImplBlock, ctx: &mut CodegenContext) {
    ctx.emit_indent();
    ctx.write("impl");
    if let Some(generics) = &i.generics {
        emit_generic_params(generics, ctx);
    }
    ctx.write(" ");
    if let Some(trait_name) = &i.trait_name {
        ctx.write(&trait_name.name.name);
        if let Some(args) = &trait_name.generics {
            ctx.write("<");
            for (j, ty) in args.args.iter().enumerate() {
                if j > 0 {
                    ctx.write(", ");
                }
                ctx.write(&emit_type(ty));
            }
            ctx.write(">");
        }
        ctx.write(" for ");
    }
    ctx.write(&i.type_name.name.name);
    if let Some(args) = &i.type_name.generics {
        ctx.write("<");
        for (j, ty) in args.args.iter().enumerate() {
            if j > 0 {
                ctx.write(", ");
            }
            ctx.write(&emit_type(ty));
        }
        ctx.write(">");
    }
    ctx.writeln(" {");
    ctx.indent();
    for method in &i.items {
        emit_function(method, ctx);
        ctx.newline();
    }
    ctx.dedent();
    ctx.emit_indent();
    ctx.writeln("}");
}

// =============================================================================
// Helpers
// =============================================================================

fn emit_generic_params(generics: &GenericParams, ctx: &mut CodegenContext) {
    ctx.write("<");
    for (i, param) in generics.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        ctx.write(&param.name.name);
    }
    ctx.write(">");
}

fn emit_function_sig(sig: &FunctionSig, ctx: &mut CodegenContext) {
    ctx.emit_indent();
    ctx.write("fn ");
    ctx.write(&sig.name.name);
    if let Some(generics) = &sig.generics {
        emit_generic_params(generics, ctx);
    }
    ctx.write("(");
    for (i, param) in sig.params.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        match &param.name {
            ParamName::SelfParam => ctx.write("self"),
            ParamName::Ident(id) => {
                ctx.write(&id.name);
                ctx.write(": ");
                ctx.write(&emit_type(&param.ty));
            }
            ParamName::GenericBound(id, _) => {
                ctx.write(&id.name);
                ctx.write(": ");
                ctx.write(&emit_type(&param.ty));
            }
        }
    }
    ctx.write(")");
    let is_unit = matches!(sig.return_type, Type::Primitive(PrimitiveType::Unit, _));
    if !is_unit {
        ctx.write(" -> ");
        ctx.write(&emit_type_return(&sig.return_type));
    }
    ctx.writeln(";");
}

fn emit_block_body(block: &lang4llm_ast::BlockExpr, ctx: &mut CodegenContext) {
    for stmt in &block.stmts {
        emit_stmt(stmt, ctx);
    }
    if let Some(e) = &block.expr {
        ctx.emit_indent();
        emit_expr(e, ctx);
        ctx.newline();
    }
}
