# Phase 1: Language Specification Plan

**Status**: In Progress
**Started**: 2026-01-26
**Dependencies**: Phase 0 (D1-D5 complete)
**Type**: Design/Documentation (no runtime code)

---

## Objective

Create complete formal specification for Lang4LLM programming language based on validated design rules from Phase 0 research.

**Success Criterion**: Unambiguous, implementable specification ready for Phase 2 (Parser & Type Checker)

---

## Deliverables

### 1. Formal Grammar Specification
**File**: `grammar/grammar.bnf`
**Input**: DR-GRM-001 through DR-GRM-004 (Context-Free Grammar rules)
**Output**: Complete BNF/EBNF grammar specification

**Requirements**:
- Context-free grammar (DR-GRM-001: I=0.96)
- Unambiguous parsing (DR-GRM-002: single valid parse)
- Minimal nesting depth (DR-GRM-003: ≤3 levels)
- Fixed precedence (DR-GRM-004: no context-dependent parsing)

**Components**:
- Lexical rules (tokens, keywords, operators)
- Syntax rules (expressions, statements, declarations)
- Precedence and associativity table

---

### 2. Type System Specification
**File**: `type-system/types.md`
**Input**: DR-SEM-001 through DR-SEM-004 (Semantic rules)
**Output**: Complete type system specification

**Requirements**:
- Explicit type declarations (DR-SEM-001: no implicit coercion)
- Static type checking (DR-SEM-002: compile-time verification)
- Lexical scoping (DR-SEM-003: no variable shadowing)
- Mandatory initialization (DR-SEM-004: no undefined variables)

**Components**:
- Primitive types (int, float, bool, string, unit)
- Composite types (arrays, tuples, structs, enums)
- Type inference rules
- Type checking algorithm
- Generics/polymorphism (if applicable)

---

### 3. Syntax Specification
**File**: `syntax/syntax.md`
**Input**: DR-SYN-001 through DR-SYN-005 (Syntax rules)
**Output**: Concrete syntax specification with examples

**Requirements**:
- Keywords over symbols (DR-SYN-001: reduce ambiguity)
- Explicit delimiters (DR-SYN-002: no whitespace-sensitive syntax)
- Bounds-checked indexing (DR-SYN-003: prevent off-by-one errors)
- Linear control flow (DR-SYN-004: no goto, minimal nesting)
- Consistent naming (DR-SYN-005: no case-sensitive identifiers)

**Components**:
- Variable declarations
- Function definitions
- Control flow (if, match, loops)
- Module system
- Import/export syntax

---

### 4. Semantics Specification
**File**: `semantics/semantics.md`
**Input**: DR-TOK-* (Token efficiency), DR-ERR-* (Error prevention)
**Output**: Operational semantics and runtime behavior

**Requirements**:
- Iterator-first design (DR-ERR-001: eliminates 22% boundary errors)
- First-class error handling (DR-ERR-002: explicit error types)
- Exhaustive pattern matching (DR-ERR-003: no missing cases)
- Immutable by default (DR-ERR-004: explicit mutability)

**Components**:
- Evaluation order
- Variable binding semantics
- Function call semantics
- Error propagation model
- Iterator protocol
- Pattern matching semantics

---

### 5. Memory Model Specification
**File**: `memory-model/memory.md`
**Input**: Phase 0 findings (Rust ownership insights)
**Output**: Memory management strategy

**Requirements**:
- Safety guarantees (no undefined behavior)
- Predictable performance (deterministic allocation/deallocation)
- LLM-friendly (simple mental model)

**Components**:
- Ownership model (if applicable)
- Reference/borrowing rules (if applicable)
- Garbage collection strategy (if applicable)
- Stack vs heap allocation rules
- Lifetime annotations (if applicable)

---

## Input: Phase 0 Design Rules

### Grammar Rules (4 rules)
| ID | Rule | Impact | Phase 1 Component |
|----|------|--------|-------------------|
| DR-GRM-001 | Context-Free Grammar | I=0.96 | grammar.bnf (entire file) |
| DR-GRM-002 | Unambiguous Parsing | I=0.88 | grammar.bnf (disambiguation notes) |
| DR-GRM-003 | Minimal Nesting | I=0.82 | syntax.md (nesting rules) |
| DR-GRM-004 | Fixed Precedence | I=0.85 | grammar.bnf (precedence table) |

