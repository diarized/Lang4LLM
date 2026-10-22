# Phase 1 Specification Fixes Required

**Date**: 2026-01-31
**Context**: Pre-Integration Review (Phase 1.6)
**Total Issues**: 18 (5 Critical, 5 Important, 5 Moderate, 3 Minor)

**Reference**: See PHASE1_DECISIONS.md for approved language design decisions

---

## 🔴 CRITICAL FIXES (Must Fix Before Phase 2)

### Fix #1: Add Range Syntax to Grammar

**Problem**: `syntax.md` documents slicing (`arr[0..5]`, `arr[0..=4]`) but grammar has no range operators.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line ~47 (after comparison_op): Add range operators
range_op ::= ".." | "..="

# Line ~136 (in primary_expression, after block_expression): Add range expression
| range_expression

# Line ~210 (after block_expression definition): Add new production
# Range expression (for slicing)
range_expression ::= expression? range_op expression?
```

**Validation**: Parse `arr[0..5]`, `arr[0..=4]`, `arr[..5]`, `arr[5..]` without errors.

---

### Fix #2: Add Closure/Lambda Syntax to Grammar

**Problem**: `semantics.md` shows `|x| x * 2` but grammar has no closure syntax.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line ~136 (in primary_expression, after range_expression): Add closure
| closure_expression

# Line ~211 (after range_expression definition): Add new production
# Closure expression (lambda)
closure_expression ::= "|" parameter_list? "|" expression
```

**Validation**: Parse `|x| x + 1`, `|a, b| a * b`, `|| 42` without errors.

---

### Fix #3: Add `loop` Keyword and Statement

**Problem**: `semantics.md` recommends `loop { ... }` but grammar doesn't support it.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line 26-33 (keyword list): Add "loop"
keyword ::= "let" | "mut" | "fn" | "return" | "if" | "else" | "match"
          | "for" | "in" | "while" | "loop" | "break" | "continue"
          | "struct" | "enum" | "impl" | "trait"
          | "import" | "export" | "module"
          | "true" | "false" | "none"
          | "int" | "float" | "bool" | "string" | "unit"
          | "and" | "or" | "not"
          | "as" | "is"

# Line 216-224 (in statement, after continue_statement): Add loop_statement
| loop_statement

# Line ~252 (after for_statement definition): Add new production
# Loop statement (explicit infinite loop)
loop_statement ::= "loop" block_expression
```

**Validation**: Parse `loop { if cond { break; } }` without errors.

---

### Fix #4: Remove Implicit Returns from Semantics

**Problem**: `semantics.md` shows implicit returns but decision is to require explicit `return`.

**Files to modify**: `semantics/semantics.md`

**Changes**:
1. **Remove lines 141-163** (implicit return section)
2. **Replace all implicit return examples** with explicit returns:
   - Line ~147-161: Remove entire "Implicit Returns (Rust-Style)" section
   - Search for `{ a }` pattern, replace with `{ return a; }`
   - Search for `{ b }` pattern, replace with `{ return b; }`

**Validation**: No examples show blocks returning values without `return` keyword.

---

### Fix #5: Add Enum Variant Construction to Grammar

**Problem**: Examples show `Option::Some(42)` but grammar can't parse enum construction.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line ~136 (in primary_expression, after closure_expression): Add enum variant
| enum_variant_literal

# Line ~212 (after closure_expression definition): Add new production
# Enum variant literal (construction)
enum_variant_literal ::= identifier "::" identifier ("(" expression_list ")")?
```

**Validation**: Parse `Option::Some(42)`, `Result::Ok(value)`, `Status::Pending` without errors.

---

## 🟡 IMPORTANT FIXES (Should Fix)

### Fix #6: Clarify Array Type Semantics

**Problem**: `types.md` says "fixed-size" but `memory.md` says maps to `Vec<T>` (growable).

**Files to modify**: `type-system/types.md`

**Changes**:
- **Line 145**: Change from "Fixed-size (size determined at creation)"
- **To**: "Dynamically-sized with capacity management (maps to Rust Vec<T>)"
- **Add line ~161**: "Note: Arrays can grow/shrink via `.push()`, `.pop()` methods (see stdlib)"

