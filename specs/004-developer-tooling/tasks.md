# Tasks: Phase 7 — Developer Tooling

**Input**: Design documents from `/specs/004-developer-tooling/`
**Prerequisites**: plan.md ✓ spec.md ✓ research.md ✓ data-model.md ✓ contracts/ ✓

**Organization**: Tasks grouped by user story. US1 (spans) is foundational — must complete before US2/US3. US4 (formatter) is independent after Foundational phase. US2 and US3 share `diagnostic.rs` — must be sequential.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 1: Setup (Baseline Verification)

**Purpose**: Confirm the starting state before any changes

- [X] T001 Verify baseline: `cargo test --workspace` passes in `lang4llm-rs/`
- [X] T002 [P] Verify baseline eval: `lang4llm eval --suite` scores 92/92 across all three tiers

**Checkpoint**: Baseline confirmed — 92 benchmarks pass, all existing tests green

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: SourceMap infrastructure and parse() signature change — BLOCKS all user stories

- [X] T003 Create `SourceMap` struct with `from_source()`, `resolve()`, and `span()` methods in `lang4llm-rs/crates/ast/src/source_map.rs`
- [X] T004 Add `pub mod source_map;` export and re-export in `lang4llm-rs/crates/ast/src/lib.rs`
- [X] T005 Add unit tests for SourceMap: empty string, single line, multi-line, UTF-8 edge cases, boundary offsets in `lang4llm-rs/crates/ast/src/source_map.rs`
- [X] T006 Update `grammar.lalrpop` header to accept `source_map: &SourceMap` parameter in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T007 Update `parse()` function signature to accept `&SourceMap` and thread it to the LALRPOP parser in `lang4llm-rs/crates/parser/src/lib.rs`
- [X] T008 Fix ParseError span construction: convert byte offsets to real line:col via SourceMap in `lang4llm-rs/crates/parser/src/lib.rs`
- [X] T009 Update all `parse()` call sites to build SourceMap and pass it: CLI commands in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T010 [P] Update `parse()` call in transpiler: build SourceMap in `lang4llm-rs/crates/transpiler/src/lib.rs`
- [X] T011 [P] Update `parse()` calls in all test files across parser, typeck, transpiler, and CLI integration tests
- [X] T012 Verify `cargo test --workspace` still passes with SourceMap threaded but all grammar rules still using `Span::dummy()`

**Checkpoint**: SourceMap compiles and passes tests. parse() accepts SourceMap everywhere. No functional change yet — spans still dummy.

---

## Phase 3: User Story 1 — Accurate Error Location Reporting (Priority: P1)

**Goal**: Replace all 138 `Span::dummy()` in grammar.lalrpop with `source_map.span(@L, @R)` so every AST node carries real source positions.

**Independent Test**: Compile programs with known errors at specific locations; verify reported line:col matches actual error site.

### Implementation for User Story 1

- [X] T013 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for top-level item rules (~30 occurrences: Program, ModuleDecl, ImportStmt, FunctionDef, Param, StructDef, FieldDef, EnumDef, Variant, TraitDef, FunctionSig, ImplBlock, GenericParams, GenericArgs) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T014 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for type rules (~12 occurrences: Primitive×5, Array, Tuple, Generic×2, FunctionType, Inferred) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T015 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for statement rules (~13 occurrences: LetStmt×2, AssignStmt, ExprStmt×2, ReturnStmt, BreakStmt, ContinueStmt, WhileStmt, ForStmt, LoopStmt, Empty, single-arm if) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T016 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for binary expression rules (~24 occurrences: Or, And, Comp×6, Add×2, Mul×3 in both Expr and ExprNS chains) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T017 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for unary and postfix expression rules (~20 occurrences: UnaryExpr×4, CallExpr, ArrayAccess, FieldAccess, IndexExpr ranges in both chains) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T018 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for primary expression rules (~25 occurrences: Closure×2, EnumVariant×2, Variable×2, StructLiteral, Literals×6×2, ArrayLit, TupleLit, FieldInit, MatchExpr, IfExpr×2, BlockExpr) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T019 [US1] Replace `Span::dummy()` with `source_map.span(start, end)` for pattern rules (~14 occurrences: Wildcard, Ident, Tuple, Struct, Enum×3, LitPat×6, FieldPat) in `lang4llm-rs/crates/parser/src/grammar.lalrpop`
- [X] T020 [US1] Verify zero `Span::dummy()` occurrences remain in grammar: `grep -c 'Span::dummy' grammar.lalrpop` must equal 0
- [X] T021 [US1] Add span accuracy integration tests: programs with type errors at known line:col, assert reported span matches in `lang4llm-rs/crates/cli/tests/`
- [X] T022 [US1] Verify `cargo test --workspace` passes and 92/92 benchmarks still pass