### Syntax Rules (5 rules)
| ID | Rule | Impact | Phase 1 Component |
|----|------|--------|-------------------|
| DR-SYN-001 | Keywords over Symbols | I=0.72 | syntax.md (keyword list) |
| DR-SYN-002 | Explicit Delimiters | I=0.85 | grammar.bnf (delimiter rules) |
| DR-SYN-003 | Bounds-Checked Indexing | I=0.78 | semantics.md (array access) |
| DR-SYN-004 | Linear Control Flow | I=0.65 | syntax.md (control structures) |
| DR-SYN-005 | Consistent Naming | I=0.68 | syntax.md (identifier rules) |

### Semantic Rules (4 rules)
| ID | Rule | Impact | Phase 1 Component |
|----|------|--------|-------------------|
| DR-SEM-001 | Explicit Types | I=0.76 | types.md (type declarations) |
| DR-SEM-002 | Static Type Checking | I=0.70 | types.md (type checker spec) |
| DR-SEM-003 | Lexical Scoping | I=0.58 | semantics.md (scope rules) |
| DR-SEM-004 | Mandatory Initialization | I=0.62 | semantics.md (initialization) |

### Token Efficiency Rules (3 rules)
| ID | Rule | Impact | Phase 1 Component |
|----|------|--------|-------------------|
| DR-TOK-001 | Minimal Token Count | I=0.62 | syntax.md (concise syntax) |
| DR-TOK-002 | Predictable Tokens | I=0.58 | grammar.bnf (regular tokens) |
| DR-TOK-003 | Compressed Keywords | I=0.30 | syntax.md (keyword selection) |

### Error Prevention Rules (4 rules)
| ID | Rule | Impact | Phase 1 Component |
|----|------|--------|-------------------|
| DR-ERR-001 | Iterator-First Design | I=0.88 | semantics.md (iterators) |
| DR-ERR-002 | First-Class Errors | I=0.72 | semantics.md (error handling) |
| DR-ERR-003 | Exhaustive Matching | I=0.68 | semantics.md (pattern matching) |
| DR-ERR-004 | Immutable Default | I=0.55 | semantics.md (mutability) |

---

## Task Breakdown

### Phase 1.1: Grammar Specification (Priority: P1)
**Duration**: Est. 1-2 sessions
**Deliverable**: grammar/grammar.bnf

Tasks:
1. Define lexical rules (tokens, keywords, literals, operators)
2. Define expression grammar (precedence, associativity)
3. Define statement grammar (control flow, declarations)
4. Define module grammar (imports, exports)
5. Create precedence table
6. Validate grammar is context-free (no ambiguity)
7. Add grammar examples and test cases

**Success Criteria**:
- Grammar is context-free (DR-GRM-001)
- All production rules unambiguous (DR-GRM-002)
- Maximum nesting depth ≤3 documented (DR-GRM-003)
- Fixed precedence table complete (DR-GRM-004)

---

### Phase 1.2: Type System Specification (Priority: P1)
**Duration**: Est. 1-2 sessions
**Deliverable**: type-system/types.md

Tasks:
1. Define primitive types (int, float, bool, string, unit)
2. Define composite types (array, tuple, struct, enum)
3. Define type inference rules
4. Define type checking algorithm
5. Define generic/polymorphism rules (if applicable)
6. Document type system properties (soundness, completeness)
7. Add type system examples and edge cases

**Success Criteria**:
- All types explicitly declared (DR-SEM-001)
- Static type checking specified (DR-SEM-002)
- Type soundness proven or justified
- Type inference rules clear and deterministic

---

### Phase 1.3: Syntax Specification (Priority: P2)
**Duration**: Est. 1 session
**Deliverable**: syntax/syntax.md

Tasks:
1. Document variable declaration syntax
2. Document function definition syntax
3. Document control flow syntax (if, match, loops)
4. Document module system syntax
5. Create syntax examples for all constructs
6. Define identifier naming rules
7. Validate syntax maps to grammar.bnf

**Success Criteria**:
- Keywords used over symbols (DR-SYN-001)
- Explicit delimiters documented (DR-SYN-002)
- Bounds-checked indexing specified (DR-SYN-003)
- Linear control flow enforced (DR-SYN-004)
- Consistent naming rules (DR-SYN-005)

---

### Phase 1.4: Semantics Specification (Priority: P2)
**Duration**: Est. 1-2 sessions
**Deliverable**: semantics/semantics.md

Tasks:
1. Define evaluation order (strict vs lazy, left-to-right)
2. Define variable binding semantics
3. Define function call semantics
4. Define iterator protocol (DR-ERR-001)
5. Define error handling model (DR-ERR-002)
6. Define pattern matching semantics (DR-ERR-003)
7. Define mutability rules (DR-ERR-004)
8. Define scope rules (DR-SEM-003)
9. Define initialization rules (DR-SEM-004)

