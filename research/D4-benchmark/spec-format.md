# Benchmark Specification Format

**Purpose**: Template for creating LLM code generation benchmark specifications
**Version**: 1.0
**Date**: 2026-01-20

---

## Overview

Each benchmark specification defines a programming task in natural language, providing sufficient detail for an LLM to generate a correct implementation while remaining technology-agnostic and language-independent.

**Design Principles**:
1. **Natural language only** - No code in specifications
2. **Unambiguous requirements** - Clear acceptance criteria
3. **Self-contained** - No external dependencies unless specified
4. **Testable** - Observable behavior with concrete examples
5. **Complexity-graded** - Tiered by cognitive load (simple/medium/complex)

---

## Template Structure

```markdown
# [Spec ID]: [Task Title]

**Tier**: [Simple | Medium | Complex]
**Category**: [Data Structures | Algorithms | String Processing | Math | File I/O | etc.]
**Estimated Lines of Code**: [10-20 | 30-50 | 60-100+]

## Task Description

[1-3 sentence natural language description of what needs to be implemented]

## Requirements

### Functional Requirements

1. [Requirement 1 with specific behavior]
2. [Requirement 2 with specific behavior]
3. [...]

### Input Specification

- **Parameter 1**: [Type description] - [Constraints, range, format]
- **Parameter 2**: [Type description] - [Constraints, range, format]
- **Return**: [Type description] - [Format, structure]

### Constraints

- [Constraint 1: e.g., time complexity, space complexity]
- [Constraint 2: e.g., input size limits]
- [Edge cases to handle]

## Examples

### Example 1: [Basic case]

**Input**: [Concrete input values]
**Output**: [Expected output]
**Explanation**: [Why this output for this input]

### Example 2: [Edge case]

**Input**: [Concrete input values]
**Output**: [Expected output]
**Explanation**: [Why this output for this input]

### Example 3: [Complex case]

[Additional examples as needed to clarify behavior]

## Test Cases

### Required Coverage

- Basic functionality (happy path)
- Edge cases (empty, null, boundary values)
- Error conditions (invalid input)
- Performance requirements (if specified)

### Minimum Test Count

- Simple tier: 5+ test cases
- Medium tier: 8+ test cases
- Complex tier: 12+ test cases

## Success Criteria

Implementation passes when:
- [ ] All test cases pass
- [ ] Handles all specified edge cases
- [ ] Meets complexity constraints (if specified)
- [ ] Produces correct output format

## Notes

[Any clarifications, hints, or common pitfalls]
```

---

## Tier Definitions

### Simple Tier (BS-S-XXX)

**Complexity**: Single function, straightforward algorithm
**Cognitive Load**: Low (1-2 concepts)
**LOC**: 10-20 lines
**Time**: 5-10 minutes for human expert

**Characteristics**:
- Single responsibility
- No complex data structures
- Minimal control flow (1-2 branches)
- Clear input/output mapping

**Examples**:
- String reversal
- Sum of array
- Check if number is even
- Find maximum in list
- Convert temperature units

**Error Taxonomy Coverage**: Primarily syntax errors, basic semantic errors

### Medium Tier (BS-M-XXX)

**Complexity**: Multiple functions or moderate algorithm
**Cognitive Load**: Medium (3-4 concepts)
**LOC**: 30-50 lines
**Time**: 15-25 minutes for human expert

**Characteristics**:
- Multiple functions or complex single function
- Standard data structures (arrays, dictionaries, sets)
- Moderate control flow (nested loops, multiple branches)
- Requires planning/decomposition

**Examples**:
- Binary search implementation
- Sort with custom comparator
- Parse and validate structured data
- Simple state machine
- Recursive tree traversal

**Error Taxonomy Coverage**: Semantic errors, logic errors, off-by-one errors

### Complex Tier (BS-C-XXX)

**Complexity**: Multi-function module or advanced algorithm
**Cognitive Load**: High (5+ concepts)
**LOC**: 60-100+ lines
**Time**: 30-60 minutes for human expert

**Characteristics**:
- Multiple interdependent functions
- Custom data structures or class hierarchies
- Complex control flow (recursion, state management)
- Requires architectural decisions

**Examples**:
- Implement LRU cache
- Graph algorithms (shortest path, cycle detection)
- Parser for simple grammar
- Concurrent producer-consumer
- Design pattern implementation (observer, strategy)

