# Implementation Plan: LLM-Friendly Programming Language Design Research

**Branch**: `001-llm-lang-research` | **Date**: 2026-01-20 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-llm-lang-research/spec.md`

## Summary

Phase 0 research to establish evidence-based design rules for a programming language optimized for LLM code generation. Produces 5 deliverables: error taxonomy (D1), design guide (D2), metrics framework (D3), benchmark suite (D4), and validation report (D5). All deliverables required before any compiler implementation begins.

## Technical Context

**Project Type**: Research/Documentation (no runtime code)
**Primary Tools**: Literature analysis, LLM API access, benchmark frameworks
**Output Format**: Markdown documents, structured data (JSON/YAML for benchmarks)
**Target Languages for Analysis**: Python, Rust, TypeScript, Go
**LLM Models for Experiments**: GPT-4, Claude, DeepSeek
**Benchmarks**: HumanEval, MBPP, SWE-bench, EvalPlus
**Grammar Tools**: SynCode (CFG-constrained decoding)

**Constraints**:
- Research must be reproducible (methodology documented)
- All claims must have research backing (citations)
- Deliverables must be independently verifiable

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Design Check (2026-01-20)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | Spec created before plan |
| II. Technology-Agnostic Specs | PASS | Spec describes research outcomes, not implementation |
| III. Template-Based Consistency | PASS | Following template structure |
| IV. Iterative Refinement | PASS | Checklist validated in spec phase |
| V. User Story Independence | PASS | 5 stories map to 5 independent deliverables |
| VI. Research-Driven Decisions | PASS | This IS the research phase |

### Post-Design Check (2026-01-20)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | Plan follows spec structure |
| II. Technology-Agnostic Specs | PASS | Tools in plan only, spec unchanged |
| III. Template-Based Consistency | PASS | data-model, contracts, quickstart generated |
| IV. Iterative Refinement | PASS | research.md resolves methodology decisions |
| V. User Story Independence | PASS | D1-D5 deliverables independently testable |
| VI. Research-Driven Decisions | PASS | All decisions documented with rationale |

**Gate Status**: PASSED - Ready for `/speckit:tasks`

## Project Structure

### Documentation (this feature)

```text
specs/001-llm-lang-research/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file
├── research.md          # Phase 0: Literature synthesis and methodology
├── checklists/
│   └── requirements.md  # Specification quality checklist (complete)
└── tasks.md             # Phase 2: Task breakdown (via /speckit:tasks)
```

### Research Deliverables (repository root)

```text
research/
├── D1-error-taxonomy/
│   ├── taxonomy.md           # Error pattern catalog (50+ patterns)
│   ├── patterns/             # Individual pattern files
│   │   ├── syntax/           # Syntax error patterns
│   │   ├── semantic/         # Semantic error patterns
│   │   └── logic/            # Logic error patterns
│   └── cross-language.md     # Python vs Rust vs TS vs Go comparison
│
├── D2-design-guide/
│   ├── guide.md              # Main design guide (20+ rules)
│   ├── rules/                # Individual rule files (DR-001, DR-002, etc.)
│   └── counter-examples.md   # What NOT to do
│
├── D3-metrics/
│   ├── metrics.md            # Metric definitions and computation methods
│   ├── correlations.md       # Correlation analysis with benchmarks
│   └── data/                 # Raw metric data (JSON)
│
├── D4-benchmark/
│   ├── README.md             # Benchmark usage guide
│   ├── specs/                # 100+ specification files
│   │   ├── simple/           # Tier 1: Single function
│   │   ├── medium/           # Tier 2: Multi-function
│   │   └── complex/          # Tier 3: Multi-module
│   ├── reference/            # Reference implementations
│   └── tests/                # Test cases for each spec
│
├── D5-validation/
│   ├── report.md             # Final validation report
│   └── features/             # Per-feature validation data
│
└── literature/
    ├── papers.md             # Annotated bibliography
    └── notes/                # Paper-by-paper analysis notes
