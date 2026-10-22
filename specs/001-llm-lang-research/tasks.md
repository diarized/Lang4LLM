# Tasks: LLM-Friendly Programming Language Design Research

**Input**: Design documents from `/specs/001-llm-lang-research/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not applicable (research project, no runtime code)

**Organization**: Tasks grouped by user story (D1-D5 deliverables) to enable independent completion and validation.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which deliverable/user story this task belongs to (US1=D1, US2=D2, US3=D3, US4=D4, US5=D5)
- Include exact file paths in descriptions

## Path Conventions

Research deliverables in `research/` at repository root:
- `research/literature/` - Paper analysis
- `research/D1-error-taxonomy/` - Error patterns (US1)
- `research/D2-design-guide/` - Design rules (US2)
- `research/D3-metrics/` - LLM-friendliness metrics (US3)
- `research/D4-benchmark/` - Benchmark suite (US4)
- `research/D5-validation/` - Validation report (US5)

---

## Phase 1: Setup (Project Infrastructure)

**Purpose**: Create directory structure and initialize research environment

- [X] T001 Create research directory structure per plan.md in research/
- [X] T002 [P] Create literature notes template files in research/literature/notes/P01.md through P11.md
- [X] T003 [P] Create annotated bibliography template in research/literature/papers.md
- [X] T004 [P] Set up JSON schema validator script for deliverable validation
- [X] T005 Copy deliverable schemas to research/schemas/ from specs/001-llm-lang-research/contracts/deliverable-schemas.yaml

**Checkpoint**: Directory structure ready, templates in place

---

## Phase 2: Foundational - Literature Deep Dive (Blocking)

**Purpose**: Extract insights from existing research - MUST complete before any deliverable work

**⚠️ CRITICAL**: No deliverable (D1-D5) work can begin until literature review is complete

- [X] T006 Collect and download 11 primary papers (P01-P11) per research.md
- [X] T007 [P] Analyze P01 (ICSE 2025: Code Generation Errors) in research/literature/notes/P01.md
- [X] T008 [P] Analyze P02 (Where Do LLMs Struggle) in research/literature/notes/P02.md
- [X] T009 [P] Analyze P03 (LLM System Failure Modes) in research/literature/notes/P03.md
- [X] T010 [P] Analyze P04 (SynCode Grammar Augmentation) in research/literature/notes/P04.md
- [X] T011 [P] Analyze P05 (Grammar-Based Code Representation) in research/literature/notes/P05.md
- [X] T012 [P] Analyze P06 (Type-Constrained Code Generation) in research/literature/notes/P06.md
- [X] T013 [P] Analyze P07 (What's Wrong with LLM Code) in research/literature/notes/P07.md
- [X] T014 [P] Analyze P08 (Deep Dive into LLM Mistakes) in research/literature/notes/P08.md
- [X] T015 [P] Analyze P09 (LLM Code Generation Errors) in research/literature/notes/P09.md
- [X] T016 [P] Analyze P10 (Critical Review of Benchmarks) in research/literature/notes/P10.md
- [X] T017 [P] Analyze P11 (LLM Code Generation Agents Survey) in research/literature/notes/P11.md
- [X] T018 Synthesize consensus patterns across all papers in research/literature/papers.md
- [X] T019 Document research gaps requiring original experiments in research/literature/papers.md
- [X] T020 Extract initial error taxonomy structure from literature in research/literature/papers.md

**Checkpoint**: Literature review complete - deliverable work can now begin

---

## Phase 3: User Story 1 - Error Taxonomy (Priority: P1) 🎯 MVP

**Goal**: Catalog 50+ LLM code generation error patterns (Deliverable D1)

**Independent Test**: Validate taxonomy.md contains 50+ patterns with root causes, covering 3+ benchmarks and 4 languages

### Implementation for D1

- [X] T021 [US1] Set up evaluation environment: configure LLM API access (GPT-4, Claude, DeepSeek)
- [X] T022 [US1] Download HumanEval benchmark subset (50 problems) for error analysis
- [X] T023 [US1] Download MBPP benchmark subset (50 problems) for error analysis
- [X] T024 [US1] Download SWE-bench subset (representative sample) for error analysis
- [X] T025 [US1] Run GPT-4 on HumanEval subset, collect failed outputs in research/D1-error-taxonomy/data/gpt4-humaneval/
- [X] T026 [P] [US1] Run Claude on HumanEval subset, collect failed outputs in research/D1-error-taxonomy/data/claude-humaneval/
- [X] T027 [P] [US1] Run DeepSeek on HumanEval subset, collect failed outputs in research/D1-error-taxonomy/data/deepseek-humaneval/
- [X] T028 [US1] Categorize syntax errors from all models in research/D1-error-taxonomy/patterns/syntax/
- [X] T029 [US1] Categorize semantic errors from all models in research/D1-error-taxonomy/patterns/semantic/
- [X] T030 [US1] Categorize logic errors from all models in research/D1-error-taxonomy/patterns/logic/
- [X] T031 [P] [US1] Document Python-specific patterns in research/D1-error-taxonomy/patterns/by-language/python.md
- [X] T032 [P] [US1] Document Rust-specific patterns in research/D1-error-taxonomy/patterns/by-language/rust.md
- [X] T033 [P] [US1] Document TypeScript-specific patterns in research/D1-error-taxonomy/patterns/by-language/typescript.md
- [X] T034 [P] [US1] Document Go-specific patterns in research/D1-error-taxonomy/patterns/by-language/go.md
- [X] T035 [US1] Calculate frequency data for all patterns in research/D1-error-taxonomy/frequency-analysis.md
- [X] T036 [US1] Create cross-language comparison in research/D1-error-taxonomy/cross-language.md
- [X] T037 [US1] Compile main taxonomy document in research/D1-error-taxonomy/taxonomy.md
- [X] T038 [US1] Validate all pattern files against ErrorPattern schema
- [X] T039 [US1] Verify 50+ patterns, 3+ benchmarks, 4 languages coverage in taxonomy.md

**Checkpoint**: D1 Error Taxonomy complete - can verify 50+ patterns independently

---

## Phase 4: User Story 2 - Design Guide (Priority: P1)

**Goal**: Produce 20+ concrete, testable design rules (Deliverable D2)

**Independent Test**: Validate guide.md contains 20+ rules, each with research backing, validation method, and counter-examples

### Implementation for D2

- [X] T040 [US2] Map error patterns (D1) to language features that cause them in research/D2-design-guide/error-feature-mapping.md
- [X] T041 [US2] Draft syntax rules (DR-SYN-001 through DR-SYN-005) in research/D2-design-guide/rules/syntax/
- [X] T042 [US2] Draft grammar rules (DR-GRM-001 through DR-GRM-004) in research/D2-design-guide/rules/grammar/
- [X] T043 [US2] Draft semantic rules (DR-SEM-001 through DR-SEM-004) in research/D2-design-guide/rules/semantic/
- [X] T044 [US2] Draft token efficiency rules (DR-TOK-001 through DR-TOK-003) in research/D2-design-guide/rules/token/
- [X] T045 [US2] Draft error prevention rules (DR-ERR-001 through DR-ERR-004) in research/D2-design-guide/rules/error-prevention/
- [X] T046 [US2] Add research citations to each rule from literature review
- [X] T047 [US2] Define validation method for each rule (which benchmarks/metrics to use)
- [X] T048 [US2] Create counter-examples document in research/D2-design-guide/counter-examples.md
- [X] T049 [US2] Rank rules by impact on LLM accuracy (priority 1-20)
- [X] T050 [US2] Compile main design guide in research/D2-design-guide/guide.md
- [X] T051 [US2] Validate all rule files against DesignRule schema
- [X] T052 [US2] Verify 20+ rules with required fields in guide.md

**Checkpoint**: D2 Design Guide complete - can verify 20+ rules independently

---

## Phase 5: User Story 3 - Metrics Framework (Priority: P2)

**Goal**: Define and validate LLM-friendliness metrics (Deliverable D3)

**Independent Test**: Validate metrics show statistically significant correlation (p < 0.05) with benchmark performance

### Implementation for D3

- [X] T053 [US3] Define token predictability metric (M-001) in research/D3-metrics/definitions/M-001-token-predictability.md
- [X] T054 [P] [US3] Define parse ambiguity metric (M-002) in research/D3-metrics/definitions/M-002-parse-ambiguity.md
- [X] T055 [P] [US3] Define AST complexity metric (M-003) in research/D3-metrics/definitions/M-003-ast-complexity.md
- [X] T056 [P] [US3] Define error recovery metric (M-004) in research/D3-metrics/definitions/M-004-error-recovery.md
- [X] T057 [US3] Compute metrics for Python code samples in research/D3-metrics/data/python-scores.json
- [X] T058 [P] [US3] Compute metrics for Rust code samples in research/D3-metrics/data/rust-scores.json
- [X] T059 [P] [US3] Compute metrics for TypeScript code samples in research/D3-metrics/data/typescript-scores.json
- [X] T060 [P] [US3] Compute metrics for Go code samples in research/D3-metrics/data/go-scores.json
- [X] T061 [US3] Gather benchmark performance data (HumanEval pass@1 by language) in research/D3-metrics/data/benchmark-performance.json
- [X] T062 [US3] Calculate Spearman correlation coefficients in research/D3-metrics/correlations.md
- [X] T063 [US3] Validate statistical significance (p < 0.05) for each metric
- [X] T064 [US3] Compile main metrics document in research/D3-metrics/metrics.md
- [X] T065 [US3] Validate all metric files against Metric schema
- [X] T066 [US3] Verify correlation p-values in metrics.md

**Checkpoint**: D3 Metrics Framework complete - can verify correlation significance independently

---

## Phase 6: User Story 4 - Benchmark Suite (Priority: P2)

**Goal**: Create 100+ specification-implementation pairs for feature validation (Deliverable D4)

**Independent Test**: Validate benchmark contains 100+ specs across 3 tiers, each with reference implementation and tests

### Implementation for D4

- [X] T067 [US4] Define specification format template in research/D4-benchmark/spec-format.md
- [X] T068 [US4] Create 40 simple-tier specs (BS-S-001 through BS-S-040) in research/D4-benchmark/specs/simple/
- [X] T069 [P] [US4] Create 40 medium-tier specs (BS-M-001 through BS-M-040) in research/D4-benchmark/specs/medium/
- [X] T070 [P] [US4] Create 20 complex-tier specs (BS-C-001 through BS-C-020) in research/D4-benchmark/specs/complex/
- [X] T071 [US4] Write reference implementations for simple specs in research/D4-benchmark/reference/simple/
- [X] T072 [P] [US4] Write reference implementations for medium specs in research/D4-benchmark/reference/medium/
- [X] T073 [P] [US4] Write reference implementations for complex specs in research/D4-benchmark/reference/complex/
- [X] T074 [US4] Write test cases for simple specs in research/D4-benchmark/tests/simple/
- [X] T075 [P] [US4] Write test cases for medium specs in research/D4-benchmark/tests/medium/
- [X] T076 [P] [US4] Write test cases for complex specs in research/D4-benchmark/tests/complex/
- [X] T077 [US4] Verify all reference implementations pass their tests
- [X] T078 [US4] Run benchmark validation with GPT-4, Claude, DeepSeek
- [X] T079 [US4] Create benchmark README in research/D4-benchmark/README.md
- [X] T080 [US4] Validate all spec files against BenchmarkSpec schema
- [X] T081 [US4] Verify 100+ specs with balanced tier distribution

**Checkpoint**: D4 Benchmark Suite complete - can run LLM validation independently

---

## Phase 7: User Story 5 - Validation Report (Priority: P3)

**Goal**: Gate for proceeding to language implementation (Deliverable D5)

**Independent Test**: Validate report covers 100% of proposed features with recommendations and confidence levels

### Implementation for D5

- [X] T082 [US5] Define list of proposed language features from design rules (D2) in research/D5-validation/feature-list.md
- [X] T083 [US5] Test each syntax feature against benchmark, document in research/D5-validation/features/syntax/
- [X] T084 [P] [US5] Test each grammar feature against benchmark, document in research/D5-validation/features/grammar/
- [X] T085 [P] [US5] Test each semantic feature against benchmark, document in research/D5-validation/features/semantic/
- [X] T086 [P] [US5] Test each token feature against benchmark, document in research/D5-validation/features/token/
- [X] T087 [P] [US5] Test each error-prevention feature against benchmark, document in research/D5-validation/features/error-prevention/
- [X] T088 [US5] Calculate success rates and failure modes for each feature
- [X] T089 [US5] Generate include/exclude/modify recommendations with confidence levels
- [X] T090 [US5] Document alternatives for excluded/modified features
- [X] T091 [US5] Compile final validation report in research/D5-validation/report.md
- [X] T092 [US5] Validate all feature files against FeatureValidation schema
- [X] T093 [US5] Verify 100% feature coverage in report.md

**Checkpoint**: D5 Validation Report complete - Phase 0 gate ready for approval

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and documentation

- [X] T094 [P] Cross-reference all deliverables for consistency
- [X] T095 [P] Validate all JSON/YAML files against schemas in contracts/deliverable-schemas.yaml
- [X] T096 Run quickstart.md validation checklist
- [X] T097 Update specs/001-llm-lang-research/spec.md status to "Complete"
- [X] T098 Create executive summary of all findings in research/SUMMARY.md
- [X] T099 Final review: verify all success criteria (SC-001 through SC-008) are met

**Checkpoint**: Phase 0 Research complete - ready for language implementation phase

---

## Phase 9: Language Specification (Phase 1)

**Purpose**: Formal language specification — grammar, type system, semantics, memory model

**Beads epic**: `Lang4LLM_specification-stq` (Phase 1: Language Specification)

- [X] S001 Syntax Specification (`Lang4LLM_specification-6qv`)
- [X] S002 Grammar Specification — EBNF (`Lang4LLM_specification-5jl`)
- [X] S003 Type System Specification (`Lang4LLM_specification-n9d`)
- [X] S004 Semantics Specification (`Lang4LLM_specification-al0`)
- [X] S005 Memory Model Specification (`Lang4LLM_specification-isg`)
- [X] S006 Fix #1-5: Critical grammar issues — range, closure, loop, enum, implicit returns (`Lang4LLM_specification-cnx`)
- [X] S007 Fix #6-10: Important clarifications — array, string, self, stdlib, tuple (`Lang4LLM_specification-1z8`)
- [X] S008 Fix #11: Add generics to grammar and type system (`Lang4LLM_specification-jd8`)
- [X] S009 Fix #12-18: Moderate/minor documentation improvements (`Lang4LLM_specification-6ay`)
- [X] S010 Research LALRPOP conflict resolution (`Lang4LLM_specification-ldu`)
- [X] S011 Phase 1 Integration & Validation (`Lang4LLM_specification-w3f`)

**Checkpoint**: Language specification complete — ready for compiler implementation

---

## Phase 10: Parser & Type Checker (Phase 2)

**Purpose**: Rust implementation of parser + type checker validating all 12 example programs

**Beads epic**: `Lang4LLM_specification-iw0` (Phase 2: Parser & Type Checker Implementation) — **open (blocked on 2.11)**

- [X] I001 Phase 2.1: Project Setup — Rust workspace, crate structure (`Lang4LLM_specification-52j`)
- [X] I002 Phase 2.2: AST Definition — all node types, spans, serialization (`Lang4LLM_specification-7bi`)
- [X] I003 Phase 2.3: Lexer Implementation — all tokens, whitespace/comment handling (`Lang4LLM_specification-6rb`)
- [X] I004 Phase 2.4: Parser Implementation — LALRPOP grammar, all constructs (`Lang4LLM_specification-y3n`)
- [X] I005 Phase 2.5: Symbol Table — scoping, shadowing, multi-pass registration (`Lang4LLM_specification-oh3`)
- [X] I006 Phase 2.6: Type Checking Basics — literals, binary ops, let, return (`Lang4LLM_specification-ech`)
- [X] I007 Phase 2.7: Type Checking Advanced — generics, traits, impl blocks, exhaustiveness (`Lang4LLM_specification-8zj`)
- [X] I008 Phase 2.8: Error Reporting — typed errors, spans, diagnostics (`Lang4LLM_specification-61w`)
- [X] I009 Phase 2.9: CLI Tool — `check` and `parse` subcommands (`Lang4LLM_specification-h3y`)
- [X] I010 Phase 2.10: Testing — 12/12 integration examples, 20/20 error tests, 160 total tests (`Lang4LLM_specification-5b3`)
- [X] I011 Phase 2.11: Documentation — API docs, examples, CLAUDE.md (`Lang4LLM_specification-1e1`)

**Checkpoint**: 12/12 integration examples passing, 20/20 error detection tests passing (2026-02-21)

---

## Phase 11: Transpiler to Rust (Phase 3)

**Purpose**: Transpile Lang4LLM source → compilable Rust source code
**Prerequisites**: Phase 2 complete (parser + type checker validated)
**Testing Target**: All 12 example programs transpile, compile with `rustc`, produce correct output

**Beads epic**: `Lang4LLM_specification-aue` (Phase 3: Transpiler to Rust)

### Phase 11.1: Setup (Transpiler Crate)

**Goal**: Create transpiler crate skeleton in the workspace

- [X] C001 Create transpiler crate: `lang4llm-rs/crates/transpiler/Cargo.toml` with `lang4llm-ast` dependency, add `"crates/transpiler"` to workspace members in `lang4llm-rs/Cargo.toml`, add `lang4llm-transpiler = { path = "crates/transpiler" }` to workspace dependencies
- [X] C002 Create module file structure: `lang4llm-rs/crates/transpiler/src/lib.rs`, `context.rs`, `types.rs`, `builtins.rs`, `items.rs`, `stmts.rs`, `exprs.rs`, `patterns.rs` — each with module doc comment and placeholder public types

**Checkpoint**: `cargo check -p lang4llm-transpiler` succeeds

---

### Phase 11.2: Foundation (Context, Types, Builtins)

**Goal**: Type mapping, builtin recognition, and codegen infrastructure — blocks all codegen work

- [X] C003 Implement `CodegenContext` in `lang4llm-rs/crates/transpiler/src/context.rs` — buffer (`String`), `indent_level: u32`, methods: `write()`, `writeln()`, `indent()`, `dedent()`, `newline()`, `emit_indent()`, `finish() -> String`
- [X] C004 [P] Implement type mapping in `lang4llm-rs/crates/transpiler/src/types.rs` — function `emit_type(ty: &Type) -> String` covering: `int→i64`, `float→f64`, `bool→bool`, `string→String`, `unit→()`, `[T]→Vec<T>`, `(T,U)→(T,U)`, `fn(T)->U→impl Fn(T)->U` (param position), generic types, `Inferred→_`
- [X] C005 [P] Implement builtin function detection and translation in `lang4llm-rs/crates/transpiler/src/builtins.rs` — function `is_builtin(name: &str) -> bool`, function `emit_builtin_call(name: &str, args: &[Expr], ctx: &mut CodegenContext)` covering: `print→println!`, `range→a..b`, `push→receiver.push()`, `len→receiver.len()`, `sqrt→(x).sqrt()`, `as_float→(x as f64)`, `as_int→(x as i64)`
- [X] C006 Implement public API in `lang4llm-rs/crates/transpiler/src/lib.rs` — `pub fn transpile(program: &Program) -> Result<String, Vec<TranspileError>>`, `pub fn transpile_with_options(...)`, `TranspileOptions` struct, `TranspileError` enum, program-level emission (preamble `#![allow(unused)]`, iterate items)

