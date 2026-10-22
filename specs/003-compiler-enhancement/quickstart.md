# Quickstart: Phase 6 — Compiler Enhancement Development

**Branch**: `003-compiler-enhancement`

## Prerequisites

- Rust toolchain (stable, 1.75+)
- `rustc` in PATH (for compile tests)
- Working directory: `lang4llm-rs/`

## Build

```bash
cd lang4llm-rs
cargo build -p lang4llm-cli -q
```

## Run Full Test Suite

```bash
# Unit + integration tests
cargo test --workspace

# Benchmark eval (92 tests)
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/simple/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/medium/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/complex/
```

## Verify Specific Fixes

### Gap 2 — Generic bounds (`compile_08_closures`)
```bash
cargo test -p lang4llm-transpiler --test transpile_compile compile_08_closures
```

### Gap 1 — String concat without workaround
Write a test program:
```lang4llm
fn greet(name: string) -> string {
    return "Hello, " + name;
}

fn main() -> unit {
    let msg: string = "Result: " + greet("world");
    print(msg);
    return none;
}
```
```bash
./target/debug/lang4llm compile /tmp/test_concat.lang4llm
rustc --edition 2021 /tmp/test_concat.rs -o /tmp/test_concat && /tmp/test_concat
# Expected: Result: Hello, world
```

### Gap 3 — `contains` on arrays
```lang4llm
fn main() -> unit {
    let nums: [int] = [1, 2, 3];
    let found: bool = contains(nums, 2);
    print(to_string(found));
    return none;
}
```
```bash
./target/debug/lang4llm compile /tmp/test_contains.lang4llm
rustc --edition 2021 /tmp/test_contains.rs -o /tmp/test_contains && /tmp/test_contains
# Expected: true
```

### Gap 4 — Error spans
```lang4llm
fn main() -> unit {
    let x: int = 42;
    let y: string = x;
    return none;
}
```
```bash
./target/debug/lang4llm check /tmp/test_spans.lang4llm
# Expected: error at line 3 (not line 0:0 or 1:1)
```

## Development Loop

1. Edit source files in `lang4llm-rs/crates/{parser,typeck,transpiler}/src/`
2. `cargo build -p lang4llm-cli -q` (fast incremental build)
3. `cargo test --workspace` (verify no regressions)
4. `./target/debug/lang4llm eval --suite ...` (verify benchmark pass rate)

## Key Files Per Gap

| Gap | Files to Edit |
|-----|--------------|
| 1 — String concat | `transpiler/src/context.rs`, `transpiler/src/exprs.rs`, `transpiler/src/lib.rs` |
| 2 — Generic bounds | `transpiler/src/items.rs` |
| 3 — `contains([T], T)` | `typeck/src/builtins.rs`, `typeck/src/typecheck.rs` |
| 4 — Error spans | `parser/src/lexer.rs`, `parser/src/grammar.lalrpop` |

## After Grammar Changes

Grammar changes require regenerating the parser. LALRPOP runs automatically in `build.rs`:
```bash
cargo build -p lang4llm-parser
```

## Acceptance Gate

All of the following must pass before closing Phase 6:

```bash
# 1. No regressions
cargo test --workspace 2>&1 | grep -E "FAILED|passed|failed"

# 2. compile_08_closures now passes
cargo test -p lang4llm-transpiler --test transpile_compile compile_08_closures

# 3. Benchmark eval 92/92
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/simple/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/medium/
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/complex/
```
