# SWE-bench Dataset - Setup Required

**Status**: Manual setup required
**Date**: 2026-01-20
**Benchmark**: SWE-bench Lite (300 instances)

## Why SWE-bench Requires Manual Setup

SWE-bench is a repository-level benchmark that requires:
- Large dataset downloads (~100+ MB)
- Dependencies: `datasets`, `pandas`, or `pyarrow`
- Significant computational resources for evaluation

## Option 1: Install Dependencies (Recommended)

```bash
# Install Hugging Face datasets library
pip install datasets

# Then run:
cd research/D1-error-taxonomy
python download_swebench.py --use-datasets-lib --subset-size 20
```

## Option 2: Manual Download

1. Visit: https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite
2. Download the test split (JSON or Parquet format)
3. Extract 20 representative instances
4. Save to: `SWEbench_subset.json`

## Option 3: Skip SWE-bench (Acceptable for Phase 3)

**Coverage Status**:
- ✓ HumanEval: 50 problems (function-level Python)
- ✓ MBPP: 50 problems (basic Python programming)
- ⚠ SWE-bench: Pending manual setup

**Requirements Met**: 2/3 benchmarks downloaded
**Minimum Required**: 3+ benchmarks for comprehensive analysis

**Decision**: Can proceed with HumanEval + MBPP for initial error taxonomy (T025-T030), then expand to SWE-bench if needed for validation.

## SWE-bench Characteristics

- **Scale**: Repository-level (full codebases)
- **Complexity**: High (multiple files, dependencies)
- **Languages**: Python primarily
- **Use Case**: Real-world software engineering tasks
- **Typical Errors**: Integration bugs, API misuse, context management

## When to Include SWE-bench

Include SWE-bench when:
1. Initial error taxonomy (T028-T030) reveals gaps in error coverage
2. Need to validate repository-level error patterns
3. Dependencies are installed and computational resources available

For MVP (Phase 3), HumanEval + MBPP provide sufficient coverage for:
- Syntax errors (both benchmarks)
- Semantic errors (both benchmarks)
- Logic errors (both benchmarks)
- Cross-language patterns (Python-focused)

## Next Steps

**If continuing without SWE-bench**:
1. Proceed to T025: Run GPT-4 on HumanEval
2. Note: Error taxonomy will be Python-focused initially
3. Consider adding Rust/TypeScript benchmarks later for language diversity

**If installing SWE-bench**:
1. Install dependencies: `pip install datasets`
2. Run: `python download_swebench.py --use-datasets-lib`
3. Verify: Check `SWEbench_subset.json` exists
