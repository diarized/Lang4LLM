# Standard Library Signatures (Phase 4)

**Version**: 2.0
**Date**: 2026-02-21
**Status**: Implemented (Phase 4 complete)
**Purpose**: Complete stdlib function reference — all 29 functions implemented as transpiler-level builtins

---

## Core Functions

### Range Iterator

```lang4llm
fn range(start: int, end: int) -> Iterator<int>;
```

**Description**: Creates exclusive range `[start, end)`.

**Examples**:
```lang4llm
for i in range(0, 10) {  // Iterates 0, 1, 2, ..., 9
    print(i);
}
```

**Transpiles to**: Rust `start..end` range expression

---

### Print Function

```lang4llm
fn print(msg: string) -> unit;
```

**Description**: Prints message to stdout.

**Examples**:
```lang4llm
print("Hello, World!");
print("Value: " + x);
```

**Transpiles to**: Rust `println!("{}", msg)`

---

### Length Function

```lang4llm
fn len<T>(arr: [T]) -> int;
```

**Description**: Returns array length.

**Examples**:
```lang4llm
let arr: [int] = [1, 2, 3];
let size: int = len(arr);  // Returns 3
```

**Transpiles to**: Rust `arr.len()`

---

### Enumerate Iterator

```lang4llm
fn enumerate<T>(arr: [T]) -> Iterator<(int, T)>;
```

**Description**: Returns iterator of (index, value) pairs.

**Examples**:
```lang4llm
let names: [string] = ["Alice", "Bob", "Charlie"];
for (i, name) in enumerate(names) {
    print("Index: " + i + ", Name: " + name);
}
```

**Transpiles to**: Rust `arr.iter().enumerate()`

---

## Array Methods

### Push Method

```lang4llm
fn push<T>(arr: mut [T], item: T) -> unit;
```

**Description**: Appends item to end of array.

**Examples**:
```lang4llm
let mut numbers: [int] = [1, 2, 3];
push(numbers, 4);  // numbers = [1, 2, 3, 4]
```

**Transpiles to**: Rust `arr.push(item)`

---

### Pop Method

```lang4llm
fn pop<T>(arr: mut [T]) -> Option<T>;
```

**Description**: Removes and returns last element.

**Examples**:
```lang4llm
let mut numbers: [int] = [1, 2, 3];
let last: Option<int> = pop(numbers);  // Some(3), numbers = [1, 2]
```

**Transpiles to**: Rust `arr.pop()`

---

## String Functions

### String Length

```lang4llm
fn len(s: string) -> int;
```

**Description**: Returns string length in bytes.

**Examples**:
```lang4llm
let msg: string = "Hello";
let size: int = len(msg);  // Returns 5
```

**Transpiles to**: Rust `s.len()`

---

## Mathematical Functions

### Square Root

```lang4llm
fn sqrt(x: float) -> float;
```

**Description**: Returns square root of x.

**Examples**:
```lang4llm
let result: float = sqrt(16.0);  // Returns 4.0
```

**Transpiles to**: Rust `x.sqrt()`

---

## Type Conversion

### Integer to Float

```lang4llm
fn as_float(x: int) -> float;
```

**Description**: Converts integer to float.

**Examples**:
```lang4llm
let x: int = 42;
let y: float = as_float(x);  // 42.0
```

**Transpiles to**: Rust `x as f64`

---

### Float to Integer (Truncate)

```lang4llm
fn as_int(x: float) -> int;
```

**Description**: Converts float to integer (truncates decimal part).

**Examples**:
```lang4llm
let x: float = 42.7;
let y: int = as_int(x);  // 42
```

**Transpiles to**: Rust `x as i64`

---

---

## Phase 4 Functions (Implemented)

### Math Functions

| Function | Signature | Rust Output |
|----------|-----------|-------------|
| `abs` | `fn abs<T>(x: T) -> T` | `(x).abs()` |
| `min` | `fn min(a: int, b: int) -> int` | `std::cmp::min(a, b)` |
| `max` | `fn max(a: int, b: int) -> int` | `std::cmp::max(a, b)` |
| `pow` | `fn pow(base: int, exp: int) -> int` | `(base).pow(exp as u32)` |
| `floor` | `fn floor(x: float) -> float` | `(x).floor()` |
| `ceil` | `fn ceil(x: float) -> float` | `(x).ceil()` |
| `round` | `fn round(x: float) -> float` | `(x).round()` |

### String Functions

