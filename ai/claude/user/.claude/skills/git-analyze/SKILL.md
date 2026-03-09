---
name: git-analyze
description: Use when the user wants to understand a git codebase's recent activity, direction, contributors, common bugs, major initiatives, and roadmap signals. Analyzes git history, PRs, issues, and commit patterns to build a comprehensive picture.
tools: Bash, Read, Glob, Grep, Agent
---

# Git Codebase Analyzer

Produce a comprehensive analysis of a git repository's recent activity, direction, and health.

## Analysis Sections

Generate a report covering ALL of the following sections. Use the git commands and techniques listed under each section to gather data, then synthesize findings into clear, opinionated prose — not just raw command output.

---

### 1. Recent Activity Summary

**What's been happening lately?**

```bash
# Last 30 days of commit activity (volume, pace)
git log --oneline --since="30 days ago" | wc -l
git log --oneline --since="7 days ago" | wc -l

# Recent commits with context
git log --oneline --since="30 days ago" --no-merges | head -50

# Most active files recently
git log --since="30 days ago" --no-merges --pretty=format: --name-only | sort | uniq -c | sort -rn | head -20

# Merge commits (PRs landing)
git log --oneline --merges --since="30 days ago" | head -20
```

Summarize: Is the project active? Accelerating or slowing down? What areas of the codebase are getting the most attention?

---

### 2. Major Initiatives & Themes

**What are the big efforts underway?**

```bash
# Look at branch names for feature work
git branch -r --sort=-committerdate | head -30

# Commit messages grouped by common prefixes/themes
git log --oneline --since="90 days ago" --no-merges | head -100

# Large commits (many files changed = likely big features or refactors)
git log --since="90 days ago" --no-merges --pretty=format:"%h %s" --shortstat | head -60

# Tags and releases
git tag --sort=-creatordate | head -10
git log --tags --simplify-by-decoration --pretty="format:%ai %d" | head -10
```

Group commits into themes/initiatives. Identify 3-7 major workstreams. Look for patterns in branch names, commit prefixes, and which parts of the codebase are changing together.

---

### 3. Common Bugs & Issues

**What keeps breaking? What pain points exist?**

```bash
# Fix commits
git log --oneline --since="90 days ago" --no-merges --grep="fix" -i | head -30

# Bug-related commits
git log --oneline --since="90 days ago" --no-merges --grep="bug" -i | head -20

# Revert commits (something went wrong)
git log --oneline --since="90 days ago" --grep="revert" -i | head -10

# Hotfix branches
git branch -r --sort=-committerdate | grep -i -E "fix|hotfix|bug|patch" | head -10

# Files that appear frequently in fix commits (fragile areas)
git log --since="90 days ago" --no-merges --grep="fix" -i --pretty=format: --name-only | sort | uniq -c | sort -rn | head -15
```

Identify: Recurring problem areas, fragile parts of the codebase, types of bugs that keep appearing, and whether fixes are quick patches or deeper corrections.

---

### 4. Major Changes & Breaking Work

**What big shifts are happening?**

```bash
# Commits with large diffs
git log --since="90 days ago" --no-merges --pretty=format:"%h %s" --numstat | awk '/^[0-9]/ {adds+=$1; dels+=$2} /^[a-f0-9]/ {if (adds+dels > 200) print prev_line, "+"adds, "-"dels; adds=0; dels=0; prev_line=$0}'

# Deleted files (removing old code)
git log --since="90 days ago" --no-merges --diff-filter=D --pretty=format: --name-only | sort -u | head -20

# New files added
git log --since="90 days ago" --no-merges --diff-filter=A --pretty=format: --name-only | sort | uniq -c | sort -rn | head -20

# Refactor commits
git log --oneline --since="90 days ago" --no-merges --grep="refactor" -i | head -15

# Migration or schema changes
git log --oneline --since="90 days ago" --no-merges --grep="migrat" -i | head -10

# Breaking changes
git log --oneline --since="90 days ago" --no-merges --grep="break" -i | head -10
git log --oneline --since="90 days ago" --no-merges --grep="BREAKING" | head -10
```

