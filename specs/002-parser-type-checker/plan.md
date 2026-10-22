# Phase 2 Implementation Plan: Parser & Type Checker

**Feature**: Parser and Type Checker for Lang4LLM
**Phase**: 2
**Estimated Effort**: 4-6 weeks
**Target**: Rust implementation

---

## Architecture

```
lang4llm-rs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── ast/                # AST definitions
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # AST node types
│   │       ├── span.rs     # Source location tracking
│   │       └── visitor.rs  # AST traversal
│   ├── parser/             # Parser implementation
│   │   ├── Cargo.toml
│   │   ├── build.rs        # LALRPOP build script
│   │   └── src/
│   │       ├── lib.rs      # Parser API
│   │       ├── grammar.lalrpop  # LALRPOP grammar
│   │       └── lexer.rs    # Tokenizer
│   ├── typeck/             # Type checker
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # Type checker API
│   │       ├── symtab.rs   # Symbol table
│   │       ├── typecheck.rs # Type validation
│   │       ├── exhaustive.rs # Pattern completeness
│   │       └── errors.rs   # Type errors
│   └── cli/                # Command-line tool
│       ├── Cargo.toml
│       └── src/
│           └── main.rs     # CLI entry point
├── tests/
│   ├── parser_tests.rs     # Parser integration tests
│   ├── typeck_tests.rs     # Type checker tests
│   └── examples/           # 12 Lang4LLM example programs
└── docs/
    ├── architecture.md     # System design
    └── usage.md            # User guide
```

---

## Tech Stack

- **Language**: Rust 1.70+
- **Parser**: LALRPOP (parser generator)
- **Error Reporting**: miette + ariadne (beautiful diagnostics)
- **CLI**: clap v4 (argument parsing)
- **Testing**: cargo test + integration tests
- **CI**: GitHub Actions (lint, test, build)

---

## File Structure

### Workspace `Cargo.toml`
```toml
[workspace]
members = ["crates/ast", "crates/parser", "crates/typeck", "crates/cli"]
resolver = "2"

[workspace.dependencies]
lalrpop-util = "0.20"
miette = { version = "7.0", features = ["fancy"] }
ariadne = "0.4"
clap = { version = "4.5", features = ["derive"] }
```

### AST Module (`crates/ast/`)
**Purpose**: Define AST node types, source spans, visitors

**Key Types**:
- `Program` (top-level)
- `Item` (functions, structs, enums, traits, impls)
- `Stmt` (let, assignment, expression, return, etc.)
- `Expr` (literals, variables, binary ops, function calls, etc.)
- `Type` (primitive, array, tuple, generic, etc.)
- `Pattern` (literal, identifier, wildcard, tuple, struct, enum)
- `Span` (file, start line/col, end line/col)

### Parser Module (`crates/parser/`)
**Purpose**: Tokenize source code, parse into AST

**Components**:
- `grammar.lalrpop`: LALRPOP grammar (translates grammar.ebnf)
- `lexer.rs`: Tokenizer (keywords, identifiers, literals, operators)
- `lib.rs`: Public API (`parse_file`, `parse_str`)

### Type Checker Module (`crates/typeck/`)
**Purpose**: Validate types, check exhaustiveness, enforce rules

**Components**:
- `symtab.rs`: Symbol table (scopes, name resolution)
- `typecheck.rs`: Type validation logic
- `exhaustive.rs`: Pattern matching completeness
- `errors.rs`: Type error types with source locations

### CLI Module (`crates/cli/`)
**Purpose**: Command-line tool for parsing/checking

**Commands**:
- `lang4llm check <file>` - Parse and type-check
- `lang4llm parse <file>` - Parse only (show AST)
- `lang4llm --version` - Show version

---

## Implementation Phases

### Phase 2.1: Project Setup ✅
**Goal**: Create Rust workspace structure

**Tasks**:
1. Create workspace with 4 crates
2. Configure LALRPOP dependency
3. Set up CI/CD (GitHub Actions)
4. Create basic README

**Validation**: `cargo build` succeeds

---

### Phase 2.2: AST Definition
**Goal**: Define all AST node types