**Checkpoint**: `cargo test -p lang4llm-transpiler` passes, type mapping covers all AST `Type` variants

---

### Phase 11.3: Codegen — Patterns & Expressions

**Goal**: Emit Rust code for all expression and pattern AST nodes

- [X] C007 Implement pattern codegen in `lang4llm-rs/crates/transpiler/src/patterns.rs` — function `emit_pattern(pat: &Pattern, ctx: &mut CodegenContext)` covering: `Literal`, `Ident`, `Wildcard(_)`, `Tuple((a, b))`, `Struct(Name { field: pat })`, `Enum(Name::Variant(pat))`
- [X] C008 Implement literal, variable, paren, and block expression codegen in `lang4llm-rs/crates/transpiler/src/exprs.rs` — function `emit_expr(expr: &Expr, ctx: &mut CodegenContext)` starting with: `Literal::Int/Float/Bool/String/None`, `Variable`, `Paren`, `Block { stmts; expr }`
- [X] C009 Implement binary expression codegen with string concat chain flattening in `lang4llm-rs/crates/transpiler/src/exprs.rs` — detect `BinOp::Add` involving string literal → collect chain into `format!("{}{}{}", a, b, c)`; numeric `+/-/*/÷/%` → direct operators; comparison → `==/</>` etc; logical `and→&&`, `or→||`
- [X] C010 Implement unary expression codegen in `lang4llm-rs/crates/transpiler/src/exprs.rs` — `not→!`, `Neg→-`
- [X] C011 Implement call expression codegen with builtin dispatch in `lang4llm-rs/crates/transpiler/src/exprs.rs` — check `is_builtin()` → delegate to `emit_builtin_call()`; regular calls → `func(args...)`; method calls via `FieldAccess` → `obj.method(args...)`
- [X] C012 Implement array, tuple, struct literal and access codegen in `lang4llm-rs/crates/transpiler/src/exprs.rs` — `ArrayLiteral→vec![...]`, `TupleLiteral→(a, b)`, `StructLiteral→Name { f: v }`, `ArrayAccess→arr[idx]`, `FieldAccess→obj.field`
- [X] C013 Implement if, match, range, closure, enum variant codegen in `lang4llm-rs/crates/transpiler/src/exprs.rs` — `IfExpr→if cond { } else { }`, `MatchExpr→match expr { arms }` (uses `emit_pattern`), `RangeExpr→start..end / start..=end`, `ClosureExpr→move |params| body`, `EnumVariantLiteral→Enum::Variant(args)`

