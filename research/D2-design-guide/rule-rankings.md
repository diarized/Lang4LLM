# Design Rule Priority Rankings (T049)

**Purpose**: Rank all 20 design rules by impact on LLM code generation accuracy
**Date**: 2026-01-20
**Methodology**: Combined error frequency, mitigation impact, and research confidence

---

## Priority Scale

**Priority 1-5**: CRITICAL (Must-have, >70% impact)
**Priority 6-10**: HIGH (Strong impact, 50-70%)
**Priority 11-15**: MEDIUM (Moderate impact, 30-50%)
**Priority 16-20**: LOW (Nice-to-have, <30%)

---

## Ranked Rules (Priority 1-20)

| Rank | Rule ID | Name | Impact | Rationale |
|------|---------|------|--------|-----------|
| 1 | DR-GRM-001 | Context-Free Grammar Foundation | 0.96 | SynCode: 96% syntax error reduction |
| 2 | DR-ERR-001 | Iterator-First Design | 0.75 | EP-LOG-001 (22%) is #1 error |
| 3 | DR-SYN-003 | Bounds-Checked Indexing | 0.80 | Prevents 15% syntax + 22% boundary errors |
| 4 | DR-GRM-002 | SynCode-Compatible Grammar | 0.75 | Enables constrained decoding tools |
| 5 | DR-SYN-002 | No Significant Whitespace | 0.85 | Python: 20% errors vs Rust: 0% |
| 6 | DR-GRM-003 | Predictable Token Sequences | 0.65 | Aids LLM next-token prediction |
| 7 | DR-GRM-004 | Minimal Grammar Ambiguity | 0.60 | Unambiguous parsing |
| 8 | DR-SYN-004 | Explicit Operator Distinction | 0.55 | Prevents 10% operator confusion |
| 9 | DR-SEM-001 | Mandatory Variable Declarations | 0.60 | EP-SEM-001 (18%) prevention |
| 10 | DR-SEM-002 | Strong Static Typing | 0.70 | 70% semantic error reduction |
| 11 | DR-SEM-004 | Exhaustive Return Checking | 0.60 | Eliminates EP-SEM-004 (12%) |
| 12 | DR-SEM-003 | Limited Positional Parameters | 0.50 | EP-SEM-003 (14%) mitigation |
| 13 | DR-TOK-003 | Rich Standard Library | 0.45 | Reduces EP-LOG-002 (20%) via stdlib |
| 14 | DR-TOK-001 | Symbols Over Keywords | 0.40 | 35% token reduction (GlyphLang) |
| 15 | DR-SYN-005 | Consistent Statement Terminators | 0.30 | Minor syntax clarity |
| 16 | DR-TOK-002 | Minimal Boilerplate | 0.35 | Reduces token overhead |
| 17 | DR-ERR-002 | Prefer Bounded Iteration | 0.30 | EP-LOG-004 (15%) partial mitigation |
| 18 | DR-ERR-003 | Explicit Null Handling | 0.50 | Prevents null errors (not measured in D1) |
| 19 | DR-ERR-004 | Compiler-Enforced Completeness | 0.55 | Prevents incomplete patterns |
| 20 | DR-SYN-001 | Explicit Delimiters (Limited Nesting) | 0.70 | EP-SYN-001 (12%) but lower rank due to trade-offs |

---

## Priority Tiers

### Tier 1: CRITICAL (Must Implement First)

**Rules**: DR-GRM-001, DR-ERR-001, DR-SYN-003, DR-GRM-002, DR-SYN-002

**Rationale**:
- **DR-GRM-001**: Foundation for all grammar-based benefits (96% error reduction)
- **DR-ERR-001**: Addresses #1 logic error (boundary conditions, 22%)
- **DR-SYN-003**: Prevents highest-frequency syntax error (15%) + boundary logic errors
- **DR-GRM-002**: Enables LLM tooling (SynCode integration)
- **DR-SYN-002**: Eliminates entire error class (indentation, 20% in Python)

**Combined Impact**: Implementing these 5 rules alone would eliminate ~60% of all errors

---

### Tier 2: HIGH PRIORITY (Core Language Features)

**Rules**: DR-GRM-003, DR-GRM-004, DR-SYN-004, DR-SEM-001, DR-SEM-002

**Rationale**:
- Grammar rules (DR-GRM-003, DR-GRM-004) complete CFG foundation
- Type system (DR-SEM-002) prevents 70% of semantic errors
- Variable declarations (DR-SEM-001) prevent 18% undefined errors
- Operator clarity (DR-SYN-004) eliminates confusion errors

**Combined Impact**: Additional ~25% error reduction

---

### Tier 3: MEDIUM PRIORITY (Safety & Quality)

**Rules**: DR-SEM-004, DR-SEM-003, DR-TOK-003, DR-TOK-001, DR-SYN-005

