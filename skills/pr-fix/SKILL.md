---
name: pr-fix
description: Use when the user wants to implement fixes for a PR review or PR review comments, not just plan them. Applies must-fix issues by default (should-fix and nitpicks on request), loops re-reviewing after each pass until the diff is actually clean, and replies to fixed comments with the commit sha once the user approves.
tools: Bash, Read, Edit, Write, Grep, Glob, Agent
argument-hint: <PR/MR number or URL, or nothing to use findings already in this conversation> [--should-fix] [--nitpicks] [--all]
---

# Fix PR

Take findings from a PR review or comments on a PR and actually fix them. This is the doing half of `/pr-review` and `/pr-comments-resolve`: those two produce a list of problems, this skill closes it out with real code changes, verified, not just marked done.

## Arguments

`$ARGUMENTS`

- A PR/MR number or URL: fetch the diff and review comments fresh.
- Nothing: use findings already produced in this conversation by `/pr-review`, `/pr-review-quick`, `/diff-review`, `/diff-review-quick`, or `/pr-comments-resolve`.
- `--should-fix`: also fix 🟡 should-fix issues.
- `--nitpicks`: also fix 🟢 nitpicks.
- `--all`: fix everything regardless of severity.

Default scope is 🔴 must-fix only. Say so up front so the user knows what's in and out of scope before anything gets touched.

## Process

### 1. Gather the findings

If findings already exist in this conversation, use them as written. Do not re-derive or soften them, the review already did that work.

If not, fetch fresh:

```bash
gh pr view <number> --json title,body,headRefName,files
gh pr diff <number>
gh api "repos/{owner}/{repo}/pulls/{number}/comments" --paginate
gh api graphql -f query='
  query($owner: String!, $repo: String!, $pr: Int!) {
    repository(owner: $owner, name: $repo) {
      pullRequest(number: $pr) {
        reviewThreads(first: 100) {
          nodes { id isResolved isOutdated path line comments(first: 20) { nodes { id author { login } body } } }
        }
      }
    }
  }' -f owner=OWNER -f repo=REPO -F pr=<number>
```

GitLab: `glab mr view`, `glab mr diff`, `glab api projects/:id/merge_requests/<number>/discussions`.

Run the actual review dimensions from `/pr-review` (or `/diff-review` for uncommitted work) against the diff. Classify PR comments the same way `/pr-comments-resolve` does: 🔴 must-fix, 🟠 improve, 💬 question, 🤝 push back, 📦 follow-up, ✅ already addressed. Only 🔴 must-fix and 🟠 improve items are candidates for a code fix. Questions, pushback, and follow-ups are out of scope here, leave those for `/pr-comments-respond` or for the user to answer directly.

### 2. Build the fix queue

One entry per issue, review findings and comments merged into a single list. Collapse duplicates, three comments about the same missing null check is one queue entry. Note which source each entry came from (a review finding, or a specific comment/thread id) so a reply can be traced back to it later.

Filter to the requested severity scope. Tell the user what's in scope and what's being skipped, with counts, before touching any code.

### 3. Fix and verify loop

This is the part that has to actually work, not just look done. Repeat until clean:

1. Apply every queued fix for this pass.
2. Run the project's tests, lint, and type checks if it has them (same detection as `/implement`: Makefile targets first, then `uv run` / `npm run` directly).
3. Re-review the changed files against the review dimensions, focused on two questions: is each targeted issue actually gone, not just touched, and did any fix introduce a new must-fix issue such as a regression, a broken test, or a new bug.
4. If step 3 finds nothing new and tests, lint, and types are all green, stop. The loop is done.
5. Otherwise, queue whatever's left, the unresolved originals plus anything newly introduced, and go again.
6. After 4 passes without reaching clean, stop and report exactly what's still failing and why. Don't keep guessing.

Do not mark an issue fixed because code was written for it. Mark it fixed because a re-review pass looked at the result and confirmed the failure mode is actually closed.

### 4. Commit

Use `/commit` conventions: Conventional Commits, logical grouping, imperative mood. Group related fixes into one commit, keep unrelated fixes in separate commits the same way `/commit` would split them. This matters even if the user doesn't normally want auto-commits, a reply that cites "fixed in `<sha>`" needs a real commit to point at.

### 5. Reply to comments (only if the source included PR comments, and only with approval)

For every comment whose issue got fixed, draft a reply in the style of `/pr-comments-respond`:

> Fixed in `<sha>`: one line on what changed.

Show every draft next to the comment it answers, along with the "in scope but not fixed" and "out of scope" lists, so the user sees the whole picture, not just the wins. Ask before posting anything.

If approved, post the same way `/pr-comments-respond` does:

```bash
# GitHub: reply inside the thread
gh api "repos/{owner}/{repo}/pulls/{number}/comments/{comment_id}/replies" -f body='Fixed in `<sha>`: ...'

# GitLab: reply inside the discussion
glab api "projects/:id/merge_requests/<number>/discussions/<discussion_id>/notes" -f body='Fixed in `<sha>`: ...'
```

Resolve a thread only if that fix is genuinely complete and the team's convention is author-resolves. When unsure, reply and leave it open for the reviewer.

### 6. Report

```markdown
## Fix Report - PR #123

**Scope:** must-fix only (3 flagged, 3 fixed) · should-fix and nitpicks skipped (2, 4)

| # | Issue | Source | Status | Commit | Verified by |
|---|-------|--------|--------|--------|--------------|
| 1 | retry can double-charge | pr-review | fixed | `a1b2c3d` | new test `test_retry_idempotent`, 2 loop passes |
| 2 | missing null check on webhook payload | comment (@priya) | fixed | `a1b2c3d` | existing test now covers it |
| 3 | timeout hardcoded | comment (@sam) | blocked | none | needs a decision on the default value, see comment |

**Tests:** 42 passed · **Lint:** clean · **Types:** clean
**Comment replies:** 2 drafted, posted after approval
```

## Rules

- Must-fix only by default. Don't touch should-fix or nitpick items unless asked, even if they're trivial.
- Never invent or reinterpret a finding. If a review comment is genuinely wrong, say so and leave it for the user. Don't silently skip it, and don't silently comply if the fix would make things worse.
- The verify loop is not optional and not a formality. An issue isn't done until a re-review pass confirms it, and a fix isn't safe until tests, lint, and types are green again.
- Cap at 4 passes. If still not clean, report the exact remaining failure. Don't fake a green result.
- Commit in logical groups, not one giant commit for every fix.
- Never cite a commit that isn't actually on the remote branch. Push before replying if the branch isn't up to date, and say so.
- Never post a reply or resolve a thread without explicit approval, even if the fixes themselves were pre-approved.
- Write replies and reports the way a person would: plain, specific, no filler, no em dashes.
- If a fix needs to touch code well outside the flagged lines, explain why before doing it.
