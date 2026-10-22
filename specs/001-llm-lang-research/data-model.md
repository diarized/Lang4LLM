# Data Model: Research Artifacts

**Branch**: `001-llm-lang-research` | **Date**: 2026-01-20
**Purpose**: Define structure and relationships of research deliverables

## Entity Overview

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Error Pattern  │────▶│  Design Rule    │────▶│ Feature Valid.  │
│     (D1)        │     │     (D2)        │     │     (D5)        │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                       │                       ▲
        │                       │                       │
        ▼                       ▼                       │
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│     Metric      │◀───▶│   Benchmark     │─────│  Benchmark      │
│     (D3)        │     │   Spec (D4)     │     │  Result         │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## D1: Error Pattern

An error pattern represents a categorized LLM code generation failure mode.

### Schema

```yaml
ErrorPattern:
  id: string          # Format: EP-{category}-{number} (e.g., EP-SYN-001)
  category: enum      # SYNTAX | SEMANTIC | LOGIC
  subcategory: string # e.g., "boundary", "type-mismatch", "missing-condition"
  name: string        # Human-readable name
  description: string # Detailed explanation

  frequency:
    overall: float    # 0.0-1.0, proportion of all errors
    by_model:         # Per-model frequency
      gpt4: float
      claude: float
      deepseek: float
    by_language:      # Per-language frequency
      python: float
      rust: float
      typescript: float
      go: float

  root_cause: string  # Why LLMs make this error

  trigger_conditions: # Language features that enable this error
    - feature: string
      severity: enum  # HIGH | MEDIUM | LOW

  examples:           # Code examples demonstrating the error
    - language: string
      incorrect: string
      correct: string
      explanation: string

  related_patterns:   # Cross-references
    - pattern_id: string
      relationship: enum  # CAUSES | CAUSED_BY | SIMILAR_TO

  sources:            # Research citations
    - paper_id: string
      section: string
```

### Validation Rules

- `id` must be unique across all patterns
- `frequency.overall` must sum to 1.0 across all patterns (within category)
- At least one `example` required
- At least one `source` required

## D2: Design Rule

A design rule is a concrete, testable guideline for language design.

### Schema

```yaml
DesignRule:
  id: string           # Format: DR-{category}-{number} (e.g., DR-SYN-001)
  category: enum       # SYNTAX | GRAMMAR | SEMANTIC | TOKEN | ERROR_PREVENTION
  name: string         # Short name (e.g., "Explicit Delimiters")
  description: string  # Full rule statement

  priority: int        # 1 = highest impact, higher = lower impact
  impact_score: float  # Estimated reduction in error rate (0.0-1.0)

  rationale: string    # Why this rule helps LLMs

  research_backing:    # Evidence supporting the rule
    - source: string   # Paper ID or URL
      finding: string  # Relevant finding
      confidence: enum # HIGH | MEDIUM | LOW

  validation_method:   # How to test this rule
    approach: string   # Description of validation approach
    metrics:           # Which metrics to measure
      - metric_id: string
    benchmark_specs:   # Which benchmarks to use
      - spec_id: string

  counter_examples:    # When NOT to apply this rule
    - scenario: string
      reason: string
      alternative: string

  related_patterns:    # Error patterns this rule addresses
    - pattern_id: string
      mitigation: enum # PREVENTS | REDUCES | DETECTS

  implementation_notes: # Guidance for language implementers
    - note: string
```

### Validation Rules

- `id` must be unique across all rules
- `priority` must be unique (no ties)
- At least one `research_backing` entry with confidence HIGH or MEDIUM
- At least one `related_patterns` entry
- `validation_method` must be executable (not vague)

## D3: Metric

A metric quantifies LLM-friendliness of language design choices.

### Schema

