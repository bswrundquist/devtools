---
name: commit
description: Use when the user asks to commit changes. Uses conventional commits format, describes what was done, and splits into multiple commits when changes are logically distinct.
tools: Bash
---

# Commit

Create well-structured git commits using the Conventional Commits specification.

## Process

1. **Analyze changes** - Run `git status` and `git diff` (staged and unstaged) to understand all changes.
2. **Group logically** - Determine if changes should be split into multiple commits. Split when:
   - Changes touch unrelated areas (e.g., a bug fix and a new feature)
   - There are distinct logical units of work (e.g., refactor + new behavior)
   - Tests were added/changed separately from the code they test (use judgment - if tightly coupled, commit together)
3. **Stage and commit** - For each logical group, stage the relevant files and commit with a proper message.

## Conventional Commits Format

```
<type>(<optional scope>): <description>

[optional body]
```

### Types

- `feat` - A new feature or capability
- `fix` - A bug fix
- `refactor` - Code restructuring without changing behavior
- `docs` - Documentation changes
- `test` - Adding or updating tests
- `chore` - Maintenance tasks (deps, config, build)
- `style` - Formatting, whitespace, semicolons (no logic change)
- `perf` - Performance improvements
- `ci` - CI/CD configuration changes

### Rules

- **Description must explain what was done**, not just name the files. Bad: `feat: update user.py`. Good: `feat(auth): add password reset flow with email verification`.
- Keep the first line under 72 characters.
- Use the body for additional context when the description alone isn't enough.
- Use imperative mood: "add", "fix", "change" (not "added", "fixes", "changed").
- Scope is optional but encouraged when it adds clarity.
- Add `!` after type/scope for breaking changes: `feat(api)!: change response format`.

### Examples

```
feat(payments): add Stripe webhook handler for subscription events

fix: prevent duplicate form submissions on slow connections

refactor(db): extract query builder from repository classes

chore: upgrade FastAPI to 0.110 and update deprecated endpoints

feat(search): add full-text search with PostgreSQL tsvector

fix(auth): handle expired JWT tokens gracefully instead of 500 error

The token validation middleware now catches ExpiredSignatureError
and returns a 401 with a clear message prompting re-authentication.
```

## Multiple Commits

When splitting into multiple commits, commit in logical order (foundations first):

1. Refactors or preparatory changes
2. Core feature/fix implementation
3. Tests
4. Documentation or config updates

Use `git add <specific files>` to stage only the files for each commit. Never use `git add -A` or `git add .` when splitting commits.

## Commit Message via HEREDOC

Always pass commit messages using a HEREDOC to ensure proper formatting:

```bash
git commit -m "$(cat <<'EOF'
feat(scope): description here

Optional body with more detail.
EOF
)"
```
