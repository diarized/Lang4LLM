# LLM Evaluation Infrastructure

**Status**: Infrastructure ready, actual runs require API keys
**Tasks**: T025 (GPT-4), T026 (Claude), T027 (DeepSeek)
**Date**: 2026-01-20

## What's Ready

✓ **Evaluation script created**: `run_evaluation.py`
✓ **Benchmarks downloaded**: HumanEval (50), MBPP (50)
✓ **Output directories created**: `data/{model}-{benchmark}/`
✓ **API configuration**: `api-config.template.json`

## Running Evaluations

### Prerequisites

1. **API Keys**: Configure in `.env` file (see SETUP.md)
2. **Verify access**: Run `python verify_api_access.py`
3. **Install dependencies**:
   ```bash
   pip install openai anthropic requests python-dotenv
   ```

### Execute Evaluation

```bash
# Single model, single benchmark
python run_evaluation.py --model gpt4 --benchmark humaneval

# Multiple models
python run_evaluation.py --model gpt4 --model claude --benchmark humaneval

# All models (requires all API keys)
python run_evaluation.py --all

# Limit problems (for testing)
python run_evaluation.py --model gpt4 --benchmark humaneval --limit 10
```

### Expected Outputs

Each evaluation creates:

```
data/{model}-{benchmark}/
├── results.jsonl          # All responses (pass + fail)
├── failures.jsonl         # Failed attempts only (for taxonomy)
├── summary.json           # Statistics (pass rate, error types)
```

### Cost Estimates

| Model | Problems | Est. Tokens | Est. Cost |
|-------|----------|-------------|-----------|
| GPT-4 | 50 (HumanEval) | ~100k | $1-2 USD |
| Claude | 50 (HumanEval) | ~100k | $0.50-1 USD |
| DeepSeek | 50 (HumanEval) | ~100k | $0.10-0.20 USD |

**Total for full evaluation**: ~$2-5 USD

## Alternative: Use Example Data

For taxonomy development without running evaluations, we've extracted error patterns from literature (Phase 2).

**Literature-based error patterns** (from P01-P11 analysis):
- Syntax errors: Off-by-one, missing delimiters, incorrect operators
- Semantic errors: Type mismatches, undefined variables, scope issues
- Logic errors: Boundary conditions, edge cases, algorithmic mistakes

**Proceed to T028-T030** using literature-documented patterns instead of actual LLM outputs.

## Evaluation Results (When Available)

After running evaluations, results will be in:
- `data/gpt4-humaneval/failures.jsonl`
- `data/claude-humaneval/failures.jsonl`
- `data/deepseek-humaneval/failures.jsonl`

These failure files feed into:
- **T028**: Categorize syntax errors
- **T029**: Categorize semantic errors
- **T030**: Categorize logic errors

## Status Tracking

| Task | Model | Benchmark | Status | Notes |
|------|-------|-----------|--------|-------|
| T025 | GPT-4 | HumanEval | Infrastructure ready | Requires OPENAI_API_KEY |
| T026 | Claude | HumanEval | Infrastructure ready | Requires ANTHROPIC_API_KEY |
| T027 | DeepSeek | HumanEval | Infrastructure ready | Requires DEEPSEEK_API_KEY |

**Decision**: Proceeding with literature-based taxonomy (sufficient for MVP)
**Rationale**:
- Phase 2 literature review already cataloged 50+ error patterns from research
- Actual LLM runs are validation, not discovery
- Can run evaluations later for empirical validation (Phase 4/5)

## Next Steps

**Option A: Run evaluations now** (if API keys available):
1. Configure API keys in `.env`
2. Run: `python run_evaluation.py --all`
3. Proceed to T028-T030 with real failure data

**Option B: Use literature patterns** (recommended for MVP):
1. Proceed directly to T028-T030
2. Use error patterns from Phase 2 literature review
3. Run evaluations later for empirical validation (Phase 5: D3 Metrics)

**Recommendation**: Option B - Literature patterns are sufficient for D1 Error Taxonomy MVP
