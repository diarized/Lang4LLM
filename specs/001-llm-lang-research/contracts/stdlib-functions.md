# Standard Library Function Contracts (Phase 4)

**Branch**: `001-llm-lang-research` | **Date**: 2026-02-21
**Purpose**: Formal signatures and transpilation contracts for all stdlib functions

## Contract Format

Each function specifies:
- **Signature**: Lang4LLM type signature
- **Transpiles to**: Exact Rust output pattern
- **Preconditions**: Type constraints on arguments
- **Postconditions**: Return type guarantees
- **Side effects**: Mutation, I/O, panics

---

## I/O Functions

### `print`
```
Signature:    fn print(msg: string) -> unit
Transpiles:   println!("{}", {msg})
Side effects: stdout write
```

## Range Functions

### `range`
```
Signature:    fn range(start: int, end: int) -> Iterator<int>
Transpiles:   {start}..{end}
Notes:        Only valid in for-loop iterator position
```

## Math Functions

### `sqrt`
```
Signature:    fn sqrt(x: float) -> float
Transpiles:   ({x}).sqrt()
Precondition: x >= 0.0 (panics on negative in debug mode)
```

### `abs`
```
Signature:    fn abs(x: int) -> int       // overloaded
              fn abs(x: float) -> float
Transpiles:   ({x}).abs()
Notes:        Works on both int and float via Rust trait dispatch
```

### `min`
```
Signature:    fn min(a: int, b: int) -> int
Transpiles:   std::cmp::min({a}, {b})
```

### `max`
```
Signature:    fn max(a: int, b: int) -> int
Transpiles:   std::cmp::max({a}, {b})
```

### `pow`
```
Signature:    fn pow(base: int, exp: int) -> int
Transpiles:   ({base}).pow({exp} as u32)
Precondition: exp >= 0
```

### `floor`
```
Signature:    fn floor(x: float) -> float
Transpiles:   ({x}).floor()
```

### `ceil`
```
Signature:    fn ceil(x: float) -> float
Transpiles:   ({x}).ceil()
```

### `round`
```
Signature:    fn round(x: float) -> float
Transpiles:   ({x}).round()
```

## Conversion Functions

### `as_float`
```
Signature:    fn as_float(x: T) -> float
Transpiles:   ({x} as f64)
Precondition: x is numeric type
```

### `as_int`
```
Signature:    fn as_int(x: T) -> int
Transpiles:   { let __s = format!("{}", {x}); __s.parse::<i64>().unwrap_or(0) }
Notes:        Works for both numeric and string inputs
```

### `to_string`
```
Signature:    fn to_string(x: T) -> string
Transpiles:   format!("{}", {x})
Notes:        Universal conversion via Rust Display trait
```

## String Functions

### `contains` (string overload)
```
Signature:    fn contains(s: string, substring: string) -> bool
Transpiles:   {s}.contains({substring}.as_str())
```

### `split`
```
Signature:    fn split(s: string, delimiter: string) -> [string]
Transpiles:   {s}.split({delimiter}.as_str()).map(|s| s.to_string()).collect::<Vec<String>>()
```

### `trim`
```
Signature:    fn trim(s: string) -> string
Transpiles:   {s}.trim().to_string()
Notes:        Returns new string (owned), trims both ends
```

### `replace`
```
Signature:    fn replace(s: string, from: string, to: string) -> string
Transpiles:   {s}.replace({from}.as_str(), {to}.as_str())
Notes:        Replaces all occurrences
```

### `starts_with`
```
Signature:    fn starts_with(s: string, prefix: string) -> bool
Transpiles:   {s}.starts_with({prefix}.as_str())
```

### `ends_with`
```
Signature:    fn ends_with(s: string, suffix: string) -> bool
Transpiles:   {s}.ends_with({suffix}.as_str())
```

### `to_upper`
```
Signature:    fn to_upper(s: string) -> string
Transpiles:   {s}.to_uppercase()
```

### `to_lower`
```
Signature:    fn to_lower(s: string) -> string
Transpiles:   {s}.to_lowercase()
```

### `chars`
```
Signature:    fn chars(s: string) -> [string]
Transpiles:   {s}.chars().map(|c| c.to_string()).collect::<Vec<String>>()
Notes:        Returns Vec of single-character strings (not char type)
```

## Array Functions

### `push`
```
Signature:    fn push(arr: mut [T], item: T) -> unit
Transpiles:   {arr}.push({item})
Side effects: Mutates arr
```

### `pop`
```
Signature:    fn pop(arr: mut [T]) -> T
Transpiles:   {arr}.pop().unwrap()
Side effects: Mutates arr
Notes:        Panics on empty array (Rust unwrap)
```

### `len`
```
Signature:    fn len(x: T) -> int
Transpiles:   {x}.len()
Notes:        Works on arrays and strings
```

### `enumerate`
```
Signature:    fn enumerate(arr: [T]) -> Iterator<(int, T)>
Transpiles:   {arr}.into_iter().enumerate()
Notes:        Only valid in for-loop iterator position
```

### `sort`
```
Signature:    fn sort(arr: mut [T]) -> unit
Transpiles:   {arr}.sort()
Precondition: T: Ord (comparable)
Side effects: Mutates arr in-place
```

### `reverse`
```
Signature:    fn reverse(arr: mut [T]) -> unit
Transpiles:   {arr}.reverse()
Side effects: Mutates arr in-place
```

### `map`
```
Signature:    fn map(arr: [T], f: fn(T) -> U) -> [U]
Transpiles:   {arr}.into_iter().map({f}).collect::<Vec<_>>()
Notes:        Consumes input array, produces new array
```

### `filter`
```
Signature:    fn filter(arr: [T], f: fn(T) -> bool) -> [T]
Transpiles:   {arr}.into_iter().filter(|x| {f}(x.clone())).collect::<Vec<_>>()
Notes:        Consumes input array, produces new array
```

### `is_empty`
```
Signature:    fn is_empty(arr: [T]) -> bool
Transpiles:   {arr}.is_empty()
```

### `contains` (array overload)
```
Signature:    fn contains(arr: [T], item: T) -> bool
Transpiles:   {arr}.contains(&{item})
Notes:        Requires T: PartialEq
```

### `concat`
```
Signature:    fn concat(a: [T], b: [T]) -> [T]
Transpiles:   { let mut __v = {a}; __v.extend({b}); __v }
Notes:        Produces new array from two inputs
```

### `join`
```
Signature:    fn join(arr: [string], separator: string) -> string
Transpiles:   {arr}.join({separator}.as_str())
Precondition: arr must be [string]
```

### `first`
```
Signature:    fn first(arr: [T]) -> T
Transpiles:   {arr}.first().cloned().unwrap()
Notes:        Panics on empty array
```

---

## Summary

| Category | Count | Functions |
|----------|-------|-----------|
| I/O | 1 | print |
| Range | 1 | range |
| Math | 8 | sqrt, abs, min, max, pow, floor, ceil, round |
| Conversion | 3 | as_float, as_int, to_string |
| String | 10 | contains, split, trim, replace, starts_with, ends_with, to_upper, to_lower, chars, len* |
| Array | 13 | push, pop, len*, enumerate, sort, reverse, map, filter, is_empty, contains, concat, join, first |
| **Total** | **29** | (len and contains are overloaded) |
