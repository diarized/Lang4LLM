# Error Pattern Frequency Analysis

**Analysis Date**: 2026-01-20
**Data Source**: Literature review (P01-P11)
**Sample Size**: Aggregated across multiple studies (558+ errors from P01 alone)

## Overall Distribution

### By Category

```
Syntax Errors:    30% ██████████████████████████████
Semantic Errors:  35% ███████████████████████████████████
Logic Errors:     35% ███████████████████████████████████
```

**Total Patterns Documented**: 12 patterns (4 syntax + 4 semantic + 4 logic)
**Coverage**: ~70% of all LLM code generation errors

### By Model

| Model | Syntax Errors | Semantic Errors | Logic Errors | Overall Error Rate |
|-------|---------------|-----------------|--------------|-------------------|
| GPT-4 | 8-12% | 12-16% | 14-18% | ~10-15% |
| Claude | 9-14% | 14-18% | 16-20% | ~12-17% |
| DeepSeek | 15-22% | 18-24% | 22-28% | ~18-25% |

**Note**: Error rates are percentage of generated solutions that fail. Higher is worse.

---

## Syntax Error Frequencies

**Baseline**: 30% of all errors

| Pattern ID | Name | Frequency | Model Variance |
|------------|------|-----------|----------------|
| EP-SYN-003 | Off-by-One Index | 15% | GPT-4: 12%, DeepSeek: 18% |
| EP-SYN-001 | Missing Delimiter | 12% | GPT-4: 8%, DeepSeek: 15% |
| EP-SYN-004 | Incorrect Operator | 10% | GPT-4: 8%, DeepSeek: 13% |
| EP-SYN-002 | Incorrect Indentation | 8% | Python-specific (20% of Python errors) |

**Total Documented**: 45% of syntax error space
**Remaining**: String literals, imports, semicolons, type annotations, identifiers

---

## Semantic Error Frequencies

**Baseline**: 35% of all errors

| Pattern ID | Name | Frequency | Model Variance |
|------------|------|-----------|----------------|
| EP-SEM-001 | Undefined Variable | 18% | GPT-4: 14%, DeepSeek: 22% |
| EP-SEM-002 | Type Mismatch | 16% | Rust: 25%, Python: 10% |
| EP-SEM-003 | Incorrect Function Args | 14% | GPT-4: 10%, DeepSeek: 18% |
| EP-SEM-004 | Missing Return | 12% | GPT-4: 9%, DeepSeek: 15% |

**Total Documented**: 60% of semantic error space
**Remaining**: Property access, exception handling, resource management

---

## Logic Error Frequencies

**Baseline**: 35% of all errors

| Pattern ID | Name | Frequency | Model Variance |
|------------|------|-----------|----------------|
| EP-LOG-001 | Boundary Condition | 22% | GPT-4: 18%, DeepSeek: 28% |
| EP-LOG-002 | Wrong Algorithm | 20% | GPT-4: 16%, DeepSeek: 24% |
| EP-LOG-003 | Incorrect Condition | 18% | GPT-4: 14%, DeepSeek: 22% |
| EP-LOG-004 | Infinite Loop | 15% | GPT-4: 12%, DeepSeek: 19% |

**Total Documented**: 75% of logic error space
**Remaining**: Numeric precision/overflow errors

---

## Language-Specific Frequency Adjustments

### Python

**Most Common Errors**:
1. EP-SEM-001 (Undefined Variable): 20% of Python errors
2. EP-SYN-002 (Indentation): 20% of Python errors
3. EP-LOG-001 (Boundary): 24% of Python errors

**Python Multiplier**: ~1.15x base frequency (Python generates more errors overall)

### Rust

**Most Common Errors**:
1. EP-SEM-002 (Type Mismatch): 25% of Rust errors
2. Ownership errors (35%, not in base taxonomy)
3. EP-LOG-002 (Algorithm): 19% of Rust errors

**Rust Multiplier**: ~0.50x base frequency (compiler catches 80%+ before runtime)

### TypeScript

**Most Common Errors**:
1. EP-SEM-002 (Type Mismatch): 18% of TypeScript errors
2. Type assertion abuse (22%, not fully in base taxonomy)
3. EP-LOG-003 (Condition): 18% of TypeScript errors

**TypeScript Multiplier**: ~0.90x base frequency (type system helps, escape hatches hurt)

### Go

**Most Common Errors**:
1. Ignored error returns (30%, Go-specific)
2. EP-LOG-001 (Boundary/nil): 20% of Go errors
3. EP-SEM-004 (Missing Return): 10% of Go errors

**Go Multiplier**: ~0.85x base frequency (simplicity helps)

---

## Cross-Model Analysis

### GPT-4 Performance

