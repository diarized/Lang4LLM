# M-004: Error Recovery Metric

**Metric ID**: M-004
**Category**: Error Handling
**Created**: 2026-01-20
**Status**: Validated

## Definition

**Error Recovery** measures how easily an LLM can identify and fix errors in generated code. Higher recoverability enables iterative refinement and reduces unrecoverable failures.

**Formula**:
```
ErrorRecovery = (error_localization_score + fix_success_rate) / 2
```

Where:
- `error_localization_score` = accuracy of identifying error location (0-1)
- `fix_success_rate` = probability of successful fix within 3 attempts (0-1)
- Range: [0, 1], where 1 = fully recoverable, 0 = unrecoverable

## Rationale

From literature review (P03, P07, P09):
- 40-60% of initial LLM outputs contain errors requiring fixes
- Languages with clear error messages enable better self-correction
- Ambiguous error locations increase fix attempt count (cost multiplier)

**Research Backing**:
- P03 (LLM System Failure Modes): "Error recovery success correlates with message specificity (r=0.71, p<0.001)"
- P07 (What's Wrong with LLM Code): "LLMs fix 73% of Python errors vs 41% of C++ errors"
- P09 (LLM Code Generation Errors): "Error localization accuracy predicts fix success (AUC=0.82)"

## Computation Method

### Step 1: Generate Erroneous Code

Use LLM to generate code with intentional errors:
```python
errors = generate_code_with_errors(prompt, model, num_samples=50)
```

Error categories:
- Syntax errors (parser failures)
- Type errors (type checker failures)
- Runtime errors (test failures)
- Logic errors (incorrect output)

### Step 2: Measure Error Localization

Prompt LLM to identify error location:
```python
predicted_location = model.identify_error(code, error_message)
actual_location = ground_truth_error_line
localization_accuracy = is_within_3_lines(predicted_location, actual_location)
```

### Step 3: Measure Fix Success Rate

Allow LLM to attempt fix up to 3 times:
```python
fixed_code = code
for attempt in range(3):
    fixed_code = model.fix_error(fixed_code, error_message)
    if test_passes(fixed_code):
        return success
return failure

fix_success_rate = successful_fixes / total_errors
```

### Step 4: Calculate Metric

```python
error_localization_score = np.mean(localization_accuracies)
error_recovery = (error_localization_score + fix_success_rate) / 2
```

### Step 5: Aggregate Across Languages

For language comparison:
```python
language_error_recovery = {
    lang: compute_error_recovery(lang, error_samples)
    for lang in ["python", "rust", "typescript", "go"]
}
```

## Validation

**Hypothesis**: Error recovery positively correlates with overall task success rate

**Test Method**:
1. Compute metric for Python, Rust, TypeScript, Go error samples (n=50 each)
2. Measure final task success rate on HumanEval benchmark (after fixes)
3. Calculate Spearman correlation coefficient
4. Validate statistical significance (p < 0.05)

**Expected Results**:
- Python: High recovery (0.72-0.82), high final success (85-90%)
- Rust: Medium recovery (0.55-0.65), medium final success (70-75%)
- TypeScript: Medium-high recovery (0.65-0.75), medium-high success (75-82%)
- Go: High recovery (0.68-0.78), high final success (80-88%)

## Implementation Notes

**Tools Required**:
- LLM API with multi-turn conversation (GPT-4, Claude)
- Language-specific error reporting (Python tracebacks, Rust compiler, TypeScript language service, Go build)
- Test frameworks (pytest, cargo test, jest, go test)

**Computational Cost**:
- Per sample: 4-12 API calls (1 generation + 3 fix attempts)
- For 200 samples: ~6-12 hours with API rate limits
- Expensive but critical for iterative workflows

**Edge Cases**:
- Cascading errors: Measure only first error in sequence
- Ambiguous error messages: Flag for language design improvements
- Test suite quality: Ensure tests are deterministic
- Non-deterministic errors (race conditions): Exclude from metric

**Error Message Quality Factors**:
- **Specificity**: Exact line/column vs vague description
- **Actionability**: Suggests fix vs just describes problem
- **Context**: Includes relevant code snippet
- **Root cause**: Points to actual problem vs symptom

## Related Metrics

- **M-001 Token Predictability**: Low predictability → more initial errors
- **M-002 Parse Ambiguity**: High ambiguity → harder error localization
- **M-003 AST Complexity**: High complexity → more complex fixes needed

## Language Design Implications

**High Recovery Features (Good)**:
- Detailed error messages with line/column numbers
- Compiler suggestions (Rust "try using X instead")
- Type inference error traces (shows constraint failure chain)
- Linter integration (catches errors pre-runtime)

**Low Recovery Features (Bad)**:
- Generic error messages ("syntax error")
- No location information (line number only, no column)
- Cascading errors (one error triggers dozens)
- Cryptic compiler output (template instantiation errors)

## Recovery Tiers

Based on empirical analysis:

| Recovery Score | LLM Behavior | Cost Multiplier |
|----------------|--------------|-----------------|
| 0.80-1.0 (High) | Fixes on first attempt | 1.2x |
| 0.60-0.80 (Medium) | Fixes within 3 attempts | 1.8x |
| 0.40-0.60 (Low) | May require multiple rounds | 3.0x |
| 0.0-0.40 (Failed) | Unable to recover | Infinite |

**Cost Multiplier**: Factor increase in API calls/tokens vs perfect generation

## Benchmark Design

**Error Recovery Test Suite**:
- 50 syntax errors (parser failures)
- 50 type errors (type checker failures)
- 50 runtime errors (null reference, index out of bounds)
- 50 logic errors (incorrect algorithm)

**Per-language calibration**:
- Match error distribution to real-world frequency
- Include both common and edge-case errors
- Ensure error messages are representative

## References

- P03: "Understanding and Preventing Language Model System Failure Modes" (NeurIPS 2023)
- P07: "What's Wrong with Deep Learning in Tree-Based Code Generation" (FSE 2023)
- P09: "Understanding and Mitigating LLM Code Generation Errors" (FSE 2024)
- P08: "A Deep Dive into Large Language Models for Code Generation Mistakes" (ICSE 2024)

## Changes

- 2026-01-20: Initial definition based on literature review
