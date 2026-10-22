# Research Document: LLM-Friendly Programming Language Design

**Branch**: `001-llm-lang-research` | **Date**: 2026-01-20
**Purpose**: Consolidate research findings and methodology decisions for Phase 0

## Literature Sources

### Primary Research Papers

| ID | Title | Source | Key Contribution |
|----|-------|--------|------------------|
| P01 | Code Generation Error Characteristics | ICSE 2025 | Error taxonomy from 558 incorrect solutions across 6 LLMs |
| P02 | Where Do LLMs Still Struggle? | arXiv 2511.04355 | 4 recurring weakness patterns across major LLMs |
| P03 | LLM System Failure Modes | arXiv 2511.19933 | System-level failures in production |
| P04 | SynCode - Grammar Augmentation | arXiv 2403.01632 | 96.07% syntax error reduction with CFG constraints |
| P05 | Grammar-Based Code Representation | arXiv 2503.05507 | Grammar benefits beyond syntax |
| P06 | Type-Constrained Code Generation | arXiv 2504.09246 | Type constraints improve semantic correctness |
| P07 | What's Wrong with LLM-Generated Code | arXiv 2407.06153 | Detailed error analysis |
| P08 | Deep Dive into LLM Code Mistakes | arXiv 2411.01414 | Fine-grained mistake taxonomy |
| P09 | LLM Code Generation Error Characteristics | arXiv 2406.08731 | Error pattern analysis |
| P10 | Critical Review of Code Benchmarks | arXiv 2406.12655 | Benchmark limitations and conflicts |
| P11 | LLM-based Code Generation Agents Survey | arXiv 2508.00083 | Agent-based generation patterns |

### Tools and Frameworks

| ID | Name | Type | Relevance |
|----|------|------|-----------|
| T01 | GlyphLang | AI-First Language | Design principles: symbols > keywords, 35% fewer tokens |
| T02 | EvalPlus | Benchmark Framework | Extended test coverage (HumanEval 80x, MBPP 35x) |
| T03 | HumanEval | Benchmark | 164 Python problems, primary evaluation |
| T04 | MBPP | Benchmark | 974 entry-level tasks |
| T05 | SWE-bench | Benchmark | Real GitHub issues, complex scenarios |
| T06 | SynCode | Grammar Tool | CFG-constrained decoding implementation |

## Key Research Decisions

### Decision 1: Error Taxonomy Structure

**Decision**: Use hierarchical taxonomy with 3 top-level categories: Syntax, Semantic, Logic

**Rationale**:
- ICSE 2025 paper (P01) establishes this as consensus structure
- Aligns with compiler error classification
- Enables targeted language design interventions

**Alternatives Considered**:
- Flat list of errors: Rejected - no structure for prioritization
- Language-specific taxonomies: Rejected - prevents cross-language comparison
- Severity-based: Rejected - orthogonal dimension, can be added as attribute

### Decision 2: Benchmark Selection

**Decision**: Primary = HumanEval + EvalPlus extension; Secondary = MBPP; Validation = SWE-bench subset

**Rationale**:
- HumanEval: Most widely used, enables comparison with prior work
- EvalPlus: 80x more tests catches edge cases (P10 criticizes thin test coverage)
- MBPP: Entry-level tasks for simpler feature validation
- SWE-bench: Real-world complexity but noisy (selective use)

**Alternatives Considered**:
- HumanEval only: Rejected - known data leakage concerns
- Custom benchmark only: Rejected - no comparison with prior work
- All benchmarks equally: Rejected - resource constraints, prioritize depth

### Decision 3: Target Languages for Cross-Language Analysis

**Decision**: Python, Rust, TypeScript, Go

**Rationale**:
- Python: Most LLM training data, baseline for comparison
- Rust: Memory safety via ownership, strictest type system
- TypeScript: Popular typed language, structural typing
- Go: Simple grammar, explicit error handling, no generics (1.18+)

**Alternatives Considered**:
- Adding Java: Rejected - verbose, less distinct from TS
- Adding Haskell: Rejected - pure functional niche, less LLM training data
- C/C++: Rejected - memory safety issues conflate with LLM errors

### Decision 4: LLM Models for Experiments

**Decision**: GPT-4, Claude (Sonnet/Opus), DeepSeek-V3

**Rationale**:
- GPT-4: Industry standard, most comparison data exists
- Claude: Strong code generation, different architecture
- DeepSeek: Open-weights, enables reproducibility, strong benchmark scores

