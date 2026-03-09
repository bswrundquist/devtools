---
name: tech-debt
description: Use when the user wants to identify and catalog technical debt in the codebase. Finds TODOs, complex functions, dead code, missing types, untested areas, and other code health issues. Produces a prioritized report.
tools: Bash, Read, Grep, Glob, Agent
---

# Tech Debt

Identify, categorize, and prioritize technical debt across the codebase.

## Process

1. **Scan markers** - Find explicit debt markers (TODO, FIXME, HACK, XXX, NOQA).
2. **Analyze complexity** - Find overly complex functions and files.
3. **Detect dead code** - Find unused imports, functions, variables, and files.
4. **Check type coverage** - Find functions missing type hints.
5. **Identify patterns** - Find code smells and anti-patterns.
6. **Prioritize** - Rank findings by impact and effort.

## Scans

### 1. Explicit Debt Markers
```bash
# Find all TODO/FIXME/HACK/XXX comments
grep -rn 'TODO\|FIXME\|HACK\|XXX\|NOQA\|WORKAROUND\|TEMPORARY\|KLUDGE' --include='*.py' .

# Count by type
grep -roh 'TODO\|FIXME\|HACK\|XXX' --include='*.py' . | sort | uniq -c | sort -rn
```

Categorize by urgency:
- **FIXME/HACK** — Known bugs or ugly workarounds (fix soon)
- **TODO** — Planned improvements (schedule)
- **XXX** — Dangerous or fragile code (investigate)

### 2. Complexity Analysis
Find large and complex functions:
```bash
# Files with the most lines of code (excluding tests, migrations)
find . -name '*.py' ! -path '*/test*' ! -path '*migration*' ! -path '*__pycache__*' -exec wc -l {} + | sort -rn | head -20

# Long functions (functions with many lines)
# Look for function definitions and count lines to next definition
```

Flag:
- Functions over 50 lines
- Files over 500 lines
- Classes over 300 lines
- Deeply nested code (3+ levels of indentation in logic)
- Functions with more than 5 parameters

### 3. Dead Code Detection
```bash
# Unused imports (if ruff/flake8 available)
ruff check --select F401 . 2>/dev/null
flake8 --select F401 . 2>/dev/null

# Functions defined but never called elsewhere
# Find all function definitions, then check for usage
```

Look for:
- Imported but unused modules
- Defined but uncalled functions (check all call sites)
- Commented-out code blocks (more than 5 lines)
- Unreachable code after return/raise/break
- Feature flags that are always on/off

### 4. Type Coverage
```bash
# Functions missing return type hints
grep -rn 'def .*):$' --include='*.py' . | grep -v '-> '

# If mypy is available
mypy . --ignore-missing-imports 2>/dev/null | head -50
```

### 5. Code Smells
Search for common anti-patterns:

- **God objects** — Classes with 10+ methods or 20+ attributes
- **Copy-paste** — Duplicate or near-duplicate code blocks
- **Magic numbers** — Hardcoded numeric values without named constants
- **Broad exception handling** — `except Exception` or bare `except:`
- **String-typed interfaces** — Using strings where enums or types should be used
- **Deep nesting** — More than 3 levels of if/for/try nesting
- **Long parameter lists** — Functions with 5+ parameters (should be a dataclass/config)

### 6. Test Gaps
```bash
# Find source files without corresponding test files
# Compare src/ files against test/ files
```

- Source modules with no test file
- Test files that are significantly smaller than source files
- Test files with no assertions

## Output Format

### Tech Debt Report

**Summary Stats:**
- Total debt markers: N (X FIXME, Y TODO, Z HACK)
- Large functions (>50 lines): N
- Files without tests: N
- Functions without type hints: N

### 🔴 High Priority (Fix Soon)
Issues that cause bugs, block development, or indicate fragile code.

| Issue | Location | Description | Effort |
|-------|----------|-------------|--------|
| FIXME | auth.py:42 | Token refresh race condition | Medium |

### 🟡 Medium Priority (Schedule)
Issues that hurt maintainability or developer experience.

### 🟢 Low Priority (When Convenient)
Style issues, missing types on internal functions, minor improvements.

### Refactoring Targets
Top 5 files/functions that would benefit most from refactoring, based on:
- High churn (frequently changed) + high complexity
- Multiple debt markers in the same area
- Large size + poor test coverage

### Recommendations
Prioritized action items with estimated effort (small/medium/large).

## Rules

- Exclude test files, migrations, auto-generated code, and vendored dependencies from complexity analysis.
- Don't count type hints missing on test functions or `__init__.py` as debt.
- Context matters: a 100-line function that's a clear state machine is fine; a 30-line function with nested conditionals may not be.
- Focus on actionable findings — "this file is big" is not useful; "this file has 3 functions over 50 lines that could be extracted" is.
- If the project is small (<20 files), adjust thresholds accordingly.
- Don't flag dependencies as tech debt here — that's what `/dependency-audit` is for.