**Tasks**:
1. Define `Span` type (file, line, column)
2. Define `Expr` enum (all expression types)
3. Define `Stmt` enum (all statement types)
4. Define `Type` enum (all type annotations)
5. Define `Pattern` enum (all pattern types)
6. Define `Item` enum (functions, structs, enums, traits, impls)
7. Define `Program` struct (top-level)
8. Implement `Display` traits for debugging
9. Create AST visitor trait

**Validation**: AST nodes compile, display correctly

---

### Phase 2.3: Lexer Implementation
**Goal**: Tokenize Lang4LLM source code

**Tasks**:
1. Define `Token` enum (keywords, identifiers, literals, operators)
2. Implement keyword recognition
3. Implement identifier lexing
4. Implement number literals (int, float)
5. Implement string literals (with escape sequences)
6. Implement operators and delimiters
7. Handle comments (line, block)
8. Track source locations (Span)
9. Error handling (invalid tokens)

**Validation**: Tokenize all 12 example programs

---

### Phase 2.4: Parser Implementation (LALRPOP)
**Goal**: Convert tokens → AST

**Tasks**:
1. Translate grammar.ebnf to LALRPOP syntax
2. Implement operator precedence rules
3. Parse expressions (literals, binary ops, function calls, etc.)
4. Parse statements (let, assignment, return, etc.)
5. Parse items (functions, structs, enums, traits, impls)
6. Parse types (primitives, arrays, tuples, generics)
7. Parse patterns (all pattern types)
8. Error recovery (continue parsing after error)
9. Attach source spans to AST nodes

**Validation**: Parse all 12 example programs, produce AST

---

### Phase 2.5: Symbol Table
**Goal**: Resolve names, build symbol table

**Tasks**:
1. Define `Symbol` type (name, type, location)
2. Define `Scope` type (stack of symbol tables)
3. Implement scope push/pop
4. Resolve variable references
5. Resolve function references
6. Resolve type references (structs, enums)
7. Enforce no shadowing (DR-SEM-003)
8. Handle generics (type parameters)

**Validation**: All names resolve correctly in examples

---

### Phase 2.6: Type Checking - Basics
**Goal**: Validate basic type rules

**Tasks**:
1. Type-check literals (int, float, bool, string)
2. Type-check variables (lookup in symbol table)
3. Type-check binary operators (+, -, *, /, %, ==, etc.)
4. Type-check function calls (argument types match parameters)
5. Type-check assignments (type compatibility)
6. Type-check return statements (match function return type)
7. Enforce explicit type annotations (DR-SEM-001)
8. Enforce mandatory initialization (DR-SEM-004)

**Validation**: Basic examples type-check correctly

---

### Phase 2.7: Type Checking - Advanced
**Goal**: Handle generics, traits, pattern matching

**Tasks**:
1. Type-check generic functions (monomorphization)
2. Type-check generic types (Option<T>, Result<T,E>)
3. Type-check trait bounds (T: Trait)
4. Type-check pattern matching (all arms type-compatible)
5. Enforce exhaustive matching (DR-ERR-004)
6. Type-check struct/enum construction
7. Type-check field access
8. Type-check array indexing (bounds checks are runtime)

**Validation**: All 12 examples type-check

---

### Phase 2.8: Error Reporting
**Goal**: Generate helpful error messages

