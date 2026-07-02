---
name: pre-commit-setup
description: Use when adding pre-commit to a project, choosing and configuring hooks (ruff, mypy, etc.), writing custom local hooks, or integrating pre-commit with CI.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Pre-commit Hooks

Set up and maintain pre-commit hooks — fast local quality gates that mirror CI.

## Baseline config for a Python/uv project

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v5.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-added-large-files
        args: [--maxkb=500]
      - id: check-merge-conflict
      - id: detect-private-key

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.12.1
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  # Slow hooks run on push, not on every commit
  - repo: local
    hooks:
      - id: mypy
        name: mypy
        entry: uv run mypy src/
        language: system
        types: [python]
        pass_filenames: false
        stages: [pre-push]
```

## Install and adopt

```bash
uv add --dev pre-commit
uv run pre-commit install                        # commit-stage hooks
uv run pre-commit install --hook-type pre-push   # push-stage hooks
uv run pre-commit run --all-files                # one-time full sweep on adoption
```

Commit the full-sweep fixes as a standalone `chore: apply pre-commit to all files` commit so they don't pollute feature diffs.

## Local hooks with uv

`language: system` runs tools from the project venv — the exact versions from `uv.lock`, matching CI and dev.

```yaml
- repo: local
  hooks:
    - id: pytest-fast
      name: fast unit tests
      entry: uv run pytest tests/unit -x -q
      language: system
      pass_filenames: false
      stages: [pre-push]
    - id: uv-lock-check
      name: uv lock is current
      entry: uv lock --check
      language: system
      files: ^pyproject\.toml$
      pass_filenames: false
```

## Keeping hooks current

```bash
uv run pre-commit autoupdate        # bump every rev: to the latest tag
uv run pre-commit run --all-files   # verify nothing new breaks before committing
```

## CI integration

Run the same hooks in CI — local hooks are advisory, CI is the gate.

```yaml
# GitHub Actions
lint:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: astral-sh/setup-uv@v5
    - run: uv sync --frozen
    - run: uv run pre-commit run --all-files
```

```yaml
# GitLab CI
pre-commit:
  stage: lint
  image: python:3.12-slim
  variables:
    PRE_COMMIT_HOME: "$CI_PROJECT_DIR/.cache/pre-commit"
  cache:
    paths: [.cache/pre-commit]
  script:
    - pip install uv && uv sync --frozen
    - uv run pre-commit run --all-files
```

Alternatively enable pre-commit.ci — it runs hooks on PRs, pushes autofixes, and opens weekly autoupdate PRs. Note: it cannot run `language: system` hooks; keep those in your own CI job.

## Skipping hooks honestly

| Mechanism | Effect | When acceptable |
|---|---|---|
| `SKIP=mypy git commit` | Skips only the named hook ids; everything else runs | A known-broken or flaky hook you'll fix soon |
| `git commit --no-verify` | Skips ALL hooks silently | Emergencies only (mid-rebase, hotfix) — CI still enforces |

`SKIP` is targeted and honest. `--no-verify` is a blanket bypass — if it becomes a habit, the hook setup is broken (too slow, too noisy): fix that instead.

## Rules

- Always pin `rev:` to a specific tag — never a branch name. Unpinned hooks make results non-reproducible across machines.
- Keep slow hooks (mypy, test suites) on `stages: [pre-push]` — commit hooks over a few seconds get bypassed by everyone.
- Run `pre-commit autoupdate` periodically (or delegate to pre-commit.ci), then re-verify with `run --all-files`.
- On adoption, run `pre-commit run --all-files` once and commit the sweep separately from feature work.
- Use `language: system` + `uv run` for tools already in project dev-deps — one pinned version of ruff/mypy, not two drifting ones.
- CI must run `pre-commit run --all-files` (or pre-commit.ci) — never trust local hooks alone.
- Don't stack competing formatters: if `ruff-format` is in, black is out.
- Prefer `SKIP=<id>` over `--no-verify`; reserve `--no-verify` for genuine emergencies.
- Hooks must be deterministic and fast — no network calls, no "sometimes fails" hooks; those belong in CI jobs.