**Checkpoint**: All `Expr` and `Pattern` variants handled, unit tests for string concat flattening

---

### Phase 11.4: Codegen — Statements & Items

**Goal**: Emit Rust code for all statement and top-level item AST nodes

- [X] C014 Implement statement codegen in `lang4llm-rs/crates/transpiler/src/stmts.rs` — function `emit_stmt(stmt: &Stmt, ctx: &mut CodegenContext)` covering: `Let→let [mut] name: Type = expr;`, `Assign→target = expr;`, `ExprStmt→expr;`, `Return→return expr;`, `Break→break;`, `Continue→continue;`, `While→while cond { body }`, `For→for var in iter { body }`, `Loop→loop { body }`, `Empty→;`
- [X] C015 Implement function codegen in `lang4llm-rs/crates/transpiler/src/items.rs` — function `emit_function(f: &FunctionDef, ctx: &mut CodegenContext)` covering: generic params `<T, U>`, parameter list (transform `self: Type` → `self`, regular params with mapped types), return type (omit for `unit`/`main`), body block
- [X] C016 [P] Implement struct and enum codegen in `lang4llm-rs/crates/transpiler/src/items.rs` — `emit_struct`: `#[derive(Debug, Clone)] struct Name<T> { fields }`, `emit_enum`: `#[derive(Debug, Clone)] enum Name<T> { Variant, Variant(types) }`
- [X] C017 [P] Implement trait and impl block codegen in `lang4llm-rs/crates/transpiler/src/items.rs` — `emit_trait`: `trait Name { fn signatures; }`, `emit_impl`: `impl [Trait for] Type { methods }`, handle generic params on impl blocks

