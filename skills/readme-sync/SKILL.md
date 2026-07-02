---
name: readme-sync
description: Use when loose markdown files (NOTES.md, PLAN.md, ANALYSIS.md, TODO.md) have accumulated in a repo, the README is out of date or missing info that exists in scattered docs, or the user says "clean up the docs" or "consolidate markdown files".
tools: Bash, Read, Write, Edit, Grep, Glob
---

# README Sync

Condense accumulated loose markdown files into one clean README.md, then delete the files that were absorbed.

## Process

1. **Discover** - Find loose markdown, excluding standard keepers:

   ```bash
   find . -maxdepth 2 -name "*.md" \
     -not -name "README.md" -not -name "CHANGELOG.md" \
     -not -name "CONTRIBUTING.md" -not -name "LICENSE.md" \
     -not -name "CODE_OF_CONDUCT.md" -not -name "SECURITY.md" \
     -not -path "./docs/*" -not -path "./.git/*" \
     -not -path "./node_modules/*" -not -path "./.venv/*"
   ```

2. **Check freshness** - For each file, when was it last touched and why:

   ```bash
   git log -1 --format="%as %s" -- NOTES.md
   ```

   A PLAN.md untouched since before the feature it describes shipped is stale. Cross-check claims against the current code (does the Makefile target it mentions still exist?).

3. **Read everything** - All loose files plus the current README, in full. Note what is still true, what duplicates the README, and what contradicts the code.

4. **Classify** - Give each file a verdict:

   | Verdict | Meaning |
   |---------|---------|
   | Absorb | Content still true and useful — fold into README, then delete the file |
   | Delete | Stale, superseded, or one-off scratch — nothing worth keeping |
   | Keep | Standard file or genuinely standalone doc (ADRs, docs site pages) |

5. **Present the plan** - Show the table below to the user and wait for approval. Do not touch any file before this step.

6. **Rewrite the README** - Merge absorbed content into the structure below. Condense aggressively; verify every command actually works before writing it in.

7. **Delete approved files** - Only files marked Absorb or Delete in the approved plan. Use `git rm` for tracked files so the deletion shows in the diff; plain `rm` otherwise. Do not commit unless asked.

## Output Format

Present the plan before acting:

```markdown
## Consolidation Plan

| File | Last touched | Verdict | Notes |
|------|--------------|---------|-------|
| SETUP.md | 2026-03-14 | Absorb | Install steps → Quickstart |
| NOTES.md | 2026-06-20 | Absorb | Deploy gotchas → Deployment |
| PLAN.md | 2025-11-02 | Delete | Auth rework shipped in v2.1 |
| TODO.md | 2026-06-28 | Keep | Active task list, still in use |

Proceed? I'll rewrite README.md, then delete the Absorb/Delete files.
```

## README Structure

Sections that earn their place — skip any that would be empty:

| Section | Contents |
|---------|----------|
| What it is | One or two sentences: what the project does and for whom |
| Quickstart | Install and run in under a minute of reading (`uv sync`, `make run`) |
| Usage | The handful of operations people actually perform |
| Development | Setup, tests, lint (`uv run pytest`, `uv run ruff check`) |
| Deployment | How it ships, if applicable |

## Rules

- Show the plan and get approval before deleting anything — no exceptions.
- Never delete a file that wasn't absorbed or explicitly marked stale in the approved plan.
- Never touch CHANGELOG.md, CONTRIBUTING.md, LICENSE*, CODE_OF_CONDUCT.md, SECURITY.md, or anything under `docs/`.
- Absorb means condense, not paste — every line of the new README earns its place.
- The code is the source of truth: when a doc contradicts it, follow the code and drop the doc's claim.
- Verify commands before writing them into the README (check Makefile targets, pyproject scripts exist).
- If content is genuinely reference-grade and too long for the README, propose moving it under `docs/` instead of deleting it.
- If there is no README, create one from the absorbed content and the code.
- Don't invent content that exists in neither the source docs nor the code.
- `git rm` for tracked files; never commit or push unless the user asks.