```yaml
Metric:
  id: string           # Format: M-{number} (e.g., M-001)
  name: string         # e.g., "Token Predictability Score"
  abbreviation: string # e.g., "TPS"

  definition: string   # Formal mathematical definition
  interpretation: string # What the metric measures (plain language)

  computation:
    formula: string    # Mathematical formula
    algorithm: string  # Step-by-step computation method
    complexity: string # Time/space complexity
    tools_required:    # Software needed
      - tool: string
        purpose: string

  range:
    min: float
    max: float
    optimal: string    # "higher" | "lower" | "target value"

  correlation:         # Correlation with benchmark performance
    benchmark: string  # Which benchmark
    coefficient: float # Spearman/Pearson coefficient
    p_value: float     # Statistical significance
    sample_size: int   # Number of data points

  language_scores:     # Computed scores for reference languages
    python: float
    rust: float
    typescript: float
    go: float

  limitations:         # Known issues with this metric
    - limitation: string
      mitigation: string
```

### Validation Rules

- `correlation.p_value` must be < 0.05 for metric to be valid
- `computation.algorithm` must be reproducible
- `language_scores` must include all 4 target languages

## D4: Benchmark Specification

A benchmark specification defines a test case for validating language features.

### Schema

```yaml
BenchmarkSpec:
  id: string           # Format: BS-{tier}-{number} (e.g., BS-S-001)
  tier: enum           # SIMPLE | MEDIUM | COMPLEX
  name: string         # Short descriptive name

  specification:       # Natural language spec
    description: string
    inputs:
      - name: string
        type: string   # Abstract type (not language-specific)
        constraints: string
    outputs:
      - name: string
        type: string
        constraints: string
    examples:
      - input: any
        output: any
        explanation: string

  features_tested:     # Language features this spec validates
    - feature: string
      aspect: string   # What aspect of the feature

  complexity_factors:  # Why this tier
    - factor: string
      contribution: enum # HIGH | MEDIUM | LOW

  reference_implementation:
    language: string   # Python (canonical)
    code: string       # Working implementation
    verified: boolean  # Has been tested

  test_cases:
    - name: string
      input: any
      expected_output: any
      category: enum   # BASIC | EDGE | CORNER | STRESS

  edge_cases:          # Explicitly documented edge cases
    - description: string
      input: any
      expected_behavior: string

  related_specs:       # Cross-references
    - spec_id: string
      relationship: string
```

### Validation Rules

- `id` must be unique
- `tier` must match complexity heuristics (SIMPLE: 1 function, MEDIUM: 2-5 functions, COMPLEX: modules)
- `reference_implementation.verified` must be true
- At least 3 `test_cases` required
- At least 1 `edge_cases` entry required

## D5: Feature Validation

A feature validation records the assessment of a proposed language feature.

### Schema

```yaml
FeatureValidation:
  id: string           # Format: FV-{feature_id} (e.g., FV-explicit-delimiters)
  feature_name: string # Name of proposed language feature
  feature_description: string

  related_rules:       # Design rules that recommend this feature
    - rule_id: string

  benchmark_results:
    specs_tested: int  # Number of benchmark specs used
    pass_rate:
      overall: float   # 0.0-1.0
      by_tier:
        simple: float
        medium: float
        complex: float
      by_model:
        gpt4: float
        claude: float
        deepseek: float

  comparison:          # vs alternative designs
    baseline: string   # What we're comparing against
    improvement: float # Percentage improvement over baseline
    statistical_significance: float # p-value

  failure_analysis:    # Common failures with this feature
    - failure_mode: string
      frequency: float
      root_cause: string

  recommendation: enum # INCLUDE | EXCLUDE | MODIFY
  confidence: enum     # HIGH | MEDIUM | LOW

  rationale: string    # Why this recommendation

  modification_needed: # Only if recommendation = MODIFY
    current_design: string
    proposed_change: string
    expected_improvement: string

  alternatives:        # Other designs considered
    - design: string
      pass_rate: float
      why_rejected: string
```

### Validation Rules

- `id` must be unique
- `benchmark_results.specs_tested` must be >= 10
- `recommendation` must be justified by `rationale`
- If `recommendation` = MODIFY, `modification_needed` must be populated
- `confidence` HIGH requires `comparison.statistical_significance` < 0.01

