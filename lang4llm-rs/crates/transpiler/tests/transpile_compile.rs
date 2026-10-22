//! Integration test: transpiled examples compile with rustc.
//!
//! All 15 examples compile successfully.
//! Tests are skipped if `rustc` is not available in PATH.

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

fn compile_rust_source(name: &str, source: &str) -> bool {
    let safe_name = name.replace('.', "_");
    let tmp_src = std::env::temp_dir().join(format!("lang4llm_test_{}.rs", safe_name));
    let tmp_bin = std::env::temp_dir().join(format!("lang4llm_test_{}_bin", safe_name));

    std::fs::write(&tmp_src, source).expect("write temp .rs file");

    let output = Command::new("rustc")
        .args(["--edition", "2021", "-o"])
        .arg(&tmp_bin)
        .arg(&tmp_src)
        .output()
        .expect("invoke rustc");

    let _ = std::fs::remove_file(&tmp_src);
    let _ = std::fs::remove_file(&tmp_bin);

    if !output.status.success() {
        eprintln!(
            "rustc stderr for {}:\n{}",
            name,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    output.status.success()
}


macro_rules! compile_test {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            if !rustc_available() {
                eprintln!("rustc not found — skipping {}", $file);
                return;
            }
            let rust_src = transpile_to_string($file);
            assert!(
                compile_rust_source($file, &rust_src),
                "rustc should compile transpiled output for {}",
                $file
            );
        }
    };
}

// All 15 examples that compile
compile_test!(compile_01_hello_world,        "01_hello_world.lang4llm");
compile_test!(compile_02_fibonacci,          "02_fibonacci.lang4llm");
compile_test!(compile_03_pattern_matching,   "03_pattern_matching.lang4llm");
compile_test!(compile_04_structs,            "04_structs.lang4llm");
compile_test!(compile_05_generics,           "05_generics.lang4llm");
compile_test!(compile_06_result_error,       "06_result_error_handling.lang4llm");
compile_test!(compile_07_arrays,             "07_arrays.lang4llm");
compile_test!(compile_08_closures,           "08_closures.lang4llm");
compile_test!(compile_09_traits,             "09_traits.lang4llm");
compile_test!(compile_10_complex_matching,   "10_complex_matching.lang4llm");
compile_test!(compile_11_loop_control,       "11_loop_control.lang4llm");
compile_test!(compile_12_ranges,             "12_ranges.lang4llm");
compile_test!(compile_13_stdlib_math,        "13_stdlib_math.lang4llm");
compile_test!(compile_14_stdlib_strings,     "14_stdlib_strings.lang4llm");
compile_test!(compile_15_stdlib_collections, "15_stdlib_collections.lang4llm");
