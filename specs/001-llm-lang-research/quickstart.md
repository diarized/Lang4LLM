# Quickstart: LLM-Friendly Language Design Research

**Branch**: `001-llm-lang-research` | **Date**: 2026-01-20

## Overview

This research project produces 5 deliverables that must be completed before any compiler implementation begins:

| Deliverable | Output | Success Criteria |
|-------------|--------|------------------|
| D1: Error Taxonomy | `research/D1-error-taxonomy/` | 50+ patterns, 3+ benchmarks |
| D2: Design Guide | `research/D2-design-guide/` | 20+ rules with research backing |
| D3: Metrics | `research/D3-metrics/` | Correlation p < 0.05 |
| D4: Benchmark Suite | `research/D4-benchmark/` | 100+ specifications |
| D5: Validation Report | `research/D5-validation/` | 100% feature coverage |

## Prerequisites

### Required Access

- [ ] LLM API keys: GPT-4, Claude, DeepSeek
- [ ] Benchmark datasets: HumanEval, MBPP access
- [ ] Research papers: arXiv access or institutional subscription

### Recommended Tools

- Python 3.11+ (for running experiments)
- JSON/YAML validator (for schema validation)
- Statistical analysis tool (for correlation tests)

## Getting Started

### 1. Set Up Directory Structure

```bash
mkdir -p research/{D1-error-taxonomy/patterns/{syntax,semantic,logic},D2-design-guide/rules,D3-metrics/data,D4-benchmark/{specs/{simple,medium,complex},reference,tests},D5-validation/features,literature/notes}
```

### 2. Start Literature Review

Begin with the 11 primary papers listed in `specs/001-llm-lang-research/research.md`:

```bash
# Create paper notes template
for i in $(seq -w 1 11); do
  echo "# Paper P$i Notes" > research/literature/notes/P$i.md
  echo "" >> research/literature/notes/P$i.md
  echo "## Key Findings" >> research/literature/notes/P$i.md
  echo "## Error Patterns Identified" >> research/literature/notes/P$i.md
  echo "## Design Implications" >> research/literature/notes/P$i.md
done
```

### 3. Begin Error Analysis

Once literature review identifies initial taxonomy:

1. Select 50 problems from HumanEval
2. Run each problem through GPT-4, Claude, DeepSeek (3 attempts each)
3. Collect all non-passing outputs
4. Categorize using taxonomy from literature

### 4. Validate Deliverables

Each deliverable has a schema in `specs/001-llm-lang-research/contracts/deliverable-schemas.yaml`.

Validate JSON outputs:

```bash
# Example: validate error pattern
python -c "
import yaml
import jsonschema
import json

with open('specs/001-llm-lang-research/contracts/deliverable-schemas.yaml') as f:
    schemas = yaml.safe_load(f)

with open('research/D1-error-taxonomy/patterns/syntax/EP-SYN-001.json') as f:
    pattern = json.load(f)

jsonschema.validate(pattern, schemas['components']['schemas']['ErrorPattern'])
print('Valid!')
"
```

## Phase Sequence

```
Phase 0.1: Literature    ──┐
                          ├──▶ Phase 0.2: Error Analysis ──┬──▶ Phase 0.3: Design Rules
                          │                                │
                          │                                └──▶ Phase 0.4: Metrics
                          │
                          └───────────────────────────────────▶ Phase 0.5: Benchmark
                                                                      │
                                                                      ▼
                                                               Phase 0.6: Validation
                                                                      │
                                                                      ▼
                                                               ✓ GATE PASSED
                                                               (Begin implementation)
```

## Key Files Reference

| Purpose | Location |
|---------|----------|
| Feature specification | `specs/001-llm-lang-research/spec.md` |
| Implementation plan | `specs/001-llm-lang-research/plan.md` |
| Research decisions | `specs/001-llm-lang-research/research.md` |
| Data model | `specs/001-llm-lang-research/data-model.md` |
| Schemas | `specs/001-llm-lang-research/contracts/deliverable-schemas.yaml` |

## Common Tasks

### Add New Error Pattern

1. Create file: `research/D1-error-taxonomy/patterns/{category}/EP-{CAT}-{NNN}.json`
2. Follow schema: `ErrorPattern` in `contracts/deliverable-schemas.yaml`
3. Link to related patterns if applicable
4. Update frequency data in parent category

### Add New Design Rule

1. Create file: `research/D2-design-guide/rules/DR-{CAT}-{NNN}.json`
2. Follow schema: `DesignRule` in `contracts/deliverable-schemas.yaml`
3. Link to error patterns it addresses
4. Define validation method (which benchmarks to use)

### Add New Benchmark Specification

1. Create spec file: `research/D4-benchmark/specs/{tier}/BS-{T}-{NNN}.yaml`
2. Create reference implementation: `research/D4-benchmark/reference/BS-{T}-{NNN}.py`
3. Create tests: `research/D4-benchmark/tests/BS-{T}-{NNN}_test.py`
4. Verify reference passes all tests

## Success Checklist

Before declaring Phase 0 complete:

- [ ] `research/D1-error-taxonomy/taxonomy.md` exists with 50+ patterns
- [ ] `research/D2-design-guide/guide.md` exists with 20+ rules
- [ ] `research/D3-metrics/correlations.md` shows p < 0.05
- [ ] `research/D4-benchmark/` contains 100+ validated specs
- [ ] `research/D5-validation/report.md` covers all features
- [ ] All JSON/YAML files validate against schemas
- [ ] Cross-references between deliverables are valid

## Next Steps After Research

