---
name: readme-sync
description: Use when loose markdown files have accumulated in the repo — notes, plans, analysis docs, summaries Claude left behind. Finds them, distills the relevant content, folds it into README.md cleanly, and removes the absorbed files.
tools: Glob, Read, Bash, Write, Edit
---

# README Sync

Find loose markdown files Claude has accumulated, extract what's still relevant, and consolidate it into a clean README.md. Delete the absorbed files.

## Process

1. **Inventory** - Find all `.md` files in the repo. Exclude intentional docs (README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE.md, files inside `docs/`, `node_modules/`, `.git/`).
2. **Read README** - Read the current README.md to understand existing structure and content.
3. **Read loose files** - Read every candidate file fully. Note: file name, what it contains, when it seems to have been written, and whether the content is still relevant.
4. **Distill** - For each loose file, decide:
   - **Absorb**: content is useful and not already in README → extract key facts
   - **Discard**: content is stale, redundant, obvious, or no longer true → skip it
   - **Skip**: file is clearly intentional (e.g. a skill, a spec, a template) → leave it alone
5. **Rewrite README** - Incorporate the absorbed content. Keep the README concise: every sentence should earn its place. Prefer bullet points and short sections over prose. Do not pad.
6. **Delete absorbed files** - Remove the loose markdown files that were absorbed or discarded. Leave intentional docs untouched.
7. **Report** - List what was absorbed, what was discarded, and what was left alone — with one-line reasons.

## What counts as a "loose" file

Claude tends to leave behind files like:
- `NOTES.md`, `PLAN.md`, `PLANNING.md`, `TODO.md`
- `ANALYSIS.md`, `SUMMARY.md`, `ARCHITECTURE.md`, `DESIGN.md`
- `CONTEXT.md`, `SCRATCH.md`, `RESEARCH.md`, `FINDINGS.md`
- Anything with a date or session stamp in the name
- Any `.md` file in the repo root that isn't a standard doc

## What to absorb vs discard

**Absorb** if the content:
- Explains the project's purpose, architecture, or key design decisions
- Describes how to install, run, or use the project
- Documents non-obvious conventions, constraints, or trade-offs
- Lists commands, environment variables, or config that a developer would need

**Discard** if the content:
- Is a task list for work that's already done
- Describes a plan that was already executed
- Duplicates what's already in README or code comments
- Is speculative ("we might want to...") with no clear decision
- Is a scratchpad or exploration that led nowhere

## README structure

After absorbing, the README should follow this order (include only sections that have real content):

1. **Project name + one-line description** (heading + tagline)
2. **What it does** (2–5 bullets or a short paragraph)
3. **Install / Quickstart** (the fastest path to using it)
4. **Usage** (commands, options, examples — just the essential ones)
5. **Architecture / How it works** (only if non-obvious; keep it short)
6. **Configuration** (env vars, config files — if any)
7. **Development** (how to run tests, contribute)

Omit sections that would be empty or filler. Don't add sections just because they're conventional.

## Rules

- Write every line of the README as if a new engineer needs it on day one — cut anything they'd figure out in 30 seconds.
- Never invent information. Only write what you can confirm from the absorbed files or the existing README.
- Preserve any content in the current README that isn't covered by the absorbed files.
- Do not delete files outside the repo root or `docs/` unless they are clearly session artifacts.
- If unsure whether a file is intentional, leave it and mention it in the report.
- After writing the README, delete the absorbed files — do not leave them behind.
