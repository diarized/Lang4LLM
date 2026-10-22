#!/usr/bin/env python3
"""
Download MBPP (Mostly Basic Python Problems) Benchmark Subset

Downloads MBPP dataset and creates a subset of 50 problems for error analysis.

Source: https://github.com/google-research/google-research/tree/master/mbpp
License: Apache 2.0

Usage:
    python download_mbpp.py [--subset-size 50]
"""

import json
import argparse
from pathlib import Path
from typing import List, Dict
import urllib.request


def download_mbpp() -> List[Dict]:
    """Download MBPP dataset from GitHub."""
    url = "https://raw.githubusercontent.com/google-research/google-research/master/mbpp/mbpp.jsonl"

    print(f"Downloading MBPP from: {url}")

    try:
        # Download data
        with urllib.request.urlopen(url) as response:
            data = response.read().decode('utf-8')

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
        print("1. Visit: https://github.com/google-research/google-research/tree/master/mbpp")
        print("2. Download mbpp.jsonl")
        print("3. Save to: research/D1-error-taxonomy/data/benchmarks/mbpp/")
        raise


def create_subset(problems: List[Dict], subset_size: int) -> List[Dict]:
    """Create a representative subset of problems."""
    # Filter sanitized problems (clean, well-tested subset)
    # MBPP has different problem sets: sanitized, full, etc.

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
        "problem_ids": [p.get("task_id", p.get("text", "")[:30]) for p in problems[:10]],
        "sample_problem": problems[0] if problems else None,
        "statistics": {
            "avg_prompt_length": sum(len(p.get("text", "")) for p in problems) / len(problems) if problems else 0,
            "avg_test_count": sum(len(p.get("test_list", [])) for p in problems) / len(problems) if problems else 0,
            "has_code_field": sum(1 for p in problems if "code" in p),
            "has_test_list": sum(1 for p in problems if "test_list" in p),
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
    print(f"Has code field: {summary['statistics']['has_code_field']}")
    print(f"Has test list: {summary['statistics']['has_test_list']}")
    print("=" * 60)


def main():
    parser = argparse.ArgumentParser(description="Download MBPP benchmark subset")
    parser.add_argument("--subset-size", type=int, default=50,
                        help="Number of problems to include in subset (default: 50)")
    parser.add_argument("--output-dir", type=str,
                        default="research/D1-error-taxonomy/data/benchmarks/mbpp",
                        help="Output directory for dataset")
    args = parser.parse_args()

    print("=" * 60)
    print("MBPP Benchmark Download")
    print("=" * 60)

    # Download full dataset
    try:
        problems = download_mbpp()
    except Exception:
        return 1

    # Create subset
    subset = create_subset(problems, args.subset_size)

    # Save files
    output_dir = Path(args.output_dir)

    # Save full dataset
    save_dataset(problems, output_dir / "MBPP_full.jsonl", format="jsonl")

    # Save subset
    save_dataset(subset, output_dir / "MBPP_subset.jsonl", format="jsonl")
    save_dataset(subset, output_dir / "MBPP_subset.json", format="json")

    # Generate summary
    generate_summary(subset, output_dir / "MBPP_subset.json")

    print("\n✓ MBPP download complete")
    print("\nNext steps:")
    print("  1. Review: cat research/D1-error-taxonomy/data/benchmarks/mbpp/summary.json")
    print("  2. Run T024: Download SWE-bench benchmark")
    print("  3. Begin LLM evaluations (T025-T027)")

    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
