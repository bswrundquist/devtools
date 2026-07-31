# devtools

Version-controlled Claude Code config: skills, agents, and starter `CLAUDE.md`
files, plus a tiny installer that copies them into `~/.claude`.

## Layout

```
skills/             one directory per skill, each with a SKILL.md
  SKILLS_GUIDE.md   how skills work and how to invoke them
agents/             custom subagent definitions (one .md per agent)
templates/
  user/CLAUDE.md    ~/.claude/CLAUDE.md starter (installed only if missing)
  project/CLAUDE.md starter for a project's .claude/CLAUDE.md (copied manually)
src/devtools/       the installer (stdlib-only Python)
```

## Install

Straight from the repo with [uv](https://docs.astral.sh/uv/):

```bash
uvx --from git+ssh://git@github.com/bswrundquist/devtools devtools
```

Or from a local checkout:

```bash
uv run devtools
```

(Not `uvx --from .` — uvx caches the built tool by name and version, so it
can silently run a stale build after you edit skills. `uv run` always uses
the working tree.)

Options:

```bash
devtools --dry-run   # preview without writing
devtools --force     # also overwrite files that differ from the shipped version
devtools --dest DIR  # install somewhere other than ~/.claude ($CLAUDE_DIR works too)
```

What it does:

- **skills/ and agents/** — new files are added. Files already in `~/.claude`
  are *never* overwritten by default: if yours differs from the shipped
  version (you edited it, or you wrote your own with the same name), it is
  reported and left alone. Pass `--force` to take the repo's version.
- **CLAUDE.md** — installed only if you don't already have one. Your existing
  `~/.claude/CLAUDE.md` is never overwritten.
- Nothing in `~/.claude` is ever deleted. If you remove a skill from this
  repo, delete it from `~/.claude/skills` by hand.

## Updating this repo from your live config

`~/.claude` is where skills actually get edited and battle-tested. To pull the
live state back into the repo:

```bash
rsync -a --exclude .DS_Store ~/.claude/skills/ skills/
rsync -a --exclude .DS_Store ~/.claude/agents/ agents/
git diff   # review, then commit
```

## Add or edit a skill

Name it `<object>-<action>` when it belongs to a family, so related skills
autocomplete together: `pr-review`, `pr-fix`, `pr-comments-resolve`,
`pr-comments-respond`, `issue-create`. Skills tied to a specific tool follow
the same shape with the tool as the object: `pyspark-debug`, `dbt-write`,
`terraform-write`. Standalone verbs with nothing to group with stay bare:
`commit`, `push`.

```bash
mkdir skills/my-skill
$EDITOR skills/my-skill/SKILL.md
uvx --from . devtools
```

A `SKILL.md` starts with frontmatter Claude Code uses to decide when to load it:

```markdown
---
name: my-skill
description: Use when ... (this is how Claude decides to invoke it)
---

Instructions for the skill go here.
```

## Project-level config

For per-repo instructions, copy the starter into the project:

```bash
mkdir -p /path/to/project/.claude
cp templates/project/CLAUDE.md /path/to/project/.claude/CLAUDE.md
```

(Or just run `/init` inside Claude Code, which generates one from the codebase.)
