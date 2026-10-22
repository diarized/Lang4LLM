# Implementation Plan: Phase 6 — Compiler Enhancement

**Branch**: `003-compiler-enhancement` | **Date**: 2026-02-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/003-compiler-enhancement/spec.md`

---

## Summary

Four targeted fixes to the `lang4llm-rs` compiler: (1) extend string-concat detection to cover
user-defined function return types, eliminating the `to_string()` workaround; (2) add an implicit
`Clone` bound to all generic type parameters so `emit_call_arg`'s `.clone()` calls compile;
(3) teach the typechecker to accept `contains([T], T)` — the transpiler already emits this
correctly; (4) embed spans in `Ident` tokens so typechecker errors cite real line numbers.

All changes are confined to existing crates. No new crates, no surface syntax changes, no new
dependencies.

---

## Technical Context

**Language/Version**: Rust 1.75+ (workspace edition 2021)
**Primary Dependencies**: lalrpop 0.20, lalrpop-util 0.20; thiserror 1.0, miette 7.0; clap 4.5
**Storage**: N/A — in-memory compiler pipeline
**Testing**: `cargo test --workspace`; benchmark eval harness `lang4llm eval --suite`
**Target Platform**: Linux, same toolchain as current CI
**Project Type**: Single Rust workspace (5 crates, modifications only)
**Performance Goals**: No regression in compile throughput
**Constraints**: All 92 benchmark tests pass; `compile_08_closures` moves fail→pass
**Scale/Scope**: ~6 files changed, ~150 lines net new/modified

---

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | spec.md complete before this plan |
| II. Technology-Agnostic Spec | PASS | spec.md contains no Rust/LALRPOP mentions |
| III. Template Consistency | PASS | All mandatory sections present |
| IV. Iterative Refinement | PASS | No [NEEDS CLARIFICATION] markers remain |
| V. User Story Independence | PASS | 4 stories, each independently testable |
| VI. Research-Driven Decisions | PASS | research.md complete; all root causes verified in code |

**Gate result**: PASS

---

## Project Structure

### Documentation (this feature)

```text
specs/003-compiler-enhancement/
├── spec.md
├── plan.md                      # This file
├── research.md                  # Root cause analysis (complete)
├── data-model.md                # Entity changes
├── contracts/
│   └── compiler-interfaces.md  # Internal API contracts
├── quickstart.md                # Dev setup
├── checklists/requirements.md
└── tasks.md                     # Phase 2 output (not yet created)
```

### Source Code

```text
lang4llm-rs/crates/
├── parser/src/
│   ├── lexer.rs             [Gap 4] TokenKind::Ident(String,Span)
│   └── grammar.lalrpop      [Gap 4] Ident rule uses real span
├── typeck/src/
│   ├── builtins.rs          [Gap 3] contains note/comment update
│   └── typecheck.rs         [Gap 3] relax contains([T],T) check
└── transpiler/src/
    ├── context.rs           [Gap 1] fn_return_types field
    ├── lib.rs               [Gap 1] pre-scan pass
    ├── exprs.rs             [Gap 1] contains_string_expr + ctx
    └── items.rs             [Gap 2] implicit Clone bound
```

No new files, no new crates.

---

## Phase 0: Research — Complete

See [research.md](research.md) for full analysis. Summary of decisions:

| Gap | Root Cause | Fix Strategy |
|-----|-----------|--------------|
| 1 — String concat | `contains_string_expr` heuristic ignores user fn return types | Add `fn_return_types: HashMap<String,Type>` to `CodegenContext`; extend detection |
| 2 — Generic bounds | `emit_call_arg` clones unconditionally; no `Clone` bound emitted | Add implicit `T: Clone` to every generic param's where clause in `emit_function` |
| 3 — `contains([T],T)` | typeck registers `contains` as `(string,string)->bool` only | Special-case in typeck call-check: accept `([T], T)` as second overload |
| 4 — Error spans | Grammar uses `Span::dummy()` everywhere; lexer spans discarded | Embed `Span` in `TokenKind::Ident`; propagate through grammar `Ident` rule |

---

## Phase 1: Design & Contracts — Complete

### Gap 1: String Concat (User Story 1, P1)

**Changed interfaces**:
- `CodegenContext::fn_return_types: HashMap<String, Type>` (new field)
- `transpile()` pre-scans `program.items` to build map before codegen
- `contains_string_expr(expr, ctx)` gains context parameter; checks `ctx.fn_return_types` for `Expr::Call` to named user functions

**Key invariant**: Existing string literal and builtin-call detection unchanged. New branch only fires for user-defined `fn` calls.

**Test added**: Unit test `test_string_concat_user_fn` — verifies `"prefix: " + user_fn()` emits `format!`.

---

### Gap 2: Generic Bounds (User Story 2, P1)

**Changed interface**: `emit_function` in `items.rs` — after building `where_clauses`, iterate `f.generics.params` and ensure `Clone` is in each param's bound list.

**Effect on `08_closures.lang4llm`**:
```rust
// Before
fn apply<T, U>(x: T, f: fn(T) -> U) -> U { ... }

// After
fn apply<T, U>(x: T, f: fn(T) -> U) -> U
where
    T: Clone,
    U: Clone,
{ ... }
```

**Edge case**: Functions with no generic params — loop body executes zero times, no change.

---

### Gap 3: `contains([T], T)` (User Story 3, P2)

**Changed interface**: `typecheck.rs` call-check for builtin `contains`:
- If `args[0]` infers to `Type::Array(_)`: accept, return `bool`
- If `args[0]` infers to `Type::Primitive(String)`: existing check (arg1 must be `string`)

No transpiler change — `builtins.rs` already emits `arr.contains(&x)` which is correct for arrays.

**Regression test**: `contains("hello", "ell")` must still typecheck.

---

### Gap 4: Error Spans (User Story 4, P2)

**Changed interfaces**:
1. `TokenKind::Ident(String)` → `TokenKind::Ident(String, Span)` in lexer
2. Grammar extern: `"ident" => TokenKind::Ident(<(String, Span)>)`
3. Grammar `Ident` rule: unpack `(name, span)` → `Ident { name, span }`

**Scope boundary**: Only `Ident` nodes. `Expr`, `Stmt`, `BinaryExpr` retain `Span::dummy()`.
Expression spans are a Phase 7 item.

**Risk**: Grammar change requires `cargo build -p lang4llm-parser` to regenerate the committed
`grammar.rs`. The regenerated file must be committed alongside the `.lalrpop` changes.

---

## Acceptance Criteria (Phase Gate)

| Criterion | Verification |
|-----------|-------------|
| `compile_08_closures` passes | `cargo test compile_08_closures` |
| No string concat regressions | `cargo test -p lang4llm-transpiler` |
| `contains([int], int)` accepted by typeck | New test program |
| Typeck errors cite line N for error on line N | Manual test |
| 92/92 benchmarks | `lang4llm eval --suite` (all tiers) |
| `cargo test --workspace` 0 failures | CI gate |