## Relationships Summary

| From | To | Relationship | Cardinality |
|------|-----|--------------|-------------|
| ErrorPattern | DesignRule | addresses | N:M |
| DesignRule | ErrorPattern | mitigates | N:M |
| DesignRule | BenchmarkSpec | validated_by | 1:N |
| BenchmarkSpec | DesignRule | tests | N:M |
| Metric | BenchmarkSpec | computed_from | 1:N |
| FeatureValidation | DesignRule | evaluates | N:1 |
| FeatureValidation | BenchmarkSpec | uses | 1:N |

---

## Phase 3: Transpiler Entities

### TranspileUnit

Represents a single transpilation pass from Lang4LLM AST to Rust source.

```yaml
TranspileUnit:
  input: Program              # Parsed + type-checked AST
  output: String              # Generated Rust source code
  errors: Vec<TranspileError> # Errors encountered during codegen
  metadata:
    source_file: String       # Original .lang4llm file path
    lang4llm_version: String  # Compiler version
    generated_at: DateTime    # Timestamp
```

### CodegenContext

Mutable state during transpilation.

```yaml
CodegenContext:
  buffer: String              # Accumulated output
  indent_level: u32           # Current indentation depth
  indent_str: String          # Indentation characters (default: 4 spaces)
  in_string_concat: bool      # Currently inside a string concatenation chain
  current_impl_type: Option<String>  # Type name if inside an impl block
```

### TypeMapping

Mapping from Lang4LLM type AST nodes to Rust type strings.

```yaml
TypeMapping:
  primitive_map:
    Int: "i64"
    Float: "f64"
    Bool: "bool"
    String: "String"
    Unit: "()"
  array: "[T]" -> "Vec<T>"
  tuple: "(T, U)" -> "(T, U)"
  function_param: "fn(T) -> U" -> "impl Fn(T) -> U"
  function_return: "fn(T) -> U" -> "Box<dyn Fn(T) -> U>"
  generic: "Name<T>" -> "Name<T>"
```

### BuiltinMapping

Translation rules for builtin functions.

```yaml
BuiltinMapping:
  entries:
    - name: "print"
      pattern: "print(expr)"
      translation: 'println!("{}", expr)'
      transform: macro_call

    - name: "range"
      pattern: "range(start, end)"
      translation: "start..end"
      transform: operator_rewrite
      context: for_loop_only  # Standalone: (start..end).collect()

    - name: "push"
      pattern: "push(receiver, item)"
      translation: "receiver.push(item)"
      transform: receiver_rewrite

    - name: "len"
      pattern: "len(receiver)"
      translation: "receiver.len()"
      transform: receiver_rewrite

    - name: "sqrt"
      pattern: "sqrt(expr)"
      translation: "(expr).sqrt()"
      transform: method_call

    - name: "as_float"
      pattern: "as_float(expr)"
      translation: "(expr as f64)"
      transform: cast_expression

    - name: "as_int"
      pattern: "as_int(expr)"
      translation: "(expr as i64)"
      transform: cast_expression
```

### TranspileError

Errors that can occur during transpilation.

```yaml
TranspileError:
  kind: enum
    - UnsupportedFeature    # AST node not yet supported
    - AmbiguousTranslation  # Multiple valid Rust translations
    - InvalidBuiltin        # Unknown builtin function
    - TypeInfoRequired      # Codegen needs type info not in AST
  message: String
  span: Span                # Source location
  suggestion: Option<String> # Possible fix
```

### Relationships

```
Program (AST)
    │
    ▼
TranspileUnit ──uses──▶ CodegenContext
    │                         │
    ├──uses──▶ TypeMapping    │
    ├──uses──▶ BuiltinMapping │
    │                         │
    ▼                         ▼
String (Rust source)    TranspileError[]
```

---

## Phase 4: Standard Library Entities

### BuiltinEntry

Expanded registry entry for a single stdlib builtin function.

