# Error Taxonomy Evaluation Environment Setup

**Purpose**: Configure LLM API access for error pattern analysis

**Date**: 2026-01-20
**Phase**: 3 (User Story 1 - Error Taxonomy)

## Prerequisites

### Required API Keys

Obtain API keys from:

1. **OpenAI** (GPT-4): https://platform.openai.com/api-keys
2. **Anthropic** (Claude): https://console.anthropic.com/
3. **DeepSeek** (DeepSeek Coder): https://platform.deepseek.com/

### Required Python Packages

```bash
pip install openai>=1.0.0
pip install anthropic>=0.18.0
pip install requests>=2.31.0
pip install python-dotenv>=1.0.0
```

Or use requirements file (if created):
```bash
pip install -r requirements.txt
```

## Configuration

### 1. Environment Variables

Create a `.env` file in the **repository root**:

```bash
# .env (repository root)
OPENAI_API_KEY=sk-proj-...
ANTHROPIC_API_KEY=sk-ant-...
DEEPSEEK_API_KEY=...
```

**⚠️ CRITICAL**: The `.env` file is already in `.gitignore`. Never commit API keys.

### 2. API Configuration

The `api-config.template.json` file contains model settings:

```json
{
  "models": {
    "gpt4": {
      "provider": "openai",
      "model_name": "gpt-4-turbo-preview",
      "temperature": 0.0,
      ...
    },
    ...
  }
}
```

**Customization**:
- `temperature`: 0.0 for deterministic outputs (reproducibility)
- `max_tokens`: Adjust based on expected output length
- `timeout`: Increase for slow networks

### 3. Verify Setup

Run the verification script:

```bash
cd research/D1-error-taxonomy/
python verify_api_access.py
```

**Expected Output**:
```
============================================================
LLM API Access Verification
============================================================

GPT4:
----------------------------------------
  ✓ OPENAI_API_KEY found (64 chars)
  ✓ OpenAI API accessible (50 models available)

CLAUDE:
----------------------------------------
  ✓ ANTHROPIC_API_KEY found (108 chars)
  ✓ Anthropic API accessible (response: 2 chars)

DEEPSEEK:
----------------------------------------
  ✓ DEEPSEEK_API_KEY found (...)
  ⚠ DeepSeek API test not implemented

============================================================
SUMMARY
============================================================
✓ GPT4: READY
✓ CLAUDE: READY
✗ DEEPSEEK: NOT READY

✗ Some APIs are not ready
```

## Directory Structure

After setup, the evaluation environment includes:

```
research/D1-error-taxonomy/
├── SETUP.md                      # This file
├── api-config.template.json      # API configuration
├── verify_api_access.py          # Setup verification script
├── data/                         # Benchmark data and outputs
│   ├── gpt4-humaneval/          # GPT-4 outputs (T025)
│   ├── claude-humaneval/        # Claude outputs (T026)
│   ├── deepseek-humaneval/      # DeepSeek outputs (T027)
│   └── evaluation.log           # Execution logs
└── patterns/                     # Error categorization (T028-T030)
    ├── syntax/
    ├── semantic/
    └── logic/
```

## Troubleshooting

### API Key Issues

**Problem**: `✗ OPENAI_API_KEY not set`
**Solution**: Ensure `.env` file exists in repository root, not in `research/D1-error-taxonomy/`

**Problem**: `✗ OpenAI API error: Incorrect API key`
**Solution**: Verify key on https://platform.openai.com/api-keys (check for `sk-proj-` prefix)

**Problem**: `✗ Anthropic API error: invalid x-api-key`
**Solution**: Anthropic keys start with `sk-ant-`, verify on console.anthropic.com

### Package Issues

**Problem**: `ModuleNotFoundError: No module named 'openai'`
**Solution**: `pip install openai>=1.0.0`

**Problem**: `openai.BadRequestError: Unrecognized request argument supplied: model`
**Solution**: Upgrade to openai>=1.0.0 (older versions use different API)

### Rate Limiting

**Problem**: `RateLimitError: Rate limit exceeded`
**Solution**:
- OpenAI: Check usage tier (tier 1 = 500 RPM, tier 2+ = higher)
- Anthropic: Default is 50 requests/min for Claude 3.5
- Reduce `batch_size` in `api-config.template.json`

### Cost Management

**Expected Costs** (approximate, as of 2026-01-20):

| Task | Model | Requests | Est. Tokens | Est. Cost |
|------|-------|----------|-------------|-----------|
| T025 | GPT-4 | 50 | ~100k | $1-2 USD |
| T026 | Claude | 50 | ~100k | $0.50-1 USD |
| T027 | DeepSeek | 50 | ~100k | $0.10-0.20 USD |

**Total Phase 3**: ~$2-5 USD

**Cost Controls**:
- Use subsets (50 problems, not full benchmarks)
- Set `max_tokens` limits
- Enable `save_intermediate_results` to avoid re-runs

## Next Steps

After successful verification:

1. **T022-T024**: Download benchmark datasets (HumanEval, MBPP, SWE-bench)
2. **T025-T027**: Run LLM evaluations and collect error outputs
3. **T028-T030**: Categorize error patterns

## References

- OpenAI API Docs: https://platform.openai.com/docs/api-reference
- Anthropic API Docs: https://docs.anthropic.com/claude/reference/
- HumanEval Benchmark: https://github.com/openai/human-eval
- MBPP Benchmark: https://github.com/google-research/google-research/tree/master/mbpp
