.PHONY: bootstrap fmt lint test check cli \
        ai-user-dry-run ai-user-install \
        ai-repo-dry-run ai-repo-install

bootstrap:
	uv sync

fmt:
	uv run ruff format .

lint:
	uv run ruff check .

test:
	uv run pytest

check: lint test

cli:
	uv run devtools --help

ai-user-dry-run:
	uv run devtools ai install --user --no-repo --dry-run

ai-user-install:
	uv run devtools ai install --user --no-repo

ai-repo-dry-run:
	uv run devtools ai install --repo --no-user --repo-root . --dry-run

ai-repo-install:
	uv run devtools ai install --repo --no-user --repo-root .
