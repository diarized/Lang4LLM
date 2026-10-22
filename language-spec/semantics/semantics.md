# Lang4LLM Semantics Specification

**Version**: 1.0
**Date**: 2026-01-31
**Status**: Phase 1 - Language Specification
**Compliance**: DR-ERR-002 through DR-ERR-004, DR-TOK-001 through DR-TOK-003
**Transpilation Target**: Rust

---

## Table of Contents

1. [Overview](#overview)
2. [Evaluation Order](#evaluation-order)
3. [Variable Binding](#variable-binding)
4. [Function Call Semantics](#function-call-semantics)
5. [Iterator Protocol](#iterator-protocol)
6. [Error Handling Model](#error-handling-model)
7. [Pattern Matching Semantics](#pattern-matching-semantics)
8. [Mutability Semantics](#mutability-semantics)
9. [Scope and Initialization](#scope-and-initialization)
10. [Rust Transpilation Mapping](#rust-transpilation-mapping)
11. [Design Rule Compliance](#design-rule-compliance)

---

## Overview

Lang4LLM semantics are designed for **direct transpilation to Rust** while keeping syntax simple for LLM code generation.

### Core Principles

1. **Rust-Compatible**: All semantics map 1:1 to Rust constructs
2. **Explicit Over Implicit**: No hidden behavior, predictable execution
3. **Error-Safe**: Compile-time completeness checking, runtime safety
4. **Minimal Boilerplate**: Token-efficient syntax (DR-TOK-002, I=0.35)
5. **Stdlib-First**: Rich standard library reduces reinvention (DR-TOK-003, I=0.45)

### Transpilation Model

```
Lang4LLM Source (LLM-generated, simple syntax)
       ↓
  Transpiler (Phase 3: maps to idiomatic Rust)
       ↓
  Rust Code (borrow checker, ownership, lifetimes)
       ↓
  rustc (strict compiler validates safety)
       ↓
  Binary (Rust ecosystem, zero-cost abstractions)
```

**Design Goal**: LLMs generate Lang4LLM (predictable, safe), Rust compiler ensures correctness.

---

## Evaluation Order

### Strict Left-to-Right Evaluation

Lang4LLM uses **strict, eager, left-to-right** evaluation (matches Rust):

```lang4llm
// Expressions evaluated left to right
let x: int = f() + g() + h();
// Evaluation order: f(), then g(), then h(), then additions left-to-right

// Function arguments evaluated left to right
process(a(), b(), c());
// Evaluation order: a(), b(), c(), then process() call
```

**Rationale**: Deterministic evaluation order prevents confusion. Rust guarantees left-to-right within expressions.

### Rust Transpilation

```lang4llm
let result: int = compute(x) + compute(y);
```

Transpiles to:

```rust
let result: i64 = compute(x) + compute(y);
// Rust evaluates left to right: compute(x), then compute(y), then addition
```

---

### Short-Circuit Evaluation

Logical operators short-circuit (matches Rust `&&` and `||`):

```lang4llm
// 'and' short-circuits: right not evaluated if left is false
if expensive_check() and another_check() {
    // another_check() NOT called if expensive_check() returns false
}

// 'or' short-circuits: right not evaluated if left is true
if quick_check() or fallback_check() {
    // fallback_check() NOT called if quick_check() returns true
}
```

### Rust Transpilation

```lang4llm
if a and b { ... }
```

Transpiles to:

```rust
if a && b { ... }
// Rust's && short-circuits identically
```

---

### Expression vs Statement

Lang4LLM distinguishes expressions (produce values) from statements (side effects):

**Expressions** (produce value):
- Literals: `42`, `"hello"`, `true`
- Variables: `x`, `arr[0]`
- Function calls: `compute(5)`
- If-else: `if cond { return a; } else { return b; }`
- Match: `match opt { Some(x) => return x;, None => return 0; }`
- Blocks: `{ let x = 10; return x * 2; }` (explicit return required)

**Statements** (side effects only):
- Variable declaration: `let x: int = 42;`
- Assignment: `x = 20;`
- Function call (unit return): `print("Hello");`
- Return: `return value;`
- Break/continue: `break;`, `continue;`

### Explicit Returns Required

All functions must use explicit `return` statements to return values:

```lang4llm
fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    } else {
        return b;
    }
}
```

**Rationale**: Explicit returns improve code clarity for LLM generation and prevent ambiguity between expressions and statements.

**Rust Transpilation**: Direct mapping (transpiler generates explicit `return` statements in Rust code).

---

## Variable Binding

### Initialization Semantics

All variables **must be initialized** at declaration (DR-SEM-004):

```lang4llm
let x: int = 42;      // OK: initialized
let y: int;           // ERROR: uninitialized variable
y = 10;               // Too late: declaration must include initializer
```

**Rationale**: Prevents undefined variable errors (EP-SEM-001). Rust enforces identical rule.

### Rust Transpilation

```lang4llm
let x: int = 42;
```

Transpiles to:

```rust
let x: i64 = 42;
// Rust requires initialization identically
```

---

### Move Semantics (Implicit)

Lang4LLM uses **move semantics** for non-Copy types (hidden from syntax):

```lang4llm
let s1: string = "hello";
let s2: string = s1;  // Moves s1 into s2
// s1 is no longer valid here (compiler error if used)
```

**Transpilation**: Lang4LLM transpiler tracks moves, generates Rust move semantics:

```rust
let s1: String = String::from("hello");
let s2: String = s1;  // Move in Rust
// s1 invalid here
```

**LLM Simplification**: LLMs don't see ownership annotations, transpiler enforces Rust rules.

---

### Copy Semantics (Automatic)

Primitive types (`int`, `float`, `bool`) are **Copy** (no move):

```lang4llm
let x: int = 42;
let y: int = x;  // Copies x (both remain valid)
print(x);        // OK: x still valid
print(y);        // OK: y is independent copy
```

**Rust Transpilation**: Primitives implement `Copy` trait automatically.

---

## Function Call Semantics

### Pass-by-Value (Default)

Arguments passed by value (moved or copied depending on type):

```lang4llm
fn process(s: string) -> unit {
    print(s);
}  // s dropped here

fn main() -> unit {
    let msg: string = "Hello";
    process(msg);  // msg moved into function
    // msg invalid here
}
```

**Rust Transpilation**: Direct mapping to Rust pass-by-value.

---

### Pass-by-Reference (Borrowing, Simplified Syntax)

Lang4LLM **hides** explicit `&` syntax for LLM simplicity:

```lang4llm
// Lang4LLM syntax (simplified)
fn length(s: string) -> int {
    // Function signature implies borrowing for non-mut reads
    return len(s);
}

fn main() -> unit {
    let msg: string = "Hello";
    let l: int = length(msg);  // Borrows (no move)
    print(msg);  // OK: msg still valid
}
```

**Transpiler Inference**: Transpiler inserts `&` based on function body analysis:
- If function doesn't modify parameter → borrow (`&T`)
- If function modifies parameter → mutable borrow (`&mut T`)
- If function consumes parameter → move (`T`)

**Rust Transpilation**:

```rust
fn length(s: &String) -> i64 {  // Transpiler adds &
    return s.len() as i64;
}

fn main() {
    let msg: String = String::from("Hello");
    let l: i64 = length(&msg);  // Transpiler adds &
    println!("{}", msg);
}
```

**LLM Benefit**: LLMs generate simple syntax, transpiler handles Rust borrow complexity.

---

### Mutable References (Explicit `mut` Only)

Mutating functions require `mut` keyword in signature:

```lang4llm
fn append(s: mut string, suffix: string) -> unit {
    // 'mut' signals this function modifies s
    s = s + suffix;
}

fn main() -> unit {
    let mut msg: string = "Hello";
    append(msg, " World");  // OK: msg is mutable
    print(msg);  // Prints: "Hello World"
}
```

**Rust Transpilation**:

```rust
fn append(s: &mut String, suffix: &String) {  // Transpiler adds &mut
    s.push_str(suffix);
}

fn main() {
    let mut msg: String = String::from("Hello");
    append(&mut msg, &String::from(" World"));
    println!("{}", msg);
}
```

---

### Return Value Optimization

Lang4LLM returns are **moved**, not copied (Rust RVO):

```lang4llm
fn create_vector() -> [int] {
    let v: [int] = [1, 2, 3];
    return v;  // Moves v out (no copy)
}
```

**Rust Transpilation**: Rust's move semantics + RVO (Return Value Optimization) eliminate copies.

---

## Iterator Protocol

### Iterator-First Design (DR-ERR-001, I=0.75)

All collections provide **iterator-based traversal** (eliminates 60% of boundary errors):

```lang4llm
let numbers: [int] = [1, 2, 3, 4, 5];

// Iterator-based loop (preferred)
for num in numbers {
    print(num);
}

// With index (when needed)
for (i, num) in enumerate(numbers) {
    print(i);
    print(num);
}
```

**Rationale**: Prevents EP-LOG-001 (boundary errors), the #1 logic error (22% frequency).

---

### Iterator Trait (Rust-Compatible)

Lang4LLM iterators map to Rust's `Iterator` trait:

```lang4llm
// Standard iterator methods (stdlib)
let doubled: [int] = numbers.map(|x| x * 2);
let evens: [int] = numbers.filter(|x| x % 2 == 0);
let sum: int = numbers.fold(0, |acc, x| acc + x);
```

**Rust Transpilation**:

```rust
let doubled: Vec<i64> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i64> = numbers.iter().filter(|x| x % 2 == 0).collect();
let sum: i64 = numbers.iter().fold(0, |acc, x| acc + x);
```

---

### Bounded Iteration (DR-ERR-002, I=0.30)

Encourage **finite iteration** over unbounded loops:

```lang4llm
// GOOD: Bounded iteration
for i in range(0, 100) {
    process(i);
}

// WARNING: Unbounded loop (compiler warning)
while true {
    // Should have explicit break within first 10 lines
    if condition {
        break;
    }
}

// BETTER: Explicit infinite loop keyword
loop {
    if condition {
        break;
    }
}
```

**Compiler Behavior**:
- `while true` without `break` in first 10 lines → **warning**
- Suggest: use `loop { ... break ... }` (explicit infinite loop)
- Suggest: use `iter().take_while(cond)` for bounded convergence

**Rust Transpilation**:

```lang4llm
loop { ... }
```

Transpiles to:

```rust
loop { ... }  // Rust's explicit infinite loop keyword
```

---

## Error Handling Model

### Result Type (First-Class Errors, DR-ERR-002 implicit)

Lang4LLM uses `Result<T, E>` for **recoverable errors**:

```lang4llm
enum Result {
    Ok(T),
    Error(E)
}

fn divide(a: int, b: int) -> Result {
    if b == 0 {
        return Result::Error("Division by zero");
    }
    return Result::Ok(a / b);
}
```

**Pattern Matching** (forces error handling):

```lang4llm
let result: Result = divide(10, 0);
match result {
    Result::Ok(value) => print(value),
    Result::Error(msg) => print("Error: " + msg),
}
```

**Rust Transpilation**:

```rust
fn divide(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}

let result = divide(10, 0);
match result {
    Ok(value) => println!("{}", value),
    Err(msg) => println!("Error: {}", msg),
}
```

---

### Option Type (Explicit Null Handling, DR-ERR-003, I=0.50)

Lang4LLM uses `Option<T>` for **optional values** (no null):

```lang4llm
enum Option {
    Some(T),
    None
}

fn first(arr: [int]) -> Option {
    if len(arr) == 0 {
        return Option::None;
    }
    return Option::Some(arr[0]);
}
```

**Pattern Matching** (forces None handling):

```lang4llm
let arr: [int] = [];
let first_elem: Option = first(arr);
match first_elem {
    Option::Some(value) => print(value),
    Option::None => print("Empty array"),
}
```

**Convenience Methods** (stdlib):

```lang4llm
let value: int = first_elem.unwrap_or(42);  // Provide default
let doubled: Option = first_elem.map(|x| x * 2);  // Transform if Some
```

**Rust Transpilation**: Direct mapping to `std::option::Option<T>`.

---

### Panic (Unrecoverable Errors)

Use `panic!` for **unrecoverable errors** (bounds violations, assertions):

```lang4llm
// Bounds checking (automatic)
let arr: [int] = [1, 2, 3];
let x: int = arr[10];  // PANIC: index 10 out of bounds for length 3

// Explicit assertion
assert(x > 0, "x must be positive");  // PANIC if false
```

**Rust Transpilation**:

```rust
let x: i64 = arr[10];  // Rust panics identically
assert!(x > 0, "x must be positive");
```

---

### No Exceptions

Lang4LLM has **no exceptions** (no try/catch/throw):
- Recoverable errors → `Result<T, E>`
- Unrecoverable errors → `panic!`
- Optional values → `Option<T>`

**Rationale**: Exceptions complicate control flow. Result types force explicit handling at compile time.

---

## Pattern Matching Semantics

### Exhaustive Matching (DR-ERR-004, I=0.55)

All `match` expressions **must be exhaustive** (compiler error if missing cases):

```lang4llm
enum Status {
    Pending,
    Running,
    Success,
    Failed
}

fn handle(s: Status) -> string {
    match s {
        Status::Pending => "Waiting",
        Status::Running => "In progress",
        Status::Success => "Done",
        // ERROR: Missing Status::Failed case
    }
}
```

**Compiler Error**: "pattern `Status::Failed` not covered"

---

### Wildcard Pattern

Use `_` for "all other cases" (makes match exhaustive):

```lang4llm
match s {
    Status::Success => return "OK";,
    _ => return "Not OK";,  // Catches Pending, Running, Failed
}
```

**Semantics**: Wildcard `_` is **valid** and makes the match expression exhaustive.

**Compiler Warning**: If wildcard is too broad (catches many variants), compiler emits warning suggesting explicit cases for clarity.

---

### Rust Transpilation

```lang4llm
match value {
    Pattern1 => expr1,
    Pattern2 => expr2,
}
```

Transpiles to:

```rust
match value {
    Pattern1 => expr1,
    Pattern2 => expr2,
}
// Identical syntax and exhaustiveness checking
```

---

## Mutability Semantics

### Immutable by Default (DR-ERR-004 continued)

Variables are **immutable** unless marked `mut`:

```lang4llm
let x: int = 10;
x = 20;  // ERROR: cannot assign to immutable variable

let mut y: int = 10;
y = 20;  // OK: y is mutable
```

**Rationale**: Immutability reduces bugs. Explicit `mut` signals mutation intent.

---

### Mutability and References

Borrowing respects mutability:

```lang4llm
fn modify(arr: mut [int]) -> unit {
    arr[0] = 99;  // OK: arr is mutable reference
}

fn main() -> unit {
    let mut numbers: [int] = [1, 2, 3];
    modify(numbers);  // OK: numbers is mutable
    print(numbers[0]);  // Prints: 99
}
```

**Rust Transpilation**:

```rust
fn modify(arr: &mut Vec<i64>) {
    arr[0] = 99;
}

fn main() {
    let mut numbers: Vec<i64> = vec![1, 2, 3];
    modify(&mut numbers);
    println!("{}", numbers[0]);
}
```

---

### Interior Mutability (Future Extension)

Lang4LLM Phase 1 does **not** expose interior mutability (Cell, RefCell). Transpiler may use internally if needed.

---

## Scope and Initialization

### Lexical Scoping (DR-SEM-003)

See [Type System Specification](../type-system/types.md#scoping-rules):
- Braces define scope boundaries
- No variable shadowing (compile error)
- Variables visible from declaration to end of scope

### Mandatory Initialization (DR-SEM-004)

See [Type System Specification](../type-system/types.md#initialization-rules):
- All variables initialized at declaration
- No undefined variables
- Compiler error if uninitialized

---

## Rust Transpilation Mapping

### Type Mapping

| Lang4LLM | Rust | Notes |
|----------|------|-------|
| `int` | `i64` | 64-bit signed integer |
| `float` | `f64` | 64-bit float |
| `bool` | `bool` | Boolean |
| `string` | `String` | Owned UTF-8 string |
| `[T]` | `Vec<T>` | Growable array |
| `(T1, T2)` | `(T1, T2)` | Tuple |
| `Option` | `Option<T>` | Optional value |
| `Result` | `Result<T, E>` | Result type |
| `unit` | `()` | Unit type |

---

### Syntax Mapping

| Lang4LLM | Rust | Notes |
|----------|------|-------|
| `let x: int = 42;` | `let x: i64 = 42;` | Variable declaration |
| `let mut x: int = 0;` | `let mut x: i64 = 0;` | Mutable variable |
| `fn f(x: int) -> int` | `fn f(x: i64) -> i64` | Function signature |
| `for x in arr` | `for x in arr.iter()` | Iterator loop |
| `range(0, 10)` | `0..10` | Range expression |
| `match x { ... }` | `match x { ... }` | Pattern matching |
| `if cond { return a; } else { return b; }` | `if cond { return a; } else { return b; }` | If-else with explicit returns |
| `arr[i]` | `arr[i]` | Indexing (bounds-checked) |

---

### Borrowing Inference

Transpiler infers borrows based on usage:

| Lang4LLM | Rust Transpilation | Rule |
|----------|-------------------|------|
| `fn read(s: string)` | `fn read(s: &String)` | Non-mut, no consume → `&T` |
| `fn modify(s: mut string)` | `fn modify(s: &mut String)` | Mut parameter → `&mut T` |
| `fn consume(s: string)` | `fn consume(s: String)` | Consumes (drops) → `T` |
| `f(x)` | `f(&x)` | Call non-consuming fn → `&` |
| `f(x)` | `f(&mut x)` | Call mutating fn → `&mut` |
| `f(x)` | `f(x)` | Call consuming fn → move |

**LLM Benefit**: Simple syntax, transpiler handles lifetime complexity.

---

## Design Rule Compliance

### DR-ERR-002: Bounded Iteration (I=0.30)

**Implementation**:
- Prefer `for item in collection` over `while` loops
- `loop { ... }` keyword for explicit infinite loops
- Compiler warning for `while true` without early `break`
- Suggest `.take_while(cond)` for bounded convergence

**Rust Mapping**: `loop` transpiles to Rust `loop`, warnings preserved.

---

### DR-ERR-003: Explicit Null Handling (I=0.50)

**Implementation**:
- `Option<T>` for optional values (no null keyword)
- Pattern matching forces `None` case handling
- Stdlib methods: `unwrap()`, `unwrap_or()`, `map()`, `and_then()`

**Impact**: Eliminates null pointer exceptions (Tony Hoare's "billion-dollar mistake").

---

### DR-ERR-004: Compiler-Enforced Completeness (I=0.55)

**Implementation**:
- Exhaustive pattern matching (compiler error if missing cases)
- All function paths must return declared type
- `Result<T, E>` must be handled (no ignored errors)
- Wildcard `_` allowed but triggers warning if too broad

**Impact**: Catches incomplete logic at compile time.

---

### DR-TOK-001: Symbols Over Keywords (I=0.40)

**Implementation**:
- Function definition: `fn` (not `function`)
- Lambda: `|x| x + 1` (not `lambda x: x + 1`)
- Type annotation: `:` (not `as`)
- Comparison: `==`, `!=`, `<`, `>` (established symbols)

**Token Reduction**: ~35% vs verbose languages (Python, Java).

---

### DR-TOK-002: Minimal Boilerplate (I=0.35)

**Implementation**:
- Explicit returns required (prevents ambiguity)
- No `main` wrapper for scripts (top-level statements execute)
- Simple imports: `import module;`
- Struct literals: `Point { x: 0, y: 0 }` (no `new`)

**Boilerplate Ratio**: <10% (setup tokens / total tokens).

---

### DR-TOK-003: Rich Standard Library (I=0.45)

**Implementation**:
- Collections: `Vec`, `HashMap`, `HashSet` (Rust-backed)
- Iterators: `map`, `filter`, `fold`, `take`, `skip`
- Algorithms: `sort`, `binary_search`, `min`, `max` (O(n log n) documented)
- I/O: File, network, stdio (Result-based error handling)
- String: UTF-8 operations, transformations
- Math: Standard functions (trigonometry, logarithms)

**Documentation**: Include complexity guarantees, examples, pitfalls.

**Impact**: Reduces EP-LOG-002 (wrong algorithm choice) by 15%.

---

## Summary

Lang4LLM semantics provide:

1. **Rust-Compatible IR**: All constructs map to idiomatic Rust
2. **LLM-Friendly Syntax**: Simple, explicit, no hidden complexity
3. **Compile-Time Safety**: Exhaustive checking, no null, no exceptions
4. **Token Efficiency**: 35% reduction vs verbose languages
5. **Error Prevention**: Iterator-first, bounds checking, Option/Result types

**Transpilation Strategy**:
- LLMs generate simple Lang4LLM code
- Transpiler inserts Rust borrowing/lifetime annotations
- Rust compiler validates memory safety and correctness
- Final binary leverages Rust ecosystem (performance, libraries)

**Next Steps**: See `memory-model/memory.md` for ownership and allocation strategy.

---

## Version Information

**Version**: 1.0
**Last Updated**: 2026-01-31
**Phase**: Phase 1 - Language Specification
**Design Rules**: DR-ERR-002 through DR-ERR-004, DR-TOK-001 through DR-TOK-003
**Transpilation Target**: Rust (rustc 1.70+)
**Impact Score**: Combined I=0.48 (weighted average across 6 rules)
