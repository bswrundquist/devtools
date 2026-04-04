---
name: github-actions
description: Use when writing or debugging GitHub Actions workflows — CI/CD pipelines, matrix builds, caching, secrets, reusable workflows, and common patterns for testing and deployment.
tools: Read, Write, Edit, Bash, Glob
---

# GitHub Actions

Write and debug GitHub Actions workflows.

## Workflow structure

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install dependencies
        run: pip install -e ".[dev]"
      - name: Run tests
        run: pytest
```

## Triggers (`on:`)

```yaml
on:
  push:
    branches: [main, "release/**"]
    paths: ["src/**", "tests/**"]   # only run if these paths changed
  pull_request:
    branches: [main]
    types: [opened, synchronize, reopened]
  schedule:
    - cron: "0 8 * * 1"             # every Monday at 8am UTC
  workflow_dispatch:                  # manual trigger via UI or API
    inputs:
      environment:
        description: "Target environment"
        required: true
        default: "staging"
        type: choice
        options: [staging, production]
  workflow_call:                      # callable from other workflows
    inputs:
      version:
        type: string
        required: true
```

## Jobs

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    outputs:
      version: ${{ steps.get-version.outputs.version }}
    steps:
      - id: get-version
        run: echo "version=$(cat VERSION)" >> $GITHUB_OUTPUT

  deploy:
    needs: [build, test]         # wait for both to succeed
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    environment: production      # requires manual approval if configured
    steps:
      - run: echo "Deploying version ${{ needs.build.outputs.version }}"
```

## Matrix builds

```yaml
jobs:
  test:
    strategy:
      fail-fast: false           # don't cancel other jobs if one fails
      matrix:
        python-version: ["3.10", "3.11", "3.12"]
        os: [ubuntu-latest, macos-latest]
        exclude:
          - os: macos-latest
            python-version: "3.10"
        include:
          - os: ubuntu-latest
            python-version: "3.12"
            extra: "--run-slow-tests"
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}
      - run: pytest ${{ matrix.extra }}
```

## Caching

```yaml
- name: Cache uv packages
  uses: actions/cache@v4
  with:
    path: ~/.cache/uv
    key: ${{ runner.os }}-uv-${{ hashFiles('uv.lock') }}
    restore-keys: |
      ${{ runner.os }}-uv-

# For pip
- uses: actions/setup-python@v5
  with:
    python-version: "3.12"
    cache: "pip"               # built-in pip caching

# For uv
- uses: astral-sh/setup-uv@v3
  with:
    enable-cache: true
```

## Secrets and environment variables

```yaml
jobs:
  deploy:
    env:
      # Available to all steps in this job
      NODE_ENV: production
      API_URL: https://api.example.com
    steps:
      - name: Deploy
        env:
          # Step-level env (preferred for secrets)
          DATABASE_URL: ${{ secrets.DATABASE_URL }}
          API_KEY: ${{ secrets.API_KEY }}
        run: ./deploy.sh

      # Set a variable for subsequent steps
      - run: echo "TIMESTAMP=$(date +%s)" >> $GITHUB_ENV
      - run: echo "Timestamp is $TIMESTAMP"
```

Secrets are set in: Repository → Settings → Secrets and variables → Actions.

## Reusable workflows

Define a callable workflow:

```yaml
# .github/workflows/run-tests.yml
on:
  workflow_call:
    inputs:
      python-version:
        type: string
        default: "3.12"
    secrets:
      TEST_DB_URL:
        required: true

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: ${{ inputs.python-version }}
      - run: pytest
        env:
          DATABASE_URL: ${{ secrets.TEST_DB_URL }}
```

Call it from another workflow:

```yaml
jobs:
  test:
    uses: ./.github/workflows/run-tests.yml
    with:
      python-version: "3.12"
    secrets:
      TEST_DB_URL: ${{ secrets.TEST_DB_URL }}
```

## Common patterns

### Lint + test + build

```yaml
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v3
      - run: uv run ruff check .
      - run: uv run ruff format --check .
      - run: uv run mypy .

  test:
    needs: lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v3
        with:
          enable-cache: true
      - run: uv run pytest --cov --cov-report=xml
      - uses: codecov/codecov-action@v4
        with:
          token: ${{ secrets.CODECOV_TOKEN }}
```

### Release on tag

```yaml
on:
  push:
    tags: ["v*"]

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v3
      - run: uv build
      - name: Publish to PyPI
        run: uv publish
        env:
          UV_PUBLISH_TOKEN: ${{ secrets.PYPI_TOKEN }}
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: dist/*
          generate_release_notes: true
```

### Docker build and push

```yaml
jobs:
  docker:
    runs-on: ubuntu-latest
    permissions:
      packages: write
    steps:
      - uses: actions/checkout@v4
      - uses: docker/setup-buildx-action@v3
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - uses: docker/build-push-action@v6
        with:
          push: ${{ github.ref == 'refs/heads/main' }}
          tags: ghcr.io/${{ github.repository }}:${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

### Conditional steps

```yaml
steps:
  - name: Only on main
    if: github.ref == 'refs/heads/main'
    run: ./deploy.sh

  - name: Only on PRs
    if: github.event_name == 'pull_request'
    run: ./preview-deploy.sh

  - name: Only if previous step failed
    if: failure()
    run: ./notify-failure.sh

  - name: Always run (cleanup)
    if: always()
    run: ./cleanup.sh
```

## Permissions

Explicitly set the minimum permissions needed:

```yaml
permissions:
  contents: read       # default — read repo
  pull-requests: write # comment on PRs
  packages: write      # push to GHCR
  id-token: write      # OIDC auth (for cloud providers)
```

Set at workflow level (applies to all jobs) or per-job level.

## Debugging

```yaml
# Enable debug logging for a run
# Set secret: ACTIONS_STEP_DEBUG = true
# Set secret: ACTIONS_RUNNER_DEBUG = true

steps:
  - name: Dump context
    run: |
      echo "Event: ${{ github.event_name }}"
      echo "Ref: ${{ github.ref }}"
      echo "SHA: ${{ github.sha }}"
      echo "Actor: ${{ github.actor }}"

  - name: Dump environment
    run: env | sort
```

## Rules

- Pin action versions to a full SHA for security-critical workflows: `uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683`.
- Use `actions/checkout@v4`, `actions/setup-python@v5` — check for latest major versions.
- Never put secrets directly in `run:` blocks — use `env:` to pass them so they're masked in logs.
- Set `fail-fast: false` on matrix builds when you want to see all failures, not just the first.
- Use `needs:` to express real dependencies between jobs — don't sequence everything if jobs can run in parallel.
- Cache dependencies keyed on the lock file hash — `hashFiles('uv.lock')`, `hashFiles('package-lock.json')`.
- Use `$GITHUB_OUTPUT` (not `set-output`, which is deprecated) for step outputs.
- Use `$GITHUB_ENV` (not `::set-env::`, which is deprecated) for environment variables.