**Validation**: Consistency between types.md and memory.md.

---

### Fix #7: Define String Indexing Return Type

**Problem**: `syntax.md` says returns "single-character string", `types.md` says "character".

**Files to modify**: `type-system/types.md`, `syntax/syntax.md`

**Decision needed**: Does `s[i]` return `char` type or single-char `string`?

**Recommendation**: No `char` type, return single-char `string`.

**Changes**:
- **types.md line 116**: Change to "Indexing: `s[0]` returns single-character string (not char type)"
- **syntax.md line 525**: Keep "single-character string" (already correct)

**Validation**: All string indexing examples return `string` type.

---

### Fix #8: Add `self` Keyword to Grammar

**Problem**: Method examples show `self: Point` but grammar doesn't allow `self`.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line 26-33 (keyword list): Add "self"
| "self"

# Line 260-262 (parameter definition): Allow self
parameter ::= ("self" | identifier) ":" type_annotation
```

**Validation**: Parse `fn method(self: Point) -> int` without errors.

---

### Fix #9: Document Stdlib Signatures (Minimal)

**Problem**: Examples use `range()`, `len()`, `print()` without definitions.

**Files to modify**: Create new file `language-spec/stdlib-signatures.md`

**Content**:
```markdown
# Standard Library Signatures (Phase 1)

**Status**: Forward declarations only (implementations in Phase 4)

## Core Functions

### Range Iterator
```lang4llm
fn range(start: int, end: int) -> Iterator<int>;
```
Creates exclusive range `[start, end)`.

### Print Function
```lang4llm
fn print(msg: string) -> unit;
```
Prints message to stdout.

### Length Function
```lang4llm
fn len<T>(arr: [T]) -> int;
```
Returns array length.

### Enumerate Iterator
```lang4llm
fn enumerate<T>(arr: [T]) -> Iterator<(int, T)>;
```
Returns iterator of (index, value) pairs.
```

**Validation**: All example code references documented functions.

---

### Fix #10: Add Tuple Assignment to Grammar

**Problem**: `syntax.md` shows `(a, b) = (b, a)` but grammar doesn't support tuple patterns in assignment.

**Files to modify**: `grammar/grammar.ebnf`

**Changes**:
```ebnf
# Line 230 (assignment_statement): Allow tuple patterns
assignment_statement ::= (assignable_expression | tuple_pattern) "=" expression ";"
```

**Validation**: Parse `(a, b) = (b, a);` without errors.

---

## 🟢 MODERATE FIXES (Consider Fixing)

### Fix #11: Add Generics to Grammar and Type System

**Problem**: Generics marked "future" but decision is to include in Phase 1.

**Files to modify**: `grammar/grammar.ebnf`, `type-system/types.md`

**Grammar changes** (grammar.ebnf):
```ebnf
# Line ~87 (after type_annotation): Add generic type parameters
generic_params ::= "<" identifier ("," identifier)* ">"
trait_bounds ::= ":" identifier ("+" identifier)*

# Modify type_annotation to include generic types:
type_annotation ::= primitive_type
                  | array_type
                  | tuple_type
                  | generic_type
                  | struct_type
                  | enum_type
                  | function_type

generic_type ::= identifier generic_args?
generic_args ::= "<" type_annotation ("," type_annotation)* ">"

# Modify function_definition (line 258):
function_definition ::= "fn" identifier generic_params? "(" parameter_list? ")" "->" type_annotation block_expression

# Modify struct_definition (line 268):
struct_definition ::= "struct" identifier generic_params? "{" field_definition_list? "}"

# Modify enum_definition (line 278):
enum_definition ::= "enum" identifier generic_params? "{" variant_list "}"

# Modify trait_definition (line 288):
trait_definition ::= "trait" identifier generic_params? "{" trait_item* "}"

