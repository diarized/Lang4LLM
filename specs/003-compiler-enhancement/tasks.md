# Tasks: Phase 6 — Compiler Enhancement

**Input**: Design documents from `/specs/003-compiler-enhancement/`
**Prerequisites**: plan.md ✓ spec.md ✓ research.md ✓ data-model.md ✓ contracts/ ✓

**Organization**: Tasks grouped by user story for independent implementation and testing.
All four user stories touch **different files** — they can be implemented fully in parallel.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 1: Setup (Baseline Verification)

**Purpose**: Confirm the exact starting state before any changes

- [ ] T001 Verify baseline: `cargo test --workspace` passes all tests except `compile_08_closures` in `lang4llm-rs/`
- [ ] T002 [P] Verify baseline eval: `lang4llm eval --suite` scores 92/92 across all three tiers in `lang4llm-rs/`

**Checkpoint**: Baseline confirmed — 92 benchmarks pass, compile_08_closures fails

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: No cross-cutting foundation needed — all 4 gaps touch separate files and can proceed independently. This phase is intentionally minimal.

**⚠️ CRITICAL**: All user story phases can start in parallel after Phase 1 completes.

- [ ] T003 Read and understand `lang4llm-rs/crates/transpiler/src/context.rs` + `exprs.rs` + `items.rs` to confirm understanding of CodegenContext structure and `contains_string_expr` / `emit_function` locations

**Checkpoint**: Architecture understood — US1–US4 can now proceed in parallel

---

## Phase 3: User Story 1 — String Concatenation Without Workarounds (Priority: P1)

**Goal**: `"prefix: " + user_fn()` where `user_fn() -> string` emits valid `format!()` without
requiring the user to write `to_string(user_fn())`.

**Independent Test**:
```bash
# Write /tmp/test_str_concat.lang4llm:
#   fn greet(name: string) -> string { return "Hello, " + name; }
#   fn main() -> unit { print("msg: " + greet("world")); return none; }
# Then:
cd lang4llm-rs && ./target/debug/lang4llm compile /tmp/test_str_concat.lang4llm
rustc --edition 2021 /tmp/test_str_concat.rs -o /tmp/test_str_concat && /tmp/test_str_concat
# Expected output: msg: Hello, world
```

### Implementation for User Story 1

- [ ] T004 [US1] Add `fn_return_types: HashMap<String, Type>` field to `CodegenContext` struct in `lang4llm-rs/crates/transpiler/src/context.rs`; update `new()` and add `with_fn_types()` constructor
- [ ] T005 [US1] In `lang4llm-rs/crates/transpiler/src/lib.rs`'s `transpile()`: pre-scan `program.items` to build `HashMap<String, Type>` (function name → return type); pass it to `CodegenContext::with_fn_types()`
- [ ] T006 [US1] In `lang4llm-rs/crates/transpiler/src/exprs.rs`: change `contains_string_expr(expr)` signature to `contains_string_expr(expr, ctx)` and `is_string_concat_chain` likewise; add match arm for `Expr::Call` to user-defined functions that checks `ctx.fn_return_types`
- [ ] T007 [US1] Update all internal callers of `contains_string_expr` and `is_string_concat_chain` in `lang4llm-rs/crates/transpiler/src/exprs.rs` to pass `ctx`
- [ ] T008 [US1] Add unit test `test_string_concat_user_fn` in `lang4llm-rs/crates/transpiler/src/exprs.rs` that verifies `"prefix: " + user_fn()` where `user_fn: string` emits `format!`
- [ ] T009 [US1] Run `cargo test -p lang4llm-transpiler` and verify existing string concat tests still pass; run `lang4llm eval --suite` to confirm 92/92 unchanged in `lang4llm-rs/`

**Checkpoint**: `"prefix: " + user_fn()` compiles and runs correctly; all 92 benchmarks still pass

---

## Phase 4: User Story 2 — Generic Bounds Propagated to Output (Priority: P1)

**Goal**: `fn f<T, U>(x: T, ...)` emits `where T: Clone, U: Clone` so `emit_call_arg`'s `.clone()` calls satisfy Rust's type checker. `compile_08_closures` moves from fail to pass.

**Independent Test**:
```bash
cd lang4llm-rs && cargo test -p lang4llm-transpiler --test transpile_compile compile_08_closures
# Expected: test compile_08_closures ... ok  (was FAILED)
```

