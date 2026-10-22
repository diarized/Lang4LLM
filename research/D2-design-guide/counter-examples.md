# Design Rule Counter-Examples (T048)

**Purpose**: Document scenarios where design rules should NOT be applied
**Date**: 2026-01-20
**Context**: Every rule has exceptions - this document catalogs them

---

## What Are Counter-Examples?

Counter-examples are legitimate scenarios where applying a design rule would:
1. **Harm usability** - Make the language unusable for specific domains
2. **Conflict with other goals** - Trade-off between competing principles
3. **Introduce worse problems** - The cure is worse than the disease

For each counter-example, we provide:
- **Scenario**: When the rule doesn't apply
- **Reason**: Why the rule fails here
- **Alternative**: How to handle this case

---

## Syntax Rules

### DR-SYN-001: Explicit Delimiters with Nesting Limits

**Counter-Example 1: Functional Composition**
- **Scenario**: Highly compositional functional code (map/filter/reduce chains)
- **Reason**: Nested function calls naturally exceed 3 levels
- **Alternative**: Allow deeper nesting for function composition specifically, enforce elsewhere
- **Example**:
  ```
  // Exceeds 3 levels but is idiomatic functional code
  data.filter(|x| x > 0)
      .map(|x| x * 2)
      .reduce(|a, b| a + b)
      .unwrap_or(0)
  ```

**Counter-Example 2: Mathematical Expressions**
- **Scenario**: Algebraic formulas requiring deep nesting
- **Reason**: Mathematical notation naturally has arbitrary nesting depth
- **Alternative**: Separate expression grammar from statement grammar with different limits
- **Example**: `((a + b) * (c - d)) / ((e * f) + (g / h))`

---

### DR-SYN-002: No Significant Whitespace

**Counter-Example 1: Beginner Readability**
- **Scenario**: Teaching programming to beginners
- **Reason**: Python's indentation is praised for forcing readable structure
- **Alternative**: Enforce formatting via auto-formatter, not parser
- **Resolution**: Use braces but provide strong formatter (like `rustfmt`, `gofmt`)

**Counter-Example 2: Minimal Syntax Goals**
- **Scenario**: Minimalist language design (fewer characters)
- **Reason**: Braces add visual noise and token overhead
- **Alternative**: Use lightweight delimiters (`begin/end` keywords, or require formatting)
- **Resolution**: Trade-off accepted - explicit delimiters prioritized over brevity

---

### DR-SYN-003: Bounds-Checked Indexing

**Counter-Example 1: Performance-Critical Code**
- **Scenario**: High-performance numerical computing, game engines
- **Reason**: Bounds checking adds 5-10% runtime overhead
- **Alternative**: Provide `unsafe` blocks for verified unchecked access
- **Example**:
  ```
  unsafe {
      let value = array[index]  // unchecked, developer responsibility
  }
  ```

**Counter-Example 2: Systems Programming**
- **Scenario**: Low-level memory manipulation, kernel code
- **Reason**: Direct memory access requires unchecked indexing
- **Alternative**: Separate `unsafe` mode with explicit opt-in

---

### DR-SYN-004: Explicit Operator Distinction

**Counter-Example 1: Developer Familiarity**
- **Scenario**: Migration from C/Java/JavaScript ecosystems
- **Reason**: `:=` is unfamiliar, `=` is universal across languages
- **Alternative**: Compiler error for assignment in conditionals (Swift-style)
- **Resolution**: Familiarity vs safety - prioritize safety, provide migration tools

**Counter-Example 2: Token Minimalism**
- **Scenario**: Reducing total token count
- **Reason**: Reusing `=` for assignment/comparison saves tokens
- **Alternative**: Distinct tokens improve clarity more than brevity
- **Resolution**: Clarity prioritized over minimal token count

---

### DR-SYN-005: Consistent Statement Terminators

**Counter-Example 1: REPL-Friendly Syntax**
- **Scenario**: Interactive programming, exploratory coding
- **Reason**: Requiring semicolons in REPL is tedious
- **Alternative**: Different rules for REPL vs source files (adds complexity)
- **Resolution**: Newline termination with explicit continuation (Go-style)