**Rationale**:
- Completeness checking (DR-SEM-004) catches incomplete logic
- Parameter limits (DR-SEM-003) reduce argument errors
- Standard library (DR-TOK-003) guides algorithm choice
- Token efficiency (DR-TOK-001) improves context usage
- Statement terminators (DR-SYN-005) add consistency

**Combined Impact**: Additional ~10% error reduction + usability improvements

---

### Tier 4: LOW PRIORITY (Polish & Optimization)

**Rules**: DR-TOK-002, DR-ERR-002, DR-ERR-003, DR-ERR-004, DR-SYN-001

**Rationale**:
- Boilerplate reduction (DR-TOK-002) is quality-of-life
- Bounded iteration (DR-ERR-002) only partially mitigates loops
- Null handling (DR-ERR-003) prevents errors not measured in D1
- Completeness checking (DR-ERR-004) overlaps with DR-SEM-004
- Nesting limits (DR-SYN-001) have significant trade-offs

**Combined Impact**: Additional ~5% error reduction + language polish

---

## Implementation Strategy

### Phase 1 MVP: Tier 1 Only (5 rules)

**Goal**: Maximum error reduction with minimal features

**Rules to Implement**:
1. DR-GRM-001: CFG grammar specification
2. DR-SYN-002: Brace-based scoping (no indentation)
3. DR-SYN-003: Bounds-checked indexing
4. DR-GRM-002: SynCode integration
5. DR-ERR-001: Iterator-first loops

**Expected Outcome**: 60% error reduction vs Python baseline

**Validation**: Run D4 benchmark, measure error rates

---

### Phase 2 Core: + Tier 2 (10 rules total)

**Add**:
6. DR-GRM-003: Predictable tokens (LL(1) grammar)
7. DR-GRM-004: Unambiguous grammar
8. DR-SYN-004: Distinct operators (`:=` vs `=`)
9. DR-SEM-001: Mandatory declarations
10. DR-SEM-002: Static typing

**Expected Outcome**: 85% error reduction vs Python baseline

---

### Phase 3 Polish: + Tier 3 (15 rules total)

**Add**:
11. DR-SEM-004: Return checking
12. DR-SEM-003: Parameter limits
13. DR-TOK-003: Rich stdlib
14. DR-TOK-001: Symbolic syntax
15. DR-SYN-005: Statement terminators

**Expected Outcome**: 95% error reduction vs Python baseline

---

### Phase 4 Complete: + Tier 4 (20 rules total)

**Add**:
16. DR-TOK-002: Minimal boilerplate
17. DR-ERR-002: Bounded iteration hints
18. DR-ERR-003: Option types (no null)
19. DR-ERR-004: Exhaustive matching
20. DR-SYN-001: Nesting limits

**Expected Outcome**: Near-optimal LLM code generation accuracy

---

## Research Confidence Levels

### HIGH Confidence (Proven in Literature)

- DR-GRM-001 (SynCode: 96% reduction)
- DR-SYN-002 (D1: Python 20% vs Rust 0%)
- DR-SYN-003 (P01, P02: #1 error pattern)
- DR-SEM-002 (P06: Type constraints improve correctness)
- DR-ERR-001 (P01: Boundary conditions #1)

**Action**: Implement immediately, high certainty of impact

---

### MEDIUM Confidence (Supported by Cross-Language Data)

- DR-SEM-001 (D1: Python 25% vs Rust 5%)
- DR-SEM-004 (Rust design: 0% missing returns)
- DR-GRM-002 (SynCode architecture extensibility)
- DR-ERR-003 (Rust/Kotlin null safety)

**Action**: Implement with D4 benchmark validation

---

### LOW Confidence (Design Principles, Limited Data)

- DR-TOK-001 (GlyphLang: 35% tokens, but limited LLM testing)
- DR-SEM-003 (Clean Code principles, not LLM-specific)
- DR-TOK-002 (Usability, indirect error impact)
- DR-ERR-002 (Detection only, not prevention)
- DR-SYN-005 (Minor impact)

**Action**: Implement in later phases, validate experimentally in D5

---

## Priority Adjustments Based on Goals

### Goal: Maximum Error Reduction
**Prioritize**: Tier 1 + Tier 2 (rules 1-10)

### Goal: LLM-First Experience
**Prioritize**: DR-GRM-001, DR-GRM-002, DR-TOK-001, DR-TOK-003 (grammar + tokens)

### Goal: Safety-Critical Applications
**Prioritize**: DR-SEM-002, DR-SEM-004, DR-ERR-003, DR-ERR-004 (type safety + exhaustiveness)

### Goal: Beginner-Friendly
**Prioritize**: DR-SYN-002, DR-SEM-001, DR-TOK-002, DR-TOK-003 (clear syntax + stdlib)

---

## Changelog

**2026-01-20**: Initial ranking based on D1 error taxonomy and impact scores

---

**Status**: T049 COMPLETE
**Next**: T050 - Compile main design guide