### Implementation for User Story 2

- [ ] T010 [US2] In `lang4llm-rs/crates/transpiler/src/items.rs`'s `emit_function`: after building `where_clauses` from `f.generics` and `ParamName::GenericBound`, iterate over `f.generics.params` and add `"Clone"` to each param's bound list if not already present
- [ ] T011 [US2] Run `cargo test -p lang4llm-transpiler --test transpile_compile compile_08_closures` and confirm it now passes
- [ ] T012 [US2] Run `cargo test --workspace` to confirm zero regressions; run `lang4llm eval --suite` to confirm 92/92 still pass in `lang4llm-rs/`

**Checkpoint**: `compile_08_closures` passes; 93 total tests pass (92 benchmarks + this compile test)

---

## Phase 5: User Story 3 — `contains` Works on Arrays (Priority: P2)

**Goal**: `contains([1, 2, 3], 2)` type-checks and returns `true`. The transpiler already emits
correct Rust (`.contains(&x)`) — only the typechecker needs updating.

**Independent Test**:
```bash
# Write /tmp/test_contains.lang4llm:
#   fn main() -> unit { let found: bool = contains([1, 2, 3], 2); print(to_string(found)); return none; }
cd lang4llm-rs && ./target/debug/lang4llm compile /tmp/test_contains.lang4llm
rustc --edition 2021 /tmp/test_contains.rs -o /tmp/test_contains && /tmp/test_contains
# Expected: true
```

### Implementation for User Story 3

- [ ] T013 [US3] In `lang4llm-rs/crates/typeck/src/typecheck.rs`: locate the builtin call-checking logic for `contains`; add a branch: if `args[0]` infers to `Type::Array(_)`, accept `args[1]` as any type and return `bool`; existing `(string, string)` path unchanged
- [ ] T014 [US3] Run `cargo test -p lang4llm-typeck` and confirm `contains(string, string)` cases still pass; add a manual test program to verify `contains([int], int)` accepts
- [ ] T015 [US3] Run `cargo test --workspace` + `lang4llm eval --suite` (all tiers) to confirm no regressions in `lang4llm-rs/`

**Checkpoint**: `contains(arr, val)` type-checks for array types; string `contains` unchanged; 92/92 benchmarks pass

---

## Phase 6: User Story 4 — Accurate Error Locations in Diagnostics (Priority: P2)

**Goal**: Typechecker errors report the real line number of the offending code, not `0:0` or `1:1`.
Achieved by embedding `Span` in `TokenKind::Ident` and propagating through the grammar.

**Independent Test**:
```bash
# Write /tmp/test_spans.lang4llm (10+ lines with type error on line 10):
#   fn main() -> unit {
#     let a: int = 1;
#     let b: int = 2;
#     let c: int = 3;
#     let d: int = 4;
#     let e: int = 5;
#     let f: int = 6;
#     let g: int = 7;
#     let h: int = 8;
#     let x: string = h;   <- error on line 10
#     return none;
#   }
cd lang4llm-rs && ./target/debug/lang4llm check /tmp/test_spans.lang4llm
# Expected: error message cites line 10 (not 0:0 or 1:1)
```

### Implementation for User Story 4

- [ ] T016 [US4] In `lang4llm-rs/crates/parser/src/lexer.rs`: change `TokenKind::Ident(String)` variant to `TokenKind::Ident(String, Span)`; update `lex_ident_or_keyword()` to embed the computed span as `TokenKind::Ident(name, span)` in addition to the outer `Token.span`
- [ ] T017 [US4] Update all match arms on `TokenKind::Ident` in `lang4llm-rs/crates/parser/src/lexer.rs` (tests and any other uses) to handle the new two-field variant
- [ ] T018 [US4] In `lang4llm-rs/crates/parser/src/grammar.lalrpop`: update `extern` block `"ident"` entry from `TokenKind::Ident(<String>)` to `TokenKind::Ident(<(String, Span)>)`; update `Ident` grammar rule to unpack `(name, span)` and construct `Ident { name: tok.0, span: tok.1 }`
- [ ] T019 [US4] Run `cargo build -p lang4llm-parser` to regenerate the LALRPOP-generated `grammar.rs`; verify build succeeds
- [ ] T020 [US4] Run `cargo test --workspace` to confirm no parse regressions; verify error messages for a deliberate type error now cite line > 1
- [ ] T021 [US4] Run `lang4llm eval --suite` (all tiers) to confirm 92/92 benchmarks still pass in `lang4llm-rs/`

