# Feature Specification: Phase 7 — Developer Tooling

**Feature Branch**: `004-developer-tooling`
**Created**: 2026-03-02
**Status**: Draft
**Input**: User description: "Phase 7"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Accurate Error Location Reporting (Priority: P1)

A developer (or LLM agent) writes a Lang4LLM program containing a type error deep inside a nested expression. The compiler currently points to line 1, column 1 for many errors because most AST nodes carry dummy source spans. After this feature, every error message pinpoints the exact line and column where the problem occurs, with a source code snippet and underline highlighting the offending expression.

**Why this priority**: Without accurate spans, all other tooling (diagnostics, formatter layout decisions, structured output) lacks the foundational location data. Phase 6 explicitly deferred expression-level spans to Phase 7.

**Independent Test**: Can be fully tested by compiling programs with known errors at specific locations and verifying the reported line:column matches the actual error site. Delivers immediate value: developers stop guessing where errors are.

**Acceptance Scenarios**:

1. **Given** a program with a type mismatch in a binary expression on line 15, **When** `lang4llm check` runs, **Then** the error message references line 15 with the correct column range, not line 1:1.
2. **Given** a program with an undefined variable inside an if-else branch on line 30, **When** `lang4llm check` runs, **Then** the error underlines the specific variable reference, not the entire statement.
3. **Given** a program with a nested function call where an argument has the wrong type on line 8 column 20, **When** `lang4llm check` runs, **Then** the error span covers the mismatched argument expression, not the enclosing call.

---

### User Story 2 - Machine-Readable Error Output (Priority: P2)

An LLM code generation pipeline compiles Lang4LLM code and needs to parse errors programmatically to feed corrections back to the model. Currently, errors are only human-readable strings. After this feature, all compiler commands support a `--format json` flag that emits structured error output with file path, span (line, column, byte offset), error code, message, and optional fix suggestions.

**Why this priority**: The project's core mission is LLM-friendly code generation. Structured error output closes the feedback loop — LLMs can parse errors and self-correct. This is the highest-value differentiator for the target audience.

**Independent Test**: Can be fully tested by running the compiler with `--format json` on programs with known errors and validating the JSON output against a schema. Delivers value: any LLM integration can parse and act on compiler errors.

**Acceptance Scenarios**:

1. **Given** a program with two type errors, **When** `lang4llm check --format json` runs, **Then** stdout contains a JSON array of two error objects, each with `file`, `span`, `code`, `message`, and `severity` fields.
2. **Given** a valid program, **When** `lang4llm check --format json` runs, **Then** stdout contains an empty error array and a `success: true` field.
3. **Given** a program with a syntax error, **When** `lang4llm compile --format json` runs, **Then** the JSON output includes the error with a `suggestion` field containing a human-readable fix hint.
4. **Given** a malformed source file (empty, binary content), **When** `lang4llm check --format json` runs, **Then** a single error object is returned with an appropriate error code, not a crash or unstructured panic.

---

### User Story 3 - Enhanced Diagnostic Messages (Priority: P3)

A developer learning Lang4LLM makes a common mistake (e.g., using `!` instead of `not`, forgetting a type annotation, passing wrong number of arguments). The compiler already shows basic error messages with ariadne formatting, but suggestions are generic. After this feature, diagnostics include context-aware "did you mean?" suggestions, related information spans (e.g., "function defined here expects 2 arguments"), and Lang4LLM-specific guidance referencing design rules.

**Why this priority**: Builds on P1 (accurate spans) to add contextual information. Improves both human and LLM experience — LLMs benefit from precise fix suggestions as much as humans do.

**Independent Test**: Can be tested by compiling programs with each common error pattern and verifying the diagnostic output contains the expected suggestion text. Delivers value: shorter debugging cycles for both humans and LLMs.

**Acceptance Scenarios**:

1. **Given** a program using `&&` instead of `and`, **When** `lang4llm check` runs, **Then** the error message suggests "use `and` instead of `&&`".
2. **Given** a program calling `foo(x, y, z)` where `foo` takes 2 parameters, **When** `lang4llm check` runs, **Then** the error shows both the call site span AND the function definition span ("function `foo` defined here takes 2 parameters").
3. **Given** a program with an undefined variable `countr`, **When** `lang4llm check` runs and a variable `counter` exists in scope, **Then** the error suggests "did you mean `counter`?".
4. **Given** a program assigning an integer to a string variable, **When** `lang4llm check` runs, **Then** the error suggests "use `to_string(value)` to convert".

---

### User Story 4 - Source Code Formatter (Priority: P4)

A developer (or LLM) produces Lang4LLM code with inconsistent formatting — irregular indentation, missing spaces around operators, inconsistent newlines. The `lang4llm fmt` command reformats the source to a canonical style, making all Lang4LLM code uniform. This is especially valuable for LLM-generated code which often has formatting inconsistencies.

**Why this priority**: Formatting normalization reduces noise in LLM training data and benchmark comparisons. Lower priority because it doesn't affect correctness and the language is still young. However, establishing a canonical style early prevents ecosystem fragmentation.

**Independent Test**: Can be tested by formatting known input files and comparing against expected output files. Delivers value: all Lang4LLM code looks the same regardless of who (or what) wrote it.

**Acceptance Scenarios**:

1. **Given** a syntactically valid program with 2-space indentation, **When** `lang4llm fmt` runs, **Then** the output uses 4-space indentation throughout.
2. **Given** a valid program, **When** `lang4llm fmt` runs twice consecutively, **Then** the second run produces identical output to the first (idempotency).
3. **Given** a program with a syntax error, **When** `lang4llm fmt` runs, **Then** the formatter reports the syntax error and exits without modifying the file.
4. **Given** a valid program formatted with `lang4llm fmt`, **When** the formatted output is compiled, **Then** it produces identical runtime behavior to the original (semantic preservation).
5. **Given** a directory of `.lang4llm` files, **When** `lang4llm fmt --check <dir>` runs, **Then** it reports which files would change without modifying them (dry-run mode).