**Tasks**:
1. Define error types (ParseError, TypeError)
2. Attach source spans to errors
3. Format errors with file/line/column
4. Show source code context (ariadne)
5. Provide suggestions for common errors
6. Color-coded output (miette)
7. Multiple error reporting (don't stop at first error)

**Validation**: Error messages are clear, actionable

---

### Phase 2.9: CLI Tool
**Goal**: Command-line interface

**Tasks**:
1. Argument parsing (clap)
2. `check` command: parse + type-check
3. `parse` command: parse only
4. File I/O (read Lang4LLM files)
5. Pretty-print AST (optional debug mode)
6. Pretty-print errors
7. Exit codes (0 = success, 1 = error)
8. `--version` flag

**Validation**: CLI works on example programs

---

### Phase 2.10: Testing
**Goal**: Comprehensive test coverage

**Tasks**:
1. Unit tests: AST construction
2. Unit tests: Lexer (all token types)
3. Unit tests: Parser (all grammar productions)
4. Unit tests: Type checker (all type rules)
5. Integration tests: All 12 examples parse + type-check
6. Integration tests: Intentional errors (syntax, type)
7. Integration tests: Error message validation
8. Benchmark: Parser performance

**Validation**: All tests pass, coverage > 80%

---

### Phase 2.11: Documentation
**Goal**: Usage and architecture docs

**Tasks**:
1. README: Installation, usage examples
2. `docs/architecture.md`: System design
3. `docs/usage.md`: CLI guide
4. API documentation (rustdoc)
5. CONTRIBUTING.md: Developer guide

**Validation**: Documentation complete, clear

---

## Task Dependencies

```
Setup (2.1)
  ↓
AST (2.2)
  ↓
Lexer (2.3)
  ↓
Parser (2.4) → Error Reporting (2.8)
  ↓              ↓
Symbol Table (2.5) → CLI (2.9)
  ↓              ↓
Type Check Basic (2.6) → Testing (2.10)
  ↓              ↓
Type Check Advanced (2.7) → Docs (2.11)
```

**Parallel opportunities**:
- Error reporting can be developed alongside type checking
- CLI can be developed once parser is functional
- Documentation can be written incrementally

---

## Validation Checkpoints

### Checkpoint 1: Parser Complete
- [ ] All 12 examples parse without errors
- [ ] AST structure matches grammar.ebnf
- [ ] Source spans attached to all nodes
- [ ] Parser handles syntax errors gracefully

### Checkpoint 2: Type Checker Complete
- [ ] All 12 examples type-check without errors
- [ ] Generics work correctly
- [ ] Exhaustive matching enforced
- [ ] Mandatory initialization enforced

### Checkpoint 3: Integration Complete
- [ ] CLI tool works: `lang4llm check <file>`
- [ ] Error messages are helpful
- [ ] Test coverage > 80%
- [ ] Documentation complete

---

## Phase 2 Gate Criteria

- [ ] Parser handles all grammar constructs (100% grammar.ebnf coverage)
- [ ] AST accurately represents source code
- [ ] Type checker validates all type rules (types.md compliance)
- [ ] Exhaustive matching enforcement (DR-ERR-004)
- [ ] Mandatory initialization enforcement (DR-SEM-004)
- [ ] All 12 Phase 1 examples validate successfully
- [ ] Error messages show context with file/line/column
- [ ] CLI tool: `lang4llm check <file>` works
- [ ] Test coverage > 80%
- [ ] Documentation complete (architecture, usage, API)

**Total**: 10 gate criteria

---

## Estimated Timeline

| Phase | Duration | Effort (hours) |
|-------|----------|----------------|
| 2.1 Setup | 1 day | 8 |
| 2.2 AST | 2-3 days | 16-24 |
| 2.3 Lexer | 3-4 days | 24-32 |
| 2.4 Parser | 1 week | 40 |
| 2.5 Symbol Table | 3-4 days | 24-32 |
| 2.6 Type Check Basic | 1 week | 40 |
| 2.7 Type Check Advanced | 1 week | 40 |
| 2.8 Error Reporting | 3-4 days | 24-32 |
| 2.9 CLI | 2 days | 16 |
| 2.10 Testing | 1 week | 40 |
| 2.11 Documentation | 3 days | 24 |
| **Total** | **4-6 weeks** | **296-376 hours** |

---

## Success Metrics

1. **Correctness**: All 12 examples parse and type-check
2. **Error Quality**: Users can fix errors without consulting docs
3. **Performance**: Parse 1000-line file in < 100ms
4. **Test Coverage**: > 80% line coverage
5. **Documentation**: Complete API docs + user guide

---

## Version Information

**Version**: 1.0
**Date**: 2026-01-31
**Phase**: Phase 2 Planning
**Status**: Ready to implement
**Dependencies**: Phase 1 Complete
