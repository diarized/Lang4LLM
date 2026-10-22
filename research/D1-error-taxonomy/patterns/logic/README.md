# Logic Error Patterns

**Category**: LOGIC
**Last Updated**: 2026-01-20
**Status**: Initial taxonomy (4 patterns documented)

## Overview

Logic errors are the most subtle class: code is syntactically and semantically correct but produces wrong results or behavior. LLMs generate logic errors due to:
- Pattern matching without deep problem understanding
- Training data bias toward "happy path" examples
- Lack of systematic edge case enumeration
- Incomplete reasoning about algorithmic correctness

**Detection Challenge**: Logic errors pass compilation and type checking, requiring test execution or formal verification to detect.

## Pattern Index

| ID | Name | Frequency | Severity | Languages |
|----|------|-----------|----------|-----------|
| EP-LOG-001 | Boundary Condition | 22% | HIGH | All |
| EP-LOG-002 | Wrong Algorithm | 20% | HIGH | All |
| EP-LOG-003 | Incorrect Condition | 18% | HIGH | All |
| EP-LOG-004 | Infinite Loop | 15% | MEDIUM | All |

**Total Coverage**: 75% of logic errors documented (4/~5 major patterns)

## Common Triggers

**High Risk**:
- Collection processing (lists, arrays) → EP-LOG-001
- Complex problem requiring specific algorithm → EP-LOG-002
- Complex boolean expressions (>2 conditions) → EP-LOG-003
- While loops with complex conditions → EP-LOG-004

**Medium Risk**:
- Numeric computations (division, modulo) → EP-LOG-001
- Multiple valid but efficiency-varying approaches → EP-LOG-002
- Negated conditions → EP-LOG-003
- Recursive functions → EP-LOG-004

## Cross-Language Analysis

### Python
- **Primary patterns**: EP-LOG-001 (boundaries), EP-LOG-004 (loops)
- **Frequency**: 24% logic errors are boundary-related
- **Weakness**: Dynamic typing hides logic errors until runtime

### Rust
- **Primary patterns**: EP-LOG-002 (algorithm), EP-LOG-004 (recursion)
- **Frequency**: Lower overall due to explicit error handling
- **Strength**: Panic on bounds violations catches some EP-LOG-001

### TypeScript
- **Primary patterns**: EP-LOG-001 (boundaries), EP-LOG-003 (conditions)
- **Frequency**: Similar to JavaScript patterns
- **Weakness**: Type system doesn't prevent logic errors

### Go
- **Primary patterns**: EP-LOG-001 (boundaries), EP-LOG-003 (conditions)
- **Frequency**: Moderate, explicit error handling helps
- **Strength**: Range-based loops reduce EP-LOG-004

## Design Implications

**For D2 Design Guide**:
- **DR-ERR-001**: Explicit boundary checks (prevent EP-LOG-001)
- **DR-SEM-003**: Algorithm hints in type system (guide EP-LOG-002)
- **DR-SYN-005**: Simplified boolean operators (reduce EP-LOG-003)
- **DR-ERR-002**: Loop invariant annotations (detect EP-LOG-004)

**For D3 Metrics**:
- **M-003 (AST Complexity)**: Correlates with all logic error patterns
- **M-004 (Error Recovery)**: Impacts ability to detect logic errors

**For D4 Benchmark**:
- Must include boundary cases explicitly (test EP-LOG-001)
- Multiple algorithmic approaches per problem (test EP-LOG-002)
- Complex boolean conditions (test EP-LOG-003)
- Recursion and iteration variants (test EP-LOG-004)

## Frequency Analysis

Based on literature review (P01, P02, P07, P08):

```
Logic Error Distribution:
EP-LOG-001 (Boundary):   22% ██████████████████████
EP-LOG-002 (Algorithm):  20% ████████████████████
EP-LOG-003 (Condition):  18% ██████████████████
EP-LOG-004 (Loop):       15% ███████████████
Other patterns:          25% █████████████████████████
```

## Severity Analysis

**Critical** (Causes incorrect results):
- EP-LOG-001: Data corruption, crashes
- EP-LOG-002: Fundamentally wrong answers
- EP-LOG-003: Security vulnerabilities (auth bypass)

**High** (Causes hangs/crashes):
- EP-LOG-004: Resource exhaustion, stack overflow

**Mitigation Difficulty**:
- **Syntax errors**: Easy (compiler catches)
- **Semantic errors**: Medium (type checker catches)
- **Logic errors**: Hard (requires testing, proofs, or human review)

## Testing Requirements

Logic errors require:
1. **Comprehensive test suites**: Cover boundary cases, edge inputs
2. **Property-based testing**: Systematic input space exploration
3. **Formal verification**: Mathematical proofs for critical code
4. **Code review**: Human inspection for algorithmic correctness

**Language Design Impact**:
- Make correctness properties expressible in type system
- Provide built-in boundary checking
- Simplify boolean logic syntax
- Enforce loop termination proofs

## Research Backing

**Primary Sources**:
- P01 (ICSE 2025): Wrong direction, incorrect condition, missing condition
- P02 (Weakness Patterns): Systematic edge case failures
- P07 (Bug Taxonomy): Logical bug category
- P08 (Mistakes): Algorithmic mistake classification

**Evidence Strength**: HIGH (consistent across multiple independent studies)

**Key Finding**: Logic errors are the most persistent across LLM improvements. While syntax/semantic error rates decrease with model upgrades, logic errors remain prevalent.

## Next Steps

**Expansion** (for comprehensive taxonomy):
1. EP-LOG-005: Numeric precision/overflow errors

**Validation** (Phase 5):
- D4 benchmark must explicitly test all logic patterns
- Frequency validation requires extensive test execution
- Cross-model comparison to identify model-specific logic biases
