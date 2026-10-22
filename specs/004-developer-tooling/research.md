# Research: Phase 7 — Developer Tooling

**Branch**: `004-developer-tooling` | **Date**: 2026-03-02

---

## Decision 1: Span Propagation Strategy

### Problem

138 occurrences of `Span::dummy()` in `grammar.lalrpop`. Only `Ident` nodes carry real spans (via Phase 6's `TokenKind::Ident((String, Span))` approach). All expressions, statements, types, and patterns report `0:0-0:0`.

### Options Evaluated

| Approach | Invasiveness | Correctness | Maintenance |
|----------|-------------|-------------|-------------|
| A. Embed spans in all token kinds (Phase 6 pattern) | Very high — modify every `TokenKind` variant and every grammar rule that matches them | Accurate — each token carries its exact position | High — every new token/rule must embed span |
| B. Use LALRPOP `@L`/`@R` byte offsets + source map | Moderate — add `@L`/`@R` to grammar rules, build line:col map once | Accurate — byte offsets are exact | Low — `@L`/`@R` auto-provided by LALRPOP |
| C. Post-parse span inference from Ident nodes | Minimal — use existing Ident spans as anchors, infer parent spans | Inaccurate — many nodes have no Ident child | None — but quality is poor |

### Decision: Approach B — `@L`/`@R` byte offsets with source map

**Rationale**:
- LALRPOP automatically provides `@L` (start byte offset) and `@R` (end byte offset) for any rule via `<start:@L> ... <end:@R>` syntax
- A single `SourceMap` struct (built once from source text) converts byte offsets to line:col
- Grammar rules change from `Span::dummy()` to `Span::from_offsets(&source_map, start, end)` — systematic, mechanical replacement
- The `Ident` approach from Phase 6 still works alongside this — `Ident` spans will come from the lexer (most precise), other spans from `@L`/`@R`
- Far less invasive than modifying all 50+ `TokenKind` variants

**Implementation detail**: The `SourceMap` (a precomputed `Vec<usize>` of line-start byte offsets) must be threaded into the LALRPOP parser. LALRPOP supports passing user state via the grammar header: `grammar<'src>(source_map: &'src SourceMap)`. Each rule can then call `source_map.span(start, end) -> Span`.

**Risk**: LALRPOP `@L`/`@R` incur no runtime cost (they're the current token's position in the byte stream). The `SourceMap` lookup is O(log N) via binary search on line starts — negligible for files under 10K lines.

### Alternatives Rejected

- **Approach A**: Would require modifying 50+ `TokenKind` variants, each grammar extern type mapping, and every pattern match. Massive change for no additional accuracy vs `@L`/`@R`.
- **Approach C**: Too lossy. Expressions like `a + b * c` have no `Ident` child for the `*` node — span would be guessed. Unacceptable for SC-001 (100% accurate spans).

---

## Decision 2: JSON Output Schema

### Problem

Need a stable JSON output format for all compiler commands, usable by LLM pipelines.

### Decision: Rust-standard `serde_json` with explicit schema

**Schema**:
```json
{
  "success": bool,
  "errors": [
    {
      "code": "E0001",
      "severity": "error",
      "message": "human-readable text",
      "file": "path/to/file.lang4llm",
      "span": {
        "start_line": 15,
        "start_col": 8,
        "end_line": 15,
        "end_col": 22
      },
      "suggestion": "optional fix hint",
      "related": [
        {
          "message": "function defined here",
          "span": { ... }
        }
      ]
    }
  ],
  "result": { ... }  // command-specific
}
```

**Rationale**:
- `serde`/`serde_json` already workspace dependencies — zero new deps
- Derive `Serialize` on a `DiagnosticOutput` struct for type-safe serialization
- Error codes (`E0001`-`E9999`) assigned by category — stable across compiler versions
- `related` spans support multi-span diagnostics (FR-007)

### Error Code Registry

| Code | Category | TypeError Variant |
|------|----------|-------------------|
| E0001 | Type mismatch | `TypeMismatch` |
| E0002 | Undefined symbol | `UndefinedSymbol` |
| E0003 | Missing type annotation | `MissingTypeAnnotation` |
| E0004 | Uninitialized variable | `Uninitialized` |
| E0005 | Invalid binary operation | `InvalidBinaryOp` |
| E0006 | Argument count mismatch | `ArgumentCountMismatch` |
| E0007 | Field not found | `FieldNotFound` |
| E0008 | Non-exhaustive match | `NonExhaustiveMatch` |
| E0009 | Generic mismatch | `GenericMismatch` |
| E0010 | Trait bound violation | `TraitBoundViolation` |
| E0011 | Symbol error (shadowing) | `Symbol(Shadowing)` |
| E0012 | Symbol error (wrong kind) | `Symbol(WrongKind)` |
| E1001 | Lexical error | `ParseError::LexError` |
| E1002 | Syntax error | `ParseError::SyntaxError` |

---

## Decision 3: "Did You Mean?" Implementation

### Problem

`TypeError::UndefinedSymbol` currently shows "not found in this scope" with a generic note. Need to suggest similar names from the symbol table.

### Decision: Levenshtein distance computed at error emission site in typechecker

**Implementation**:
1. Add `pub fn all_names_in_scope(&self) -> Vec<&str>` to `SymbolTable`
2. At the point where `UndefinedSymbol` is emitted in `typecheck.rs`, collect candidates with edit distance <= 2
3. Add `candidates: Vec<String>` field to `TypeError::UndefinedSymbol`
4. `diagnostic.rs` renders candidates as "did you mean `counter`?"

**Rationale**:
- Levenshtein is O(nm) per candidate — with typically < 100 symbols in scope and name lengths < 30, this is sub-microsecond per error
- No external crate needed — trivial 20-line Levenshtein implementation
- Candidates computed at check time when scope context is available, not at rendering time

### Alternatives Rejected

- **External crate (strsim)**: Adds dependency for a trivial algorithm. Not justified.
- **Rendering-time computation**: `report_type_error` doesn't have access to the symbol table. Would require passing scope snapshot through the error pipeline. More invasive than embedding candidates in the error.

---

## Decision 4: Formatter Architecture

### Problem

Need an AST-based formatter. The parser discards comments and original whitespace. The transpiler emits Rust, not Lang4LLM.

### Decision: New `lang4llm-formatter` crate with copied `CodegenContext`

**Architecture**:
- New crate: `crates/formatter/` with dependency on `lang4llm-ast` and `lang4llm-parser`
- Copy `CodegenContext` (90 lines — buffer + indent) rather than extracting to shared crate (avoids workspace churn)
- Implement `fmt_program`, `fmt_item`, `fmt_stmt`, `fmt_expr`, `fmt_type`, `fmt_pattern` — mirroring transpiler structure but emitting Lang4LLM syntax
- Parser entry point: `pub fn format_source(source: &str) -> Result<String, Vec<ParseError>>`

**Canonical style rules** (derived from 92 reference implementations):
- 4-space indentation (matches transpiler output)
- Opening brace on same line: `fn foo() {`, `if cond {`, `while cond {`
- `} else {` on same line
- Spaces around binary operators: `a + b`, `x and y`
- Space after commas: `f(a, b)`, `[1, 2, 3]`
- No trailing comma in parameter lists, struct fields, array literals
- Single blank line between top-level items
- No blank lines inside function bodies (consecutive stmts)
- Line endings: LF only (normalize CRLF)
- Match arms: `Pat => expr,` with trailing comma

**Limitation**: Comments are discarded by the lexer. The formatter is a **normalizer**, not a round-trip tool. This is documented as a known limitation.

### Alternatives Rejected

- **Reuse transpiler with toggle**: Transpiler has Rust-specific logic (`.clone()`, `format!`, `as usize`, `#[derive]`) embedded throughout every `emit_*` function. Parameterizing all of these would be more complex than a clean formatter implementation.
- **Move `CodegenContext` to shared crate**: 90 lines of code isn't worth a workspace restructure. Copy is simpler and isolated.
- **CST-based formatter**: Requires rebuilding the parser to preserve comments/whitespace as CST nodes. Massive scope increase for a feature the language doesn't need yet.

---

## Decision 5: ParseError Span Fix

### Problem

`ParseError` spans are constructed from LALRPOP byte offsets stored as `Span::new(byte_offset, 0, byte_offset, 0)` — `start_line` is the byte offset, `start_col` is 0. The `diagnostic.rs` conversion function interprets `col=0` as a sentinel and returns byte 0. Parse error locations are always wrong.

### Decision: Use the same SourceMap for parse error span conversion

- Build `SourceMap` before calling the LALRPOP parser
- After parse failure, convert byte offsets in `ParseError.location` to real line:col via `source_map.resolve(byte_offset) -> (line, col)`
- Store corrected `Span` in the error before returning to CLI

This is a prerequisite for US1 and US2 (accurate spans in all error types).

---

## Decision 6: Diagnostic Coverage for Remaining TypeError Variants

### Problem

5 TypeError variants fall through to the catch-all `_ => format!("Error: {}", error)` in `diagnostic.rs`:
- `Symbol(Shadowing)` — has two spans (original + shadowing)
- `Symbol(WrongKind)` — has span
- `Exhaustiveness` — opaque error type
- `GenericMismatch` — has span + message
- `NonExhaustiveMatch` — has span + missing patterns
- `TraitBoundViolation` — has span + type/trait names

### Decision: Add dedicated ariadne handlers for all 5

Each gets a proper `Report::build()` with source labels and context-aware notes. The `Symbol(Shadowing)` variant is notable because it carries TWO spans — this naturally demonstrates multi-span diagnostics (FR-007).

---

## Decision 7: Lang4LLM-Specific Diagnostic Suggestions

### Problem

LLMs frequently generate code using other languages' conventions (e.g., `&&` instead of `and`, `!` instead of `not`, `null` instead of `none`).

### Decision: Pattern-matched suggestions in lexer and typechecker

**Lexer-level** (detected during tokenization):
- `&&` → "use `and` instead of `&&`"
- `||` → "use `or` instead of `||`"
- `!` (before expression) → "use `not` instead of `!`"
- `null`/`NULL`/`nil` → "use `none` instead"
- `var`/`val`/`const` → "use `let` or `let mut`"

**Typechecker-level** (detected during type checking):
- Int assigned to String variable → "use `to_string(value)` to convert"
- Float assigned to Int variable → "integer truncation — use explicit cast"
- String `+` non-string → "use `to_string(expr)` to concatenate"
- Calling with wrong arg count → show function definition span and expected signature

These cover the most common D1 Error Taxonomy patterns (EP-SYN-001 through EP-SYN-004, EP-SEM-001 through EP-SEM-003).
