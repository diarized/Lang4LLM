#!/usr/bin/env python3
"""
LLM Benchmark Evaluation Runner

Runs specified LLM models on benchmark problems and collects error outputs.

Usage:
    # Run single model on single benchmark
    python run_evaluation.py --model gpt4 --benchmark humaneval

    # Run multiple models in sequence
    python run_evaluation.py --model gpt4 --model claude --benchmark humaneval

    # Run all models on all benchmarks
    python run_evaluation.py --all

Prerequisites:
    - API keys configured in .env file (see SETUP.md)
    - verify_api_access.py passed successfully
    - Benchmark datasets downloaded (T022-T024)

Outputs:
    research/D1-error-taxonomy/data/{model}-{benchmark}/
    ├── results.jsonl          # All responses (pass + fail)
    ├── failures.jsonl         # Failed attempts only
    ├── errors.jsonl           # Categorized errors
    └── summary.json           # Statistics
"""

import os
import sys
import json
import time
import argparse
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass, asdict
import logging

try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    pass


# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    handlers=[
        logging.FileHandler('data/evaluation.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)


@dataclass
class EvaluationResult:
    """Single evaluation result."""
    problem_id: str
    model: str
    benchmark: str
    prompt: str
    response: str
    success: bool
    error_type: Optional[str] = None
    error_message: Optional[str] = None
    execution_time: float = 0.0
    timestamp: str = ""


class LLMEvaluator:
    """Base class for LLM evaluation."""

    def __init__(self, model_name: str, api_key: str):
        self.model_name = model_name
        self.api_key = api_key

    def generate(self, prompt: str) -> str:
        """Generate code from prompt. Override in subclasses."""
        raise NotImplementedError

    def evaluate_problem(self, problem: Dict) -> EvaluationResult:
        """Evaluate a single problem."""
        problem_id = problem.get('task_id', problem.get('id', 'unknown'))

        logger.info(f"Evaluating {problem_id} with {self.model_name}")

        # Construct prompt
        prompt = self._construct_prompt(problem)

        # Generate response
        start_time = time.time()
        try:
            response = self.generate(prompt)
            execution_time = time.time() - start_time

            # Validate response (basic syntax check)
            success, error_type, error_msg = self._validate_response(response, problem)

            return EvaluationResult(
                problem_id=problem_id,
                model=self.model_name,
                benchmark="",  # Set by runner
                prompt=prompt,
                response=response,
                success=success,
                error_type=error_type,
                error_message=error_msg,
                execution_time=execution_time,
                timestamp=time.strftime("%Y-%m-%d %H:%M:%S")
            )

        except Exception as e:
            execution_time = time.time() - start_time
            logger.error(f"Generation failed for {problem_id}: {e}")

            return EvaluationResult(
                problem_id=problem_id,
                model=self.model_name,
                benchmark="",
                prompt=prompt,
                response="",
                success=False,
                error_type="GENERATION_ERROR",
                error_message=str(e),
                execution_time=execution_time,
                timestamp=time.strftime("%Y-%m-%d %H:%M:%S")
            )

    def _construct_prompt(self, problem: Dict) -> str:
        """Construct evaluation prompt from problem."""
        # HumanEval format
        if 'prompt' in problem:
            return problem['prompt']

        # MBPP format
        if 'text' in problem:
            return f"Write a Python function that:\n{problem['text']}\n\n"

        # SWE-bench format
        if 'problem_statement' in problem:
            return problem['problem_statement']

        return str(problem)

    def _validate_response(self, response: str, problem: Dict) -> Tuple[bool, Optional[str], Optional[str]]:
        """
        Basic validation of generated code.

        Returns: (success, error_type, error_message)
        """
        # Syntax check (Python)
        try:
            compile(response, '<string>', 'exec')
        except SyntaxError as e:
            return False, "SYNTAX_ERROR", str(e)
        except Exception as e:
            return False, "COMPILE_ERROR", str(e)

        # Basic semantic checks
        if not response.strip():
            return False, "EMPTY_RESPONSE", "Model returned empty response"

        if "TODO" in response or "NotImplemented" in response:
            return False, "INCOMPLETE", "Model returned incomplete implementation"

        # Consider it successful if it compiles
        # Actual correctness requires running tests (not done here)
        return True, None, None


class GPT4Evaluator(LLMEvaluator):
    """OpenAI GPT-4 evaluator."""

    def __init__(self, api_key: str):
        super().__init__("gpt4", api_key)
        try:
            import openai
            self.client = openai.OpenAI(api_key=api_key)
        except ImportError:
            raise ImportError("openai package not installed. Install with: pip install openai")

    def generate(self, prompt: str) -> str:
        """Generate code using GPT-4."""
        response = self.client.chat.completions.create(
            model="gpt-4-turbo-preview",
            messages=[
                {"role": "system", "content": "You are a Python code generator. Generate only code, no explanations."},
                {"role": "user", "content": prompt}
            ],
            temperature=0.0,
            max_tokens=2048
        )
        return response.choices[0].message.content


class ClaudeEvaluator(LLMEvaluator):
    """Anthropic Claude evaluator."""

    def __init__(self, api_key: str):
        super().__init__("claude", api_key)
        try:
            import anthropic
            self.client = anthropic.Anthropic(api_key=api_key)
        except ImportError:
            raise ImportError("anthropic package not installed. Install with: pip install anthropic")

    def generate(self, prompt: str) -> str:
        """Generate code using Claude."""
        response = self.client.messages.create(
            model="claude-3-5-sonnet-20241022",
            max_tokens=4096,
            temperature=0.0,
            messages=[
                {"role": "user", "content": f"Generate Python code only, no explanations:\n\n{prompt}"}
            ]
        )
        return response.content[0].text


class DeepSeekEvaluator(LLMEvaluator):
    """DeepSeek Coder evaluator."""

    def __init__(self, api_key: str):
        super().__init__("deepseek", api_key)
        self.api_key = api_key
        logger.warning("DeepSeek API integration not fully implemented")

    def generate(self, prompt: str) -> str:
        """Generate code using DeepSeek."""
        # TODO: Implement DeepSeek API call
        # DeepSeek may use OpenAI-compatible API
        raise NotImplementedError("DeepSeek API not yet implemented")


def load_benchmark(benchmark_name: str, subset: bool = True) -> List[Dict]:
    """Load benchmark dataset."""
    base_path = Path("data/benchmarks")

    if benchmark_name == "humaneval":
        file_name = "HumanEval_subset.json" if subset else "HumanEval_full.jsonl"
        file_path = base_path / "humaneval" / file_name
    elif benchmark_name == "mbpp":
        file_name = "MBPP_subset.json" if subset else "MBPP_full.jsonl"
        file_path = base_path / "mbpp" / file_name
    elif benchmark_name == "swebench":
        file_name = "SWEbench_subset.json"
        file_path = base_path / "swe-bench" / file_name
    else:
        raise ValueError(f"Unknown benchmark: {benchmark_name}")

    if not file_path.exists():
        raise FileNotFoundError(f"Benchmark file not found: {file_path}")

    # Load JSON or JSONL
    if file_path.suffix == ".json":
        with open(file_path) as f:
            return json.load(f)
    else:  # .jsonl
        with open(file_path) as f:
            return [json.loads(line) for line in f if line.strip()]


def create_evaluator(model_name: str) -> LLMEvaluator:
    """Create evaluator for specified model."""
    if model_name == "gpt4":
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError("OPENAI_API_KEY not set in environment")
        return GPT4Evaluator(api_key)

    elif model_name == "claude":
        api_key = os.getenv("ANTHROPIC_API_KEY")
        if not api_key:
            raise ValueError("ANTHROPIC_API_KEY not set in environment")
        return ClaudeEvaluator(api_key)

    elif model_name == "deepseek":
        api_key = os.getenv("DEEPSEEK_API_KEY")
        if not api_key:
            raise ValueError("DEEPSEEK_API_KEY not set in environment")
        return DeepSeekEvaluator(api_key)

    else:
        raise ValueError(f"Unknown model: {model_name}")


def save_results(results: List[EvaluationResult], output_dir: Path):
    """Save evaluation results."""
    output_dir.mkdir(parents=True, exist_ok=True)

    # Save all results
    with open(output_dir / "results.jsonl", 'w') as f:
        for result in results:
            f.write(json.dumps(asdict(result)) + '\n')

    # Save failures only
    failures = [r for r in results if not r.success]
    with open(output_dir / "failures.jsonl", 'w') as f:
        for result in failures:
            f.write(json.dumps(asdict(result)) + '\n')

    # Generate summary
    summary = {
        "total": len(results),
        "passed": len([r for r in results if r.success]),
        "failed": len(failures),
        "pass_rate": len([r for r in results if r.success]) / len(results) if results else 0,
        "error_types": {},
        "avg_execution_time": sum(r.execution_time for r in results) / len(results) if results else 0
    }

    # Count error types
    for result in failures:
        error_type = result.error_type or "UNKNOWN"
        summary["error_types"][error_type] = summary["error_types"].get(error_type, 0) + 1

    with open(output_dir / "summary.json", 'w') as f:
        json.dump(summary, f, indent=2)

    logger.info(f"Results saved to {output_dir}")
    logger.info(f"Pass rate: {summary['pass_rate']:.1%} ({summary['passed']}/{summary['total']})")


def main():
    parser = argparse.ArgumentParser(description="Run LLM benchmark evaluation")
    parser.add_argument("--model", action="append", choices=["gpt4", "claude", "deepseek"],
                        help="Model to evaluate (can specify multiple)")
    parser.add_argument("--benchmark", choices=["humaneval", "mbpp", "swebench"],
                        default="humaneval", help="Benchmark to use")
    parser.add_argument("--all", action="store_true",
                        help="Run all models on all benchmarks")
    parser.add_argument("--limit", type=int, default=None,
                        help="Limit number of problems to evaluate")
    args = parser.parse_args()

    # Determine which models to run
    if args.all:
        models = ["gpt4", "claude", "deepseek"]
        benchmarks = ["humaneval", "mbpp"]
    else:
        models = args.model or ["gpt4"]
        benchmarks = [args.benchmark]

    print("=" * 60)
    print("LLM Benchmark Evaluation")
    print("=" * 60)
    print(f"Models: {', '.join(models)}")
    print(f"Benchmarks: {', '.join(benchmarks)}")
    print("=" * 60)

    for benchmark_name in benchmarks:
        # Load benchmark
        logger.info(f"Loading benchmark: {benchmark_name}")
        try:
            problems = load_benchmark(benchmark_name)
        except FileNotFoundError as e:
            logger.error(e)
            continue

        if args.limit:
            problems = problems[:args.limit]

        logger.info(f"Loaded {len(problems)} problems")

        for model_name in models:
            logger.info(f"\n{'='*60}")
            logger.info(f"Evaluating {model_name} on {benchmark_name}")
            logger.info(f"{'='*60}")

            try:
                # Create evaluator
                evaluator = create_evaluator(model_name)

                # Run evaluation
                results = []
                for i, problem in enumerate(problems, 1):
                    logger.info(f"Progress: {i}/{len(problems)}")
                    result = evaluator.evaluate_problem(problem)
                    result.benchmark = benchmark_name
                    results.append(result)

                    # Rate limiting
                    time.sleep(1)

                # Save results
                output_dir = Path(f"data/{model_name}-{benchmark_name}")
                save_results(results, output_dir)

            except Exception as e:
                logger.error(f"Evaluation failed for {model_name}: {e}")
                continue

    print("\n✓ Evaluation complete")
    print("\nNext steps:")
    print("  1. Review failure outputs in data/{model}-{benchmark}/failures.jsonl")
    print("  2. Run T028-T030: Categorize errors into taxonomy")

    return 0


if __name__ == "__main__":
    sys.exit(main())
