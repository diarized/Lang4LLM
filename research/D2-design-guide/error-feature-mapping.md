# Error Pattern → Language Feature Mapping (T040)

**Purpose**: Map each error pattern from D1 to the language features that enable or prevent it
**Date**: 2026-01-20
**Input**: D1 Error Taxonomy (`research/D1-error-taxonomy/taxonomy.md`)
**Output**: Feature analysis → design rules (DR-001 through DR-020)

---

## Mapping Methodology

For each error pattern:
1. Identify **trigger features**: Language characteristics that enable the error
2. Identify **prevention features**: Design choices that would prevent/mitigate it
3. Rate **mitigation impact**: HIGH (>50% reduction), MEDIUM (20-50%), LOW (<20%)
4. Extract **design rule candidate**: Concrete language design recommendation

---

## Syntax Error Patterns (4)

### EP-SYN-001: Missing Delimiter

**Error Pattern**: Mismatched or missing brackets, braces, parentheses (12% frequency)

**Trigger Features**:
- Complex nesting (>3 levels)
- Multiple delimiter types (`[]`, `{}`, `()`)
- Multiline expressions without explicit continuations
- Implicit scope boundaries

**Prevention Features**:
- ✅ **Explicit delimiters with minimal nesting** (HIGH impact)
  - Limit nesting depth to 3-4 levels syntactically
  - Use single delimiter type where possible
  - Require explicit line continuations

- ✅ **Structural syntax highlighting in grammar** (MEDIUM impact)
  - Grammar rules that enforce balanced delimiters
  - Editor-friendly token boundaries

**Design Rule Candidate**: **DR-SYN-001** - Explicit Delimiters with Nesting Limits

**Research Backing**:
- P04 (SynCode): CFG constraints reduce syntax errors 96%
- P05: Grammar-based representation improves structural correctness

---

### EP-SYN-002: Incorrect Indentation

**Error Pattern**: Wrong whitespace levels (8% overall, 20% in Python)

**Trigger Features**:
- Significant whitespace for scope
- Mixed tabs/spaces
- Inconsistent indentation semantics
- Invisible characters affecting parsing

**Prevention Features**:
- ✅ **No significant whitespace** (HIGH impact)
  - Use explicit delimiters (`{}`, `begin/end`) instead of indentation
  - Whitespace is style, not syntax

- ✅ **Strict formatting rules** (MEDIUM impact)
  - Enforce consistent indentation via formatter
  - Reject mixed tabs/spaces

**Design Rule Candidate**: **DR-SYN-002** - No Significant Whitespace

**Research Backing**:
- D1 cross-language analysis: Python (20% indentation errors) vs Rust (0%)
- GlyphLang design: Symbols over whitespace-dependent syntax

---

### EP-SYN-003: Off-by-One Index

**Error Pattern**: Array bounds errors, `[i-1]` vs `[i]` confusion (15% frequency)

**Trigger Features**:
- Zero-based indexing
- Manual bounds checking required
- Implicit slice boundaries
- Inclusive vs exclusive ranges (e.g., `0..10` vs `0...10`)

**Prevention Features**:
- ✅ **Bounds-checked indexing** (HIGH impact)
  - Compiler inserts runtime checks (like Rust)
  - Panic/exception on out-of-bounds access

- ✅ **Range syntax clarity** (HIGH impact)
  - Explicit inclusive (`0..=10`) vs exclusive (`0..<10`) notation
  - No ambiguous range operators

- ✅ **One-based indexing option** (LOW impact)
  - Controversial, but eliminates -1/+1 mental overhead
  - Trade-off: Conflicts with C interop conventions

**Design Rule Candidate**: **DR-SYN-003** - Bounds-Checked Indexing with Clear Ranges

**Research Backing**:
- P01, P02: Boundary conditions are #1 logic error
- Rust ownership model: Bounds checking eliminates entire error class

---

### EP-SYN-004: Incorrect Operator

**Error Pattern**: Wrong operator choice (`=` vs `==`, `&` vs `&&`) (10% frequency)

**Trigger Features**:
- Similar-looking operators with different semantics (`=` assign vs `==` compare)
- Overloaded operators (bitwise vs logical AND)
- Implicit type conversions enabling wrong operator usage
- Operator precedence ambiguity