**Error Taxonomy Coverage**: All error types including architectural mistakes, concurrency bugs

---

## Category Taxonomy

### Data Structures

**Simple**: Array operations, basic list manipulation
**Medium**: Stack, queue, hash table usage
**Complex**: Custom data structures (tree, graph, heap)

### Algorithms

**Simple**: Linear search, basic iteration
**Medium**: Binary search, sorting, simple recursion
**Complex**: Dynamic programming, graph algorithms, backtracking

### String Processing

**Simple**: Concatenation, substring, case conversion
**Medium**: Pattern matching, parsing, validation
**Complex**: Regular expressions, lexical analysis

### Mathematics

**Simple**: Basic arithmetic, rounding, modulo
**Medium**: Prime checking, GCD/LCM, combinatorics
**Complex**: Number theory, numerical methods

### File I/O

**Simple**: Read text file, write to file
**Medium**: CSV parsing, JSON serialization
**Complex**: Binary file handling, streaming

### Concurrency

**Medium**: Thread-safe counter, simple locks
**Complex**: Producer-consumer, thread pools, synchronization

### Object-Oriented Design

**Medium**: Class with encapsulation, inheritance
**Complex**: Design patterns, polymorphism, composition

---

## Language-Agnostic Guidelines

### Avoid Language-Specific Features

❌ **Don't specify**:
- "Use a lambda function"
- "Create a class with properties"
- "Implement using list comprehension"
- "Use async/await"

✓ **Do specify**:
- "Return a function that adds X to its input"
- "Encapsulate state with methods to access it"
- "Transform the list by applying a function to each element"
- "Handle operations that may complete at different times"

### Type Descriptions (Not Type Annotations)

❌ **Don't write**: `numbers: List[int]`
✓ **Do write**: "A list of integers"

❌ **Don't write**: `Optional[str]`
✓ **Do write**: "A string, or null/none if not found"

### Behavioral Specifications

Focus on **what**, not **how**:

❌ **Implementation details**: "Use a hash table to store seen values"
✓ **Behavioral requirement**: "Determine if value has been seen before in O(1) average time"

❌ **Implementation details**: "Loop through array with index"
✓ **Behavioral requirement**: "Process each element in order"

---

## Example Specifications

### Example 1: Simple Tier

```markdown
# BS-S-001: Check Even Number

**Tier**: Simple
**Category**: Mathematics
**Estimated LOC**: 5-10

## Task Description

Determine whether a given integer is even (divisible by 2 with no remainder).

## Requirements

### Functional Requirements

1. Accept a single integer as input
2. Return true/yes if the number is even, false/no if odd
3. Handle negative numbers correctly

### Input Specification

- **number**: An integer (whole number, positive, negative, or zero)
- **Return**: A boolean value (true for even, false for odd)

### Constraints

- Input range: any valid integer
- No external libraries required
- Solution should be constant time O(1)

## Examples

### Example 1: Positive even

**Input**: 4
**Output**: true
**Explanation**: 4 is divisible by 2

### Example 2: Positive odd

**Input**: 7
**Output**: false
**Explanation**: 7 is not divisible by 2

### Example 3: Negative even

**Input**: -2
**Output**: true
**Explanation**: -2 is divisible by 2

### Example 4: Zero

**Input**: 0
**Output**: true
**Explanation**: 0 is divisible by 2

## Test Cases

### Required Coverage

- Positive even numbers
- Positive odd numbers
- Negative even numbers
- Negative odd numbers
- Zero

### Minimum Test Count

5 test cases

## Success Criteria

- [ ] Correctly identifies all even numbers
- [ ] Correctly identifies all odd numbers
- [ ] Handles negative numbers
- [ ] Handles zero

## Notes

Zero is considered even by mathematical convention.
```

### Example 2: Medium Tier

