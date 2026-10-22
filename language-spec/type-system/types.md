# Lang4LLM Type System Specification

**Version**: 1.0
**Date**: 2026-01-26
**Status**: Phase 1 - Language Specification
**Compliance**: DR-SEM-001 through DR-SEM-004

---

## Table of Contents

1. [Overview](#overview)
2. [Primitive Types](#primitive-types)
3. [Composite Types](#composite-types)
4. [Type Declarations](#type-declarations)
5. [Type Checking](#type-checking)
6. [Type Inference](#type-inference)
7. [Type Compatibility](#type-compatibility)
8. [Traits and Polymorphism](#traits-and-polymorphism)
9. [Design Rule Compliance](#design-rule-compliance)

---

## Overview

Lang4LLM uses a **static, explicit type system** designed to maximize LLM code generation accuracy by eliminating implicit conversions and requiring explicit type annotations.

### Core Principles

1. **Explicit Types** (DR-SEM-001): No implicit type coercion
2. **Static Checking** (DR-SEM-002): All types verified at compile-time
3. **Mandatory Initialization** (DR-SEM-004): No undefined variables
4. **Lexical Scoping** (DR-SEM-003): No variable shadowing

### Type System Properties

- **Soundness**: Well-typed programs do not have runtime type errors
- **Decidability**: Type checking always terminates
- **Explicitness**: All variable types must be declared (no implicit inference beyond locals)

---

## Primitive Types

### Integer Type

**Syntax**: `int`
**Size**: 64-bit signed integer
**Range**: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
**Default**: No default value (must be initialized)

```lang4llm
let x: int = 42;
let y: int = -100;
```

**Operations**:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- No implicit conversion to/from other types

---

### Float Type

**Syntax**: `float`
**Size**: 64-bit IEEE 754 floating-point
**Range**: ±1.7976931348623157 × 10^308 (double precision)
**Default**: No default value (must be initialized)

```lang4llm
let pi: float = 3.14159;
let e: float = 2.71828;
```

**Operations**:
- Arithmetic: `+`, `-`, `*`, `/` (no modulo for floats)
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **No implicit conversion** from int (use explicit cast: `x as float`)

---

### Boolean Type

**Syntax**: `bool`
**Values**: `true`, `false`
**Default**: No default value (must be initialized)

```lang4llm
let is_valid: bool = true;
let has_error: bool = false;
```

**Operations**:
- Logical: `and`, `or`, `not`
- Comparison: `==`, `!=`
- No conversion to/from integers (no truthiness)

---

### String Type

**Syntax**: `string`
**Encoding**: UTF-8
**Mutability**: Immutable (create new strings, don't modify)
**Default**: No default value (must be initialized)

```lang4llm
let name: string = "Alice";
let message: string = "Hello, World!";
```

**Operations**:
- Concatenation: `+` operator
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=` (lexicographic)
- Indexing: `s[i]` returns single-character string (not char type, bounds-checked)

---

### Unit Type

**Syntax**: `unit`
**Value**: `none` (single value)
**Purpose**: Represents absence of meaningful value (like `void` in C, `()` in Rust)

**Note**: `none` is the unit value, distinct from `Option::None` enum variant.

```lang4llm
fn print_message(msg: string) -> unit {
    print(msg);
    return none;  // Returns unit value (not Option::None)
}
```

**Use Cases**:
- Function return type when no value returned
- Placeholder in generic types
- Explicit "nothing" value

---

## Composite Types

### Array Type

**Syntax**: `[T]` where T is any type
**Characteristics**:
- Dynamically-sized with capacity management (maps to Rust `Vec<T>`)
- Homogeneous (all elements same type)
- Bounds-checked access (runtime validation)
- Zero-indexed

```lang4llm
let numbers: [int] = [1, 2, 3, 4, 5];
let names: [string] = ["Alice", "Bob", "Charlie"];
let empty: [int] = [];  // Empty array
```

**Operations**:
- Indexing: `arr[i]` (bounds-checked, panics if out of range)
- Length: `arr.length()` → int
- Iteration: `for x in arr { ... }`
- Modification: Arrays can grow/shrink via `.push()`, `.pop()` methods (see stdlib)

**Bounds Checking** (DR-SYN-003):
```lang4llm
let arr: [int] = [1, 2, 3];
let x: int = arr[0];     // OK: 0 is in bounds
let y: int = arr[10];    // Runtime panic: index 10 out of bounds for array of length 3
```

---

### Tuple Type

**Syntax**: `(T1, T2, ..., Tn)` where Ti are types
**Characteristics**:
- Fixed-size (arity known at compile-time)
- Heterogeneous (elements can have different types)
- Immutable (create new tuples, don't modify)
- Indexed by position (zero-based)

```lang4llm
let point: (int, int) = (10, 20);
let person: (string, int, bool) = ("Alice", 30, true);
```

**Operations**:
- Field access: `tuple.0`, `tuple.1`, etc.
- Pattern matching: `let (x, y) = point;`

**Example**:
```lang4llm
fn divide(a: int, b: int) -> (int, int) {
    let quotient: int = a / b;
    let remainder: int = a % b;
    return (quotient, remainder);
}

let result: (int, int) = divide(10, 3);
let q: int = result.0;  // 3
let r: int = result.1;  // 1
```

---

### Struct Type

**Syntax**: User-defined with `struct` keyword
**Characteristics**:
- Named fields with declared types
- Nominal typing (name matters, not just structure)
- Immutable by default (fields cannot be reassigned unless marked `mut`)

**Definition**:
```lang4llm
struct Point {
    x: int,
    y: int
}
```

**Creation**:
```lang4llm
let p: Point = Point { x: 10, y: 20 };
```

**Field Access**:
```lang4llm
let x_coord: int = p.x;
let y_coord: int = p.y;
```

**Mutable Structs**:
```lang4llm
let mut p: Point = Point { x: 0, y: 0 };
p.x = 10;  // OK: p is mutable
p.y = 20;  // OK
```

---

### Enum Type

**Syntax**: User-defined with `enum` keyword
**Characteristics**:
- Tagged union (sum type)
- Each variant can carry data
- Exhaustive pattern matching required (DR-ERR-003)

**Definition**:
```lang4llm
enum Option {
    Some(int),
    None
}

enum Result {
    Ok(string),
    Err(string)
}
```

**Creation**:
```lang4llm
let value: Option = Option::Some(42);
let empty: Option = Option::None;
```

**Pattern Matching** (exhaustive):
```lang4llm
fn unwrap_or(opt: Option, default: int) -> int {
    match opt {
        Option::Some(x) => x,
        Option::None => default,
    }  // Compiler ensures all variants covered
}
```

**Compiler Error** if non-exhaustive:
```lang4llm
match opt {
    Option::Some(x) => x,
    // ERROR: Missing pattern Option::None
}
```

---

### Function Type

**Syntax**: `fn(T1, T2, ...) -> R` where Ti are parameter types, R is return type
**Characteristics**:
- First-class values (can be passed, returned, stored)
- Static types (parameter and return types known at compile-time)

**Example**:
```lang4llm
fn apply(f: fn(int) -> int, x: int) -> int {
    return f(x);
}

fn double(x: int) -> int {
    return x * 2;
}

let result: int = apply(double, 21);  // 42
```

---

## Type Declarations

### Variable Declarations

All variables **must** have explicit type annotations and initialization (DR-SEM-004):

```lang4llm
let x: int = 42;              // OK: type + value
let mut y: int = 10;          // OK: mutable variable
```

**Compiler Errors**:
```lang4llm
let x;                        // ERROR: Missing type annotation
let y: int;                   // ERROR: Missing initialization
let z = 42;                   // ERROR: Missing explicit type
```

---

### Function Declarations

All parameters and return types **must** be explicitly annotated:

```lang4llm
fn add(a: int, b: int) -> int {
    return a + b;
}
```

**Compiler Errors**:
```lang4llm
fn add(a, b) -> int {         // ERROR: Missing parameter types
    return a + b;
}

fn add(a: int, b: int) {      // ERROR: Missing return type
    return a + b;
}
```

---

## Type Checking

### Static Type Checking (DR-SEM-002)

All type errors detected at **compile-time** (before runtime):

**Type Mismatch**:
```lang4llm
let x: int = "hello";         // ERROR: Expected int, found string
```

**Function Argument Mismatch**:
```lang4llm
fn square(x: int) -> int {
    return x * x;
}

let result: int = square(3.14);  // ERROR: Expected int, found float
```

**Return Type Mismatch**:
```lang4llm
fn get_name() -> string {
    return 42;                // ERROR: Expected string, found int
}
```

---

### Type Checking Algorithm

**Type Judgment**: `Γ ⊢ e : T` means "under context Γ, expression e has type T"

**Rules**:

1. **Variable Lookup**:
   ```
   x: T ∈ Γ
   ──────────
   Γ ⊢ x : T
   ```

2. **Literal Types**:
   ```
   ────────────────
   Γ ⊢ 42 : int

   ────────────────
   Γ ⊢ true : bool

   ────────────────────
   Γ ⊢ "hello" : string
   ```

3. **Function Application**:
   ```
   Γ ⊢ f : fn(T1, T2) -> R    Γ ⊢ arg1 : T1    Γ ⊢ arg2 : T2
   ───────────────────────────────────────────────────────────
   Γ ⊢ f(arg1, arg2) : R
   ```

4. **Binary Operations** (arithmetic):
   ```
   Γ ⊢ e1 : int    Γ ⊢ e2 : int
   ─────────────────────────────
   Γ ⊢ e1 + e2 : int
   ```

5. **If Expression**:
   ```
   Γ ⊢ cond : bool    Γ ⊢ then_branch : T    Γ ⊢ else_branch : T
   ──────────────────────────────────────────────────────────────
   Γ ⊢ if cond then_branch else else_branch : T
   ```

6. **Match Expression**:
   ```
   Γ ⊢ scrutinee : E    Γ ⊢ arm1 : T    ...    Γ ⊢ armN : T    (exhaustive E)
   ──────────────────────────────────────────────────────────────────────────
   Γ ⊢ match scrutinee { arm1, ..., armN } : T
   ```

---

## Type Declarations

**Policy**: Lang4LLM requires **explicit types for all declarations** (no type inference).

**Rationale**: Explicit types improve LLM code generation accuracy by reducing ambiguity and preventing type errors.

**Examples**:

```lang4llm
let x: int = 42;              // OK: explicit type
let y = 42;                   // ERROR: type inference not supported

fn add(a: int, b: int) -> int { ... }  // OK: all types explicit
fn add(a, b) { ... }                   // ERROR: parameter types required
```

**Exception**: Generic type parameters are inferred from usage in function calls (see Generic Types section).

---

## Type Compatibility

### No Implicit Conversions (DR-SEM-001)

Types **never** convert implicitly:

```lang4llm
let x: int = 42;
let y: float = x;             // ERROR: Cannot assign int to float

// Explicit conversion required:
let z: float = x as float;    // OK: Explicit cast
```

**Allowed Explicit Casts**:
- `int as float` (widening, always safe)
- `float as int` (truncation, may lose precision)
- `int as bool` (ERROR: No conversion defined)
- `bool as int` (ERROR: No conversion defined)

---

### Structural vs Nominal Typing

**Structs/Enums**: Nominal typing (name matters)
```lang4llm
struct PointA { x: int, y: int }
struct PointB { x: int, y: int }

let p1: PointA = PointA { x: 0, y: 0 };
let p2: PointB = p1;          // ERROR: PointA ≠ PointB (different names)
```

**Tuples**: Structural typing (structure matters)
```lang4llm
let p1: (int, int) = (0, 0);
let p2: (int, int) = p1;      // OK: Same structure
```

---

## Traits and Polymorphism

### Trait Definitions

Traits define interfaces for types (similar to type classes in Haskell, interfaces in Java):

```lang4llm
trait Printable {
    fn to_string(self: Self) -> string;
}
```

### Trait Implementation

```lang4llm
struct Point {
    x: int,
    y: int
}

impl Printable for Point {
    fn to_string(self: Point) -> string {
        return "Point(" + (self.x as string) + ", " + (self.y as string) + ")";
    }
}
```

### Generic Functions (Future)

**Note**: Full generics deferred to Phase 2+. Placeholder syntax:

---

## Generic Types and Type Parameters

**Status**: Phase 1 feature (user decision 2026-01-31)

Lang4LLM supports **generic programming** through parametric polymorphism with trait bounds. Generics enable code reuse while maintaining type safety.

### Generic Type Parameters

**Syntax**: `<T>` or `<T, U, V>` where `T`, `U`, `V` are type parameter names

**Conventions**:
- Use single uppercase letters: `T`, `U`, `V`, `E`, `K` (Rust convention)
- `T` for generic type
- `E` for error type
- `K` for key type
- `V` for value type

### Generic Functions

Generic functions work with any type that satisfies trait bounds:

```lang4llm
fn identity<T>(x: T) -> T {
    return x;
}

fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a);
}

// Usage
let x: int = identity(42);        // T = int
let y: string = identity("hello"); // T = string
```

**Monomorphization**: Compiler generates separate function for each concrete type used (zero runtime cost).

### Generic Structs

Structs can have type parameters:

```lang4llm
struct Point<T> {
    x: T,
    y: T
}

// Usage
let p1: Point<int> = Point { x: 10, y: 20 };
let p2: Point<float> = Point { x: 1.5, y: 2.5 };
```

**Multiple type parameters**:

```lang4llm
struct Pair<T, U> {
    first: T,
    second: U
}

let p: Pair<int, string> = Pair { first: 42, second: "answer" };
```

### Generic Enums

Enums can have type parameters for polymorphic data structures:

```lang4llm
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Usage
let x: Option<int> = Option::Some(42);
let y: Option<int> = Option::None;

let ok: Result<int, string> = Result::Ok(100);
let err: Result<int, string> = Result::Err("failed");
```

### Trait Bounds

Trait bounds constrain type parameters to types implementing specific traits:

**Syntax**: `T: Trait` or `T: Trait1 + Trait2`

```lang4llm
trait Display {
    fn to_string(self: Display) -> string;
}

fn print_value<T: Display>(value: T) -> unit {
    print(value.to_string());
    return none;
}
```

**Multiple bounds**:

```lang4llm
trait Clone {
    fn clone(self: Clone) -> Clone;
}

fn duplicate<T: Clone + Display>(value: T) -> T {
    print(value.to_string());
    return value.clone();
}
```

### Generic Traits

Traits themselves can be generic:

```lang4llm
trait Iterator<T> {
    fn next(mut self: Iterator<T>) -> Option<T>;
}

trait Collection<T> {
    fn add(mut self: Collection<T>, item: T) -> unit;
    fn len(self: Collection<T>) -> int;
}
```

### Generic Implementations

Implement traits for generic types:

```lang4llm
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        return Point { x: x, y: y };
    }
}

impl<T: Display> Point<T> {
    fn print(self: Point<T>) -> unit {
        print("Point(" + self.x.to_string() + ", " + self.y.to_string() + ")");
        return none;
    }
}
```

**Trait implementation with bounds**:

```lang4llm
impl<T: Clone> Clone for Option<T> {
    fn clone(self: Option<T>) -> Option<T> {
        match self {
            Option::Some(value) => return Option::Some(value.clone());,
            Option::None => return Option::None;,
        }
    }
}
```

### Type Inference for Generics

**Explicit type annotation required** (no inference):

```lang4llm
let x: Option<int> = Option::Some(42);  // OK: explicit type
let y = Option::Some(42);                // ERROR: type cannot be inferred
```

**Function generic parameters inferred from arguments**:

```lang4llm
fn identity<T>(x: T) -> T { return x; }

let x: int = identity(42);  // T inferred as int from argument
```

### Common Generic Types (Standard Library)

**Option<T>**: Nullable value (replaces null)

**Note**: `Option::None` is an enum variant (namespaced), distinct from `none` keyword (unit value).

```lang4llm
enum Option<T> {
    Some(T),
    None  // Enum variant, accessed as Option::None
}

fn divide(a: int, b: int) -> Option<int> {
    if b == 0 {
        return Option::None;  // Enum variant (not 'none' keyword)
    } else {
        return Option::Some(a / b);
    }
}
```

**Result<T, E>**: Error handling (no exceptions)

```lang4llm
enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn parse_int(s: string) -> Result<int, string> {
    // Implementation would parse string to int
    if valid(s) {
        return Result::Ok(parse(s));
    } else {
        return Result::Err("invalid integer");
    }
}
```

**Vec<T>**: Dynamic array (alias for `[T]`)

```lang4llm
let numbers: Vec<int> = [1, 2, 3, 4, 5];
let names: Vec<string> = ["Alice", "Bob"];
```

### Monomorphization Semantics

Lang4LLM uses **monomorphization** (Rust-style):

1. **Compile-time**: Generic functions/types instantiated for each concrete type used
2. **Zero runtime cost**: No virtual dispatch, no type erasure
3. **Code size**: Separate compiled code for each instantiation

**Example**:

```lang4llm
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}

let x: int = max(10, 20);      // Generates max_int(a: i64, b: i64)
let y: float = max(1.5, 2.5);  // Generates max_float(a: f64, b: f64)
```

Transpiles to:

```rust
fn max_int(a: i64, b: i64) -> i64 { if a > b { a } else { b } }
fn max_float(a: f64, b: f64) -> f64 { if a > b { a } else { b } }
```

### Limitations

**No higher-kinded types**: Type parameters cannot themselves be generic

```lang4llm
// NOT supported (higher-kinded type)
fn map<F<_>, T, U>(container: F<T>, f: fn(T) -> U) -> F<U> { ... }
```

**No default type parameters**: All type parameters must be explicitly specified

```lang4llm
// NOT supported (default type parameter)
struct Container<T = int> { value: T }
```

**No associated types**: Traits cannot define associated types (Phase 2+ feature)

```lang4llm
// NOT supported (associated type)
trait Iterator {
    type Item;
    fn next(self: Iterator) -> Option<Item>;
}
```

### Rust Transpilation

Generic Lang4LLM code transpiles directly to Rust generics:

| Lang4LLM | Rust Transpilation |
|----------|-------------------|
| `fn f<T>(x: T) -> T` | `fn f<T>(x: T) -> T` |
| `struct Point<T> { x: T }` | `struct Point<T> { x: T }` |
| `enum Option<T> { Some(T), None }` | `enum Option<T> { Some(T), None }` |
| `T: Trait` | `T: Trait` |
| `T: Trait1 + Trait2` | `T: Trait1 + Trait2` |
| `impl<T> Point<T>` | `impl<T> Point<T>` |

**Zero impedance**: Lang4LLM generics map 1:1 to Rust generics (no semantic gap).

---

## Design Rule Compliance

### DR-SEM-001: Explicit Type Declarations ✅

**Requirement**: No implicit type coercion
**Implementation**:
- All variables require explicit type annotations
- No implicit conversions between types
- Explicit cast syntax: `x as T`

**Impact Score**: I=0.76 (16% type error reduction)

---

### DR-SEM-002: Static Type Checking ✅

**Requirement**: All types verified at compile-time
**Implementation**:
- Type checking phase runs before code execution
- All type errors caught before runtime
- Type soundness guaranteed

**Impact Score**: I=0.70 (70% correctness improvement)

---

### DR-SEM-003: Lexical Scoping ✅

**Requirement**: No variable shadowing
**Implementation**:
- Variables cannot be redeclared in nested scopes
- Compiler error if shadowing detected

**Example**:
```lang4llm
let x: int = 10;
{
    let x: int = 20;  // ERROR: Variable 'x' already declared in outer scope
}
```

**Impact Score**: I=0.58 (reduces scope confusion)

---

### DR-SEM-004: Mandatory Initialization ✅

**Requirement**: Variables must be initialized at declaration
**Implementation**:
- `let x: T;` without initialization is a compiler error
- All variables guaranteed to have valid value
- Eliminates "undefined variable" errors (EP-SEM-001: 18% of semantic errors)

**Impact Score**: I=0.62 (18% undefined variable error elimination)

---

## Type System Soundness

### Soundness Theorem

**Theorem**: If `⊢ e : T` (expression e has type T), then evaluation of e either:
1. Produces a value of type T
2. Diverges (infinite loop)
3. Panics (explicit runtime check failure: bounds check, exhaustive match)

**Note**: Type soundness does NOT prevent runtime panics from:
- Array bounds violations (bounds checking is dynamic)
- Exhaustive match failures (if compiler check bypassed)
- Explicit panic calls

### Progress and Preservation

**Progress**: Well-typed expressions either evaluate to a value or take a step.
**Preservation**: If `e : T` and `e → e'`, then `e' : T`.

**Proof Sketch**: By induction on typing derivation.

---

## Examples

### Type-Safe Division

```lang4llm
enum DivResult {
    Ok(int),
    DivByZero
}

fn safe_divide(a: int, b: int) -> DivResult {
    if b == 0 {
        return DivResult::DivByZero;
    } else {
        return DivResult::Ok(a / b);
    }
}

fn main() -> unit {
    let result: DivResult = safe_divide(10, 0);
    match result {
        DivResult::Ok(value) => print("Result: " + (value as string)),
        DivResult::DivByZero => print("Error: Division by zero"),
    }
}
```

---

## Summary

Lang4LLM's type system prioritizes:
1. **Explicitness**: All types visible in code
2. **Safety**: Static checking eliminates runtime type errors
3. **Simplicity**: No complex inference, clear rules
4. **LLM-Friendly**: Minimal ambiguity, predictable behavior

**Next**: Semantics specification (evaluation rules, error handling, iterator protocol)
