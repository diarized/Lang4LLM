//! Diagnostic reporting for Lang4LLM errors
//!
//! Provides beautiful error messages with source context using ariadne.

use ariadne::{Color, Label, Report, ReportKind, Source};
use lang4llm_ast::Span;
use lang4llm_parser::ParseError;
use lang4llm_typeck::TypeError;
use std::path::Path;

/// Report a parse error with source context
pub fn report_parse_error(
    path: &Path,
    source: &str,
    error: &ParseError,
) -> String {
    let filename = path.display().to_string();

    match error {
        ParseError::LexError { location, message } => {
            let span_range = span_to_range(source, location);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message("Lexical error")
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(message)
                        .with_color(Color::Red),
                )
                .with_note(get_lex_error_suggestion(message))
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        ParseError::SyntaxError { location, message } => {
            let span_range = span_to_range(source, location);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message("Syntax error")
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(message)
                        .with_color(Color::Red),
                )
                .with_note(get_syntax_error_suggestion(message))
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
    }
}

/// Report a type error with source context
pub fn report_type_error(
    path: &Path,
    source: &str,
    error: &TypeError,
) -> String {
    let filename = path.display().to_string();

    match error {
        TypeError::TypeMismatch { span, expected, found } => {
            let span_range = span_to_range(source, span);

            let note = get_type_mismatch_suggestion(expected, found);
            let mut builder = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Type mismatch: expected {expected}, found {found}"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(format!("expected {expected}, found {found}"))
                        .with_color(Color::Red),
                );
            if !note.is_empty() {
                builder = builder.with_note(note);
            }
            let report = builder.finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::UndefinedSymbol { name, span, candidates } => {
            let span_range = span_to_range(source, span);

            // Build "did you mean?" note
            let note = if candidates.is_empty() {
                "Consider:\n  • Declaring the variable before using it\n  • Checking for typos in the variable name".to_string()
            } else if candidates.len() == 1 {
                format!("did you mean `{}`?", candidates[0])
            } else {
                let list = candidates.iter().map(|c| format!("`{c}`")).collect::<Vec<_>>().join(", ");
                format!("did you mean one of: {list}?")
            };

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Undefined symbol: {name}"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(format!("'{name}' not found in this scope"))
                        .with_color(Color::Red),
                )
                .with_note(note)
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::MissingTypeAnnotation { name, span } => {
            let span_range = span_to_range(source, span);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Missing type annotation for '{name}'"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message("explicit type required here")
                        .with_color(Color::Red),
                )
                .with_note(format!("Lang4LLM requires explicit type annotations (DR-SEM-001)\nExample: let {name}: i32 = ..."))
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::Uninitialized { name, span } => {
            let span_range = span_to_range(source, span);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Uninitialized variable '{name}'"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message("must be initialized")
                        .with_color(Color::Red),
                )
                .with_note(format!("Lang4LLM requires all variables to be initialized (DR-SEM-004)\nExample: let {name}: Type = initial_value"))
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::InvalidBinaryOp { op, left, right, span } => {
            let span_range = span_to_range(source, span);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Invalid operation: {left} {op:?} {right}"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(format!("cannot apply {op:?} to {left} and {right}"))
                        .with_color(Color::Red),
                )
                .with_note(get_binary_op_suggestion(left, right))
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::ArgumentCountMismatch { expected, found, span, def_span, fn_name } => {
            let span_range = span_to_range(source, span);
            let def_range = span_to_range(source, def_span);

            let mut builder = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Wrong number of arguments to `{fn_name}`: expected {expected}, found {found}"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(format!("{expected} argument{} expected, {found} provided",
                            if *expected == 1 { "" } else { "s" }))
                        .with_color(Color::Red),
                );

            // Add secondary label pointing to the function definition (if not dummy span)
            if def_span.start_line > 0 && !def_range.is_empty() {
                builder = builder.with_label(
                    Label::new((&filename, def_range))
                        .with_message(format!("`{fn_name}` defined here with {expected} parameter{}",
                            if *expected == 1 { "" } else { "s" }))
                        .with_color(Color::Blue),
                );
            }

            let report = builder.finish();
            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::FieldNotFound { ty, field, span } => {
            let span_range = span_to_range(source, span);

            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Field '{field}' not found"))
                .with_label(
                    Label::new((&filename, span_range.clone()))
                        .with_message(format!("type {ty} has no field '{field}'"))
                        .with_color(Color::Red),
                )
                .with_note("Check the struct definition or for typos in the field name")
                .finish();

            let mut output = Vec::new();
            report
                .write((&filename, Source::from(source)), &mut output)
                .expect("Failed to write report");
            String::from_utf8(output).expect("Invalid UTF-8 in report")
        }
        TypeError::GenericMismatch { span, message } => {
            let span_range = span_to_range(source, span);
            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message("Generic type mismatch")
                .with_label(
                    Label::new((&filename, span_range))
                        .with_message(message)
                        .with_color(Color::Red),
                )
                .finish();
            let mut output = Vec::new();
            report.write((&filename, Source::from(source)), &mut output).expect("write");
            String::from_utf8(output).expect("utf8")
        }

        TypeError::NonExhaustiveMatch { span, missing } => {
            let span_range = span_to_range(source, span);
            let missing_list = missing.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", ");
            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message("Non-exhaustive match")
                .with_label(
                    Label::new((&filename, span_range))
                        .with_message(format!("missing patterns: {missing_list}"))
                        .with_color(Color::Red),
                )
                .with_note("Add match arms for all missing patterns, or add a wildcard arm `_ => ...`")
                .finish();
            let mut output = Vec::new();
            report.write((&filename, Source::from(source)), &mut output).expect("write");
            String::from_utf8(output).expect("utf8")
        }

        TypeError::TraitBoundViolation { span, ty, trait_name } => {
            let span_range = span_to_range(source, span);
            let report = Report::build(ReportKind::Error, &filename, span_range.start)
                .with_message(format!("Trait bound violation: {ty} does not implement {trait_name}"))
                .with_label(
                    Label::new((&filename, span_range))
                        .with_message(format!("type `{ty}` required to implement `{trait_name}`"))
                        .with_color(Color::Red),
                )
                .finish();
            let mut output = Vec::new();
            report.write((&filename, Source::from(source)), &mut output).expect("write");
            String::from_utf8(output).expect("utf8")
        }

        TypeError::Symbol(sym_err) => {
            use lang4llm_typeck::SymbolError;
            match sym_err {
                SymbolError::Undefined { name, span } => {
                    let span_range = span_to_range(source, span);
                    let report = Report::build(ReportKind::Error, &filename, span_range.start)
                        .with_message(format!("Undefined symbol: `{name}`"))
                        .with_label(
                            Label::new((&filename, span_range))
                                .with_message(format!("`{name}` not found in scope"))
                                .with_color(Color::Red),
                        )
                        .with_note("Check for typos or add a declaration before this use")
                        .finish();
                    let mut output = Vec::new();
                    report.write((&filename, Source::from(source)), &mut output).expect("write");
                    String::from_utf8(output).expect("utf8")
                }
                SymbolError::Shadowing { name, original, shadowing } => {
                    let orig_range = span_to_range(source, original);
                    let shadow_range = span_to_range(source, shadowing);
                    let report = Report::build(ReportKind::Error, &filename, shadow_range.start)
                        .with_message(format!("Variable shadowing not allowed: `{name}`"))
                        .with_label(
                            Label::new((&filename, shadow_range))
                                .with_message("second declaration here (shadowing)")
                                .with_color(Color::Red),
                        )
                        .with_label(
                            Label::new((&filename, orig_range))
                                .with_message("first declaration here")
                                .with_color(Color::Blue),
                        )
                        .with_note("Lang4LLM prohibits variable shadowing (DR-SEM-003). Use a different name.")
                        .finish();
                    let mut output = Vec::new();
                    report.write((&filename, Source::from(source)), &mut output).expect("write");
                    String::from_utf8(output).expect("utf8")
                }
                SymbolError::WrongKind { name, expected, found, span } => {
                    let span_range = span_to_range(source, span);
                    let report = Report::build(ReportKind::Error, &filename, span_range.start)
                        .with_message(format!("Wrong symbol kind: `{name}`"))
                        .with_label(
                            Label::new((&filename, span_range))
                                .with_message(format!("`{name}` is a {found:?} but a {expected:?} is expected here"))
                                .with_color(Color::Red),
                        )
                        .finish();
                    let mut output = Vec::new();
                    report.write((&filename, Source::from(source)), &mut output).expect("write");
                    String::from_utf8(output).expect("utf8")
                }
            }
        }

        TypeError::Exhaustiveness(exh_err) => {
            use lang4llm_typeck::ExhaustivenessError;
            match exh_err {
                ExhaustivenessError::NonExhaustive { span, missing } => {
                    let span_range = span_to_range(source, span);
                    let missing_list = missing.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", ");
                    let report = Report::build(ReportKind::Error, &filename, span_range.start)
                        .with_message("Non-exhaustive match expression")
                        .with_label(
                            Label::new((&filename, span_range))
                                .with_message(format!("missing patterns: {missing_list}"))
                                .with_color(Color::Red),
                        )
                        .with_note("Add match arms for all missing variants, or add a wildcard arm `_ => ...`")
                        .finish();
                    let mut output = Vec::new();
                    report.write((&filename, Source::from(source)), &mut output).expect("write");
                    String::from_utf8(output).expect("utf8")
                }
                ExhaustivenessError::UnknownVariant { span, enum_name, variant } => {
                    let span_range = span_to_range(source, span);
                    let report = Report::build(ReportKind::Error, &filename, span_range.start)
                        .with_message(format!("Unknown variant `{variant}` in enum `{enum_name}`"))
                        .with_label(
                            Label::new((&filename, span_range))
                                .with_message(format!("`{enum_name}` has no variant `{variant}`"))
                                .with_color(Color::Red),
                        )
                        .finish();
                    let mut output = Vec::new();
                    report.write((&filename, Source::from(source)), &mut output).expect("write");
                    String::from_utf8(output).expect("utf8")
                }
            }
        }
    }
}

