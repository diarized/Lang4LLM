# Feature Specification: Phase 6 — Compiler Enhancement

**Feature Branch**: `003-compiler-enhancement`
**Created**: 2026-02-28
**Status**: Draft

## User Scenarios & Testing *(mandatory)*

### User Story 1 — String Concatenation Works Without Workarounds (Priority: P1)

A Lang4LLM developer writes natural string concatenation using `+`. When one operand is a `string` and the other is a value returned from a user-defined function, the compiler produces valid output without requiring the developer to wrap the call in `to_string()`.

**Why this priority**: This is the most common workaround imposed on benchmark reference implementations. Eliminating it directly improves generated code quality and removes a footgun that LLMs frequently stumble on.

**Independent Test**: Can be fully tested by compiling `let s = "result: " + my_fn()` where `my_fn` returns `string`, and verifying the output program runs correctly.

**Acceptance Scenarios**:

1. **Given** a Lang4LLM program where `my_fn()` has return type `string`, **When** the developer writes `"prefix: " + my_fn()`, **Then** the compiler emits valid output without errors and the result is the concatenated string.
2. **Given** a program where `arr[i]` has type `string` and is used with `+`, **When** the program is compiled, **Then** no manual `to_string()` wrapper is required.
3. **Given** a program that previously required `to_string(user_fn())` as a workaround, **When** the workaround is removed, **Then** the program still compiles and produces identical output.

---

### User Story 2 — Generic Bounds Propagated to Output (Priority: P1)

A Lang4LLM developer writes a generic function with type bounds (e.g., `fn f<T: Clone>(x: T) -> T`). The compiler emits output that preserves the bound, allowing programs using generics to compile and run correctly.

**Why this priority**: The known failing test `compile_08_closures` is blocked by this. It is a soundness gap: the compiler accepts programs it cannot correctly emit.

**Independent Test**: Compile a program containing `fn identity<T: Clone>(x: T) -> T { return x; }` and call it with a value; verify the output compiles without `trait bound not satisfied` errors.

**Acceptance Scenarios**:

1. **Given** a function `fn f<T: Clone>(x: T) -> T`, **When** compiled, **Then** the emitted output contains the `Clone` bound on `T`.
2. **Given** the `compile_08_closures` benchmark, **When** run through the full eval harness, **Then** it passes (moves from failing to passing status).
3. **Given** a generic function with multiple bounds (e.g., `T: Clone + Display`), **When** compiled, **Then** all bounds appear in the output.

---

### User Story 3 — `contains` Works on Arrays (Priority: P2)

A Lang4LLM developer uses `contains(arr, value)` to check whether an array contains a specific element, without writing a manual helper loop.

**Why this priority**: The current limitation forces verbose workarounds in every program needing membership testing. Extending the builtin is a contained change with high ergonomic payoff.

**Independent Test**: Compile and run `let found = contains([1, 2, 3], 2)` and verify `found` is `true`.

**Acceptance Scenarios**:

1. **Given** `contains([1, 2, 3], 2)`, **When** evaluated, **Then** the result is `true`.
2. **Given** `contains(["a", "b", "c"], "d")`, **When** evaluated, **Then** the result is `false`.
3. **Given** `contains(string_var, substring)` (existing string behavior), **When** compiled, **Then** behavior is unchanged.
4. **Given** an empty array `contains([], value)`, **When** evaluated, **Then** the result is `false`.

---

### User Story 4 — Accurate Error Locations in Diagnostics (Priority: P2)

When a Lang4LLM program has a type error, the compiler reports the line and column of the actual error, not `1:1`. The developer can navigate directly to the problem.

**Why this priority**: Accurate spans are table stakes for usable diagnostics. Line 1:1 for every error is actively misleading on programs longer than a few lines.

**Independent Test**: Write a program with a deliberate type error on line 10 and verify the error message cites line 10.

**Acceptance Scenarios**:

1. **Given** a type mismatch on line 15 column 8, **When** the typechecker reports the error, **Then** the error message includes `15:8` (or equivalent line reference).
2. **Given** an undefined variable on line 3, **When** reported, **Then** the error cites line 3, not `1:1`.
3. **Given** a program with multiple errors, **When** all errors are reported, **Then** each error references its own distinct location.

---

### Edge Cases

- What happens when both sides of `+` are ambiguous at compile time (neither is a string literal)? The compiler falls back to integer addition; no behavior change.
- What happens when `contains` is called with a `[string]` array and a `string` element? Should work via the array path, not the substring path.
- What happens when a generic function has zero type bounds? The existing behavior (no bounds emitted) is preserved.
- What happens when span information is missing for a node (e.g., synthesized AST nodes)? The compiler emits a best-effort span or falls back to the enclosing function's span rather than `1:1`.
- What if `compile_08_closures` exercises features beyond generic bounds? Those failures are out of scope for this phase; the spec only covers the bounds propagation gap.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The compiler MUST resolve the return type of user-defined function calls at the call site and use that type information when determining how to emit `+` expressions.
- **FR-002**: The compiler MUST emit string concatenation (rather than numeric addition) when either operand of `+` has a statically-known `string` type, regardless of whether that operand is a literal, variable, array access, or function call.
- **FR-003**: The compiler MUST propagate all type bounds declared on generic type parameters in function signatures into the emitted output.
- **FR-004**: The `contains` builtin MUST accept a `[T]` array as its first argument and a `T` value as its second, returning `bool` indicating element membership.
- **FR-005**: The `contains` builtin MUST continue to accept `(string, string)` arguments with unchanged substring-search semantics.
- **FR-006**: The typechecker MUST attach source span information (line, column) to every AST node during parsing and propagate those spans through type-checking passes.
- **FR-007**: All diagnostic messages produced by the typechecker MUST include the source span of the relevant AST node, accurate to at least line number.
- **FR-008**: All 92 currently-passing benchmark tests MUST continue to pass after all changes.
- **FR-009**: The `compile_08_closures` benchmark MUST pass after the generic bounds fix is applied.
- **FR-010**: The Lang4LLM surface syntax (grammar, keywords, operator set) MUST NOT change.

### Key Entities

- **Span**: Source location metadata (file, line, column, length) attached to AST nodes; used by diagnostics to report error positions.
- **TypeEnv**: The compiler's type environment mapping identifiers to their types; must be queryable during transpilation to resolve operand types for `+`.
- **GenericBound**: A constraint on a type parameter (e.g., `T: Clone`); must survive from parse through to output emission.
- **Builtin Dispatch**: The mechanism that selects the builtin implementation based on argument types; must be extended to route `contains([T], T)` to array membership logic.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 93 benchmark tests pass (92 existing + `compile_08_closures`); zero regressions among previously-passing tests.
- **SC-002**: All reference implementations that previously required `to_string(user_fn())` workarounds compile and produce correct output without those wrappers.
- **SC-003**: Error messages for type errors in programs longer than 5 lines cite the correct line number in 100% of cases (no `1:1` fallback for non-synthetic nodes).
- **SC-004**: `contains(array, element)` returns correct results for integer, float, and string arrays in all tested cases.
- **SC-005**: No change to the Lang4LLM grammar or any existing test behavior outside the four fixed gaps.

---

## Assumptions

- Span information can be added to AST nodes without breaking existing parser consumers; the AST crate is owned within this workspace.
- The typechecker's environment is accessible from the transpiler (or the transpiler can be passed type annotations alongside the AST); the current architecture supports this without a major refactor.
- "Generic bounds" in scope means trait bounds on type parameters in function signatures only — not where-clauses, associated types, or higher-ranked trait bounds.
- `contains([T], T)` equality uses structural equality matching the language's `==` semantics; no custom comparator is needed.
- Compile-time constant folding and dead code elimination (gap 5 from the description) are explicitly deferred to a later phase; this spec does not cover them.
- The `compile_08_closures` test failure is solely due to missing bound propagation. If additional issues are found during implementation, they are tracked separately.