# Modify impl_block (line 298):
impl_block ::= "impl" generic_params? trait_name? "for" type_name generic_args? "{" impl_item* "}"
```

**Type system changes** (types.md):
- **Add new section** after line 531: "Generic Types and Type Parameters"
- **Content**: 200-300 lines covering:
  - Generic type parameters (`<T>`)
  - Trait bounds (`T: Trait`)
  - Monomorphization semantics
  - Type parameter inference rules
  - Examples: `Option<T>`, `Result<T,E>`, `Vec<T>`

**Effort**: 2-3 hours for grammar, 4-6 hours for type system specification.

---

### Fix #12: Remove/Clarify Type Inference Claims

**Problem**: Contradictory statements about type inference.

**Files to modify**: `type-system/types.md`

**Changes**:
- **Line 440-447**: Remove entire "Limited Inference" section
- **Replace with**: "Lang4LLM requires explicit types for all declarations (no inference)"
- **Line 323**: Keep error example (already correct)

**Validation**: No mention of type inference anywhere in specs.

---

### Fix #13: Clarify Wildcard Pattern Semantics

**Problem**: Unclear if wildcard `_` triggers error or warning.

**Files to modify**: `type-system/types.md`, `semantics/semantics.md`

**Changes**:
- **types.md line 596**: Change to "Wildcard `_` is VALID (makes match exhaustive)"
- **Add**: "Compiler WARNING if wildcard too broad (suggest explicit cases)"
- **semantics.md line 596**: Keep current text (already correct)

**Validation**: Consistent messaging about wildcard behavior.

---

### Fix #14: Clarify Block Return with Semicolons

**Problem**: Unclear what happens when last expression has semicolon.

**Files to modify**: `syntax/syntax.md`

**Changes**:
- **Add after line 662**:
```markdown
**Block Return Rules**:
- Last expression WITHOUT semicolon → block returns expression value
- Last expression WITH semicolon → becomes statement, block returns `unit`

**Example**:
```lang4llm
let x: int = { 42 };      // Returns 42 (no semicolon)
let y: unit = { 42; };    // Returns unit (semicolon makes it statement)
```
```

**Validation**: Clear rules for block return values.

---

### Fix #15: Distinguish `none` Keyword from `Option::None`

**Problem**: Potential confusion between `none` (unit) and `Option::None` (variant).

**Files to modify**: `type-system/types.md`

**Changes**:
- **Line 124**: Add note: "Note: `none` is the unit value, distinct from `Option::None` enum variant"
- **Line 264**: Add note: "Note: `Option::None` is enum variant (namespaced), not `none` keyword"

**Validation**: Clear distinction in documentation.

---

## 📝 MINOR FIXES (Documentation/Clarity)

### Fix #16: Harmonize Precedence Table Numbering

**Problem**: Different numbering styles in grammar vs syntax.

**Files to modify**: `grammar/grammar.ebnf`, `syntax/syntax.md`

**Changes**: Use consistent format (highest to lowest, 1-8) in both files.

---

### Fix #17: Standardize Error Terminology

**Problem**: Inconsistent use of "panic", "compile error", "runtime error".

**Files to modify**: All specs

**Standard terms**:
- **Compile error**: Static type/syntax errors (caught before runtime)
- **Runtime panic**: Bounds violations, assertion failures (abort program)
- **Compiler warning**: Non-fatal issues (code compiles)

**Search and replace** to use these consistently.

---

### Fix #18: Consider Moving Transpiler Algorithms

**Problem**: Implementation details in language spec.

**Files to modify**: `memory-model/memory.md`

**Decision**: Keep algorithms (useful as reference implementation guidance) OR move to Phase 2/3 docs.

**Recommendation**: Keep for Phase 1 (demonstrates feasibility).

---

## Execution Order

**Critical first** (blocks Phase 2):
1. Fix #1: Range syntax
2. Fix #2: Closure syntax
3. Fix #3: Loop keyword
4. Fix #4: Remove implicit returns
5. Fix #5: Enum construction

**Important next** (affects usability):
6. Fix #6-10: Clarifications and stdlib

**Moderate/Minor** (polish):
11. Fix #11-18: Generics, documentation cleanup

---

## Validation Checklist

After all fixes:
- [ ] Grammar parses all example code
- [ ] No contradictions between files
- [ ] All 20 design rules addressed
- [ ] Cross-references valid
- [ ] Examples demonstrate all features
- [ ] Terminology consistent

---

## Version Information

**Version**: 1.0
**Date**: 2026-01-31
**Phase**: Phase 1.6 Pre-Integration
**Total Effort**: Estimated 8-12 hours for all fixes
