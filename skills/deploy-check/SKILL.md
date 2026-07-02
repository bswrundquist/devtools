---
name: deploy-check
description: Use when the user wants to verify readiness before deploying. Runs a pre-deployment checklist covering git state, CI status, Docker builds, environment config, migrations, and health endpoints.
tools: Bash, Read, Grep, Glob, Agent
---

# Deploy Check

Run a comprehensive pre-deployment checklist to verify everything is ready to ship.

## Process

Run all checks, report results as pass/fail/warn, and give a final go/no-go verdict.

## Checklist

### 1. Git State
```bash
# Clean working tree (no uncommitted changes)
git status --porcelain

# On expected branch
git branch --show-current

# Up to date with remote
git fetch origin && git log HEAD..origin/$(git branch --show-current) --oneline

# All commits pushed
git log origin/$(git branch --show-current)..HEAD --oneline
```
- ✅ Working tree is clean
- ✅ On the correct branch (main/release branch)
- ✅ Up to date with remote (no unpulled commits)
- ✅ All local commits pushed

### 2. CI/CD Status
```bash
# Check latest CI run for current branch
gh run list --branch $(git branch --show-current) --limit 1 --json status,conclusion,name

# Or check specific PR
gh pr checks <number>
```
- ✅ Latest CI run passed
- ✅ All required checks green
- ⚠️ Flag any skipped or pending checks

### 3. Tests
```bash
# Run test suite
pytest --tb=short -q 2>/dev/null || python -m pytest --tb=short -q 2>/dev/null

# Or check if tests pass in CI (if running locally is slow)
```
- ✅ All tests pass
- ⚠️ Note any skipped tests

### 4. Docker Build
```bash
# Verify Docker image builds successfully
docker build -t deploy-check-test . 2>&1 | tail -5

# Check image size
docker images deploy-check-test --format '{{.Size}}'

# Clean up
docker rmi deploy-check-test 2>/dev/null
```
- ✅ Docker image builds successfully
- ⚠️ Flag unusually large images

### 5. Environment & Configuration
```bash
# Check for required env vars referenced in code
grep -rh 'os\.environ\|os\.getenv\|config\.' --include='*.py' . | grep -oP '["'"'"']\K[A-Z_]{3,}(?=["'"'"'])' | sort -u
```
- ✅ All required environment variables documented
- ✅ No secrets hardcoded in source
- ✅ `.env` is in `.gitignore`
- ⚠️ Flag any `.env.example` that's out of date vs. actual env var usage

### 6. Dependencies
```bash
# Check for lock file freshness
# Python
ls requirements.txt pyproject.toml uv.lock poetry.lock 2>/dev/null

# Verify deps install cleanly
pip install -r requirements.txt --dry-run 2>/dev/null
```
- ✅ Lock file exists and is committed
- ✅ Dependencies resolve without conflicts

### 7. Database Migrations
```bash
# Check for pending migrations (framework-specific)
# Alembic
alembic heads 2>/dev/null
alembic current 2>/dev/null

# Django
python manage.py showmigrations --plan 2>/dev/null
```
- ✅ No pending migrations
- ✅ Migration files committed
- ⚠️ Flag destructive migrations (DROP TABLE, DROP COLUMN)

### 8. Infrastructure
```bash
# Pulumi preview (if applicable)
cd infra && pulumi preview --diff 2>/dev/null

# Check Pulumi stack config
pulumi config 2>/dev/null
```
- ✅ Infrastructure changes previewed
- ⚠️ Flag any resource deletions or replacements

### 9. API / Health Check
```bash
# If there's a local server running or staging URL
curl -sf http://localhost:8000/health 2>/dev/null
curl -sf http://localhost:8000/v1/models 2>/dev/null
```
- ✅ Health endpoint responds
- ⚠️ Skip if no local server running (note it)

## Output Format

```
## Deploy Readiness Check

| Check              | Status | Details                          |
|--------------------|--------|----------------------------------|
| Git State          | ✅     | Clean, on main, pushed           |
| CI/CD              | ✅     | All checks passed                |
| Tests              | ✅     | 42 passed, 0 failed              |
| Docker Build       | ✅     | Built in 45s, 1.2GB              |
| Environment        | ⚠️     | Missing NEW_VAR in .env.example  |
| Dependencies       | ✅     | All resolved                     |
| Migrations         | ➖     | No migration framework detected  |
| Infrastructure     | ✅     | No changes                       |
| Health Check       | ➖     | No local server running          |

### Verdict: ✅ READY TO DEPLOY
(or ❌ NOT READY — fix N issues first)
```

## Rules

- Run checks in parallel where possible for speed.
- Use ➖ for checks that don't apply (no database, no Docker, etc.) — don't fail on them.
- If a check can't be run (tool not installed, no permissions), mark it ⚠️ with explanation.
- Never run `pulumi up`, `alembic upgrade`, or any destructive command — only preview/check commands.
- Don't run Docker build if it would take more than 2 minutes — check if a recent image exists instead.
- For large test suites, check CI results instead of running locally.
- Be explicit about what's blocking deployment vs. what's advisory.
