//! Integration test: all 15 example programs transpile to non-empty Rust source.
//!
//! This test verifies the transpile() API returns Ok for every example.
//! It does NOT check that the output compiles — see transpile_compile.rs for that.

use std::path::PathBuf;

fn examples_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR = lang4llm-rs/crates/transpiler
    // → lang4llm-rs/crates → lang4llm-rs → Lang4LLM_specification
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()  // crates/
        .parent().unwrap()  // lang4llm-rs/
        .parent().unwrap()  // Lang4LLM_specification/
        .to_path_buf();
    repo.join("language-spec/examples")
}

fn transpile_file(name: &str) -> Result<String, String> {
    let path = examples_dir().join(name);
    let source =
        std::fs::read_to_string(&path).map_err(|e| format!("read {name}: {e}"))?;

    let sm = lang4llm_ast::SourceMap::from_source(&source);
    let program = lang4llm_parser::parse(&source, &sm)
        .map_err(|e| format!("parse {name}: {e:?}"))?;

    lang4llm_typeck::check_program(&program)
        .map_err(|errs| format!("typecheck {name}: {:?}", errs))?;

    lang4llm_transpiler::transpile(&program)
        .map_err(|errs| format!("transpile {name}: {:?}", errs))
}

macro_rules! transpile_test {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            let result = transpile_file($file);
            let rust_src = result.expect("should transpile without error");
            assert!(!rust_src.is_empty(), "output should be non-empty");
            assert!(rust_src.contains("fn main"), "output should contain fn main");
        }
    };
}

transpile_test!(transpile_01_hello_world,        "01_hello_world.lang4llm");
transpile_test!(transpile_02_fibonacci,          "02_fibonacci.lang4llm");
transpile_test!(transpile_03_pattern_matching,   "03_pattern_matching.lang4llm");
transpile_test!(transpile_04_structs,            "04_structs.lang4llm");
transpile_test!(transpile_05_generics,           "05_generics.lang4llm");
transpile_test!(transpile_06_result_error,       "06_result_error_handling.lang4llm");
transpile_test!(transpile_07_arrays,             "07_arrays.lang4llm");
transpile_test!(transpile_08_closures,           "08_closures.lang4llm");
transpile_test!(transpile_09_traits,             "09_traits.lang4llm");
transpile_test!(transpile_10_complex_matching,   "10_complex_matching.lang4llm");
transpile_test!(transpile_11_loop_control,       "11_loop_control.lang4llm");
transpile_test!(transpile_12_ranges,             "12_ranges.lang4llm");
transpile_test!(transpile_13_stdlib_math,        "13_stdlib_math.lang4llm");
transpile_test!(transpile_14_stdlib_strings,     "14_stdlib_strings.lang4llm");
transpile_test!(transpile_15_stdlib_collections, "15_stdlib_collections.lang4llm");