```

**Structure Decision**: Research-centric layout with deliverables as top-level directories. Each deliverable (D1-D5) maps to a user story and can be completed/validated independently.

## Research Phases

### Phase 0.1: Literature Deep Dive (Blocking)

**Objective**: Extract actionable insights from existing research

**Tasks**:
1. Collect and organize 21+ papers from research spec references
2. Extract error taxonomies and frequency data from each paper
3. Identify consensus patterns across studies
4. Document gaps requiring original experiments
5. Create annotated bibliography

**Output**: `research/literature/papers.md`, `research/literature/notes/`

**Success Gate**: All referenced papers analyzed, consensus documented

### Phase 0.2: Error Analysis (Blocking)

**Objective**: Catalog LLM code generation failure modes

**Tasks**:
1. Set up evaluation environment (LLM APIs, benchmark access)
2. Run GPT-4, Claude, DeepSeek on HumanEval subset
3. Categorize all failures using taxonomy from literature
4. Identify language-specific error patterns (Python, Rust, TS, Go)
5. Quantify error frequencies
6. Cross-reference with literature findings

**Output**: `research/D1-error-taxonomy/`

**Success Gate**: 50+ error patterns cataloged, 3+ benchmarks covered

### Phase 0.3: Design Rule Synthesis

**Objective**: Translate error analysis into concrete design rules

**Tasks**:
1. Map error patterns to language features that cause them
2. Draft design rules to prevent each error class
3. Document rationale with research citations
4. Create counter-examples for each rule
5. Rank rules by impact on LLM accuracy

**Output**: `research/D2-design-guide/`

**Success Gate**: 20+ rules, each with rationale and validation method

### Phase 0.4: Metrics Definition

**Objective**: Create quantitative measures for LLM-friendliness

**Tasks**:
1. Define token predictability metric
2. Define parse ambiguity metric (CFG-based)
3. Define AST complexity metric
4. Define error recovery metric
5. Compute metrics for existing languages
6. Correlate with benchmark performance
7. Validate statistical significance

**Output**: `research/D3-metrics/`

**Success Gate**: Metrics correlate with performance (p < 0.05)

### Phase 0.5: Benchmark Construction

**Objective**: Create test suite for validating language features

**Tasks**:
1. Define specification format (natural language + constraints)
2. Write 100+ specifications across complexity tiers
3. Create reference implementations
4. Write test cases for each specification
5. Include edge cases
6. Validate with multiple LLMs

**Output**: `research/D4-benchmark/`

**Success Gate**: 100+ specs, balanced complexity, all features covered

### Phase 0.6: Feature Validation

**Objective**: Gate for proceeding to language implementation

**Tasks**:
1. Define list of proposed language features
2. Test each feature against benchmark suite
3. Document success rates and failure modes
4. Make include/exclude/modify recommendations
5. Calculate confidence levels
6. Compile final validation report

**Output**: `research/D5-validation/`

**Success Gate**: 100% feature coverage, all recommendations documented

## Complexity Tracking

No violations to justify. This is a research project with straightforward document-based deliverables.

## Dependencies & Risks

| Dependency | Risk | Mitigation |
|------------|------|------------|
| LLM API access | Rate limits, costs | Budget allocation, caching |
| Benchmark availability | Data leakage concerns | Use EvalPlus extended tests |
| Paper access | Paywalled research | arXiv preprints, institutional access |
| Compute for experiments | Resource constraints | Prioritize high-impact experiments |

## Timeline Constraints

**Hard Gate**: Phase 0 MUST complete before any compiler implementation.

**Phase Dependencies**:
```
0.1 Literature → 0.2 Error Analysis → 0.3 Design Rules
                                   ↘
                      0.4 Metrics ← 0.5 Benchmark → 0.6 Validation
```

Phases 0.3 and 0.4 can run in parallel after 0.2 completes.
Phase 0.5 can start after 0.3 completes (rules inform benchmark design).
Phase 0.6 requires all other phases complete.

---

## Phase 3: Transpiler to Rust

**Date**: 2026-02-21 | **Prerequisites**: Phase 2 complete (parser + type checker validated)
**Goal**: Transpile Lang4LLM source → compilable Rust source code

### Technical Context

**Project Type**: Compiler implementation (Rust crate)
**Language**: Rust 2021 edition (workspace member in `lang4llm-rs/`)
**Input**: Parsed + type-checked `Program` AST from `lang4llm-ast` crate
**Output**: Valid Rust source code as `String`
**Testing Target**: All 12 example programs transpile and compile with `rustc`

**Key Dependencies**:
- `lang4llm-ast` — AST definitions (existing)
- `lang4llm-parser` — Parser (existing, for pipeline integration)
- `lang4llm-typeck` — Type checker (existing, for pipeline integration)

### Architecture

```
Source (.lang4llm)
    │
    ▼