**Prevention Features**:
- ✅ **Distinct operator syntax** (HIGH impact)
  - Assignment: `:=` or `<-` (not `=`)
  - Equality: `==` (reserved for comparison only)
  - No bitwise operators in logical context

- ✅ **Explicit precedence** (MEDIUM impact)
  - Require parentheses for mixed precedence
  - Minimal operator overloading

**Design Rule Candidate**: **DR-SYN-004** - Explicit Operator Distinction

**Research Backing**:
- Go: `:=` for declaration eliminates `=` vs `==` confusion
- P07: Operator confusion in 10% of errors

---

## Semantic Error Patterns (4)

### EP-SEM-001: Undefined Variable

**Error Pattern**: Using variable before declaration (18% frequency)

**Trigger Features**:
- No mandatory variable declarations (`var`, `let`, `const`)
- Global scope pollution
- Dynamic scoping
- Implicit variable creation

**Prevention Features**:
- ✅ **Mandatory variable declarations** (HIGH impact - 60% reduction)
  - Require explicit `let x: T = value` or `var x = value`
  - No implicit globals
  - Lexical scoping only

- ✅ **Compile-time name resolution** (HIGH impact)
  - Symbol tables checked at compile time
  - Unused variable warnings

**Design Rule Candidate**: **DR-SEM-001** - Mandatory Variable Declarations

**Research Backing**:
- D1: Python (dynamic) 25% undefined errors vs Rust (explicit) 5%
- TypeScript: Adding type annotations reduces semantic errors by 40%

---

### EP-SEM-002: Type Mismatch

**Error Pattern**: Incompatible types in assignment/function calls (16% frequency)

**Trigger Features**:
- Dynamic typing (no compile-time checks)
- Implicit type conversions
- Union types without narrowing
- Structural typing without validation

**Prevention Features**:
- ✅ **Strong static typing** (HIGH impact - 70% reduction)
  - Mandatory type annotations for function signatures
  - Type inference for local variables only
  - No implicit conversions

- ✅ **Structural vs nominal typing** (MEDIUM impact)
  - Nominal typing: Explicit type relationships
  - Structural typing: Duck typing with compile-time checks

**Design Rule Candidate**: **DR-SEM-002** - Strong Static Typing with Minimal Inference

**Research Backing**:
- P06: Type-constrained generation improves semantic correctness
- D1: Rust (strong typing) 25% type errors vs Python (dynamic) 38%

---

### EP-SEM-003: Incorrect Function Arguments

**Error Pattern**: Wrong number, order, or types of parameters (14% frequency)

**Trigger Features**:
- Many parameters (>4-5)
- Positional-only parameters
- Optional parameters without defaults
- Variadic functions

**Prevention Features**:
- ✅ **Limited function parameters** (HIGH impact)
  - Maximum 3-4 positional parameters
  - Use structs/records for >4 parameters

- ✅ **Named arguments** (MEDIUM impact)
  - Require names for optional parameters
  - Explicit parameter passing: `foo(x=1, y=2)`

**Design Rule Candidate**: **DR-SEM-003** - Limited Positional Parameters (<4)

**Research Backing**:
- Clean Code: 2-3 parameters ideal, >3 requires refactoring
- D1: Parameter count correlates with EP-SEM-003 frequency

---

### EP-SEM-004: Missing Return Statement

**Error Pattern**: Function doesn't return on all paths (12% frequency)

**Trigger Features**:
- Implicit returns allowed
- Conditional returns without else branch
- No compiler enforcement of return completeness

**Prevention Features**:
- ✅ **Compiler-enforced returns** (HIGH impact - eliminates 100%)
  - Every code path must return or panic
  - Type system tracks control flow

- ✅ **Expression-based returns** (MEDIUM impact)
  - Last expression is implicit return (Rust-style)
  - Forces awareness of return value

**Design Rule Candidate**: **DR-SEM-004** - Exhaustive Return Checking

**Research Backing**:
- Rust compiler: `missing return` error caught at compile time
- D1: Python allows no-return, Rust prevents it (0% occurrence)

---

## Logic Error Patterns (4)

