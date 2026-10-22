# Lang4LLM Test Files

This directory contains test files for the Lang4LLM compiler.

## Test Files

### `valid_syntax_test.l4l`
Simple valid program for testing basic parsing and type checking.

**Purpose**: Verify basic compilation pipeline works

**Expected**: Should parse successfully (type errors may occur due to type checker bugs, which are separate issues)

### `error_reporting_test.l4l`
Test file with intentional errors to verify diagnostic quality.

**Purpose**: Test error reporting with various error types

**Expected**: Should produce clear, helpful error messages with source context

## Running Tests

### Parse a file
```bash
cargo build
./target/debug/lang4llm parse tests/valid_syntax_test.l4l
```

### Type-check a file
```bash
cargo build
./target/debug/lang4llm check tests/valid_syntax_test.l4l
```

## Expected Error Output

Error messages should include:
- File path and location (line:column)
- Source code context with highlighting
- Clear error description
- Helpful suggestions when applicable
- Color-coded output

Example:
```
Error: Type mismatch: expected i32, found bool
   ╭─[file.l4l:5:14]
   │
 5 │     let x: i32 = true;
   │              │
   │              ╰─ expected i32, found bool
   │
   │ Note: Expected a boolean value (true or false)
───╯
```
