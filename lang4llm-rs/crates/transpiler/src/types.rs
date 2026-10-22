//! Type mapping from Lang4LLM types to Rust types.

use lang4llm_ast::{FunctionType, GenericArgs, PrimitiveType, Type};

/// Emit a Lang4LLM type as its Rust equivalent string.
///
/// `in_param_position` controls whether `fn(T)->U` becomes `impl Fn(T)->U`
/// (parameter position) or `Box<dyn Fn(T)->U>` (return/field position).
pub fn emit_type(ty: &Type) -> String {
    emit_type_inner(ty, true)
}

/// Emit a type in return/field position (function types become `Box<dyn Fn>`).
pub fn emit_type_return(ty: &Type) -> String {
    emit_type_inner(ty, false)
}

fn emit_type_inner(ty: &Type, param_position: bool) -> String {
    match ty {
        Type::Primitive(prim, _) => emit_primitive(*prim),
        Type::Array(inner, _) => format!("Vec<{}>", emit_type_inner(inner, param_position)),
        Type::Tuple(types, _) => {
            let inner: Vec<String> = types.iter().map(|t| emit_type_inner(t, param_position)).collect();
            format!("({})", inner.join(", "))
        }
        Type::Generic(ident, args, _) => {
            if let Some(args) = args {
                format!("{}<{}>", ident.name, emit_generic_args(args))
            } else {
                ident.name.clone()
            }
        }
        Type::Struct(ident, _) => ident.name.clone(),
        Type::Enum(ident, _) => ident.name.clone(),
        Type::Function(ft) => emit_function_type(ft, param_position),
        Type::Inferred(_) => "_".to_string(),
    }
}

fn emit_primitive(prim: PrimitiveType) -> String {
    match prim {
        PrimitiveType::Int => "i64".to_string(),
        PrimitiveType::Float => "f64".to_string(),
        PrimitiveType::Bool => "bool".to_string(),
        PrimitiveType::String => "String".to_string(),
        PrimitiveType::Unit => "()".to_string(),
    }
}

fn emit_function_type(ft: &FunctionType, param_position: bool) -> String {
    let params: Vec<String> = ft.params.iter().map(|t| emit_type_inner(t, true)).collect();
    let ret = emit_type_inner(&ft.return_type, false);
    let sig = if ret == "()" {
        format!("Fn({})", params.join(", "))
    } else {
        format!("Fn({}) -> {}", params.join(", "), ret)
    };
    if param_position {
        format!("impl {}", sig)
    } else {
        format!("Box<dyn {}>", sig)
    }
}

fn emit_generic_args(args: &GenericArgs) -> String {
    args.args
        .iter()
        .map(|t| emit_type_inner(t, true))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang4llm_ast::{GenericArgs, Ident, PrimitiveType, Span, Type};

    fn dummy_span() -> Span {
        Span::new(0, 0, 0, 0)
    }

    fn ident(name: &str) -> Ident {
        Ident::new(name.to_string(), dummy_span())
    }

    #[test]
    fn test_primitives() {
        assert_eq!(emit_type(&Type::Primitive(PrimitiveType::Int, dummy_span())), "i64");
        assert_eq!(emit_type(&Type::Primitive(PrimitiveType::Float, dummy_span())), "f64");
        assert_eq!(emit_type(&Type::Primitive(PrimitiveType::Bool, dummy_span())), "bool");
        assert_eq!(emit_type(&Type::Primitive(PrimitiveType::String, dummy_span())), "String");
        assert_eq!(emit_type(&Type::Primitive(PrimitiveType::Unit, dummy_span())), "()");
    }

    #[test]
    fn test_array() {
        let inner = Type::Primitive(PrimitiveType::Int, dummy_span());
        let ty = Type::Array(Box::new(inner), dummy_span());
        assert_eq!(emit_type(&ty), "Vec<i64>");
    }

    #[test]
    fn test_tuple() {
        let ty = Type::Tuple(
            vec![
                Type::Primitive(PrimitiveType::Int, dummy_span()),
                Type::Primitive(PrimitiveType::String, dummy_span()),
            ],
            dummy_span(),
        );
        assert_eq!(emit_type(&ty), "(i64, String)");
    }

    #[test]
    fn test_generic_no_args() {
        let ty = Type::Generic(ident("Option"), None, dummy_span());
        assert_eq!(emit_type(&ty), "Option");
    }

    #[test]
    fn test_generic_with_args() {
        let args = GenericArgs {
            args: vec![Type::Primitive(PrimitiveType::Int, dummy_span())],
            span: dummy_span(),
        };
        let ty = Type::Generic(ident("Option"), Some(args), dummy_span());
        assert_eq!(emit_type(&ty), "Option<i64>");
    }

    #[test]
    fn test_function_type_param() {
        use lang4llm_ast::FunctionType;
        let ft = FunctionType {
            params: vec![Type::Primitive(PrimitiveType::Int, dummy_span())],
            return_type: Box::new(Type::Primitive(PrimitiveType::Bool, dummy_span())),
            span: dummy_span(),
        };
        let ty = Type::Function(ft);
        assert_eq!(emit_type(&ty), "impl Fn(i64) -> bool");
    }

    #[test]
    fn test_inferred() {
        let ty = Type::Inferred(dummy_span());
        assert_eq!(emit_type(&ty), "_");
    }
}
