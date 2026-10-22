# Implementation Plan: Phase 7 — Developer Tooling

**Branch**: `004-developer-tooling` | **Date**: 2026-03-02 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/004-developer-tooling/spec.md`

---

## Summary

Four improvements to the Lang4LLM developer experience: (1) complete span tracking across all 138 AST nodes currently using `Span::dummy()`, via LALRPOP `@L`/`@R` byte offsets and a new `SourceMap`; (2) machine-readable `--format json` output on all CLI commands with stable error codes; (3) enhanced diagnostic messages — "did you mean?" suggestions, multi-span labels, coverage of all 12 `TypeError` variants; (4) a new `lang4llm-formatter` crate implementing `lang4llm fmt` for canonical source formatting.

Changes span 5 existing crates and 1 new crate. No new external dependencies beyond what the workspace already provides.

---

## Technical Context

**Language/Version**: Rust 1.75+ (workspace edition 2021)
**Primary Dependencies**: lalrpop 0.20, lalrpop-util 0.20; thiserror 1.0; ariadne (diagnostics); clap 4.5; serde 1.0, serde_json 1.0; colored
**Storage**: N/A — in-memory compiler pipeline
**Testing**: `cargo test --workspace`; benchmark eval harness `lang4llm eval --suite`
**Target Platform**: Linux, same toolchain as current CI
**Project Type**: Single Rust workspace (5 crates + 1 new, modifications to all existing)
**Performance Goals**: No regression in compile throughput; SourceMap construction O(n), lookup O(log L)
**Constraints**: All 92 benchmark tests continue to pass; zero `Span::dummy()` in grammar after span work
**Scale/Scope**: ~15 files changed/created, ~1500-2000 lines net new/modified

---

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | spec.md complete before this plan |
| II. Technology-Agnostic Spec | PASS | spec.md references CLI commands only, no Rust/crate names |
| III. Template Consistency | PASS | All mandatory sections present |
| IV. Iterative Refinement | PASS | No [NEEDS CLARIFICATION] markers remain; checklist 16/16 |
| V. User Story Independence | PASS | 4 stories at P1-P4, each independently testable |
| VI. Research-Driven Decisions | PASS | research.md complete; 7 decisions documented with alternatives |

**Gate result**: PASS

---

## Project Structure

### Documentation (this feature)

```text
specs/004-developer-tooling/
├── spec.md                          # Feature specification
├── plan.md                          # This file
├── research.md                      # 7 technical decisions with rationale
├── data-model.md                    # 9 entities (3 modified, 6 new)
├── quickstart.md                    # Dev setup and verification scenarios
├── contracts/
│   └── compiler-interfaces.md       # 6 interface contracts + test contracts
├── checklists/
│   └── requirements.md              # Spec quality checklist (16/16 PASS)
└── tasks.md                         # Phase 2 output (speckit:tasks)
```

### Source Code

```text
lang4llm-rs/crates/
├── ast/src/
│   ├── source_map.rs           [NEW]  Byte→line:col conversion (US1)
│   ├── span.rs                 [MOD]  Span::from_offsets constructor
│   └── lib.rs                  [MOD]  pub mod source_map
├── parser/src/
│   ├── grammar.lalrpop         [MOD]  138× Span::dummy() → source_map.span(@L,@R) (US1)
│   └── lib.rs                  [MOD]  parse() accepts &SourceMap; ParseError span fix
├── typeck/src/
│   ├── typecheck.rs            [MOD]  UndefinedSymbol candidates, ArgCount def_span (US3)
│   └── symtab.rs               [MOD]  all_names_in_scope() method (US3)
├── transpiler/src/
│   └── lib.rs                  [MOD]  Build SourceMap, pass to parse() (US1 ripple)
├── formatter/                  [NEW]  Entire crate (US4)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                    format_source() entry point
│       ├── context.rs                CodegenContext (copied from transpiler)
│       ├── items.rs                  fmt_function, fmt_struct, fmt_enum, fmt_trait, fmt_impl
│       ├── stmts.rs                  fmt_stmt for all statement variants
│       ├── exprs.rs                  fmt_expr with precedence-aware parenthesization
│       ├── types.rs                  fmt_type for all type variants
│       └── patterns.rs              fmt_pattern for all pattern variants
├── cli/src/
│   ├── main.rs                 [MOD]  --format global flag, fmt subcommand (US2, US4)
│   ├── diagnostic.rs           [MOD]  All 12 TypeError handlers, JSON output (US2, US3)
│   └── json_output.rs          [NEW]  DiagnosticOutput/DiagnosticError structs (US2)
└── Cargo.toml                  [MOD]  workspace members += formatter
```

**Structure Decision**: Extends existing single Rust workspace with 1 new crate. No new external dependencies. All modifications follow established crate architecture.

---

## Phase 0: Research — Complete

See [research.md](research.md) for full analysis. Summary:

| # | Decision | Choice | Key Rationale |
|---|----------|--------|---------------|
| 1 | Span strategy | LALRPOP `@L`/`@R` + SourceMap | Least invasive; 138 mechanical replacements vs 50+ token kind changes |
| 2 | JSON schema | Envelope with stable error codes | `serde_json` already available; 14 error codes mapped |
| 3 | "Did you mean?" | Levenshtein at check time, embed in error | No external crate; ~20 lines of code; O(n*m) per candidate trivial at scope sizes |
| 4 | Formatter arch | New `lang4llm-formatter` crate, copied CodegenContext | Transpiler has too much Rust-specific logic to parameterize |
| 5 | ParseError spans | Reuse SourceMap for byte→line:col conversion | Prerequisite for US1; currently parse error spans are all broken |
| 6 | Diagnostic coverage | Add ariadne handlers for all 5 catch-all variants | Shadowing error naturally demonstrates multi-span |
| 7 | Lang4LLM suggestions | Pattern-matched at lexer + typechecker levels | Covers top D1 error patterns |

---

## Phase 1: Design & Contracts — Complete

### US1: Span Tracking (P1)

**Changed interfaces**:
- `SourceMap::from_source(src)` → precomputed line-start table
- `grammar.lalrpop` header: `grammar<'src>(source_map: &'src SourceMap)` — threaded through all rules
- `parse()` in `lib.rs`: `pub fn parse(source: &str, source_map: &SourceMap) -> Result<Program, Vec<ParseError>>`
- All 138 grammar rules: `Span::dummy()` → `source_map.span(start, end)` using `<start:@L>...<end:@R>`
- `ParseError.location` converted from byte offset to real Span via SourceMap

**Key invariant**: After this change, no `Span::dummy()` remains in grammar. Ident spans from Phase 6 lexer continue to work (most precise). Other nodes use grammar-level `@L`/`@R` spans.

**Ripple effects**: All `parse()` call sites must build SourceMap first:
- `crates/cli/src/main.rs` (check, parse, compile, eval commands)
- `crates/transpiler/src/lib.rs` (transpile function)
- `crates/typeck/` (test files that call parse)
- All integration tests

---

### US2: JSON Output (P2)

**New structs** (in `crates/cli/src/json_output.rs`):
- `DiagnosticOutput { success, errors, result }` — derives `Serialize`
- `DiagnosticError { code, severity, message, file, span, suggestion, related }` — derives `Serialize`
- `SpanInfo { start_line, start_col, end_line, end_col }` — derives `Serialize`
- `RelatedInfo { message, span }` — derives `Serialize`
- `CommandResult` enum — derives `Serialize`

**CLI changes**:
- Global `--format` flag (clap enum: `human | json`)
- Each command handler checks format and either renders ariadne (human) or serializes JSON
- Error code mapping: `TypeError` variant → `"E0001"`..`"E0012"`, `ParseError` → `"E1001"`..`"E1002"`

---

### US3: Enhanced Diagnostics (P3)

**Modified types**:
- `TypeError::UndefinedSymbol` gains `candidates: Vec<String>` field
- `TypeError::ArgumentCountMismatch` gains `def_span: Span` and `fn_name: String` fields
- `SymbolTable` gains `all_names_in_scope()` method

**New diagnostic handlers** (5 catch-all variants get dedicated ariadne reports):
1. `Symbol(Shadowing)` — dual-span label: "variable `x` already declared here" + "shadowing attempt here"
2. `Symbol(WrongKind)` — "expected function, found variable"
3. `GenericMismatch` — show message with span
4. `NonExhaustiveMatch` — list missing patterns
5. `TraitBoundViolation` — show type and required trait

**Suggestion improvements**:
- `UndefinedSymbol`: "did you mean `X`?" when edit distance <= 2
- `ArgumentCountMismatch`: show function definition span + expected signature
- `TypeMismatch`: enhanced conversion hints (int→string: "use to_string()", float→int: "explicit cast needed")
- Lexer: specific messages for `&&`→`and`, `||`→`or`, `!`→`not`, `null`→`none`

---

### US4: Formatter (P4)

**New crate**: `lang4llm-formatter` depends on `lang4llm-ast`, `lang4llm-parser`

**Entry point**: `pub fn format_source(source: &str) -> Result<String, Vec<ParseError>>`

**Formatting rules** (from reference implementation analysis):
- 4-space indentation
- Opening brace on same line as declaration/control flow
- `} else {` on same line
- Spaces around binary operators
- Space after commas in lists
- No trailing comma in parameter lists, struct fields, array literals
- Single blank line between top-level items
- LF line endings (normalize CRLF)
- Match arms: `Pat => expr,` with trailing comma

**CLI integration**: `lang4llm fmt [--check] [--write] <PATH>...`

**Limitation**: Comments are discarded by the parser. Formatter is a normalizer, not a round-trip tool.

---

## Acceptance Criteria (Phase Gate)

| Criterion | Verification |
|-----------|-------------|
| No `Span::dummy()` in grammar.lalrpop | `grep -c 'Span::dummy' grammar.lalrpop` = 0 |
| All type errors cite correct line | Span accuracy test suite (new) |
| JSON output valid for all error types | JSON schema test suite (new) |
| "Did you mean?" for undefined symbols | Dedicated test with known typos |
| Multi-span for arg count mismatch | Test showing call site + def site |
| All 12 TypeError variants have dedicated handlers | No `_ =>` catch-all in diagnostic.rs |
| `lang4llm fmt` is idempotent | Format all 92 reference impls twice, diff |
| `lang4llm fmt` preserves semantics | Format all 92, re-run eval, same output |
| 92/92 benchmarks still pass | `lang4llm eval --suite` (all tiers) |
| `cargo test --workspace` 0 failures | Full test suite |
