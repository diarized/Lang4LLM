//! Tests for AST construction

use crate::*;

#[test]
fn test_span_creation() {
    let span = Span::new(1, 5, 1, 10);
    assert_eq!(span.start_line, 1);
    assert_eq!(span.start_col, 5);
    assert_eq!(span.end_line, 1);
    assert_eq!(span.end_col, 10);
}

#[test]
fn test_span_merge() {
    let span1 = Span::new(1, 5, 1, 10);
    let span2 = Span::new(1, 15, 1, 20);
    let merged = span1.merge(span2);

    assert_eq!(merged.start_line, 1);
    assert_eq!(merged.start_col, 5);
    assert_eq!(merged.end_line, 1);
    assert_eq!(merged.end_col, 20);
}

#[test]
fn test_span_display() {
    let span = Span::new(10, 5, 12, 20);
    assert_eq!(format!("{}", span), "10:5-12:20");
}

#[test]
fn test_binary_op_creation() {
    let lhs = Box::new(Expr::Literal(Literal::Int(1), Span::dummy()));
    let rhs = Box::new(Expr::Literal(Literal::Int(2), Span::dummy()));

    let expr = Expr::Binary(BinaryExpr {
        lhs,
        op: BinOp::Add,
        rhs,
        span: Span::dummy(),
    });

    match expr {
        Expr::Binary(BinaryExpr { op: BinOp::Add, .. }) => (),
        _ => panic!("Expected Binary::Add"),
    }
}

#[test]
fn test_function_def_creation() {
    let func = FunctionDef {
        name: Ident {
            name: "test".to_string(),
            span: Span::dummy(),
        },
        generics: None,
        params: vec![],
        return_type: Type::Primitive(PrimitiveType::Int, Span::dummy()),
        body: BlockExpr {
            stmts: vec![],
            expr: None,
            span: Span::dummy(),
        },
        span: Span::dummy(),
    };

    assert_eq!(func.name.name, "test");
    assert_eq!(func.params.len(), 0);
}

#[test]
fn test_let_statement_creation() {
    let stmt = Stmt::Let(LetStmt {
        mutable: false,
        name: Ident {
            name: "x".to_string(),
            span: Span::dummy(),
        },
        ty: Type::Primitive(PrimitiveType::Int, Span::dummy()),
        init: Some(Box::new(Expr::Literal(Literal::Int(42), Span::dummy()))),
        span: Span::dummy(),
    });

    match stmt {
        Stmt::Let(LetStmt { name, ty: Type::Primitive(PrimitiveType::Int, _), .. }) => {
            assert_eq!(name.name, "x");
        }
        _ => panic!("Expected Let statement"),
    }
}

#[test]
fn test_match_expression_creation() {
    let expr = Expr::Match(MatchExpr {
        expr: Box::new(Expr::Literal(Literal::Bool(true), Span::dummy())),
        arms: vec![
            MatchArm {
                pattern: Pattern::Literal(Literal::Bool(true), Span::dummy()),
                body: Box::new(Expr::Literal(Literal::Int(1), Span::dummy())),
                span: Span::dummy(),
            },
        ],
        span: Span::dummy(),
    });

    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert_eq!(arms.len(), 1);
        }
        _ => panic!("Expected Match expression"),
    }
}

#[test]
fn test_struct_def_creation() {
    let struct_def = StructDef {
        name: Ident {
            name: "Point".to_string(),
            span: Span::dummy(),
        },
        generics: None,
        fields: vec![
            FieldDef {
                name: Ident {
                    name: "x".to_string(),
                    span: Span::dummy(),
                },
                ty: Type::Primitive(PrimitiveType::Int, Span::dummy()),
                span: Span::dummy(),
            },
            FieldDef {
                name: Ident {
                    name: "y".to_string(),
                    span: Span::dummy(),
                },
                ty: Type::Primitive(PrimitiveType::Int, Span::dummy()),
                span: Span::dummy(),
            },
        ],
        span: Span::dummy(),
    };

    assert_eq!(struct_def.name.name, "Point");
    assert_eq!(struct_def.fields.len(), 2);
    assert_eq!(struct_def.fields[0].name.name, "x");
    assert_eq!(struct_def.fields[1].name.name, "y");
}