```markdown
# BS-M-015: Binary Search

**Tier**: Medium
**Category**: Algorithms
**Estimated LOC**: 20-30

## Task Description

Implement binary search to find the position of a target value in a sorted list.

## Requirements

### Functional Requirements

1. Search for a target value in a sorted list
2. Return the index (position) of the target if found
3. Return -1 or null if target not found
4. Assume input list is sorted in ascending order
5. Use binary search algorithm (divide-and-conquer approach)

### Input Specification

- **sorted_list**: A list of integers sorted in ascending order (no duplicates)
- **target**: An integer to search for
- **Return**: Integer index (0-based position) if found, -1 if not found

### Constraints

- List length: 0 to 100,000 elements
- Time complexity: O(log n)
- Space complexity: O(1) (iterative) or O(log n) (recursive)
- Do not use built-in search functions

## Examples

### Example 1: Target found

**Input**: sorted_list = [1, 3, 5, 7, 9, 11], target = 7
**Output**: 3
**Explanation**: 7 is at index 3 (0-indexed)

### Example 2: Target not found (too large)

**Input**: sorted_list = [1, 3, 5, 7, 9, 11], target = 12
**Output**: -1
**Explanation**: 12 is not in the list

### Example 3: Target not found (too small)

**Input**: sorted_list = [1, 3, 5, 7, 9, 11], target = 0
**Output**: -1
**Explanation**: 0 is not in the list

### Example 4: Empty list

**Input**: sorted_list = [], target = 5
**Output**: -1
**Explanation**: Cannot find element in empty list

### Example 5: Single element (found)

**Input**: sorted_list = [5], target = 5
**Output**: 0
**Explanation**: 5 is at index 0

## Test Cases

### Required Coverage

- Target at beginning of list
- Target at end of list
- Target in middle of list
- Target not in list (smaller than all elements)
- Target not in list (larger than all elements)
- Target not in list (between elements)
- Empty list
- Single element list (found and not found)

### Minimum Test Count

8 test cases

## Success Criteria

- [ ] Finds target when present
- [ ] Returns -1 when target absent
- [ ] Handles empty list
- [ ] Handles single element
- [ ] Uses O(log n) algorithm (verifiable by step count)

## Notes

Common mistakes:
- Off-by-one errors in index calculations
- Infinite loops when bounds not updated correctly
- Integer overflow in midpoint calculation (use `low + (high - low) / 2`)
```

### Example 3: Complex Tier

```markdown
# BS-C-007: LRU Cache Implementation

**Tier**: Complex
**Category**: Data Structures
**Estimated LOC**: 80-120

## Task Description

Implement a Least Recently Used (LRU) cache that stores key-value pairs with a fixed capacity and evicts the least recently used item when capacity is exceeded.

## Requirements

### Functional Requirements

1. Support `get(key)` operation to retrieve value by key
2. Support `put(key, value)` operation to store or update key-value pair
3. Maintain fixed capacity specified at creation
4. When capacity exceeded, evict least recently used item
5. `get` and `put` operations update the "recently used" status
6. Items are considered "used" when accessed via `get` or updated via `put`

### Input Specification

- **capacity**: Integer (1 to 10,000) - maximum number of items
- **get(key)**: Retrieve value for given key
  - Returns value if key exists
  - Returns null/none if key not found
  - Updates key to most recently used
- **put(key, value)**: Store or update key-value pair
  - If key exists, update value and mark as most recently used
  - If key doesn't exist and cache is full, evict LRU item first
  - If key doesn't exist and cache not full, add new item

### Constraints

- Time complexity: O(1) for both get and put operations
- Space complexity: O(capacity)
- Keys and values can be any comparable type (integers for testing)
- No external caching libraries allowed

## Examples

### Example 1: Basic operations

**Operations**:
```
cache = LRUCache(capacity=2)
cache.put(1, 100)
cache.put(2, 200)
cache.get(1)        // returns 100
cache.put(3, 300)   // evicts key 2 (least recently used)
cache.get(2)        // returns null (was evicted)
cache.get(3)        // returns 300
```

**Explanation**:
- After put(1, 100) and put(2, 200): cache contains {1:100, 2:200}
- After get(1): key 1 is most recent, key 2 is least recent
- After put(3, 300): capacity exceeded, evict key 2, cache contains {1:100, 3:300}
- get(2) returns null (evicted)
- get(3) returns 300 and marks 3 as most recent

### Example 2: Update existing key

**Operations**:
```
cache = LRUCache(capacity=2)
cache.put(1, 100)
cache.put(2, 200)
cache.put(1, 150)   // update value, mark as most recent
cache.put(3, 300)   // evicts key 2
cache.get(1)        // returns 150
```

**Explanation**:
- put(1, 150) updates existing key and marks it most recent
- Eviction chooses key 2 (now least recent)

### Example 3: Capacity 1

**Operations**:
```
cache = LRUCache(capacity=1)
cache.put(1, 100)
cache.put(2, 200)   // evicts key 1
cache.get(1)        // returns null
cache.get(2)        // returns 200
```

## Test Cases

### Required Coverage

- Basic put and get operations
- Eviction when capacity exceeded
- Update existing key (no eviction)
- Get on non-existent key
- Get on evicted key
- Capacity of 1 (edge case)
- Multiple evictions in sequence
- Interleaved get/put affecting LRU order
- Fill to capacity without eviction
- Access pattern testing LRU ordering

### Minimum Test Count

12 test cases

## Success Criteria

- [ ] Get and put operate in O(1) time
- [ ] Correctly evicts LRU item when capacity exceeded
- [ ] Updates recency on both get and put
- [ ] Handles capacity=1 edge case
- [ ] Returns correct values after evictions
- [ ] Updates work correctly (no eviction if key exists)

## Notes

**Common Implementation Strategies**:
- Hash table + doubly linked list
- Hash table keys point to linked list nodes
- Linked list maintains LRU order (head = most recent, tail = least recent)

**Common Mistakes**:
- Not updating recency on get operations
- Not updating recency when updating existing key
- Off-by-one errors in capacity checking
- Not removing evicted keys from hash table
- Inefficient O(n) eviction instead of O(1)

**Data Structure Hints**:
- Need O(1) lookup (hash table)
- Need O(1) removal of arbitrary element (doubly linked list)
- Need O(1) move to front (doubly linked list with head/tail pointers)
```