**Checkpoint**: All typechecker errors for user-typed identifiers cite correct line numbers; no regressions

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, doc updates, and commit hygiene

- [ ] T022 Run full acceptance gate: `cargo test --workspace` — 0 failures in `lang4llm-rs/`
- [ ] T023 [P] Run full benchmark eval across all tiers and confirm 92/92 in `lang4llm-rs/`
- [ ] T024 [P] Confirm `compile_08_closures` passes: `cargo test -p lang4llm-transpiler --test transpile_compile compile_08_closures`
- [ ] T025 Update `lang4llm-rs/CLAUDE.md`: remove "Known pre-existing failure: compile_08_closures" note; add notes for any new patterns (contains array, generic Clone bound)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1
- **User Stories (Phase 3–6)**: All depend on Phase 2 — but they are **fully independent of each other** (different files throughout)
- **Polish (Phase 7)**: Depends on all user story phases complete

### User Story Dependencies

All four user stories are in separate files with **zero cross-dependencies**:

| Story | Crate | Files touched | Depends on other US? |
|-------|-------|---------------|----------------------|
| US1 — String concat | transpiler | `context.rs`, `lib.rs`, `exprs.rs` | No |
| US2 — Generic bounds | transpiler | `items.rs` | No |
| US3 — `contains([T],T)` | typeck | `typecheck.rs` | No |
| US4 — Error spans | parser | `lexer.rs`, `grammar.lalrpop` | No |

### Parallel Opportunities

- T001 ‖ T002 (Phase 1 baseline checks)
- T004–T008 ‖ T010–T011 ‖ T013–T014 ‖ T016–T020 (all 4 user stories in full parallel after Phase 2)
- T022 ‖ T023 ‖ T024 ‖ T026 (Phase 7 final checks)

---

## Parallel Example: All Stories in Parallel

```bash
# After Phase 2 completes, four workstreams can proceed simultaneously:

# Workstream A — US1 (String concat)
# Files: context.rs, lib.rs, exprs.rs (transpiler crate)
Tasks: T004 → T005 → T006 → T007 → T008 → T009

# Workstream B — US2 (Generic bounds)
# Files: items.rs (transpiler crate, different file than A)
Tasks: T010 → T011 → T012

# Workstream C — US3 (contains array)
# Files: typecheck.rs (typeck crate)
Tasks: T013 → T014 → T015

# Workstream D — US4 (Error spans)
# Files: lexer.rs, grammar.lalrpop (parser crate)
Tasks: T016 → T017 → T018 → T019 → T020 → T021
```

---

## Implementation Strategy

### MVP First (User Story 2 — compile_08_closures fix)

The highest-impact, lowest-effort fix is US2 (~10 lines, 1 file):

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 4: User Story 2 (T010–T012)
4. **STOP and VALIDATE**: `cargo test compile_08_closures` passes
5. Known failing test eliminated — delivers value immediately

### Incremental Delivery

1. Phase 1+2: Baseline confirmed
2. Phase 4 (US2): Generic bounds fix → `compile_08_closures` passes
3. Phase 3 (US1): String concat fix → workarounds eliminated
4. Phase 5 (US3): `contains` array → new builtin capability
5. Phase 6 (US4): Error spans → diagnostic accuracy
6. Phase 7: Polish + full validation

### Single Developer Sequential (Recommended Order)

Given effort estimates:

| Story | Effort | Order |
|-------|--------|-------|
| US2 — Generic bounds | ~10 lines, 1 file | 1st (highest impact, trivial effort) |
| US3 — `contains([T],T)` | ~20 lines, 1 file | 2nd (single file typeck change) |
| US1 — String concat | ~50 lines, 3 files | 3rd (moderate, 3 transpiler files) |
| US4 — Error spans | ~30 lines, 2 files + parser regen | 4th (grammar regen adds risk) |

---

## Notes

- All [P] tasks = different files, zero conflict risk
- Grammar change (T018) requires `cargo build -p lang4llm-parser` to regenerate `grammar.rs`
- US4 (spans) is the highest-risk task due to parser regeneration; do last
- No new crates, no new dependencies, no surface syntax changes
- Commit after each user story's checkpoint for clean rollback points
