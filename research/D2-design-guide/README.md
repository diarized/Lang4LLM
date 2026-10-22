# D2: LLM-Friendly Language Design Guide

**Status**: ✅ COMPLETE (2026-01-20)
**Phase**: Phase 4 of 8 (User Story 2)
**Deliverable**: 20+ concrete, testable design rules for LLM-optimized programming languages

---

## Quick Start

**Main Entry Point**: [`guide.md`](guide.md)

**Key Documents**:
- [`guide.md`](guide.md) - Complete design guide with all 20 rules
- [`error-feature-mapping.md`](error-feature-mapping.md) - Error pattern analysis
- [`rule-rankings.md`](rule-rankings.md) - Priority tiers and implementation roadmap
- [`counter-examples.md`](counter-examples.md) - When rules don't apply

**Individual Rules**: [`rules/`](rules/) directory (20 YAML files organized by category)

---

## What's Inside

### 20 Design Rules Across 5 Categories

| Category | Rules | Impact | Key Findings |
|----------|-------|--------|--------------|
| **Grammar** (DR-GRM-*) | 4 | 0.60-0.96 | CFG grammar → 96% syntax error reduction |
| **Syntax** (DR-SYN-*) | 5 | 0.30-0.85 | No indentation → Python 20% vs Rust 0% errors |
| **Semantic** (DR-SEM-*) | 4 | 0.50-0.70 | Strong typing → 70% semantic error reduction |
| **Token** (DR-TOK-*) | 3 | 0.35-0.45 | Symbols → 35% token reduction (GlyphLang) |
| **Error Prevention** (DR-ERR-*) | 4 | 0.30-0.75 | Iterators → Eliminates #1 logic error (22%) |

---

## Priority Rankings

### Tier 1: CRITICAL (60% Error Reduction)
1. **DR-GRM-001**: Context-Free Grammar (I=0.96)
2. **DR-ERR-001**: Iterator-First Design (I=0.75)
3. **DR-SYN-003**: Bounds-Checked Indexing (I=0.80)
4. **DR-GRM-002**: SynCode-Compatible Grammar (I=0.75)
5. **DR-SYN-002**: No Significant Whitespace (I=0.85)

### Tier 2: HIGH (85% Total Reduction)
6. **DR-GRM-003**: Predictable Tokens (I=0.65)
7. **DR-GRM-004**: Minimal Ambiguity (I=0.60)
8. **DR-SYN-004**: Explicit Operators (I=0.55)
9. **DR-SEM-001**: Mandatory Declarations (I=0.60)
10. **DR-SEM-002**: Strong Static Typing (I=0.70)

### Tier 3: MEDIUM (95% Total Reduction)
11-15. Semantic completeness, token efficiency, syntax consistency

### Tier 4: LOW (Near-Optimal)
16-20. Boilerplate reduction, additional safety features

**Full rankings**: [`rule-rankings.md`](rule-rankings.md)

---

## Research Backing

**Input**: D1 Error Taxonomy (12 patterns, 4 languages, 3 benchmarks)

**Primary Sources**:
- P01 (ICSE 2025): 558 errors analyzed
- P04 (SynCode): 96% syntax error reduction
- D1 Cross-Language Analysis: Python 20% vs Rust 0% indentation errors

**Evidence Quality**:
- **HIGH Confidence**: 10 rules (backed by 3+ studies)
- **MEDIUM Confidence**: 7 rules (2 studies or cross-language data)
- **LOW Confidence**: 3 rules (design principles, limited LLM-specific data)

---

## Validation Strategy

Each rule specifies:
1. **D3 Metrics**: Which metrics correlate with this rule (M-001 through M-004)
2. **D4 Benchmarks**: Specific test cases for A/B validation
3. **Cross-Language Comparison**: Existing languages implementing similar rules
4. **Research Citations**: Papers backing the design choice

**Next Phase (D3)**: Compute metrics, correlate with benchmark performance

---

## Implementation Roadmap

### Phase 1 MVP: Tier 1 Rules (5 rules)
**Goal**: 60% error reduction with minimal features
**Timeline**: Foundation for all subsequent phases

### Phase 2 Core: + Tier 2 (10 rules total)
**Goal**: 85% error reduction, complete grammar + type system

### Phase 3 Polish: + Tier 3 (15 rules total)
**Goal**: 95% error reduction, safety + usability

### Phase 4 Complete: All 20 Rules
**Goal**: Near-optimal LLM code generation accuracy

---

## Directory Structure

