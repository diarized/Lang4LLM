# Compiler Interface Contracts: Phase 6

**Branch**: `003-compiler-enhancement` | **Date**: 2026-02-28

These contracts define the internal function and type signatures that change in this phase.
They serve as implementation targets and regression guards.

---

## Contract 1: `CodegenContext` — `fn_return_types` Field

**Crate**: `lang4llm-transpiler`
**File**: `crates/transpiler/src/context.rs`

### Before
```rust
pub struct CodegenContext {
    output: String,
    indent_level: usize,
}
```

### After
```rust
pub struct CodegenContext {
    output: String,
    indent_level: usize,
    /// Return types of user-defined functions, for string-concat detection.
    pub fn_return_types: HashMap<String, Type>,
}
```

### Constructor change
```rust
// Before
impl CodegenContext {
    pub fn new() -> Self { ... }
}

// After
impl CodegenContext {
    pub fn new() -> Self { Self { output: String::new(), indent_level: 0, fn_return_types: HashMap::new() } }
    pub fn with_fn_types(fn_return_types: HashMap<String, Type>) -> Self { ... }
}
```

---

## Contract 2: `transpile()` — Pre-scan pass

**Crate**: `lang4llm-transpiler`
**File**: `crates/transpiler/src/lib.rs`

### Invariants (unchanged)
```rust
pub fn transpile(program: &Program) -> Result<String, TranspileError>
```

### Behavioral change
Before emitting items, the function now builds `fn_return_types`:
```
For each Item::Function(f) in program.items:
    fn_return_types.insert(f.name.name.clone(), f.return_type.clone())
```
This map is passed into `CodegenContext::with_fn_types(fn_return_types)`.

**No signature change** — consumers (`cli`, tests) call `transpile(program)` unchanged.

---

## Contract 3: `contains_string_expr()` — Extended detection

**Crate**: `lang4llm-transpiler`
**File**: `crates/transpiler/src/exprs.rs`

### Signature change
```rust
// Before: pure function, no context needed
fn contains_string_expr(expr: &Expr) -> bool

// After: requires context for fn return type lookup
fn contains_string_expr(expr: &Expr, ctx: &CodegenContext) -> bool
```

### New match arm added
```
Expr::Call(c):
  if func is Expr::Variable(ident) AND !is_builtin(&ident.name):
    if ctx.fn_return_types.get(&ident.name) == Some(Type::Primitive(PrimitiveType::String, _)):
      return true
```

### Callers to update
- `is_string_concat_chain(expr)` → `is_string_concat_chain(expr, ctx)`
- `emit_binary(b, ctx)` → passes `ctx` to `is_string_concat_chain`

---

## Contract 4: `emit_function()` — Implicit `Clone` bound

**Crate**: `lang4llm-transpiler`
**File**: `crates/transpiler/src/items.rs`

### Behavioral change (no signature change)
After building `where_clauses` from `f.generics` and `ParamName::GenericBound`, add `Clone`
to every generic type param's bounds if not already present:

```
For each param in f.generics.params:
    let name = param.name.name.clone()
    let entry = where_clauses entry for name (create if missing)
    if entry.bounds does not contain "Clone":
        entry.bounds.push("Clone")
```

**Effect**: `fn map_array<T, U>(...)` emits:
```rust
fn map_array<T, U>(...)
where
    T: Clone,
    U: Clone,
{ ... }
```

---

## Contract 5: `typeck/builtins.rs` — `contains` overload

**Crate**: `lang4llm-typeck`
**File**: `crates/typeck/src/builtins.rs`

### Change
The `contains` symbol is registered with a marker indicating it is overloaded.
Alternatively: register it with a generic/array-accepting type that the typechecker's
`check_builtin_call` recognizes specially.

**Chosen approach**: Add a dedicated check in `typecheck.rs` for the `contains` builtin call:
```
check_call("contains", [arg0, arg1]):
  t0 = infer_type(arg0)
  if t0 == string:
    check arg1 is string; return bool
  if t0 == [T]:
    infer_type(arg1)  // just ensure it's well-typed; no constraint
    return bool
  else:
    TypeError::TypeMismatch
```

No change to the `Symbol` registration structure.

---

## Contract 6: `TokenKind::Ident` — Span embedding

**Crate**: `lang4llm-parser`
**File**: `crates/parser/src/lexer.rs`

### Before
```rust
Ident(String),
```

### After
```rust
Ident(String, Span),
```

### Lexer emission change
All sites producing `TokenKind::Ident(name)` → `TokenKind::Ident(name, span)` where `span`
is `Span::new(start_line, start_col, self.line, self.col)` captured at lex time.

---

## Contract 7: Grammar `Ident` rule — Real span

**Crate**: `lang4llm-parser`
**File**: `crates/parser/src/grammar.lalrpop`

### Before
```lalrpop
Ident: Ident = {
    <s:"ident"> => Ident { name: s, span: Span::dummy() }
};
```

### After
```lalrpop
Ident: Ident = {
    <tok:"ident"> => Ident { name: tok.0, span: tok.1 }
};
```

The `extern` block token definition must change from:
```lalrpop
"ident" => TokenKind::Ident(<String>),
```
to:
```lalrpop
"ident" => TokenKind::Ident(<(String, Span)>),
```

---

## Invariants Preserved

All contracts above preserve:
- `transpile(program: &Program) -> Result<String, TranspileError>` — unchanged public API
- `check_program(program: &Program) -> Result<(), Vec<TypeError>>` — unchanged public API
- `parse(source: &str) -> Result<Program, ParseError>` — unchanged public API
- All 92 currently-passing benchmark tests
- All existing unit tests in `transpiler/src/exprs.rs`