**Checkpoint**: All `Stmt` and `Item` variants handled, can transpile simple programs end-to-end

---

### Phase 11.5: CLI Integration

**Goal**: Add `compile` subcommand to the CLI tool

- [X] C018 Add `compile` subcommand to `lang4llm-rs/crates/cli/src/main.rs` — parse → typecheck → transpile pipeline, `--output` / `-o` flag (default: `<input>.rs`), write generated Rust source to file, add `lang4llm-transpiler` dependency to `lang4llm-rs/crates/cli/Cargo.toml`
- [X] C019 Add `--check` and `--run` flags to compile subcommand in `lang4llm-rs/crates/cli/src/main.rs` — `--check`: invoke `rustc --edition 2021` on output, report success/failure; `--run`: compile + execute, capture and display output; `--verbose`: show transpilation details; proper exit codes (0=ok, 1=parse, 2=transpile, 3=rustc, 4=runtime)

**Checkpoint**: `cargo run --bin lang4llm compile ../language-spec/examples/01_hello_world.lang4llm` produces `.rs` file

---

### Phase 11.6: Integration Testing

**Goal**: Verify all 12 examples through full transpilation pipeline

- [X] C020 Write transpile integration test in `lang4llm-rs/tests/transpile_examples.rs` — for each of the 12 example programs: parse source, type-check, transpile, assert `Ok(rust_source)`, assert output is non-empty and contains `fn main`
- [X] C021 Write compile verification test in `lang4llm-rs/tests/transpile_compile.rs` — for each of the 12 transpiled outputs: write to temp file, invoke `rustc --edition 2021`, assert exit code 0; skip if `rustc` not available
- [X] C022 Write runtime verification test in `lang4llm-rs/tests/transpile_run.rs` — for examples with `print()` output (01, 02, 07, 10, 11): compile, execute, capture stdout, assert expected output substrings

