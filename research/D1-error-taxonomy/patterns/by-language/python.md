# Python-Specific Error Patterns

**Language**: Python 3.x
**Last Updated**: 2026-01-20
**Analysis Scope**: HumanEval, MBPP benchmarks

## Overview

Python is the most common target for LLM code generation. Its dynamic typing and significant whitespace create unique error patterns not present in other languages.

## Error Pattern Distribution

```
Syntax Errors:      32% ████████████████████████████████
Semantic Errors:    38% ██████████████████████████████████████
Logic Errors:       30% ██████████████████████████████████
```

**Total Patterns Analyzed**: 100+ from literature (P01, P02, P07, P08)

## Python-Specific Patterns

### 1. Indentation Errors (EP-SYN-002)

**Frequency**: 20% of Python syntax errors
**Severity**: MEDIUM (blocks execution)

**Pattern**: Mixing tabs/spaces, inconsistent indentation within blocks

```python
# Incorrect
def process(items):
    for item in items:
      if item.valid:  # 2 spaces
          result = transform(item)  # 4 spaces - inconsistent
            save(result)  # 6 spaces - wrong
    return True

# Correct
def process(items):
    for item in items:
        if item.valid:
            result = transform(item)
            save(result)
    return True
```

**Root Cause**: Python's semantic whitespace requires precise indentation tracking. LLMs lose indentation context across generation steps.

**Mitigation**: Language design should avoid significant whitespace.

---

### 2. Undefined Variable in Conditional Scope (EP-SEM-001)

**Frequency**: 20% of Python semantic errors
**Severity**: HIGH (NameError at runtime)

**Pattern**: Variable defined only inside conditional block

```python
# Incorrect
def find_user(user_id):
    if user_id in cache:
        user = cache[user_id]
    return user  # NameError if not in cache

# Correct
def find_user(user_id):
    user = None
    if user_id in cache:
        user = cache[user_id]
    return user
```

**Root Cause**: Python's dynamic scoping allows late binding. LLMs assume variables exist without explicit declaration.

**Mitigation**: Require explicit variable declarations (like Rust's `let`).

---

### 3. Missing `self` in Methods (EP-SEM-003)

**Frequency**: 12% of Python semantic errors
**Severity**: MEDIUM (TypeError at runtime)

**Pattern**: Forgot `self` parameter or `self.` prefix

```python
# Incorrect
class DataProcessor:
    def process(data):  # Missing 'self'
        return transform(data)

    def save(self):
        filename = self.generate_name()  # Missing 'self.'
        # ...

# Correct
class DataProcessor:
    def process(self, data):
        return transform(data)

    def save(self):
        filename = self.generate_name()
        # ...
```

**Root Cause**: Explicit `self` is unique to Python among popular languages. LLMs trained on Java/C++ omit it.

**Mitigation**: Implicit `this`/`self` (like most OOP languages).

---

### 4. Dictionary KeyError (EP-LOG-001)

**Frequency**: 16% of Python logic errors
**Severity**: HIGH (crashes on missing keys)

**Pattern**: Accessing dict key without checking existence

```python
# Incorrect
def get_config(key):
    return config[key]  # KeyError if key missing

# Correct
def get_config(key):
    return config.get(key, default_value)
```

**Root Cause**: Python dicts raise KeyError instead of returning null/None. LLMs expect JavaScript/TypeScript-like behavior.

**Mitigation**: Return Option/Result types instead of exceptions.

---

### 5. List Index Out of Range (EP-SYN-003 + EP-LOG-001)

**Frequency**: 14% of Python logic errors
**Severity**: HIGH (IndexError at runtime)

**Pattern**: Off-by-one errors, accessing beyond list bounds

```python
# Incorrect
def get_last(items):
    return items[len(items)]  # Off-by-one

# Correct
def get_last(items):
    if not items:
        raise ValueError("Empty list")
    return items[-1]  # Pythonic negative indexing
```

**Root Cause**: Python's 0-indexed lists vs natural language "first element" (1-indexed).

**Mitigation**: Bounds-checked indexing (panic on out-of-bounds).

---

## Python-Specific Strengths

**What LLMs Handle Well**:
1. **List comprehensions**: High success rate (pattern-matchable)
2. **String formatting**: f-strings widely used in training data
3. **Context managers**: `with` statements well-represented
4. **Exception handling**: `try/except` commonly generated correctly

## Python-Specific Weaknesses

**What LLMs Struggle With**:
1. **Generator expressions**: Confused with list comprehensions
2. **Decorators**: Syntax understood, semantics often wrong
3. **Metaclasses**: Rarely correct in generated code
4. **Type hints**: Optional, often omitted or incorrect
5. **Async/await**: Concurrency model confuses synchronous patterns

## Design Recommendations for LLM-Friendly Language

Based on Python analysis:

1. **Avoid significant whitespace** → Use explicit block delimiters
2. **Require variable declarations** → Explicit `let`/`const`/`var`
3. **Implicit self in methods** → Match mainstream OOP conventions
4. **Return Option<T> instead of exceptions** → Make errors explicit
5. **Bounds-checked indexing** → Panic instead of silent errors
6. **Mandatory type annotations** → Enable static checking

## Cross-Reference

**Patterns Present in Python**:
- All syntax patterns (EP-SYN-001 through EP-SYN-004)
- All semantic patterns (EP-SEM-001 through EP-SEM-004)
- All logic patterns (EP-LOG-001 through EP-LOG-004)

**Python-Unique**:
- EP-SYN-002 (indentation) - most severe in Python
- EP-SEM-001 (scope) - dynamic scoping exacerbates
- Missing `self` - Python-specific convention

## References

- P01: HumanEval analysis (100% Python)
- P02: Python-heavy benchmark suite
- P07: Python bug taxonomy
- P08: Python mistake classification
