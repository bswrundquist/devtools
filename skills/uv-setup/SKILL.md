---
name: uv-setup
description: Use when setting up Python projects, managing dependencies, lock files, and virtual environments, structuring workspaces in monorepos, or running uv in CI and Docker.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# uv

Manage Python projects, dependencies, interpreters, and environments with uv — the only tool needed.

## Daily commands

| Command | What it does |
|---------|--------------|
| `uv init myproj` / `uv init --lib myproj` | New app / library project (src layout) |
| `uv add httpx` | Add dependency, update lock, sync venv |
| `uv add --dev pytest ruff mypy` | Add to `[dependency-groups]` dev |
| `uv remove httpx` | Remove dependency |
| `uv run pytest -q` | Run inside the project venv (auto-syncs first) |
| `uv sync` | Materialize venv from lock |
| `uv sync --frozen` | Sync exactly from lock; fail if out of date (CI) |
| `uv lock --upgrade-package httpx` | Upgrade one package in the lock |
| `uv lock --upgrade` | Upgrade everything |
| `uvx ruff check .` | Run a tool without installing it |

## Project setup

```bash
uv init --lib myproj && cd myproj
uv python pin 3.12               # writes .python-version — commit it
uv add httpx pydantic
uv add --dev pytest mypy ruff
uv run pytest -q
```

`uv run` is the universal entry point — never activate the venv manually, never call bare `python` or `pytest`.

## Dependency groups

```toml
[project]
dependencies = ["httpx>=0.28"]

[dependency-groups]
dev = ["pytest>=8", "mypy>=1.13", "ruff>=0.8"]
docs = ["mkdocs-material>=9"]
```

`dev` installs by default with `uv sync`; exclude with `--no-dev`, add others with `--group docs`. Use `[dependency-groups]` for dev tooling — not the legacy `[tool.uv.dev-dependencies]` and not `[project.optional-dependencies]`.

## Python versions

```bash
uv python install 3.12      # uv downloads and manages interpreters — no pyenv
uv python pin 3.12          # .python-version, respected by uv run/sync
```

## Workspaces (monorepos)

Root `pyproject.toml`:

```toml
[tool.uv.workspace]
members = ["packages/*", "apps/*"]
```

A member depending on a sibling:

```toml
[project]
dependencies = ["core"]

[tool.uv.sources]
core = { workspace = true }
```

One `uv.lock` at the root covers all members. Target one member with `uv run --package api pytest`.

## Docker

```dockerfile
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim

WORKDIR /app
ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy

# Layer 1: dependencies only — cached until pyproject/lock change
COPY pyproject.toml uv.lock ./
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev --no-install-project

# Layer 2: project code
COPY src/ src/
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev

CMD ["uv", "run", "--no-sync", "python", "-m", "myproj"]
```

Copy pyproject + lock before source so dependency layers survive code changes.

## GitHub Actions

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v5
        with:
          enable-cache: true
      - run: uv python install
      - run: uv sync --frozen
      - run: uv run ruff check .
      - run: uv run mypy src
      - run: uv run pytest -q
```

## Rules

- Always commit `uv.lock` and `.python-version` — reproducibility is the whole point.
- CI and Docker use `uv sync --frozen` — never bare `uv sync`, which may silently re-lock.
- `uv run <cmd>` for everything; if you are typing `source .venv/bin/activate`, stop.
- Dev tooling goes in `[dependency-groups]`, never in `[project.dependencies]`.
- Upgrade deliberately: `uv lock --upgrade-package <name>`, then review the lock diff in the PR.
- `uvx` for one-off tools — don't pollute the project with them.
- No pip, no poetry, no requirements.txt; export only when a legacy consumer demands it: `uv export --format requirements-txt`.
- In Docker, split sync into deps-only (`--no-install-project`) then full sync to maximize layer cache hits.
- One lock per workspace root; members never get their own.
- Pin the uv version in CI via `setup-uv`'s `version:` input when tooling reproducibility matters.