**Checkpoint**: All 12 examples: transpile ✓, compile ✓, runtime output ✓

---

### Phase 11.7: Polish & Documentation

**Goal**: Documentation, unit test coverage, final verification

- [X] C023 [P] Add unit tests to codegen modules — at minimum: `types.rs` (all primitive mappings, array, tuple, function), `builtins.rs` (all 7 builtins), `patterns.rs` (all 6 pattern types), `context.rs` (indent/dedent)
- [X] C024 [P] Update `lang4llm-rs/README.md` — add Transpiler section: usage (`compile` subcommand), architecture diagram update (add transpiler stage), Phase 3 status
- [X] C025 Full pipeline verification — run `cargo test --workspace` (all existing Phase 2 tests + new transpiler tests pass), verify 12/12 transpile + compile, update Phase 3 checkpoint status

**Checkpoint**: Phase 3 complete — transpiler fully functional, documented, tested

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup
    ↓
Phase 2: Literature (BLOCKS all deliverables)
    ↓
┌───────────────────────────────────────────┐
│ Phase 3: D1 Error Taxonomy (P1) 🎯 MVP    │
│     ↓                                     │
│ Phase 4: D2 Design Guide (P1)             │
│     ↓                                     │
│ ┌─────────────────┬─────────────────┐     │
│ │ Phase 5: D3     │ Phase 6: D4     │     │
│ │ Metrics (P2)    │ Benchmark (P2)  │     │
│ └─────────────────┴─────────────────┘     │
│     ↓                   ↓                 │
│     └───────┬───────────┘                 │
│             ↓                             │
│ Phase 7: D5 Validation (P3)               │
└───────────────────────────────────────────┘
    ↓
