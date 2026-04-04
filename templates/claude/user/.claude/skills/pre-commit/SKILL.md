---
name: pre-commit
description: Use when setting up or maintaining pre-commit hooks — configuring .pre-commit-config.yaml, choosing hooks, running checks, writing custom hooks, and integrating with CI.
tools: Read, Write, Edit, Bash
---

# pre-commit

Set up and maintain pre-commit hooks to catch issues before they're committed.

## Setup

```bash
pip install pre-commit
# or
uv add --dev pre-commit

# Install the git hooks
pre-commit install

# Install commit-msg hooks too (for conventional commits, etc.)
pre-commit install --hook-type commit-msg
```

## .pre-commit-config.yaml

```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v5.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-json
      - id: check-merge-conflict
      - id: check-added-large-files
        args: [--maxkb=500]
      - id: no-commit-to-branch
        args: [--branch, main, --branch, master]

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.8.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.13.0
    hooks:
      - id: mypy
        additional_dependencies: [pydantic, types-requests]
```

## Useful hooks

### General

```yaml
- repo: https://github.com/pre-commit/pre-commit-hooks
  rev: v5.0.0
  hooks:
    - id: trailing-whitespace      # remove trailing whitespace
    - id: end-of-file-fixer        # ensure files end with newline
    - id: check-yaml               # validate YAML syntax
    - id: check-toml               # validate TOML syntax
    - id: check-json               # validate JSON syntax
    - id: check-merge-conflict     # detect merge conflict markers
    - id: check-added-large-files  # prevent large file commits
    - id: check-case-conflict      # catch case-insensitive filename conflicts
    - id: mixed-line-ending        # normalize line endings
    - id: detect-private-key       # catch accidentally committed keys
    - id: no-commit-to-branch      # prevent commits directly to main
```

### Python

```yaml
- repo: https://github.com/astral-sh/ruff-pre-commit
  rev: v0.8.0
  hooks:
    - id: ruff           # lint + autofix
      args: [--fix]
    - id: ruff-format    # format (replaces black)

- repo: https://github.com/pre-commit/mirrors-mypy
  rev: v1.13.0
  hooks:
    - id: mypy
      additional_dependencies: [pydantic>=2, sqlalchemy[mypy]]
      args: [--ignore-missing-imports]

- repo: https://github.com/Lucas-C/pre-commit-hooks-safety
  rev: v1.3.3
  hooks:
    - id: python-safety-dependencies-check
      files: requirements
```

### Secrets detection

```yaml
- repo: https://github.com/Yelp/detect-secrets
  rev: v1.5.0
  hooks:
    - id: detect-secrets
      args: [--baseline, .secrets.baseline]

# Alternative: gitleaks
- repo: https://github.com/gitleaks/gitleaks
  rev: v8.21.0
  hooks:
    - id: gitleaks
```

### Conventional commits

```yaml
- repo: https://github.com/compilerla/conventional-pre-commit
  rev: v3.6.0
  hooks:
    - id: conventional-pre-commit
      stages: [commit-msg]
      args: [feat, fix, docs, style, refactor, test, chore, ci]
```

### Docker

```yaml
- repo: https://github.com/hadolint/hadolint
  rev: v2.13.0
  hooks:
    - id: hadolint-docker
```

### YAML / JSON / TOML

```yaml
- repo: https://github.com/adrienverge/yamllint
  rev: v1.35.1
  hooks:
    - id: yamllint
      args: [-d, relaxed]

- repo: https://github.com/python-jsonschema/check-jsonschema
  rev: 0.29.4
  hooks:
    - id: check-github-workflows     # validate .github/workflows/*.yml
    - id: check-gitlab-ci            # validate .gitlab-ci.yml
```

## Running hooks

