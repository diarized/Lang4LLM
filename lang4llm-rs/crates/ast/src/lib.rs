//! Abstract Syntax Tree (AST) definitions for Lang4LLM
//!
//! This crate provides the AST node types that represent Lang4LLM programs.
//! The AST is produced by the parser and consumed by the type checker.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Source location tracking
pub mod span;

/// Source map: byte offset → line:col conversion
pub mod source_map;

/// AST node definitions
pub mod ast;

/// AST visitor trait for traversal
pub mod visitor;

#[cfg(test)]
mod tests;

// Re-export main types
pub use ast::*;
pub use source_map::SourceMap;
pub use span::Span;