**Counter-Example 2: Scripting Language Brevity**
- **Scenario**: Quick one-liners and shell scripts
- **Reason**: Semicolons add visual noise for simple scripts
- **Alternative**: Newline as terminator, backslash continuation
- **Resolution**: Accept newline termination with clear continuation rules

---

## Grammar Rules

### DR-GRM-001: Context-Free Grammar

**Counter-Example 1: Significant Indentation**
- **Scenario**: Python-style whitespace-dependent syntax
- **Reason**: Indentation is context-sensitive (not CFG-compatible)
- **Alternative**: Avoid significant whitespace entirely (DR-SYN-002)
- **Resolution**: Rule conflict - DR-SYN-002 resolves this

**Counter-Example 2: Type-Dependent Parsing**
- **Scenario**: C++ template syntax parsing
- **Reason**: Full type resolution requires context-sensitive grammar
- **Alternative**: Separate type checking from parsing, keep grammar CFG
- **Resolution**: Parse as CFG, resolve types in later compiler phase

---

### DR-GRM-002: SynCode-Compatible Grammar

**Counter-Example: Experimental Language Features**
- **Scenario**: Novel syntax not yet supported by SynCode
- **Reason**: SynCode may not have grammar definition for new constructs
- **Alternative**: Extend SynCode grammar, or defer experimental features
- **Resolution**: Prioritize compatibility, contribute grammar extensions upstream

---

### DR-GRM-003: Predictable Token Sequences

**Counter-Example: Operator Precedence**
- **Scenario**: Complex mathematical expressions with precedence
- **Reason**: Precedence parsing requires some lookahead
- **Alternative**: Use precedence climbing algorithm (acceptable complexity)
- **Resolution**: Minimal lookahead acceptable for well-understood patterns

---

### DR-GRM-004: Minimal Grammar Ambiguity

**Counter-Example: Dangling Else Problem**
- **Scenario**: Classic `if/else` ambiguity
- **Reason**: Grammatically ambiguous which `else` matches which `if`
- **Alternative**: Require braces or `end` keyword (eliminates ambiguity)
- **Resolution**: Enforce explicit delimiters to resolve ambiguity

---

## Semantic Rules

### DR-SEM-001: Mandatory Variable Declarations

**Counter-Example: REPL/Scripting Convenience**
- **Scenario**: Quick interactive experimentation
- **Reason**: Explicit declarations are verbose for throwaway code
- **Alternative**: Different mode for REPL (adds complexity)
- **Resolution**: Uniform rules - brief syntax (`let x = 5`) acceptable overhead

---

### DR-SEM-002: Strong Static Typing

**Counter-Example 1: Rapid Prototyping**
- **Scenario**: Early-stage development with changing requirements
- **Reason**: Type annotations slow iteration when types are in flux
- **Alternative**: Gradual typing - allow `Any` type during prototyping
- **Example**:
  ```
  fn prototype(data: Any) -> Any {  // relaxed for prototyping
      // ... refine types later
  }
  ```

**Counter-Example 2: Dynamic Dispatch**
- **Scenario**: Plugin systems, runtime polymorphism
- **Reason**: Static types limit runtime flexibility
- **Alternative**: Provide trait objects/interfaces for polymorphism
- **Resolution**: Type system features (traits) handle most dynamic needs

---

### DR-SEM-003: Limited Positional Parameters

**Counter-Example 1: Mathematical Functions**
- **Scenario**: Matrix operations, geometric transformations
- **Reason**: Operations like `multiply(a, b, c, d, e, f)` are natural for matrices
- **Alternative**: Use dedicated matrix types to encapsulate parameters
- **Example**: `Matrix::multiply(transform_matrix)` instead of 6 parameters

**Counter-Example 2: Builder Patterns**
- **Scenario**: Configuration-heavy APIs
- **Reason**: Many configuration options require many parameters
- **Alternative**: Builder object or configuration struct
- **Resolution**: Encourage builder pattern for >4 parameters

---

### DR-SEM-004: Exhaustive Return Checking