### EP-LOG-001: Boundary Condition Error

**Error Pattern**: Off-by-one, empty collection handling (22% frequency - highest!)

**Trigger Features**:
- Manual loop bounds (`for i in range(len(arr))`)
- No first-class boundary handling
- Silent failure on empty collections
- Unchecked assumptions about collection size

**Prevention Features**:
- ✅ **Iterator-first design** (HIGH impact)
  - `for item in collection` (not `for i in 0..len`)
  - Iterators handle boundaries automatically

- ✅ **Explicit boundary checks** (HIGH impact)
  - `collection.first()` returns `Option<T>` (Rust-style)
  - Compiler forces handling of empty case

- ✅ **Array slicing with safe defaults** (MEDIUM impact)
  - `arr[0..5]` clamps to valid range (doesn't panic)

**Design Rule Candidate**: **DR-ERR-001** - Iterator-First with Explicit Boundaries

**Research Backing**:
- P01, P02: Boundary conditions are #1 logic error across all LLMs
- Rust: Iterator-based loops eliminate 60% of boundary errors

---

### EP-LOG-002: Wrong Algorithm Choice

**Error Pattern**: Using inefficient/incorrect algorithm (20% frequency)

**Trigger Features**:
- No standard library guidance
- Multiple ways to solve same problem
- Performance characteristics unclear
- Missing common patterns in stdlib

**Prevention Features**:
- ✅ **Rich standard library** (MEDIUM impact)
  - Provide canonical implementations (sort, search, common algorithms)
  - Document complexity guarantees

- ✅ **Type hints for algorithm selection** (LOW impact)
  - `sorted(O(n log n))` vs `approximate_sort(O(n))`
  - Make trade-offs explicit

**Design Rule Candidate**: **DR-TOK-003** - Rich Standard Library with Complexity Hints

**Research Backing**:
- P11: LLM agents improve with more stdlib documentation
- D1: Generic implementations reduce algorithm errors by 15%

---

### EP-LOG-003: Incorrect Conditional Logic

**Error Pattern**: Wrong boolean expressions, missing cases (18% frequency)

**Trigger Features**:
- Complex boolean expressions (`&&`, `||`, `!` mixed)
- Implicit truthiness/falsiness
- Missing exhaustive pattern matching
- Operator precedence ambiguity

**Prevention Features**:
- ✅ **Simplified boolean operators** (HIGH impact)
  - `and`, `or`, `not` (keywords, not symbols)
  - Required parentheses for precedence

- ✅ **Exhaustive pattern matching** (HIGH impact)
  - `match` requires handling all cases
  - Compiler error for missing branches

- ✅ **No implicit truthiness** (MEDIUM impact)
  - Only `true`/`false` are boolean (not `0`, `""`, `null`)

**Design Rule Candidate**: **DR-LOG-001** - Explicit Boolean Logic with Pattern Matching

**Research Backing**:
- Rust: Exhaustive `match` prevents logic errors
- P08: Implicit truthiness causes 12% of condition errors

---

### EP-LOG-004: Infinite Loop

**Error Pattern**: Loop never terminates (15% frequency)

**Trigger Features**:
- `while true` with missing break condition
- Loop variable not modified
- No timeout/iteration limits
- Complex termination logic

**Prevention Features**:
- ✅ **Loop invariant hints** (LOW impact)
  - Compiler warnings for suspicious loops
  - Detect non-modified loop variables

- ✅ **Prefer bounded iteration** (MEDIUM impact)
  - `for item in collection` over `while true`
  - Encourage finite iterations

**Design Rule Candidate**: **DR-ERR-002** - Prefer Bounded Iteration

**Research Backing**:
- P09: Infinite loops often result from unbounded `while` constructs
- Iterator pattern eliminates 40% of loop termination errors

---

## Cross-Cutting Features

### Language-Wide Design Principles

Based on error pattern analysis, these features apply across multiple error categories:

#### 1. **Grammar Constraints** (DR-GRM-001 through DR-GRM-004)

**Preventing**: EP-SYN-001, EP-SYN-004, EP-SEM-003

**Features**:
- Context-free grammar with minimal ambiguity
- SynCode-compatible (CFG-constrained LLM decoding)
- Predictable token sequences
- Limited nesting depth

**Impact**: 96% syntax error reduction (P04)

---

#### 2. **Type System Strength** (DR-SEM-002, DR-SEM-004)

**Preventing**: EP-SEM-002, EP-SEM-004, EP-LOG-001

**Features**:
- Static typing with inference (local only)
- Option/Result types for fallibility
- No null pointers
- Exhaustiveness checking

**Impact**: 60-70% semantic error reduction (D1)

---

#### 3. **Token Efficiency** (DR-TOK-001 through DR-TOK-003)

**Preventing**: All categories (improves LLM context usage)

**Features**:
- Symbols over keywords where clear (`->` vs `function`)
- Minimal boilerplate
- Concise common patterns
- 30-40% fewer tokens than Python/TypeScript

**Impact**: Reduces context window pressure, improves completion accuracy

**Research Backing**: GlyphLang achieves 35% token reduction

---

## Summary: Error Pattern → Design Rule Mapping

| Error Pattern | Frequency | Design Rule(s) | Impact | Category |
|---------------|-----------|----------------|--------|----------|
| EP-SYN-001 | 12% | DR-SYN-001, DR-GRM-002 | HIGH | Syntax |
| EP-SYN-002 | 8% (20% Python) | DR-SYN-002 | HIGH | Syntax |
| EP-SYN-003 | 15% | DR-SYN-003, DR-ERR-001 | HIGH | Syntax |
| EP-SYN-004 | 10% | DR-SYN-004 | HIGH | Syntax |
| EP-SEM-001 | 18% | DR-SEM-001 | HIGH | Semantic |
| EP-SEM-002 | 16% | DR-SEM-002 | HIGH | Semantic |
| EP-SEM-003 | 14% | DR-SEM-003 | HIGH | Semantic |
| EP-SEM-004 | 12% | DR-SEM-004 | HIGH | Semantic |
| EP-LOG-001 | 22% | DR-ERR-001, DR-LOG-002 | HIGH | Logic |
| EP-LOG-002 | 20% | DR-TOK-003 | MEDIUM | Logic |
| EP-LOG-003 | 18% | DR-LOG-001 | HIGH | Logic |
| EP-LOG-004 | 15% | DR-ERR-002 | MEDIUM | Logic |

**Total Rules Identified**: 20 core rules (DR-001 through DR-020)

---

## Next Steps (T041-T045)

Create individual design rule files organized by category:

### Syntax Rules (5 rules)
- DR-SYN-001: Explicit Delimiters with Nesting Limits
- DR-SYN-002: No Significant Whitespace
- DR-SYN-003: Bounds-Checked Indexing
- DR-SYN-004: Explicit Operator Distinction
- DR-SYN-005: Consistent Statement Terminators

### Grammar Rules (4 rules)
- DR-GRM-001: Context-Free Grammar
- DR-GRM-002: SynCode-Compatible Structure
- DR-GRM-003: Predictable Token Sequences
- DR-GRM-004: Limited Ambiguity

### Semantic Rules (4 rules)
- DR-SEM-001: Mandatory Variable Declarations
- DR-SEM-002: Strong Static Typing
- DR-SEM-003: Limited Positional Parameters
- DR-SEM-004: Exhaustive Return Checking

### Token Efficiency Rules (3 rules)
- DR-TOK-001: Symbols Over Keywords
- DR-TOK-002: Minimal Boilerplate
- DR-TOK-003: Rich Standard Library

### Error Prevention Rules (4 rules)
- DR-ERR-001: Iterator-First Design
- DR-ERR-002: Prefer Bounded Iteration
- DR-ERR-003: Explicit Null Handling (Option Types)
- DR-ERR-004: Compiler-Enforced Completeness

---

## Validation Plan

Each design rule will be validated via:
1. **Benchmark testing** (D4): Test variants with/without rule
2. **Metric correlation** (D3): Measure impact on LLM-friendliness scores
3. **Cross-language comparison** (D1): Compare existing languages implementing similar rules
4. **Research citations** (literature): Link to empirical studies

---

**Status**: T040 COMPLETE
**Next Task**: T041 - Draft syntax rules (DR-SYN-001 through DR-SYN-005)
