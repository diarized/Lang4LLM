//! Integration tests for span accuracy (US1 — Phase 7 Developer Tooling)
//!
//! Verifies that AST nodes carry real source positions after the grammar.lalrpop
//! Span::dummy() → source_map.span(@L, @R) migration.
//!
//! Each test specifies a program with a deliberate error at a known line:col
//! and asserts the reported span is non-dummy (start_line > 0) and matches
//! the expected location.

use lang4llm_ast::SourceMap;

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Parse + type-check; return the first TypeError, or panic.
fn first_type_error(source: &str) -> lang4llm_typeck::TypeError {
    let sm = SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm)
        .expect("should parse without error");
    let errors = lang4llm_typeck::check_program(&program)
        .expect_err("should produce at least one type error");
    errors.into_iter().next().unwrap()
}

/// Parse + type-check; return all TypeErrors, or panic.
fn all_type_errors(source: &str) -> Vec<lang4llm_typeck::TypeError> {
    let sm = SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm)
        .expect("should parse without error");
    lang4llm_typeck::check_program(&program)
        .expect_err("should produce at least one type error")
}

/// Extract span from any TypeError variant.
fn error_span(e: &lang4llm_typeck::TypeError) -> lang4llm_ast::Span {
    use lang4llm_typeck::{SymbolError, TypeError::*};
    match e {
        TypeMismatch { span, .. }
        | UndefinedSymbol { span, .. }
        | MissingTypeAnnotation { span, .. }
        | Uninitialized { span, .. }
        | InvalidBinaryOp { span, .. }
        | ArgumentCountMismatch { span, .. }
        | GenericMismatch { span, .. }
        | NonExhaustiveMatch { span, .. }
        | TraitBoundViolation { span, .. }
        | FieldNotFound { span, .. } => *span,
        // Symbol wraps SymbolError (tuple variant)
        Symbol(sym_err) => match sym_err {
            SymbolError::Undefined { span, .. } => *span,
            SymbolError::Shadowing { original, .. } => *original,
            SymbolError::WrongKind { span, .. } => *span,
        },
        // Exhaustiveness wraps ExhaustivenessError — extract span
        Exhaustiveness(exh_err) => {
            use lang4llm_typeck::ExhaustivenessError;
            match exh_err {
                ExhaustivenessError::NonExhaustive { span, .. } => *span,
                ExhaustivenessError::UnknownVariant { span, .. } => *span,
            }
        }
    }
}

// ─── span non-dummy invariant ─────────────────────────────────────────────────

/// All type errors should now carry a real (non-dummy) span.
/// A dummy span has start_line == 0, start_col == 0.
fn assert_real_span(span: lang4llm_ast::Span, ctx: &str) {
    assert!(
        span.start_line > 0,
        "{ctx}: expected real span (start_line > 0), got {span:?}"
    );
}

// ─── UndefinedSymbol location ─────────────────────────────────────────────────

#[test]
fn test_span_undefined_symbol_line() {
    // 'undefined_var' is on line 3
    let source = r#"
fn main() -> unit {
    let x: int = undefined_var;
    return;
}
"#;
    let err = first_type_error(source);
    let span = error_span(&err);
    // Should report real location — line 3, not 0:0
    assert_real_span(span, "UndefinedSymbol");
    assert_eq!(span.start_line, 3, "undefined_var is on line 3");
}

// ─── TypeMismatch location ────────────────────────────────────────────────────

#[test]
fn test_span_type_mismatch_line() {
    // Assign a bool where int is expected — on line 3
    let source = r#"
fn main() -> unit {
    let x: int = true;
    return;
}
"#;
    let err = first_type_error(source);
    let span = error_span(&err);
    assert_real_span(span, "TypeMismatch");
    assert_eq!(span.start_line, 3, "type mismatch on line 3");
}

// ─── ArgumentCountMismatch location ──────────────────────────────────────────

#[test]
fn test_span_arg_count_mismatch_line() {
    // foo takes 1 arg; called with 2 — call is on line 6
    let source = r#"
fn foo(x: int) -> int {
    return x;
}
fn main() -> unit {
    let result: int = foo(1, 2);
    return;
}
"#;
    let err = first_type_error(source);
    let span = error_span(&err);
    assert_real_span(span, "ArgumentCountMismatch");
    assert_eq!(span.start_line, 6, "bad call is on line 6");
}

// ─── FieldNotFound location ───────────────────────────────────────────────────

#[test]
fn test_span_field_not_found_line() {
    // Access nonexistent field on line 8
    let source = r#"
struct Point {
    x: int,
    y: int,
}
fn main() -> unit {
    let p: Point = Point { x: 1, y: 2 };
    let val: int = p.z;
    return;
}
"#;
    let err = first_type_error(source);
    let span = error_span(&err);
    assert_real_span(span, "FieldNotFound");
    assert_eq!(span.start_line, 8, "field access on line 8");
}

// ─── Multiple errors — each has a distinct real span ─────────────────────────

#[test]
fn test_span_multiple_errors_distinct_lines() {
    // Two undefined symbols on lines 3 and 4
    let source = r#"
fn main() -> unit {
    let a: int = missing_a;
    let b: int = missing_b;
    return;
}
"#;
    let errors = all_type_errors(source);
    assert!(errors.len() >= 2, "expected at least 2 errors");

    let spans: Vec<_> = errors.iter().map(|e| error_span(e)).collect();

    // All spans should be real
    for (i, &span) in spans.iter().enumerate() {
        assert_real_span(span, &format!("error[{i}]"));
    }

    // The two undefined-symbol errors should be on different lines
    let lines: std::collections::HashSet<usize> =
        spans.iter().map(|s| s.start_line).collect();
    assert!(
        lines.len() >= 2,
        "expected errors on distinct lines; got lines: {:?}",
        lines
    );
}

// ─── Parse error location ─────────────────────────────────────────────────────

#[test]
fn test_span_parse_error_non_dummy() {
    // Missing semicolon — parse error should carry a real location
    let source = r#"
fn main() -> unit {
    let x: int = 5
    return;
}
"#;
    let sm = SourceMap::from_source(source);
    let err = lang4llm_parser::parse(source, &sm)
        .expect_err("should fail to parse");

    // Extract span from ParseError variants (LexError | SyntaxError)
    use lang4llm_parser::ParseError;
    let span = match &err {
        ParseError::LexError { location, .. } => *location,
        ParseError::SyntaxError { location, .. } => *location,
    };

    // The span should point somewhere after line 1 (not dummy 0:0)
    // A dummy span has start_line == 0
    assert!(
        span.start_line > 0,
        "parse error should have real span, got {span:?}"
    );
}
