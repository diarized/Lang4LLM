#!/usr/bin/env python3
"""
Download SWE-bench (Software Engineering Benchmark) Subset

Downloads SWE-bench dataset and creates a representative sample for error analysis.

Source: https://www.swebench.com / https://huggingface.co/datasets/princeton-nlp/SWE-bench
License: MIT

Usage:
    python download_swebench.py [--subset-size 20]

Note: SWE-bench is large (2k+ instances). We use a small representative sample.
"""

import json
import argparse
from pathlib import Path
from typing import List, Dict
import urllib.request


def download_swebench_lite() -> List[Dict]:
    """
    Download SWE-bench Lite (smaller, curated subset).

    SWE-bench Lite is a filtered version with 300 instances vs 2,294 in full SWE-bench.
    More suitable for research purposes.
    """
    # Using the public Hugging Face dataset files
    url = "https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite/raw/main/data/test-00000-of-00001.parquet"

    print(f"Downloading SWE-bench Lite from Hugging Face...")
    print(f"Note: This requires pandas and pyarrow libraries")

    try:
        import pandas as pd

        # Download parquet file
        df = pd.read_parquet(url)

        # Convert to list of dicts
        problems = df.to_dict('records')

        print(f"✓ Downloaded {len(problems)} problems from SWE-bench Lite")
        return problems

    except ImportError as e:
        print(f"✗ Missing dependencies: {e}")
        print("\nInstall required packages:")
        print("  pip install pandas pyarrow")
        raise
    except Exception as e:
        print(f"✗ Download failed: {e}")
        print("\nFallback options:")
        print("1. Install dependencies: pip install datasets")
        print("2. Use datasets library to download SWE-bench")
        print("3. Visit: https://www.swebench.com")
        raise


def download_with_datasets_library() -> List[Dict]:
    """Alternative download using Hugging Face datasets library."""
    try:
        from datasets import load_dataset

        print("Downloading SWE-bench Lite using datasets library...")
        dataset = load_dataset("princeton-nlp/SWE-bench_Lite", split="test")

        problems = [dict(item) for item in dataset]
        print(f"✓ Downloaded {len(problems)} problems")
        return problems

    except ImportError:
        print("✗ datasets library not installed")
        print("  Install with: pip install datasets")
        raise
    except Exception as e:
        print(f"✗ Download failed: {e}")
        raise


def create_subset(problems: List[Dict], subset_size: int) -> List[Dict]:
    """Create a representative subset of problems."""
    if len(problems) <= subset_size:
        return problems

    # Strategy: Select diverse problems across different repositories
    # Try to get variety in:
    # - Difficulty (if available)
    # - Repository (if available)
    # - Problem type

    # For now, select evenly spaced problems
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
                f.write(json.dumps(problem, default=str) + '\n')
    elif format == "json":
        with open(output_path, 'w') as f:
            json.dump(problems, f, indent=2, default=str)
    else:
        raise ValueError(f"Unsupported format: {format}")

    print(f"✓ Saved to: {output_path}")


def generate_summary(problems: List[Dict], output_path: Path):
    """Generate a summary report of the dataset."""
    # Analyze problem structure
    repos = set()
    languages = set()

    for p in problems:
        if "repo" in p:
            repos.add(p["repo"])
        if "language" in p:
            languages.add(p["language"])
        elif "instance_id" in p:
            # Extract repo from instance_id (format: repo__issue)
            repo_part = p["instance_id"].split("__")[0] if "__" in p["instance_id"] else ""
            if repo_part:
                repos.add(repo_part)

    summary = {
        "total_problems": len(problems),
        "unique_repositories": len(repos),
        "languages": list(languages) if languages else ["Unknown"],
        "sample_instance_ids": [p.get("instance_id", "")[:50] for p in problems[:5]],
        "statistics": {
            "avg_problem_statement_length": sum(len(str(p.get("problem_statement", ""))) for p in problems) / len(problems) if problems else 0,
            "has_test_patch": sum(1 for p in problems if "test_patch" in p),
            "has_gold_patch": sum(1 for p in problems if "patch" in p or "gold_patch" in p),
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
    print(f"Unique repositories: {summary['unique_repositories']}")
    print(f"Languages: {', '.join(summary['languages'])}")
    print(f"Sample IDs: {summary['sample_instance_ids'][:3]}...")
    print(f"Avg problem statement: {summary['statistics']['avg_problem_statement_length']:.1f} chars")
    print(f"Has test patch: {summary['statistics']['has_test_patch']}")
    print(f"Has gold patch: {summary['statistics']['has_gold_patch']}")
    print("=" * 60)


def main():
    parser = argparse.ArgumentParser(description="Download SWE-bench subset")
    parser.add_argument("--subset-size", type=int, default=20,
                        help="Number of problems to include in subset (default: 20)")
    parser.add_argument("--output-dir", type=str,
                        default="research/D1-error-taxonomy/data/benchmarks/swe-bench",
                        help="Output directory for dataset")
    parser.add_argument("--use-datasets-lib", action="store_true",
                        help="Use Hugging Face datasets library instead of direct download")
    args = parser.parse_args()

    print("=" * 60)
    print("SWE-bench Benchmark Download")
    print("=" * 60)
    print("Note: Using SWE-bench Lite (300 problems) instead of full (2,294)")
    print("=" * 60)

    # Download full dataset
    try:
        if args.use_datasets_lib:
            problems = download_with_datasets_library()
        else:
            problems = download_swebench_lite()
    except Exception:
        print("\n⚠ Download failed. Trying alternative method...")
        try:
            problems = download_with_datasets_library()
        except Exception:
            print("\n✗ All download methods failed")
            print("\nManual download:")
            print("1. Visit: https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite")
            print("2. Download dataset")
            print("3. Save to: research/D1-error-taxonomy/data/benchmarks/swe-bench/")
            return 1

    # Create subset
    subset = create_subset(problems, args.subset_size)

    # Save files
    output_dir = Path(args.output_dir)

    # Save full dataset (Lite version)
    save_dataset(problems, output_dir / "SWEbench_Lite_full.jsonl", format="jsonl")

    # Save subset
    save_dataset(subset, output_dir / "SWEbench_subset.jsonl", format="jsonl")
    save_dataset(subset, output_dir / "SWEbench_subset.json", format="json")

    # Generate summary
    generate_summary(subset, output_dir / "SWEbench_subset.json")

    print("\n✓ SWE-bench download complete")
    print("\nBenchmark summary:")
    print("  - HumanEval: 50 problems (function-level Python)")
    print("  - MBPP: 50 problems (basic Python programming)")
    print("  - SWE-bench: 20 problems (repository-level software engineering)")
    print("\nNext steps:")
    print("  1. Verify API access: python verify_api_access.py")
    print("  2. Run T025-T027: Execute LLM evaluations on benchmarks")

    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
