# Lang4LLM Memory Model Specification

**Version**: 1.0
**Date**: 2026-01-31
**Status**: Phase 1 - Language Specification
**Memory Management**: Rust Ownership Model (Compile-Time, Zero GC)
**Transpilation Target**: Rust

---

## Table of Contents

1. [Overview](#overview)
2. [Ownership Model](#ownership-model)
3. [Borrowing Rules](#borrowing-rules)
4. [Lifetime Management](#lifetime-management)
5. [Stack vs Heap Allocation](#stack-vs-heap-allocation)
6. [RAII and Deterministic Cleanup](#raii-and-deterministic-cleanup)
7. [Transpiler Inference Algorithms](#transpiler-inference-algorithms)
8. [Memory Safety Guarantees](#memory-safety-guarantees)
9. [Performance Characteristics](#performance-characteristics)
10. [LLM Mental Model](#llm-mental-model)

---

## Overview

Lang4LLM uses **Rust's ownership model** for memory management, accessed through **simplified syntax** for LLM code generation.

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│  LLM generates Lang4LLM (simple syntax, no & or ')      │
│  - let x: string = "hello";                             │
│  - fn process(s: string) -> unit                        │
│  - No explicit borrows, no lifetime annotations         │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Deterministic Transpiler (Phase 3)                     │
│  - Infers borrows (&T vs &mut T vs T)                   │
│  - Inserts lifetime annotations where needed            │
│  - Maps types (string → String, [T] → Vec<T>)           │
│  - 100% algorithmic, rule-based (NOT AI)                │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Rust Code (with ownership annotations)                 │
│  - let x: String = String::from("hello");               │
│  - fn process(s: &String)                               │
│  - Explicit &, &mut, lifetimes inserted                 │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  rustc Borrow Checker (final safety validation)         │
│  - Verifies no use-after-free                           │
│  - Verifies no data races                               │
│  - Verifies no dangling pointers                        │
│  - Compile error if unsafe                              │
└─────────────────────────────────────────────────────────┘
                          ↓
                      Binary
```

### Core Principle

**Separation of Concerns**:
- **LLMs**: Generate simple, predictable code (no memory management syntax)
- **Transpiler**: Algorithmically insert ownership annotations (deterministic rules)
- **Rust Compiler**: Validate memory safety (borrow checker = final authority)

---

## Ownership Model

Lang4LLM follows **Rust's ownership rules** (enforced at compile time, zero runtime overhead):

### Rule 1: Single Owner

Each value has **exactly one owner**:

```lang4llm
let s1: string = "hello";  // s1 owns the string
let s2: string = s1;       // Ownership moves from s1 to s2
// s1 is no longer valid here (compile error if used)
```

**Rust Transpilation**:

```rust
let s1: String = String::from("hello");
let s2: String = s1;  // Move in Rust
// s1 invalid here
```

**LLM Mental Model**: Variable `s1` becomes "consumed" after move. Transpiler tracks this.

---

### Rule 2: Ownership Transfer

Passing a value to a function **transfers ownership** (unless borrowing):

```lang4llm
fn consume(s: string) -> unit {
    print(s);
}  // s dropped here (memory freed)

fn main() -> unit {
    let msg: string = "Hello";
    consume(msg);  // msg moved into function
    // msg invalid here (compile error if used)
}
```

**Rust Transpilation**:

```rust
fn consume(s: String) {  // Takes ownership
    println!("{}", s);
}  // s dropped here (String::drop called)

fn main() {
    let msg: String = String::from("Hello");
    consume(msg);  // Move
    // msg invalid
}
```

---

### Rule 3: Scope-Based Cleanup

When owner goes out of scope, value is **automatically freed** (RAII):

```lang4llm
fn example() -> unit {
    let s: string = "hello";  // s allocated
    print(s);
}  // s goes out of scope, memory freed automatically
```

**Rust Transpilation**:

```rust
fn example() {
    let s: String = String::from("hello");
    println!("{}", s);
}  // String::drop called automatically (no manual free)
```

---

## Borrowing Rules

### Immutable Borrow (&T)

Multiple readers allowed simultaneously:

```lang4llm
fn read_length(s: string) -> int {
    return len(s);  // Reads s, doesn't modify
}

fn main() -> unit {
    let msg: string = "Hello";
    let l1: int = read_length(msg);  // Borrow
    let l2: int = read_length(msg);  // Borrow again (OK)
    print(msg);  // Original still valid
}
```

**Transpiler Inference** (algorithmic):
- `read_length` body analysis: reads `s`, doesn't mutate → **infer `&String`**
- Call sites: insert `&msg` automatically

**Rust Transpilation**:

```rust
fn read_length(s: &String) -> i64 {  // Transpiler added &
    return s.len() as i64;
}

fn main() {
    let msg: String = String::from("Hello");
    let l1: i64 = read_length(&msg);  // Transpiler added &
    let l2: i64 = read_length(&msg);
    println!("{}", msg);
}
```

---

### Mutable Borrow (&mut T)

**Only one** mutable borrow at a time:

```lang4llm
fn append_suffix(s: mut string, suffix: string) -> unit {
    s = s + suffix;  // Modifies s
}

fn main() -> unit {
    let mut msg: string = "Hello";
    append_suffix(msg, " World");  // Mutable borrow
    print(msg);  // Prints: "Hello World"
}
```

**Transpiler Inference**:
- `append_suffix` body analysis: mutates `s` → **infer `&mut String`**
- Call site: insert `&mut msg`

**Rust Transpilation**:

```rust
fn append_suffix(s: &mut String, suffix: &String) {
    s.push_str(suffix);
}

fn main() {
    let mut msg: String = String::from("Hello");
    append_suffix(&mut msg, &String::from(" World"));
    println!("{}", msg);
}
```

---

### Borrow Checking Rules

Rust's borrow checker enforces (at compile time):

1. **No simultaneous mutable and immutable borrows**:
   ```lang4llm
   let mut x: int = 10;
   let r1: int = x;        // Immutable borrow (read)
   x = 20;                 // ERROR: can't mutate while borrowed
   ```

2. **No multiple mutable borrows**:
   ```lang4llm
   let mut s: string = "hello";
   append_suffix(s, "1");  // Mutable borrow
   append_suffix(s, "2");  // OK: first borrow ended
   // But NOT: append_suffix(s, "1") + append_suffix(s, "2") in same expression
   ```

3. **Borrows must not outlive owner**:
   ```lang4llm
   fn dangling_reference() -> string {
       let s: string = "temp";
       return s;  // ERROR: would return reference to dropped value
   }
   ```

**Transpiler Enforcement**: Transpiler generates Rust code, rustc validates rules.

---

## Lifetime Management

### Implicit Lifetimes (Hidden from LLMs)

Lang4LLM **does not expose lifetime syntax** (`'a`) to LLMs. Transpiler infers lifetimes algorithmically.

**Example: Lifetime Elision**

```lang4llm
// Lang4LLM (no lifetime annotations)
fn first_word(s: string) -> string {
    // Returns substring (borrows from s)
    return s[0..5];
}
```

**Rust Transpilation** (transpiler inserts lifetime):

```rust
fn first_word<'a>(s: &'a String) -> &'a String {
    // Lifetime 'a ties output to input
    return &s[0..5];
}
```

**Transpiler Rule**: Output borrows from input → same lifetime.

---

### Lifetime Inference Algorithm

Transpiler applies **Rust's lifetime elision rules**:

1. **Rule 1**: Each borrowed parameter gets its own lifetime:
   ```rust
   fn f(x: &T, y: &U)  →  fn f<'a, 'b>(x: &'a T, y: &'b U)
   ```

2. **Rule 2**: If exactly one input lifetime, output gets same lifetime:
   ```rust
   fn f(x: &T) -> &U  →  fn f<'a>(x: &'a T) -> &'a U
   ```

3. **Rule 3**: If multiple inputs, lifetimes are independent (explicit annotation needed):
   ```rust
   fn f(x: &T, y: &U) -> &V  →  Compile error (ambiguous)
   ```

**LLM Benefit**: No lifetime syntax in Lang4LLM. Transpiler handles complexity.

---

### Struct Lifetimes

Structs that store references need lifetimes (transpiler inferred):

```lang4llm
// Lang4LLM (simple syntax)
struct Excerpt {
    text: string  // Borrows string (doesn't own)
}

fn create_excerpt(s: string) -> Excerpt {
    return Excerpt { text: s };
}
```

**Rust Transpilation**:

```rust
struct Excerpt<'a> {  // Transpiler adds lifetime
    text: &'a String
}

fn create_excerpt<'a>(s: &'a String) -> Excerpt<'a> {
    return Excerpt { text: s };
}
```

**Transpiler Analysis**: `text` field borrows → struct needs lifetime parameter.

---

## Stack vs Heap Allocation

### Allocation Strategy

Lang4LLM follows **Rust's allocation rules**:

| Type | Storage | Reason |
|------|---------|--------|
| Primitives (`int`, `float`, `bool`) | Stack | Fixed size, Copy |
| Tuples (fixed size) | Stack | Known size at compile time |
| Arrays (dynamic `[T]`) | Heap | Size unknown at compile time |
| Strings (`string`) | Heap | Size varies (UTF-8 text) |
| Structs (simple) | Stack | Fixed size fields |
| Enums (variants) | Stack | Largest variant size |
| Boxed values | Heap | Explicitly heap-allocated |

---

### Stack Allocation (Fast, Fixed Size)

```lang4llm
let x: int = 42;          // Stack (8 bytes)
let point: (int, int) = (10, 20);  // Stack (16 bytes)
```

**Rust Transpilation**:

```rust
let x: i64 = 42;  // Stack
let point: (i64, i64) = (10, 20);  // Stack
```

**Properties**:
- **Fast allocation**: Just move stack pointer
- **Automatic cleanup**: Pop stack frame on scope exit
- **Deterministic**: Size known at compile time

---

### Heap Allocation (Dynamic Size)

```lang4llm
let s: string = "hello";      // Heap (String buffer)
let arr: [int] = [1, 2, 3];   // Heap (Vec buffer)
```

**Rust Transpilation**:

```rust
let s: String = String::from("hello");  // Heap
let arr: Vec<i64> = vec![1, 2, 3];      // Heap
```

**Properties**:
- **Dynamic size**: Grows/shrinks as needed
- **Owned allocation**: Single owner, RAII cleanup
- **Zero-copy moves**: Pointer moved, not data

---

### Box (Explicit Heap)

For recursive types or large values (future extension):

```lang4llm
// Future: Box<T> for heap-allocated values
struct Node {
    value: int,
    next: Box<Node>  // Recursive type requires heap
}
```

**Rust Transpilation**:

```rust
struct Node {
    value: i64,
    next: Box<Node>  // Box forces heap allocation
}
```

---

## RAII and Deterministic Cleanup

### RAII (Resource Acquisition Is Initialization)

Resources **automatically cleaned up** when owner goes out of scope:

```lang4llm
fn process_file(path: string) -> unit {
    let file: File = open(path);  // Acquire resource
    let content: string = read(file);
    print(content);
}  // file automatically closed here (File::drop called)
```

**Rust Transpilation**:

```rust
fn process_file(path: &String) {
    let file: File = File::open(path).unwrap();
    let content: String = std::fs::read_to_string(path).unwrap();
    println!("{}", content);
}  // Drop trait calls file.close() automatically
```

**Benefits**:
- **No manual cleanup**: No `close()`, `free()`, `delete`
- **Exception-safe**: Cleanup happens even on panic
- **Deterministic**: Cleanup at known point (scope exit)

---

### Drop Trait

All types implement `Drop` trait (automatic or custom):

```lang4llm
// Lang4LLM (cleanup implicit)
let s: string = "hello";
// s automatically freed at scope exit
```

**Rust Transpilation**:

```rust
let s: String = String::from("hello");
// String::drop deallocates heap buffer at scope exit
```

**Custom Drop** (future extension for user types):

```rust
impl Drop for File {
    fn drop(&mut self) {
        // Close file handle, flush buffers
        self.handle.close();
    }
}
```

---

## Transpiler Inference Algorithms

### Algorithm 1: Borrow Type Inference

**Input**: Function definition and body AST
**Output**: Borrow annotations (`&T`, `&mut T`, or `T`)

```python
def infer_borrow_type(param: Parameter, body: AST) -> BorrowType:
    if is_consumed(param, body):
        # Parameter moved/dropped inside function
        return BorrowType.Owned  # T (no &)

    elif is_mutated(param, body):
        # Parameter modified (reassigned, fields mutated)
        return BorrowType.MutableBorrow  # &mut T

    else:
        # Parameter only read (no mutations)
        return BorrowType.ImmutableBorrow  # &T

def is_consumed(param, body):
    # Check if param is:
    # - Returned by value
    # - Passed to consuming function
    # - Stored in owned field
    ...

def is_mutated(param, body):
    # Check if param is:
    # - Reassigned (param = ...)
    # - Passed to mutating function
    # - Fields modified (param.field = ...)
    ...
```

**Example**:

```lang4llm
fn read(s: string) -> int {
    return len(s);  // Only reads s
}
// Transpiler infers: fn read(s: &String) -> i64

fn consume(s: string) -> unit {
    print(s);  // Uses s, then drops
}
// Transpiler infers: fn consume(s: String)

fn modify(s: mut string) -> unit {
    s = s + "!";  // Mutates s
}
// Transpiler infers: fn modify(s: &mut String)
```

---

### Algorithm 2: Lifetime Inference

**Input**: Function signature with borrows
**Output**: Lifetime annotations

```python
def infer_lifetimes(func: Function) -> Function:
    borrowed_params = [p for p in func.params if is_borrow(p)]
    borrowed_return = is_borrow(func.return_type)

    if not borrowed_return:
        # No output borrows → no lifetimes needed
        return func

    elif len(borrowed_params) == 1:
        # Rule 2: Single input borrow → output has same lifetime
        func.return_lifetime = borrowed_params[0].lifetime

    else:
        # Multiple input borrows → require explicit annotation
        # (For Phase 1: prefer owned returns to avoid complexity)
        raise CompileError("Ambiguous lifetime, return owned value instead")

    return func
```

**Example**:

```lang4llm
fn first(arr: [int]) -> int {
    return arr[0];  // Borrows from arr
}
// Transpiler infers: fn first<'a>(arr: &'a Vec<i64>) -> &'a i64
```

---

### Algorithm 3: Move/Copy Detection

**Input**: Variable usage pattern
**Output**: Move or copy semantic

```python
def is_copy_type(typ: Type) -> bool:
    # Primitives are Copy
    if typ in [INT, FLOAT, BOOL]:
        return True

    # Tuples are Copy if all fields are Copy
    if isinstance(typ, Tuple):
        return all(is_copy_type(f) for f in typ.fields)

    # Everything else is Move (String, Vec, custom types)
    return False

def check_usage_after_move(var: Variable, usage: AST):
    if was_moved(var) and is_used(var, usage):
        raise CompileError(f"Use of moved value: {var.name}")
```

---

## Memory Safety Guarantees

Lang4LLM provides **Rust's memory safety guarantees** (verified at compile time):

### Guarantee 1: No Use-After-Free

```lang4llm
let s1: string = "hello";
let s2: string = s1;  // s1 moved
// print(s1);  // COMPILE ERROR: s1 moved
```

**rustc Error**: "borrow of moved value: `s1`"

---

### Guarantee 2: No Data Races

```lang4llm
// Cannot have mutable + immutable borrows simultaneously
let mut x: int = 10;
let r1: int = x;   // Immutable borrow
// x = 20;         // COMPILE ERROR: cannot mutate while borrowed
print(r1);
```

**rustc Error**: "cannot assign to `x` because it is borrowed"

---

### Guarantee 3: No Dangling Pointers

```lang4llm
fn bad() -> string {
    let s: string = "temp";
    return s;  // OK: returns owned value (not reference)
}

// If we tried to return reference (transpiler would insert &):
// fn bad<'a>() -> &'a String {
//     let s: String = String::from("temp");
//     return &s;  // COMPILE ERROR: s dropped at end of function
// }
```

**rustc Error**: "returns a reference to data owned by the current function"

---

### Guarantee 4: No Null Pointer Dereferences

Lang4LLM has **no null** (use `Option<T>`):

```lang4llm
fn find(arr: [int], target: int) -> Option {
    for item in arr {
        if item == target {
            return Option::Some(item);
        }
    }
    return Option::None;  // Explicit "not found"
}
```

**Rust Transpilation**: `Option<i64>` (no null pointers possible)

---

## Performance Characteristics

### Zero-Cost Abstractions

Lang4LLM ownership model has **zero runtime overhead**:

- **Compile-time**: Borrow checker validates safety
- **Runtime**: No reference counting, no garbage collection, no runtime checks
- **Performance**: Identical to manual C memory management (but safe)

---

### Memory Overhead

| Feature | Runtime Cost | Reason |
|---------|--------------|--------|
| Ownership | **Zero** | Compile-time only |
| Borrowing | **Zero** | Pointers (native CPU) |
| Lifetimes | **Zero** | Compile-time only |
| Drop | **Zero** | Inlined destructor call |
| Bounds checking | **Small** | Array indexing only (can optimize away) |

**Option<T> Optimization**: Rust optimizes `Option<&T>` to single pointer (null optimization).

---

### Comparison to GC Languages

| Aspect | Lang4LLM (Rust) | Python/Java (GC) |
|--------|-----------------|------------------|
| Memory overhead | 0% | 20-50% (GC metadata) |
| Pause times | None | 1-100ms (GC pauses) |
| Determinism | Perfect | Unpredictable GC |
| Manual management | None | None |
| Safety | Compile-time | Runtime (null checks) |

---

## LLM Mental Model

### What LLMs See (Simple)

LLMs generate Lang4LLM without thinking about memory:

```lang4llm
// LLM writes simple code
fn process_data(data: [int]) -> int {
    let mut sum: int = 0;
    for num in data {
        sum = sum + num;
    }
    return sum;
}
```

**LLM knows**:
- Variables have types
- Functions take parameters
- Loops iterate over collections
- Values can be mutable or immutable

**LLM doesn't know**:
- Where data is stored (stack vs heap)
- When memory is freed (automatic)
- Borrow vs move semantics (transpiler handles)
- Lifetime annotations (invisible)

---

### What Transpiler Handles (Complex)

Transpiler inserts Rust ownership details:

```rust
// Transpiler generates this
fn process_data(data: &Vec<i64>) -> i64 {  // Added &
    let mut sum: i64 = 0;
    for num in data.iter() {  // Added .iter()
        sum = sum + num;
    }
    return sum;
}
```

**Transpiler responsibilities**:
- Infer `&Vec<i64>` (borrow, not move)
- Insert `.iter()` for iteration
- Map `int` → `i64`, `[int]` → `Vec<i64>`
- Verify usage patterns (move tracking)

---

### What Rust Compiler Validates (Safety)

rustc verifies memory safety:

- ✅ No use-after-free
- ✅ No dangling pointers
- ✅ No data races
- ✅ No null dereferences

**If invalid**: Compile error with helpful message.

---

## Summary

Lang4LLM memory model:

1. **Based on Rust ownership** (compile-time, zero overhead)
2. **Simplified syntax for LLMs** (no `&`, `&mut`, `'a`)
3. **Deterministic transpiler** (algorithmic borrow/lifetime inference)
4. **Rust compiler validates** (final safety check)
5. **No garbage collection** (RAII, deterministic cleanup)
6. **Zero-cost abstractions** (performance = manual C, safety = automatic)

**Design Philosophy**:
- LLMs generate simple, predictable code
- Transpiler handles Rust complexity (deterministic rules)
- Rust compiler guarantees safety (borrow checker)
- Runtime has zero memory management overhead

**Next Steps**: See `PHASE1_PLAN.md` for integration and validation tasks.

---

## Version Information

**Version**: 1.0
**Last Updated**: 2026-01-31
**Phase**: Phase 1 - Language Specification (Complete)
**Memory Model**: Rust Ownership (Compile-Time, Zero GC)
**Transpilation**: Deterministic compiler (NOT LLM-based)
**Safety**: Guaranteed at compile time by rustc borrow checker
