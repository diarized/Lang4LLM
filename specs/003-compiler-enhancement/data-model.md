# Data Model: Phase 6 — Compiler Enhancement

**Branch**: `003-compiler-enhancement` | **Date**: 2026-02-28

## Overview

This feature does not introduce new persistent data or storage. All entities are in-memory
compiler data structures. This document describes the entities that change or are introduced.

---

## Entity: `FnReturnTypeMap`

**What it is**: A lookup table from function name to return type, built from the top-level
function definitions in a `Program` before transpilation begins.

**Where it lives**: `CodegenContext` in `transpiler/src/context.rs`

**Fields**:
```
fn_return_types: HashMap<String, Type>
  key:   function name (String)
  value: return type (lang4llm_ast::Type)
```

**Lifecycle**:
- Created in `transpiler::transpile()` by scanning `program.items`
- Immutable throughout the codegen pass
- Consulted by `contains_string_expr()` in `exprs.rs` when checking if a `Call` returns `string`

**Constraints**:
- Only top-level named functions are indexed (not closures, lambdas, or methods)
- User-defined functions only; builtins are handled by the existing `is_string_returning_builtin`
  check (unchanged)

---

## Entity: `GenericBound` (augmented)

**What it is**: A constraint on a generic type parameter (`T: Clone`). Already exists in the
AST as `GenericParamDef.bound: Option<Ident>`.

**Change**: The transpiler's `emit_function` now synthesizes an implicit `Clone` bound for every
generic type parameter, merged with any user-declared bounds, before emitting the `where` clause.

**Effective where-clause semantics**:
```
declared bounds    +  implicit Clone
─────────────────────────────────────
T: Display         →  T: Display + Clone
U: (none)          →  U: Clone
V: Clone           →  V: Clone          (de-duplicated)
```

**Representation** (in `emit_function`, no AST change):
```
where_clauses: Vec<(String, Vec<String>)>
  each entry: (type_param_name, [bound1, bound2, ...])
  guaranteed: "Clone" is always present for every generic param
```

---

## Entity: `Span` (propagated)

**What it is**: Source location `(start_line, start_col, end_line, end_col)`. Already exists in
`ast/src/span.rs`. Currently `Span::dummy()` (all zeros) is used everywhere in the grammar.

**Change**: `Ident` nodes in the AST will carry real spans from the lexer instead of `Span::dummy()`.

**Token change**:
```
Before:  TokenKind::Ident(String)        — span discarded
After:   TokenKind::Ident(String, Span)  — span preserved from lexer
```

**Propagation**:
- Lexer already computes `Span` for every token. The `Ident` token will now embed it.
- Grammar `Ident` rule unpacks `(name, span)` → constructs `Ident { name, span }` with real location.
- All grammar rules that construct AST nodes with an `Ident` name field will inherit the real span
  from that `Ident` (function definitions, variable declarations, type names, etc.).

**Non-change**: Expression spans (`Expr`, `Stmt`, `BinaryExpr`) remain `Span::dummy()` in
Phase 6. They will be addressed in Phase 7.

---

## Entity: `contains` Builtin — Overload

**What it is**: The `contains` builtin in the typechecker's symbol table.

**Current state**: Registered as `(string, string) -> bool` only.

**Change**: The typechecker will accept `contains(arr: [T], val: T) -> bool` by relaxing the
type check when the first argument is an array type.

**Dispatch logic** (typechecker side):
```
contains(arg0, arg1):
  if type(arg0) == string → check arg1: string (existing behavior)
  if type(arg0) == [T]    → accept, return bool (new behavior)
  else → TypeError
```

**Transpiler side**: Already correct (emits `arr.contains(&x)` for any 2-argument call).

---

## No New Entities

- No new crates, no new files (existing files modified)
- No new AST node types
- No database schema, no serialization format changes
- No changes to the `lang4llm_compatible` JSON sidecar format