Phase 8: Polish
    ↓
Phase 9: Language Specification (Phase 1) ✓
    ↓
Phase 10: Parser & Type Checker (Phase 2) ✓
    ↓
Phase 11: Transpiler to Rust (Phase 3)
    ├── 11.1 Setup
    │     ↓
    ├── 11.2 Foundation (types, builtins, context)
    │     ↓
    ├── 11.3 Codegen: Patterns & Expressions
    │     ↓
    ├── 11.4 Codegen: Statements & Items
    │     ↓
    ├── 11.5 CLI Integration
    │     ↓
    ├── 11.6 Integration Testing
    │     ↓
    └── 11.7 Polish & Documentation
```

### User Story Dependencies

- **US1 (D1 Error Taxonomy)**: Depends on Literature phase only
- **US2 (D2 Design Guide)**: Depends on US1 (error patterns inform rules)
- **US3 (D3 Metrics)**: Depends on US1 (need error data for correlation)
- **US4 (D4 Benchmark)**: Depends on US2 (rules inform spec design)
- **US5 (D5 Validation)**: Depends on US2, US3, US4 (needs all inputs)

### Parallel Opportunities

**Within Phase 2 (Literature)**:
- T007 through T017 (paper analysis) can all run in parallel

**Within Phase 3 (D1)**:
- T026, T027 (model runs) can run in parallel
- T031, T032, T033, T034 (language docs) can run in parallel

**Within Phase 5 (D3)**:
- T054, T055, T056 (metric definitions) can run in parallel
- T058, T059, T060 (metric computation) can run in parallel

**Within Phase 6 (D4)**:
- T069, T070 (spec creation) can run in parallel
- T072, T073 (reference impl) can run in parallel
- T075, T076 (test writing) can run in parallel

**Within Phase 7 (D5)**:
- T084, T085, T086, T087 (feature testing) can run in parallel

**Within Phase 11 (Transpiler)**:
- C004, C005 (type mapping, builtins) can run in parallel after C003
- C016, C017 (struct/enum, trait/impl) can run in parallel after C015
- C023, C024 (unit tests, README) can run in parallel

---

## Parallel Example: Literature Analysis

```bash
# Launch all paper analyses in parallel:
Task: "Analyze P01 (ICSE 2025: Code Generation Errors) in research/literature/notes/P01.md"
Task: "Analyze P02 (Where Do LLMs Struggle) in research/literature/notes/P02.md"
Task: "Analyze P03 (LLM System Failure Modes) in research/literature/notes/P03.md"
# ... (T007-T017 all parallel)
```

---

## Implementation Strategy

### MVP First (D1 Error Taxonomy Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Literature (CRITICAL - blocks all deliverables)
3. Complete Phase 3: D1 Error Taxonomy
4. **STOP and VALIDATE**: Verify 50+ patterns, 3+ benchmarks, 4 languages
5. D1 delivers immediate value as design reference

### Incremental Delivery

1. Setup + Literature → Foundation ready
2. D1 Error Taxonomy → Delivers design insights (MVP!)
3. D2 Design Guide → Delivers actionable rules
4. D3 Metrics + D4 Benchmark (parallel) → Delivers validation capability
5. D5 Validation → Gates implementation phase

### Research Team Strategy

With multiple researchers:

1. Team completes Setup + Literature together
2. Once Literature is done:
   - Researcher A: D1 Error Taxonomy
   - Researcher B: Start D2 outline (dependent on A)
3. After D1 + D2:
   - Researcher A: D3 Metrics
   - Researcher B: D4 Benchmark (parallel)
4. D5 Validation requires both D3 and D4

---

## Phase 12: Standard Library Foundation — Setup (Phase 4.0)

**Purpose**: Verify Phase 3 baseline, plan file modifications

- [X] D001 Run `cargo test --workspace` in `lang4llm-rs/` to verify all 155+ existing tests pass (regression baseline)
- [X] D002 Verify all 12 existing examples transpile+compile: `cargo test -p lang4llm-transpiler --test transpile_compile`

**Checkpoint**: Phase 3 baseline confirmed — stdlib work can begin

---

## Phase 13: Tier 2 — Specified Builtins (Phase 4.1) — US1: Core Builtins

**Goal**: Implement `pop` and `enumerate` — complete stdlib-signatures.md coverage

**Independent Test**: `pop(arr)` transpiles to `arr.pop().unwrap()`, `enumerate(arr)` transpiles to `arr.into_iter().enumerate()` in for-loop context

- [X] D003 [US1] Register `pop` type signature (fn(T) -> T) in `lang4llm-rs/crates/typeck/src/builtins.rs`
- [X] D004 [P] [US1] Register `enumerate` type signature (fn([T]) -> Iterator<(int, T)>) in `lang4llm-rs/crates/typeck/src/builtins.rs`
- [X] D005 [US1] Implement `pop` transpilation (`arr.pop().unwrap()`) in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D006 [P] [US1] Implement `enumerate` transpilation (`arr.into_iter().enumerate()`) in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D007 [US1] Add `pop` and `enumerate` to `is_builtin()` match in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D008 [US1] Add unit tests for `pop` and `enumerate` builtins in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D009 [US1] Run `cargo test --workspace` — verify existing 12 examples still pass + new tests green

**Checkpoint**: 9 builtins total (7 existing + 2 new). stdlib-signatures.md fully covered.

---

## Phase 14: Tier 3 — Math Builtins (Phase 4.2) — US2: Math Functions

**Goal**: Implement 7 math builtins: abs, min, max, pow, floor, ceil, round

**Independent Test**: Each math builtin transpiles to correct Rust method/function call, compiles with rustc

- [X] D010 [P] [US2] Register type signatures for `abs` (int->int, float->float), `min` (int,int->int), `max` (int,int->int), `pow` (int,int->int), `floor` (float->float), `ceil` (float->float), `round` (float->float) in `lang4llm-rs/crates/typeck/src/builtins.rs`
- [X] D011 [US2] Implement 7 math builtins transpilation in `lang4llm-rs/crates/transpiler/src/builtins.rs`: `abs(x)` -> `({x}).abs()`, `min(a,b)` -> `std::cmp::min({a},{b})`, `max(a,b)` -> `std::cmp::max({a},{b})`, `pow(b,e)` -> `({b}).pow({e} as u32)`, `floor(x)` -> `({x}).floor()`, `ceil(x)` -> `({x}).ceil()`, `round(x)` -> `({x}).round()`
- [X] D012 [US2] Add all 7 math names to `is_builtin()` match in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D013 [US2] Add unit tests for all 7 math builtins in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D014 [US2] Run `cargo test --workspace` — verify all existing + new tests pass

**Checkpoint**: 16 builtins total. All math operations available.

---

## Phase 15: Tier 4 — String Builtins (Phase 4.3) — US3: String Functions

**Goal**: Implement 10 string builtins: to_string, contains (string), split, trim, replace, starts_with, ends_with, to_upper, to_lower, chars

**Independent Test**: Each string builtin transpiles to correct Rust string method call, compiles with rustc

- [X] D015 [P] [US3] Register type signatures for all 10 string functions in `lang4llm-rs/crates/typeck/src/builtins.rs`: `to_string(T)->string`, `contains(string,string)->bool`, `split(string,string)->[string]`, `trim(string)->string`, `replace(string,string,string)->string`, `starts_with(string,string)->bool`, `ends_with(string,string)->bool`, `to_upper(string)->string`, `to_lower(string)->string`, `chars(string)->[string]`
- [X] D016 [US3] Implement 10 string builtins transpilation in `lang4llm-rs/crates/transpiler/src/builtins.rs` per contracts in `specs/001-llm-lang-research/contracts/stdlib-functions.md`
- [X] D017 [US3] Add all 10 string names to `is_builtin()` match in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D018 [US3] Add unit tests for all 10 string builtins in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D019 [US3] Run `cargo test --workspace` — verify all existing + new tests pass

**Checkpoint**: 26 builtins total. All string operations available.

---

## Phase 16: Tier 5 — Array Builtins (Phase 4.4) — US4: Array/Collection Functions

**Goal**: Implement 9 array builtins: sort, reverse, map, filter, is_empty, contains (array), concat, join, first

**Independent Test**: Each array builtin transpiles to correct Rust iterator/Vec method, compiles with rustc

- [X] D020 [P] [US4] Register type signatures for all 9 array functions in `lang4llm-rs/crates/typeck/src/builtins.rs`: `sort(mut [T])->unit`, `reverse(mut [T])->unit`, `map([T], fn(T)->U)->[U]`, `filter([T], fn(T)->bool)->[T]`, `is_empty([T])->bool`, `contains([T], T)->bool`, `concat([T],[T])->[T]`, `join([string], string)->string`, `first([T])->T`
- [X] D021 [US4] Implement 9 array builtins transpilation in `lang4llm-rs/crates/transpiler/src/builtins.rs` per contracts — key patterns: `sort`/`reverse` -> receiver rewrite, `map`/`filter` -> `into_iter().{op}().collect()`, `contains` -> `arr.contains(&item)`, `concat` -> block expression with extend, `join` -> `arr.join(sep)`, `first` -> `arr.first().cloned().unwrap()`
- [X] D022 [US4] Update `is_builtin()` to include all 9 array names, handling `contains` overload (already added for string in Phase 15) in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D023 [US4] Add unit tests for all 9 array builtins in `lang4llm-rs/crates/transpiler/src/builtins.rs`
- [X] D024 [US4] Run `cargo test --workspace` — verify all existing + new tests pass

**Checkpoint**: All 29 builtins implemented. `contains` overloaded for string and array.

---

## Phase 17: Example Programs (Phase 4.5) — US5: Validation

**Goal**: Write 3 new example programs demonstrating stdlib, verify full pipeline

**Independent Test**: All 3 new examples parse, typecheck, transpile, compile with rustc, and produce correct output

- [X] D025 [US5] Write `language-spec/examples/13_stdlib_math.lang4llm` — demonstrates: abs, min, max, pow, floor, ceil, round, as_float, to_string
- [X] D026 [P] [US5] Write `language-spec/examples/14_stdlib_strings.lang4llm` — demonstrates: contains, split, trim, replace, starts_with, ends_with, to_upper, to_lower, chars, join
- [X] D027 [P] [US5] Write `language-spec/examples/15_stdlib_collections.lang4llm` — demonstrates: sort, reverse, map, filter, is_empty, concat, first, enumerate, pop
- [X] D028 [US5] Verify all 3 new examples: `cargo run --bin lang4llm compile --run` for each in `lang4llm-rs/`
- [X] D029 [US5] Verify all 12 existing examples still pass: `cargo test -p lang4llm-transpiler --test transpile_compile`

**Checkpoint**: 15 example programs total. All parse + typecheck + transpile + compile + run.

---

## Phase 18: Integration Testing (Phase 4.6)

**Goal**: Add new examples to test harness, full regression

- [X] D030 [P] Add 3 new examples to `lang4llm-rs/crates/transpiler/tests/transpile_examples.rs` as `transpile_13_stdlib_math`, `transpile_14_stdlib_strings`, `transpile_15_stdlib_collections`
- [X] D031 [P] Add 3 new examples to `lang4llm-rs/crates/transpiler/tests/transpile_compile.rs` as `compile_13_stdlib_math`, `compile_14_stdlib_strings`, `compile_15_stdlib_collections`
- [X] D032 Add 3 new examples to `lang4llm-rs/crates/transpiler/tests/transpile_run.rs` with expected output assertions for `13_stdlib_math`, `14_stdlib_strings`, `15_stdlib_collections`
- [X] D033 Run full test suite: `cargo test --workspace` — target: 15/15 transpile, 15/15 compile, 8/8 runtime, 20/20 error-handling

**Checkpoint**: All integration tests green. 15/15 transpile, 15/15 compile, 8/8 runtime.

---

## Phase 19: Documentation & Polish (Phase 4.7)

**Purpose**: Update all documentation to reflect 29-function stdlib

- [X] D034 [P] Update `language-spec/stdlib-signatures.md` — add all 22 new function signatures with types and descriptions
- [X] D035 [P] Update `lang4llm-rs/README.md` — Phase 4 status section, stdlib function table, test count update (15/15)
- [X] D036 [P] Add comprehensive module-level doc comment to `lang4llm-rs/crates/transpiler/src/builtins.rs` listing all 29 builtins with their transpilation patterns
- [X] D037 Run final full verification: `cargo test --workspace` + `cargo clippy --workspace` — zero warnings
- [X] D038 Update `lang4llm-rs/crates/typeck/src/builtins.rs` module doc to list all 29 registered builtins

**Checkpoint**: Phase 4 complete — 29 stdlib functions, 15 examples, full documentation

---

## Dependencies & Execution Order (Phase 4)

### Phase Dependencies

```
Phase 12: Setup (verify baseline)
    ↓
