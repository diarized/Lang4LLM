# Transpiler API Contract

**Date**: 2026-02-21 | **Phase**: 3 (Transpiler to Rust)

## Crate: `lang4llm-transpiler`

### Public API

```rust
/// Transpile a Lang4LLM program to Rust source code.
///
/// # Arguments
/// * `program` - Parsed and type-checked AST
///
/// # Returns
/// * `Ok(String)` - Valid Rust source code
/// * `Err(Vec<TranspileError>)` - Errors preventing transpilation
pub fn transpile(program: &Program) -> Result<String, Vec<TranspileError>>;

/// Transpile with configuration options.
pub fn transpile_with_options(
    program: &Program,
    options: &TranspileOptions,
) -> Result<String, Vec<TranspileError>>;
```

### Configuration

```rust
pub struct TranspileOptions {
    /// Indentation string (default: "    " / 4 spaces)
    pub indent: String,
    /// Include #![allow(...)] preamble (default: true)
    pub emit_preamble: bool,
    /// Add #[derive(Debug, Clone)] to structs/enums (default: true)
    pub derive_debug: bool,
    /// Use `move` keyword for closures (default: true)
    pub move_closures: bool,
}
```

### Error Type

```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum TranspileError {
    #[error("Unsupported feature at {span}: {message}")]
    UnsupportedFeature { message: String, span: Span },

    #[error("Ambiguous translation at {span}: {message}")]
    AmbiguousTranslation { message: String, span: Span },

    #[error("Unknown builtin function at {span}: {name}")]
    InvalidBuiltin { name: String, span: Span },
}
```

## CLI Extension: `compile` Subcommand

### Usage

```
lang4llm compile <file> [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `--output`, `-o` | Output file path | `<input>.rs` |
| `--check` | Compile output with rustc (verify only) | false |
| `--run` | Compile and execute output | false |
| `--verbose`, `-v` | Show transpilation details | false |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Parse/type-check error |
| 2 | Transpilation error |
| 3 | Rust compilation error (--check/--run) |
| 4 | Runtime error (--run) |

### Pipeline

```
                   ┌──── parse error ──── exit 1
                   │
file.lang4llm ──▶ Parse ──▶ TypeCheck ──▶ Transpile ──▶ file.rs
                              │              │
                              └── type err   └── transpile err
                                  exit 1         exit 2
                                                    │
                                    ┌───────────────┤
                                    ▼               ▼
                              (--check) rustc  (--run) rustc + exec
                                    │               │
                                    └── exit 3      └── exit 3/4
```

## Test Contract

### Integration Test: Transpile + Compile

For each of the 12 example programs:

1. **Parse**: `lang4llm_parser::parse(source)` succeeds
2. **TypeCheck**: `lang4llm_typeck::check_program(&ast)` succeeds
3. **Transpile**: `lang4llm_transpiler::transpile(&ast)` returns `Ok(rust_source)`
4. **Compile**: `rustc --edition 2021 <output.rs>` exits 0
5. **Run** (where applicable): output matches expected

### Transpile Correctness Contract

| Input (Lang4LLM) | Output (Rust) | Invariant |
|-------------------|---------------|-----------|
| `fn main() -> unit` | `fn main()` | Unit return omitted |
| `let x: int = 5` | `let x: i64 = 5` | Type mapped |
| `let mut y: int = 0` | `let mut y: i64 = 0` | Mutability preserved |
| `print("hello")` | `println!("{}", "hello")` | Builtin → macro |
| `"a" + 1` | `format!("{}{}", "a", 1)` | String concat |
| `range(0, n)` | `0..n` | In for-loop context |
| `push(arr, x)` | `arr.push(x)` | Receiver rewrite |
| `and` / `or` / `not` | `&&` / `\|\|` / `!` | Operator mapping |
| `none` | `()` | Unit literal |
| `self: Type` | `self` | Self param |
| `[T]` | `Vec<T>` | Array type |
