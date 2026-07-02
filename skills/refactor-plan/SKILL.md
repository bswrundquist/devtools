---
name: refactor-plan
description: Use when the user wants a plan to refactor existing code — captures what the current code actually solves, sets up characterization tests as a safety net, and lays out incremental, individually validatable steps so the new code provably solves the same problems.
tools: Bash, Read, Grep, Glob, Agent
argument-hint: <path or module> [goal, e.g. "split the god class", "make it async"]
---

# Refactor Plan

Plan a refactor around one guarantee: **the new code solves the same problems as the old code, and we can prove it at every step.** Behavior first, structure second, improvements last.

## Arguments

`$ARGUMENTS` — the target (path, module, or class name) plus an optional goal:

- `src/billing/` — plan a refactor of this area; infer the goal from the code's pain points.
- `src/billing/engine.py split the god class` — target plus explicit goal.

## Process

### 1. Scope and history

What is this code, how does it change, who knows it:

```bash
git log --oneline -15 -- src/billing/
git log --since="6 months ago" --no-merges --pretty=format: --name-only -- src/billing/ \
  | sort | uniq -c | sort -rn | head          # hot spots inside the target
git shortlog -sn --since="6 months ago" -- src/billing/ | head -5   # who to talk to
```

### 2. Behavior inventory — what does this code solve?

Read the code. List every responsibility, invariant, side effect, and **quirk** — quirks count, because callers may depend on them. Then find everyone who depends on it:

```bash
grep -rn "BillingEngine" --include="*.py" | grep -v test | head -30   # call sites
grep -rn "from billing" --include="*.py" -l | head                    # importers
```

### 3. Safety-net audit

What's already locked in by tests, and what isn't:

```bash
uv run pytest tests/billing/ -q
uv run pytest --cov=src/billing --cov-report=term-missing tests/ -q
```

List the **characterization tests to write before refactoring** — golden tests that pin current behavior, including the quirks (marked as quirks, so a future intentional change knows what it's changing).

### 4. Target design

The new structure, with an explicit old → new mapping. Every current responsibility either lands somewhere in the new design or is explicitly, intentionally dropped — nothing falls silently.

### 5. Migration steps

Break the refactor into steps where each one:

- keeps the full test suite green,
- is independently revertable,
- is small enough to review (< ~400 lines of diff).

Use strangler/parallel-run patterns when replacing a whole component: build the new path beside the old, diff their outputs on real inputs, then cut over.

## Output Format

```markdown
## Refactor Plan: <target> — <goal>

### What this code solves today
Responsibilities, invariants, side effects, and quirks — each with `file:line`.

### Safety net
| Behavior | Covered by | To write first |
|----------|-----------|----------------|
| proration rounds half-up | `test_prorate.py::test_rounding` | — |
| discount stacking order (quirk) | nothing | golden test on 5 real invoices |

### Old → New mapping
| Today | Becomes | Notes |
|-------|---------|-------|
| `BillingEngine.run()` (600 lines) | `Pipeline` + 4 stage classes | same public entry point |
| implicit retry in `_post()` | dropped — moved to caller | **intentional change**, see below |

### Steps
| Step | Change | Validated by | Risk / rollback |
|------|--------|--------------|-----------------|
| 0 | Write characterization tests | new tests green on current code | none — additive |
| 1 | Extract `ProrationCalc`, old code delegates | full suite green | revert one commit |

### Explicitly changing
Any behavior that will differ, why that's intentional, and who signs off on it.
```

## Rules

- **No step without a validation gate.** A step that can't be checked isn't a step, it's a hope.
- Step 0 is always the safety net: if coverage is too thin to refactor safely, the first steps of the plan *are* the tests.
- Behavior parity first — improvements and "while we're here" fixes ride in separate steps or separate PRs, never smuggled into a parity step.
- A quirk is a feature until proven otherwise: check the call sites before planning to "fix" surprising behavior, and route real fixes through **Explicitly changing**.
- Every responsibility in the inventory must appear in the mapping table — dropped ones explicitly.
- Hand off to `/implement` to execute the plan step by step.
