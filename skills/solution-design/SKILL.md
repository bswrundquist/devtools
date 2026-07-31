---
name: solution-design
description: Use when the user wants a thorough solution design from an issue, epic, or description — produces a self-contained HTML design document with work breakdown, knowledge-transfer plan, open questions, and a final overview table. Opens the file when done.
tools: Bash, Read, Grep, Glob, Write, Agent, WebFetch
argument-hint: <issue ref, URL, or description> [--out <path>]
---

# Solution Design

Produce a full solution design — the thorough sibling of `/issue-summary`. The deliverable is a single self-contained HTML file: what will be done, in which repos, how it will be validated, what knowledge transfer is needed, and which details are still missing. Opens in the browser when done.

## Arguments

`$ARGUMENTS` — an issue reference (same forms as `/issue-summary`: URL, `#42`, `acme/api#42`, `PLAT-123`) or a free-text description of the work.

- `--out <path>` — write the HTML to this path instead of the default location.

## Process

### 1. Gather inputs

- Fetch the issue and its comments (`gh issue view`, `glab issue view`, `jira issue view` — see `/issue-summary` for the exact commands).
- **Read the actual code** in the affected areas — entry points, current behavior, existing tests. A design written without reading the code is fiction.

```bash
git remote get-url origin 2>/dev/null
git log --oneline -10 -- src/affected/area/     # recent churn in the target area
grep -rn "EntryPoint" --include="*.py" -l | head
```

- For large or multi-repo scans, fan out `Explore` agents rather than reading everything inline.

### 2. Design

Decide the approach. Note at least one alternative considered and why it lost. Break the work into items with dependencies — these become the overview table rows.

### 3. Write the document

Required sections, in this order:

1. **Executive summary** — the `/issue-summary` shape: 1–2 paragraphs, repos, key persons.
2. **Background & current state** — how it works today, with `file:line` references you actually read.
3. **Proposed solution** — architecture, data flow, alternatives considered.
4. **Work breakdown by repo** — per repo: what changes, new/modified files, interfaces.
5. **Testing & validation** — how each part is proven correct; what "done" means.
6. **Rollout & sequencing** — order, migrations, feature flags, reversibility.
7. **Knowledge transfer** — who needs to learn what; the artifact for each (doc, ADR, demo, pairing session, runbook); when it happens.
8. **Open questions & missing details** — each entry: what's missing, why it matters, and *who or what can close the gap*.
9. **Overview table** — always last:

| # | Work item | Repo | Type | Size | Depends on | Suggested owner | Validated by |
|---|-----------|------|------|------|------------|-----------------|--------------|

Types: `feat` / `refactor` / `infra` / `docs` / `test`. Sizes: S / M / L — never hours.

### 4. Render, write, open

Self-contained HTML: inline `<style>`, no external assets. Skeleton:

```html
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Solution Design: {title}</title>
<style>
  body { font: 16px/1.6 -apple-system, system-ui, sans-serif; color: #1a1a1a;
         max-width: 900px; margin: 2rem auto; padding: 0 1.5rem; }
  h1 { border-bottom: 3px solid #2563eb; padding-bottom: .4rem; }
  h2 { margin-top: 2.2rem; border-bottom: 1px solid #e5e7eb; padding-bottom: .3rem; }
  code { background: #f3f4f6; padding: .1em .35em; border-radius: 4px; font-size: .9em; }
  table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
  th, td { border: 1px solid #d1d5db; padding: .5rem .7rem; text-align: left; vertical-align: top; }
  th { background: #f9fafb; }
  .meta { color: #6b7280; font-size: .9rem; }
  .badge { display: inline-block; padding: .1em .6em; border-radius: 999px; font-size: .8em; font-weight: 600; }
  .badge.s { background: #dcfce7; } .badge.m { background: #fef9c3; } .badge.l { background: #fee2e2; }
  .open-q { background: #fffbeb; border-left: 4px solid #f59e0b; padding: .6rem 1rem; margin: .6rem 0; }
</style>
</head>
<body>
<h1>Solution Design: {title}</h1>
<p class="meta">{date} · source: <a href="{issue_url}">{issue_ref}</a> · status: {Draft|Ready}</p>
<!-- sections 1–9 -->
</body>
</html>
```

Default output path:

```bash
DATE=$(date +%Y-%m-%d)
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  mkdir -p docs/designs
  FILE="docs/designs/${DATE}-<slug>.html"
else
  FILE="${DATE}-<slug>-design.html"
fi
```

Write the file, then open it:

```bash
open "$FILE"
```

## Rules

- Every claim about current behavior must be backed by a file you actually read — cite `file:line` in the Background section.
- The overview table must cover 100% of the work described in prose; if it's not a row, it's not in the plan.
- Open questions are first-class output, not an apology — a design that surfaces the right five gaps is better than one that papers over them.
- If an open question could change the fundamental approach, mark the document **Draft** in the header and say which question gates it.
- Knowledge transfer is about people: name the audience per item, not just "write docs".
- Sizes are S/M/L relative to the team's normal PR — never estimate hours.
- Finish by opening the file and summarizing in chat: work-item count, biggest risk, and the open questions that need answers first. Offer `/implement` as the next step.