```bash
# Run all hooks against all files (useful on first setup)
pre-commit run --all-files

# Run a specific hook
pre-commit run ruff --all-files

# Run against staged files only (what happens on commit)
pre-commit run

# Run against specific files
pre-commit run --files src/myfile.py

# Skip hooks for one commit
SKIP=mypy git commit -m "wip"
# or
git commit -m "wip" --no-verify   # skips all hooks (avoid this)

# Skip hooks by hook ID (preferred over --no-verify)
SKIP=mypy,ruff git commit -m "temp"
```

## Updating hooks

```bash
# Update all hooks to latest versions
pre-commit autoupdate

# Update a specific repo
pre-commit autoupdate --repo https://github.com/astral-sh/ruff-pre-commit
```

Pin `rev:` to a specific tag, not a branch. `pre-commit autoupdate` handles this.

## Hook configuration options

```yaml
hooks:
  - id: ruff
    name: "Ruff linter"            # display name in output
    args: [--fix, --exit-non-zero-on-fix]
    files: '^src/'                  # only run on files matching this regex
    exclude: '^tests/fixtures/'     # exclude files matching this regex
    types: [python]                 # only Python files
    types_or: [python, pyi]        # Python or stub files
    stages: [pre-commit, pre-push]  # when to run (default: pre-commit)
    always_run: false               # run even if no matching files
    pass_filenames: false           # don't pass filenames to the hook
    language_version: python3.12   # specific Python version
```

## Writing a custom hook

### Local hook (inline script, no separate repo)

```yaml
repos:
  - repo: local
    hooks:
      - id: check-migrations
        name: Check for missing migrations
        language: python
        entry: python scripts/check_migrations.py
        files: 'src/models/'
        pass_filenames: false

      - id: run-pytest-fast
        name: Run fast tests
        language: system
        entry: uv run pytest -m "not slow" -q
        pass_filenames: false
        stages: [pre-push]   # only on push, not every commit
```

### Hook script (in the same repo)

```python
#!/usr/bin/env python3
# scripts/check_migrations.py
"""Verify all model changes have a corresponding migration."""
import subprocess
import sys

result = subprocess.run(["alembic", "check"], capture_output=True, text=True)
if result.returncode != 0:
    print("ERROR: Unmigrated model changes detected.")
    print(result.stdout)
    print("Run: alembic revision --autogenerate -m 'describe change'")
    sys.exit(1)
```

## CI integration

Run pre-commit in CI to catch issues that slipped through:

### GitHub Actions

```yaml
- uses: actions/checkout@v4
- uses: actions/setup-python@v5
  with:
    python-version: "3.12"
- uses: pre-commit/action@v3.0.1
  # Uses pre-commit.ci or runs locally; caches hook envs
```

Or with cache:

```yaml
- uses: actions/cache@v4
  with:
    path: ~/.cache/pre-commit
    key: pre-commit-${{ hashFiles('.pre-commit-config.yaml') }}
- run: pre-commit run --all-files
```

### GitLab CI

```yaml
pre-commit:
  image: python:3.12-slim
  variables:
    PRE_COMMIT_HOME: $CI_PROJECT_DIR/.pre-commit-cache
  cache:
    key: pre-commit-${{ .pre-commit-config.yaml }}
    paths:
      - .pre-commit-cache/
  script:
    - pip install pre-commit
    - pre-commit run --all-files
```

## .pre-commit-config.yaml for a typical Python project

```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v5.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-merge-conflict
      - id: check-added-large-files
      - id: no-commit-to-branch
        args: [--branch, main]

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.8.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.13.0
    hooks:
      - id: mypy
        additional_dependencies: [pydantic>=2]
```

## Rules

- Always pin `rev:` to a specific tag — never a branch name.
- Run `pre-commit autoupdate` periodically and commit the result.
- Prefer `SKIP=hook-id` over `--no-verify` — it skips specific hooks instead of all of them.
- Hooks that auto-fix files (ruff --fix, ruff-format) will fail the first run and fix the file — stage the changes and commit again.
- Keep slow hooks (mypy, pytest) on `stages: [pre-push]` not `pre-commit` — don't slow down every commit.
- Add `.pre-commit-config.yaml` to CI so hooks are enforced even if developers skip them locally.
