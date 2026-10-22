# Phase 1 Language Design Decisions

**Date**: 2026-01-31
**Context**: Pre-Integration Review (Phase 1.6)
**Status**: Approved by user

---

## Core Language Decisions

### 1. No Implicit Returns ❌

**Decision**: Require explicit `return` statements for all code paths.

**Rationale**:
- Eliminates ambiguity (explicit > implicit)
- Clearer for LLM code generation
- No confusion about block return values

**Example**:
```lang4llm
// REQUIRED (explicit):
fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

// FORBIDDEN (implicit):
fn max(a: int, b: int) -> int {
    if a > b { a } else { b }  // ERROR: Missing return
}
```

---

### 2. No Type Inference ❌

**Decision**: All types must be explicitly declared (no inference).

**Rationale**:
- Reduces LLM cognitive load
- Makes code self-documenting
- Prevents type mismatch errors (EP-SEM-001: 18% reduction)

**Example**:
```lang4llm
// REQUIRED (explicit):
let x: int = 42;
let name: string = "Alice";

// FORBIDDEN (inferred):
let x = 42;           // ERROR: Missing type annotation
let name = "Alice";   // ERROR: Missing type annotation
```

---

### 3. Generics in Phase 1 ✅

**Decision**: Define generics NOW (Phase 1) with full grammar and type system support.

**Rationale**:
- Prevents breaking changes to syntax later
- Allows stdlib to use generic types (`Vec<T>`, `Option<T>`, `Result<T,E>`)
- Complete type system from start

**Scope**:
- Generic type parameters: `<T>`
- Trait bounds: `T: Trait`
- Generic functions and structs
- Monomorphization semantics (transpile to Rust generics)

**Impact**:
- Grammar: +50-100 lines (generic syntax)
- Type System: +200-300 lines (type parameter rules)
- Effort: +2-3 days specification work

---

### 4. Defer Standard Library 🕒

**Decision**: Do NOT define standard library in Phase 1.

**What to do**:
- Examples use `range()`, `print()`, `len()` without full definitions
- Add minimal type signatures (forward declarations) for type-checking
- Full stdlib implementation deferred to Phase 4

**Minimal signatures** (add to specs):
```lang4llm
// Stdlib (Phase 4 - signatures only)
fn range(start: int, end: int) -> Iterator<int>;
fn print(msg: string) -> unit;
fn len<T>(arr: [T]) -> int;
fn enumerate<T>(arr: [T]) -> Iterator<(int, T)>;
```

---

## Implementation Notes

### Implicit Returns Removal
- **Files to update**: `semantics.md` (remove lines 141-163), all examples
- **Pattern**: Replace `{ expr }` with `{ return expr; }`

### Type Inference Enforcement
- **Already correct** in specs (types.md:323, types.md:446)
- **Verify**: No inference examples in specs

### Generics Addition
- **Grammar**: Add `<T>` syntax, bounds, where clauses
- **Type System**: Add monomorphization, trait resolution, type parameter rules
- **Examples**: Update to use `Option<T>`, `Result<T,E>`, `Vec<T>`

### Stdlib Deferral
- **Add**: Signature-only declarations (1 page)
- **Note**: "Full implementations in Phase 4"

---

## Version Information

**Version**: 1.0
**Date**: 2026-01-31
**Phase**: Phase 1.6 Pre-Integration
**Next**: Apply fixes to specifications
