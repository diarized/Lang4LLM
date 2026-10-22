# Semantic Error Patterns

**Category**: SEMANTIC
**Last Updated**: 2026-01-20
**Status**: Initial taxonomy (4 patterns documented)

## Overview

Semantic errors are violations of language meaning and type rules that pass syntax checks but fail during compilation or runtime. LLMs generate semantic errors due to:
- Lack of global symbol table tracking during generation
- No type inference or checking during token prediction
- Cross-language type system confusion
- Limited understanding of non-local dependencies

## Pattern Index

| ID | Name | Frequency | Severity | Languages |
|----|------|-----------|----------|-----------|
| EP-SEM-001 | Undefined Variable | 18% | HIGH | All |
| EP-SEM-002 | Type Mismatch | 16% | HIGH | Rust, TS, Go |
| EP-SEM-003 | Incorrect Function Args | 14% | HIGH | All |
| EP-SEM-004 | Missing Return | 12% | MEDIUM | All |

**Total Coverage**: 60% of semantic errors documented (4/~7 major patterns)

## Common Triggers

**High Risk**:
- Multi-line function bodies → EP-SEM-001
- Generic/template types → EP-SEM-002
- Functions with >3 parameters → EP-SEM-003
- Multiple return paths → EP-SEM-004

**Medium Risk**:
- Nested scopes (closures, lambdas) → EP-SEM-001
- Optional/nullable types → EP-SEM-002
- Optional/default parameters → EP-SEM-003
- Early return conditions → EP-SEM-004

## Cross-Language Analysis

### Python
- **Primary patterns**: EP-SEM-001 (scope), EP-SEM-004 (implicit None)
- **Frequency**: 20% semantic errors are undefined variables
- **Weakness**: Dynamic typing masks type mismatches until runtime

### Rust
- **Primary patterns**: EP-SEM-002 (strict types), EP-SEM-001 (ownership)
- **Frequency**: 25% semantic errors are type-related
- **Strength**: Compiler catches 90%+ semantic errors at compile time

### TypeScript
- **Primary patterns**: EP-SEM-002 (type safety), EP-SEM-003 (API misuse)
- **Frequency**: Similar to JavaScript but more caught statically
- **Weakness**: Type inference can hide errors

### Go
- **Primary patterns**: EP-SEM-002 (strict types), EP-SEM-004 (error handling)
- **Frequency**: Lower due to simpler type system
- **Strength**: Explicit error handling reduces missing returns

## Design Implications

**For D2 Design Guide**:
- **DR-SEM-001**: Explicit variable declarations (reduce EP-SEM-001)
- **DR-SEM-002**: Strong static typing (eliminate EP-SEM-002)
- **DR-SEM-003**: Limited function parameter count (prevent EP-SEM-003)
- **DR-SEM-004**: Mandatory return statements (enforce EP-SEM-004)

**For D3 Metrics**:
- **M-001 (Token Predictability)**: Correlates with EP-SEM-001 frequency
- **M-003 (AST Complexity)**: Impacts EP-SEM-004 occurrence

## Frequency Analysis

Based on literature review (P01, P02, P06, P07, P08):

```
Semantic Error Distribution:
EP-SEM-001 (Undefined):   18% ██████████████████
EP-SEM-002 (Type):        16% ████████████████
EP-SEM-003 (Args):        14% ██████████████
EP-SEM-004 (Return):      12% ████████████
Other patterns:           40% ████████████████████████
```

## Type System Impact

**Strict Type Systems** (Rust, Go):
- Catch 80-90% of semantic errors at compile time
- Type mismatches (EP-SEM-002) become syntax-like errors
- Lower runtime semantic error rate

**Gradual Typing** (TypeScript, Python with hints):
- Catch 40-60% of semantic errors statically
- Depends on type annotation coverage
- Escape hatches (`any`, type ignoring) reduce effectiveness

**Dynamic Typing** (Python, JavaScript):
- Most semantic errors become runtime errors
- Higher LLM error rate (fewer compile-time guards)
- Testing burden increases

## Research Backing

**Primary Sources**:
- P01 (ICSE 2025): Semantic characteristics framework
- P02 (Weakness Patterns): Cross-model validation
- P06 (Type Constraints): Type-constrained generation benefits
- P07 (Bug Taxonomy): Extended semantic categorization
- P08 (Mistakes): API misuse and semantic bugs

**Evidence Strength**: HIGH (converging findings across multiple studies)

## Next Steps

**Expansion** (for comprehensive taxonomy):
1. EP-SEM-005: Attribute/property access errors
2. EP-SEM-006: Incorrect exception handling
3. EP-SEM-007: Resource management errors (memory, files)

**Validation** (Phase 5):
- Cross-check with D4 benchmark suite results
- Correlate with D3 metrics (especially M-001, M-003)
- Verify frequency estimates with empirical LLM runs
