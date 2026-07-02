---
name: study-session
description: Use when the user wants to study a body of documentation — produces a clear overview of what each document covers, how they connect, what's missing or stale, and what else is worth gathering.
tools: Bash, Read, Grep, Glob, Agent, WebFetch
argument-hint: [paths, globs, or URLs] [--focus <topic>]
---

# Study Session

Digest a documentation corpus and come out with a map: what exists, what each piece covers, how the pieces connect, where the gaps and contradictions are, and what's worth hunting down next.

## Arguments

`$ARGUMENTS` — what to study:

- Paths or globs: `docs/ README.md adr/*.md`
- URLs (fetched with WebFetch): wiki pages, published docs.
- Nothing — discover the docs in the current repo (step 1).
- `--focus <topic>` — weight the analysis toward one topic (e.g. `--focus deployment`).

## Process

### 1. Collect the corpus

```bash
find . \( -name "*.md" -o -name "*.rst" -o -name "*.adoc" \) \
  -not -path "*/node_modules/*" -not -path "*/.venv/*" -not -path "*/.git/*" | head -50
ls docs/ adr/ 2>/dev/null
```

Also count as documentation: OpenAPI specs, `CHANGELOG`, runbooks, well-commented config, Makefile help targets.

### 2. Read everything

Read every document in the corpus. For large corpora (roughly 15+ docs), fan out `Explore` agents per directory or theme and synthesize their summaries — but read the load-bearing docs (README, architecture, main runbook) yourself.

### 3. Check freshness and truth

Docs lie by aging. For each significant doc:

```bash
git log -1 --format="%as %an" -- docs/architecture.md   # last touched, by whom
```

Spot-check load-bearing claims against the code — does the referenced module, flag, or endpoint still exist?

```bash
grep -rn "LEGACY_SYNC_ENABLED" --include="*.py" | head   # doc says this flag controls sync — does it?
```

### 4. Synthesize

## Output Format

```markdown
## Study Session: <corpus>

### What we have
| Doc | Covers | Last touched | State |
|-----|--------|--------------|-------|
| `README.md` | setup, quickstart | 2026-05 | current |
| `docs/architecture.md` | service map | 2024-11 | stale — still shows the old queue |

(States: **current** / **aging** / **stale** / **contradicts X**)

### How it fits together
Prose walk-through of the corpus as a system: the entry point, which doc is upstream
of which, the shared concepts, where a reader naturally flows next. A short outline
or mermaid diagram if the relationships are non-linear.

### Gaps and stale spots
- Topics with no doc at all (setup? architecture? runbook? decision records? onboarding?)
- Doc-vs-code drift and doc-vs-doc contradictions, with the evidence.

### Worth adding / looking for
- Specific docs to write, and for whom.
- Things that probably exist elsewhere — wiki, Confluence, a ticket, someone's head — and who to ask.

### Suggested reading order
1. ...
```

## Rules

- Verify before trusting: a doc is only "current" if its load-bearing claims survived a spot-check against the code.
- Distinguish **missing** (nobody wrote it) from **unfindable** (exists but not linked from anywhere) — both are gaps, with different fixes.
- Connections are the point — a list of summaries without the "how it fits together" story is not a study session.
- Contradictions get quoted, not paraphrased: show both statements and say which one the code supports.
- Keep the inventory in the table and the story in prose; don't bury the narrative in cells.
- Offer to save the write-up to a file (e.g. `docs/STUDY.md`) if the user wants to keep it.