**Checkpoint**: All AST nodes carry real spans. Error messages cite correct line:col. SC-001 verified.

---

## Phase 4: User Story 2 — Machine-Readable Error Output (Priority: P2)

**Goal**: `--format json` flag on all CLI commands producing structured error output with stable error codes.

**Independent Test**: Run compiler with `--format json`, validate JSON against schema.

### Implementation for User Story 2

- [X] T023 [US2] Create JSON output structs (DiagnosticOutput, DiagnosticError, SpanInfo, RelatedInfo, CommandResult) with Serialize derives in `lang4llm-rs/crates/cli/src/json_output.rs`
- [X] T024 [US2] Implement error code mapping function: TypeError variant → E0001-E0012, ParseError → E1001-E1002 in `lang4llm-rs/crates/cli/src/json_output.rs`
- [X] T025 [US2] Implement conversion functions: TypeError → DiagnosticError, ParseError → DiagnosticError, Span → SpanInfo in `lang4llm-rs/crates/cli/src/json_output.rs`
- [X] T026 [US2] Add `--format` global flag (clap enum: human/json) to CLI struct in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T027 [US2] Wire JSON output path into `check` command: if format=json, serialize DiagnosticOutput instead of ariadne rendering in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T028 [US2] Wire JSON output path into `parse`, `compile`, and `eval` commands in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T029 [US2] Handle edge cases: empty source file, malformed input → graceful JSON error (not panic) in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T030 [US2] Add integration tests: JSON output for type errors, parse errors, success case, empty file in `lang4llm-rs/crates/cli/tests/`
- [X] T031 [US2] Verify `cargo test --workspace` passes

**Checkpoint**: `lang4llm check --format json` produces valid JSON with error codes and spans. SC-002 verified.

---

## Phase 5: User Story 3 — Enhanced Diagnostic Messages (Priority: P3)

**Goal**: "Did you mean?" suggestions, multi-span labels, coverage of all TypeError variants.

**Independent Test**: Compile programs with each error pattern, verify suggestions and multi-span labels appear.

### Implementation for User Story 3

