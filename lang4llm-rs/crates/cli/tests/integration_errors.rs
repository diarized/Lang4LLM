//! Integration tests for error detection and reporting
//!
//! These tests verify that the compiler correctly detects errors and produces
//! helpful diagnostic messages for common mistakes.

/// Helper function to verify that parsing fails with an error
fn expect_parse_error(source: &str, description: &str) {
    let sm = lang4llm_ast::SourceMap::from_source(source);
    match lang4llm_parser::parse(source, &sm) {
        Ok(_) => panic!("{}: Expected parse error, but parsing succeeded", description),
        Err(e) => {
            println!("✓ {}: Parse error detected: {:?}", description, e);
        }
    }
}

/// Helper function to verify that type checking fails with an error
fn expect_type_error(source: &str, description: &str) {
    let sm = lang4llm_ast::SourceMap::from_source(source);
    let program = match lang4llm_parser::parse(source, &sm) {
        Ok(p) => p,
        Err(e) => panic!("{}: Parse error (expected type error): {:?}", description, e),
    };

    match lang4llm_typeck::check_program(&program) {
        Ok(()) => panic!("{}: Expected type error, but type checking succeeded", description),
        Err(errors) => {
            println!("✓ {}: {} type error(s) detected", description, errors.len());
            for (i, err) in errors.iter().enumerate() {
                println!("  Error {}: {:?}", i + 1, err);
            }
        }
    }
}

// ============================================================================
// Syntax Errors
// ============================================================================

#[test]
fn test_error_missing_semicolon() {
    let source = r#"
fn main() -> unit {
    let x: int = 5
    return none;
}
"#;
    expect_parse_error(source, "Missing semicolon after let statement");
}

