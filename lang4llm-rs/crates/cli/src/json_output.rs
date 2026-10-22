//! Machine-readable JSON output structs and conversion utilities.
//!
//! Activated by `--format json` on any CLI command. The output schema is:
//!
//! ```json
//! {
//!   "success": true | false,
//!   "errors": [ <DiagnosticError>, ... ],
//!   "result": { <command-specific> } | null
//! }
//! ```

use lang4llm_ast::Span;
use lang4llm_parser::ParseError;
use lang4llm_typeck::{ExhaustivenessError, SymbolError, TypeError};
use serde::Serialize;

// ─── Output structs ───────────────────────────────────────────────────────────

/// Top-level JSON envelope for all CLI commands.
#[derive(Serialize)]
pub struct DiagnosticOutput {
    pub success: bool,
    pub errors: Vec<DiagnosticError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CommandResult>,
}

/// A single diagnostic error entry.
#[derive(Serialize)]
pub struct DiagnosticError {
    /// Stable error code (E0001–E0012 for type errors, E1001–E1002 for parse errors).
    pub code: String,
    /// Always "error".
    pub severity: String,
    /// Human-readable error message.
    pub message: String,
    /// Source file name (path as string).
    pub file: String,
    /// Primary error location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<SpanInfo>,
    /// Optional fix suggestion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
    /// Secondary labels (e.g., function definition site for arg-count mismatches).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<RelatedInfo>,
}

/// Serialisable span (1-indexed line:col).
#[derive(Serialize, Clone, Copy)]
pub struct SpanInfo {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

/// A secondary diagnostic label with its own span.
#[derive(Serialize)]
pub struct RelatedInfo {
    pub message: String,
    pub span: SpanInfo,
}

/// Command-specific success payload (currently unused but reserved for future use).
#[derive(Serialize)]
pub struct CommandResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Convert an AST Span to SpanInfo. Returns None for dummy spans.
fn span_to_info(span: Span) -> Option<SpanInfo> {
    if span.start_line == 0 {
        None
    } else {
        Some(SpanInfo {
            start_line: span.start_line,
            start_col: span.start_col,
            end_line: span.end_line,
            end_col: span.end_col,
        })
    }
}

// ─── Error code mapping ───────────────────────────────────────────────────────

/// Map a TypeError variant to its stable error code string.
pub fn type_error_code(e: &TypeError) -> &'static str {
    match e {
        TypeError::TypeMismatch { .. }          => "E0001",
        TypeError::UndefinedSymbol { .. }       => "E0002",
        TypeError::MissingTypeAnnotation { .. } => "E0003",
        TypeError::Uninitialized { .. }         => "E0004",
        TypeError::InvalidBinaryOp { .. }       => "E0005",
        TypeError::ArgumentCountMismatch { .. } => "E0006",
        TypeError::GenericMismatch { .. }       => "E0007",
        TypeError::NonExhaustiveMatch { .. }    => "E0008",
        TypeError::TraitBoundViolation { .. }   => "E0009",
        TypeError::FieldNotFound { .. }         => "E0010",
        TypeError::Symbol(SymbolError::Undefined { .. })  => "E0011",
        TypeError::Symbol(SymbolError::Shadowing { .. })  => "E0011",
        TypeError::Symbol(SymbolError::WrongKind { .. })  => "E0011",
        TypeError::Exhaustiveness(ExhaustivenessError::NonExhaustive { .. }) => "E0012",
        TypeError::Exhaustiveness(ExhaustivenessError::UnknownVariant { .. }) => "E0012",
    }
}

/// Map a ParseError variant to its stable error code string.
pub fn parse_error_code(e: &ParseError) -> &'static str {
    match e {
        ParseError::LexError { .. }    => "E1001",
        ParseError::SyntaxError { .. } => "E1002",
    }
}

// ─── Conversion functions ─────────────────────────────────────────────────────