- [X] T032 [US3] Implement Levenshtein distance function (private helper, ~20 lines) in `lang4llm-rs/crates/typeck/src/typecheck.rs`
- [X] T033 [US3] Add `all_names_in_scope() -> Vec<&str>` method to SymbolTable in `lang4llm-rs/crates/typeck/src/symtab.rs`
- [X] T034 [US3] Add `candidates: Vec<String>` field to `TypeError::UndefinedSymbol` variant and update all construction sites in `lang4llm-rs/crates/typeck/src/typecheck.rs`
- [X] T035 [US3] Compute candidates at UndefinedSymbol emission: call `all_names_in_scope()`, filter by edit distance <= 2, cap at 5 results in `lang4llm-rs/crates/typeck/src/typecheck.rs`
- [X] T036 [US3] Add `def_span: Span` and `fn_name: String` fields to `TypeError::ArgumentCountMismatch` and populate from symbol table at emission site in `lang4llm-rs/crates/typeck/src/typecheck.rs`
- [X] T037 [US3] Update JSON conversion in `json_output.rs` to handle new UndefinedSymbol.candidates and ArgumentCountMismatch.def_span fields in `lang4llm-rs/crates/cli/src/json_output.rs`
- [X] T038 [US3] Add dedicated ariadne handler for `TypeError::Symbol(Shadowing)` with dual-span labels (original + shadowing site) in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T039 [US3] Add dedicated ariadne handler for `TypeError::Symbol(WrongKind)` in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T040 [US3] Add dedicated ariadne handler for `TypeError::GenericMismatch` in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T041 [US3] Add dedicated ariadne handler for `TypeError::NonExhaustiveMatch` listing missing patterns in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T042 [US3] Add dedicated ariadne handler for `TypeError::TraitBoundViolation` in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T043 [US3] Remove `_ =>` catch-all from `report_type_error()` — all variants now have dedicated handlers in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T044 [US3] Update `UndefinedSymbol` handler to show "did you mean `X`?" from candidates field in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T045 [US3] Update `ArgumentCountMismatch` handler to show function definition span as secondary label in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T046 [US3] Enhance `TypeMismatch` handler with context-aware conversion hints (int→string, float→int, string+non-string) in `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T047 [US3] Add Lang4LLM-specific lexer suggestions: `&&`→`and`, `||`→`or`, `!`→`not`, `null`→`none` in `lang4llm-rs/crates/parser/src/lexer.rs` and `lang4llm-rs/crates/cli/src/diagnostic.rs`
- [X] T048 [US3] Add tests for "did you mean?" suggestions, multi-span arg count, and Lang4LLM-specific suggestions in `lang4llm-rs/crates/cli/tests/`
- [X] T049 [US3] Verify `cargo test --workspace` passes and 92/92 benchmarks still pass

**Checkpoint**: All 12 TypeError variants have dedicated handlers. "Did you mean?" works. Multi-span labels show. SC-003 verified.

---

## Phase 6: User Story 4 — Source Code Formatter (Priority: P4)

**Goal**: `lang4llm fmt` command to reformat source files to canonical style.

**Independent Test**: Format benchmark reference impls, verify idempotency and semantic preservation.

### Implementation for User Story 4

- [X] T050 [US4] Create `lang4llm-formatter` crate: `Cargo.toml` with deps on `lang4llm-ast` and `lang4llm-parser`; add to workspace `Cargo.toml` members in `lang4llm-rs/crates/formatter/`
- [X] T051 [US4] Implement `CodegenContext` (copy from transpiler: buffer + 4-space indent + write/emit_line/indent/dedent) in `lang4llm-rs/crates/formatter/src/context.rs`
- [X] T052 [US4] Implement `fmt_type()` for all Type variants (Primitive, Array, Tuple, Generic, Function, Inferred) in `lang4llm-rs/crates/formatter/src/types.rs`
- [X] T053 [US4] Implement `fmt_pattern()` for all Pattern variants (Literal, Ident, Wildcard, Tuple, Struct, Enum) in `lang4llm-rs/crates/formatter/src/patterns.rs`
- [X] T054 [US4] Implement `fmt_expr()` with precedence-aware parenthesization for all Expr variants in `lang4llm-rs/crates/formatter/src/exprs.rs`
- [X] T055 [US4] Implement `fmt_stmt()` for all Stmt variants (Let, Assign, Expr, Return, Break, Continue, While, For, Loop, Empty) in `lang4llm-rs/crates/formatter/src/stmts.rs`
- [X] T056 [US4] Implement `fmt_function()`, `fmt_struct()`, `fmt_enum()`, `fmt_trait()`, `fmt_impl()` in `lang4llm-rs/crates/formatter/src/items.rs`
- [X] T057 [US4] Implement `format_source()` entry point: parse → fmt_program → return string; `format_file()` and `check_formatted()` file-level APIs in `lang4llm-rs/crates/formatter/src/lib.rs`
- [X] T058 [US4] Add unit tests for formatter: basic programs, indentation, operator spacing, struct literals, match arms in `lang4llm-rs/crates/formatter/src/lib.rs`
- [X] T059 [US4] Add `fmt` subcommand to CLI with `--check` and `--write` flags, file and directory support in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T060 [US4] Wire `fmt` subcommand to JSON output path (if `--format json`, serialize FormatResult) in `lang4llm-rs/crates/cli/src/main.rs`
- [X] T061 [US4] Add idempotency test: format all 92 benchmark reference impls twice, assert second pass produces no changes in `lang4llm-rs/crates/formatter/tests/`
- [X] T062 [US4] Add semantic preservation test: format all 92 benchmarks, compile+run, assert stdout matches original in `lang4llm-rs/crates/cli/tests/`
- [X] T063 [US4] Verify `cargo test --workspace` passes

**Checkpoint**: `lang4llm fmt` works on all benchmark files. Idempotent. Semantics preserved. SC-004, SC-005 verified.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, regression testing

- [X] T064 Document all error codes (E0001–E1002) with meaning, common causes, and fix suggestions in `lang4llm-rs/ERROR_CODES.md`
- [X] T065 [P] Update `lang4llm-rs/CLAUDE.md` with Phase 7 changes: new crate, SourceMap, formatter, --format flag
- [X] T066 Run full benchmark eval regression: 92/92 across all three tiers
- [X] T067 Run `cargo clippy --workspace` and fix any new warnings
- [X] T068 Run quickstart.md verification scenarios end-to-end
- [X] T069 Final `cargo test --workspace` — all tests green

**Checkpoint**: SC-001 through SC-006 all verified. Feature complete.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — BLOCKS all user stories
- **Phase 3 (US1 — Spans)**: Depends on Phase 2 — BLOCKS US2 and US3 (they need real spans)
- **Phase 4 (US2 — JSON)**: Depends on Phase 3 (US1) — sequential with US3 (shared `diagnostic.rs`)
- **Phase 5 (US3 — Diagnostics)**: Depends on Phase 4 (US2) — needs JSON structs for conversion updates
- **Phase 6 (US4 — Formatter)**: Depends on Phase 2 only — can run in PARALLEL with US1/US2/US3
- **Phase 7 (Polish)**: Depends on all user story phases

### Critical Path

```
Phase 1 → Phase 2 → Phase 3 (US1) → Phase 4 (US2) → Phase 5 (US3) → Phase 7
                  ↘ Phase 6 (US4) ─────────────────────────────────↗