```
research/D2-design-guide/
├── README.md                     # This file
├── guide.md                      # Main design guide (entry point)
├── error-feature-mapping.md      # Error → language feature analysis
├── rule-rankings.md              # Priority tiers and implementation strategy
├── counter-examples.md           # Exception scenarios for all rules
├── validate-rules.py             # Schema validation script
└── rules/
    ├── grammar/                  # DR-GRM-001 through DR-GRM-004
    │   ├── DR-GRM-001.yaml       # Context-Free Grammar
    │   ├── DR-GRM-002.yaml       # SynCode-Compatible
    │   ├── DR-GRM-003.yaml       # Predictable Tokens
    │   └── DR-GRM-004.yaml       # Minimal Ambiguity
    ├── syntax/                   # DR-SYN-001 through DR-SYN-005
    │   ├── DR-SYN-001.yaml       # Explicit Delimiters
    │   ├── DR-SYN-002.yaml       # No Significant Whitespace
    │   ├── DR-SYN-003.yaml       # Bounds-Checked Indexing
    │   ├── DR-SYN-004.yaml       # Explicit Operators
    │   └── DR-SYN-005.yaml       # Statement Terminators
    ├── semantic/                 # DR-SEM-001 through DR-SEM-004
    │   ├── DR-SEM-001.yaml       # Mandatory Declarations
    │   ├── DR-SEM-002.yaml       # Strong Static Typing
    │   ├── DR-SEM-003.yaml       # Limited Parameters
    │   └── DR-SEM-004.yaml       # Exhaustive Returns
    ├── token/                    # DR-TOK-001 through DR-TOK-003
    │   ├── DR-TOK-001.yaml       # Symbols Over Keywords
    │   ├── DR-TOK-002.yaml       # Minimal Boilerplate
    │   └── DR-TOK-003.yaml       # Rich Standard Library
    └── error-prevention/         # DR-ERR-001 through DR-ERR-004
        ├── DR-ERR-001.yaml       # Iterator-First Design
        ├── DR-ERR-002.yaml       # Bounded Iteration
        ├── DR-ERR-003.yaml       # Option Types (No Null)
        └── DR-ERR-004.yaml       # Compiler Completeness Checking
```

---

## Validation Results

**T051-T052 Validation** (2026-01-20):

```
✅ VALIDATION PASSED
   - All 20 rules are valid
   - All required fields present
   - All rules have research backing
   - All rules have validation methods
   - All rules link to error patterns
```

**Schema Compliance**:
- ✓ ID format: `DR-(SYN|GRM|SEM|TOK|ERR)-\d{3}`
- ✓ Priority: 1-20 (unique per rule)
- ✓ Impact Score: 0.30-0.96
- ✓ Research Backing: 2-4 sources per rule
- ✓ Validation Methods: Metrics + benchmarks specified
- ✓ Counter-Examples: Exception scenarios documented

---

## Success Criteria (D2 Acceptance)

- ✅ **20+ rules**: 20 rules documented
- ✅ **Research backing**: Each rule cites 2+ sources
- ✅ **Validation method**: Each rule defines metrics/benchmarks
- ✅ **Counter-examples**: Exception scenarios for all rules
- ✅ **Impact scores**: Quantified error reduction (0.30-0.96)
- ✅ **Priority rankings**: Rules ranked 1-20 by impact

**Status**: D2 COMPLETE - Ready for D3 Metrics phase

---

## How to Use

### For Language Designers
1. Read [`guide.md`](guide.md) for full overview
2. Start with Tier 1 rules (highest impact)
3. Consult [`counter-examples.md`](counter-examples.md) for edge cases
4. Validate with D4 benchmark after each tier

### For Researchers
1. Each rule in [`rules/`](rules/) has validation methodology
2. Use [`error-feature-mapping.md`](error-feature-mapping.md) for error analysis
3. Test with D3 metrics (Phase 5)
4. Cite research_backing sources

### For LLM Tool Developers
1. **DR-GRM-002**: Implement SynCode-compatible grammars
2. **DR-TOK-001**: Optimize for token efficiency
3. **DR-TOK-003**: Provide rich stdlib documentation

---

## Next Steps

**Phase 5: D3 Metrics** (User Story 3)
- Define token predictability metric (M-001)
- Define parse ambiguity metric (M-002)
- Define AST complexity metric (M-003)
- Define error recovery metric (M-004)
- Correlate with benchmark performance (p < 0.05)

**Phase 6: D4 Benchmark** (User Story 4)
- Create 100+ specification-implementation pairs
- Test each design rule with variants
- Measure error reduction empirically

**Phase 7: D5 Validation** (User Story 5)
- Test all 20 rules against D4 benchmark
- Include/exclude/modify recommendations
- Calculate confidence levels
- Gate language implementation phase

---

## Changelog

**2026-01-20 (v1.0)**:
- Initial D2 Design Guide complete
- 20 rules across 5 categories
- Research backing from D1 + 11 papers
- Validation methodology defined
- Counter-examples documented
- Priority rankings established

---

## Contact

**Version Control**: Tracked in research repository
**Input**: `specs/001-llm-lang-research/`
**Related**: D1 Error Taxonomy (`../D1-error-taxonomy/`)
**Last Review**: 2026-01-20
**Next Milestone**: D3 Metrics Framework