Lexer (crates/parser/src/lexer.rs)          ← existing
    │ Token stream
    ▼
Parser (crates/parser/src/grammar.lalrpop)  ← existing
    │ AST (crates/ast/)
    ▼
Type Checker (crates/typeck/)               ← existing
    │ Validated AST
    ▼
Transpiler (crates/transpiler/)             ← NEW (Phase 3)
    │ Rust source code (String)
    ▼
CLI Output / File (crates/cli/)             ← extended
```

### Crate Structure

```
crates/transpiler/
├── Cargo.toml
└── src/
    ├── lib.rs          # Public API: transpile(program) -> Result<String>
    ├── context.rs      # CodegenContext: buffer, indentation, scope tracking
    ├── types.rs        # Type mapping: Lang4LLM types → Rust types
    ├── builtins.rs     # Builtin function translation (print, range, push, len, sqrt, as_*)
    ├── items.rs        # Top-level items: fn, struct, enum, trait, impl
    ├── stmts.rs        # Statements: let, assign, return, loops, break/continue
    ├── exprs.rs        # Expressions: literals, binary, unary, call, match, if, closure
    └── patterns.rs     # Pattern codegen for match arms
```

### Type Mapping

| Lang4LLM | Rust | Notes |
|----------|------|-------|
| `int` | `i64` | Fixed-width for determinism |
| `float` | `f64` | IEEE 754 double |
| `bool` | `bool` | Direct |
| `string` | `String` | Owned string |
| `unit` | `()` | Unit type |
| `[T]` | `Vec<T>` | Heap-allocated array |
| `(T, U)` | `(T, U)` | Direct tuple mapping |
| `fn(T) -> U` | `impl Fn(T) -> U` | Parameter position |
| `fn(T) -> U` | `Box<dyn Fn(T) -> U>` | Return position |
| `Option<T>` | User-defined enum | Not Rust stdlib (transpiled as-is) |
| `Result<T, E>` | User-defined enum | Not Rust stdlib (transpiled as-is) |

### Operator Mapping

| Lang4LLM | Rust | Context |
|----------|------|---------|
| `and` | `&&` | Logical |
| `or` | `\|\|` | Logical |
| `not` | `!` | Logical |
| `+` (string) | `format!("{}{}", lhs, rhs)` | String concat |
| `+` (numeric) | `+` | Arithmetic |
| `..` | `..` | Range |
| `..=` | `..=` | Inclusive range |

### Builtin Translation

| Lang4LLM Call | Rust Output | Transformation |
|---------------|-------------|----------------|
| `print(expr)` | `println!("{}", expr)` | Macro call |
| `range(a, b)` | `a..b` | Operator, in for-loop context |
| `push(arr, x)` | `arr.push(x)` | Receiver rewrite |
| `len(arr)` | `arr.len()` | Receiver rewrite |
| `sqrt(x)` | `(x).sqrt()` | Method on f64 |
| `as_float(x)` | `(x as f64)` | Cast expression |
| `as_int(x)` | `(x as i64)` | Cast expression |

### Special Transformations

**String concatenation chain flattening**:
```
Lang4LLM: "Point(" + self.x + ", " + self.y + ")"
Rust:     format!("Point({}, {})", self.x, self.y)
```

**Self parameter**:
```
Lang4LLM: fn method(self: Type) -> T
Rust:     fn method(self) -> T
```

**Struct derive macros** (generated for all structs/enums):
```rust
#[derive(Debug, Clone)]
struct Point { x: i64, y: i64 }
```

### Testing Strategy

**Unit tests**: Per-module tests for each codegen module
**Integration tests**: All 12 example programs must:
1. Transpile without error (Lang4LLM → Rust string)
2. Compile with `rustc` (syntax + type validity)
3. Produce output matching expected behavior (runtime correctness)

**Test matrix**:

| Example | Key Features Tested |
|---------|-------------------|
| 01_hello_world | print, main, unit return |
| 02_fibonacci | mut, for-range, string concat |
| 03_pattern_matching | user-defined enum, match, generics |
| 04_structs | struct, impl, self, method call |
| 05_generics | generic fn, generic struct, generic impl |
| 06_result_error | Result enum, match on enum, string concat |
| 07_arrays | Vec, indexing, for-in iteration |
| 08_closures | closure syntax, higher-order fn, fn type params |
| 09_traits | trait def, impl for trait, trait bounds |
| 10_complex_matching | nested patterns, struct patterns |
| 11_loop_control | while, for, continue, enum |
| 12_ranges | range ops, array slicing |

### Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| String concat without type info | Medium | Heuristic: detect string literal in `+` chain |
| Closure captures | Medium | Use `move` by default for closures |
| Trait method dispatch | High | Start with static dispatch, skip dynamic for MVP |
| Array slicing lifetimes | Low | Always `.to_vec()` for owned semantics |
| Generic trait bounds | Medium | Direct mapping works for simple cases |

### Constitution Check (Phase 3)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | Plan created before implementation |
| II. Technology-Agnostic Specs | N/A | Phase 3 is inherently technology-specific (Rust target) |
| III. Template-Based Consistency | PASS | Following plan template structure |
| IV. Iterative Refinement | PASS | 8 research decisions documented with alternatives |
| V. User Story Independence | PASS | Transpiler is independently testable via 12 examples |
| VI. Research-Driven Decisions | PASS | All design choices justified with rationale |

---

## Phase 4: Standard Library Foundation

**Date**: 2026-02-21 | **Prerequisites**: Phase 3 complete (transpiler + all 12 examples compile)
**Goal**: Implement 22 new stdlib functions across 5 categories, with type checker integration and new example programs

### Technical Context

**Project Type**: Compiler extension (Rust crates)
**Language**: Rust 2021 edition (workspace member in `lang4llm-rs/`)
**Affected Crates**: `lang4llm-transpiler` (builtins), `lang4llm-typeck` (signatures), `lang4llm-cli` (unchanged)
**Testing Target**: All new builtins transpile, compile with `rustc`, produce correct output
**Design Rules**: DR-TOK-003 (Rich Standard Library, I=0.45)

**Key Dependencies**:
- `lang4llm-ast` — AST definitions (unchanged)
- `lang4llm-parser` — Parser (unchanged — builtins are regular function calls)
- `lang4llm-typeck` — Type checker builtins registry (extended)
- `lang4llm-transpiler` — Builtin translation (extended)
- `language-spec/stdlib-signatures.md` — Phase 1 forward declarations (completed)

### Architecture

Stdlib is implemented as **transpiler-level translations** — no runtime library needed. Each builtin maps to idiomatic Rust code via argument rewriting.

```
Lang4LLM call (free function)
    │
    ▼