/// Convert a TypeError to a DiagnosticError.
pub fn type_error_to_diagnostic(e: &TypeError, file: &str) -> DiagnosticError {
    use lang4llm_typeck::{SymbolError, TypeError::*};

    let code = type_error_code(e).to_string();
    let message = e.to_string();

    // Extract primary span
    let span = match e {
        TypeMismatch { span, .. }
        | UndefinedSymbol { span, .. }
        | MissingTypeAnnotation { span, .. }
        | Uninitialized { span, .. }
        | InvalidBinaryOp { span, .. }
        | ArgumentCountMismatch { span, .. }
        | GenericMismatch { span, .. }
        | NonExhaustiveMatch { span, .. }
        | TraitBoundViolation { span, .. }
        | FieldNotFound { span, .. } => span_to_info(*span),
        Symbol(sym_err) => match sym_err {
            SymbolError::Undefined { span, .. } => span_to_info(*span),
            SymbolError::Shadowing { shadowing, .. } => span_to_info(*shadowing),
            SymbolError::WrongKind { span, .. } => span_to_info(*span),
        },
        Exhaustiveness(exh_err) => match exh_err {
            ExhaustivenessError::NonExhaustive { span, .. } => span_to_info(*span),
            ExhaustivenessError::UnknownVariant { span, .. } => span_to_info(*span),
        },
    };

    // Build related info (secondary spans)
    let mut related = Vec::new();

    // Shadowing errors: add original declaration site as related span
    if let Symbol(SymbolError::Shadowing { name, original, .. }) = e {
        if let Some(orig_info) = span_to_info(*original) {
            related.push(RelatedInfo {
                message: format!("'{name}' first declared here"),
                span: orig_info,
            });
        }
    }

    // ArgumentCountMismatch: add function definition site as related span
    if let ArgumentCountMismatch { fn_name, def_span, .. } = e {
        if let Some(def_info) = span_to_info(*def_span) {
            related.push(RelatedInfo {
                message: format!("`{fn_name}` defined here"),
                span: def_info,
            });
        }
    }

    // UndefinedSymbol: expose candidates as suggestion
    if let UndefinedSymbol { candidates, .. } = e {
        if !candidates.is_empty() {
            // candidates are returned in the suggestion field (overrides generic hint)
        }
        let _ = candidates; // suppress unused warning — used in suggestion()
    }

    // Suggestion (type-specific hints)
    let suggestion = type_error_suggestion(e);

    DiagnosticError {
        code,
        severity: "error".to_string(),
        message,
        file: file.to_string(),
        span,
        suggestion,
        related,
    }
}

/// Convert a ParseError to a DiagnosticError.
pub fn parse_error_to_diagnostic(e: &ParseError, file: &str) -> DiagnosticError {
    let code = parse_error_code(e).to_string();
    let (message, span) = match e {
        ParseError::LexError { location, message } => (message.clone(), span_to_info(*location)),
        ParseError::SyntaxError { location, message } => (message.clone(), span_to_info(*location)),
    };

    DiagnosticError {
        code,
        severity: "error".to_string(),
        message,
        file: file.to_string(),
        span,
        suggestion: None,
        related: Vec::new(),
    }
}

// ─── Suggestion hints ─────────────────────────────────────────────────────────

fn type_error_suggestion(e: &TypeError) -> Option<String> {
    match e {
        TypeError::TypeMismatch { expected, found, .. } => {
            match (expected.as_str(), found.as_str()) {
                ("string", _) => Some(format!("use to_string(value) to convert {found} to string")),
                ("int", "float") => Some("use int(value) to truncate float to int".to_string()),
                ("float", "int") => Some("use float(value) to widen int to float".to_string()),
                _ => None,
            }
        }
        TypeError::UndefinedSymbol { candidates, name, .. } => {
            if candidates.is_empty() {
                Some(format!("check spelling of '{name}' or add a let binding"))
            } else if candidates.len() == 1 {
                Some(format!("did you mean `{}`?", candidates[0]))
            } else {
                let list = candidates.iter().map(|c| format!("`{c}`")).collect::<Vec<_>>().join(", ");
                Some(format!("did you mean one of: {list}?"))
            }
        }
        _ => None,
    }
}
