#!/usr/bin/env python3
"""
Real API Testing: F-ERR-001 Iterator-First Design
================================================

Tests the hypothesis that iterator-based approaches reduce boundary errors
compared to index-based loops using the Anthropic Claude API.

Experiment Design:
- 12 boundary-sensitive coding problems
- 2 variants per problem: iterator-first vs index-based
- Claude Sonnet 4.5 model
- Measures: boundary errors, off-by-one errors, correctness
"""

import os
import json
import time
from pathlib import Path
from typing import Dict, List, Tuple
from dataclasses import dataclass, asdict
from anthropic import Anthropic

# Test configuration
API_KEY_PATH = Path.home() / "Documents/AI_API_keys/LangLLM.key"
MODEL = "claude-sonnet-4-5-20250929"
MAX_TOKENS = 2000
TEMPERATURE = 0.7

# Results directory
RESULTS_DIR = Path(__file__).parent / "results"
RESULTS_DIR.mkdir(exist_ok=True)


@dataclass
class TestCase:
    """Represents a single test case."""
    id: str
    name: str
    description: str
    test_cases: List[Dict]
    difficulty: str  # easy, medium, hard
    boundary_type: str  # empty, single, off_by_one, wraparound


@dataclass
class TestResult:
    """Results from running a test case."""
    test_id: str
    variant: str  # iterator or index
    code: str
    passed: bool
    boundary_error: bool
    off_by_one: bool
    error_type: str
    execution_time: float


