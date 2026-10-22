//! Integration test: examples with print() output produce expected runtime results.
//!
//! Only tests examples that have print() calls with known expected output.
//! Skipped if rustc is not available.

use std::path::PathBuf;
use std::process::Command;

fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()  // crates/
        .parent().unwrap()  // lang4llm-rs/
        .parent().unwrap()  // Lang4LLM_specification/
        .join("language-spec/examples")
}

fn rustc_available() -> bool {
    Command::new("rustc").arg("--version").output().is_ok()
}

fn transpile_to_string(name: &str) -> String {
    let path = examples_dir().join(name);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {name}: {e}"));
    let sm = lang4llm_ast::SourceMap::from_source(&source);
    let program = lang4llm_parser::parse(&source, &sm)
        .unwrap_or_else(|e| panic!("parse {name}: {e:?}"));
    lang4llm_typeck::check_program(&program)
        .unwrap_or_else(|e| panic!("typecheck {name}: {e:?}"));
    lang4llm_transpiler::transpile(&program)
        .unwrap_or_else(|e| panic!("transpile {name}: {e:?}"))
}

fn compile_and_run(name: &str, source: &str) -> Option<String> {
    let safe = name.replace(['.', '/'], "_");
    let tmp_src = std::env::temp_dir().join(format!("lang4llm_run_{}.rs", safe));
    let tmp_bin = std::env::temp_dir().join(format!("lang4llm_run_{}_bin", safe));

    std::fs::write(&tmp_src, source).ok()?;

    let compile = Command::new("rustc")
        .args(["--edition", "2021", "-o"])
        .arg(&tmp_bin)
        .arg(&tmp_src)
        .output()
        .ok()?;

    if !compile.status.success() {
        let _ = std::fs::remove_file(&tmp_src);
        return None;
    }

    let run = Command::new(&tmp_bin).output().ok()?;
    let _ = std::fs::remove_file(&tmp_src);
    let _ = std::fs::remove_file(&tmp_bin);

    Some(String::from_utf8_lossy(&run.stdout).to_string())
}

macro_rules! run_test {
    ($name:ident, $file:expr, $expected:expr) => {
        #[test]
        fn $name() {
            if !rustc_available() {
                eprintln!("rustc not found — skipping runtime test for {}", $file);
                return;
            }
            let rust_src = transpile_to_string($file);
            let output = compile_and_run($file, &rust_src)
                .expect("should compile and run");
            let expected: &str = $expected;
            assert!(
                output.contains(expected),
                "expected output to contain {:?}, got {:?}",
                expected,
                output
            );
        }
    };
}

// Example 01: Hello World → "Hello, World!"
run_test!(run_01_hello_world,  "01_hello_world.lang4llm",       "Hello, World!");

// Example 02: Fibonacci → "Fibonacci(10) = 55"
run_test!(run_02_fibonacci,    "02_fibonacci.lang4llm",         "Fibonacci(10) = 55");

// Example 07: Arrays → "Sum: 15"
run_test!(run_07_arrays,       "07_arrays.lang4llm",            "Sum: 15");

// Example 10: Complex matching → "Running at 75%"
run_test!(run_10_complex_matching, "10_complex_matching.lang4llm", "Running at 75%");

// Example 11: Loop control → "First even: 8"
run_test!(run_11_loop_control, "11_loop_control.lang4llm",      "First even: 8");

// Example 13: Stdlib math → pow(2,10)=1024
run_test!(run_13_stdlib_math,        "13_stdlib_math.lang4llm",        "pow(2, 10) = 1024");

// Example 14: Stdlib strings → joined output
run_test!(run_14_stdlib_strings,     "14_stdlib_strings.lang4llm",     "apple | banana | cherry");

// Example 15: Stdlib collections → first = 42
run_test!(run_15_stdlib_collections, "15_stdlib_collections.lang4llm", "first: 42");