**Alternatives Considered**:
- Single model (GPT-4 only): Rejected - model-specific biases
- Many models (8+): Rejected - resource constraints, diminishing returns
- Open models only: Rejected - excludes best performers

### Decision 5: Metric Correlation Method

**Decision**: Spearman rank correlation with pass@1 as target metric

**Rationale**:
- Spearman: Robust to non-linear relationships, handles outliers
- pass@1: Most stringent, reflects practical use (one-shot generation)
- Rank-based: Avoids parametric assumptions

**Alternatives Considered**:
- Pearson correlation: Rejected - assumes linearity
- pass@k (k>1): Rejected - weaker signal, hides first-attempt failures
- Custom composite metric: Rejected - adds complexity without proven benefit

### Decision 6: Design Rule Validation Method

**Decision**: A/B testing with syntax variants + statistical significance test

**Rationale**:
- Direct measurement of rule impact on LLM accuracy
- Controls for confounding variables
- Reproducible methodology

**Alternatives Considered**:
- Expert review only: Rejected - subjective, not evidence-based
- Post-hoc analysis: Rejected - cannot establish causation
- Formal proof: Rejected - infeasible for all rules, complement not replace

## Methodology

### Error Collection Process

1. **Sample Selection**: Random 50 problems from each benchmark (150 total)
2. **Model Runs**: 3 attempts per problem per model (9 attempts × 150 = 1350 samples)
3. **Failure Extraction**: Collect all non-passing code
4. **Categorization**: Two-pass classification (automated then manual review)
5. **Frequency Counting**: Aggregate by error type, language, model

### Design Rule Derivation Process

1. **Error-to-Feature Mapping**: For each error pattern, identify language feature(s) that enable it
2. **Rule Synthesis**: Propose rule that prevents/mitigates the error
3. **Literature Validation**: Check if rule aligns with existing research
4. **Counter-Example Generation**: Find cases where rule would hurt
5. **Priority Assignment**: Rank by error frequency × severity

### Metric Computation Process

1. **Token Predictability**: Measure perplexity of next token given context (lower = better)
2. **Parse Ambiguity**: Count CFG derivation paths for same token sequence (fewer = better)
3. **AST Complexity**: Compute average depth × branching factor (lower = better)
4. **Error Recovery**: Measure % of partial programs that can be completed correctly

### Benchmark Construction Process

1. **Specification Writing**: Natural language + formal constraints + examples
2. **Reference Implementation**: Verified correct implementation
3. **Test Generation**: Property-based + edge case enumeration
4. **Difficulty Calibration**: Run on multiple LLMs, adjust tier assignment
5. **Coverage Verification**: Ensure all language features tested

## Open Questions

### OQ-1: How to handle model updates?

Models update frequently. Need strategy for:
- Version pinning during experiments
- Re-validation protocol when models change
- Longevity of findings

**Proposed Resolution**: Pin specific model versions (e.g., gpt-4-0613), document in methodology, plan annual re-validation.

### OQ-2: What constitutes "LLM-friendly" vs "beginner-friendly"?

Some design choices that help LLMs may also help human beginners (explicit types, clear syntax). How to separate?

**Proposed Resolution**: Compare LLM performance improvement vs beginner study data from literature. Focus on LLM-specific patterns (tokenization, context window) where human impact differs.

### OQ-3: How to validate rules for features that don't exist yet?

Benchmark for new language requires features to exist. Chicken-and-egg problem.

**Proposed Resolution**:
1. Test rules via proxy (e.g., "no implicit conversions" tested on Python vs TypeScript)
2. Use micro-benchmarks with minimal language subsets
3. Accept some rules as design axioms pending post-implementation validation

---

## Phase 3: Transpiler to Rust — Research Decisions

**Date**: 2026-02-21
**Purpose**: Technical decisions for Lang4LLM → Rust transpiler implementation

### Decision 7: Transpiler Architecture

**Decision**: AST-to-source string emission (direct code generation)

**Rationale**:
- Lang4LLM and Rust share significant syntactic similarity (structs, enums, match, generics, closures)
- AST-to-source is simplest to implement, debug, and maintain
- Output is human-readable Rust, aiding LLM users who inspect generated code
- No external dependency on `syn`/`proc-macro2` crates

**Alternatives Considered**:
- AST-to-AST via `syn` crate: Rejected — adds dependency, over-engineering for MVP
- IR-based (MIR/HIR): Rejected — optimization not needed at this stage, premature complexity
- Template-based (Tera/Askama): Rejected — too rigid for expression-level codegen

### Decision 8: String Concatenation Translation

