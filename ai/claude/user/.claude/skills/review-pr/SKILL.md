---
name: review-pr
description: Use when the user asks to review a pull request. Fetches PR details and diff, then provides a structured code review covering correctness, security, performance, and style.
tools: Bash, Read, Grep, Glob, Agent
---

# Review PR

Perform a thorough, structured code review of a pull request.

## Process

1. **Fetch PR details** - Use `gh pr view <number> --json title,body,author,baseRefName,headRefName,files,additions,deletions` to get PR metadata.
2. **Fetch the diff** - Use `gh pr diff <number>` to get the full diff. For large PRs, also use `gh pr view <number> --json files` to see the file list and prioritize review.
3. **Read surrounding context** - For non-trivial changes, read the full files being modified (not just the diff) to understand the context. Check how changed functions are called, what interfaces are being implemented, etc.
4. **Review systematically** - Analyze the changes across all dimensions below.
5. **Deliver structured feedback** - Present findings organized by severity.

## Review Dimensions

### Correctness
- Does the logic do what the PR description claims?
- Are there edge cases not handled (nulls, empty collections, boundary values, concurrency)?
- Are error paths handled correctly?
- Do new functions have correct return types and values?

### Security
- Input validation at system boundaries (user input, API payloads, URL params)
- SQL injection, XSS, command injection, path traversal
- Secrets or credentials accidentally included
- Permissions and authorization checks
- Dependency vulnerabilities (new deps added)

### Performance
- O(n²) or worse algorithms where O(n) or O(n log n) is possible
- N+1 query patterns
- Missing database indexes for new queries
- Unnecessary allocations in hot paths
- Large payloads or unbounded collections

### Design & Maintainability
- Does the change fit the existing architecture and patterns of the codebase?
- Are abstractions at the right level (not over-engineered, not under-abstracted)?
- Naming clarity — do names communicate intent?
- Single responsibility — does each function/class do one thing?

### Tests
- Are new code paths tested?
- Do tests cover edge cases and error paths?
- Are tests testing behavior (not implementation details)?
- Could existing tests break due to these changes?

### Operational
- Logging for debuggability (especially error paths)
- Configuration changes that could affect deployments
- Database migrations — are they reversible?
- Feature flags or gradual rollout needed?

## Output Format

Organize findings by severity:

### 🔴 Must Fix
Issues that would cause bugs, security vulnerabilities, or data loss.

### 🟡 Should Fix
Issues that hurt maintainability, performance, or deviate from project patterns.

### 🟢 Nitpicks
Style preferences, minor naming suggestions, optional improvements.

### Summary
- One-paragraph overall assessment
- Verdict: **Approve**, **Request Changes**, or **Approve with Suggestions**

## Rules

- Always read surrounding code context, not just the diff in isolation.
- Be specific — reference file names, line numbers, and code snippets.
- Suggest concrete fixes, not just "this could be better."
- Acknowledge what's done well — good PRs deserve recognition.
- If the PR is too large to review effectively, say so and suggest splitting.
- Don't nitpick formatting if a formatter/linter is configured in the project.
- Use `gh pr review <number> --comment --body "..."` only if the user asks to post the review to GitHub.
