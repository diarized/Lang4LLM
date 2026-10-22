# Spec: Phase 2 - Parser & Type Checker Implementation

**ID**: 002-parser-type-checker
**Phase**: 2
**Status**: Planning
**Dependencies**: Phase 1 Complete (Language Specification)
**Estimated Effort**: 4-6 weeks
**Target Language**: Rust

---

## Overview

Implement a parser and type checker for Lang4LLM based on the Phase 1 formal specification. The parser will convert Lang4LLM source code into an Abstract Syntax Tree (AST), and the type checker will validate type correctness and exhaustive pattern matching.

---

## Goals

1. **Parser**: Convert Lang4LLM source → AST
2. **Type Checker**: Validate types, exhaustive matching, initialization
3. **Error Reporting**: Clear, actionable error messages (DR-GRM-003)
4. **Validation**: All 12 Phase 1 example programs parse and type-check
5. **CLI Tool**: Command-line interface for parsing/checking Lang4LLM files

---

## Success Criteria

- ✅ Parser handles all grammar.ebnf constructs
- ✅ AST accurately represents all language features
- ✅ Type checker validates all type rules from types.md
- ✅ Exhaustive matching enforcement (DR-ERR-004)
- ✅ Mandatory initialization check (DR-SEM-004)
- ✅ All 12 example programs validate successfully
- ✅ Error messages show file/line/column with context
- ✅ CLI tool: `lang4llm check <file>`

---

## Technical Approach

### Parser Strategy

**Option 1: Parser Generator (Recommended)**
- Tool: LALRPOP (Rust parser generator, EBNF-like syntax)
- Pros: Direct grammar.ebnf translation, proven correctness
- Cons: Learning curve for LALRPOP syntax

**Option 2: Hand-Written Recursive Descent**
- Implementation: Manual LL(1) parser in Rust
- Pros: Full control, educational value
- Cons: More code, manual maintenance

**Decision**: Use LALRPOP for Phase 2 (faster, less error-prone)

### AST Design

**Principles**:
- One AST node type per grammar production
- Preserve source locations (file, line, column) for errors
- Typed AST (generic over type parameter during type checking)

**Structure**:
```rust
// Untyped AST (parser output)
enum Expr {
    Literal(Literal),
    Variable(Ident),
    BinaryOp { lhs: Box<Expr>, op: BinOp, rhs: Box<Expr> },
    // ... all expression types
}

// Typed AST (type checker output)
enum TypedExpr {
    Literal(Literal, Type),
    Variable(Ident, Type),
    BinaryOp { lhs: Box<TypedExpr>, op: BinOp, rhs: Box<TypedExpr>, ty: Type },
    // ... all expression types with types
}
```

### Type Checker Architecture

**Phases**:
1. **Symbol Resolution**: Build symbol table, resolve identifiers
2. **Type Inference** (limited): Generic type parameters from usage
3. **Type Checking**: Validate type annotations, check compatibility
4. **Exhaustive Matching**: Verify pattern completeness (DR-ERR-004)
5. **Initialization Check**: Ensure all variables initialized (DR-SEM-004)

**Algorithm**:
- Two-pass: first pass builds symbol table, second pass type-checks
- Environment-based scoping (stack of symbol tables)
- Type unification for generic parameters

---

## Deliverables

| Component | File | Description |
|-----------|------|-------------|
| **Parser** | parser/src/parser.lalrpop | LALRPOP grammar → AST |
| **AST** | ast/src/lib.rs | AST node definitions |
| **Type Checker** | typeck/src/lib.rs | Type validation logic |
| **CLI** | cli/src/main.rs | Command-line tool |
| **Tests** | tests/ | Parser/type checker tests |
| **Documentation** | docs/ | Usage guide, architecture |

---

## Implementation Plan

### Phase 2.1: Project Setup (1 day)
- Create Rust workspace with 4 crates: `ast`, `parser`, `typeck`, `cli`
- Set up LALRPOP dependency
- Configure cargo build
- Create basic directory structure

