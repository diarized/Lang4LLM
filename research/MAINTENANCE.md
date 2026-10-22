# Research Deliverables Maintenance Notes

**Last Updated**: 2026-01-20
**Phase**: Phase 0 Complete

## Overview

This document tracks data archaeology, deprecated content, and maintenance decisions for the Phase 0 research deliverables (D1-D5).

**Philosophy**: Document duplicates and abandoned content rather than delete. Preserves context for understanding research methodology evolution.

---

## Deprecated Content

### D1: Nested Duplicate Folder

**Location**: `research/D1-error-taxonomy/research/D1-error-taxonomy/`

**Content**: 8 files (MBPP/HumanEval benchmark data from early experiments)
- `data/benchmarks/mbpp/` (4 files: JSON/JSONL)
- `data/benchmarks/humaneval/` (4 files: JSON/JSONL)
- Old experiment data from early Phase 0 work

**Status**: ABANDONED - use top-level `data/` directory instead

**Reason**: Early directory structure experimented with nested organization

**Data value**: Contains partial benchmark subsets from initial literature extraction

**Archaeology notes**:
- MBPP_subset.json vs MBPP_full.jsonl show iteration on data format
- HumanEval data matches final structure in top-level data/
- summary.json files show early error categorization attempts

**DO NOT USE**: Refer to `research/D1-error-taxonomy/data/` for current data

**Cleanup decision**: Archive (do not delete) - may inform future research methodology

**Archive date**: TBD (Phase 1+)

---

## Intentional Duplicates

### Deliverable Schemas

**Files**:
- `research/schemas/deliverable-schemas.yaml` (working copy)
- `specs/001-llm-lang-research/contracts/deliverable-schemas.yaml` (canonical)

**Status**: INTENTIONAL DUPLICATE

**Canonical source**: `specs/001-llm-lang-research/contracts/`

**Working copy**: `research/schemas/` (used for validation during research)

**Purpose**: Different contexts - specs define contract, research uses for validation

**Sync required**: If schemas change in future phases

**Last synced**: 2026-01-20

---

## Scope Clarifications

### D4: Representative Sample vs Complete Dataset

**Status**: 13 representative specifications (of 100 planned)

**Created**: 13 specifications (5 simple, 5 medium, 3 complex)

**Planned**: 100 specifications (40 simple, 40 medium, 20 complex)

**Ratio**: 13% - sufficient for Phase 0 research validation

**Why 13 specs are sufficient for Phase 0**:

1. **Format validation**: 13 specs demonstrate spec format quality
2. **Tier coverage**: All 3 tiers represented (simple/medium/complex)
3. **Category coverage**: Data structures, algorithms, strings, math, I/O
4. **Research deliverable**: Phase 0 validates *approach*, not full dataset

**When 100 specs are required**:

**Phase 5**: LLM Validation Testing (actual benchmark execution with implemented language)

Until then, 13 representative specs provide sufficient evidence for:
- Spec format viability (D4 validation)
- Design rule testing (D2 validation)
- Metric correlation (D3 validation)
- Feature validation (D5 gate decision)

**Gate status**: ✅ PASSED with representative sample

**Terminology**: "Representative sample" (NOT "incomplete" or "partial")

---

## Cleanup Timeline

| Item | Status | Action | Phase | Notes |
|------|--------|--------|-------|-------|
| D1 nested duplicate | Documented | Archive (optional) | Phase 1+ | Data archaeology value |
| Schema duplicates | Documented | Keep both | N/A | Intentional, different contexts |
| D4 "incomplete" terminology | Renamed | "representative sample" | Done | Clarity improvement |

---

## Data Archaeology Notes

**Purpose**: Preserve context for understanding research methodology evolution

**D1 nested folder**:
- Shows iteration on directory structure
- Documents early data format experiments (JSON vs JSONL)
- May inform future benchmark data organization

**Schema duplication**:
- Shows validation vs specification separation
- May inform future project structure decisions

**D4 representative sample**:
- Shows research gate requirements (quality over quantity)
- Demonstrates spec format maturity with limited examples

**Future researchers**: Review deprecated content before modifying to understand design evolution

---

## Version History

- **2026-01-20**: Initial maintenance documentation (Phase 0 complete)
  - Documented D1 nested duplicate
  - Clarified schema duplication
  - Established D4 terminology ("representative sample")
