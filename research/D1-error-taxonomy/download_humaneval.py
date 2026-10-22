#!/usr/bin/env python3
"""
Download HumanEval Benchmark Subset

Downloads HumanEval dataset and creates a subset of 50 problems for error analysis.

Source: https://github.com/openai/human-eval
License: MIT

Usage:
    python download_humaneval.py [--subset-size 50]
"""

import json
import argparse
from pathlib import Path
from typing import List, Dict
import urllib.request


def download_humaneval() -> List[Dict]:
    """Download HumanEval dataset from GitHub."""
    url = "https://raw.githubusercontent.com/openai/human-eval/master/data/HumanEval.jsonl.gz"

    print(f"Downloading HumanEval from: {url}")

    try:
        import gzip
        import io

        # Download compressed data
        with urllib.request.urlopen(url) as response:
            compressed_data = response.read()

        # Decompress
        with gzip.GzipFile(fileobj=io.BytesIO(compressed_data)) as f:
            data = f.read().decode('utf-8')

        # Parse JSONL
        problems = []
        for line in data.strip().split('\n'):
            if line:
                problems.append(json.loads(line))

        print(f"✓ Downloaded {len(problems)} problems")
        return problems

    except Exception as e:
        print(f"✗ Download failed: {e}")
        print("\nFallback: Manual download required")
        print("1. Visit: https://github.com/openai/human-eval")
        print("2. Download data/HumanEval.jsonl.gz")
        print("3. Extract to: research/D1-error-taxonomy/data/benchmarks/humaneval/")
        raise


def create_subset(problems: List[Dict], subset_size: int) -> List[Dict]:
    """Create a representative subset of problems."""
    # Strategy: Select evenly distributed problems across difficulty spectrum
    # Assuming problems are roughly ordered by ID

    if len(problems) <= subset_size:
        return problems

    # Select evenly spaced problems
    step = len(problems) / subset_size
    indices = [int(i * step) for i in range(subset_size)]
    subset = [problems[i] for i in indices]

    print(f"✓ Created subset: {len(subset)} problems (from {len(problems)} total)")
    return subset


def save_dataset(problems: List[Dict], output_path: Path, format: str = "jsonl"):
    """Save problems to file."""
    output_path.parent.mkdir(parents=True, exist_ok=True)

    if format == "jsonl":
        with open(output_path, 'w') as f:
            for problem in problems:
                f.write(json.dumps(problem) + '\n')
    elif format == "json":
        with open(output_path, 'w') as f:
            json.dump(problems, f, indent=2)
    else:
        raise ValueError(f"Unsupported format: {format}")

    print(f"✓ Saved to: {output_path}")


def generate_summary(problems: List[Dict], output_path: Path):
    """Generate a summary report of the dataset."""
    summary = {
        "total_problems": len(problems),
        "problem_ids": [p["task_id"] for p in problems],
        "sample_problem": problems[0] if problems else None,
        "statistics": {
            "avg_prompt_length": sum(len(p["prompt"]) for p in problems) / len(problems) if problems else 0,
            "avg_test_count": sum(len(p.get("test", "").split("assert")) - 1 for p in problems) / len(problems) if problems else 0,
        }
    }

    summary_path = output_path.parent / "summary.json"
    with open(summary_path, 'w') as f:
        json.dump(summary, f, indent=2)

    print(f"✓ Summary saved to: {summary_path}")

    # Print summary
    print("\n" + "=" * 60)
    print("DATASET SUMMARY")
    print("=" * 60)
    print(f"Total problems: {summary['total_problems']}")
    print(f"Problem IDs: {summary['problem_ids'][:5]}... (showing first 5)")
    print(f"Avg prompt length: {summary['statistics']['avg_prompt_length']:.1f} chars")
    print(f"Avg test count: {summary['statistics']['avg_test_count']:.1f}")
    print("=" * 60)


def main():
    parser = argparse.ArgumentParser(description="Download HumanEval benchmark subset")
    parser.add_argument("--subset-size", type=int, default=50,
                        help="Number of problems to include in subset (default: 50)")
    parser.add_argument("--output-dir", type=str,
                        default="research/D1-error-taxonomy/data/benchmarks/humaneval",
                        help="Output directory for dataset")
    args = parser.parse_args()

    print("=" * 60)
    print("HumanEval Benchmark Download")
    print("=" * 60)

    # Download full dataset
    try:
        problems = download_humaneval()
    except Exception:
        return 1

    # Create subset
    subset = create_subset(problems, args.subset_size)

    # Save files
    output_dir = Path(args.output_dir)

    # Save full dataset
    save_dataset(problems, output_dir / "HumanEval_full.jsonl", format="jsonl")

    # Save subset
    save_dataset(subset, output_dir / "HumanEval_subset.jsonl", format="jsonl")
    save_dataset(subset, output_dir / "HumanEval_subset.json", format="json")

    # Generate summary
    generate_summary(subset, output_dir / "HumanEval_subset.json")

    print("\n✓ HumanEval download complete")
    print("\nNext steps:")
    print("  1. Review: cat research/D1-error-taxonomy/data/benchmarks/humaneval/summary.json")
    print("  2. Run T023: Download MBPP benchmark")
    print("  3. Run T024: Download SWE-bench benchmark")

    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
