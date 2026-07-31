---
name: pr-comments-resolve
description: Use when the user wants to collect the review comments on a PR/MR and turn them into a concrete resolution plan — what to fix, what to push back on, what only needs a reply. Planning only; pairs with /implement to apply fixes and /pr-comments-respond to reply.
tools: Bash, Read, Grep, Glob, Agent
argument-hint: <PR/MR number or URL>
---

# Resolve PR Comments

Gather every review comment on a PR/MR and produce a resolution plan. This is the `/solution-design` of code review: the code already exists, so the plan is about closing the review out — fix what's right, push back on what isn't, reply to what only needs an answer.

## Arguments

`$ARGUMENTS` — the PR/MR: a number (`123`, platform from `git remote`), a URL, or nothing (infer the PR for the current branch via `gh pr view` / `glab mr view`).

## Process

### 1. Fetch all comments

**GitHub:**

```bash
gh pr view 123 --json title,url,headRefName,reviews,comments

# Inline review comments with file/line anchors
gh api "repos/{owner}/{repo}/pulls/123/comments" --paginate

# Thread state — unresolved and outdated flags
gh api graphql -f query='
  query($owner: String!, $repo: String!, $pr: Int!) {
    repository(owner: $owner, name: $repo) {
      pullRequest(number: $pr) {
        reviewThreads(first: 100) {
          nodes {
            id isResolved isOutdated path line
            comments(first: 20) { nodes { author { login } body createdAt } }
          }
        }
      }
    }
  }' -f owner=OWNER -f repo=REPO -F pr=123
```

**GitLab:**

```bash
glab mr view 123 --comments
glab api "projects/:id/merge_requests/123/discussions" --paginate
```

### 2. Check each comment against current reality

Comments go stale. For each one, read the file at the referenced location and check whether commits since the comment already address it:

```bash
git log --oneline --since="<comment date>" -- path/to/file.py
```

### 3. Classify

| Class | Meaning |
|-------|---------|
| 🔴 must-fix | Real bug, security issue, or broken behavior |
| 🟠 improve | Valid design/style point worth doing now |
| 💬 question | Needs an answer, not a code change |
| 🤝 push back | Reviewer's suggestion would make things worse — disagree with evidence |
| 📦 follow-up | Valid but out of scope — becomes a new issue |
| ✅ already addressed | Fixed by a later commit or outdated by a rebase |

### 4. Plan

Group related comments into single work items (three comments about the same missing validation = one fix). Order by dependency, then severity.

## Output Format

```markdown
## Comment Resolution Plan — PR #123 (<title>)

**12 comments in 8 threads** · 5 actionable · 4 reply-only · 3 already addressed

| # | Comment (author) | Location | Class | Plan |
|---|------------------|----------|-------|------|
| 1 | "retry can double-charge" (@priya) | `api/webhooks.py:88` | 🔴 must-fix | Check idempotency key before charging; add replay test |
| 2 | "why not use the shared client?" (@sam) | `worker/http.py:12` | 💬 question | Reply: shared client pins TLS 1.2, this endpoint needs 1.3 |
| 3 | "rename to Manager" (@sam) | `api/queue.py:30` | 🤝 push back | Keep `Dispatcher` — matches the 6 existing `*Dispatcher` classes |

### Work items
Grouped, ordered changes where multiple comments collapse into one fix.

### Needs your call
Pushback and out-of-scope items to confirm with the user before anyone replies.
```

End by offering the handoffs: `/implement` to apply the fixes, `/pr-comments-respond` to draft and post the replies.

## Rules

- **Read-only.** Never post replies, resolve threads, or change code — this skill produces the plan.
- Blind compliance is not a plan: if the reviewer is wrong, the plan is a respectful pushback with evidence (existing conventions, a benchmark, a counterexample), not a silent code change.
- Every actionable row says how the fix will be validated, even if that's just "existing test X now covers it".
- Check `isOutdated` / later commits before planning work — don't plan fixes for things already fixed.
- Reply-only items get the intended answer sketched in the plan, so `/pr-comments-respond` isn't starting cold.
- If a comment is ambiguous, the plan for it is "clarify with <author>", not a guess.
