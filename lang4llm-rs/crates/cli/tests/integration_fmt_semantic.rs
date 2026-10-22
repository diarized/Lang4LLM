//! Semantic preservation tests (T062): format all 92 benchmark reference
//! implementations, then compile+run the formatted version and verify that
//! stdout matches the original expected output.
//!
//! Run from `lang4llm-rs/`:
//!   cargo test -p lang4llm-cli --test integration_fmt_semantic

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
struct Sidecar {
    #[serde(default)]
    lang4llm_compatible: bool,
    #[serde(default)]
    expected: Vec<String>,
}

fn reference_dirs() -> Vec<PathBuf> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../research/D4-benchmark/reference");
    vec![base.join("simple"), base.join("medium"), base.join("complex")]
}

fn collect_pairs(dirs: &[PathBuf]) -> Vec<(PathBuf, PathBuf)> {
    let mut pairs = Vec::new();
    for dir in dirs {
        if let Ok(rd) = fs::read_dir(dir) {
            let mut files: Vec<PathBuf> = rd
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().map(|e| e == "lang4llm").unwrap_or(false))
                .collect();
            files.sort();
            for f in files {
                let json = f.with_extension("json");
                if json.exists() {
                    pairs.push((f, json));
                }
            }
        }
    }
    pairs
}

fn compile_and_run(source: &str) -> Option<String> {
    // Write to temp file
    let tmp_src = std::env::temp_dir().join("fmt_sem_test.lang4llm");
    let tmp_rs = std::env::temp_dir().join("fmt_sem_test.rs");
    let tmp_bin = std::env::temp_dir().join("fmt_sem_test_bin");

    fs::write(&tmp_src, source).ok()?;

    // Transpile
    let sm = lang4llm_ast::SourceMap::from_source(source);
    let program = lang4llm_parser::parse(source, &sm).ok()?;
    let rust_src = lang4llm_transpiler::transpile(&program).ok()?;
    fs::write(&tmp_rs, &rust_src).ok()?;

    // Compile with rustc
    let status = Command::new("rustc")
        .args([
            tmp_rs.to_str()?,
            "-o",
            tmp_bin.to_str()?,
            "--edition", "2021",
            "-A", "warnings",
        ])
        .output()
        .ok()?;

    if !status.status.success() {
        return None;
    }

    // Run
    let out = Command::new(&tmp_bin).output().ok()?;
    let _ = fs::remove_file(&tmp_bin);

    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

#[test]
fn test_semantic_preservation_all_benchmarks() {
    let dirs = reference_dirs();
    let pairs = collect_pairs(&dirs);
    assert!(!pairs.is_empty(), "no benchmark pairs found");

    let mut skipped = 0usize;
    let mut passed = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for (lang_path, json_path) in &pairs {
        // Load sidecar
        let sidecar_raw = fs::read_to_string(json_path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", json_path.display()));
        let sidecar: Sidecar = serde_json::from_str(&sidecar_raw)
            .unwrap_or_else(|e| panic!("bad sidecar {}: {e}", json_path.display()));

        if !sidecar.lang4llm_compatible {
            skipped += 1;
            continue;
        }

        // Read original source
        let source = fs::read_to_string(lang_path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", lang_path.display()));

        // Format once
        let formatted = match lang4llm_formatter::format_source(&source) {
            Ok(f) => f,
            Err(e) => {
                failures.push(format!("FORMAT ERROR {}: {e}", lang_path.display()));
                continue;
            }
        };

        // Compile and run the formatted version
        let stdout = match compile_and_run(&formatted) {
            Some(s) => s,
            None => {
                failures.push(format!(
                    "COMPILE/RUN FAILED: {}",
                    lang_path.display()
                ));
                continue;
            }
        };

        // Match expected strings using same substring-contains logic as eval harness.
        // This tolerates pre-existing reference impl quirks (e.g. BS-S-030 duplicate line).
        let mut missing: Vec<&str> = Vec::new();
        for exp in &sidecar.expected {
            if !stdout.contains(exp.as_str()) {
                missing.push(exp.as_str());
            }
        }

        if !missing.is_empty() {
            failures.push(format!(
                "SEMANTIC MISMATCH: {}\n  missing lines: {:?}\n  got: {:?}",
                lang_path.display(),
                missing,
                stdout.lines().collect::<Vec<_>>()
            ));
        } else {
            passed += 1;
        }
    }

    if !failures.is_empty() {
        panic!(
            "{} semantic preservation failures ({} passed, {} skipped):\n{}",
            failures.len(),
            passed,
            skipped,
            failures.join("\n\n")
        );
    }

    eprintln!(
        "semantic preservation: {passed} passed, {skipped} skipped"
    );
}
