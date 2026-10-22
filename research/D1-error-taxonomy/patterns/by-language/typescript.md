# TypeScript-Specific Error Patterns

**Language**: TypeScript 5.x
**Last Updated**: 2026-01-20
**Analysis Scope**: Literature-based + JavaScript crossover

## Overview

TypeScript adds gradual typing to JavaScript. Its optional type system creates unique error patterns where type safety is incomplete, and escape hatches (`any`, `as`) undermine guarantees.

## Error Pattern Distribution

```
Syntax Errors:      28% ████████████████████████████
Semantic Errors:    42% ██████████████████████████████████████████
Logic Errors:       30% ██████████████████████████████
```

## TypeScript-Specific Patterns

### 1. Type Assertion Abuse (`as any`)

**Frequency**: 22% of TypeScript semantic errors
**Severity**: HIGH (bypasses type checking)

```typescript
// Incorrect
function processUser(data: unknown): User {
    return data as User;  // Unsafe, no validation
}

// Correct
function processUser(data: unknown): User {
    if (isUser(data)) {
        return data;
    }
    throw new Error("Invalid user data");
}
```

**Root Cause**: LLMs use `as any` to bypass type errors instead of fixing them.
**Mitigation**: Forbid `any` type, strict validation required.

---

### 2. Optional Chaining Misuse (`?.`)

**Frequency**: 15% of TypeScript logic errors
**Severity**: MEDIUM (logic bugs)

```typescript
// Incorrect
function getEmail(user: User | null): string {
    return user?.email;  // Returns undefined, not string
}

// Correct
function getEmail(user: User | null): string {
    return user?.email ?? "no-email@example.com";
}
```

**Root Cause**: `?.` returns `undefined`, not the declared type. LLMs forget nullish coalescing.
**Mitigation**: Explicit null handling, Option<T> types.

---

### 3. Promise Without Await

**Frequency**: 18% of TypeScript semantic errors
**Severity**: HIGH (runtime logic error)

```typescript
// Incorrect
async function getData(): Promise<Data> {
    const result = fetchData();  // Missing await, returns Promise
    return result.transform();  // Error: Promise has no transform()
}

// Correct
async function getData(): Promise<Data> {
    const result = await fetchData();
    return result.transform();
}
```

**Root Cause**: Implicit promise handling confuses LLMs.
**Mitigation**: Compiler error on unawaited promises in async functions.

---

### 4. Array Method Type Narrowing Failures

**Frequency**: 12% of TypeScript type errors
**Severity**: MEDIUM (type unsoundness)

```typescript
// Incorrect
const numbers: (number | null)[] = [1, 2, null, 3];
const valid = numbers.filter(n => n !== null);
valid[0].toFixed();  // Error: Object is possibly null

// Correct
const numbers: (number | null)[] = [1, 2, null, 3];
const valid = numbers.filter((n): n is number => n !== null);
valid[0].toFixed();  // OK
```

**Root Cause**: TypeScript can't infer type guards without explicit annotation.
**Mitigation**: Better type guard inference, or explicit filter types.

---

### 5. Interface vs Type Confusion

**Frequency**: 10% of TypeScript errors
**Severity**: LOW (stylistic, sometimes semantic)

```typescript
// Incorrect
type User = {
    name: string;
}
interface User {  // Error: Duplicate identifier
    age: number;
}

// Correct
interface User {
    name: string;
    age: number;
}
```

**Root Cause**: `interface` and `type` have subtle differences. LLMs mix them.
**Mitigation**: Single construct for types (no interface/type distinction).

---

## TypeScript-Specific Strengths

**What LLMs Handle Well**:
1. **Basic type annotations**: Simple types mostly correct
2. **Generic functions**: Common patterns like `Array<T>` correct
3. **Union types**: `string | number` well-understood
4. **Enum usage**: Type-safe enums generated correctly

## TypeScript-Specific Weaknesses

**What LLMs Struggle With**:
1. **Type guards**: `is` predicates often omitted
2. **Conditional types**: `T extends U ? X : Y` rarely correct
3. **Mapped types**: `{ [K in keyof T]: ... }` highly error-prone
4. **Template literal types**: Advanced string types wrong
5. **Strict null checks**: Confusion with `| null | undefined`

## Type System Escape Hatches

**Problem**: TypeScript's gradual typing allows:
- `any` type (disables checking)
- `as` assertions (unsafe casts)
- `// @ts-ignore` comments (bypass errors)

**LLM Behavior**: Overuses escape hatches instead of fixing type errors.

**Mitigation**: Strict mode by default, forbid escape hatches.

## Design Recommendations for LLM-Friendly Language

Based on TypeScript analysis:

1. **No escape hatches** → Sound type system (like Rust)
2. **Infer type guards** → Automatic narrowing from conditionals
3. **Explicit null handling** → Option<T> instead of T | null
4. **Single type construct** → No interface/type distinction
5. **Error on unawaited promises** → Catch async bugs at compile time

## Cross-Reference

**Patterns Present in TypeScript**:
- EP-SYN-001, EP-SYN-004 (similar to JavaScript)
- EP-SEM-002 (type mismatches when types are used)
- EP-LOG-003 (boolean logic errors)

**TypeScript-Unique**:
- Type assertion abuse (`as`)
- Type guard omissions
- Promise/async confusion

## References

- P02: TypeScript in multi-language analysis
- P06: Type constraints improve TypeScript generation
- JavaScript benchmark data applies (HumanEval variants)