# Test cases focusing on boundary conditions
TEST_CASES = [
    TestCase(
        id="TC-001",
        name="Find Maximum in Array",
        description="Find the maximum value in an array of integers",
        test_cases=[
            {"input": [1, 5, 3, 9, 2], "expected": 9},
            {"input": [42], "expected": 42},
            {"input": [-5, -2, -10], "expected": -2},
            {"input": [], "expected": None},
        ],
        difficulty="easy",
        boundary_type="empty"
    ),
    TestCase(
        id="TC-002",
        name="Array Rotation",
        description="Rotate array to the right by k steps",
        test_cases=[
            {"input": ([1, 2, 3, 4, 5], 2), "expected": [4, 5, 1, 2, 3]},
            {"input": ([1], 3), "expected": [1]},
            {"input": ([1, 2, 3], 0), "expected": [1, 2, 3]},
            {"input": ([], 5), "expected": []},
        ],
        difficulty="medium",
        boundary_type="wraparound"
    ),
    TestCase(
        id="TC-003",
        name="First Missing Positive",
        description="Find the smallest missing positive integer",
        test_cases=[
            {"input": [1, 2, 0], "expected": 3},
            {"input": [3, 4, -1, 1], "expected": 2},
            {"input": [7, 8, 9, 11, 12], "expected": 1},
            {"input": [], "expected": 1},
        ],
        difficulty="hard",
        boundary_type="empty"
    ),
    TestCase(
        id="TC-004",
        name="Remove Duplicates",
        description="Remove duplicates from sorted array in-place",
        test_cases=[
            {"input": [1, 1, 2], "expected": ([1, 2], 2)},
            {"input": [0, 0, 1, 1, 1, 2, 2, 3, 3, 4], "expected": ([0, 1, 2, 3, 4], 5)},
            {"input": [1], "expected": ([1], 1)},
            {"input": [], "expected": ([], 0)},
        ],
        difficulty="easy",
        boundary_type="single"
    ),
    TestCase(
        id="TC-005",
        name="Sliding Window Maximum",
        description="Find maximum in each sliding window of size k",
        test_cases=[
            {"input": ([1, 3, -1, -3, 5, 3, 6, 7], 3), "expected": [3, 3, 5, 5, 6, 7]},
            {"input": ([1], 1), "expected": [1]},
            {"input": ([1, -1], 1), "expected": [1, -1]},
            {"input": ([9, 11], 2), "expected": [11]},
        ],
        difficulty="hard",
        boundary_type="off_by_one"
    ),
    TestCase(
        id="TC-006",
        name="Binary Search",
        description="Implement binary search on sorted array",
        test_cases=[
            {"input": ([1, 2, 3, 4, 5], 3), "expected": 2},
            {"input": ([1, 2, 3, 4, 5], 6), "expected": -1},
            {"input": ([5], 5), "expected": 0},
            {"input": ([], 1), "expected": -1},
        ],
        difficulty="easy",
        boundary_type="off_by_one"
    ),
    TestCase(
        id="TC-007",
        name="Two Sum",
        description="Find indices of two numbers that add up to target",
        test_cases=[
            {"input": ([2, 7, 11, 15], 9), "expected": [0, 1]},
            {"input": ([3, 2, 4], 6), "expected": [1, 2]},
            {"input": ([3, 3], 6), "expected": [0, 1]},
            {"input": ([1], 1), "expected": None},
        ],
        difficulty="easy",
        boundary_type="single"
    ),
    TestCase(
        id="TC-008",
        name="Merge Sorted Arrays",
        description="Merge two sorted arrays into first array",
        test_cases=[
            {"input": ([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3), "expected": [1, 2, 2, 3, 5, 6]},
            {"input": ([1], 1, [], 0), "expected": [1]},
            {"input": ([0], 0, [1], 1), "expected": [1]},
            {"input": ([2, 0], 1, [1], 1), "expected": [1, 2]},
        ],
        difficulty="medium",
        boundary_type="empty"
    ),
    TestCase(
        id="TC-009",
        name="Longest Consecutive Sequence",
        description="Find length of longest consecutive elements sequence",
        test_cases=[
            {"input": [100, 4, 200, 1, 3, 2], "expected": 4},
            {"input": [0, 3, 7, 2, 5, 8, 4, 6, 0, 1], "expected": 9},
            {"input": [1], "expected": 1},
            {"input": [], "expected": 0},
        ],
        difficulty="hard",
        boundary_type="empty"
    ),
    TestCase(
        id="TC-010",
        name="Product Except Self",
        description="Return array where output[i] = product of all elements except nums[i]",
        test_cases=[
            {"input": [1, 2, 3, 4], "expected": [24, 12, 8, 6]},
            {"input": [-1, 1, 0, -3, 3], "expected": [0, 0, 9, 0, 0]},
            {"input": [2, 3], "expected": [3, 2]},
            {"input": [5], "expected": [1]},
        ],
        difficulty="medium",
        boundary_type="single"
    ),
    TestCase(
        id="TC-011",
        name="Trapping Rain Water",
        description="Calculate how much water can be trapped after raining",
        test_cases=[
            {"input": [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], "expected": 6},
            {"input": [4, 2, 0, 3, 2, 5], "expected": 9},
            {"input": [1], "expected": 0},
            {"input": [], "expected": 0},
        ],
        difficulty="hard",
        boundary_type="empty"
    ),
    TestCase(
        id="TC-012",
        name="Valid Palindrome Check",
        description="Check if array reads same forwards and backwards",
        test_cases=[
            {"input": [1, 2, 3, 2, 1], "expected": True},
            {"input": [1, 2], "expected": False},
            {"input": [5], "expected": True},
            {"input": [], "expected": True},
        ],
        difficulty="easy",
        boundary_type="empty"
    ),
]


def load_api_key() -> str:
    """Load Anthropic API key from file."""
    return API_KEY_PATH.read_text().strip()


def create_prompt_iterator(test_case: TestCase) -> str:
    """Create prompt emphasizing iterator-based approach."""
    return f"""Write a Python function to solve this problem using iterator-based patterns:

Problem: {test_case.name}
{test_case.description}

Requirements:
- Use iterator-based patterns (for item in collection, enumerate, list comprehensions)
- Avoid manual index management (for i in range(len(...)))
- Use safe access methods when indexing is needed
- Handle empty collections gracefully

Test cases:
{json.dumps(test_case.test_cases, indent=2)}

Return only the Python function code, no explanations."""


def create_prompt_index(test_case: TestCase) -> str:
    """Create prompt emphasizing index-based approach."""
    return f"""Write a Python function to solve this problem using index-based loops:

Problem: {test_case.name}
{test_case.description}

Requirements:
- Use index-based loops (for i in range(len(...)))
- Access elements via indexing (array[i])
- Manage loop boundaries manually

Test cases:
{json.dumps(test_case.test_cases, indent=2)}

Return only the Python function code, no explanations."""


def call_claude_api(prompt: str, client: Anthropic) -> str:
    """Call Claude API with given prompt."""
    message = client.messages.create(
        model=MODEL,
        max_tokens=MAX_TOKENS,
        temperature=TEMPERATURE,
        messages=[{"role": "user", "content": prompt}]
    )
    return message.content[0].text


def extract_code(response: str) -> str:
    """Extract Python code from response."""
    # Remove markdown code blocks if present
    if "```python" in response:
        code = response.split("```python")[1].split("```")[0].strip()
    elif "```" in response:
        code = response.split("```")[1].split("```")[0].strip()
    else:
        code = response.strip()
    return code


def check_boundary_errors(code: str, test_case: TestCase) -> Tuple[bool, bool, str]:
    """
    Analyze code for boundary errors.

    Returns: (has_boundary_error, has_off_by_one, error_description)
    """
    boundary_error = False
    off_by_one = False
    error_desc = "No errors detected"

    # Check for common boundary error patterns
    patterns = {
        "unchecked_index": r"arr\[i\]" in code and "if" not in code.lower(),
        "off_by_one_range": "range(len(" in code and "- 1" not in code and "+ 1" not in code,
        "empty_access": "arr[0]" in code and "if arr" not in code and "if len(arr)" not in code,
        "missing_bounds_check": "[" in code and "]" in code and "IndexError" not in code,
    }

    # Detect off-by-one patterns
    if "range(len(" in code:
        # Check for correct range usage
        if "range(len(" in code and "- 1" in code:
            off_by_one = True
            error_desc = "Potential off-by-one: manual length adjustment"

    # Detect boundary errors
    if any(patterns.values()):
        boundary_error = True
        error_desc = f"Boundary error patterns detected: {[k for k, v in patterns.items() if v]}"

    return boundary_error, off_by_one, error_desc


def test_code_execution(code: str, test_case: TestCase) -> Tuple[bool, str]:
    """
    Execute code against test cases.

    Returns: (all_passed, error_message)
    """
    try:
        # Create a safe execution environment
        exec_globals = {}
        exec(code, exec_globals)

        # Find the function (assume it's the first defined function)
        func_name = None
        for name, obj in exec_globals.items():
            if callable(obj) and not name.startswith("__"):
                func_name = name
                break

        if not func_name:
            return False, "No function found in code"

        func = exec_globals[func_name]

        # Run test cases
        for i, test in enumerate(test_case.test_cases):
            input_data = test["input"]
            expected = test["expected"]

            # Handle different input formats
            if isinstance(input_data, tuple):
                result = func(*input_data)
            else:
                result = func(input_data)

            # Compare result
            if isinstance(expected, tuple):
                if result != expected:
                    return False, f"Test case {i+1} failed: expected {expected}, got {result}"
            else:
                if result != expected:
                    return False, f"Test case {i+1} failed: expected {expected}, got {result}"

        return True, "All tests passed"

    except IndexError as e:
        return False, f"IndexError (boundary error): {str(e)}"
    except Exception as e:
        return False, f"Execution error: {type(e).__name__}: {str(e)}"


def run_test_case(test_case: TestCase, variant: str, client: Anthropic) -> TestResult:
    """Run a single test case with given variant."""
    start_time = time.time()

    # Create prompt
    if variant == "iterator":
        prompt = create_prompt_iterator(test_case)
    else:
        prompt = create_prompt_index(test_case)

    # Call API
    print(f"  Testing {test_case.id} ({variant})...", end=" ", flush=True)
    response = call_claude_api(prompt, client)
    code = extract_code(response)

    # Analyze code
    has_boundary_error, has_off_by_one, error_desc = check_boundary_errors(code, test_case)

    # Execute tests
    all_passed, exec_error = test_code_execution(code, test_case)

    execution_time = time.time() - start_time

    result = TestResult(
        test_id=test_case.id,
        variant=variant,
        code=code,
        passed=all_passed,
        boundary_error=has_boundary_error,
        off_by_one=has_off_by_one,
        error_type=error_desc if not all_passed else "None",
        execution_time=execution_time
    )

    status = "✓" if all_passed else "✗"
    print(f"{status} ({execution_time:.1f}s)")

    return result


def analyze_results(results: List[TestResult]) -> Dict:
    """Analyze test results and compute metrics."""
    iterator_results = [r for r in results if r.variant == "iterator"]
    index_results = [r for r in results if r.variant == "index"]

    def compute_metrics(results_subset):
        total = len(results_subset)
        passed = sum(1 for r in results_subset if r.passed)
        boundary_errors = sum(1 for r in results_subset if r.boundary_error)
        off_by_one_errors = sum(1 for r in results_subset if r.off_by_one)

        return {
            "total": total,
            "passed": passed,
            "pass_rate": passed / total if total > 0 else 0,
            "boundary_errors": boundary_errors,
            "boundary_error_rate": boundary_errors / total if total > 0 else 0,
            "off_by_one_errors": off_by_one_errors,
            "off_by_one_rate": off_by_one_errors / total if total > 0 else 0,
        }

    iterator_metrics = compute_metrics(iterator_results)
    index_metrics = compute_metrics(index_results)

    # Compute improvements
    improvement = {
        "pass_rate_delta": iterator_metrics["pass_rate"] - index_metrics["pass_rate"],
        "boundary_error_reduction": (
            (index_metrics["boundary_error_rate"] - iterator_metrics["boundary_error_rate"]) /
            index_metrics["boundary_error_rate"] * 100
            if index_metrics["boundary_error_rate"] > 0 else 0
        ),
        "off_by_one_reduction": (
            (index_metrics["off_by_one_rate"] - iterator_metrics["off_by_one_rate"]) /
            index_metrics["off_by_one_rate"] * 100
            if index_metrics["off_by_one_rate"] > 0 else 0
        ),
    }

    return {
        "iterator": iterator_metrics,
        "index": index_metrics,
        "improvement": improvement,
    }


def save_results(results: List[TestResult], analysis: Dict):
    """Save results to JSON files."""
    timestamp = time.strftime("%Y%m%d_%H%M%S")

    # Save raw results
    results_file = RESULTS_DIR / f"test_results_{timestamp}.json"
    with open(results_file, "w") as f:
        json.dump([asdict(r) for r in results], f, indent=2)

    # Save analysis
    analysis_file = RESULTS_DIR / f"analysis_{timestamp}.json"
    with open(analysis_file, "w") as f:
        json.dump(analysis, f, indent=2)

    print(f"\nResults saved:")
    print(f"  - {results_file}")
    print(f"  - {analysis_file}")


def print_summary(analysis: Dict):
    """Print summary of results."""
    print("\n" + "="*70)
    print("TEST SUMMARY - F-ERR-001 Iterator-First Design")
    print("="*70)

    print("\nIterator-Based Approach:")
    print(f"  Pass Rate: {analysis['iterator']['pass_rate']*100:.1f}%")
    print(f"  Boundary Errors: {analysis['iterator']['boundary_errors']}/{analysis['iterator']['total']}")
    print(f"  Off-by-One Errors: {analysis['iterator']['off_by_one_errors']}/{analysis['iterator']['total']}")

    print("\nIndex-Based Approach:")
    print(f"  Pass Rate: {analysis['index']['pass_rate']*100:.1f}%")
    print(f"  Boundary Errors: {analysis['index']['boundary_errors']}/{analysis['index']['total']}")
    print(f"  Off-by-One Errors: {analysis['index']['off_by_one_errors']}/{analysis['index']['total']}")

    print("\nImprovement (Iterator vs Index):")
    print(f"  Pass Rate Delta: {analysis['improvement']['pass_rate_delta']*100:+.1f}%")
    print(f"  Boundary Error Reduction: {analysis['improvement']['boundary_error_reduction']:+.1f}%")
    print(f"  Off-by-One Reduction: {analysis['improvement']['off_by_one_reduction']:+.1f}%")

    print("\n" + "="*70)

    # Compare to hypothesis
    expected_improvement = 39.2
    expected_boundary_reduction = 81.8

    print("\nHypothesis Validation:")
    print(f"  Expected Improvement: +{expected_improvement}% pass rate")
    print(f"  Actual Improvement: {analysis['improvement']['pass_rate_delta']*100:+.1f}% pass rate")
    print(f"  Expected Boundary Reduction: -{expected_boundary_reduction}%")
    print(f"  Actual Boundary Reduction: {analysis['improvement']['boundary_error_reduction']:+.1f}%")

    if analysis['improvement']['boundary_error_reduction'] >= expected_boundary_reduction * 0.7:
        print("\n  ✓ HYPOTHESIS CONFIRMED: Iterator-first design significantly reduces boundary errors")
    else:
        print("\n  ⚠ HYPOTHESIS NEEDS REVIEW: Results differ from expectations")


def main():
    """Run the API testing experiment."""
    print("="*70)
    print("API Testing: F-ERR-001 Iterator-First Design")
    print("="*70)
    print(f"\nConfiguration:")
    print(f"  Model: {MODEL}")
    print(f"  Test Cases: {len(TEST_CASES)}")
    print(f"  Variants: iterator, index")
    print(f"  Total API Calls: {len(TEST_CASES) * 2}")
    print(f"  Estimated Cost: ~${len(TEST_CASES) * 2 * 0.015:.2f}")
    print()

    # Load API key
    api_key = load_api_key()
    client = Anthropic(api_key=api_key)

    # Run tests
    results = []
    for test_case in TEST_CASES:
        print(f"\nTest Case: {test_case.name} ({test_case.difficulty}, {test_case.boundary_type})")

        # Test iterator variant
        result_iterator = run_test_case(test_case, "iterator", client)
        results.append(result_iterator)

        # Small delay to avoid rate limiting
        time.sleep(0.5)

        # Test index variant
        result_index = run_test_case(test_case, "index", client)
        results.append(result_index)

        time.sleep(0.5)

    # Analyze results
    print("\n" + "="*70)
    print("Analyzing results...")
    analysis = analyze_results(results)

    # Save results
    save_results(results, analysis)

    # Print summary
    print_summary(analysis)


if __name__ == "__main__":
    main()
