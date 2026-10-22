//! Integration tests for the lexer
//!
//! Tests the lexer against all 12 example Lang4LLM programs.

use lang4llm_parser::lexer::{Lexer, TokenKind};
use std::fs;
use std::path::Path;

fn read_example(name: &str) -> String {
    // CARGO_MANIFEST_DIR is lang4llm-rs/crates/parser
    // We need to go up to lang4llm-rs/../language-spec/examples
    let parser_crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lang4llm_rs_dir = parser_crate_dir.parent().unwrap().parent().unwrap();
    let repo_root = lang4llm_rs_dir.parent().unwrap();

    let example_path = repo_root.join("language-spec").join("examples").join(name);

    fs::read_to_string(&example_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", example_path.display(), e))
}

#[test]
fn test_lex_hello_world() {
    let source = read_example("01_hello_world.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: fn main () -> unit { print ( "..." ) ; return none ; } EOF
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Fn));
    assert!(tokens
        .iter()
        .any(|t| matches!(&t.kind, TokenKind::Ident((s, _)) if s == "main")));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Unit));
    assert!(tokens
        .iter()
        .any(|t| matches!(&t.kind, TokenKind::StringLit(_))));
    assert!(tokens.last().unwrap().kind == TokenKind::Eof);
}

#[test]
fn test_lex_fibonacci() {
    let source = read_example("02_fibonacci.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: let mut for in range
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Mut));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::For));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::In));
    assert!(tokens
        .iter()
        .any(|t| matches!(&t.kind, TokenKind::Ident((s, _)) if s == "range")));
}

#[test]
fn test_lex_pattern_matching() {
    let source = read_example("03_pattern_matching.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: enum match =>
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Enum));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Match));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::FatArrow));
}

#[test]
fn test_lex_structs() {
    let source = read_example("04_structs.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: struct
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Struct));
}

#[test]
fn test_lex_generics() {
    let source = read_example("05_generics.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: impl, <, >
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Impl));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Lt));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Gt));
}

#[test]
fn test_lex_result_error_handling() {
    let source = read_example("06_result_error_handling.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should parse successfully
    assert!(tokens.last().unwrap().kind == TokenKind::Eof);
}

#[test]
fn test_lex_arrays() {
    let source = read_example("07_arrays.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: [ ]
    assert!(tokens.iter().any(|t| t.kind == TokenKind::LBracket));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::RBracket));
}

#[test]
fn test_lex_closures() {
    let source = read_example("08_closures.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should parse successfully
    assert!(tokens.last().unwrap().kind == TokenKind::Eof);
}

#[test]
fn test_lex_traits() {
    let source = read_example("09_traits.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: trait impl
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Trait));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Impl));
}

#[test]
fn test_lex_complex_matching() {
    let source = read_example("10_complex_matching.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: match enum
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Match));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Enum));
}

#[test]
fn test_lex_loop_control() {
    let source = read_example("11_loop_control.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: while, continue (example uses while loop + continue)
    assert!(tokens.iter().any(|t| t.kind == TokenKind::While));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Continue));
}

#[test]
fn test_lex_ranges() {
    let source = read_example("12_ranges.lang4llm");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    // Should contain: .. (range operators)
    assert!(tokens
        .iter()
        .any(|t| t.kind == TokenKind::Range || t.kind == TokenKind::RangeEq));
}

#[test]
fn test_all_examples_tokenize() {
    let examples = [
        "01_hello_world.lang4llm",
        "02_fibonacci.lang4llm",
        "03_pattern_matching.lang4llm",
        "04_structs.lang4llm",
        "05_generics.lang4llm",
        "06_result_error_handling.lang4llm",
        "07_arrays.lang4llm",
        "08_closures.lang4llm",
        "09_traits.lang4llm",
        "10_complex_matching.lang4llm",
        "11_loop_control.lang4llm",
        "12_ranges.lang4llm",
    ];

    for example in examples {
        let source = read_example(example);
        let mut lexer = Lexer::new(&source);
        let result = lexer.tokenize();

        assert!(
            result.is_ok(),
            "Failed to tokenize {}: {:?}",
            example,
            result.err()
        );

        let tokens = result.unwrap();
        assert!(!tokens.is_empty(), "No tokens produced for {}", example);
        assert_eq!(
            tokens.last().unwrap().kind,
            TokenKind::Eof,
            "Last token should be EOF for {}",
            example
        );
    }
}
