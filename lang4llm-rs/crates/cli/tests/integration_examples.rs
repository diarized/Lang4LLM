//! Integration tests for all 12 Lang4LLM example programs
//!
//! These tests verify that each example program:
//! 1. Parses successfully (no syntax errors)
//! 2. Type-checks successfully (no type errors)
//! 3. Demonstrates language features correctly

use std::fs;
use std::path::PathBuf;

/// Helper function to get the path to an example file
fn example_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to crates/
    path.pop(); // Go up to lang4llm-rs/
    path.pop(); // Go up to project root
    path.push("language-spec");
    path.push("examples");
    path.push(filename);
    path
}

/// Helper function to parse and type-check a program
fn check_program(source: &str) -> Result<(), String> {
    // Parse
    let sm = lang4llm_ast::SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm)
        .map_err(|e| format!("Parse error: {:?}", e))?;

    // Type check
    lang4llm_typeck::check_program(&program)
        .map_err(|errors| {
            let error_messages: Vec<String> = errors
                .iter()
                .map(|e| format!("{:?}", e))
                .collect();
            format!("Type errors:\n{}", error_messages.join("\n"))
        })?;

    Ok(())
}

// ============================================================================
// Example 01: Hello World
// ============================================================================

#[test]
fn test_example_01_hello_world() {
    let path = example_path("01_hello_world.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 01: Hello World - PASSED"),
        Err(e) => panic!("Example 01 failed:\n{}", e),
    }
}

// ============================================================================
// Example 02: Fibonacci
// ============================================================================

#[test]
fn test_example_02_fibonacci() {
    let path = example_path("02_fibonacci.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 02: Fibonacci - PASSED"),
        Err(e) => panic!("Example 02 failed:\n{}", e),
    }
}

// ============================================================================
// Example 03: Pattern Matching
// ============================================================================

#[test]
fn test_example_03_pattern_matching() {
    let path = example_path("03_pattern_matching.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 03: Pattern Matching - PASSED"),
        Err(e) => panic!("Example 03 failed:\n{}", e),
    }
}

// ============================================================================
// Example 04: Structs
// ============================================================================

#[test]
fn test_example_04_structs() {
    let path = example_path("04_structs.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 04: Structs - PASSED"),
        Err(e) => panic!("Example 04 failed:\n{}", e),
    }
}

// ============================================================================
// Example 05: Generics
// ============================================================================

#[test]
fn test_example_05_generics() {
    let path = example_path("05_generics.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 05: Generics - PASSED"),
        Err(e) => panic!("Example 05 failed:\n{}", e),
    }
}

// ============================================================================
// Example 06: Result Error Handling
// ============================================================================

#[test]
fn test_example_06_result_error_handling() {
    let path = example_path("06_result_error_handling.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 06: Result Error Handling - PASSED"),
        Err(e) => panic!("Example 06 failed:\n{}", e),
    }
}

// ============================================================================
// Example 07: Arrays
// ============================================================================

#[test]
fn test_example_07_arrays() {
    let path = example_path("07_arrays.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 07: Arrays - PASSED"),
        Err(e) => panic!("Example 07 failed:\n{}", e),
    }
}

// ============================================================================
// Example 08: Closures
// ============================================================================

#[test]
fn test_example_08_closures() {
    let path = example_path("08_closures.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 08: Closures - PASSED"),
        Err(e) => panic!("Example 08 failed:\n{}", e),
    }
}

// ============================================================================
// Example 09: Traits
// ============================================================================

#[test]
fn test_example_09_traits() {
    let path = example_path("09_traits.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 09: Traits - PASSED"),
        Err(e) => panic!("Example 09 failed:\n{}", e),
    }
}

// ============================================================================
// Example 10: Complex Matching
// ============================================================================

#[test]
fn test_example_10_complex_matching() {
    let path = example_path("10_complex_matching.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 10: Complex Matching - PASSED"),
        Err(e) => panic!("Example 10 failed:\n{}", e),
    }
}

// ============================================================================
// Example 11: Loop Control
// ============================================================================

#[test]
fn test_example_11_loop_control() {
    let path = example_path("11_loop_control.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 11: Loop Control - PASSED"),
        Err(e) => panic!("Example 11 failed:\n{}", e),
    }
}

// ============================================================================
// Example 12: Ranges
// ============================================================================

#[test]
fn test_example_12_ranges() {
    let path = example_path("12_ranges.lang4llm");
    let source = fs::read_to_string(&path)
        .expect(&format!("Failed to read {}", path.display()));

    match check_program(&source) {
        Ok(()) => println!("✓ Example 12: Ranges - PASSED"),
        Err(e) => panic!("Example 12 failed:\n{}", e),
    }
}

// ============================================================================
// Summary Test: Run all examples together
// ============================================================================

#[test]
fn test_all_examples_summary() {
    let examples = vec![
        ("01_hello_world.lang4llm", "Hello World"),
        ("02_fibonacci.lang4llm", "Fibonacci"),
        ("03_pattern_matching.lang4llm", "Pattern Matching"),
        ("04_structs.lang4llm", "Structs"),
        ("05_generics.lang4llm", "Generics"),
        ("06_result_error_handling.lang4llm", "Result Error Handling"),
        ("07_arrays.lang4llm", "Arrays"),
        ("08_closures.lang4llm", "Closures"),
        ("09_traits.lang4llm", "Traits"),
        ("10_complex_matching.lang4llm", "Complex Matching"),
        ("11_loop_control.lang4llm", "Loop Control"),
        ("12_ranges.lang4llm", "Ranges"),
    ];

    let mut passed = 0;
    let mut failed = Vec::new();

    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Integration Tests: All 12 Lang4LLM Example Programs");
    println!("═══════════════════════════════════════════════════════════\n");

    for (filename, name) in &examples {
        let path = example_path(filename);
        match fs::read_to_string(&path) {
            Ok(source) => {
                match check_program(&source) {
                    Ok(()) => {
                        println!("  ✓ {:02}. {} - PASSED", passed + 1, name);
                        passed += 1;
                    }
                    Err(e) => {
                        println!("  ✗ {}. {} - FAILED", failed.len() + 1, name);
                        failed.push((name, e));
                    }
                }
            }
            Err(e) => {
                println!("  ✗ {}. {} - FAILED (file not found)", failed.len() + 1, name);
                failed.push((name, format!("File read error: {}", e)));
            }
        }
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("  Results: {}/{} tests passed", passed, examples.len());
    println!("═══════════════════════════════════════════════════════════\n");

    if !failed.is_empty() {
        println!("Failed tests:\n");
        for (name, error) in &failed {
            println!("  ✗ {}:", name);
            println!("    {}\n", error);
        }
        panic!("{} test(s) failed", failed.len());
    }
}
