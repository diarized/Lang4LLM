# Compiler Interface Contracts: Phase 7 — Developer Tooling

**Branch**: `004-developer-tooling` | **Date**: 2026-03-02

---

## Contract 1: SourceMap API

**Crate**: `lang4llm-ast`
**File**: `crates/ast/src/source_map.rs`

```
SourceMap::from_source(src: &str) -> SourceMap
  PRE:  src is valid UTF-8
  POST: line_starts[0] == 0; line_starts.len() == (number of \n in src) + 1
  PERF: O(n) where n = src.len()

SourceMap::resolve(byte_offset: usize) -> (line: usize, col: usize)
  PRE:  byte_offset <= src.len()
  POST: line >= 1, col >= 1 (1-indexed)
  POST: if byte_offset == 0 => (1, 1)
  POST: if byte_offset > src.len() => clamp to last valid position
  PERF: O(log L) where L = number of lines (binary search)

SourceMap::span(start: usize, end: usize) -> Span
  POST: span.start_line >= 1, span.end_line >= 1
  POST: if start > end => swap to canonical form
```

---

## Contract 2: Grammar Span Threading

**Crate**: `lang4llm-parser`
**File**: `crates/parser/src/grammar.lalrpop`

```
grammar<'src>(source_map: &'src SourceMap);

// Every AST-producing rule uses @L/@R:
FunctionDef: FunctionDef = {
    <start:@L> "fn" <name:Ident> ... <end:@R> => {
        FunctionDef { ..., span: source_map.span(start, end) }
    }
}
```

**Contract**:
- Every grammar rule producing an AST node with a `span: Span` field MUST use `source_map.span(@L, @R)` instead of `Span::dummy()`
- After this change, `Span::dummy()` MUST NOT appear anywhere in `grammar.lalrpop`
- The `parse()` function signature changes to accept `&SourceMap` as a parameter
- All call sites of `parse()` must build a `SourceMap` first and pass it through

---

## Contract 3: JSON Output Schema

**Crate**: `lang4llm-cli`
**Trigger**: `--format json` flag on any command

### Success Response

```json
{
  "success": true,
  "errors": [],
  "result": { <command-specific> }
}
```

### Error Response

```json
{
  "success": false,
  "errors": [
    {
      "code": "E0001",
      "severity": "error",
      "message": "Type mismatch: expected int, found string",
      "file": "example.lang4llm",
      "span": {
        "start_line": 15,
        "start_col": 8,
        "end_line": 15,
        "end_col": 22
      },
      "suggestion": "use to_string(value) to convert",
      "related": [
        {
          "message": "variable declared here with type int",
          "span": {
            "start_line": 3,
            "start_col": 5,
            "end_line": 3,
            "end_col": 15
          }
        }
      ]
    }
  ],
  "result": null
}
```

### Invariants

- `success == (errors.len() == 0)`
- `code` is always a string matching pattern `E[0-9]{4}`
- `severity` is always `"error"` (no warnings in current compiler)
- `span` fields are 1-indexed; `0:0` means "unknown location"
- `suggestion` is `null` when no fix is available
- `related` is an empty array `[]` when no related info exists
- Output is always valid JSON, even on internal compiler errors
- Exit code: 0 on success, 1 on compilation errors, 2 on internal errors

---

## Contract 4: Formatter API

**Crate**: `lang4llm-formatter`
**File**: `crates/formatter/src/lib.rs`

```
pub fn format_source(source: &str) -> Result<String, Vec<ParseError>>
  PRE:  source is valid UTF-8
  POST on Ok: output is syntactically valid Lang4LLM
  POST on Ok: parse(output) produces structurally equivalent AST to parse(source)
  POST on Ok: format_source(output) == Ok(output)  (idempotency)
  POST on Err: source has syntax errors; ParseError list is returned

pub fn format_file(path: &Path) -> Result<FormatResult, io::Error>
  POST: FormatResult { changed: bool, original: String, formatted: String }

pub fn check_formatted(path: &Path) -> Result<bool, io::Error>
  POST: returns true if file is already canonically formatted
```

---

## Contract 5: Enhanced TypeError

**Crate**: `lang4llm-typeck`

### UndefinedSymbol with candidates

```
TypeError::UndefinedSymbol {
    name: String,
    span: Span,
    candidates: Vec<String>,   // NEW — edit distance <= 2
}

INVARIANT: candidates.len() <= 5 (limit to avoid noise)
INVARIANT: candidates sorted by edit distance (closest first)
```

### ArgumentCountMismatch with definition span

```
TypeError::ArgumentCountMismatch {
    expected: usize,
    found: usize,
    span: Span,           // call site
    def_span: Span,       // NEW — function definition location
    fn_name: String,      // NEW — for display
}
```

---

## Contract 6: CLI Command Additions

**Crate**: `lang4llm-cli`

### Global flag

```
--format <FORMAT>   Output format [default: human] [possible values: human, json]
```

Applies to: `check`, `parse`, `compile`, `eval`

### New subcommand: `fmt`

```
lang4llm fmt [OPTIONS] <PATH>...

Arguments:
  <PATH>...  Files or directories to format

Options:
  --check    Check if files are formatted without modifying them
  --write    Write formatted output back to files (default: stdout for single file)
```

**Behavior**:
- Single file, no `--write`: print formatted output to stdout
- Single file, `--write`: overwrite file in place
- Directory: recurse for `*.lang4llm` files, implies `--write`
- `--check`: exit 0 if all files formatted, exit 1 if any would change; list unformatted files on stderr

---

## Test Contracts

### Span Accuracy Tests

For each error type, provide a `.lang4llm` test file with a known error at a specific location. Assert that the reported span matches the expected line:col range.

```
test_span_binary_type_error:
  INPUT:  "fn main() {\n    let x: int = \"hello\" + 5;\n}\n"
  ERROR:  E0005 at span 2:19-2:32  (the `"hello" + 5` expression)

test_span_undefined_var:
  INPUT:  "fn main() {\n    print(countr);\n}\n"
  ERROR:  E0002 at span 2:11-2:17  (the `countr` identifier)

test_span_arg_count:
  INPUT:  "fn foo(x: int) -> int { return x; }\nfn main() { foo(1, 2); }\n"
  ERROR:  E0006 at span 2:13-2:22  (the `foo(1, 2)` call)
```

### JSON Output Tests

```
test_json_type_error:
  INPUT:  program with type error
  FLAGS:  --format json
  ASSERT: stdout parses as valid JSON
  ASSERT: $.success == false
  ASSERT: $.errors[0].code matches /E[0-9]{4}/
  ASSERT: $.errors[0].span.start_line > 0

test_json_success:
  INPUT:  valid program
  FLAGS:  --format json
  ASSERT: $.success == true
  ASSERT: $.errors == []
```

### Formatter Tests

```
test_fmt_idempotent:
  FOR EACH file in research/D4-benchmark/reference/**/*.lang4llm:
    formatted = format_source(read(file))
    ASSERT: format_source(formatted) == formatted

test_fmt_semantic_preservation:
  FOR EACH file in research/D4-benchmark/reference/**/*.lang4llm:
    formatted = format_source(read(file))
    ASSERT: compile_and_run(formatted).stdout == compile_and_run(read(file)).stdout
```
