# Phase 2.12: Parser & Type Checker Enhancement

**Date**: 2026-02-13
**Goal**: Implement missing features to make integration tests pass
**Trigger**: Integration tests identified 7 categories of implementation gaps

---

## Overview

Phase 2.10 Priority 3 created 34 integration tests that revealed expected gaps between Phase 1 specification and Phase 2 implementation. This phase systematically addresses those gaps.

**Target**: 12/12 integration test examples passing

---

## Implementation Gaps (From INTEGRATION_TEST_REPORT.md)

### Priority 1: Builtin Functions (BLOCKS 5/12 examples)

**Impact**: HIGH - Examples 01, 02, 04, 07 fail due to undefined `print` and `range`

**Missing Symbols**:
- `print(string) -> unit` - Standard output
- `range(int, int) -> Iterator<int>` - Range iterator constructor
- Potentially other stdlib functions

**Solution**:
1. Create builtin symbol table in type checker
2. Initialize with standard library function signatures
3. Pre-populate symbol table before type checking

**Estimated Effort**: 2-3 hours

**Files to Modify**:
- `crates/typeck/src/typecheck.rs` - Add builtin initialization
- `crates/typeck/src/builtins.rs` - New file for builtin definitions

---

### Priority 2: Closure Syntax (BLOCKS Example 08)

**Impact**: MEDIUM - One example fails

**Issue**: Line 422 - Closure parameter syntax not recognized
- **Expected**: `|param: Type| -> ReturnType { body }`
- **Status**: Grammar may define it, parser doesn't implement it

**Solution**:
1. Review LALRPOP grammar for closure rules
2. Implement closure expression parsing
3. Add closure type checking
4. Support closure inference (may defer to later phase)

**Estimated Effort**: 4-6 hours

**Files to Modify**:
- `crates/parser/src/grammar.lalrpop` - Closure parsing rules
- `crates/ast/src/ast.rs` - Closure AST node (may exist)
- `crates/typeck/src/typecheck.rs` - Closure type checking

---

### Priority 3: Range Operators (BLOCKS Example 12)

**Impact**: MEDIUM - One example fails

**Issue**: Line 372 - Range operators (`..`, `..=`) not supported
- **Expected**: `0..10`, `0..=10` in expressions
- **Status**: Grammar defines ranges, parser doesn't implement

**Solution**:
1. Add range operator tokens to lexer (may exist)
2. Implement range expression parsing
3. Add range type checking
4. Support range literals

**Estimated Effort**: 2-3 hours

**Files to Modify**:
- `crates/parser/src/lexer.rs` - Range tokens
- `crates/parser/src/grammar.lalrpop` - Range expressions
- `crates/ast/src/ast.rs` - Range AST node
- `crates/typeck/src/typecheck.rs` - Range type checking

---

### Priority 4: Trait Definitions (BLOCKS Example 09)

**Impact**: MEDIUM - One example fails

**Issue**: Line 357 - Trait generic parameters not parsed correctly
- **Expected**: `trait Trait<T> { ... }`
- **Status**: Trait parsing incomplete

**Solution**:
1. Review trait parsing in grammar
2. Fix generic parameter handling in traits
3. Implement trait type checking
4. Support trait bounds (may defer)

**Estimated Effort**: 6-8 hours

**Files to Modify**:
- `crates/parser/src/grammar.lalrpop` - Trait parsing
- `crates/ast/src/ast.rs` - Trait AST (may exist)
- `crates/typeck/src/typecheck.rs` - Trait type checking

---

### Priority 5: Complex Pattern Matching (BLOCKS Examples 03, 06, 10)

**Impact**: HIGH - Three examples fail

**Issue**: Lines 253, 404, 539 - Match arm patterns not fully supported
- **Expected**: Nested patterns, struct patterns, tuple patterns
- **Status**: Basic patterns work, complex patterns fail

**Solution**:
1. Audit pattern parsing in grammar
2. Implement missing pattern types
3. Add pattern type checking
4. Ensure exhaustiveness checking works for all patterns

**Estimated Effort**: 6-8 hours

**Files to Modify**:
- `crates/parser/src/grammar.lalrpop` - Pattern parsing
- `crates/ast/src/ast.rs` - Pattern AST variants
- `crates/typeck/src/typecheck.rs` - Pattern type checking
- `crates/typeck/src/exhaustive.rs` - Exhaustiveness for complex patterns

