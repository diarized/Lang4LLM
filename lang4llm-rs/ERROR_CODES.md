# Lang4LLM Error Codes

Stable error codes emitted by the Lang4LLM compiler (type checker and parser).
Available in `--format json` output under the `code` field.

---

## Type Errors (E0001–E0012)

### E0001 — TypeMismatch

**Meaning**: An expression's type does not match the expected type.

**Common causes**:
- Assigning a `bool` value to an `int` variable
- Returning a `string` from a function declared `-> int`
- Passing wrong argument types to a function

**Fix suggestions**:
- `int` ↔ `float`: use `int(value)` or `float(value)` conversions
- Any type → `string`: use `to_string(value)`
- `unit` type: use bare `return;` instead of `return expr;`

**Example**:
```
// error: expected int, found bool
let x: int = true;
```

---

### E0002 — UndefinedSymbol

**Meaning**: A name (type, enum variant, struct field) is used before being defined, or is defined more than once.

**Common causes**:
- Duplicate field names in a struct definition
- Duplicate variant names in an enum definition
- Referencing an enum variant that doesn't exist

**Fix suggestions**: The compiler shows "did you mean `X`?" suggestions based on edit distance when a close name exists.

---

### E0003 — MissingTypeAnnotation

**Meaning**: A variable binding lacks a required type annotation.

**Common causes**:
```
let x = 42;  // missing `: int`
```

**Fix**: Add an explicit type annotation: `let x: int = 42;`

---

### E0004 — Uninitialized

**Meaning**: A variable is used before being assigned a value.

**Fix**: Initialize the variable at declaration or assign before use.

---

### E0005 — InvalidBinaryOp

**Meaning**: A binary operator is applied to unsupported types.

**Common causes**:
- Arithmetic on `bool` values
- Comparing incompatible types

**Note**: String concatenation uses `+` and is supported via `to_string()` conversion.

---

### E0006 — ArgumentCountMismatch

**Meaning**: A function call provides the wrong number of arguments.

**Common causes**:
- Calling `add(1, 2, 3)` when `add` expects 2 arguments
- Forgetting a required argument

**Multi-span**: The error report shows both the call site and the function definition.

---

### E0007 — GenericMismatch

**Meaning**: Generic type arguments don't match the expected type parameters.

---

### E0008 — NonExhaustiveMatch

**Meaning**: A `match` expression doesn't cover all possible values of the matched type.

**Fix**: Add missing pattern arms or add a wildcard `_` arm.

---

### E0009 — TraitBoundViolation

**Meaning**: A type used where a trait bound is required doesn't implement that trait.

---

### E0010 — FieldNotFound

**Meaning**: A struct field access uses a field name that doesn't exist in the struct.

**Common causes**:
- Typo in field name: `point.z` when struct has fields `x` and `y`

---

### E0011 — Symbol (Undefined / Shadowing / WrongKind)

**Meaning**: A symbol lookup error. Covers three sub-cases:

- **Undefined**: Variable or function referenced but not in scope
- **Shadowing**: Redeclaring a variable in the same scope is not allowed in Lang4LLM
- **WrongKind**: Using a type name as a value or vice versa

**Fix for Shadowing**: Use a different variable name instead of re-declaring.

---

### E0012 — Exhaustiveness (NonExhaustive / UnknownVariant)

**Meaning**: Match exhaustiveness check failure.

- **NonExhaustive**: Not all enum variants are covered
- **UnknownVariant**: A pattern references a variant that doesn't exist in the enum

**Fix for NonExhaustive**: Add the missing variant patterns or a wildcard `_` arm.

---

## Parse Errors (E1001–E1002)

### E1001 — LexError

**Meaning**: The lexer could not tokenize the input.

**Common causes**:
- Unterminated string literal (missing closing `"`)
- Invalid character in source
- Using `&&` instead of `and`, `||` instead of `or`, `!` instead of `not`

**Lang4LLM-specific operator keywords**:
| Wrong | Correct |
|-------|---------|
| `&&`  | `and`   |
| `\|\|` | `or`  |
| `!x`  | `not x` |
| `null` | `none` |

---

### E1002 — SyntaxError

**Meaning**: The parser encountered an unexpected token.

**Common causes**:
- Missing semicolons after statements
- Incorrect keyword usage
- Malformed function signatures

---

## Error Code Summary

| Code  | Category      | Variant                  |
|-------|---------------|--------------------------|
| E0001 | Type          | TypeMismatch             |
| E0002 | Type          | UndefinedSymbol          |
| E0003 | Type          | MissingTypeAnnotation    |
| E0004 | Type          | Uninitialized            |
| E0005 | Type          | InvalidBinaryOp          |
| E0006 | Type          | ArgumentCountMismatch    |
| E0007 | Type          | GenericMismatch          |
| E0008 | Type          | NonExhaustiveMatch       |
| E0009 | Type          | TraitBoundViolation      |
| E0010 | Type          | FieldNotFound            |
| E0011 | Symbol        | Undefined/Shadowing/WrongKind |
| E0012 | Exhaustiveness | NonExhaustive/UnknownVariant |
| E1001 | Parse         | LexError                 |
| E1002 | Parse         | SyntaxError              |

---

## JSON Output Format

When using `--format json`, errors are serialized as:

```json
{
  "success": false,
  "errors": [
    {
      "code": "E0001",
      "severity": "error",
      "message": "Type mismatch: expected int, found bool",
      "file": "path/to/file.lang4llm",
      "span": {
        "start_line": 3,
        "start_col": 18,
        "end_line": 3,
        "end_col": 22
      },
      "suggestion": "use to_string(value) to convert bool to string",
      "related": [
        {
          "message": "`add` defined here",
          "span": { "start_line": 1, "start_col": 1, "end_line": 1, "end_col": 20 }
        }
      ]
    }
  ],
  "result": null
}
```

The `related` array carries secondary spans (e.g. function definition site for E0006).
The `suggestion` field provides actionable fix hints when available.
