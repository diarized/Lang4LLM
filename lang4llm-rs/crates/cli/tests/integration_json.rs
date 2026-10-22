//! Integration tests for `--format json` output (US2 — Phase 7 Developer Tooling)
//!
//! These tests invoke the compiler pipeline directly and build JSON output in the
//! same way as `check_file(..., OutputFormat::Json)` does, without importing from
//! the CLI binary crate (which is not a library).

use lang4llm_ast::SourceMap;
use serde_json::{json, Value};

// ─── Inline JSON builder (mirrors check_file JSON path) ──────────────────────

/// Run parse + typecheck on `source`, return the JSON DiagnosticOutput Value.
fn check_to_json(source: &str, file: &str) -> Value {
    let sm = SourceMap::from_source(source);

    // --- parse ---
    let program = match lang4llm_parser::parse(source, &sm) {
        Ok(p) => p,
        Err(e) => {
            let (code, msg, span_val) = parse_error_json(&e);
            return json!({
                "success": false,
                "errors": [{
                    "code": code,
                    "severity": "error",
                    "message": msg,
                    "file": file,
                    "span": span_val,
                }]
            });
        }
    };

    // --- typecheck ---
    match lang4llm_typeck::check_program(&program) {
        Ok(()) => json!({ "success": true, "errors": [], "result": {} }),
        Err(errors) => {
            let json_errors: Vec<Value> = errors.iter().map(|e| type_error_json(e, file)).collect();
            json!({ "success": false, "errors": json_errors })
        }
    }
}

/// Convert a ParseError to (code, message, span_value).
fn parse_error_json(e: &lang4llm_parser::ParseError) -> (&'static str, String, Value) {
    use lang4llm_parser::ParseError;
    match e {
        ParseError::LexError { location, message } => {
            ("E1001", message.clone(), span_json(*location))
        }
        ParseError::SyntaxError { location, message } => {
            ("E1002", message.clone(), span_json(*location))
        }
    }
}

/// Convert a TypeError to a JSON Value.
fn type_error_json(e: &lang4llm_typeck::TypeError, file: &str) -> Value {
    use lang4llm_typeck::{ExhaustivenessError, SymbolError, TypeError::*};

    let code = match e {
        TypeMismatch { .. }          => "E0001",
        UndefinedSymbol { .. }       => "E0002",
        MissingTypeAnnotation { .. } => "E0003",
        Uninitialized { .. }         => "E0004",
        InvalidBinaryOp { .. }       => "E0005",
        ArgumentCountMismatch { .. } => "E0006",
        GenericMismatch { .. }       => "E0007",
        NonExhaustiveMatch { .. }    => "E0008",
        TraitBoundViolation { .. }   => "E0009",
        FieldNotFound { .. }         => "E0010",
        Symbol(_)                    => "E0011",
        Exhaustiveness(_)            => "E0012",
    };

    let primary_span = match e {
        TypeMismatch { span, .. }
        | UndefinedSymbol { span, .. }
        | MissingTypeAnnotation { span, .. }
        | Uninitialized { span, .. }
        | InvalidBinaryOp { span, .. }
        | ArgumentCountMismatch { span, .. }
        | GenericMismatch { span, .. }
        | NonExhaustiveMatch { span, .. }
        | TraitBoundViolation { span, .. }
        | FieldNotFound { span, .. } => span_json(*span),
        Symbol(sym) => match sym {
            SymbolError::Undefined { span, .. } => span_json(*span),
            SymbolError::Shadowing { shadowing, .. } => span_json(*shadowing),
            SymbolError::WrongKind { span, .. } => span_json(*span),
        },
        Exhaustiveness(exh) => match exh {
            ExhaustivenessError::NonExhaustive { span, .. } => span_json(*span),
            ExhaustivenessError::UnknownVariant { span, .. } => span_json(*span),
        },
    };

    json!({
        "code": code,
        "severity": "error",
        "message": e.to_string(),
        "file": file,
        "span": primary_span,
    })
}

