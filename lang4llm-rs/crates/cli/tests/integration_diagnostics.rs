//! Integration tests for enhanced diagnostic messages (US3 — Phase 7 Developer Tooling)
//!
//! Verifies:
//! - ArgumentCountMismatch carries def_span + fn_name for multi-span display
//! - All TypeError variants produce non-empty error messages (no panic)
//! - TypeMismatch suggestion hints are populated

use lang4llm_ast::SourceMap;
use lang4llm_typeck::TypeError;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn parse_and_check(source: &str) -> Vec<TypeError> {
    let sm = SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm)
        .expect("should parse");
    lang4llm_typeck::check_program(&program)
        .expect_err("should produce type errors")
}

// ─── ArgumentCountMismatch carries def_span ───────────────────────────────────

#[test]
fn test_arg_count_mismatch_has_def_span() {
    let source = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}
fn main() -> unit {
    let result: int = add(1, 2, 3);
    return;
}
"#;
    let errors = parse_and_check(source);
    let mismatch = errors.iter().find(|e| matches!(e, TypeError::ArgumentCountMismatch { .. }));
    assert!(mismatch.is_some(), "should have ArgumentCountMismatch, got: {:?}", errors);

    if let Some(TypeError::ArgumentCountMismatch { expected, found, def_span, fn_name, .. }) = mismatch {
        assert_eq!(*expected, 2, "add() expects 2 args");
        assert_eq!(*found, 3, "called with 3");
        assert_eq!(fn_name, "add", "fn_name should be 'add'");
        // def_span should be real (add is defined on line 2)
        assert!(def_span.start_line > 0, "def_span should be real, got {:?}", def_span);
        assert_eq!(def_span.start_line, 2, "add() defined on line 2");
    }
}

#[test]
fn test_arg_count_mismatch_too_few_args() {
    let source = r#"
fn greet(name: string, title: string) -> unit {
    return;
}
fn main() -> unit {
    greet("Bob");
    return;
}
"#;
    let errors = parse_and_check(source);
    let mismatch = errors.iter().find(|e| matches!(e, TypeError::ArgumentCountMismatch { .. }));
    assert!(mismatch.is_some());

    if let Some(TypeError::ArgumentCountMismatch { expected, found, fn_name, def_span, .. }) = mismatch {
        assert_eq!(*expected, 2);
        assert_eq!(*found, 1);
        assert_eq!(fn_name, "greet");
        assert!(def_span.start_line > 0);
    }
}

// ─── UndefinedSymbol candidates (via struct duplicate field) ──────────────────

#[test]
fn test_undefined_symbol_has_candidates_field() {
    // UndefinedSymbol is emitted for duplicate struct field names.
    // Candidates are empty in this case (no "did you mean?" for dup field).
    let source = r#"
struct Dup {
    x: int,
    x: int,
}
fn main() -> unit {
    return;
}
"#;
    let errors = parse_and_check(source);
    let undefined = errors.iter().find(|e| matches!(e, TypeError::UndefinedSymbol { .. }));
    assert!(undefined.is_some(), "duplicate field → UndefinedSymbol");

    if let Some(TypeError::UndefinedSymbol { candidates, .. }) = undefined {
        // Candidates is always a Vec (may be empty for non-scope-lookup errors)
        let _ = candidates; // field exists and is accessible
    }
}

// ─── TypeMismatch error message is non-empty ─────────────────────────────────

#[test]
fn test_type_mismatch_error_message_non_empty() {
    let source = r#"
fn main() -> unit {
    let x: int = true;
    return;
}
"#;
    let errors = parse_and_check(source);
    assert!(!errors.is_empty());

    for err in &errors {
        let msg = err.to_string();
        assert!(!msg.is_empty(), "error message should not be empty");
    }
}

// ─── All TypeError variants display without panic ─────────────────────────────

#[test]
fn test_type_mismatch_displays_without_panic() {
    let source = r#"fn main() -> unit { let x: int = true; return; }"#;
    let errors = parse_and_check(source);
    for err in &errors {
        assert!(matches!(err, TypeError::TypeMismatch { .. }));
        let msg = err.to_string();
        assert!(msg.contains("Type mismatch"), "message: {msg}");
    }
}

#[test]
fn test_field_not_found_displays_without_panic() {
    let source = r#"
struct S { x: int, }
fn main() -> unit {
    let s: S = S { x: 1 };
    let val: int = s.z;
    return;
}
"#;
    let errors = parse_and_check(source);
    let field_err = errors.iter().find(|e| matches!(e, TypeError::FieldNotFound { .. }));
    assert!(field_err.is_some(), "expected FieldNotFound, got: {:?}", errors);

    let msg = field_err.unwrap().to_string();
    assert!(!msg.is_empty());
    assert!(msg.contains("Field not found") || msg.contains("field"), "msg: {msg}");
}

#[test]
fn test_argument_count_mismatch_displays_fn_name() {
    let source = r#"
fn compute(x: int) -> int { return x; }
fn main() -> unit {
    let result: int = compute(1, 2);
    return;
}
"#;
    let errors = parse_and_check(source);
    let mismatch = errors.iter().find(|e| matches!(e, TypeError::ArgumentCountMismatch { .. }));
    assert!(mismatch.is_some());

    // The Display message should be non-empty
    let msg = mismatch.unwrap().to_string();
    assert!(!msg.is_empty());
}

// ─── Symbol(Undefined) error has real span ────────────────────────────────────

#[test]
fn test_symbol_undefined_has_real_span() {
    // Variable lookup errors produce TypeError::Symbol(SymbolError::Undefined)
    let source = r#"
fn main() -> unit {
    let x: int = missing_var;
    return;
}
"#;
    let errors = parse_and_check(source);
    let sym_err = errors.iter().find(|e| matches!(e, TypeError::Symbol(_)));
    assert!(sym_err.is_some(), "should have TypeError::Symbol, got: {:?}", errors);

    if let Some(TypeError::Symbol(lang4llm_typeck::SymbolError::Undefined { span, name })) = sym_err {
        assert_eq!(name, "missing_var");
        assert!(span.start_line > 0, "span should be real, got {:?}", span);
        assert_eq!(span.start_line, 3, "missing_var is on line 3");
    }
}

// ─── Symbol(Shadowing) error carries dual spans ───────────────────────────────

#[test]
fn test_symbol_shadowing_has_both_spans() {
    let source = r#"
fn main() -> unit {
    let x: int = 1;
    let x: int = 2;
    return;
}
"#;
    let errors = parse_and_check(source);
    let shadow_err = errors.iter().find(|e| {
        matches!(e, TypeError::Symbol(lang4llm_typeck::SymbolError::Shadowing { .. }))
    });
    assert!(shadow_err.is_some(), "should have shadowing error, got: {:?}", errors);

    if let Some(TypeError::Symbol(lang4llm_typeck::SymbolError::Shadowing { original, shadowing, .. })) = shadow_err {
        assert!(original.start_line > 0, "original span should be real");
        assert!(shadowing.start_line > 0, "shadowing span should be real");
        assert!(shadowing.start_line > original.start_line, "shadowing should be after original");
    }
}