---

## Validation Checklist

For each specification, verify:

- [ ] **Clarity**: Can specification be understood without code?
- [ ] **Completeness**: All requirements stated explicitly?
- [ ] **Unambiguous**: No multiple valid interpretations?
- [ ] **Testable**: Observable behavior with concrete examples?
- [ ] **Language-agnostic**: No language-specific constructs?
- [ ] **Examples**: At least 3 examples covering different cases?
- [ ] **Edge cases**: Boundary conditions specified?
- [ ] **Success criteria**: Clear pass/fail conditions?

---

## Specification ID Format

**Simple Tier**: `BS-S-XXX` (001-040)
**Medium Tier**: `BS-M-XXX` (001-040)
**Complex Tier**: `BS-C-XXX` (001-020)

**Total**: 100 specifications

---

## Categories Distribution

Target distribution across categories:

| Category | Simple | Medium | Complex | Total |
|----------|--------|--------|---------|-------|
| Data Structures | 8 | 8 | 4 | 20 |
| Algorithms | 8 | 8 | 4 | 20 |
| String Processing | 8 | 8 | 3 | 19 |
| Mathematics | 8 | 6 | 2 | 16 |
| File I/O | 4 | 4 | 2 | 10 |
| Object-Oriented | 0 | 4 | 3 | 7 |
| Concurrency | 0 | 2 | 2 | 4 |
| Miscellaneous | 4 | 0 | 0 | 4 |
| **Total** | **40** | **40** | **20** | **100** |

---

## Design Rules Validation

Each specification should test adherence to design rules from D2:

**Syntax Rules (DR-SYN-XXX)**:
- Specs that benefit from keyword clarity (DR-SYN-001)
- Specs sensitive to tokenization (DR-SYN-002)

**Grammar Rules (DR-GRM-XXX)**:
- Specs requiring clear structure (DR-GRM-001)
- Specs with multiple valid parse interpretations (DR-GRM-003)

**Semantic Rules (DR-SEM-XXX)**:
- Specs requiring type reasoning (DR-SEM-001)
- Specs with error handling (DR-SEM-004)

**Token Efficiency (DR-TOK-XXX)**:
- Specs benefiting from concise syntax (DR-TOK-001)

**Error Prevention (DR-ERR-XXX)**:
- Specs with common pitfalls (DR-ERR-002)
- Specs requiring bounds checking (DR-ERR-003)

---

## References

- D1 Error Taxonomy: Informs edge cases and common mistakes
- D2 Design Guide: Specs validate specific design rules
- D3 Metrics: Specs designed to measure metric impact

---

## Changelog

- 2026-01-20: Initial specification format template created
