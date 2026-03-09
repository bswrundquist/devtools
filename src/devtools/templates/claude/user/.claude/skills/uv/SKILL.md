---
name: uv
description: Use when working with uv — Python package and project manager. Covers project setup, dependency management, workspaces, scripts, tool management, Python version management, CI integration, and Docker.
tools: Bash, Read, Write, Edit
---

# uv

Modern Python package and project manager. Replaces pip, pip-tools, virtualenv, pyenv, poetry, and pipx.

## Project setup

```bash
# New project
uv init myproject
cd myproject

# New project with package structure (src layout)
uv init myproject --package
cd myproject

# In an existing directory
uv init
```

Creates `pyproject.toml`, `.python-version`, and `hello.py`.

## Adding and removing dependencies

```bash
# Add a runtime dependency
uv add httpx pydantic

# Add a dev dependency
uv add --dev pytest ruff mypy

# Add an optional dependency group
uv add --group lint ruff mypy

# Add with version constraint
uv add "fastapi>=0.100" "sqlalchemy>=2.0,<3"

# Remove a dependency
uv remove httpx

# Sync installed packages to match lock file (after git pull, etc.)
uv sync

# Sync including dev dependencies
uv sync --dev

# Sync a specific group
uv sync --group lint
```

## pyproject.toml structure

```toml
[project]
name = "myproject"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "fastapi>=0.100",
    "pydantic>=2",
    "sqlalchemy>=2",
]

[project.optional-dependencies]
# Use [dependency-groups] instead for dev tools (uv convention)

[dependency-groups]
dev = [
    "pytest>=8",
    "ruff>=0.6",
    "mypy>=1.8",
]
lint = [
    "ruff>=0.6",
    "mypy>=1.8",
]

[project.scripts]
myapp = "myproject.cli:app"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

## Running commands

```bash
# Run in the project's virtual environment
uv run python script.py
uv run pytest
uv run myapp

# Run without installing in the venv first (useful for tools)
uv run --with httpx python -c "import httpx; print(httpx.get('https://httpbin.org/get'))"

# Open a Python shell in the venv
uv run python
```

## Python version management

```bash
# Install a Python version
uv python install 3.12
uv python install 3.11 3.12 3.13

# List available and installed versions
uv python list

# Pin the project to a version (writes .python-version)
uv python pin 3.12

# Use a specific version for a command
uv run --python 3.11 pytest
```

`.python-version` is respected by uv automatically.

## Lock file

`uv.lock` is generated automatically by `uv add`/`uv sync`. Always commit it.

```bash
# Update all dependencies to latest compatible versions
uv lock --upgrade

# Update a specific package
uv lock --upgrade-package httpx

# Check if lock file is up to date (useful in CI)
uv lock --check
```

## Scripts (inline dependencies)

Run a script with its own dependencies without a project:

```bash
uv run script.py
```

```python
# script.py
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "httpx",
#   "rich",
# ]
# ///

import httpx
from rich import print

response = httpx.get("https://api.github.com")
print(response.json())
```

```bash
# Run directly with uv
uv run script.py

# Run from a URL
uv run https://gist.github.com/.../script.py
```

## Tools (global CLI tools)

```bash
# Install a tool globally (isolated env, like pipx)
uv tool install ruff
uv tool install black
uv tool install httpie

# Run a tool without installing permanently
uvx ruff check .
uvx black .
uvx --with git+https://github.com/me/mytool mytool install

# List installed tools
uv tool list

# Upgrade a tool
uv tool upgrade ruff

# Uninstall a tool
uv tool uninstall black
```

## Workspaces (monorepo)

```toml
# Root pyproject.toml
[tool.uv.workspace]
members = ["packages/*", "apps/*"]
```

```
myrepo/
├── pyproject.toml          # workspace root
├── uv.lock                 # single lock file for all packages
├── packages/
│   ├── shared/
│   │   └── pyproject.toml
│   └── models/
│       └── pyproject.toml
└── apps/
    ├── api/
    │   └── pyproject.toml
    └── worker/
        └── pyproject.toml
```

Member packages can depend on each other:

```toml
# apps/api/pyproject.toml
[project]
dependencies = [
    "shared",    # refers to packages/shared in the workspace
    "models",
]

[tool.uv.sources]
shared = { workspace = true }
models = { workspace = true }
```

```bash
# Sync the whole workspace
uv sync

# Run in a specific workspace member
uv run --package api uvicorn api.main:app
```

## Dependency sources

```toml
[tool.uv.sources]
# Git dependency
mylib = { git = "https://github.com/me/mylib", rev = "main" }

# Local path dependency
mylib = { path = "../mylib", editable = true }

# Alternative index
torch = { index = "pytorch" }

[[tool.uv.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cpu"
```

## CI integration

### GitHub Actions

```yaml
- uses: actions/checkout@v4
- uses: astral-sh/setup-uv@v3
  with:
    enable-cache: true         # cache the uv cache dir between runs
    python-version: "3.12"     # installs Python too

- run: uv sync --frozen        # --frozen: fail if lock file is out of date
- run: uv run pytest
- run: uv run ruff check .
```

### GitLab CI

```yaml
test:
  image: python:3.12-slim
  variables:
    UV_CACHE_DIR: "$CI_PROJECT_DIR/.cache/uv"
  cache:
    key: { files: [uv.lock] }
    paths: [.cache/uv/]
  before_script:
    - pip install uv
    - uv sync --frozen
  script:
    - uv run pytest
```

## Docker

```dockerfile
FROM python:3.12-slim

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/

WORKDIR /app

# Copy dependency files first (layer caching)
COPY pyproject.toml uv.lock ./

# Install dependencies only (not the project itself yet)
RUN uv sync --frozen --no-install-project --no-dev

# Copy source
COPY src/ ./src/

# Install the project
RUN uv sync --frozen --no-dev

ENV PATH="/app/.venv/bin:$PATH"
CMD ["python", "-m", "myapp"]
```

## Building and publishing

```bash
# Build sdist and wheel
uv build

# Publish to PyPI
uv publish

# Publish with token
UV_PUBLISH_TOKEN=pypi-... uv publish

# Publish to a private index
uv publish --index-url https://my-registry.example.com/simple/
```

## Migrating from other tools

| Old tool | uv equivalent |
|---|---|
| `pip install -r requirements.txt` | `uv sync` |
| `pip install -e .` | `uv sync` |
| `pip install httpx` | `uv add httpx` |
| `pip freeze > requirements.txt` | `uv export -o requirements.txt` |
| `python -m venv .venv && source .venv/bin/activate` | `uv sync` (venv created automatically) |
| `pyenv install 3.12` | `uv python install 3.12` |
| `poetry add httpx` | `uv add httpx` |
| `pipx install ruff` | `uv tool install ruff` |
| `pipx run ruff` | `uvx ruff` |

## Rules

- Always commit `uv.lock`. It ensures reproducible installs across machines and CI.
- Use `uv sync --frozen` in CI — it fails if the lock file is out of date, catching drift early.
- Use `[dependency-groups]` (not `[project.optional-dependencies]`) for dev tools — this is the uv convention.
- Don't activate the virtual environment manually. Use `uv run` to execute commands in it.
- Use `uvx` to run one-off tools without permanently installing them.
- Pin the uv version in CI (`uses: astral-sh/setup-uv@v3` with `uv-version: "0.5.x"`) to avoid surprise breakage from uv updates.