**Decision**: Flatten `+` chains involving strings into `format!()` macro calls

**Rationale**:
- Lang4LLM allows `string + int` (implicit conversion), Rust does not
- `format!("{}{}", lhs, rhs)` handles all type combinations via `Display` trait
- Type checker already identifies string context for `BinOp::Add` (line 666-682 in typecheck.rs)
- Nested chains `"a" + b + "c" + d` flatten to `format!("{}{}{}{}", "a", b, "c", d)` for readability

**Alternatives Considered**:
- `.to_string() + &rhs.to_string()`: Rejected — verbose, allocates intermediate strings
- Runtime helper `l4l_concat()`: Rejected — pollutes generated Rust with non-standard functions
- Require explicit conversions: Rejected — changes Lang4LLM semantics

### Decision 9: Self Parameter Translation

**Decision**: Translate `self: Type` to `self` (by-value) in Rust

**Rationale**:
- Lang4LLM uses explicit `self: Type` parameter syntax
- By-value self is the correct semantic — Lang4LLM has no borrow semantics
- Keeps generated code correct without introducing lifetime complexity
- Future optimization: add `&self` when ownership analysis proves non-consuming use

**Alternatives Considered**:
- Always `&self`: Rejected — changes ownership semantics, some methods need owned self
- `&mut self` when method mutates: Rejected — requires mutation analysis not in current type checker
- Clone + `&self`: Rejected — hidden performance cost

### Decision 10: Builtin Function Translation Strategy

**Decision**: Argument-rewriting translation (transform call site, not inject runtime functions)

| Builtin | Lang4LLM | Rust Translation |
|---------|----------|-----------------|
| `print(s)` | `print("hello")` | `println!("{}", "hello")` |
| `range(a, b)` | `for i in range(0, n)` | `for i in 0..n` |
| `push(arr, item)` | `push(result, x)` | `result.push(x)` |
| `len(arr)` | `len(numbers)` | `numbers.len()` |
| `sqrt(x)` | `sqrt(val)` | `(val).sqrt()` |
| `as_float(x)` | `as_float(n)` | `(n as f64)` |
| `as_int(x)` | `as_int(s)` | Contextual: `s.parse::<i64>().unwrap()` or `s as i64` |

**Rationale**:
- Produces idiomatic Rust output (no runtime library needed)
- Each builtin has a clear, predictable translation
- `push` and `len` require argument rewriting (first arg becomes receiver)
- `range` in for-loop context becomes `a..b`; standalone requires `(a..b).collect()`

**Alternatives Considered**:
- Runtime library with wrapper functions: Rejected — non-idiomatic, adds dependency
- Trait-based dispatch: Rejected — over-engineering for 7 builtins

### Decision 11: Runtime Preamble

**Decision**: Minimal — only `#![allow(unused)]` and trait implementations when needed

**Rationale**:
- Most Lang4LLM constructs map directly to Rust without helpers
- `format!()` for string concatenation is stdlib, no preamble needed
- `println!()` for print is stdlib
- Avoid generating unused imports

**Generated preamble**:
```rust
// Generated by lang4llm transpiler v0.1.0
#![allow(unused_variables, unused_mut, dead_code)]
```

**Alternatives Considered**:
- Full prelude with helper functions: Rejected — pollutes output, hides semantics
- No preamble: Rejected — `unused_variables` warnings distracting in generated code

### Decision 12: Closure and Function Type Translation

**Decision**: `fn(T) -> U` parameters → `impl Fn(T) -> U`; local closure variables → inferred

| Context | Lang4LLM | Rust |
|---------|----------|------|
| Function parameter | `f: fn(T) -> U` | `f: impl Fn(T) -> U` |
| Local variable | `let f: fn(int) -> int = \|x\| x * 2` | `let f = \|x: i64\| x * 2` |
| Closure body | `\|x\| x * 2` | `\|x\| x * 2` (inferred) |
| Return type | `-> fn(T) -> U` | `-> Box<dyn Fn(T) -> U>` |

**Rationale**:
- `impl Fn` is most ergonomic for parameters (monomorphized, zero-cost)
- Local closures inferred by Rust compiler
- Return position requires `Box<dyn Fn>` (Rust sizing constraint)
- Matches how Rust developers would write equivalent code

**Alternatives Considered**:
- All `Box<dyn Fn>`: Rejected — unnecessary heap allocation for parameters
- Function pointers `fn(T) -> U`: Rejected — closures can't capture environment
- Generic bounds `where F: Fn(T) -> U`: Rejected — verbose, adds generic parameter

