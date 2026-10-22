# Lang4LLM Rust Implementation

Parser, type checker, and transpiler for the Lang4LLM programming language — a language designed for LLM code generation. Includes a 29-function standard library implemented as transpiler-level builtins.

## Project Structure

```
lang4llm-rs/
├── crates/
│   ├── ast/          # Abstract Syntax Tree definitions
│   ├── parser/       # LALRPOP-based parser + lexer
│   ├── typeck/       # Type checker (symbol table, exhaustiveness, builtins)
│   ├── transpiler/   # Lang4LLM → Rust source transpiler (Phase 3)
│   └── cli/          # Command-line tool (`check` / `parse` / `compile`)
└── tests/            # Integration test inputs (.lang4llm files)
```

## Building

```bash
cargo build
```

## Usage

```bash
# Parse and type-check a file
cargo run --bin lang4llm check ../language-spec/examples/01_hello_world.lang4llm

# Parse only (show AST)
cargo run --bin lang4llm parse ../language-spec/examples/01_hello_world.lang4llm

# Transpile to Rust (emit .rs file)
cargo run --bin lang4llm compile ../language-spec/examples/01_hello_world.lang4llm

# Transpile + verify Rust compiles with rustc
cargo run --bin lang4llm compile --check ../language-spec/examples/01_hello_world.lang4llm

# Transpile + compile + run
cargo run --bin lang4llm compile --run ../language-spec/examples/01_hello_world.lang4llm

# Verbose diagnostics
cargo run --bin lang4llm check --verbose <file>
cargo run --bin lang4llm compile --verbose <file>
```

## Testing

```bash
# All tests (unit + integration)
cargo test --workspace

# Integration examples only (12/12 must pass)
cargo test -p lang4llm-cli --test integration_examples

# Error detection tests only (20/20 must pass)
cargo test -p lang4llm-cli --test integration_errors

# Transpiler: all 15 examples transpile
cargo test -p lang4llm-transpiler --test transpile_examples

# Transpiler: all 15 examples compile with rustc
cargo test -p lang4llm-transpiler --test transpile_compile

# Transpiler: runtime output verification
cargo test -p lang4llm-transpiler --test transpile_run
```

## API Documentation

```bash
cargo doc --open
```

## Example Programs

15 example programs are in `../language-spec/examples/`:

| File | Demonstrates |
|------|-------------|
| `01_hello_world.lang4llm` | Functions, print, return |
| `02_fibonacci.lang4llm` | Recursion, conditionals |
| `03_pattern_matching.lang4llm` | Enums, match, Option |
| `04_structs.lang4llm` | Structs, impl blocks, self |
| `05_generics.lang4llm` | Generic functions and types |
| `06_result_error_handling.lang4llm` | Result type, error propagation |
| `07_arrays.lang4llm` | Arrays, for loops, builtins |
| `08_closures.lang4llm` | Closures, higher-order functions |
| `09_traits.lang4llm` | Traits, impl for trait, bounds |
| `10_complex_matching.lang4llm` | Nested patterns, struct patterns |
| `11_loop_control.lang4llm` | while, for, continue |
| `12_ranges.lang4llm` | Range expressions, array slicing |
| `13_stdlib_math.lang4llm` | abs, min, max, pow, floor, ceil, round, sqrt |
| `14_stdlib_strings.lang4llm` | trim, contains, split, join, replace, starts_with, ends_with, to_upper, to_lower, chars |
| `15_stdlib_collections.lang4llm` | sort, reverse, map, filter, is_empty, concat, first, pop |

## Standard Library (Phase 4)

29 builtin functions implemented as transpiler-level rewrites (zero runtime overhead):

| Category | Functions |
|----------|-----------|
| I/O | `print` |
| Iterator | `range`, `enumerate` |
| Math | `sqrt`, `abs`, `min`, `max`, `pow`, `floor`, `ceil`, `round` |
| Conversion | `as_float`, `as_int`, `to_string` |
| String | `contains`, `split`, `trim`, `replace`, `starts_with`, `ends_with`, `to_upper`, `to_lower`, `chars`, `join` |
| Array | `len`, `push`, `pop`, `sort`, `reverse`, `map`, `filter`, `is_empty`, `concat`, `first` |

## Development Status

**Phase**: 4 (Standard Library Foundation) — Complete
**Tests**: 12/12 parse+typecheck ✓ | 15/15 transpile ✓ | 15/15 rustc compile ✓ | 8/8 runtime ✓ | 20/20 error tests ✓

## Architecture

Lang4LLM compilation pipeline:

```
Source (.lang4llm)
    │
    ▼
Lexer (crates/parser/src/lexer.rs)
    │ Token stream
    ▼
Parser (crates/parser/src/grammar.lalrpop)
    │ AST (crates/ast/)
    ▼
Type Checker (crates/typeck/)
    │ Typed AST or TypeError list
    ▼
CLI Output (crates/cli/)
```

## License

MIT OR Apache-2.0
