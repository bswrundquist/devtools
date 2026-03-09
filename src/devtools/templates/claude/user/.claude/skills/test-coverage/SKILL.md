---
name: test-coverage
description: Use when the user wants to analyze test coverage — what's tested, what's not, and where the highest-risk untested code lives. Runs pytest with coverage and provides actionable analysis beyond raw numbers.
tools: Bash, Read, Grep, Glob, Agent
---

# Test Coverage

Analyze test coverage to find untested code and prioritize what to test next.

## Process

1. **Run coverage** - Execute the test suite with coverage measurement.
2. **Parse results** - Identify files and lines not covered.
3. **Risk analysis** - Cross-reference uncovered code with risk factors (error handling, auth, data mutation, external calls).
4. **Prioritize** - Rank untested areas by risk and suggest what to test first.

## Running Coverage

```bash
# Preferred: pytest with coverage
pytest --cov=. --cov-report=term-missing --cov-report=json -q 2>/dev/null

# Alternative: coverage.py directly
coverage run -m pytest -q && coverage report --show-missing && coverage json

# If neither works, check for coverage config
cat .coveragerc pyproject.toml setup.cfg 2>/dev/null | grep -A 10 '\[tool.coverage\]\|\[coverage:\]\|\[tool:pytest\]'
```

## Analysis

### Overall Coverage
Report headline numbers:
- **Line coverage**: X% (Y/Z lines)
- **Branch coverage**: X% (if available)
- **Files with 0% coverage**: list them

### Uncovered Code by Risk Level

#### 🔴 High Risk — Test These First
Code that handles:
- **Authentication & authorization** — Login, token validation, permission checks
- **Payment & financial logic** — Charging, refunds, balance calculations
- **Data mutations** — Database writes, deletes, state transitions
- **Error handling** — Exception handlers, fallback logic, retry mechanisms
- **Input validation** — Parsing user input, API request validation
- **Security boundaries** — Sanitization, access control, encryption

#### 🟡 Medium Risk
- **Business logic** — Core domain functions and calculations
- **External integrations** — API calls, webhook handlers, message queue consumers
- **Data transformations** — Serialization, format conversion, ETL steps
- **Configuration parsing** — Loading and validating config

#### 🟢 Low Risk
- **Utility functions** — Formatters, helpers, string manipulation
- **Logging and monitoring** — Log statements, metrics emission
- **CLI entry points** — Argument parsing (if using a framework like Typer)
- **Type definitions** — Dataclasses, Pydantic models (structure only)

### Per-File Breakdown

For files below the coverage threshold:
```
| File | Coverage | Missing Lines | Risk | Priority |
|------|----------|---------------|------|----------|
| auth.py | 34% | 42-67, 89-103 | 🔴 | Test first |
| api.py | 61% | 28-35, 78-92 | 🟡 | Schedule |
| utils.py | 82% | 15-18 | 🟢 | Low |
```

### Untested Functions

List specific functions with 0% coverage, grouped by risk:
```
🔴 auth.py:validate_token() — No tests, handles JWT validation
🔴 payments.py:process_refund() — No tests, mutates financial data
🟡 api.py:handle_webhook() — No tests, processes external events
🟢 utils.py:format_date() — No tests, pure formatting function
```

## Recommendations

### Quick Wins
Tests that would add the most coverage with the least effort:
- Functions that are simple but untested
- Files where a single test function would cover 50%+ of missing lines

### Critical Gaps
Untested code paths that represent real risk:
- Error handling branches that are never exercised
- Edge cases in business logic (empty inputs, boundary values, concurrent access)
- Failure modes for external service calls

### Coverage Targets
Suggest realistic targets based on current state:
- If at 30%: aim for 60% by covering high-risk paths
- If at 60%: aim for 80% by covering medium-risk and edge cases
- If at 80%+: focus on branch coverage and error paths

## Rules

- Don't treat coverage as a goal — treat it as a tool for finding risk.
- 100% coverage doesn't mean bug-free; 50% coverage on critical paths is better than 90% on boilerplate.
- Exclude from analysis: test files themselves, migrations, auto-generated code, `__init__.py` files.
- If pytest or coverage isn't installed, tell the user how to install it (`pip install pytest pytest-cov`) and stop.
- If tests fail, report the failures first — coverage numbers on a failing suite are meaningless.
- Don't suggest testing trivial code (empty `__init__.py`, simple dataclass definitions, constant declarations).
- Consider branch coverage, not just line coverage — an if/else where only the if-branch is tested has 50% branch coverage even if line coverage looks higher.