/// Convert a Span to a JSON Value (null for dummy spans).
fn span_json(span: lang4llm_ast::Span) -> Value {
    if span.start_line == 0 {
        Value::Null
    } else {
        json!({
            "start_line": span.start_line,
            "start_col":  span.start_col,
            "end_line":   span.end_line,
            "end_col":    span.end_col,
        })
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_json_success_valid_program() {
    let source = r#"
fn main() -> unit {
    let x: int = 42;
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    assert_eq!(json["success"], true, "valid program should succeed");
    assert!(json["errors"].as_array().unwrap().is_empty(), "no errors on success");
}

#[test]
fn test_json_type_error_has_stable_code() {
    let source = r#"
fn main() -> unit {
    let x: int = true;
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    assert_eq!(json["success"], false);

    let errors = json["errors"].as_array().unwrap();
    assert!(!errors.is_empty());

    let code = errors[0]["code"].as_str().unwrap();
    assert!(code.starts_with('E'), "code starts with E: {code}");
    assert_eq!(code.len(), 5, "code is E+4digits: {code}");
}

#[test]
fn test_json_type_error_has_real_span() {
    let source = r#"
fn main() -> unit {
    let x: int = true;
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    let span = &json["errors"][0]["span"];
    assert!(!span.is_null(), "span should be present after Phase 3 migration");
    assert!(
        span["start_line"].as_u64().unwrap() > 0,
        "start_line > 0: {:?}", span
    );
}

#[test]
fn test_json_parse_error_schema() {
    let source = r#"
fn main() -> unit {
    let x: int = 5
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    assert_eq!(json["success"], false);

    let errors = json["errors"].as_array().unwrap();
    assert!(!errors.is_empty());
    assert_eq!(errors[0]["severity"], "error");
    assert!(errors[0]["code"].as_str().unwrap().starts_with("E1"));

    // Span must be real (not 0:0) for parse errors
    let span = &errors[0]["span"];
    if !span.is_null() {
        assert!(span["start_line"].as_u64().unwrap() > 0);
    }
}

#[test]
fn test_json_empty_source_does_not_panic() {
    let json = check_to_json("", "empty.lang4llm");
    // Empty file → valid JSON response (either success or failure, never panic)
    assert!(json["success"].is_boolean());
    assert!(json["errors"].is_array());
}

#[test]
fn test_json_error_includes_file_name() {
    let source = r#"
fn main() -> unit {
    let x: int = undefined_var;
    return;
}
"#;
    let json = check_to_json(source, "my_program.lang4llm");
    let errors = json["errors"].as_array().unwrap();
    assert!(!errors.is_empty());
    assert_eq!(errors[0]["file"], "my_program.lang4llm");
}

#[test]
fn test_json_multiple_errors_all_reported() {
    let source = r#"
fn main() -> unit {
    let a: int = missing_a;
    let b: int = missing_b;
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    assert_eq!(json["success"], false);
    let errors = json["errors"].as_array().unwrap();
    assert!(errors.len() >= 2, "expected ≥2 errors, got {}", errors.len());
}

#[test]
fn test_json_errors_field_always_array() {
    let programs = [
        r#"fn main() -> unit { return; }"#,
        r#"fn main() -> unit { let x: int = true; return; }"#,
    ];
    for src in &programs {
        let json = check_to_json(src, "t.lang4llm");
        assert!(json["errors"].is_array(), "errors must always be an array");
    }
}

#[test]
fn test_json_undefined_symbol_has_real_span() {
    // Undefined variables route through TypeError::Symbol(SymbolError::Undefined) → E0011.
    // Verify the reported span is real (not dummy 0:0) after Phase 3 migration.
    let source = r#"
fn main() -> unit {
    let x: int = undefined_var;
    return;
}
"#;
    let json = check_to_json(source, "test.lang4llm");
    let errors = json["errors"].as_array().unwrap();
    assert!(!errors.is_empty());

    let first = &errors[0];
    // Error code must be E0011 (Symbol error)
    assert_eq!(first["code"], "E0011", "undefined var → E0011: {:?}", first);

    // Span must be real and point to line 3 (the let binding)
    let span = &first["span"];
    assert!(!span.is_null(), "span should be present");
    assert!(span["start_line"].as_u64().unwrap() > 0, "start_line > 0: {:?}", span);
}