### Phase 2.2: AST Definition (2-3 days)
- Define AST nodes for all grammar productions
- Add source location tracking (Span)
- Implement Display traits for debugging
- Create AST traversal utilities

### Phase 2.3: Parser Implementation (1 week)
- Translate grammar.ebnf to LALRPOP
- Implement lexer (tokenization)
- Handle operator precedence
- Add error recovery
- Test with example programs

### Phase 2.4: Type Checker - Symbol Resolution (3-4 days)
- Build symbol table data structure
- Implement environment (scope stack)
- Resolve variable/function/type names
- Handle shadowing rules (no shadowing allowed)

### Phase 2.5: Type Checker - Type Validation (1 week)
- Implement type checking for expressions
- Validate function signatures
- Check struct/enum definitions
- Enforce explicit type annotations (DR-SEM-001)

### Phase 2.6: Type Checker - Advanced Features (1 week)
- Generic type parameter inference
- Trait bound checking
- Exhaustive pattern matching (DR-ERR-004)
- Mandatory initialization (DR-SEM-004)

### Phase 2.7: Error Reporting (3-4 days)
- Implement error types with source locations
- Format errors with file/line/column
- Show source code context
- Provide suggestions for common errors

### Phase 2.8: CLI Tool (2 days)
- Argument parsing (clap)
- File I/O
- Pretty-print errors
- Exit codes

### Phase 2.9: Testing & Validation (1 week)
- Unit tests for parser
- Unit tests for type checker
- Integration tests with 12 example programs
- Edge case testing
- Error message validation

### Phase 2.10: Documentation (3 days)
- Architecture overview
- API documentation
- Usage examples
- Testing guide

---

## Technology Stack

**Language**: Rust (stable)
**Parser**: LALRPOP (LR parser generator)
**CLI**: clap (argument parsing)
**Testing**: cargo test + custom test harness
**Error Handling**: miette (beautiful error diagnostics)

---

## Design Rule Compliance

| Rule | Requirement | Implementation |
|------|-------------|----------------|
| DR-GRM-001 | Context-Free Grammar | LALRPOP enforces CFG |
| DR-GRM-003 | Predictable Tokens | LL(1) compatible lexer |
| DR-SEM-001 | Explicit Types | Type checker enforces |
| DR-SEM-004 | Mandatory Init | Type checker enforces |
| DR-ERR-004 | Exhaustive Matching | Pattern completeness check |

---

## Testing Strategy

### Unit Tests
- Parser: Each grammar production
- Type Checker: Each type rule
- AST: Node construction, traversal

### Integration Tests
- All 12 Phase 1 example programs
- Edge cases: empty files, syntax errors, type errors
- Error messages: validate format and content

### Property-Based Tests (stretch)
- Generate random valid Lang4LLM programs
- Verify parser always succeeds
- Verify type checker catches intentional errors

---

## Phase 2 Gate Criteria

- [ ] Parser handles all grammar constructs
- [ ] AST preserves all source information
- [ ] Type checker validates all type rules
- [ ] Exhaustive matching enforced
- [ ] Mandatory initialization enforced
- [ ] All 12 example programs validate
- [ ] Error messages show context
- [ ] CLI tool works: `lang4llm check <file>`
- [ ] Test coverage > 80%
- [ ] Documentation complete

---

## Risk Mitigation

**Risk**: LALRPOP learning curve
**Mitigation**: Start with simple grammar subset, iterate

**Risk**: Type checker complexity (generics)
**Mitigation**: Implement incrementally, test each feature

**Risk**: Poor error messages
**Mitigation**: Use miette library, test error output

**Risk**: Performance issues on large files
**Mitigation**: Profile early, optimize hot paths

---

## Future Work (Phase 3+)

- **Phase 3**: Transpiler (typed AST → Rust code)
- **Phase 4**: Standard library implementation
- **Phase 5**: LLM validation testing with benchmarks

---

## Version Information

**Version**: 1.0
**Date**: 2026-01-31
**Phase**: Phase 2 Planning
**Status**: Ready to start implementation
**Dependencies**: Phase 1 Complete (grammar.ebnf, types.md)