---

### Priority 6: Loop Control (BLOCKS Example 11)

**Impact**: LOW - One example fails

**Issue**: Line 431 - Loop/break/continue syntax issues
- **Expected**: `loop { ... break; }`
- **Status**: May be parsing as if/else instead of loop

**Solution**:
1. Debug loop statement parsing
2. Fix break/continue in loops
3. Add type checking for loop control flow

**Estimated Effort**: 2-3 hours

**Files to Modify**:
- `crates/parser/src/grammar.lalrpop` - Loop/break/continue
- `crates/typeck/src/typecheck.rs` - Loop control flow checking

---

### Priority 7: Generic Edge Cases (BLOCKS Example 05)

**Impact**: LOW - One example fails

**Issue**: Line 269 - Generic parameter parsing incomplete
- **Expected**: `fn func<T>(x: T) -> T { ... }`
- **Status**: Basic generics work, some edge cases fail

**Solution**:
1. Debug failing generic syntax
2. Fix edge case in generic parsing
3. Ensure generic type checking works

**Estimated Effort**: 3-4 hours

**Files to Modify**:
- `crates/parser/src/grammar.lalrpop` - Generic parameter edge cases
- `crates/typeck/src/typecheck.rs` - Generic type checking

---

## Implementation Strategy

### Phase A: Quick Wins (Priorities 1, 3, 6)

**Goal**: Get 5 more examples passing with minimal effort
**Estimated Time**: 6-9 hours

1. **Builtin Functions** (2-3 hours)
   - Unblocks Examples 01, 02, 04, 07
   - Quick win: +4 passing tests

2. **Range Operators** (2-3 hours)
   - Unblocks Example 12
   - Quick win: +1 passing test

3. **Loop Control** (2-3 hours)
   - Unblocks Example 11
   - Quick win: +1 passing test

**After Phase A**: 6/12 examples passing

---

### Phase B: Medium Complexity (Priorities 2, 7)

**Goal**: Add closure and generic support
**Estimated Time**: 7-10 hours

4. **Closure Syntax** (4-6 hours)
   - Unblocks Example 08
   - Moderate complexity

5. **Generic Edge Cases** (3-4 hours)
   - Unblocks Example 05
   - Debugging exercise

**After Phase B**: 8/12 examples passing

---

### Phase C: Complex Features (Priorities 4, 5)

**Goal**: Complete pattern matching and traits
**Estimated Time**: 12-16 hours

6. **Trait Definitions** (6-8 hours)
   - Unblocks Example 09
   - Complex: type system integration

7. **Complex Pattern Matching** (6-8 hours)
   - Unblocks Examples 03, 06, 10
   - Complex: multiple pattern types

**After Phase C**: 12/12 examples passing ✅

---

## Total Estimated Effort

**Minimum**: 25 hours
**Maximum**: 35 hours
**Target**: Complete in 3-4 focused sessions

---

## Success Criteria

- [ ] 12/12 integration test examples pass
- [ ] All 21 error detection tests still pass
- [ ] No regressions in existing 123 unit tests
- [ ] Code documented with inline comments
- [ ] Changes committed with clear messages

---

## Execution Plan

### Session 1: Phase A - Quick Wins
1. Implement builtin functions
2. Add range operators
3. Fix loop control
4. Run integration tests (expect 6/12 passing)

### Session 2: Phase B - Medium Complexity
5. Implement closure syntax
6. Fix generic edge cases
7. Run integration tests (expect 8/12 passing)

### Session 3: Phase C - Complex Features (Part 1)
8. Implement trait definitions
9. Run integration tests (expect 9/12 passing)

### Session 4: Phase C - Complex Features (Part 2)
10. Implement complex pattern matching
11. Run integration tests (expect 12/12 passing)
12. Verify no regressions

---

## Risk Mitigation

**Risk**: Changes break existing tests
**Mitigation**: Run full test suite after each priority

**Risk**: Parser conflicts in grammar
**Mitigation**: Test grammar changes incrementally

**Risk**: Type system complexity explodes
**Mitigation**: Start with minimal type checking, iterate

---

## Notes

- Integration tests provide immediate feedback
- Each priority is independent (can be done in any order)
- Phase A delivers quick value (50% pass rate)
- Some features may be easier/harder than estimated

---

**Created**: 2026-02-13
**Status**: Planning complete, ready to execute
**Next**: Start Phase A - Builtin Functions
