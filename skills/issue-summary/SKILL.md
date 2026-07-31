---
name: issue-summary
description: Use when the user wants a concise executive summary of an issue, epic, or ticket (GitHub, GitLab, or Jira) — what needs to be done, which repos are affected, and who the key persons are. One to two paragraphs written for stakeholders.
tools: Bash, Read, Grep, Glob, WebFetch
argument-hint: <issue ref, URL, or pasted text> [extra context]
---

# Executive Summary

Turn an issue, epic, or ticket into a 1–2 paragraph executive summary: what needs to be done, which repos will be affected, and who the key persons are. Written for someone who decides, not someone who implements.

## Arguments

`$ARGUMENTS` — the issue reference plus optional extra context. Accepted forms:

| Form | Example | Interpreted as |
|------|---------|----------------|
| URL | `https://github.com/acme/api/issues/42` | Fetch from that platform |
| Number | `42` or `#42` | Issue in the current repo (platform from `git remote`) |
| Cross-repo | `acme/api#42` | GitHub issue in another repo |
| Jira key | `PLAT-123` | Jira issue (needs `jira` CLI) |
| Free text | a pasted issue body | Use as-is, skip fetching |

Anything after the ref is steering context (e.g. `42 focus on the data-migration part`).

## Process

### 1. Fetch the issue

Detect the platform from the ref itself, or from the current repo:

```bash
git remote get-url origin 2>/dev/null   # github.com → gh, gitlab → glab
```

```bash
# GitHub
gh issue view 42 --repo acme/api --json title,body,author,assignees,labels,milestone,url,comments

# GitLab
glab issue view 42 --comments

# Jira (ankitpokhrel/jira-cli)
jira issue view PLAT-123 --comments 20
```

If the CLI is missing or unauthenticated, ask the user to paste the issue body — don't reconstruct it from guesses.

### 2. Identify affected repos

- Repo links, paths, and service names mentioned in the body and comments.
- In a multi-repo workspace, check siblings: `ls ..`
- When the repos are local, confirm the named component actually lives there:

```bash
grep -rln "payment_webhook" --include="*.py" | head
```

### 3. Identify key persons

- Issue author, assignees, and anyone whose comment sets direction or constraints.
- Owners of the affected code:

```bash
cat .github/CODEOWNERS CODEOWNERS 2>/dev/null
git shortlog -sn --since="6 months ago" -- src/payments/ | head -5
```

### 4. Write the summary

## Output Format

```markdown
## Executive Summary: <issue title> (<ref>)

<Paragraph 1: the problem or goal in plain language — why it matters and what "done" looks like.>

<Paragraph 2 (optional): the shape of the work — what changes where, notable risks or
dependencies, rough sequencing.>

**Repos affected:** `acme/api` (webhook handler), `acme/worker` (retry queue)
**Key persons:** Jane Doe (author, product owner) · Sam Lee (assignee) · Priya K (owns `src/payments/`, top committer)
**Source:** <link>
```

## Rules

- Two paragraphs maximum. If it genuinely doesn't fit, it's a design doc, not an executive summary — say so and offer `/solution-design`.
- Write for a stakeholder: outcomes and impact, not function names or line counts.
- Name repos exactly as they exist — never invent repo names from prose.
- Every key person gets a parenthetical *why they matter*.
- State unknowns as unknowns ("the consumer-side repo is not specified in the issue"); never fill gaps with plausible guesses.
- Read the comments, not just the body — scope often changes in the thread.
- Don't post anything back to the tracker.