Highlight: Architecture shifts, technology migrations, major refactors, deprecations, and any breaking changes.

---

### 5. Roadmap Signals

**Where is the project heading?**

```bash
# TODO/FIXME/HACK comments added recently
git log --since="90 days ago" --no-merges -p | grep -E "^\+" | grep -i -E "TODO|FIXME|HACK|XXX" | head -20

# Work-in-progress branches
git branch -r --sort=-committerdate | grep -i -E "wip|draft|experiment|proto|spike" | head -10

# Feature branches not yet merged
git branch -r --no-merged origin/main --sort=-committerdate 2>/dev/null | head -15
# (fallback to master if main doesn't exist)
git branch -r --no-merged origin/master --sort=-committerdate 2>/dev/null | head -15

# Recent additions to config, CI, or infrastructure (signals future direction)
git log --since="90 days ago" --no-merges --pretty=format: --name-only | grep -i -E "config|ci|deploy|infra|docker|k8s|terraform|\.yml$|\.yaml$" | sort | uniq -c | sort -rn | head -15

# Dependency changes (new tools/libraries being adopted)
git log --since="90 days ago" --no-merges -- "*requirements*" "*package.json" "*Cargo.toml" "*go.mod" "*Gemfile" "*pyproject.toml" "*pom.xml" --oneline | head -10
```

Infer likely direction from: unmerged feature branches, new dependencies, infrastructure changes, WIP work, and the trajectory of recent initiatives.

---

### 6. Contributor Analysis

**Who's doing the work?**

Run contributor stats for each time window:

```bash
# Last 6 months
echo "=== LAST 6 MONTHS ==="
git shortlog -sn --no-merges --since="6 months ago" | head -20

# Last 3 months
echo "=== LAST 3 MONTHS ==="
git shortlog -sn --no-merges --since="3 months ago" | head -20

# Last 1 month
echo "=== LAST 1 MONTH ==="
git shortlog -sn --no-merges --since="1 month ago" | head -20

# Last 2 weeks
echo "=== LAST 2 WEEKS ==="
git shortlog -sn --no-merges --since="2 weeks ago" | head -20
```

For each window, note:
- **Total contributors** vs **major contributors** (>10% of commits)
- Is contributor count growing or shrinking?
- Are there new contributors appearing recently?
- Is work concentrated (1-2 people) or distributed?
- Any contributors who dropped off or ramped up?

```bash
# Contributor churn: who's new in last 3 months vs 6 months
# (compare the two lists above)

# What are top contributors working on?
git log --since="30 days ago" --no-merges --author="<top contributor>" --pretty=format: --name-only | sort | uniq -c | sort -rn | head -10
```

Characterize the contributor profile:
- **Solo project** — 1 dominant contributor
- **Small team** — 2-5 active contributors
- **Broad community** — 10+ contributors, no single dominant one
- **Core + periphery** — Small core team with occasional outside contributions

---

### 7. Contribution Pattern

**Lots of small contributions or fewer large ones?**

```bash
# Commit size distribution (files changed per commit)
git log --since="90 days ago" --no-merges --pretty=format:"%h" --shortstat | grep -E "file" | awk '{print $1}' | sort -n | uniq -c | sort -rn | head -20

# Average commits per day
echo "Commits per day (last 90 days):"
echo "scale=1; $(git log --oneline --since='90 days ago' --no-merges | wc -l) / 90" | bc

# Commit frequency by day of week
git log --since="90 days ago" --no-merges --format="%ad" --date=format:"%A" | sort | uniq -c | sort -rn
```

Characterize: Is this a project with many small incremental changes, or fewer large changes? Do contributors make atomic commits or batch work? Is there a regular cadence or bursts of activity?

---

## Output Format

Present findings as a structured report with clear headers for each section. Use prose to synthesize — don't just dump command output. Be opinionated: draw conclusions, identify trends, flag concerns. Include specific numbers and commit references to back up observations.

End with a **TL;DR** section: 3-5 bullet points capturing the most important takeaways about the project's current state and trajectory.