| Function | Signature | Rust Output |
|----------|-----------|-------------|
| `to_string` | `fn to_string<T>(x: T) -> string` | `format!("{}", x)` |
| `contains` | `fn contains(s: string, sub: string) -> bool` | `s.contains(&sub)` |
| `split` | `fn split(s: string, delim: string) -> [string]` | `s.split(d.as_str()).map(…).collect()` |
| `trim` | `fn trim(s: string) -> string` | `s.trim().to_string()` |
| `replace` | `fn replace(s: string, from: string, to: string) -> string` | `s.replace(&from, &to)` |
| `starts_with` | `fn starts_with(s: string, prefix: string) -> bool` | `s.starts_with(prefix.as_str())` |
| `ends_with` | `fn ends_with(s: string, suffix: string) -> bool` | `s.ends_with(suffix.as_str())` |
| `to_upper` | `fn to_upper(s: string) -> string` | `s.to_uppercase()` |
| `to_lower` | `fn to_lower(s: string) -> string` | `s.to_lowercase()` |
| `chars` | `fn chars(s: string) -> [string]` | `s.chars().map(…).collect()` |

### Array/Collection Functions

| Function | Signature | Rust Output |
|----------|-----------|-------------|
| `sort` | `fn sort<T>(arr: mut [T]) -> unit` | `arr.sort()` |
| `reverse` | `fn reverse<T>(arr: mut [T]) -> unit` | `arr.reverse()` |
| `map` | `fn map<T, U>(arr: [T], f: fn(T)->U) -> [U]` | `arr.into_iter().map(f).collect()` |
| `filter` | `fn filter<T>(arr: [T], f: fn(T)->bool) -> [T]` | `arr.into_iter().filter(…).collect()` |
| `is_empty` | `fn is_empty<T>(arr: [T]) -> bool` | `arr.is_empty()` |
| `contains` | `fn contains<T>(arr: [T], item: T) -> bool` | `arr.contains(&item)` |
| `concat` | `fn concat<T>(a: [T], b: [T]) -> [T]` | `{ let mut v = a; v.extend(b); v }` |
| `join` | `fn join(arr: [string], sep: string) -> string` | `arr.join(sep.as_str())` |
| `first` | `fn first<T>(arr: [T]) -> T` | `arr.first().cloned().unwrap()` |
| `pop` | `fn pop<T>(arr: mut [T]) -> T` | `arr.pop().unwrap()` |
| `enumerate` | `fn enumerate<T>(arr: [T]) -> Iterator<(int, T)>` | `arr.into_iter().enumerate()` |

---

## Complete Function Index (29 total)

| # | Name | Category | Tier |
|---|------|----------|------|
| 1 | `print` | I/O | 1 |
| 2 | `range` | Iterator | 1 |
| 3 | `len` | Array | 1 |
| 4 | `push` | Array | 1 |
| 5 | `sqrt` | Math | 1 |
| 6 | `as_float` | Conversion | 1 |
| 7 | `as_int` | Conversion | 1 |
| 8 | `pop` | Array | 2 |
| 9 | `enumerate` | Iterator | 2 |
| 10 | `abs` | Math | 3 |
| 11 | `min` | Math | 3 |
| 12 | `max` | Math | 3 |
| 13 | `pow` | Math | 3 |
| 14 | `floor` | Math | 3 |
| 15 | `ceil` | Math | 3 |
| 16 | `round` | Math | 3 |
| 17 | `to_string` | String | 4 |
| 18 | `contains` | String/Array | 4 |
| 19 | `split` | String | 4 |
| 20 | `trim` | String | 4 |
| 21 | `replace` | String | 4 |
| 22 | `starts_with` | String | 4 |
| 23 | `ends_with` | String | 4 |
| 24 | `to_upper` | String | 4 |
| 25 | `to_lower` | String | 4 |
| 26 | `chars` | String | 4 |
| 27 | `sort` | Array | 5 |
| 28 | `reverse` | Array | 5 |
| 29 | `map` | Array | 5 |
| 30 | `filter` | Array | 5 |
| 31 | `is_empty` | Array | 5 |
| 32 | `concat` | Array | 5 |
| 33 | `join` | Array | 5 |
| 34 | `first` | Array | 5 |

---

## Notes

1. **Implementation**: All functions implemented as transpiler-level rewrites (Phase 4 complete)
2. **Type System**: Functions use `Type::Generic("T")` in typeck for polymorphic signatures
3. **Shadowing**: All stdlib names can be shadowed by user-defined variables (`is_builtin: true`)
4. **Overloading**: `contains` works for both `string` and `[T]` — both map to `.contains(&arg)`
5. **Safety**: `first` and `pop` panic on empty arrays (Lang4LLM assumes bounds-checked inputs)

---

## Version Information

**Version**: 2.0
**Date**: 2026-02-21
**Phase**: Phase 4 - Standard Library Foundation (complete)
**Status**: All 29 functions implemented, tested, and documented
