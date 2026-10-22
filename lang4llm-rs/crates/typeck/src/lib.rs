//! Type checker for Lang4LLM
//!
//! Validates type correctness, exhaustive matching, and initialization.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Built-in functions and types available without explicit import
pub mod builtins;
/// Exhaustiveness checker for pattern matching (DR-ERR-004)
pub mod exhaustive;
/// Symbol table for name resolution and scoping
pub mod symtab;
/// Type checker — core type inference and validation logic
pub mod typecheck;

use lang4llm_ast::Program;
pub use builtins::initialize_builtins;
pub use exhaustive::{ExhaustivenessChecker, ExhaustivenessError};
pub use symtab::{Symbol, SymbolError, SymbolKind, SymbolTable};
pub use typecheck::{TypeChecker, TypeError};

/// Type check a program
///
/// # Errors
///
/// Returns `TypeError` if the program has type errors.
pub fn check_program(program: &Program) -> Result<(), Vec<TypeError>> {
    let mut checker = TypeChecker::new();
    checker.check_program(program)
}