**Counter-Example 1: Void/Unit Functions**
- **Scenario**: Functions with side effects only (print, log, update state)
- **Reason**: No meaningful return value
- **Alternative**: Explicit `()` (unit type) return, still checked
- **Resolution**: Unit type makes "no return" explicit

**Counter-Example 2: Diverging Functions**
- **Scenario**: Functions that never return (infinite loops, panic)
- **Reason**: Control flow never returns to caller
- **Alternative**: Explicit `-> !` (never type) annotation
- **Example**:
  ```
  fn serve_forever() -> ! {
      loop {
          handle_request()
      }
  }
  ```

---

## Token Efficiency Rules

### DR-TOK-001: Symbols Over Keywords

**Counter-Example 1: Screen Reader Accessibility**
- **Scenario**: Visually impaired developers using screen readers
- **Reason**: Symbolic operators harder to vocalize than keywords
- **Alternative**: Provide verbal aliases in tooling/IDE, not syntax
- **Resolution**: Tooling solution - screen readers can map symbols to words

**Counter-Example 2: Non-ASCII Challenges**
- **Scenario**: Developers on limited keyboards or systems
- **Reason**: Some symbols (especially Unicode) require special input methods
- **Alternative**: Stick to ASCII-only operators
- **Resolution**: All operators must be ASCII-typable

**Counter-Example 3: Beginner Readability**
- **Scenario**: First-time programmers
- **Reason**: `function` is more explicit than `->`
- **Alternative**: Provide both syntaxes (increases complexity)
- **Resolution**: Short learning curve acceptable for long-term benefits

---

### DR-TOK-002: Minimal Boilerplate

**Counter-Example 1: Explicit Program Structure**
- **Scenario**: Large codebases requiring clear entry points
- **Reason**: Java-style `main()` provides unambiguous program start
- **Alternative**: Top-level statements for scripts, explicit entry for libraries
- **Resolution**: Context-dependent: scripts use top-level, libraries use `main()`

**Counter-Example 2: Import Clarity**
- **Scenario**: Dependency tracking and code organization
- **Reason**: Explicit imports make dependencies visible
- **Alternative**: Keep explicit imports but simplify syntax
- **Resolution**: Imports are necessary boilerplate - minimize syntax only

---

### DR-TOK-003: Rich Standard Library

**Counter-Example 1: Minimal Language Core**
- **Scenario**: Embedded systems, educational minimal languages
- **Reason**: Large stdlib increases binary size and learning curve
- **Alternative**: Modular stdlib - import only what you need
- **Resolution**: Standard library is modular, core is minimal

**Counter-Example 2: Teaching Algorithms**
- **Scenario**: Computer science education
- **Reason**: Implementing algorithms teaches fundamental concepts
- **Alternative**: Provide both stdlib and tutorial implementations
- **Resolution**: Stdlib for production, exercises for learning

---

## Error Prevention Rules

### DR-ERR-001: Iterator-First Design

**Counter-Example 1: Random Access Algorithms**
- **Scenario**: Binary search, quicksort, indexing operations
- **Reason**: Require direct indexing, not sequential iteration
- **Alternative**: Provide safe indexing via `get(i) -> Option<T>`
- **Resolution**: Iterators for sequential, safe indexing for random access

**Counter-Example 2: Parallel Iteration**
- **Scenario**: Zip iteration over multiple collections with indices
- **Reason**: Need synchronized index access across collections
- **Alternative**: `enumerate()` and `zip()` iterators
- **Resolution**: Iterator combinators handle most cases

---

### DR-ERR-002: Prefer Bounded Iteration

**Counter-Example 1: Event Loops**
- **Scenario**: Servers, GUI applications, request handlers
- **Reason**: Intentionally infinite loops for event processing
- **Alternative**: Explicit `loop { ... }` keyword, suppress warning with annotation
- **Example**:
  ```
  #[allow(infinite_loop)]
  loop {
      let request = accept_connection()
      handle(request)
  }
  ```

