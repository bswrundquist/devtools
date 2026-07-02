---
name: implement
description: Use when the user wants to implement a solution design, issue, or plan — writes clean, fully typed code with keyword arguments at call sites, adds unit tests, and iterates until tests, types, and lint are all green.
tools: Bash, Read, Edit, Write, Grep, Glob, Agent
argument-hint: <design file, issue ref, or plan> [--no-branch]
---

# Implement

Take a solution design (from `/solution-design`), an issue, or an in-conversation plan, and implement it to a finished standard: typed, tested, lint-clean, and honestly reported.

## Arguments

`$ARGUMENTS` — what to implement:

- A path to a design file (`docs/designs/2026-07-02-retry-queue.html` or `.md`) — parse its work items and overview table.
- An issue ref (same forms as `/executive-summary`) — fetch it and derive the plan.
- Nothing — use the plan already established in this conversation.
- `--no-branch` — stay on the current branch instead of creating one.

## Process

### 1. Load the plan

Read the design file or issue. Extract the work items and their dependency order — implement in that order.

### 2. Preflight

```bash
git status --short          # dirty tree? surface it before touching anything
git branch --show-current   # on main/master? branch off first
git checkout -b feat/<slug> # unless --no-branch or already on a feature branch
```

### 3. Detect the tooling

Prefer the project's own entry points over raw tools:

```bash
ls Makefile pyproject.toml uv.lock package.json 2>/dev/null
grep -E "^[a-zA-Z_-]+:" Makefile 2>/dev/null | head   # test / lint / typecheck targets?
```

If a Makefile has `test` / `lint` / `typecheck` targets, use those. Otherwise run the tools directly (`uv run pytest`, `uv run ruff check`, `uv run mypy` / `pyright`, `npm test`, ...).

### 4. Implement — code standards

- **Fully typed.** Annotations on every function signature and public attribute. Run the type checker; don't just decorate.
- **Keyword arguments at call sites — both sides of the equation named:**

```python
def build_report(*, source: Path, fmt: ReportFormat, strict: bool = True) -> Report:
    ...

report = build_report(source=source, fmt=fmt, strict=strict)
```

- Prefer keyword-only parameters (`*,`) for any function with two or more parameters.
- No `# type: ignore`, `cast`, or broad `Any` to make errors disappear — fix the types.
- Match the surrounding code's style, naming, and idioms; small focused functions.
- Write unit tests alongside each work item, not as a final batch. Test behavior, cover the error paths.

### 5. Validate — iterate until all green

```bash
make test        # or: uv run pytest -x -q
make lint        # or: uv run ruff check . && uv run ruff format --check .
make typecheck   # or: uv run mypy .   (or pyright — whatever the project configures)
```

A failure means loop back to step 4. Do not proceed to the report with anything red unless it's genuinely blocked — and then say so.

### 6. Report

```markdown
## Implementation Report

| Design item | Status | Files | Validated by |
|-------------|--------|-------|--------------|
| 1. Idempotency key on webhook | ✅ done | `api/webhooks.py`, `tests/test_webhooks.py` | 4 new tests, mypy clean |
| 2. Retry queue consumer | ⚠️ partial | `worker/consumer.py` | blocked on open question #2 (queue name) |

**Tests:** 42 passed · **Types:** clean · **Lint:** clean
```

Include `git diff --stat` for the change footprint, then suggest `/commit`.

## Rules

- Implement in the design's dependency order; don't cherry-pick the fun items first.
- If a work item is gated by an unanswered open question, implement everything else and flag it — don't guess the answer into the code.
- Never weaken validation to get to green (skipping tests, loosening assertions, ignoring types).
- Report failures honestly, with the failing output — a red report is more useful than a fake green one.
- Don't commit or push unless asked; end by suggesting `/commit`.
- If reality contradicts the design mid-implementation (API doesn't exist, approach can't work), stop and surface it rather than silently improvising a different design.
