---
name: readme-sync
description: Use when AI-generated markdown files have accumulated in the repo from previous Claude Code sessions — plans, analyses, summaries, task lists left behind. Distills relevant content into README.md and deletes the AI artifacts. Never touches human-authored docs.
tools: Glob, Read, Bash, Write, Edit
---

# README Sync

Find markdown files left behind by Claude Code during previous sessions, extract anything still worth keeping, fold it into README.md, and delete the AI artifacts. Never touch human-authored documentation.

## Process

1. **Inventory** - Find all `.md` files in the repo. Classify each as AI artifact or human doc (see below). Only process AI artifacts.
2. **Read README** - Read the current README.md to understand existing structure and content.
3. **Read each AI artifact** - Read fully. For each, decide: absorb useful content, or discard entirely.
4. **Rewrite README** - Fold in anything worth keeping. Keep it concise — every sentence earns its place.
5. **Delete AI artifacts** - Remove all files classified as AI artifacts, whether absorbed or discarded.
6. **Report** - List each file: classified as AI artifact or human doc, and what happened to it (absorbed / discarded / left alone).

## Classifying files

### AI artifacts — process these

These are files Claude Code typically creates during a session:

**By name pattern:**
- `NOTES.md`, `PLAN.md`, `PLANNING.md`, `NEXT_STEPS.md`
- `ANALYSIS.md`, `SUMMARY.md`, `FINDINGS.md`, `RESEARCH.md`
- `TASKS.md`, `TODO.md`, `PROGRESS.md`, `STATUS.md`
- `CONTEXT.md`, `SCRATCH.md`, `DESIGN.md`, `ARCHITECTURE.md`
- `IMPLEMENTATION.md`, `PROPOSAL.md`, `REVIEW.md`
- Anything with a date, timestamp, or session ID in the name

**By content pattern:**
- Starts with "I'll", "Let me", "Here's my plan", "Based on my analysis"
- Contains task checklists with `- [ ]` or `- [x]` items
- Reads like an internal monologue or step-by-step reasoning
- Describes work that was going to be done (plans, proposals)
- Contains a "Summary of changes" or "What I did" section

**By location:**
- In the repo root and clearly not a project convention
- In a subdirectory alongside source files but unrelated to them

### Human docs — never touch these

- `README.md` (the target — only ever write to it, never delete)
- `CHANGELOG.md`, `CHANGES.md`, `HISTORY.md`
- `CONTRIBUTING.md`, `CONTRIBUTORS.md`
- `LICENSE.md`, `LICENSE`
- `SECURITY.md`, `CODE_OF_CONDUCT.md`
- Anything inside `docs/`, `wiki/`, `.github/`
- Files that read like they were written for an audience (tutorials, guides, references)
- Any `.md` file that is part of the project's published documentation

**When in doubt, leave it alone.** If a file could plausibly be intentional human documentation, don't touch it — mention it in the report as "unclear, left alone."

## What to absorb vs discard

**Absorb** if the content:
- Explains a non-obvious architectural decision or constraint
- Documents how to install, run, or configure the project
- Lists commands, options, or environment variables a developer would need
- Describes a design trade-off that isn't obvious from the code

**Discard** if the content:
- Is a task list for work that's already been done
- Describes a plan that was already executed
- Is speculative or exploratory without a clear conclusion
- Duplicates what's already in README or the code itself
- Is Claude's internal reasoning (e.g., "I need to first understand X, then...")

## README structure

Incorporate absorbed content into the appropriate section. Maintain this order:

1. **Project name + one-line description**
2. **What it does** (2–5 bullets or a short paragraph)
3. **Install / Quickstart** (fastest path to running it)
4. **Usage** (commands, options, examples — only the essential ones)
5. **Architecture / How it works** (only if non-obvious; keep it short)
6. **Configuration** (env vars, config files — if any)
7. **Development** (how to run tests, contribute)

Omit sections that have nothing real to say. Don't add filler.

## Rules

- **Never delete human docs.** The only files deleted are AI artifacts.
- **Never invent.** Only write what's confirmed by the absorbed files or existing README.
- **Preserve existing README content** that isn't superseded by absorbed material.
- Write every line as if a new engineer needs it on day one.
- If a file is ambiguous, leave it and flag it in the report.
