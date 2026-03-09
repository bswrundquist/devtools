---
name: changelog
description: Use when the user wants to generate a changelog from git history. Groups commits by type, links PRs, highlights breaking changes, and produces clean markdown between any two refs (tags, branches, SHAs).
tools: Bash, Read, Grep
---

# Changelog

Generate a structured changelog from git history.

## Process

1. **Determine range** - Ask for or infer the range:
   - Between two tags: `git log v1.0.0..v1.1.0`
   - Since last tag: `git log $(git describe --tags --abbrev=0)..HEAD`
   - Between branches: `git log main..HEAD`
   - If no tags exist, use all commits or ask for a SHA range.
2. **Gather commits** - Run `git log --oneline --no-merges <range>` to get the commit list.
3. **Gather PRs** - Run `gh pr list --state merged --base main --json number,title,labels,mergedAt` to supplement with PR data if available.
4. **Parse and categorize** - Group commits by conventional commit type. Extract scopes, breaking changes, and PR references.
5. **Generate changelog** - Produce clean markdown output.

## Categorization

Map conventional commit types to changelog sections:

| Commit Type | Changelog Section |
|-------------|------------------|
| `feat` | **Features** |
| `fix` | **Bug Fixes** |
| `perf` | **Performance** |
| `refactor` | **Refactoring** |
| `docs` | **Documentation** |
| `test` | **Tests** |
| `ci` | **CI/CD** |
| `chore` | **Maintenance** |
| `style` | **Style** |
| Breaking (`!`) | **Breaking Changes** (top of changelog) |

## Output Format

```markdown
# Changelog

## [v1.2.0] - 2025-01-15

### Breaking Changes
- **api**: Response format changed from XML to JSON (#42)

### Features
- **auth**: Add OAuth2 login with Google and GitHub (#38)
- **search**: Full-text search with PostgreSQL tsvector (#35)

### Bug Fixes
- **payments**: Fix duplicate charge on retry (#41)
- Fix session expiry not respecting timezone (#39)

### Performance
- **db**: Add composite index for user lookup queries (#40)

### Maintenance
- Upgrade FastAPI to 0.110 (#37)
- Remove deprecated v1 endpoints (#36)
```

## Rules

- Group by type, then sort by scope (scoped entries before unscoped).
- Put **Breaking Changes** at the top, always.
- Include PR numbers as `(#N)` links when available.
- Omit empty sections — don't show "### Tests" if there are no test commits.
- For non-conventional commits (no type prefix), try to infer the type from the message. If unclear, put them under **Other Changes**.
- Merge commits should be excluded (`--no-merges`).
- If a commit message references a PR or issue number, include it.
- Keep descriptions concise — use the commit's first line, not the body.
- When the user doesn't specify a range, default to changes since the last tag. If no tags exist, ask.
