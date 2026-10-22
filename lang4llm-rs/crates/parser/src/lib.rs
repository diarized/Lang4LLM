//! Parser for Lang4LLM
//!
//! Converts Lang4LLM source code into an Abstract Syntax Tree (AST).
//! Uses LALRPOP for parser generation with a custom lexer.

#![warn(clippy::all)]
// Disable missing_docs for generated grammar module
#![allow(missing_docs)]

use lang4llm_ast::{Program, SourceMap, Span};
use lalrpop_util::lalrpop_mod;

/// Lexer module
pub mod lexer;

// Include LALRPOP-generated parser
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar
);

/// Parse error type
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Lexical error
    #[error("Lexical error at {location}: {message}")]
    LexError {
        /// Location of the error
        location: Span,
        /// Error message
        message: String,
    },

    /// Syntax error
    #[error("Syntax error at {location}: {message}")]
    SyntaxError {
        /// Location of the error
        location: Span,
        /// Error message
        message: String,
    },
}

/// Parse a Lang4LLM source string into an AST
///
/// # Errors
///
/// Returns `ParseError` if the source cannot be parsed.
pub fn parse(source: &str, source_map: &SourceMap) -> Result<Program, ParseError> {
    let lexer = lexer::Lexer::new(source);
    let parser = grammar::ProgramParser::new();

    parser.parse(source_map, lexer).map_err(|e| {
        match e {
            lalrpop_util::ParseError::InvalidToken { location } => ParseError::LexError {
                location: source_map.span(location, location),
                message: "Invalid token".to_string(),
            },
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                ParseError::SyntaxError {
                    location: source_map.span(location, location),
                    message: format!("Unexpected end of file, expected one of: {}", expected.join(", ")),
                }
            }
            lalrpop_util::ParseError::UnrecognizedToken { token: (start, _tok, end), expected } => {
                ParseError::SyntaxError {
                    location: source_map.span(start, end),
                    message: format!("Unexpected token, expected one of: {}", expected.join(", ")),
                }
            }
            lalrpop_util::ParseError::ExtraToken { token: (start, _tok, end) } => {
                ParseError::SyntaxError {
                    location: source_map.span(start, end),
                    message: "Extra token".to_string(),
                }
            }
            lalrpop_util::ParseError::User { error } => ParseError::SyntaxError {
                location: Span::dummy(),
                message: error,
            },
        }
    })
}
