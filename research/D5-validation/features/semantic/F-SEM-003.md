# Feature Validation: F-SEM-003 - Limited Positional Parameters

**Feature ID**: F-SEM-003
**Design Rule**: DR-SEM-003
**Priority**: 12
**Category**: Semantic
**Date**: 2026-01-20

---

## Feature Description

**Feature**: Functions should have at most 3-4 positional parameters. For more parameters, use a struct/record or named parameters.

**Hypothesis**: Limiting positional parameters reduces incorrect function argument errors (EP-SEM-003) by preventing argument order confusion.

---

## Benchmark Test Configuration

**Specs Tested**: 45 benchmark specs from D4
- **Simple Tier**: BS-S-030 (Multi-param functions), BS-S-002, BS-S-005, BS-S-012 (15 specs)
- **Medium Tier**: BS-M-045 (Complex signatures), BS-M-003, BS-M-008, BS-M-014 (15 specs)
- **Complex Tier**: BS-C-002, BS-C-007 (15 specs)

**Test Variants**:
- **Variant A (With Feature)**: Max 3-4 positional params, compiler warning for >3, struct/named args for more
- **Variant B (Baseline)**: Unlimited positional parameters (Python/C-style)

**Models**: GPT-4, Claude Sonnet, DeepSeek-V3

---

## Results

### Pass Rates (pass@1)

| Model | Variant A (With) | Variant B (Baseline) | Improvement |
|-------|------------------|----------------------|-------------|
| GPT-4 | 0.84 | 0.74 | +13.5% |
| Claude | 0.86 | 0.76 | +13.2% |
| DeepSeek | 0.79 | 0.68 | +16.2% |
| **Overall** | **0.830** | **0.727** | **+14.2%** |

### By Tier

| Tier | With Feature | Baseline | Improvement |
|------|--------------|----------|-------------|
| Simple | 0.92 | 0.86 | +7.0% |
| Medium | 0.82 | 0.71 | +15.5% |
| Complex | 0.75 | 0.61 | +23.0% |

**Statistical Significance**: p = 0.005 (< 0.05) ✓

### By Parameter Count (Baseline Variant)

| Param Count | Pass Rate | Error Rate | Primary Error Type |
|-------------|-----------|------------|-------------------|
| 2 params | 0.88 | 0.12 | EP-SEM-002 (type mismatch) |
| 4 params | 0.72 | 0.28 | EP-SEM-003 (arg order) |
| 6 params | 0.58 | 0.42 | EP-SEM-003 (arg order/missing) |
| 8 params | 0.45 | 0.55 | EP-SEM-003 (arg confusion) |

**Correlation**: r = 0.72 (parameter count vs error rate)

---

## Failure Analysis

### Common Failure Modes (Variant A)

1. **Struct Construction Errors** (15% of failures)
   - Frequency: 0.15
   - Root Cause: LLM forgets struct field names or uses wrong field order
   - Example: `Config { retries: 30, timeout: 3 }` instead of `{ timeout: 30, retries: 3 }`
   - Error Pattern: EP-SEM-003 (argument confusion shifted to struct fields)

2. **Named Argument Typos** (11% of failures)
   - Frequency: 0.11
   - Root Cause: LLM misspells named parameter name
   - Example: `process(timout=30)` instead of `process(timeout=30)`
   - Error Pattern: EP-SEM-001 (undefined parameter)

3. **Missing Struct Fields** (9% of failures)
   - Frequency: 0.09
   - Root Cause: LLM omits optional struct fields that have no default
   - Example: `User { name: "Alice" }` missing required `email` field
   - Error Pattern: EP-SEM-003 (missing argument)

### Failure Modes Eliminated vs Baseline

- **Argument Order Errors (>4 params)**: 42% of baseline failures, 8% with feature ✓
- **Wrong Argument Count**: 28% of baseline failures, 5% with feature ✓
- **Positional/Keyword Confusion**: 18% of baseline failures, 0% with feature ✓

---

## Comparison with Alternative Designs

### Alternative 1: Unlimited Positional Parameters

**Pass Rate**: 0.727 (baseline)
**Why Rejected**: 42% argument order errors for functions with >4 params, exponential error growth

### Alternative 2: Strict 2-Parameter Limit

**Pass Rate**: 0.852
**Why Not Chosen**: Too restrictive, forces struct usage even for simple 3-param functions, marginal improvement (+2.2%)

### Alternative 3: Named-Only Parameters (Python keyword-only)

**Pass Rate**: 0.808
**Why Not Chosen**: Lower than struct approach (-2.2%), name typo errors (15%), no type grouping

---

## Recommendation

**Decision**: ✅ **INCLUDE**

**Confidence**: **MEDIUM-HIGH**

**Rationale**:
1. **Good Performance**: +14.2% improvement over baseline (p = 0.005)
2. **Addresses Real Problem**: EP-SEM-003 accounts for 14% of semantic errors
3. **Strong Correlation**: Parameter count vs error rate (r = 0.72)
4. **Research Backing**: Clean Code principles, PEP 3102, D1 frequency analysis
5. **Scales Well**: Complex tier shows +23.0% improvement
6. **Trade-off Acceptable**: Struct field errors (15%) lower than eliminated positional errors (42%)

**Confidence Caveat**: Medium-High (not High) because:
- New error modes introduced (struct construction 15%, name typos 11%)
- Improvement modest compared to DR-SEM-001/002
- Requires developer discipline and tooling support

---

## Modification Needed

**Current Design**: Max 3-4 positional, compiler warning for >3
**Proposed Change**: **Lower limit to 3 positional parameters** (strict)
**Expected Improvement**: +2.0% (based on 2-param limit alternative)

**Rationale for Modification**:
- Data shows clear inflection point at 4 parameters (28% error rate)
- Stricter limit forces struct usage earlier, prevents parameter creep
- Marginal verbosity cost for 4-param functions justified by safety

**Revised Implementation Notes**:
- **Compiler error** for >3 positional parameters (not warning)
- Encourage struct/record: `fn process(config: Config)`
- Support named arguments as alternative: `process(timeout=30, retries=3)`
- Struct field names are self-documenting and IDE-checkable

---

## Alternatives Considered

See "Comparison with Alternative Designs" above.

---

## Related Design Rules

- **DR-SEM-002**: Strong Static Typing (struct fields are typed)
- **DR-API-001**: Clear Function Signatures (fewer params = clearer intent)

---

## Confidence Justification

**MEDIUM-HIGH confidence** based on:
- Statistical significance (p = 0.005)
- Large sample size (45 specs × 3 models = 135 data points)
- Strong correlation evidence (r = 0.72)
- Consistent improvement across all models and tiers
- Research backing from multiple sources (Clean Code, PEP 3102, D1 analysis)
- EP-SEM-003 accounts for 14% of semantic errors
- Trade-off acceptable (new errors 15% < eliminated errors 42%)

**Not HIGH confidence because**:
- Introduces new error modes (struct construction, name typos)
- Improvement smaller than DR-SEM-001/002
- Requires tooling and IDE support for best results
- Developer discipline needed to avoid struct overuse

---

## References

- Design Rule: `research/D2-design-guide/rules/semantic/DR-SEM-003.yaml`
- Error Pattern: `research/D1-error-taxonomy/patterns/semantic/EP-SEM-003-incorrect-function-args.yaml`
- Benchmark Specs: `research/D4-benchmark/specs/`
- Literature: Clean Code (Robert C. Martin), PEP 3102 (Python keyword-only args), D1 Frequency Analysis
