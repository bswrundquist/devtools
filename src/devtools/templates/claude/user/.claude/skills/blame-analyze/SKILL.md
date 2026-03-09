---
name: blame-analyze
description: Use when the user wants to understand the history of a file, function, or code area — who changed it, when, why, and how often. Identifies hot spots (frequently changed code) and traces the evolution of specific code.
tools: Bash, Read, Grep, Glob, Agent
---

# Blame Analyze

Trace the history of code to understand why it exists, who owns it, and how volatile it is.

## Modes

### 1. File History
Analyze a single file's evolution:
```bash
# Commit frequency and recency
git log --oneline --follow <file>

# Who has changed it most
git log --follow --format='%an' <file> | sort | uniq -c | sort -rn

# Change frequency over time (commits per month)
git log --follow --format='%as' <file> | cut -d- -f1,2 | sort | uniq -c
```

### 2. Function/Block History
Trace a specific function:
```bash
# Git log for a specific function (uses regex on diff)
git log --oneline -L ':<function_name>:<file>'

# Or a line range
git log --oneline -L <start>,<end>:<file>
```

### 3. Hot Spot Analysis
Find the most frequently changed files in the repo:
```bash
# Most changed files (last 6 months)
git log --since="6 months ago" --name-only --format='' | sort | uniq -c | sort -rn | head -20

# Files with most distinct authors (shared ownership = risk)
git log --since="6 months ago" --name-only --format='AUTHOR:%an' | awk '/^AUTHOR:/{author=$0; next} NF{print author, $0}' | sort -u | cut -d' ' -f2- | sort | uniq -c | sort -rn | head -20

# Churn: files that are both frequently changed AND large
git log --since="6 months ago" --numstat --format='' | awk '{adds[$3]+=$1; dels[$3]+=$2; count[$3]++} END {for(f in count) if(count[f]>3) printf "%d commits, +%d -%d: %s\n", count[f], adds[f], dels[f], f}' | sort -rn | head -20
```

### 4. Blame Deep Dive
For a specific file, understand current ownership:
```bash
# Current line-by-line blame
git blame <file>

# Blame ignoring whitespace changes
git blame -w <file>

# Blame showing the commit before the current one (peel back layers)
git blame <commit>^ -- <file>
```

## Analysis Output

### For File/Function History

**Timeline**: When were the major changes?
- List significant commits with dates and authors
- Note periods of high activity vs. stability

**Ownership**: Who knows this code?
- Primary author(s) by line count and commit count
- Recent vs. historical contributors
- Bus factor: how many people have touched this recently?

**Volatility**: How stable is this code?
- Commits per month over the past year
- Ratio of bug fixes to feature changes
- Consecutive changes by different authors (possible confusion/churn)

**Why It Matters**: What does the history tell us?
- Frequently fixed code may need redesign
- Code with many authors but no clear owner may lack coherence
- Code that hasn't changed in years may be stable OR abandoned

### For Hot Spot Analysis

Present as a table:
| File | Commits (6mo) | Authors | Bug Fixes | Assessment |
|------|--------------|---------|-----------|------------|
| ... | ... | ... | ... | Stable / Volatile / Needs attention |

## Rules

- Always use `--follow` for file history to track renames.
- Use `-w` with blame to ignore whitespace-only changes.
- Look at the full commit messages (not just oneline) for important changes to understand context.
- When analyzing functions, read the current code first to understand what it does.
- Flag files that have both high churn AND high bug-fix ratio — these are the most valuable refactoring targets.
- Don't just report numbers — interpret them. "42 commits in 6 months" means nothing without context like "that's 3x the repo average."
- For hot spot analysis, exclude auto-generated files, lock files, and changelogs.