#[test]
fn test_error_unbalanced_braces() {
    let source = r#"
fn main() -> unit {
    let x: int = 5;
"#;
    expect_parse_error(source, "Unbalanced braces (missing closing brace)");
}

#[test]
fn test_error_missing_type_annotation() {
    let source = r#"
fn main() -> unit {
    let x = 5;
    return none;
}
"#;
    expect_parse_error(source, "Missing type annotation (DR-SEM-001 violation)");
}

#[test]
fn test_error_invalid_token() {
    let source = r#"
fn main() -> unit {
    let x: int = 5 @ 10;
    return none;
}
"#;
    expect_parse_error(source, "Invalid token (@)");
}

// ============================================================================
// Type Errors
// ============================================================================

#[test]
fn test_error_type_mismatch_assignment() {
    let source = r#"
fn main() -> unit {
    let x: int = true;
    return none;
}
"#;
    expect_type_error(source, "Type mismatch: assigned bool to int variable");
}

#[test]
fn test_error_type_mismatch_return() {
    let source = r#"
fn returns_int() -> int {
    return true;
}

fn main() -> unit {
    let x: int = returns_int();
    return none;
}
"#;
    expect_type_error(source, "Type mismatch: returned bool from int function");
}

#[test]
fn test_error_undefined_symbol() {
    let source = r#"
fn main() -> unit {
    let x: int = undefined_variable;
    return none;
}
"#;
    expect_type_error(source, "Undefined symbol");
}

#[test]
fn test_error_undefined_function() {
    let source = r#"
fn main() -> unit {
    let x: int = undefined_function();
    return none;
}
"#;
    expect_type_error(source, "Undefined function");
}

#[test]
fn test_error_wrong_argument_count() {
    let source = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> unit {
    let x: int = add(5);
    return none;
}
"#;
    expect_type_error(source, "Wrong argument count (expected 2, got 1)");
}

#[test]
fn test_error_wrong_argument_type() {
    let source = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> unit {
    let x: int = add(5, true);
    return none;
}
"#;
    expect_type_error(source, "Wrong argument type (expected int, got bool)");
}

#[test]
fn test_error_field_not_found() {
    let source = r#"
struct Point {
    x: int,
    y: int,
}

fn main() -> unit {
    let p: Point = Point { x: 10, y: 20 };
    let z: int = p.z;
    return none;
}
"#;
    expect_type_error(source, "Field not found (z does not exist in Point)");
}

#[test]
fn test_error_missing_field_in_struct_literal() {
    let source = r#"
struct Point {
    x: int,
    y: int,
}

fn main() -> unit {
    let p: Point = Point { x: 10 };
    return none;
}
"#;
    expect_type_error(source, "Missing field in struct literal (y not provided)");
}

// ============================================================================
// Pattern Matching Errors
// ============================================================================

#[test]
fn test_error_non_exhaustive_match() {
    let source = r#"
enum Option {
    Some(int),
    None,
}

fn unwrap(opt: Option) -> int {
    match opt {
        Option::Some(x) => x,
    }
}

fn main() -> unit {
    return none;
}
"#;
    expect_type_error(source, "Non-exhaustive match (missing Option::None case)");
}

#[test]
fn test_error_match_arm_type_mismatch() {
    let source = r#"
enum Option {
    Some(int),
    None,
}

fn mixed_return(opt: Option) -> int {
    match opt {
        Option::Some(x) => x,
        Option::None => true,
    }
}

fn main() -> unit {
    return none;
}
"#;
    expect_type_error(source, "Match arm type mismatch (int vs bool)");
}

// ============================================================================
// Binary Operation Errors
// ============================================================================

#[test]
fn test_error_invalid_binary_operation() {
    let source = r#"
fn main() -> unit {
    let x: int = 5 + true;
    return none;
}
"#;
    expect_type_error(source, "Invalid binary operation (int + bool)");
}

#[test]
fn test_error_comparison_type_mismatch() {
    let source = r#"
fn main() -> unit {
    let b: bool = 5 < true;
    return none;
}
"#;
    expect_type_error(source, "Comparison type mismatch (int < bool)");
}

// ============================================================================
// Array Errors
// ============================================================================

#[test]
fn test_error_heterogeneous_array() {
    let source = r#"
fn main() -> unit {
    let arr: [int] = [1, 2, true, 4];
    return none;
}
"#;
    expect_type_error(source, "Heterogeneous array (expected all int, found bool)");
}

#[test]
fn test_error_array_index_type() {
    let source = r#"
fn main() -> unit {
    let arr: [int] = [1, 2, 3];
    let x: int = arr[true];
    return none;
}
"#;
    expect_type_error(source, "Array index must be int, not bool");
}

// ============================================================================
// Initialization Errors (DR-SEM-004)
// ============================================================================

#[test]
fn test_error_uninitialized_variable() {
    let source = r#"
fn main() -> unit {
    let x: int;
    let y: int = x + 5;
    return none;
}
"#;
    expect_type_error(source, "Uninitialized variable (DR-SEM-004 violation)");
}

// ============================================================================
// Summary Test
// ============================================================================

#[test]
fn test_all_error_scenarios_summary() {
    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Integration Tests: Error Detection");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("All error detection tests are run individually.");
    println!("Run 'cargo test integration_errors' to see results.\n");

    println!("═══════════════════════════════════════════════════════════");
    println!("  Error Categories Tested:");
    println!("═══════════════════════════════════════════════════════════");
    println!("  ✓ Syntax errors (missing semicolons, unbalanced braces)");
    println!("  ✓ Type mismatches (assignments, returns, operations)");
    println!("  ✓ Undefined symbols (variables, functions)");
    println!("  ✓ Function call errors (argument count, types)");
    println!("  ✓ Struct field errors (missing fields, unknown fields)");
    println!("  ✓ Pattern matching errors (non-exhaustive, type mismatches)");
    println!("  ✓ Array errors (heterogeneous elements, index types)");
    println!("  ✓ Initialization errors (uninitialized variables)");
    println!("═══════════════════════════════════════════════════════════\n");
}