#[test]
fn test_enum_def_creation() {
    let enum_def = EnumDef {
        name: Ident {
            name: "Option".to_string(),
            span: Span::dummy(),
        },
        generics: Some(GenericParams {
            params: vec![GenericParamDef {
                name: Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                bound: None,
            }],
            span: Span::dummy(),
        }),
        variants: vec![
            Variant {
                name: Ident {
                    name: "Some".to_string(),
                    span: Span::dummy(),
                },
                data: Some(vec![Type::Generic(
                    Ident {
                        name: "T".to_string(),
                        span: Span::dummy(),
                    },
                    None,
                    Span::dummy()
                )]),
                span: Span::dummy(),
            },
            Variant {
                name: Ident {
                    name: "None".to_string(),
                    span: Span::dummy(),
                },
                data: None,
                span: Span::dummy(),
            },
        ],
        span: Span::dummy(),
    };

    assert_eq!(enum_def.name.name, "Option");
    assert_eq!(enum_def.variants.len(), 2);
    assert_eq!(enum_def.variants[0].name.name, "Some");
    assert_eq!(enum_def.variants[1].name.name, "None");
}

#[test]
fn test_type_variants() {
    let int_type = Type::Primitive(PrimitiveType::Int, Span::dummy());
    assert!(matches!(int_type, Type::Primitive(PrimitiveType::Int, _)));

    let float_type = Type::Primitive(PrimitiveType::Float, Span::dummy());
    assert!(matches!(float_type, Type::Primitive(PrimitiveType::Float, _)));

    let bool_type = Type::Primitive(PrimitiveType::Bool, Span::dummy());
    assert!(matches!(bool_type, Type::Primitive(PrimitiveType::Bool, _)));

    let string_type = Type::Primitive(PrimitiveType::String, Span::dummy());
    assert!(matches!(string_type, Type::Primitive(PrimitiveType::String, _)));

    let array_type = Type::Array(
        Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        Span::dummy()
    );
    assert!(matches!(array_type, Type::Array(_, _)));

    let tuple_type = Type::Tuple(
        vec![
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy())
        ],
        Span::dummy()
    );
    assert!(matches!(tuple_type, Type::Tuple(_, _)));
}

#[test]
fn test_pattern_variants() {
    let lit_pattern = Pattern::Literal(Literal::Int(42), Span::dummy());
    assert!(matches!(lit_pattern, Pattern::Literal(_, _)));

    let ident_pattern = Pattern::Ident(
        Ident {
            name: "x".to_string(),
            span: Span::dummy(),
        },
        Span::dummy()
    );
    assert!(matches!(ident_pattern, Pattern::Ident(_, _)));

    let wildcard_pattern = Pattern::Wildcard(Span::dummy());
    assert!(matches!(wildcard_pattern, Pattern::Wildcard(_)));
}

#[test]
fn test_program_creation() {
    let program = Program {
        module: Some(ModuleDecl {
            name: Ident {
                name: "main".to_string(),
                span: Span::dummy(),
            },
            span: Span::dummy(),
        }),
        imports: vec![],
        items: vec![
            Item::Function(FunctionDef {
                name: Ident {
                    name: "main".to_string(),
                    span: Span::dummy(),
                },
                generics: None,
                params: vec![],
                return_type: Type::Primitive(PrimitiveType::Int, Span::dummy()),
                body: BlockExpr {
                    stmts: vec![],
                    expr: None,
                    span: Span::dummy(),
                },
                span: Span::dummy(),
            }),
        ],
        span: Span::dummy(),
    };

    assert!(program.module.is_some());
    assert_eq!(program.items.len(), 1);
}