### Decision 13: Generic Type Translation

**Decision**: Direct mapping — Lang4LLM generics → Rust generics with inferred bounds

| Lang4LLM | Rust |
|----------|------|
| `fn identity<T>(x: T) -> T` | `fn identity<T>(x: T) -> T` |
| `struct Pair<T, U>` | `struct Pair<T, U>` |
| `impl<T, U> Pair<T, U>` | `impl<T, U> Pair<T, U>` |
| `<T: Display>` | `<T: std::fmt::Display>` |

**Rationale**:
- Lang4LLM generics were designed to mirror Rust's
- Direct mapping produces idiomatic output
- Trait bounds translate 1:1 (with path qualification)

### Decision 14: Array Slicing Translation

**Decision**: Use Rust's `Index` trait and slice syntax

| Lang4LLM | Rust |
|----------|------|
| `arr[0..5]` | `arr[0..5].to_vec()` |
| `arr[0..=4]` | `arr[0..=4].to_vec()` |
| `arr[5..]` | `arr[5..].to_vec()` |
| `arr[..5]` | `arr[..5].to_vec()` |

**Rationale**:
- Rust slice indexing syntax is identical
- `.to_vec()` needed because `[T]` → `Vec<T>` semantics (Lang4LLM returns new array)
- Type checker should determine if result needs `.to_vec()` based on assignment context

---

## Phase 4: Standard Library Foundation — Research Decisions

**Date**: 2026-02-21
**Purpose**: Technical decisions for stdlib scope, implementation strategy, and type integration

### Decision 15: Stdlib Scope (Foundation vs Extended)

**Decision**: Foundation (Phase 4) = 22 new builtin functions across 5 categories. Extended (Phase 8) deferred.

**Foundation scope** (22 new + 7 existing = 29 total):

| Category | New Functions | Existing |
|----------|-------------|----------|
| Math | `abs`, `min`, `max`, `pow`, `floor`, `ceil`, `round` (7) | `sqrt` |
| String | `to_string`, `contains`, `split`, `trim`, `replace`, `starts_with`, `ends_with`, `to_upper`, `to_lower`, `chars` (10) | — |
| Array | `sort`, `reverse`, `map`, `filter`, `is_empty`, `pop`, `enumerate`, `concat`, `join` (9) [†] | `push`, `len` |
| Conversion | — | `as_float`, `as_int` |
| I/O | — | `print` |

[†] `contains` is overloaded: works on both strings and arrays.

**Rationale**:
- DR-TOK-003 (Rich Standard Library, I=0.45) validated as INCLUDE
- These 22 functions cover the most common operations in HumanEval/MBPP benchmarks
- `map`, `filter`, `sort`, `reverse` directly specified in semantics spec (Iterator Protocol)
- `enumerate`, `pop` already specified in stdlib-signatures.md but unimplemented
- Foundation scope enables writing non-trivial programs without workarounds

**Deferred to Phase 8**:
- Math: sin, cos, tan, log, exp, atan2
- String: substring, pad_left, pad_right, repeat, format (advanced)
- Array: fold, reduce, zip, any, all, find, flat_map, take, skip, chunks
- I/O: read_line, read_file, write_file
- Debug: assert, panic, dbg, type_name

**Alternatives Considered**:
- Minimal (9 from stdlib-signatures.md only): Rejected — insufficient for useful programs, would frustrate LLM generation that expects common operations
- Comprehensive (50+ functions): Rejected — scope creep, diminishing returns, defer to Phase 8
- Method-only (no free functions): Rejected — breaks existing free-function convention

### Decision 16: Free Functions vs Method Calls

**Decision**: Hybrid — stdlib functions are free-function builtins with receiver rewriting; method-call passthrough continues to work for Rust-compatible calls.

**Free-function pattern** (new builtins follow existing convention):
```
Lang4LLM: sort(arr)      → Rust: arr.sort()
Lang4LLM: trim(s)        → Rust: s.trim().to_string()
Lang4LLM: map(arr, f)    → Rust: arr.into_iter().map(f).collect()
```

**Method-call passthrough** (already works, no changes):
```
Lang4LLM: arr.len()      → Rust: arr.len()
Lang4LLM: arr.push(x)    → Rust: arr.push(x)
Lang4LLM: s.contains(x)  → Rust: s.contains(x)  [but may need &x]
```

**Rationale**:
- Free-function style is consistent with existing 7 builtins (`print`, `push`, `len`, etc.)
- Transpiler already handles receiver rewriting (first arg → receiver) for `push`, `len`
- Method passthrough is zero-effort and already works in the transpiler
- LLMs generate both styles; supporting both maximizes compatibility

