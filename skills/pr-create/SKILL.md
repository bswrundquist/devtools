---
name: pr-create
description: Use when the user wants to open a pull request (GitHub) or merge request (GitLab) from the current branch. Produces a concise, engineer-written PR — an objective, sections shaped by what actually changed, an acceptance criteria checklist, and links to anything involved. Tables and lists over prose.
tools: Bash, Read, Grep, Glob
argument-hint: [--base <branch>] [--draft] [--reviewer <user>] [--create]
---

# Create PR

Open a pull request (GitHub) or merge request (GitLab) for the current branch. The output should read like a competent engineer wrote it in three minutes flat: an objective, a handful of sections shaped by what the diff actually contains, a checklist a reviewer can verify against, and links to whatever else is involved. No filler, no restating the diff as prose.

## Arguments

`$ARGUMENTS` — all optional.

- `--base <branch>` — target branch. Detected automatically if omitted.
- `--draft` — open as a draft. Also auto-applied if the branch name contains `wip`/`draft`/`poc` or commits look unfinished.
- `--reviewer <user>` — request a reviewer.
- `--create` — actually open it. Without this, the skill only drafts and previews. Opening a PR is visible to the whole team — same standing-approval convention as `/issue-create --create`.

## Process

### 1. Detect platform and preconditions

```bash
git remote get-url origin   # github.com -> gh, gitlab -> glab
git status -sb              # confirm the branch has an upstream — stop and say so if not, point at /push
gh --version || glab --version   # confirm the CLI is installed and authenticated
```

If neither CLI is installed/authenticated, skip straight to drafting and hand the user the finished markdown to paste manually — don't block on tooling.

### 2. Determine the base branch

```bash
git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null || git remote show origin | grep 'HEAD branch'
```

Use `--base` if given. Don't assume `main`.

### 3. Gather everything the PR needs to talk about

```bash
git log <base>..HEAD --oneline              # commits on this branch
git diff <base>...HEAD --stat                # scale and shape of the change
git diff <base>...HEAD                       # full diff — for 500+ line diffs, lean on --stat and commit messages instead of reading every line
```

Read specific changed files when the diff alone doesn't explain intent (a renamed function, a config value whose meaning isn't obvious from the diff context).

### 4. Group the diff into chapters

Don't default to a flat "Changes" bullet list. Group the changed files/commits by concern the same way `/commit` groups commits — by directory, by what they touch (schema, orchestration, config, tests, docs) — and give each group a real heading named for what it *is*, not "Changes 1" / "Changes 2".

| Diff shape | Chapter(s) |
|---|---|
| One concern, one commit, < ~5 files | Skip chapters — Objective, then a single short table/list, then Acceptance Criteria |
| Schema/model change + the code that uses it | Separate chapters: e.g. `Schema`, `Query Changes` |
| New DAG/pipeline + config + tests | `Pipeline`, `Configuration`, `Testing` |
| Refactor + behavior change | `Refactor` chapter first (mechanical, low-risk to review), then the behavior change |

2–5 chapters is normal. More than that usually means the PR itself should have been split — say so instead of forcing a wall of sections.

### 5. Find what's involved

Don't invent links — only include what's actually found:

```bash
git log <base>..HEAD --format='%s%n%b' | grep -oE '#[0-9]+|[A-Z]+-[0-9]+'   # issue refs in commit messages
git branch --show-current                                                    # ticket key often lives in the branch name
```

- Issues/tickets referenced in commit messages or the branch name.
- Docs the diff touches or that a commit message points at.
- A base branch that isn't `main`/`master` — this is a stacked PR, say so explicitly and link the parent.
- Design docs from `/solution-design`, if one produced this work.

### 6. Draft acceptance criteria

Pull these from what the diff and tests actually demonstrate, not from wishful thinking:

- New/changed tests → what behavior they lock in.
- A migration/backfill → what confirms it ran clean (row counts, a specific query, a dashboard).
- A DAG/pipeline change → what confirms it's healthy (green run, runtime within X, `/pipeline-status` clean).
- No test coverage for a behavior change → call that out as a criterion instead of skipping it: "Manually verified `<X>` in `<env>`."

Each item must be something a reviewer can check without re-reading the whole diff.

### 7. Preview, then open only on confirmation

Print the full draft (see Output Format). Only run the create command if `--create` was passed or the user confirms in chat.

```bash
# GitHub
gh pr create --title "<title>" --body "<body>" --base <base> [--draft] [--reviewer <user>]

# GitLab — "Merge Request" / "MR" in all output text, not "PR"
glab mr create --title "<title>" --description "<body>" --target-branch <base> --remove-source-branch [--draft] [--assignee <user>]
```

Report the created PR/MR URL back.

## Output Format

```markdown
## Objective
<1-3 sentences: what changed and why, plain language. No "This PR introduces...".>

## <Chapter Name>
| File | Change | Why |
|---|---|---|
| `path/to/file.py` | <what> | <why, if not obvious> |

## <Another Chapter Name>
- <point, when a table doesn't fit the content>

## Acceptance Criteria
- [ ] <concrete, independently checkable>
- [ ] <concrete, independently checkable>

## Related
| Type | Link | Note |
|---|---|---|
| Issue | #123 | closes |
| Doc | `docs/designs/2026-08-01-retry.html` | design this implements |
```

Omit "Related" entirely if nothing was found — don't print an empty table.

### Title

Imperative mood, under 70 characters, states the most important change — not a file list. `Add retry backoff to payment webhook`, not `Update webhook.py`.

## Rules

- Every claim comes from the diff, commits, or something actually read — never describe behavior that isn't confirmed by the code.
- Chapters are named for what they contain (`Schema`, `Backfill`, `Testing`) — never generic (`Changes`, `Updates`, `Misc`).
- Tables for anything with 2+ parallel items (files, links). Plain bullets when a table would be one column wide. Prose only for the Objective.
- Acceptance criteria are checkable by someone who wasn't in this conversation — "tests pass" is not a criterion, "P95 on `/checkout` stays under 200ms under the existing load test" is.
- Write like a person: no em dashes, no "leverage"/"utilize"/"seamless", no restating the diff as a sentence per file, no padding a thin PR to look substantial. A three-file fix gets a three-line PR.
- Never open the PR/MR without `--create` or explicit confirmation in chat.
- Don't push the branch — if it has no upstream, stop and point at `/push`.
- Detect the base branch; don't assume `main`.
- If the CLI is missing or unauthenticated, hand back the finished markdown instead of failing.