**Counter-Example 2: Convergence Algorithms**
- **Scenario**: Iterative numerical methods (gradient descent, Newton's method)
- **Reason**: Unknown iteration count until convergence criteria met
- **Alternative**: `iter().take_while(|state| !converged(state))`
- **Resolution**: Bounded via `take_while` predicate

---

### DR-ERR-003: Explicit Null Handling

**Counter-Example 1: FFI with Null-Using Languages**
- **Scenario**: Interop with C, Java APIs that use null
- **Reason**: External APIs may return null pointers
- **Alternative**: Explicit conversion at boundary: `from_nullable(ptr) -> Option<T>`
- **Resolution**: Null allowed only at FFI boundary, convert immediately

**Counter-Example 2: Performance Overhead**
- **Scenario**: Memory-constrained environments
- **Reason**: `Option<T>` adds tag byte overhead
- **Alternative**: Compiler optimizes `Option<&T>` to pointer (null optimization)
- **Resolution**: Zero-cost abstraction for references

---

### DR-ERR-004: Compiler-Enforced Completeness

**Counter-Example 1: Catch-All Default Handling**
- **Scenario**: Partial patterns with default fallback
- **Reason**: Default case catches all remaining patterns
- **Alternative**: Require explicit wildcard `_` pattern, warn if too broad
- **Resolution**: Wildcard allowed but compiler warns on over-broad matches

**Counter-Example 2: Open/Extensible Enums**
- **Scenario**: Plugin systems with runtime-extensible types
- **Reason**: Can't check exhaustiveness for dynamically extended types
- **Alternative**: Closed enums by default, open only with `#[non_exhaustive]`
- **Resolution**: Explicit opt-out for extensibility, exhaustive by default

---

## Summary: When Rules Don't Apply

| Rule | Most Common Counter-Example | Recommended Resolution |
|------|------------------------------|------------------------|
| DR-SYN-001 | Functional composition | Allow deeper nesting for specific constructs |
| DR-SYN-002 | Beginner readability | Strong auto-formatter instead |
| DR-SYN-003 | Performance-critical code | `unsafe` blocks for verified code |
| DR-SYN-004 | Developer familiarity | Prioritize safety over familiarity |
| DR-SYN-005 | REPL convenience | Newline termination with explicit continuation |
| DR-GRM-001 | Type-dependent parsing | Separate parsing from type checking |
| DR-GRM-002 | Experimental features | Extend SynCode or defer features |
| DR-GRM-003 | Operator precedence | Acceptable complexity for precedence |
| DR-GRM-004 | Dangling else | Require explicit delimiters |
| DR-SEM-001 | REPL scripting | Uniform rules, minimal syntax |
| DR-SEM-002 | Rapid prototyping | Gradual typing with `Any` escape hatch |
| DR-SEM-003 | Mathematical functions | Use dedicated types (Matrix, Point) |
| DR-SEM-004 | Diverging functions | Explicit `-> !` never type |
| DR-TOK-001 | Screen readers | Tooling solution, not syntax |
| DR-TOK-002 | Explicit structure | Context-dependent (scripts vs libraries) |
| DR-TOK-003 | Minimal core | Modular stdlib |
| DR-ERR-001 | Random access | Provide safe indexing primitives |
| DR-ERR-002 | Event loops | Explicit `loop` keyword with annotation |
| DR-ERR-003 | FFI boundaries | Convert at boundary, Option internally |
| DR-ERR-004 | Extensible enums | Explicit `#[non_exhaustive]` opt-out |

---

## Design Philosophy

**Not all rules apply everywhere**. Language design requires balancing:
1. **Safety** vs **Performance** (bounds checking, type safety)
2. **Explicitness** vs **Brevity** (symbols vs keywords)
3. **Strictness** vs **Flexibility** (exhaustiveness, typing)
4. **Simplicity** vs **Power** (minimal syntax vs expressiveness)

**Resolution Strategy**:
- **Default to safety** - unsafe must be explicit opt-in
- **Provide escape hatches** - `unsafe`, `Any`, `#[attribute]` for edge cases
- **Context-aware** - different rules for scripts vs libraries, prototyping vs production
- **Tooling solutions** - formatters, linters, IDE support over syntax restrictions

---

**Status**: T048 COMPLETE
**Next**: T049 - Rank rules by impact (already done in priority fields)