---

### Edge Cases

- What happens when a source file is empty? The compiler should report "empty source file" rather than crash.
- What happens when `--format json` is combined with `--verbose`? JSON output should include verbose details in a `diagnostics` field, not mixed with human-readable text.
- What happens when the formatter encounters a file with mixed line endings (CRLF/LF)? The formatter should normalize to LF.
- What happens when a deeply nested expression has multiple overlapping errors? Each error should have its own non-overlapping span, with the most specific span preferred.
- What happens when the JSON output contains non-ASCII characters in error messages? JSON output must be valid UTF-8 with proper escaping.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: All AST node types MUST carry accurate source spans (start line:column to end line:column) derived from the parser, not dummy values.
- **FR-002**: All error messages MUST reference the smallest AST node that identifies the error location, not a parent node or dummy span.
- **FR-003**: All compiler commands (`check`, `parse`, `compile`, `eval`) MUST support a `--format json` flag for structured output.
- **FR-004**: JSON error output MUST include: file path, span (start/end line and column), error code (string identifier), severity (error/warning), message (human-readable), and optional suggestion.
- **FR-005**: JSON output for successful operations MUST include a `success` field and relevant result data (e.g., AST summary for `parse`, generated file path for `compile`).
- **FR-006**: Error diagnostics MUST provide context-aware suggestions for at least: wrong boolean operators (`!`/`&&`/`||` vs `not`/`and`/`or`), missing type annotations, argument count mismatches, common typos in identifiers, and type conversion hints.
- **FR-007**: Argument count mismatch errors MUST show both the call site and the function definition location (multi-span diagnostic).
- **FR-008**: Undefined symbol errors MUST suggest similarly-named symbols in scope when the edit distance is 2 or fewer.
- **FR-009**: The `lang4llm fmt` command MUST reformat valid source files to a canonical style.
- **FR-010**: The formatter MUST be idempotent (formatting already-formatted code produces identical output).
- **FR-011**: The formatter MUST preserve semantic meaning (formatted code compiles and runs identically to the original).
- **FR-012**: The formatter MUST reject syntactically invalid files with an error instead of producing malformed output.
- **FR-013**: The formatter MUST support `--check` mode that reports whether files need formatting without modifying them.
- **FR-014**: The formatter MUST support formatting individual files and directories of `.lang4llm` files.
- **FR-015**: JSON output MUST assign a stable error code to each error category (e.g., `E0001` for type mismatch, `E0002` for undefined symbol) documented in a reference table.

### Key Entities

- **Source Span**: A range in source code defined by start line:column and end line:column, attached to every AST node. The fundamental unit of location tracking.
- **Diagnostic**: A structured compiler message containing severity, error code, message text, primary span, optional secondary spans (related information), and optional suggestions.
- **Error Code**: A stable string identifier (e.g., `E0001`) mapping to an error category, documented for programmatic consumption.
- **Formatting Rule**: A deterministic transformation from parsed AST back to source text, defining the canonical style (indentation depth, spacing, line breaks).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of type errors and parse errors reported by the compiler include accurate source location (no dummy 1:1 spans), verified by running the full benchmark suite with intentionally broken variants.
- **SC-002**: JSON error output is parseable by a standard JSON parser for every error the compiler can produce, verified by a comprehensive error corpus test.
- **SC-003**: At least 80% of common error patterns (as catalogued in D1 Error Taxonomy) include a context-aware fix suggestion in the diagnostic output.
- **SC-004**: The formatter produces identical output on the second run for 100% of the existing benchmark reference implementations (idempotency).
- **SC-005**: All formatted benchmark reference implementations pass their existing eval tests (semantic preservation).
- **SC-006**: Every error code is documented with its meaning, common causes, and suggested fixes.

## Assumptions

- The LALRPOP parser can be modified to propagate spans through all grammar productions without significant performance degradation.
- The ariadne diagnostic library (already a dependency) supports multi-span labels for related-information diagnostics.
- The canonical formatting style will use 4-space indentation (matching the transpiler's Rust output) and follow conventions established in the benchmark reference implementations.
- The formatter operates on the parsed AST (parse-then-print), not on raw text manipulation, ensuring only syntactically valid transformations.
- Error codes will follow a numeric scheme (`E0001`-`E9999`) grouped by category (parse errors, type errors, etc.).
- "Did you mean?" suggestions use simple Levenshtein distance, not a more complex fuzzy matching algorithm.

## Out of Scope

- **Language Server Protocol (LSP)**: Deferred to a future phase. Requires persistent server process and incremental parsing.
- **REPL / Interactive mode**: Deferred. Requires expression-level evaluation, not just whole-program compilation.
- **Watch mode**: Deferred. File-watching and incremental recompilation are separate concerns.
- **Syntax highlighting definitions**: Deferred (TextMate/tree-sitter grammars for editors).
- **Auto-fix / code actions**: The formatter fixes style; the compiler suggests fixes in diagnostics but does not auto-apply them.
- **Warning-level diagnostics**: All current diagnostics are errors. Adding warnings (unused variables, etc.) is a future enhancement.

## Dependencies

- **Phase 6 (Complete)**: Compiler Enhancement — provides the starting codebase, partial span tracking for Ident nodes, and 92/92 benchmark pass rate.
- **LALRPOP parser**: Grammar modifications required for span propagation.
- **ariadne crate**: Already in use for diagnostic rendering; may need version update for multi-span features.
- **serde/serde_json**: Already workspace dependencies; needed for `--format json` output.
