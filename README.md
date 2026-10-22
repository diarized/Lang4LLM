# Lang4LLM

Research and specification for a programming language designed to be easy for LLMs to generate correctly, plus a Rust implementation (`lang4llm-rs/`) that parses, type-checks, formats and transpiles Lang4LLM to Rust.

## Design goals

Derived from the Phase 0 research (error taxonomy, design rules, metrics, benchmark, feature validation):

- Context-free, unambiguous, LL(1)-compatible grammar (the research reports the largest effect on syntax errors).
- Iterator-first design to avoid boundary/off-by-one logic errors.
- Bounds-checked indexing.
- Value semantics (no mutation by reference); variable shadowing within a function is rejected.
- Machine-readable diagnostics with stable error codes (`lang4llm check --format json`, see `lang4llm-rs/ERROR_CODES.md`).

The 20 design rules (DR-*) and the validation of 20 candidate features are in `research/`.

## Repository layout

| Path | Content |
|------|---------|
| `research/` | Phase 0: D1 error taxonomy, D2 design guide, D3 metrics, D4 benchmark (100 specs + reference programs), D5 feature validation, literature notes |
| `language-spec/` | Phase 1: EBNF grammar, type system, syntax, semantics, memory model, stdlib signatures, 15 example programs |
| `specs/` | Per-phase spec, plan and task documents (001 research, 002 parser/type checker, 003 compiler enhancement, 004 developer tooling) |
| `lang4llm-rs/` | Rust workspace: `ast`, `parser` (LALRPOP), `typeck`, `transpiler`, `formatter`, `cli` |
| `VALIDATION_REPORT.md`, `validate_schemas.py` | Schema validation of Phase 0 deliverables |

## Build and test

Requires a Rust toolchain (1.75+); the transpiler tests also call `rustc`.

```bash
cd lang4llm-rs
cargo build
cargo test --workspace
```

CLI usage:

```bash
./target/debug/lang4llm check   ../language-spec/examples/01_hello_world.lang4llm
./target/debug/lang4llm compile ../language-spec/examples/01_hello_world.lang4llm   # emits .rs
./target/debug/lang4llm fmt     <file.lang4llm>
./target/debug/lang4llm eval --suite ../research/D4-benchmark/reference/simple/
```

## Status

| Phase | Scope | State |
|-------|-------|-------|
| 0 | Research | Done |
| 1 | Language specification | Done |
| 2 | Parser and type checker | Done |
| 3 | Transpiler to Rust | Done |
| 4 | Standard library (29 builtins) | Done |
| 5 | Benchmarks | Done |
| 6 | Compiler enhancement | Done |
| 7 | Developer tooling (JSON diagnostics, formatter) | Done |
| 8+ | Extended standard library and later | Not started |

Benchmark eval: 92 of 100 reference programs are compatible with the current compiler and all 92 pass (36 simple, 38 medium, 18 complex); the rest are skipped (file I/O specs).

Known issue: the test `eval_hello_world_no_check` can be flaky when the CLI binary is relinked during the run; it passes in isolation.

## License

MIT, see `LICENSE`.