Transpiler builtins.rs: is_builtin() → emit_builtin_call()
    │ Argument rewriting (receiver swap, reference insertion, collect wrapping)
    ▼
Idiomatic Rust output (stdlib methods, macros, operators)
```

**No grammar/parser changes required.** Builtins are syntactically identical to regular function calls.

### Stdlib Function Registry

#### Tier 1: Already Implemented (7 functions — no changes)

| Function | Lang4LLM | Rust Output |
|----------|----------|-------------|
| `print(s)` | `print("hello")` | `println!("{}", "hello")` |
| `range(a, b)` | `range(0, n)` | `0..n` |
| `push(arr, x)` | `push(result, x)` | `result.push(x)` |
| `len(x)` | `len(arr)` | `arr.len()` |
| `sqrt(x)` | `sqrt(16.0)` | `(16.0).sqrt()` |
| `as_float(x)` | `as_float(n)` | `(n as f64)` |
| `as_int(x)` | `as_int(s)` | `format!("{}", s).parse::<i64>().unwrap_or(0)` |

#### Tier 2: Specified but Unimplemented (2 functions)

| Function | Lang4LLM | Rust Output |
|----------|----------|-------------|
| `pop(arr)` | `pop(numbers)` | `numbers.pop()` |
| `enumerate(arr)` | `for (i, x) in enumerate(arr)` | `for (i, x) in arr.into_iter().enumerate()` |

#### Tier 3: New Math Functions (7 functions)

| Function | Lang4LLM | Rust Output | Notes |
|----------|----------|-------------|-------|
| `abs(x)` | `abs(-5)` | `(-5_i64).abs()` | Works on int and float |
| `min(a, b)` | `min(x, y)` | `std::cmp::min(x, y)` | Integer version |
| `max(a, b)` | `max(x, y)` | `std::cmp::max(x, y)` | Integer version |
| `pow(base, exp)` | `pow(2, 10)` | `(2_i64).pow(10 as u32)` | Integer exponentiation |
| `floor(x)` | `floor(3.7)` | `(3.7).floor()` | Float → Float |
| `ceil(x)` | `ceil(3.2)` | `(3.2).ceil()` | Float → Float |
| `round(x)` | `round(3.5)` | `(3.5).round()` | Float → Float |

#### Tier 4: New String Functions (10 functions)

| Function | Lang4LLM | Rust Output |
|----------|----------|-------------|
| `to_string(x)` | `to_string(42)` | `format!("{}", 42)` |
| `contains(s, sub)` | `contains(s, "hello")` | `s.contains("hello")` |
| `split(s, delim)` | `split(csv, ",")` | `csv.split(",").map(\|s\| s.to_string()).collect::<Vec<String>>()` |
| `trim(s)` | `trim(input)` | `input.trim().to_string()` |
| `replace(s, from, to)` | `replace(s, "old", "new")` | `s.replace("old", "new")` |
| `starts_with(s, pre)` | `starts_with(s, "http")` | `s.starts_with("http")` |
| `ends_with(s, suf)` | `ends_with(s, ".rs")` | `s.ends_with(".rs")` |
| `to_upper(s)` | `to_upper(name)` | `name.to_uppercase()` |
| `to_lower(s)` | `to_lower(name)` | `name.to_lowercase()` |
| `chars(s)` | `chars("hello")` | `"hello".chars().map(\|c\| c.to_string()).collect::<Vec<String>>()` |

#### Tier 5: New Array Functions (9 functions)

| Function | Lang4LLM | Rust Output | Semantics |
|----------|----------|-------------|-----------|
| `sort(arr)` | `sort(numbers)` | `numbers.sort()` | In-place mutation |
| `reverse(arr)` | `reverse(arr)` | `arr.reverse()` | In-place mutation |
| `map(arr, f)` | `map(arr, \|x\| x * 2)` | `arr.into_iter().map(\|x\| x * 2).collect::<Vec<_>>()` | New array |
| `filter(arr, f)` | `filter(arr, \|x\| x > 0)` | `arr.into_iter().filter(\|x\| *x > 0_i64).collect::<Vec<_>>()` | New array |
| `is_empty(arr)` | `is_empty(list)` | `list.is_empty()` | Boolean |
| `contains(arr, x)` | `contains(arr, 42)` | `arr.contains(&42)` | Boolean (overloaded) |
| `concat(a, b)` | `concat(arr1, arr2)` | `{ let mut __v = arr1; __v.extend(arr2); __v }` | New array |
| `join(arr, sep)` | `join(words, ", ")` | `words.join(", ")` | String |
| `first(arr)` | `first(arr)` | `arr.first().cloned()` | Option-like |

### Implementation Phases

#### Phase 4.1: Transpiler Builtins — Tier 2 (Specified)

**Goal**: Implement `pop` and `enumerate` — complete stdlib-signatures.md coverage

**Files**:
- `crates/transpiler/src/builtins.rs` — Add 2 builtins
- `crates/typeck/src/builtins.rs` — Register 2 type signatures

**Checkpoint**: Existing 12 examples still pass + new builtins unit-tested

#### Phase 4.2: Transpiler Builtins — Math (Tier 3)

**Goal**: Implement 7 math builtins

**Files**:
- `crates/transpiler/src/builtins.rs` — Add 7 builtins
- `crates/typeck/src/builtins.rs` — Register 7 type signatures

**Checkpoint**: Unit tests for all math builtins, existing tests pass

#### Phase 4.3: Transpiler Builtins — String (Tier 4)

**Goal**: Implement 10 string builtins

**Files**:
- `crates/transpiler/src/builtins.rs` — Add 10 builtins (+ overloaded `contains`)
- `crates/typeck/src/builtins.rs` — Register 10 type signatures

**Checkpoint**: Unit tests for all string builtins

#### Phase 4.4: Transpiler Builtins — Array (Tier 5)

**Goal**: Implement 9 array builtins (including `map`, `filter`, `sort`, `reverse`)

**Files**:
- `crates/transpiler/src/builtins.rs` — Add 9 builtins
- `crates/typeck/src/builtins.rs` — Register 9 type signatures

**Checkpoint**: Unit tests for all array builtins

#### Phase 4.5: Example Programs

**Goal**: Write 3 new example programs demonstrating stdlib usage

**New examples**:
- `13_stdlib_math.lang4llm` — Math functions, type conversion, numeric algorithms
- `14_stdlib_strings.lang4llm` — String manipulation, split/join, transformations
- `15_stdlib_collections.lang4llm` — map/filter/sort/reverse, collection pipelines

**Files**:
- `language-spec/examples/13_stdlib_math.lang4llm`
- `language-spec/examples/14_stdlib_strings.lang4llm`
- `language-spec/examples/15_stdlib_collections.lang4llm`

**Checkpoint**: All 3 parse + typecheck + transpile + compile + run

#### Phase 4.6: Integration Testing

**Goal**: Full test coverage for all 29 stdlib functions

**Files**:
- `crates/transpiler/tests/transpile_examples.rs` — Add 3 new examples
- `crates/transpiler/tests/transpile_compile.rs` — Add 3 compile tests
- `crates/transpiler/tests/transpile_run.rs` — Add 3 runtime tests

**Checkpoint**: 15/15 transpile, 15/15 compile, 8/8 runtime

#### Phase 4.7: Documentation

**Goal**: Update documentation, stdlib-signatures.md, README

**Files**:
- `language-spec/stdlib-signatures.md` — Add all 22 new function signatures
- `lang4llm-rs/README.md` — Update Phase 4 status, stdlib section
- `lang4llm-rs/crates/transpiler/src/builtins.rs` — Module-level doc comments

**Checkpoint**: Documentation complete, all tests pass

### Testing Strategy

**Unit tests** (per builtin):
```rust
// In crates/transpiler/src/builtins.rs
#[test]
fn test_sort_builtin() {
    let mut ctx = CodegenContext::new();
    emit_builtin_call("sort", &[var("arr")], &mut ctx, &mut emit_expr);
    assert_eq!(ctx.finish(), "arr.sort()");
}
```

**Integration tests** (per example program):
1. Parse → type-check → transpile → assert non-empty Rust output
2. Write to temp file → `rustc --edition 2021` → assert exit code 0
3. Compile → execute → assert expected stdout

**Regression**: All existing 12 examples must continue passing

### Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| `filter` closure needs `*x` deref | Medium | Emit `*x` for filter predicates |
| `map`/`filter` ownership (into_iter vs iter) | Medium | Use `into_iter()` + `collect()`, clone if needed |
| `contains` overload ambiguity | Low | Transpiler doesn't need type info — both emit same pattern |
| `sort` requires `Ord` trait bound | Medium | Works for primitive types; generic sort needs bounds |
| `join` only works on `Vec<String>` | Low | Document constraint, panic at Rust compile if misused |
| `pow` argument types (u32 exponent) | Low | Cast exponent: `exp as u32` |

### Constitution Check (Phase 4)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Specification-First | PASS | stdlib-signatures.md (Phase 1) precedes implementation |
| II. Technology-Agnostic Specs | N/A | Phase 4 is inherently technology-specific (Rust transpilation) |
| III. Template-Based Consistency | PASS | Following plan template structure |
| IV. Iterative Refinement | PASS | 6 research decisions (R15-R20) documented with alternatives |
| V. User Story Independence | PASS | Stdlib testable independently via new example programs |
| VI. Research-Driven Decisions | PASS | DR-TOK-003 validates stdlib approach, all decisions justified |