```yaml
BuiltinEntry:
  name: String                  # Function name (e.g., "push", "split")
  category: enum                # IO | RANGE | MATH | CONVERSION | STRING | ARRAY
  tier: int                     # Implementation priority (1=first, 5=last)

  signature:
    params:
      - name: String
        type: String            # Lang4LLM type (e.g., "int", "[T]", "string")
        is_receiver: bool       # True if this is a method-style receiver arg
    return_type: String         # Lang4LLM return type

  transpilation:
    transform: enum             # macro_call | receiver_rewrite | method_call
                                # | cast_expression | operator_rewrite | block_expression
    pattern: String             # Lang4LLM call pattern (e.g., "push(arr, item)")
    translation: String         # Rust output pattern (e.g., "{arr}.push({item})")
    context: Option<String>     # Usage restriction (e.g., "for_loop_only")

  typeck:
    param_types: Vec<TypeSig>   # Type checker parameter signatures
    return_type: TypeSig        # Type checker return signature
    overloaded: bool            # True if multiple type signatures exist
    overload_variants:          # Only if overloaded=true
      - params: Vec<TypeSig>
        return_type: TypeSig

  preconditions:
    - constraint: String        # e.g., "exp >= 0"
      enforcement: enum         # COMPILE_TIME | RUNTIME_PANIC | UNCHECKED

  side_effects:
    mutates_receiver: bool      # True for push, pop, sort, reverse
    performs_io: bool            # True for print
    can_panic: bool             # True for pop, first (on empty)

  documentation:
    description: String
    examples:
      - lang4llm: String        # Source code example
        rust_output: String     # Expected transpilation
```

### StdlibRegistry

The complete stdlib function registry used by both transpiler and type checker.

```yaml
StdlibRegistry:
  entries: Vec<BuiltinEntry>    # All registered builtins

  categories:
    - name: String              # Category name
      count: int                # Number of functions
      functions: Vec<String>    # Function names in category

  tiers:                        # Implementation phases
    - tier: int                 # 1-5
      description: String       # What this tier covers
      functions: Vec<String>    # Functions in this tier
      depends_on: Vec<int>      # Tier dependencies

  overloads:                    # Functions with multiple signatures
    - name: String              # e.g., "contains", "len", "abs"
      variants:
        - context: String       # e.g., "string", "array", "int", "float"
          signature: String     # Full type signature for this variant
```

### StdlibRegistry Summary

```
┌──────────────────────────────────────────────────────┐
│                  StdlibRegistry                       │
│  entries: 29 BuiltinEntry                            │
│                                                       │
│  ┌─────────┐  ┌─────────┐  ┌────────────┐           │
│  │   IO    │  │  Range  │  │    Math    │           │
│  │ print   │  │ range   │  │ sqrt, abs  │           │
│  │         │  │         │  │ min, max   │           │
│  │         │  │         │  │ pow, floor │           │
│  │         │  │         │  │ ceil,round │           │
│  └─────────┘  └─────────┘  └────────────┘           │
│                                                       │
│  ┌────────────┐  ┌───────────────────────────────┐   │
│  │ Conversion │  │           String              │   │
│  │ as_float   │  │ contains, split, trim         │   │
│  │ as_int     │  │ replace, starts_with          │   │
│  │ to_string  │  │ ends_with, to_upper           │   │
│  │            │  │ to_lower, chars                │   │
│  └────────────┘  └───────────────────────────────┘   │
│                                                       │
│  ┌───────────────────────────────────────────────┐   │
│  │                    Array                       │   │
│  │ push, pop, len, enumerate, sort, reverse      │   │
│  │ map, filter, is_empty, contains, concat       │   │
│  │ join, first                                    │   │
│  └───────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────┘
```

### Relationships (Phase 4)

```
BuiltinEntry
    │
    ├──registered_in──▶ StdlibRegistry
    │
    ├──transpiled_by──▶ BuiltinMapping (transpiler/builtins.rs)
    │
    ├──type_checked_by──▶ TypeckBuiltins (typeck/builtins.rs)
    │
    └──documented_in──▶ stdlib-functions.md (contracts)
```
