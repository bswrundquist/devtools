---
name: respond-pr-comments
description: Use when the user wants to reply to PR/MR review comments — drafts concise responses that fully answer each comment and let reviewers verify what changed, then posts them only after approval.
tools: Bash, Read, Grep, Glob
argument-hint: <PR/MR number or URL> [--post]
---

# Respond to PR Comments

Draft and post replies to review comments. Each reply is concise, fully answers the question at hand, and gives the reviewer something concrete to verify — a commit, a line, a reason.

## Arguments

`$ARGUMENTS` — the PR/MR: a number, a URL, or nothing (infer from the current branch).

- `--post` — pre-approval to post: still show the drafts, but post without a second confirmation.

## Process

### 1. Fetch unresolved comments

Same commands as `/resolve-pr-comments` — but keep the **thread/discussion and comment IDs**; they're needed for posting.

```bash
# GitHub: inline threads with IDs
gh api "repos/{owner}/{repo}/pulls/123/comments" --paginate
# GitLab: discussions with IDs
glab api "projects/:id/merge_requests/123/discussions" --paginate
```

### 2. Establish the state of each thread

For every comment, determine: fixed (in which commit)? a question to answer? a point to push back on?

```bash
git fetch origin
git log --oneline origin/<pr-branch> -- path/to/file.py | head -5   # what touched this file
git branch -r --contains <sha>    # is the fix actually pushed? never cite an unpushed commit
```

### 3. Draft replies

**Reply styles:**

- **Fixed** — name the commit and what changed, one line:
  > Fixed in `a1b2c3d` — moved the timeout into `RetryPolicy` so it's configurable per caller.
- **Question** — answer directly in ≤3 sentences; link the code or doc that proves it:
  > The shared client pins TLS 1.2 and this endpoint requires 1.3 — see `worker/http.py:12`. Happy to consolidate once the shared client upgrades.
- **Pushback** — acknowledge, reason, offer a path:
  > Agreed the name is generic, but `Dispatcher` matches the six existing `*Dispatcher` classes in this package — renaming just this one would be the inconsistency. Open to a follow-up that renames the family.

**Anti-patterns:** "Done." (nothing to verify) · essays · defensive tone · answering a different question than the one asked.

### 4. Present drafts for review

Show every draft next to the comment it answers. Post nothing yet.

### 5. Post (after approval, or with `--post`)

**GitHub:**

```bash
# Reply inside an inline review thread
gh api "repos/{owner}/{repo}/pulls/123/comments/{comment_id}/replies" -f body='Fixed in `a1b2c3d` — ...'

# Top-level PR comment
gh pr comment 123 --body '...'

# Resolve a thread (thread ID from the reviewThreads GraphQL query)
gh api graphql -f query='
  mutation($id: ID!) { resolveReviewThread(input: {threadId: $id}) { thread { isResolved } } }' -f id=<thread-id>
```

**GitLab:**

```bash
# Reply inside a discussion
glab api "projects/:id/merge_requests/123/discussions/<discussion_id>/notes" -f body='...'

# Resolve a discussion
glab api -X PUT "projects/:id/merge_requests/123/discussions/<discussion_id>?resolved=true"

# Top-level MR note
glab mr note 123 -m "..."
```

## Rules

- **Never post without approval.** Drafts first, always; `--post` from the user is the only standing approval.
- Every "fixed" reply cites a commit that is verifiably on the remote branch — reviewers click through.
- One reply fully answers one thread; if you can't answer it fully, say what's missing instead of deflecting.
- Reply in the thread the comment lives in, not as a top-level comment — reviewers follow threads.
- Only resolve threads whose fix you can point at, and only if the team's convention is author-resolves; when unsure, reply and leave the thread open for the reviewer.
- Match the repo's tone; concise is not curt.
- If `/resolve-pr-comments` produced a plan in this conversation, use its sketched answers — don't re-derive from scratch.
