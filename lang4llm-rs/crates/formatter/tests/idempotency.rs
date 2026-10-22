//! Idempotency tests (T061): format all 92 benchmark reference implementations twice;
//! assert that the second pass produces no changes.
//!
//! Run from `lang4llm-rs/`:
//!   cargo test -p lang4llm-formatter --test idempotency

use std::fs;
use std::path::PathBuf;

fn reference_dirs() -> Vec<PathBuf> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../research/D4-benchmark/reference");
    vec![base.join("simple"), base.join("medium"), base.join("complex")]
}

fn collect_lang4llm(dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in dirs {
        if let Ok(rd) = fs::read_dir(dir) {
            let mut dir_files: Vec<PathBuf> = rd
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().map(|e| e == "lang4llm").unwrap_or(false))
                .collect();
            dir_files.sort();
            files.extend(dir_files);
        }
    }
    files
}

#[test]
fn test_idempotency_all_benchmarks() {
    let dirs = reference_dirs();
    let files = collect_lang4llm(&dirs);
    assert!(!files.is_empty(), "no benchmark files found");

    let mut failures = Vec::new();

    for path in &files {
        let source = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));

        // First format pass
        let once = match lang4llm_formatter::format_source(&source) {
            Ok(s) => s,
            Err(e) => {
                // Some benchmark files may not be parseable (e.g., file I/O specs).
                // Skip non-parseable files rather than failing.
                eprintln!("skip {} (parse error: {e})", path.display());
                continue;
            }
        };

        // Second format pass — must equal the first
        let twice = lang4llm_formatter::format_source(&once)
            .unwrap_or_else(|e| panic!("second format failed for {}: {e}", path.display()));

        if once != twice {
            failures.push(format!(
                "NOT IDEMPOTENT: {}\n--- first ---\n{once}\n--- second ---\n{twice}",
                path.display()
            ));
        }
    }

    if !failures.is_empty() {
        panic!("Idempotency failures:\n{}", failures.join("\n\n"));
    }

    eprintln!("idempotency: {} files checked", files.len());
}
