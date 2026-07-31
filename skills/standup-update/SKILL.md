---
name: standup-update
description: Use when the user wants a quick standup update — what they did, what they're doing next, and any blockers — pulled from recent git/PR/ticket activity. Bullet points only, ready to paste into a standup channel or thread.
tools: Bash, Read, Grep, Glob
argument-hint: [since <day/date>] [extra context]
---

# Standup Update

Turn recent git, PR, and ticket activity into a three-bullet standup update: **did**, **doing**, **blockers**. Ready to paste as-is.

## Arguments

`$ARGUMENTS` — optional. Accepted forms:

| Form | Example | Interpreted as |
|------|---------|----------------|
| Nothing | (none) | Since the last working day (Monday's standup goes back to Friday, not just the weekend) |
| Explicit window | `since monday`, `since friday` | Start of the window |
| Extra context | `since friday, skip the pyspark-debug tangent` | Steering appended after the window |

## Process

### 1. Determine the window

Default to "since the last working day." Confirm the day-of-week math with `date` if it's ambiguous (e.g. running this on a Monday).

### 2. Gather what got done

```bash
git log --author="$(git config user.name)" --since="<window>" --oneline --no-merges

gh pr list --author @me --state all --search "updated:>=<window-date>" --json number,title,state,url
glab mr list --author=@me --updated-after=<window-date>

jira issue list --assignee "$(jira me)" --updated ">=<window-date>"
```

In a multi-repo workspace check siblings (`ls ..`) for other repos with recent commits by the user — standups rarely stay inside one repo.

### 3. Gather what's next

```bash
gh pr list --author @me --state open --json number,title,url,isDraft
gh issue list --assignee @me --state open

jira issue list --assignee "$(jira me)" --status "In Progress,To Do"
```

### 4. Gather blockers

- PRs open more than ~2 business days with no review activity.
- CI failing on any of the user's open PRs (`gh pr checks <n>`).
- Tickets labeled `blocked`, or a question in a thread that's gone unanswered.
- Anything the user named explicitly in the extra-context argument.

### 5. Write the update

## Output Format

```markdown
**Did:**
- <one line per completed thing — PR merged, ticket closed, investigation finished>

**Doing:**
- <one line per thing in flight — link the PR/ticket>

**Blockers:**
- <one line per blocker, or "None">
```

## Rules

- Bullets only — no paragraphs, no restating the obvious ("worked on tickets").
- One line per item; link the PR/ticket instead of describing it in prose.
- "None" is a valid, correct answer for Blockers — never invent one to fill space.
- Only report what git/the tracker actually shows — don't guess at WIP that isn't reflected anywhere.
- Skip a source cleanly if its CLI isn't installed/authenticated (e.g. no `jira` on this machine) — note it was skipped, don't fail the whole update.
- Don't post anywhere — this is output for the user to paste themselves.