**Success Criteria**:
- Iterator-first design documented (DR-ERR-001)
- First-class error handling specified (DR-ERR-002)
- Exhaustive pattern matching required (DR-ERR-003)
- Immutable-by-default enforced (DR-ERR-004)
- Lexical scoping specified (DR-SEM-003)

---

### Phase 1.5: Memory Model Specification (Priority: P3)
**Duration**: Est. 1 session
**Deliverable**: memory-model/memory.md

Tasks:
1. Choose memory management strategy (ownership vs GC vs manual)
2. Define allocation rules (stack vs heap)
3. Define deallocation rules (deterministic vs non-deterministic)
4. Define reference semantics (if applicable)
5. Define lifetime rules (if applicable)
6. Document safety guarantees
7. Document performance characteristics

**Success Criteria**:
- Memory safety guaranteed (no undefined behavior)
- Performance predictable (deterministic or bounded)
- Mental model simple enough for LLM generation
- Trade-offs documented (safety vs performance vs simplicity)

---

### Phase 1.6: Integration & Validation (Priority: P3)
**Duration**: Est. 1 session
**Deliverable**: SPEC_SUMMARY.md, validation report

Tasks:
1. Cross-reference all specifications for consistency
2. Validate all DR-* rules are addressed
3. Create complete language specification summary
4. Generate example programs in Lang4LLM
5. Validate examples against grammar/types/semantics
6. Document known limitations and future extensions
7. Create Phase 1 gate checklist

**Success Criteria**:
- All 20 design rules addressed in specification
- No contradictions between components
- Example programs demonstrate all language features
- Specification is implementable (Phase 2 ready)

---

## Success Criteria (Phase 1 Gate)

### Completeness
- [ ] All 5 deliverables created (grammar, types, syntax, semantics, memory)
- [ ] All 20 design rules (DR-*) addressed in specifications
- [ ] Cross-references between documents are valid
- [ ] Example programs demonstrate all features

### Quality
- [ ] Grammar is provably context-free
- [ ] Type system is sound (or limitations documented)
- [ ] Semantics are unambiguous
- [ ] Memory model provides safety guarantees
- [ ] All specifications are implementable

### Validation
- [ ] At least 10 example programs written in Lang4LLM syntax
- [ ] Examples cover: variables, functions, control flow, types, errors, iterators
- [ ] Each example validated against grammar/types/semantics
- [ ] No ambiguities or contradictions found

### Documentation
- [ ] Each specification has CLAUDE.md for navigation
- [ ] Cross-references use consistent terminology
- [ ] Trade-offs and design decisions documented
- [ ] Known limitations explicitly stated

---

## Dependencies

**From Phase 0**:
- ✅ D1: Error Taxonomy (informs what to avoid)
- ✅ D2: Design Guide (20 rules = specification requirements)
- ✅ D3: Metrics Framework (validation criteria)
- ✅ D4: Benchmark Suite (test cases for examples)
- ✅ D5: Validation Report (feature priorities)

**For Phase 2**:
- ⏳ Grammar specification (parser input)
- ⏳ Type system specification (type checker input)
- ⏳ Semantics specification (interpreter/compiler behavior)
- ⏳ Memory model (code generator requirements)

---

## Risks & Mitigation

### Risk 1: Grammar Ambiguity
**Impact**: Parser cannot be deterministic
**Mitigation**: Use LALR(1) or LL(k) grammar checker, add disambiguation rules

### Risk 2: Type System Complexity
**Impact**: Hard for LLMs to generate correct types
**Mitigation**: Prefer simple types, extensive inference, explicit over implicit

### Risk 3: Memory Model Trade-offs
**Impact**: Safety vs Performance vs Simplicity conflict
**Mitigation**: Prototype multiple options, validate against Phase 0 metrics

### Risk 4: Specification Inconsistency
**Impact**: Implementation cannot satisfy all requirements
**Mitigation**: Cross-reference validation, integration phase catches conflicts

---

## Next Steps

1. Start with Phase 1.1 (Grammar) - highest impact (DR-GRM-001: I=0.96)
2. Proceed to Phase 1.2 (Type System) - depends on grammar
3. Complete Phase 1.3-1.5 (Syntax, Semantics, Memory) in parallel
4. Integration phase validates consistency
5. Gate review before Phase 2 (Parser implementation)

---

**Document Version**: 1.0
**Last Updated**: 2026-01-26
**Status**: Ready to begin Phase 1.1
