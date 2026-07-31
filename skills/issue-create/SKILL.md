---
name: issue-create
description: Use when the user wants to create a new issue, ticket, or bug report in GitHub, GitLab, or Jira from a plain description. Researches the tracker and codebase first so the draft is grounded, then ends with an acceptance criteria checklist meant to feed straight into /solution-design or /implement.
tools: Bash, Read, Grep, Glob
argument-hint: <description of the issue> [--project <repo or key>] [--create]
---

# Create Issue

Turn a plain description into a well-formed issue in GitHub, GitLab, or Jira. This is step zero of the Issue -> Ship workflow: everything downstream (`/issue-summary`, `/solution-design`, `/implement`) reads the issue this produces, so the draft has to stand on its own as context, not just as a summary of what the user typed.

## Arguments

`$ARGUMENTS` - a free-text description of the problem, bug, or request. Can be rough; turning it into something precise is the job.

- `--project <repo or key>` - explicit target: `owner/repo` for GitHub/GitLab, a project key (e.g. `PLAT`) for Jira. Skips auto-detection.
- `--create` - actually file the issue after drafting it. Without this flag, the skill only drafts and previews. Filing a ticket is visible to the whole team, so treat this the same as `pr-comments-respond --post`: the only standing approval.

## Process

### 1. Work out where this issue lives

Look for explicit signals in the description first: a Jira key prefix, the words "gitlab"/"github", a repo slug, a project key. If nothing's explicit, detect from the current repo:

```bash
git remote get-url origin 2>/dev/null   # github.com -> gh, gitlab -> glab
```

Before reaching for a CLI, check whether a tracker MCP server is connected:

```
ToolSearch("jira")
ToolSearch("gitlab")
ToolSearch("atlassian")
```

If one shows up, prefer it over the CLI. It usually returns structured project metadata (valid issue types, required custom fields, workflow states) that a CLI would otherwise force you to scrape or guess. Fall back to the CLIs below when nothing is connected.

If there's no repo and no signal in the description at all, ask which tracker and which project/repo, rather than guessing.

### 2. Ground the draft before writing it

A generic issue is not useful context for the next prompt. Do this research first:

**Check for duplicates or close relatives:**

```bash
# GitHub
gh issue list --search "<keywords>" --state all --limit 20

# GitLab
glab issue list --search "<keywords>"

# Jira
jira issue list -q "text ~ \"<keywords>\"" --plain
```

If something close already exists, surface it instead of quietly filing a near-duplicate.

**Find the actual code the description touches** (grep for it, don't guess at file names):

```bash
grep -rln "<entity from description>" --include="*.py" | head
git log --oneline -10 -- path/to/suspected/area/
```

**Pull real labels, components, issue types, and milestones** so the draft reuses what the project already has instead of inventing new taxonomy:

```bash
# GitHub
gh label list
gh api repos/{owner}/{repo}/milestones --jq '.[].title'

# GitLab
glab label list

# Jira
jira issue list --plain --columns labels | tr ',' '\n' | sort -u
jira project list
```

**Find a sensible assignee or owner** (same trick as `/issue-summary`):

```bash
cat .github/CODEOWNERS CODEOWNERS 2>/dev/null
git shortlog -sn --since="6 months ago" -- path/touched/ | head -5
```

### 3. Draft the issue

- **Title**: specific and action-oriented, not a restatement of the whole description.
- **Body**: the problem or request in plain language, what's in and out of scope, and the relevant files or related issues found in step 2. If the user's description already implies an approach, note it as a suggestion, not a decision, that's what `/solution-design` is for.
- **Metadata**: labels, assignee, milestone, only ones that already exist in the project.
- **Acceptance criteria**: a checklist at the very end. Each item must be something a different person, or a different prompt, could check off without re-reading this conversation. "Improve performance" is not a criterion. "P95 latency on `/checkout` drops below 200ms under the existing load test" is.

### 4. Preview, then create only on confirmation

Print the full draft as markdown (see Output Format). Do not run any create command yet unless `--create` was passed. Otherwise, ask the user to confirm or adjust first.

Once approved:

```bash
# GitHub
gh issue create --title "<title>" --body "<body>" --label "<label>" --assignee "<user>" --milestone "<milestone>"

# GitLab
glab issue create --title "<title>" --description "<body>" --label "<label>" --assignee "<user>" --milestone "<milestone>"

# Jira
jira issue create --type "<type>" --summary "<title>" --body "<body>" --label "<label>" --project "<key>" --no-input
```

Report back the created issue's URL or key.

## Output Format

```markdown
## Draft: <issue title>

**Target:** <GitHub | GitLab | Jira> - <owner/repo or project key> (not yet created, pass --create or confirm to file it)

### Problem
<1-2 sentences: what's wrong or needed, in plain language, grounded in what was actually found>

### Scope
- In scope: ...
- Out of scope: ...

### Relevant code
- `path/to/file.py:42` - <what's there and why it's relevant>
- Related: #38 "<title>" (open/closed, how it relates) — omit section if none found

### Suggested metadata
**Labels:** bug, backend (reused from the project's existing labels)
**Assignee:** @sam (owns `src/payments/`, top committer there)
**Milestone:** v1.4

## Acceptance Criteria
- [ ] <concrete, independently checkable condition>
- [ ] <concrete, independently checkable condition>
- [ ] <concrete, independently checkable condition>
```

After filing:

```markdown
Created: <url> (<key or #number>)
```

Then offer the next step: `/issue-summary <ref>` for stakeholders, or `/solution-design <ref>` to go straight into planning.

## Rules

- Never run the create command without `--create` or an explicit yes from the user in chat. Filing a ticket is visible to the whole team and isn't quietly undoable.
- Check for duplicates first. A near-duplicate issue is worse than no issue.
- Every claim in the draft comes from something actually read: labels that exist, an assignee who actually owns that code, a related issue that's actually related. Don't invent plausible-looking details.
- Acceptance criteria are the whole point of this skill. Each one needs to be checkable by someone (or some prompt) that wasn't in this conversation. Vague criteria produce a vague `/solution-design` next.
- Reuse the project's existing labels, issue types, and components instead of inventing new ones.
- Write like a person: clear, concise, no em dashes, no filler, no hedging boilerplate.
- State unknowns as unknowns in the draft rather than smoothing them over. The next prompt in the chain inherits whatever confidence this one states.
- If the tracker can't be determined and nothing in the repo or description hints at one, ask instead of guessing.
