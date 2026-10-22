# Syntax Error Patterns

**Category**: SYNTAX
**Last Updated**: 2026-01-20
**Status**: Initial taxonomy (4 patterns documented)

## Overview

Syntax errors are compile-time violations of language grammar rules. LLMs generate syntax errors due to:
- Token sequence prediction failures (long-range dependencies)
- Cross-language contamination (operator/syntax confusion)
- Context window limitations (losing track of nesting)

## Pattern Index

| ID | Name | Frequency | Severity | Languages |
|----|------|-----------|----------|-----------|
| EP-SYN-001 | Missing Delimiter | 12% | HIGH | All |
| EP-SYN-002 | Incorrect Indentation | 8% | MEDIUM | Python |
| EP-SYN-003 | Off-by-One Index | 15% | HIGH | All |
| EP-SYN-004 | Incorrect Operator | 10% | HIGH | All |

**Total Coverage**: 45% of syntax errors documented (4/~10 major patterns)

## Common Triggers

**High Risk**:
- Nested structures (>3 levels) → EP-SYN-001
- Loop iteration with array access → EP-SYN-003
- Conditional expressions → EP-SYN-004

**Medium Risk**:
- Multi-line expressions → EP-SYN-001, EP-SYN-002
- Boolean logic combinations → EP-SYN-004
- Class method definitions → EP-SYN-002

## Cross-Language Analysis

### Python
- **Primary patterns**: EP-SYN-002 (indentation), EP-SYN-003 (indexing)
- **Frequency**: 20% syntax errors are indentation-related
- **Mitigation**: Explicit block delimiters, reduce indentation significance

### Rust
- **Primary patterns**: EP-SYN-001 (delimiters), EP-SYN-004 (operators)
- **Frequency**: Compiler catches 60%+ before runtime
- **Strength**: Strong type system prevents operator confusion

### TypeScript
- **Primary patterns**: EP-SYN-001 (delimiters), EP-SYN-004 (operators)
- **Frequency**: Similar to JavaScript patterns
- **Weakness**: Optional semicolons cause ambiguity

### Go
- **Primary patterns**: EP-SYN-003 (indexing), EP-SYN-004 (operators)
- **Frequency**: Lower overall due to simpler syntax
- **Strength**: Consistent operator usage

## Design Implications

**For D2 Design Guide**:
- **DR-SYN-001**: Explicit delimiter requirements (reduce EP-SYN-001)
- **DR-SYN-002**: Block delimiters over indentation (eliminate EP-SYN-002)
- **DR-SYN-003**: Zero-based indexing only (standardize EP-SYN-003)
- **DR-SYN-004**: Distinct operator syntax (prevent EP-SYN-004)

**For D3 Metrics**:
- **M-002 (Parse Ambiguity)**: Correlates with EP-SYN-001, EP-SYN-004
- **M-001 (Token Predictability)**: Impacts EP-SYN-003 frequency

## Frequency Analysis

Based on literature review (P01-P11) and cross-model patterns:

```
Syntax Error Distribution:
EP-SYN-003 (Off-by-one): 15% ████████████████
EP-SYN-001 (Delimiters):  12% █████████████
EP-SYN-004 (Operators):   10% ███████████
EP-SYN-002 (Indentation):  8% █████████
Other patterns:           55% ██████████████████████████████
```

## Research Backing

**Primary Sources**:
- P01 (ICSE 2025): Two-dimensional framework, HumanEval analysis
- P04 (SynCode): 96% syntax error reduction with grammar constraints
- P07 (Bug Taxonomy): Extended categorization
- P08 (Mistakes): Detailed error classification

**Evidence Strength**: HIGH (multiple independent studies, large sample sizes)

## Next Steps

**Expansion** (for comprehensive taxonomy):
1. EP-SYN-005: Malformed string literals (quotes, escapes)
2. EP-SYN-006: Incorrect import/module syntax
3. EP-SYN-007: Missing semicolons (language-specific)
4. EP-SYN-008: Incorrect type annotations
5. EP-SYN-009: Invalid identifier names
6. EP-SYN-010: Mismatched function/method signatures

**Validation** (Phase 5):
- Run LLM evaluations (T025-T027) to verify frequency estimates
- Cross-check against D4 benchmark results
- Update frequencies with empirical data