Phase 13: Tier 2 — pop, enumerate (US1)
    ↓
Phase 14: Tier 3 — Math builtins (US2)
    ↓
Phase 15: Tier 4 — String builtins (US3)
    ↓
Phase 16: Tier 5 — Array builtins (US4)
    ↓
Phase 17: Example Programs (US5) — needs all builtins
    ↓
Phase 18: Integration Testing
    ↓
Phase 19: Documentation & Polish
```

### User Story Dependencies

- **US1 (Tier 2)**: No dependencies on other stories — can start after Phase 12
- **US2 (Math)**: Can start after US1 (shares builtins.rs files sequentially)
- **US3 (String)**: Can start after US2 (shares builtins.rs files sequentially)
- **US4 (Array)**: Can start after US3 (shares builtins.rs files sequentially)
- **US5 (Examples)**: Depends on US1-US4 (needs all builtins available)

> **Note**: US1-US4 share the same 2 files (`transpiler/builtins.rs` + `typeck/builtins.rs`), so they cannot truly run in parallel. However, within each phase, typeck and transpiler changes are [P] parallelizable.

### Parallel Opportunities

**Within Phase 13 (Tier 2)**:
- D003, D004 (typeck registrations) can run in parallel
- D005, D006 (transpiler implementations) can run in parallel

**Within Phase 14 (Math)**:
- D010 (typeck) can run in parallel with early work on D011

**Within Phase 17 (Examples)**:
- D025, D026, D027 (example programs) can all run in parallel (different files)

**Within Phase 18 (Integration Testing)**:
- D030, D031 (test file updates) can run in parallel

**Within Phase 19 (Docs)**:
- D034, D035, D036, D038 can all run in parallel (different files)

---

## Implementation Strategy (Phase 4)

### MVP First (Tier 2 + Math Only)

1. Complete Phase 12: Verify baseline
2. Complete Phase 13: Tier 2 builtins (pop, enumerate)
3. Complete Phase 14: Math builtins (abs, min, max, pow, floor, ceil, round)
4. **STOP and VALIDATE**: 16 builtins working, write a quick math example
5. Delivers immediate value: most-needed functions for numeric programs

### Incremental Delivery

1. Phase 12 → Baseline confirmed
2. Phase 13 (Tier 2) → stdlib-signatures fully covered
3. Phase 14 (Math) → Numeric programs enabled
4. Phase 15 (String) → Text processing enabled
5. Phase 16 (Array) → Collection pipelines enabled
6. Phase 17-19 → Examples, tests, docs

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific deliverable (US1=D1, US2=D2, etc.) for Phases 1-8
- For Phases 12-19 (Phase 4 stdlib): US1=Tier2, US2=Math, US3=String, US4=Array, US5=Examples
- Each deliverable should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate deliverable independently
- Phase 0 gate: All 5 deliverables must pass before compiler implementation
- Phase 4 adds 22 new builtins to the existing 7 (29 total)
- All builtins are transpiler-level translations — no runtime library needed
