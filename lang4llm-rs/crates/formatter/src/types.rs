//! Type pretty-printing.

use lang4llm_ast::{GenericArgs, PrimitiveType, Type};

use crate::context::FmtContext;

/// Format a type annotation, appending to `ctx`.
pub fn fmt_type(ty: &Type, ctx: &mut FmtContext) {
    match ty {
        Type::Primitive(p, _) => ctx.write(primitive_str(*p)),
        Type::Array(inner, _) => {
            ctx.write("[");
            fmt_type(inner, ctx);
            ctx.write("]");
        }
        Type::Tuple(elems, _) => {
            ctx.write("(");
            for (i, t) in elems.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_type(t, ctx);
            }
            ctx.write(")");
        }
        Type::Generic(name, args, _) => {
            ctx.write(&name.name);
            if let Some(a) = args {
                fmt_generic_args(a, ctx);
            }
        }
        Type::Struct(name, _) | Type::Enum(name, _) => {
            ctx.write(&name.name);
        }
        Type::Function(ft) => {
            ctx.write("fn(");
            for (i, p) in ft.params.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                fmt_type(p, ctx);
            }
            ctx.write(") -> ");
            fmt_type(&ft.return_type, ctx);
        }
        Type::Inferred(_) => ctx.write("_"),
    }
}

fn fmt_generic_args(args: &GenericArgs, ctx: &mut FmtContext) {
    ctx.write("<");
    for (i, a) in args.args.iter().enumerate() {
        if i > 0 {
            ctx.write(", ");
        }
        fmt_type(a, ctx);
    }
    ctx.write(">");
}

fn primitive_str(p: PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Int => "int",
        PrimitiveType::Float => "float",
        PrimitiveType::Bool => "bool",
        PrimitiveType::String => "string",
        PrimitiveType::Unit => "unit",
    }
}
