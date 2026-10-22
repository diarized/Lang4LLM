//! Integration tests for `lang4llm eval` subcommand.
//!
//! Tests the eval harness against known good reference implementations.
//! Skipped if rustc is not available.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap() // crates/
        .parent().unwrap() // lang4llm-rs/
        .parent().unwrap() // Lang4LLM_specification/
        .to_path_buf()
}

fn bin() -> PathBuf {
    // Workspace root is 2 levels up from crates/cli/
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap() // crates/
        .parent().unwrap() // lang4llm-rs/
        .to_path_buf();
    workspace_root.join("target/debug/lang4llm")
}

fn rustc_available() -> bool {
    Command::new("rustc").arg("--version").output().is_ok()
}

/// Eval a single existing example file — should complete without error.
#[test]
fn eval_hello_world_no_check() {
    if !rustc_available() { return; }
    let file = repo_root().join("language-spec/examples/01_hello_world.lang4llm");
    let out = Command::new(bin())
        .args(["eval"])
        .arg(&file)
        .output()
        .expect("lang4llm binary not found");
    assert!(out.status.success(), "eval should succeed: {:?}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("OK"), "expected OK in output, got: {stdout}");
}

/// Eval with explicit expected strings that we know are in hello world output.
#[test]
fn eval_hello_world_with_expected() {
    if !rustc_available() { return; }

    // Write a temp sidecar JSON
    let tmp = std::env::temp_dir().join("l4l_test_eval_hw.lang4llm");
    let tmp_json = std::env::temp_dir().join("l4l_test_eval_hw.json");

    let src = repo_root().join("language-spec/examples/01_hello_world.lang4llm");
    std::fs::copy(&src, &tmp).unwrap();

    let json = r#"{
  "spec": "test-hw",
  "title": "Hello World",
  "lang4llm_compatible": true,
  "expected": ["Hello, World!"]
}"#;
    std::fs::write(&tmp_json, json).unwrap();

    let out = Command::new(bin())
        .args(["eval", "--check"])
        .arg(&tmp)
        .output()
        .expect("lang4llm binary not found");

    let _ = std::fs::remove_file(&tmp);
    let _ = std::fs::remove_file(&tmp_json);

    assert!(out.status.success(), "eval --check should pass: {:?}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("PASS"), "expected PASS, got: {stdout}");
}

/// Eval with a wrong expected string should fail.
#[test]
fn eval_wrong_expected_fails() {
    if !rustc_available() { return; }

    let tmp = std::env::temp_dir().join("l4l_test_eval_fail.lang4llm");
    let tmp_json = std::env::temp_dir().join("l4l_test_eval_fail.json");

    let src = repo_root().join("language-spec/examples/01_hello_world.lang4llm");
    std::fs::copy(&src, &tmp).unwrap();

    let json = r#"{
  "spec": "test-fail",
  "title": "Wrong expected",
  "lang4llm_compatible": true,
  "expected": ["THIS STRING WILL NEVER APPEAR IN OUTPUT xyz123"]
}"#;
    std::fs::write(&tmp_json, json).unwrap();

    let out = Command::new(bin())
        .args(["eval", "--check"])
        .arg(&tmp)
        .output()
        .expect("lang4llm binary not found");

    let _ = std::fs::remove_file(&tmp);
    let _ = std::fs::remove_file(&tmp_json);

    assert!(!out.status.success(), "eval with wrong expected should fail");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("FAIL"), "expected FAIL marker, got: {stdout}");
}

/// Suite mode: run the examples/ directory (no JSON sidecars — should report OK).
#[test]
fn eval_suite_examples_dir() {
    if !rustc_available() { return; }
    let dir = repo_root().join("language-spec/examples");
    let out = Command::new(bin())
        .args(["eval", "--suite"])
        .arg(&dir)
        .output()
        .expect("lang4llm binary not found");
    // Should not panic/crash; examples may or may not all run but suite mode should exit
    // We just verify the command runs without a Rust panic
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!stderr.contains("panic"), "should not panic: {stderr}");
}
