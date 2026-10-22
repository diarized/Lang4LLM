//! Lang4LLM source code formatter.
//!
//! Parses a `.lang4llm` source string, pretty-prints it to canonical style,
//! and returns the formatted string. The formatter is idempotent:
//! `format_source(format_source(x)) == format_source(x)`.
//!
//! # Entry points
//!
//! - [`format_source`] — format a source string
//! - [`check_formatted`] — returns `true` if source is already canonical

pub mod context;
pub mod exprs;
pub mod items;
pub mod patterns;
pub mod stmts;
pub mod types;

use context::FmtContext;
use items::fmt_item;
use lang4llm_ast::{Program, SourceMap};
use lang4llm_parser::ParseError;
use thiserror::Error;

// ─── Public error type ───────────────────────────────────────────────────────

/// Error returned by formatter entry points.
#[derive(Debug, Error)]
pub enum FormatError {
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}

// ─── Entry points ────────────────────────────────────────────────────────────

/// Format a Lang4LLM source string to canonical style.
///
/// Returns the formatted source. If the source contains parse errors, returns
/// a `FormatError::Parse`.
pub fn format_source(source: &str) -> Result<String, FormatError> {
    let sm = SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm)?;
    Ok(fmt_program(&program))
}

/// Returns `true` if `source` is already in canonical format.
pub fn check_formatted(source: &str) -> Result<bool, FormatError> {
    let formatted = format_source(source)?;
    Ok(source == formatted)
}

// ─── Core formatter ──────────────────────────────────────────────────────────

/// Pretty-print a parsed `Program` back to Lang4LLM source.
pub fn fmt_program(program: &Program) -> String {
    let mut ctx = FmtContext::new();

    // Module declaration
    if let Some(module) = &program.module {
        ctx.write("module ");
        ctx.write(&module.name.name);
        ctx.writeln(";");
        ctx.newline();
    }

    // Import statements
    for import in &program.imports {
        ctx.write("import ");
        for (i, seg) in import.path.iter().enumerate() {
            if i > 0 {
                ctx.write(".");
            }
            ctx.write(&seg.name);
        }
        ctx.writeln(";");
    }
    if !program.imports.is_empty() {
        ctx.newline();
    }

    // Top-level items — separated by blank lines
    for (i, item) in program.items.iter().enumerate() {
        if i > 0 {
            ctx.newline();
        }
        fmt_item(item, &mut ctx);
    }

    ctx.finish()
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(source: &str) -> String {
        format_source(source).expect("should format without error")
    }

    // ── Basic function ───────────────────────────────────────────────────────

    #[test]
    fn test_simple_function() {
        let src = r#"fn main() -> unit {
    return;
}"#;
        let out = fmt(src);
        assert!(out.contains("fn main() -> unit {"));
        assert!(out.contains("return;"));
    }

    #[test]
    fn test_function_with_params() {
        let src = r#"fn add(a: int, b: int) -> int {
    return a + b;
}"#;
        let out = fmt(src);
        assert!(out.contains("fn add(a: int, b: int) -> int {"));
        assert!(out.contains("return a + b;"));
    }

    // ── Indentation ──────────────────────────────────────────────────────────

    #[test]
    fn test_indentation_4_spaces() {
        let src = r#"fn f() -> unit {
    let x: int = 1;
    return;
}"#;
        let out = fmt(src);
        // let statement should be indented exactly 4 spaces
        let let_line = out.lines().find(|l| l.trim_start().starts_with("let ")).unwrap();
        let leading = let_line.len() - let_line.trim_start().len();
        assert_eq!(leading, 4, "expected 4-space indent, got {leading}");
    }

    // ── Operator spacing ─────────────────────────────────────────────────────

    #[test]
    fn test_binary_op_spacing() {
        let src = "fn f() -> int { return 1 + 2; }";
        let out = fmt(src);
        assert!(out.contains("1 + 2"), "missing spaces around +: {out}");
    }

    #[test]
    fn test_boolean_ops_use_keywords() {
        let src = r#"fn f(a: bool, b: bool) -> bool {
    return a and b;
}"#;
        let out = fmt(src);
        assert!(out.contains("a and b"), "should use 'and' keyword: {out}");
    }

    // ── Struct literal ───────────────────────────────────────────────────────

    #[test]
    fn test_struct_definition() {
        let src = r#"struct Point {
    x: int,
    y: int,
}
fn main() -> unit {
    return;
}"#;
        let out = fmt(src);
        assert!(out.contains("struct Point {"));
        assert!(out.contains("    x: int,"));
        assert!(out.contains("    y: int,"));
    }

    // ── Match arms ───────────────────────────────────────────────────────────

    #[test]
    fn test_match_formatting() {
        let src = r#"fn describe(n: int) -> string {
    return match n {
        0 => "zero",
        _ => "other",
    };
}"#;
        let out = fmt(src);
        assert!(out.contains("match n {"), "match keyword: {out}");
        assert!(out.contains("0 => \"zero\","), "match arm: {out}");
        assert!(out.contains("_ => \"other\","), "wildcard arm: {out}");
    }

    // ── Idempotency ──────────────────────────────────────────────────────────

    #[test]
    fn test_idempotent_simple() {
        let src = r#"fn add(a: int, b: int) -> int {
    return a + b;
}
fn main() -> unit {
    let result: int = add(1, 2);
    return;
}"#;
        let once = fmt(src);
        let twice = fmt(&once);
        assert_eq!(once, twice, "formatter should be idempotent");
    }

    #[test]
    fn test_idempotent_with_struct() {
        let src = r#"struct Point {
    x: int,
    y: int,
}
fn main() -> unit {
    let p: Point = Point { x: 1, y: 2 };
    return;
}"#;
        let once = fmt(src);
        let twice = fmt(&once);
        assert_eq!(once, twice, "formatter should be idempotent");
    }
}