```

US4 (Formatter) is the only user story that can run in parallel with US1-US3.

### File Conflict Map

| File | Phases |
|------|--------|
| `grammar.lalrpop` | Phase 2 (header), Phase 3 (138 rules) |
| `parser/lib.rs` | Phase 2 (signature) |
| `cli/main.rs` | Phase 2 (call sites), Phase 4 (--format), Phase 6 (fmt cmd) |
| `cli/diagnostic.rs` | Phase 4 (JSON path), Phase 5 (all handlers) |
| `typeck/typecheck.rs` | Phase 5 (TypeError mods) |
| `formatter/*` | Phase 6 only (no conflicts) |

---

## Parallel Example: Foundational Phase

```text
# After Phase 1 passes, these can run in parallel:
T010 [P] Update parse() in transpiler/lib.rs
T011 [P] Update parse() in all test files
# T009 (CLI) must come first since it's the primary call site
```

## Parallel Example: US4 Independence

```text
# After Phase 2 completes, US4 can start immediately:
T050-T063 (Formatter) — works on entirely new crate + CLI additions
# Meanwhile, US1 can proceed on grammar.lalrpop
# No file conflicts between US1 grammar work and US4 formatter crate
```

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1: Setup verification
2. Complete Phase 2: SourceMap + parse() signature
3. Complete Phase 3: US1 — all spans accurate
4. **STOP and VALIDATE**: Error messages cite correct lines
5. This alone delivers massive value — the #1 developer pain point resolved

### Incremental Delivery

1. Setup + Foundational → SourceMap infrastructure ready
2. US1 (Spans) → All errors have accurate locations
3. US2 (JSON) → LLM pipelines can parse errors programmatically
4. US3 (Diagnostics) → "Did you mean?" and multi-span labels
5. US4 (Formatter) → Canonical code formatting
6. Each story adds value without breaking previous stories

### Task Count Summary

| Phase | Tasks | Story |
|-------|-------|-------|
| Setup | 2 | — |
| Foundational | 10 | — |
| US1 (Spans) | 10 | P1 |
| US2 (JSON) | 9 | P2 |
| US3 (Diagnostics) | 18 | P3 |
| US4 (Formatter) | 14 | P4 |
| Polish | 6 | — |
| **Total** | **69** | |