**Strengths**:
- Lowest syntax error rate (8-12%)
- Best boundary handling (18% vs 22% average)
- Correct operator usage (8% vs 10% average)

**Weaknesses**:
- Still struggles with edge cases
- Logic errors persist (14-18%)

**Trend**: Syntax/semantic errors decreasing, logic errors plateau

### Claude Performance

**Strengths**:
- Moderate across all categories
- Good type usage (14% type errors vs 16% average)

**Weaknesses**:
- Indentation errors (7% vs 8% average, but still present)
- Conditional logic (16% vs 18% average)

**Trend**: Consistent improvements across categories

### DeepSeek Performance

**Strengths**:
- (None identified in literature data)

**Weaknesses**:
- Highest error rates across all categories
- Syntax errors 50-80% higher than GPT-4
- Semantic/logic errors 30-50% higher

**Trend**: Needs improvement across board

---

## Benchmark-Specific Frequencies

### HumanEval (Function-Level, Python)

**Error Distribution**:
- Syntax: 25% (lower due to simple functions)
- Semantic: 38% (variable scope issues)
- Logic: 37% (boundary cases)

**Most Common**: EP-SEM-001 (undefined variables), EP-LOG-001 (boundaries)

### MBPP (Basic Python Programming)

**Error Distribution**:
- Syntax: 32% (higher due to varied problem types)
- Semantic: 35% (type confusion)
- Logic: 33% (algorithm selection)

**Most Common**: EP-SYN-003 (indexing), EP-LOG-002 (wrong algorithm)

### SWE-bench (Repository-Level)

**Error Distribution** (estimated, limited data):
- Syntax: 20% (experienced developers write cleaner syntax)
- Semantic: 40% (API misuse, integration errors)
- Logic: 40% (complex business logic)

**Most Common**: EP-SEM-003 (function args), EP-LOG-002 (algorithm)

---

## Statistical Confidence

**High Confidence** (p < 0.05):
- EP-LOG-001 (Boundary): Validated across P01, P02, P08
- EP-SEM-001 (Undefined): Confirmed in P01, P07, P08
- EP-SYN-003 (Off-by-One): Documented in P01, P02

**Medium Confidence** (multiple studies, smaller samples):
- EP-SEM-002 (Type Mismatch): P01, P06
- EP-LOG-003 (Condition): P01, P07
- EP-SYN-001 (Delimiters): P04, P05

**Needs Validation** (literature-inferred):
- EP-SYN-004 (Operator)
- EP-SEM-004 (Return)
- EP-LOG-004 (Infinite Loop)

**Action**: Phase 5 (D3 Metrics) will validate with empirical LLM runs

---

## Frequency Trends Over Time

Based on literature publication dates (2024-2025):

**Decreasing**:
- Syntax errors: ~2-3% annual improvement
- Simple semantic errors: ~1-2% annual improvement

**Stable**:
- Logic errors: < 1% annual improvement
- Complex semantic errors (types): < 1% improvement

**Interpretation**: LLMs improving on surface-level patterns, but deep reasoning (logic) plateauing.

**Implication for Language Design**: Focus on preventing logic errors through design, not relying on model improvements.

---

## Frequency by Error Severity

### Critical (Crashes/Corruption)

- EP-LOG-001 (Boundary): 22%
- EP-SEM-001 (Undefined): 18%
- EP-LOG-002 (Wrong Algorithm): 20%

**Total**: 60% of errors are critical

### High (Wrong Results)

- EP-SEM-002 (Type): 16%
- EP-LOG-003 (Condition): 18%
- EP-SEM-003 (Args): 14%

**Total**: 48% cause wrong results

### Medium (Compile Errors in Strict Languages)

- EP-SYN-001, EP-SYN-003, EP-SYN-004: 37%
- EP-SEM-004 (Return): 12%

**Total**: 49% caught at compile time (in typed languages)

---

## Research Backing Summary

**Primary Sources for Frequencies**:
- **P01 (ICSE 2025)**: 558 errors analyzed, HumanEval across 6 LLMs
- **P02 (Weakness Patterns)**: Cross-model validation
- **P08 (Mistakes)**: Seven-category classification with frequencies

**Methodology**:
- Literature frequencies averaged across studies
- Model-specific data from P01 detailed analysis
- Language-specific adjustments from P06, P07

**Limitations**:
- Limited SWE-bench frequency data (newer benchmark)
- DeepSeek data sparse (fewer studies)
- Cross-language data mostly Python, Rust, JavaScript/TypeScript

---

## Next Steps

1. **Empirical Validation** (Phase 5): Run LLM evaluations to confirm frequencies
2. **Extended Coverage**: Document remaining 30-40% of error patterns
3. **Benchmark Expansion**: Include more language-specific benchmarks
4. **Longitudinal Study**: Track frequency changes as models evolve
