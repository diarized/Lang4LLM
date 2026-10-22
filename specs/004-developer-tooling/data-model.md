# Data Model: Phase 7 — Developer Tooling

**Branch**: `004-developer-tooling` | **Date**: 2026-03-02

---

## Entity Changes

### 1. SourceMap (NEW)

Pre-computed line-start byte offset table for byte→line:col conversion.

| Field | Type | Description |
|-------|------|-------------|
| `line_starts` | `Vec<usize>` | Byte offset of each line start (0-indexed). `line_starts[0] = 0` always. |

**Methods**:
- `from_source(src: &str) -> SourceMap` — scan source for `\n` positions
- `resolve(byte_offset: usize) -> (usize, usize)` — binary search on `line_starts`, return (1-indexed line, 1-indexed col)
- `span(start: usize, end: usize) -> Span` — convenience: `resolve(start)` + `resolve(end)` → `Span`

**Location**: `crates/ast/src/source_map.rs` (new file in ast crate — no new dependencies)

---

### 2. Span (MODIFIED)

Existing struct in `crates/ast/src/span.rs`. No field changes.

**New constructor**:
- `Span::from_offsets(source_map: &SourceMap, start: usize, end: usize) -> Span`

---

### 3. TypeError::UndefinedSymbol (MODIFIED)

| Field | Type | Change |
|-------|------|--------|
| `name` | `String` | Existing |
| `span` | `Span` | Existing |
| `candidates` | `Vec<String>` | **NEW** — similar names with edit distance <= 2 |

---

### 4. DiagnosticOutput (NEW)

Top-level JSON output envelope for `--format json`.

| Field | Type | Description |
|-------|------|-------------|
| `success` | `bool` | `true` if no errors |
| `errors` | `Vec<DiagnosticError>` | All errors |
| `result` | `Option<CommandResult>` | Command-specific output |

---

### 5. DiagnosticError (NEW)

Single error in JSON output.

| Field | Type | Description |
|-------|------|-------------|
| `code` | `String` | Stable error code (e.g., `"E0001"`) |
| `severity` | `String` | `"error"` or `"warning"` |
| `message` | `String` | Human-readable error text |
| `file` | `String` | Source file path |
| `span` | `SpanInfo` | Start/end line:col |
| `suggestion` | `Option<String>` | Fix suggestion if available |
| `related` | `Vec<RelatedInfo>` | Secondary spans (function defs, etc.) |

---

### 6. SpanInfo (NEW)

JSON-serializable span.

| Field | Type | Description |
|-------|------|-------------|
| `start_line` | `usize` | 1-indexed |
| `start_col` | `usize` | 1-indexed |
| `end_line` | `usize` | 1-indexed |
| `end_col` | `usize` | 1-indexed |

---

### 7. RelatedInfo (NEW)

Secondary diagnostic information (e.g., "function defined here").

| Field | Type | Description |
|-------|------|-------------|
| `message` | `String` | Context message |
| `span` | `SpanInfo` | Location |

---

### 8. CommandResult (NEW, enum)

Per-command result data in JSON output.

| Variant | Fields | When |
|---------|--------|------|
| `Check` | `{}` | `lang4llm check` success |
| `Parse` | `{ items_count: usize }` | `lang4llm parse` success |
| `Compile` | `{ output_file: String }` | `lang4llm compile` success |
| `Format` | `{ formatted: bool, files_changed: Vec<String> }` | `lang4llm fmt` result |

---

### 9. SymbolTable (MODIFIED)

Existing struct in `crates/typeck/src/symtab.rs`.

**New method**:
- `pub fn all_names_in_scope(&self) -> Vec<&str>` — iterates all scopes, returns all symbol names currently visible. Used for "did you mean?" candidate computation.

---

## Entity Relationship Summary

```
SourceMap ──builds──> Span (via from_offsets)
    │
    ├── used by grammar.lalrpop (@L/@R → source_map.span())
    └── used by ParseError conversion (byte offset → Span)

TypeError ──carries──> Span
    │                    │
    │                    └── converted to SpanInfo for JSON
    │
    └── UndefinedSymbol.candidates ← SymbolTable.all_names_in_scope()

DiagnosticOutput
    ├── errors: Vec<DiagnosticError>
    │       ├── span: SpanInfo
    │       ├── suggestion: Option<String>
    │       └── related: Vec<RelatedInfo>
    │               └── span: SpanInfo
    └── result: Option<CommandResult>
```
