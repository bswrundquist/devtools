---
name: diff-review
description: Use when the user wants to review their uncommitted or staged changes before committing. A lightweight pre-commit code review that catches issues early — before they become PR comments.
tools: Bash, Read, Grep, Glob
---

# Diff Review

Review local uncommitted changes for issues before they get committed.

## Process

1. **Gather changes** - Run `git diff` (unstaged) and `git diff --cached` (staged) to see all local modifications. Run `git status` to see new/deleted files.
2. **Read context** - For each changed file, read the full file (not just the diff) to understand the surrounding code. Check how changed functions are used elsewhere.
3. **Review systematically** - Analyze changes across all dimensions below.
4. **Report findings** - Present findings organized by severity, with specific file:line references and suggested fixes.

## Review Dimensions

### Correctness
- Logic errors, off-by-one, wrong operator, inverted condition
- Unhandled edge cases (None, empty, zero, negative, boundary values)
- Error paths that swallow exceptions or return wrong types
- Type mismatches between function signatures and call sites
- Missing awaits on async functions

### Security
- Hardcoded secrets, API keys, passwords, tokens
- User input flowing into SQL, shell commands, file paths, or HTML without sanitization
- New dependencies with known vulnerabilities
- Overly permissive file permissions or CORS settings

### Bugs Waiting to Happen
- Mutable default arguments (`def f(x=[])`)
- Variable shadowing that changes behavior
- Resources opened but not closed (files, connections, cursors)
- Race conditions in concurrent code
- Imports that will fail at runtime (circular, missing package)

### Style & Consistency
- Does the change match the patterns used in the rest of the codebase?
- Naming inconsistencies (mixing camelCase and snake_case, unclear abbreviations)
- Dead code being added (unreachable branches, unused variables/imports)
- Comments that contradict the code

### Completeness
- Missing tests for new behavior
- Missing error handling at system boundaries
- Missing migration for schema changes
- Updated function signature but not all call sites

## Output Format

### Issues Found

For each issue:
```
**[SEVERITY]** file.py:42 — Brief description
> code snippet showing the problem
Suggested fix: concrete recommendation
```

Severities: 🔴 **Bug/Security** | 🟡 **Should Fix** | 🟢 **Nitpick**

### Summary
- Total issues by severity
- Overall assessment: **Clean to commit**, **Fix before committing**, or **Needs rework**
- If clean: say so briefly and don't pad with unnecessary praise

## Rules

- Be concrete — always reference file:line and show the problematic code.
- Suggest fixes, not just problems.
- Don't flag style issues if a formatter (black, ruff) is configured.
- Don't flag missing tests for trivial changes (config, docs, comments).
- If no issues found, say "Changes look clean" — don't invent problems.
- Focus on the diff, but read surrounding code for context.
- This is a quick review, not an audit. Prioritize likely bugs over theoretical concerns.