/// Convert a Span to a byte range for ariadne
fn span_to_range(source: &str, span: &Span) -> std::ops::Range<usize> {
    // Convert line:col to byte offset
    let start = line_col_to_offset(source, span.start_line, span.start_col);
    let end = line_col_to_offset(source, span.end_line, span.end_col);
    start..end
}

/// Convert line:col (1-indexed) to byte offset
fn line_col_to_offset(source: &str, line: usize, col: usize) -> usize {
    if line == 0 && col == 0 {
        // Dummy span, return 0
        return 0;
    }

    let mut current_line = 1;
    let mut offset = 0;

    for (idx, ch) in source.char_indices() {
        if current_line == line && offset + 1 == col {
            return idx;
        }
        if ch == '\n' {
            current_line += 1;
            offset = 0;
        } else {
            offset += 1;
        }
    }

    // If we didn't find the exact position, return the last offset
    source.len()
}

/// Get suggestion for lexical errors (with Lang4LLM-specific hints)
fn get_lex_error_suggestion(message: &str) -> &'static str {
    if message.contains("Invalid token") {
        "Check for unexpected characters — remember: `&&` → `and`, `||` → `or`, `!` → `not`"
    } else if message.contains("Unterminated") {
        "Make sure strings and comments are properly closed"
    } else {
        "Review the syntax for this token"
    }
}