Once all success criteria met:

1. Run `/speckit:tasks` to generate implementation task breakdown
2. Create new branch for language implementation (Phase 1+)
3. Reference research deliverables in implementation decisions

---

## Phase 3: Transpiler to Rust

### Prerequisites

- Phase 2 complete: parser + type checker validated (12/12 examples, 20/20 error tests)
- Rust toolchain: `rustc` + `cargo` (edition 2021)
- Working `lang4llm-rs/` workspace with `ast`, `parser`, `typeck`, `cli` crates

### Getting Started

```bash
# Verify Phase 2 tests still pass
cd lang4llm-rs && cargo test

# Create transpiler crate
cargo new --lib crates/transpiler --name lang4llm-transpiler

# Add to workspace members in Cargo.toml
# Add lang4llm-ast dependency
```

### Testing the Transpiler

```bash
# Transpile a single file
cargo run --bin lang4llm compile ../language-spec/examples/01_hello_world.lang4llm

# Transpile + verify with rustc
cargo run --bin lang4llm compile --check ../language-spec/examples/01_hello_world.lang4llm

# Transpile + run
cargo run --bin lang4llm compile --run ../language-spec/examples/01_hello_world.lang4llm

# Run all integration tests
cargo test --test transpile_examples
```

### Key Files Reference (Phase 3)

| Purpose | Location |
|---------|----------|
| Transpiler crate | `lang4llm-rs/crates/transpiler/` |
| API contract | `specs/001-llm-lang-research/contracts/transpiler-api.md` |
| Research decisions | `specs/001-llm-lang-research/research.md` (Decisions 7-14) |
| Example programs | `language-spec/examples/*.lang4llm` |
| Integration tests | `lang4llm-rs/tests/transpile_examples.rs` |

### Success Criteria (Phase 3)

- [ ] All 12 example programs transpile without error
- [ ] All generated Rust files compile with `rustc --edition 2021`
- [ ] Runtime output matches expected behavior for examples with `print()`
- [ ] CLI `compile` subcommand works with `--output`, `--check`, `--run` flags
- [ ] Transpiler crate has unit tests for each codegen module
- [ ] No `unsafe` code in transpiler

---

## Phase 4: Standard Library Foundation

### Prerequisites

- Phase 3 complete: transpiler validated (12/12 transpile, 12/12 rustc compile, 5/5 run)
- Generic trait bounds fix applied (commit `ca89069`)
- Rust toolchain: `rustc` + `cargo` (edition 2021)

### Getting Started

```bash
# Verify Phase 3 tests still pass
cd lang4llm-rs && cargo test --workspace

# Key files to modify:
# - crates/transpiler/src/builtins.rs  (transpilation rules)
# - crates/typeck/src/builtins.rs      (type signatures)
# - language-spec/examples/            (new example programs)
```

### Adding a New Builtin Function

1. **Type checker** (`crates/typeck/src/builtins.rs`):
   - Add type signature to `register_builtins()`
   - Follow existing pattern: `register("name", vec![param_types], return_type)`

2. **Transpiler** (`crates/transpiler/src/builtins.rs`):
   - Add translation rule to `emit_builtin_call()`
   - Match on function name, emit Rust equivalent
   - Use `emit_expr()` for each argument

3. **Contract** (`specs/001-llm-lang-research/contracts/stdlib-functions.md`):
   - Verify your implementation matches the contract

4. **Test** — write an example program using the new function:
   ```bash
   # Create example file
   vim language-spec/examples/13_new_feature.lang4llm

   # Add to transpile test suite
   # In crates/transpiler/tests/transpile_examples.rs

   # Verify transpile + compile + run
   cargo test -p lang4llm-transpiler
   ```

### Testing the Stdlib

```bash
# Run all transpiler tests (includes stdlib builtin tests)
cargo test -p lang4llm-transpiler

# Transpile + compile a specific example
cargo run --bin lang4llm compile --check ../language-spec/examples/13_stdlib_math.lang4llm

# Transpile + run
cargo run --bin lang4llm compile --run ../language-spec/examples/13_stdlib_math.lang4llm

# Run the full workspace test suite
cargo test --workspace
```

### Key Files Reference (Phase 4)

| Purpose | Location |
|---------|----------|
| Transpiler builtins | `lang4llm-rs/crates/transpiler/src/builtins.rs` |
| Type checker builtins | `lang4llm-rs/crates/typeck/src/builtins.rs` |
| Stdlib contracts | `specs/001-llm-lang-research/contracts/stdlib-functions.md` |
| Research decisions | `specs/001-llm-lang-research/research.md` (Decisions R15-R20) |
| Example programs | `language-spec/examples/*.lang4llm` |

### Implementation Tiers

| Tier | Functions | Priority |
|------|-----------|----------|
| 1 | abs, min, max, pow, to_string | Existing examples need these |
| 2 | floor, ceil, round, as_int, split, trim, replace | Math + string basics |
| 3 | starts_with, ends_with, to_upper, to_lower, chars | String utilities |
| 4 | sort, reverse, map, filter, is_empty, concat, join, first | Array operations |
| 5 | enumerate, pop | Iterator + mutation |

### Success Criteria (Phase 4)

- [ ] All 29 stdlib functions registered in type checker
- [ ] All 29 stdlib functions transpile correctly
- [ ] 3 new example programs (13_stdlib_math, 14_stdlib_strings, 15_stdlib_arrays)
- [ ] All new examples transpile + compile + run
- [ ] All existing 12 examples still pass
- [ ] Overloaded functions (contains, len, abs) resolve correctly
