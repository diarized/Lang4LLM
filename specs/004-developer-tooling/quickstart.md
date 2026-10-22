# Quickstart: Phase 7 — Developer Tooling

**Branch**: `004-developer-tooling` | **Date**: 2026-03-02

---

## Prerequisites

- Rust 1.75+ (`rustup show` to verify)
- All Phase 6 tests passing: `cd lang4llm-rs && cargo test --workspace`
- Benchmark baseline: `./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/simple/` (36/36)

## Build & Run

```bash
cd lang4llm-rs
cargo build                          # full workspace build
cargo build -p lang4llm-cli          # CLI only (faster)
```

## Test Commands

```bash
# All workspace tests
cargo test --workspace

# Specific crate tests
cargo test -p lang4llm-ast           # SourceMap tests
cargo test -p lang4llm-parser        # Span propagation tests
cargo test -p lang4llm-typeck        # Enhanced diagnostics tests
cargo test -p lang4llm-formatter     # Formatter tests (new crate)
cargo test -p lang4llm-cli           # JSON output + fmt subcommand tests

# Single test
cargo test -p lang4llm-cli --test integration_examples test_json_output
```

## Verification Scenarios

### 1. Span Accuracy

```bash
# Create a test file with a known error
echo 'fn main() {
    let x: int = "hello" + 5;
}' > /tmp/test_span.lang4llm

# Check — error should point to line 2, not line 1
./target/debug/lang4llm check /tmp/test_span.lang4llm
```

Expected: error message shows `line 2` with correct column range.

### 2. JSON Output

```bash
# JSON error output
./target/debug/lang4llm check --format json /tmp/test_span.lang4llm

# JSON success output
echo 'fn main() { print("hello"); }' > /tmp/test_ok.lang4llm
./target/debug/lang4llm check --format json /tmp/test_ok.lang4llm
```

Expected: valid JSON on stdout with `success`, `errors`, `result` fields.

### 3. Enhanced Diagnostics

```bash
# "Did you mean?" suggestion
echo 'fn main() {
    let counter: int = 0;
    print(countr);
}' > /tmp/test_suggest.lang4llm
./target/debug/lang4llm check /tmp/test_suggest.lang4llm
```

Expected: error includes "did you mean `counter`?"

### 4. Formatter

```bash
# Format a file to stdout
./target/debug/lang4llm fmt /tmp/test_ok.lang4llm

# Check formatting (dry run)
./target/debug/lang4llm fmt --check ../research/D4-benchmark/reference/simple/

# Format and overwrite
./target/debug/lang4llm fmt --write /tmp/test_ok.lang4llm
```

### 5. Full Benchmark Regression

```bash
# Must still pass all 92 benchmarks after changes
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/simple/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/medium/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/complex/
```

## New Crate Setup

The formatter crate is created at `crates/formatter/`:

```bash
# Verify workspace member added
grep -c "formatter" Cargo.toml   # should be >= 1

# Build just the formatter
cargo build -p lang4llm-formatter
cargo test -p lang4llm-formatter
```

## Key Files to Watch

| File | What changes |
|------|-------------|
| `crates/ast/src/source_map.rs` | NEW — byte-to-line:col conversion |
| `crates/parser/src/grammar.lalrpop` | 138 `Span::dummy()` → `source_map.span(@L, @R)` |
| `crates/parser/src/lib.rs` | `parse()` gains `&SourceMap` parameter |
| `crates/typeck/src/typecheck.rs` | UndefinedSymbol candidates, ArgCount def_span |
| `crates/typeck/src/symtab.rs` | New `all_names_in_scope()` method |
| `crates/cli/src/diagnostic.rs` | All 5 catch-all variants get dedicated handlers; JSON output |
| `crates/cli/src/main.rs` | `--format` global flag, `fmt` subcommand |
| `crates/formatter/` | NEW crate — entire directory |