**Alternatives Considered**:
- Method-only: Rejected — requires grammar/parser changes, breaks existing examples
- Free-function only (no passthrough): Rejected — can't prevent passthrough, don't need to
- Trait-based dispatch: Rejected — over-engineering, Rust handles this

### Decision 17: Type Checker Integration Depth

**Decision**: Register all 29 stdlib functions in type checker symbol table with proper signatures. Use generic `T` for polymorphic functions.

**Rationale**:
- Current 7 builtins already registered in `typeck/builtins.rs` — extend this pattern
- Proper type signatures catch argument count/type errors at compile time
- Generic `T` parameter handles polymorphic functions (map, filter, sort, etc.)
- Without type registration, errors only surface at Rust compilation (poor DX)

**Implementation**: Extend `initialize_builtins()` in `crates/typeck/src/builtins.rs`

**Alternatives Considered**:
- Skip type checking for new builtins: Rejected — loses compile-time safety, contradicts DR-SEM-002
- Full type inference (infer from usage): Rejected — requires type inference engine not yet built
- Lazy registration (on first use): Rejected — adds complexity without benefit

### Decision 18: String Method Reference Handling

**Decision**: String builtins transpile with automatic `&str` coercion where needed. Return owned `String` for consistency.

**Pattern**:
```
Lang4LLM: contains(s, "hello")
Rust:     s.contains("hello")  // &str auto-coerces from String

Lang4LLM: replace(s, "old", "new")
Rust:     s.replace("old", "new")  // returns new String

Lang4LLM: trim(s)
Rust:     s.trim().to_string()  // .trim() returns &str, must own
```

**Rationale**:
- Rust's `String` auto-derefs to `&str` for method receivers — no explicit conversion needed
- String literal args are `&str` by default — compatible
- Variable args of type `String` auto-deref to `&str` — compatible
- Operations returning `&str` (trim, split items) need `.to_string()` for owned semantics
- Lang4LLM's `string` type = owned `String`, so returns must be owned

**Alternatives Considered**:
- Pass `&` explicitly: Rejected — Rust's auto-deref handles this
- Return `&str`: Rejected — conflicts with Lang4LLM's owned-string semantics

### Decision 19: Iterator Method Transpilation

**Decision**: `map(arr, f)` and `filter(arr, f)` transpile to `.into_iter().map(f).collect()` pattern. Mutating operations (`sort`, `reverse`) transpile to in-place methods.

**Pattern**:
```
Lang4LLM: map(arr, |x| x * 2)
Rust:     arr.into_iter().map(|x| x * 2).collect::<Vec<_>>()

Lang4LLM: filter(arr, |x| x > 0)
Rust:     arr.into_iter().filter(|x| *x > 0).collect::<Vec<_>>()

Lang4LLM: sort(arr)
Rust:     arr.sort()    // in-place mutation

Lang4LLM: reverse(arr)
Rust:     arr.reverse() // in-place mutation
```

**Rationale**:
- `into_iter()` consumes the vec (matches Lang4LLM's ownership semantics)
- `.collect::<Vec<_>>()` produces new Vec (matches return type expectations)
- `sort` and `reverse` are naturally mutating — Rust's Vec methods are in-place
- `filter` closure receives reference in Rust — auto-deref via `*x`

**Alternatives Considered**:
- `.iter().map().collect()` (borrowing): Rejected — requires clone for owned results
- Functional sort (return new array): Rejected — Rust has no built-in functional sort
- Custom iterator wrapper: Rejected — adds runtime complexity

### Decision 20: Overloaded `contains` Function

**Decision**: `contains` is a single builtin that dispatches based on first argument: string receiver → `str::contains()`, array receiver → `[T]::contains()`.

**Rationale**:
- Both string and array `contains` are extremely common in LLM-generated code
- Same semantic: "does this collection contain this element?"
- Transpiler can emit the same Rust pattern for both (receiver rewrite)
- Rust's type system disambiguates at compile time

**Implementation**:
```
contains(s, sub)     → s.contains(&sub)    // when s: string
contains(arr, elem)  → arr.contains(&elem) // when arr: [T]
```

Both need `&` on second arg because Rust `contains` takes reference.

**Alternatives Considered**:
- Separate names (str_contains, arr_contains): Rejected — unnecessary, LLMs expect single `contains`
- No overloading (string only): Rejected — array contains is equally essential

## References

See `research/literature/papers.md` for full annotated bibliography.
