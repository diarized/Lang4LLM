# Research: Phase 6 — Compiler Enhancement

**Branch**: `003-compiler-enhancement` | **Date**: 2026-02-28
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## Gap 1: String Concatenation — Root Cause & Design

### Finding

`contains_string_expr()` in `transpiler/src/exprs.rs:381` detects "string-ness" of a `+` operand
purely syntactically. It matches:
- `Expr::Literal(Literal::String(_))` ✓
- `Expr::Call` to a hardcoded set of known string-returning builtins ✓
- `Expr::Paren` / `Expr::Binary(Add)` (recursive) ✓

It does **not** match:
- `Expr::Call` to a user-defined function that returns `string`
- `Expr::Variable` bound to a `string` value (would need per-scope type info)
- `Expr::ArrayAccess` where the element type is `string`

### Decision

Extend `contains_string_expr` to also return `true` when an `Expr::Call` targets a user-defined
function whose return type is `string`.

**Rationale**: User-function calls are the documented pain point (the `to_string(user_fn())`
workaround). Variables are harder to resolve (require scope-threaded type environment) and are
not a documented workaround target — deferring variable-lookup to a future pass.

### Implementation Approach

1. In `transpiler/src/lib.rs`'s `transpile()`, pre-scan `program.items` to build
   `HashMap<String, Type>` mapping function names to their return types.
2. Thread this map through `CodegenContext` (add a `fn_return_types` field).
3. In `contains_string_expr`, add a match arm for `Expr::Call`:
   - If func is `Expr::Variable(ident)`, look up `ctx.fn_return_types.get(&ident.name)`
   - If the return type is `Type::Primitive(PrimitiveType::String, _)`, return `true`

**Alternatives considered**:
- Full type-annotated AST pass (two-phase compile): correct but requires architectural change — deferred
- Extend heuristic to `Variable` via scope-tracked types: too invasive for this phase

**Affected files**: `transpiler/src/context.rs`, `transpiler/src/exprs.rs`, `transpiler/src/lib.rs`

---

## Gap 2: Generic Bounds — Root Cause & Design

### Finding

`emit_call_arg()` in `transpiler/src/exprs.rs` unconditionally appends `.clone()` to all
`Variable` and `FieldAccess` arguments at call sites, including arguments of generic type `T`.
This generates code like `f(x.clone())` where `x: T`, but `T: Clone` is not emitted.

`emit_generic_params()` at `transpiler/src/items.rs:230` emits only the parameter names
(`<T, U>`) with no bounds. The `where_clauses` in `emit_function` only adds user-declared
bounds (from `GenericParamDef.bound` and `ParamName::GenericBound`).

The parser preserves bounds in `GenericParamDef.bound: Option<Ident>` — so `fn f<T: Clone>`
correctly stores `bound: Some(Ident("Clone"))`. The transpiler does emit these via the
`where_clauses` list. The issue is that `T` and `U` in `08_closures.lang4llm` have **no**
user-specified bounds, so no `Clone` is emitted, yet `.clone()` is called.

### Decision

Implicitly add `Clone` as a where-clause bound for every generic type parameter in a function,
in addition to any user-specified bounds. Since `emit_call_arg` always inserts `.clone()`, every
generic parameter must satisfy `Clone` regardless.

**Rationale**: The transpiler's `.clone()` insertion is a necessary correctness mechanism.
Requiring `Clone` on all generic params is the correct soundness fix without changing call-site
emission logic. This adds an implicit constraint that is always satisfied for the concrete types
the LLM generates.

**Affected files**: `transpiler/src/items.rs` (only `emit_function`, ~5 line change)

---

## Gap 3: `contains([T], T)` — Root Cause & Design

### Finding

**Transpiler**: Already handles the array case (line 269–286 of `transpiler/src/builtins.rs`).
It emits `arr.contains(&x)` for any 2-argument `contains` call, which is valid for both
`Vec<T>` and `String` in Rust (string literal `&String` coerces to `&str` via Deref).

**Typechecker**: Registers `contains` as `(string, string) -> bool` only
(`typeck/src/builtins.rs:422`). Calling `contains(arr, val)` where `arr: [int]` fails with a
type mismatch because the checker expects `string` for the first argument.

### Decision

Add a second registration for `contains([T], T) -> bool` in the typechecker's builtin table.
Since `SymbolTable.insert` rejects duplicates (name collision), the approach is to mark
`contains` as a special overloaded builtin and relax type checking for it, or use a dedicated
overload mechanism.

**Simpler alternative chosen**: In `typeck/src/typecheck.rs`'s expression checker, detect the
special case `contains(array_expr, _)` and skip type checking (accept any array as first arg).
This is already done for `push`, `pop`, `sort`, `reverse` which accept `[T]` but are registered
with a placeholder type.

**Affected files**: `typeck/src/builtins.rs` (update `contains` registration or add overload),
`typeck/src/typecheck.rs` (relax argument type check for `contains` when first arg is array)

---

## Gap 4: Error Spans — Root Cause & Design

### Finding

- **Lexer** (`parser/src/lexer.rs`): Correctly tracks line/col; all tokens carry real `Span` values.
- **Grammar** (`parser/src/grammar.lalrpop`): Uses `Span::dummy()` for all AST node construction.
  The `Ident` rule at line 1110 extracts only the `String` from `TokenKind::Ident(String)` and
  discards the span.
- **Typechecker**: `TypeError` variants all have `span: Span` fields and would emit correct
  locations if the AST nodes carried real spans.

### Decision

Phase 6 targets "accurate error spans" at the level of identifier nodes, which covers the
primary diagnostic use case (undefined variable, type mismatch on a named value, wrong call
site). Full expression-level spans are deferred.

**Implementation approach**:

1. Change `TokenKind::Ident(String)` → `TokenKind::Ident(String, Span)` in the lexer.
2. Update all lexer sites that emit `TokenKind::Ident(name)` to `TokenKind::Ident(name, span)`.
3. Update the grammar's `extern` block to match `"ident" => TokenKind::Ident(<(String, Span)>)`.
4. Update the `Ident` grammar rule to unpack `(name, span)` and use the real span.
5. For non-Ident AST nodes (statements, expressions), use the span of their leading identifier
   or keyword token as best-effort. This is a partial but significant improvement over `0:0`.

**Scope**: This phase covers only `Ident` nodes. Expression-level spans are addressed in Phase 7.

**Alternatives considered**:
- `@L`/`@R` byte offsets in LALRPOP: requires a byte-offset-to-line-col table at parse time;
  more invasive than embedding span in token kind variant.
- Full span annotation on all grammar rules: correct but hundreds of grammar actions to update;
  deferred to Phase 7.

**Affected files**: `parser/src/lexer.rs`, `parser/src/grammar.lalrpop`

---

## Verification Plan

All changes verified by:
1. `cargo test --workspace` — all tests pass (including `compile_08_closures`)
2. `lang4llm eval --suite ../research/D4-benchmark/reference/{simple,medium,complex}/` — 92/92 pass
3. Unit tests in `transpiler/src/exprs.rs` — string concat tests extended to cover user-fn case
4. Manual test programs for each gap