/// Get suggestion for syntax errors (with Lang4LLM-specific hints)
fn get_syntax_error_suggestion(message: &str) -> &'static str {
    if message.contains("\"and\"") || message.contains("\"or\"") || message.contains("\"not\"") {
        "Lang4LLM uses keyword operators: `and` (not `&&`), `or` (not `||`), `not` (not `!`)"
    } else if message.contains("expected") {
        if message.contains("let") || message.contains("fn") {
            "Lang4LLM requires explicit type annotations on all declarations"
        } else if message.contains(";") {
            "Consider adding a semicolon to end the statement"
        } else if message.contains("}") || message.contains("{") {
            "Check that all braces are properly balanced"
        } else {
            "Review the grammar for this construct — use `none` instead of `null`/`nil`"
        }
    } else {
        "Check the syntax at this location"
    }
}

/// Get suggestion for type mismatches (Lang4LLM types)
fn get_type_mismatch_suggestion(expected: &str, found: &str) -> String {
    match (expected, found) {
        ("string", "int") | ("string", "float") | ("string", "bool") =>
            format!("use to_string({found}_value) to convert {found} to string"),
        ("int", "float") =>
            "use int(float_value) to truncate float to int".to_string(),
        ("float", "int") =>
            "use float(int_value) to widen int to float".to_string(),
        ("bool", _) =>
            format!("expected a boolean expression (true/false), found {found}"),
        ("unit", _) =>
            format!("this expression has type {found}; use `return;` for unit functions"),
        _ if expected.starts_with('[') && !found.starts_with('[') =>
            format!("expected an array type [{found}], found scalar {found}"),
        _ => String::new(),
    }
}

/// Get suggestion for binary operation errors
fn get_binary_op_suggestion(left: &str, right: &str) -> &'static str {
    if left.contains("String") || right.contains("String") {
        "String concatenation requires both operands to be strings"
    } else if left != right {
        "Both operands must have the same type for this operation"
    } else {
        "This operation is not supported for these types"
    }
}
