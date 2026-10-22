//! Parser integration tests
//!
//! Tests the parser against Lang4LLM source code, verifying AST output.

use lang4llm_ast::SourceMap;
use lang4llm_parser::parse;

/// Helper: parse and assert success
fn parse_ok(source: &str) -> lang4llm_ast::Program {
    let sm = SourceMap::from_source(source);
    match parse(source, &sm) {
        Ok(program) => program,
        Err(e) => panic!("Parse failed: {e}\nSource:\n{source}"),
    }
}

/// Helper: parse and assert failure
fn parse_err(source: &str) {
    let sm = SourceMap::from_source(source);
    assert!(parse(source, &sm).is_err(), "Expected parse error for: {source}");
}

// ============================================================================
// Basic parsing tests
// ============================================================================

#[test]
fn test_empty_program() {
    let prog = parse_ok("");
    assert!(prog.module.is_none());
    assert!(prog.imports.is_empty());
    assert!(prog.items.is_empty());
}

#[test]
fn test_minimal_function() {
    let prog = parse_ok("fn main() -> unit { return none; }");
    assert_eq!(prog.items.len(), 1);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            assert_eq!(f.name.name, "main");
            assert!(f.generics.is_none());
            assert!(f.params.is_empty());
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_with_params() {
    let prog = parse_ok("fn add(a: int, b: int) -> int { return a; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            assert_eq!(f.params.len(), 2);
            assert_eq!(f.name.name, "add");
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_generic_function() {
    let prog = parse_ok("fn identity<T>(x: T) -> T { return x; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            assert!(f.generics.is_some());
            let gp = f.generics.as_ref().unwrap();
            assert_eq!(gp.params.len(), 1);
            assert_eq!(gp.params[0].name.name, "T");
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Statement tests
// ============================================================================

#[test]
fn test_let_statement() {
    let prog = parse_ok("fn main() -> unit { let x: int = 42; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            assert_eq!(f.body.stmts.len(), 1);
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Let(s) => {
                    assert!(!s.mutable);
                    assert_eq!(s.name.name, "x");
                }
                other => panic!("Expected Let, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_let_mut_statement() {
    let prog = parse_ok("fn main() -> unit { let mut x: int = 0; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Let(s) => assert!(s.mutable),
                other => panic!("Expected Let, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_assign_statement() {
    let prog = parse_ok("fn main() -> unit { x = 5; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Assign(s) => {
                    match &s.lhs {
                        lang4llm_ast::AssignTarget::Var(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        other => panic!("Expected Var target, got {other:?}"),
                    }
                }
                other => panic!("Expected Assign, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_return_with_value() {
    let prog = parse_ok("fn main() -> int { return 42; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => assert!(s.value.is_some()),
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_return_no_value() {
    let prog = parse_ok("fn main() -> unit { return; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => assert!(s.value.is_none()),
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Expression tests
// ============================================================================

#[test]
fn test_binary_arithmetic() {
    let prog = parse_ok("fn main() -> int { return 1 + 2 * 3; }");
    // Should parse as 1 + (2 * 3) due to precedence
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    let val = s.value.as_ref().unwrap();
                    match val.as_ref() {
                        lang4llm_ast::Expr::Binary(b) => {
                            assert_eq!(b.op, lang4llm_ast::BinOp::Add);
                            // RHS should be Mul
                            match b.rhs.as_ref() {
                                lang4llm_ast::Expr::Binary(inner) => {
                                    assert_eq!(inner.op, lang4llm_ast::BinOp::Mul);
                                }
                                other => panic!("Expected Binary Mul, got {other:?}"),
                            }
                        }
                        other => panic!("Expected Binary Add, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_comparison() {
    let prog = parse_ok("fn main() -> bool { return a < b; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Binary(b) => {
                            assert_eq!(b.op, lang4llm_ast::BinOp::Lt);
                        }
                        other => panic!("Expected Binary Lt, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_logical_operators() {
    let prog = parse_ok("fn main() -> bool { return a and b or c; }");
    // Should parse as (a and b) or c
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Binary(b) => {
                            assert_eq!(b.op, lang4llm_ast::BinOp::Or);
                        }
                        other => panic!("Expected Binary Or, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_unary_not() {
    let prog = parse_ok("fn main() -> bool { return not true; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Unary(u) => {
                            assert_eq!(u.op, lang4llm_ast::UnOp::Not);
                        }
                        other => panic!("Expected Unary Not, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_call() {
    let prog = parse_ok("fn main() -> unit { print(42); }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Expr(s) => {
                    match s.expr.as_ref() {
                        lang4llm_ast::Expr::Call(c) => {
                            assert_eq!(c.args.len(), 1);
                        }
                        other => panic!("Expected Call, got {other:?}"),
                    }
                }
                other => panic!("Expected Expr, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_array_literal() {
    let prog = parse_ok("fn main() -> [int] { return [1, 2, 3]; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Array(a) => {
                            assert_eq!(a.elements.len(), 3);
                        }
                        other => panic!("Expected Array, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_if_else() {
    let prog = parse_ok("fn main() -> int { return if true { 1 } else { 0 }; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::If(_) => {}
                        other => panic!("Expected If, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_match_expression() {
    let prog = parse_ok(r#"
        fn main() -> int {
            return match x {
                1 => 10,
                2 => 20,
                _ => 0,
            };
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Match(m) => {
                            assert_eq!(m.arms.len(), 3);
                        }
                        other => panic!("Expected Match, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Type definition tests
// ============================================================================

#[test]
fn test_struct_def() {
    let prog = parse_ok("struct Point { x: float, y: float }");
    match &prog.items[0] {
        lang4llm_ast::Item::Struct(s) => {
            assert_eq!(s.name.name, "Point");
            assert_eq!(s.fields.len(), 2);
            assert!(s.generics.is_none());
        }
        other => panic!("Expected Struct, got {other:?}"),
    }
}

#[test]
fn test_generic_struct() {
    let prog = parse_ok("struct Pair<T, U> { first: T, second: U }");
    match &prog.items[0] {
        lang4llm_ast::Item::Struct(s) => {
            assert!(s.generics.is_some());
            let gp = s.generics.as_ref().unwrap();
            assert_eq!(gp.params.len(), 2);
        }
        other => panic!("Expected Struct, got {other:?}"),
    }
}

#[test]
fn test_enum_def() {
    let prog = parse_ok("enum Color { Red, Green, Blue }");
    match &prog.items[0] {
        lang4llm_ast::Item::Enum(e) => {
            assert_eq!(e.name.name, "Color");
            assert_eq!(e.variants.len(), 3);
            assert!(e.variants[0].data.is_none());
        }
        other => panic!("Expected Enum, got {other:?}"),
    }
}

#[test]
fn test_enum_with_data() {
    let prog = parse_ok("enum Option<T> { Some(T), None }");
    match &prog.items[0] {
        lang4llm_ast::Item::Enum(e) => {
            assert_eq!(e.variants.len(), 2);
            assert!(e.variants[0].data.is_some()); // Some(T) has data
            assert!(e.variants[1].data.is_none());  // None has no data
        }
        other => panic!("Expected Enum, got {other:?}"),
    }
}

#[test]
fn test_trait_def() {
    let prog = parse_ok(r#"
        trait Display {
            fn to_string(self: Display) -> string;
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Trait(t) => {
            assert_eq!(t.name.name, "Display");
            assert_eq!(t.items.len(), 1);
        }
        other => panic!("Expected Trait, got {other:?}"),
    }
}

#[test]
fn test_impl_block() {
    let prog = parse_ok(r#"
        impl Point {
            fn new(x: float, y: float) -> Point {
                return x;
            }
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Impl(i) => {
            assert!(i.trait_name.is_none());
            assert_eq!(i.type_name.name.name, "Point");
            assert_eq!(i.items.len(), 1);
        }
        other => panic!("Expected Impl, got {other:?}"),
    }
}

#[test]
fn test_trait_impl() {
    let prog = parse_ok(r#"
        impl Display for Point {
            fn to_string(self: Point) -> string {
                return "point";
            }
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Impl(i) => {
            assert!(i.trait_name.is_some());
            assert_eq!(i.trait_name.as_ref().unwrap().name.name, "Display");
            assert_eq!(i.type_name.name.name, "Point");
        }
        other => panic!("Expected Impl, got {other:?}"),
    }
}

// ============================================================================
// Control flow tests
// ============================================================================

#[test]
fn test_while_loop() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            while x < 10 {
                x = x + 1;
            }
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::While(_) => {}
                other => panic!("Expected While, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_for_loop() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            for i in items {
                print(i);
            }
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::For(s) => {
                    assert_eq!(s.var.name, "i");
                }
                other => panic!("Expected For, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_loop_with_break() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            loop {
                break;
            }
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Loop(_) => {}
                other => panic!("Expected Loop, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Module system tests
// ============================================================================

#[test]
fn test_module_decl() {
    let prog = parse_ok("module mylib;");
    assert!(prog.module.is_some());
    assert_eq!(prog.module.unwrap().name.name, "mylib");
}

#[test]
fn test_import() {
    let prog = parse_ok("import std.io;");
    assert_eq!(prog.imports.len(), 1);
    assert_eq!(prog.imports[0].path.len(), 2);
    assert_eq!(prog.imports[0].path[0].name, "std");
    assert_eq!(prog.imports[0].path[1].name, "io");
}

// ============================================================================
// Postfix expression tests
// ============================================================================

#[test]
fn test_field_access() {
    let prog = parse_ok("fn main() -> int { return point.x; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::FieldAccess(fa) => {
                            assert_eq!(fa.field.name, "x");
                        }
                        other => panic!("Expected FieldAccess, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_array_access() {
    let prog = parse_ok("fn main() -> int { return arr[0]; }");
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::ArrayAccess(_) => {}
                        other => panic!("Expected ArrayAccess, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_method_call() {
    let prog = parse_ok("fn main() -> string { return obj.to_string(); }");
    // obj.to_string() = Call(FieldAccess(Variable(obj), "to_string"), [])
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Call(c) => {
                            match c.func.as_ref() {
                                lang4llm_ast::Expr::FieldAccess(fa) => {
                                    assert_eq!(fa.field.name, "to_string");
                                }
                                other => panic!("Expected FieldAccess, got {other:?}"),
                            }
                        }
                        other => panic!("Expected Call, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Re-added feature tests: Struct Literals
// ============================================================================

#[test]
fn test_struct_literal() {
    let prog = parse_ok(r#"
        fn main() -> Point {
            return Point { x: 1, y: 2 };
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Struct(sl) => {
                            assert_eq!(sl.name.name, "Point");
                            assert_eq!(sl.fields.len(), 2);
                            assert_eq!(sl.fields[0].name.name, "x");
                            assert_eq!(sl.fields[1].name.name, "y");
                        }
                        other => panic!("Expected Struct literal, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_struct_literal_in_let() {
    parse_ok(r#"
        fn main() -> unit {
            let p: Point = Point { x: 10, y: 20 };
        }
    "#);
}

#[test]
fn test_nested_struct_literal() {
    parse_ok(r#"
        fn main() -> Rect {
            return Rect { top_left: Point { x: 0, y: 0 }, width: 10 };
        }
    "#);
}

// ============================================================================
// Re-added feature tests: Enum Variant Expressions
// ============================================================================

#[test]
fn test_enum_variant_no_args() {
    let prog = parse_ok(r#"
        fn main() -> Option {
            return Option::None;
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::EnumVariant(ev) => {
                            assert_eq!(ev.enum_name.name, "Option");
                            assert_eq!(ev.variant_name.name, "None");
                            assert!(ev.args.is_none());
                        }
                        other => panic!("Expected EnumVariant, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_enum_variant_with_args() {
    let prog = parse_ok(r#"
        fn main() -> Option {
            return Option::Some(42);
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::EnumVariant(ev) => {
                            assert_eq!(ev.enum_name.name, "Option");
                            assert_eq!(ev.variant_name.name, "Some");
                            assert!(ev.args.is_some());
                            assert_eq!(ev.args.as_ref().unwrap().len(), 1);
                        }
                        other => panic!("Expected EnumVariant, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

// ============================================================================
// Re-added feature tests: Tuple Literals and Types
// ============================================================================

#[test]
fn test_tuple_literal() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            return (1, 2, 3);
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Tuple(t) => {
                            assert_eq!(t.elements.len(), 3);
                        }
                        other => panic!("Expected Tuple, got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_parenthesized_expr_not_tuple() {
    // (1) should be parenthesized expression, NOT a tuple
    let prog = parse_ok(r#"
        fn main() -> int {
            return (1 + 2);
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Return(s) => {
                    match s.value.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Binary(_) => {} // (1 + 2) = Binary
                        other => panic!("Expected Binary (parenthesized), got {other:?}"),
                    }
                }
                other => panic!("Expected Return, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_tuple_type() {
    parse_ok(r#"
        fn swap(pair: (int, string)) -> (string, int) {
            return ("hello", 42);
        }
    "#);
}

// ============================================================================
// Re-added feature tests: Closures
// ============================================================================

#[test]
fn test_closure_basic() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            let f: fn(int) -> int = |x: int| { x + 1 };
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Let(s) => {
                    match s.init.as_ref().unwrap().as_ref() {
                        lang4llm_ast::Expr::Closure(c) => {
                            assert_eq!(c.params.len(), 1);
                        }
                        other => panic!("Expected Closure, got {other:?}"),
                    }
                }
                other => panic!("Expected Let, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_closure_no_params() {
    parse_ok(r#"
        fn main() -> unit {
            let f: fn() -> int = || { 42 };
        }
    "#);
}

#[test]
fn test_closure_multiple_params() {
    parse_ok(r#"
        fn main() -> unit {
            let f: fn(int, int) -> int = |a: int, b: int| { a + b };
        }
    "#);
}

// ============================================================================
// Re-added feature tests: Complex Patterns
// ============================================================================

#[test]
fn test_enum_pattern_with_qualifier() {
    parse_ok(r#"
        fn main() -> int {
            return match opt {
                Option::Some(v) => v,
                Option::None => 0,
            };
        }
    "#);
}

#[test]
fn test_enum_pattern_without_qualifier() {
    parse_ok(r#"
        fn main() -> int {
            return match opt {
                Some(v) => v,
                _ => 0,
            };
        }
    "#);
}

#[test]
fn test_struct_pattern() {
    parse_ok(r#"
        fn main() -> int {
            return match p {
                Point { x: px, y: py } => px + py,
                _ => 0,
            };
        }
    "#);
}

#[test]
fn test_tuple_pattern() {
    parse_ok(r#"
        fn main() -> int {
            return match pair {
                (a, b) => a + b,
                _ => 0,
            };
        }
    "#);
}

// ============================================================================
// Re-added feature tests: Complex Assign Targets
// ============================================================================

#[test]
fn test_assign_field_access() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            p.x = 10;
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Assign(s) => {
                    match &s.lhs {
                        lang4llm_ast::AssignTarget::FieldAccess(_, field, _) => {
                            assert_eq!(field.name, "x");
                        }
                        other => panic!("Expected FieldAccess target, got {other:?}"),
                    }
                }
                other => panic!("Expected Assign, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_assign_array_access() {
    let prog = parse_ok(r#"
        fn main() -> unit {
            arr[0] = 42;
        }
    "#);
    match &prog.items[0] {
        lang4llm_ast::Item::Function(f) => {
            match &f.body.stmts[0] {
                lang4llm_ast::Stmt::Assign(s) => {
                    match &s.lhs {
                        lang4llm_ast::AssignTarget::ArrayAccess(_, _, _) => {}
                        other => panic!("Expected ArrayAccess target, got {other:?}"),
                    }
                }
                other => panic!("Expected Assign, got {other:?}"),
            }
        }
        other => panic!("Expected Function, got {other:?}"),
    }
}

#[test]
fn test_assign_nested_field() {
    parse_ok(r#"
        fn main() -> unit {
            rect.top_left.x = 0;
        }
    "#);
}

#[test]
fn test_invalid_assign_target() {
    // Cannot assign to a literal
    parse_err(r#"
        fn main() -> unit {
            42 = 10;
        }
    "#);
}

// ============================================================================
// Re-added feature tests: self keyword in expressions
// ============================================================================

#[test]
fn test_self_in_method() {
    parse_ok(r#"
        impl Point {
            fn get_x(self: Point) -> int {
                return self.x;
            }
        }
    "#);
}

// ============================================================================
// Re-added feature tests: Struct literal restriction in conditions
// ============================================================================

#[test]
fn test_if_no_struct_literal_in_condition() {
    // "if x { }" should parse x as variable, { } as block (not struct literal)
    parse_ok(r#"
        fn main() -> int {
            return if x { 1 } else { 2 };
        }
    "#);
}

#[test]
fn test_while_no_struct_literal_in_condition() {
    parse_ok(r#"
        fn main() -> unit {
            while flag {
                flag = false;
            }
        }
    "#);
}

// ============================================================================
// Combined feature tests: Full programs
// ============================================================================

#[test]
fn test_full_pattern_matching_example() {
    parse_ok(r#"
        enum Option {
            Some(int),
            None
        }

        fn unwrap_or(opt: Option, default: int) -> int {
            return match opt {
                Option::Some(value) => value,
                Option::None => default,
            };
        }
    "#);
}

#[test]
fn test_full_struct_with_methods() {
    parse_ok(r#"
        struct Point {
            x: int,
            y: int
        }

        impl Point {
            fn new(x: int, y: int) -> Point {
                return Point { x: x, y: y };
            }

            fn get_x(self: Point) -> int {
                return self.x;
            }
        }
    "#);
}

// ============================================================================
// Error tests
// ============================================================================

#[test]
fn test_missing_semicolon() {
    parse_err("fn main() -> unit { return none }");
}

#[test]
fn test_missing_return_type() {
    parse_err("fn main() { return none; }");
}

// ============================================================================
// Example program tests (subset that works with current grammar)
// ============================================================================

#[test]
fn test_parse_hello_world() {
    let source = r#"
        fn main() -> unit {
            print("Hello, World!");
            return none;
        }
    "#;
    let prog = parse_ok(source);
    assert_eq!(prog.items.len(), 1);
}

#[test]
fn test_parse_fibonacci() {
    let source = r#"
        fn fib(n: int) -> int {
            let mut a: int = 0;
            let mut b: int = 1;

            for i in range(0, n) {
                let temp: int = a;
                a = b;
                b = temp + b;
            }

            return a;
        }

        fn main() -> unit {
            let result: int = fib(10);
            print("Fibonacci(10) = " + result);
            return none;
        }
    "#;
    let prog = parse_ok(source);
    assert_eq!(prog.items.len(), 2);
}

#[test]
fn test_parse_struct_def() {
    let source = r#"
        struct Point {
            x: float,
            y: float
        }

        impl Point {
            fn new(x: float, y: float) -> Point {
                return x;
            }
        }
    "#;
    let prog = parse_ok(source);
    assert_eq!(prog.items.len(), 2);
}
