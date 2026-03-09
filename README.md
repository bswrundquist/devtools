# devtools

General developer tooling repo. v1 ships a filesystem-based installer for
Claude Code configuration files. No framework, no magic — just a CLI that
copies known files from this repo into the right locations.

---

## What it is

`devtools` is a place to version-control and distribute Claude Code scaffolding
(and, eventually, other AI provider config) across your machines and projects.
Instead of managing `~/.claude/` and `.claude/` by hand, you run one command
and the right files are copied where they belong.

---

## Why user and repo are separate

Claude Code loads configuration from two locations:

| Scope | Path | Purpose |
|-------|------|---------|
| user  | `~/.claude/` | Applies to every project on your machine |
| repo  | `.claude/` in the repo root | Applies only to that project |

This repo stores templates for both under `ai/claude/user/` and
`ai/claude/repo/`. The install command copies each to its proper destination
without mixing them.

---

## How `devtools ai install` works

1. Locates this repo's root by walking upward from CWD (or the installed
   package location) until it finds a directory with both `pyproject.toml`
   and `ai/claude/`.
2. For each selected scope, resolves source and destination directories.
3. Runs a **merge-copy**: copies missing files, skips identical files,
   reports conflicts without overwriting (unless `--force`).
4. Prints a per-file summary: `copied`, `unchanged`, `conflict`.
5. Never deletes destination files.
6. Skips `.gitkeep` files.

---

## Available toggles

### Scope

| Flag | Default | Effect |
|------|---------|--------|
| `--user` / `--no-user` | `--user` | Install to `~/.claude/` |
| `--repo` / `--no-repo` | `--no-repo` | Install to `.claude/` in repo root |

### Components

| Flag | Default | Effect |
|------|---------|--------|
| `--claude-md` / `--no-claude-md` | on | Install `CLAUDE.md` |
| `--rules` / `--no-rules` | on | Install `rules/` (repo scope only) |
| `--skills` / `--no-skills` | on | Install `skills/` |
| `--agents` / `--no-agents` | on | Install `agents/` |

### Other options

| Flag | Default | Effect |
|------|---------|--------|
| `--repo-root PATH` | `.` | Repo root for repo-scope install |
| `--home-dir PATH` | `~` | Override home directory |
| `--dry-run` | off | Show what would happen, write nothing |
| `--force` | off | Overwrite conflicting destination files |

---

## Example commands

```bash
# Preview what would be installed for your user config.
devtools ai install --user --no-repo --dry-run

# Install user config for real.
devtools ai install --user --no-repo

# Preview what would be installed into the current repo.
devtools ai install --repo --no-user --repo-root . --dry-run

# Install into the current repo.
devtools ai install --repo --no-user --repo-root .

# Install both scopes in one shot.
devtools ai install --user --repo --repo-root .

# Install everything except agents.
devtools ai install --user --no-agents

# Force-overwrite a conflicting CLAUDE.md.
devtools ai install --user --no-repo --force
```

---

## Running via Makefile

```bash
make bootstrap         # uv sync
make test              # run pytest
make check             # lint + test
make ai-user-dry-run   # dry-run user install
make ai-user-install   # run user install
make ai-repo-dry-run   # dry-run repo install
make ai-repo-install   # run repo install
```

---

## Current limitations (v1)

- Claude Code only. No other AI providers.
- No MCP server setup.
- No hooks setup.
- No manifest or plugin registry.
- No network calls — local file copy only.
- Never deletes destination files (add manually if you want to prune).
- `rules/` only applies to repo scope; it is silently skipped for user scope.

---

## Extending the Claude templates

**Add a new skill (user-level):**
```
ai/claude/user/.claude/skills/my-skill.md
```

**Add a new agent (user-level):**
```
ai/claude/user/.claude/agents/my-agent.md
```

**Add a repo-level rule:**
```
ai/claude/repo/.claude/rules/my-rule.md
```

**Add a repo-level skill:**
```
ai/claude/repo/.claude/skills/my-skill.md
```

After adding files, re-run `devtools ai install` to sync them to their
destinations. Existing identical files are skipped; changed files are
reported as conflicts unless you pass `--force`.

---

## Adding another AI provider (future)

The command tree is structured to accommodate this:

```
devtools/
  src/devtools/
    commands/
      ai.py       # today: Claude Code only
      # ai.py will grow or split when other providers are added
    utils/
      paths.py
      sync.py
  ai/
    claude/       # today
    # other-provider/  (future)
```

When a new provider is added:
1. Add its templates under `ai/<provider>/`.
2. Add a subcommand (or extend `ai.py`) for its install logic.
3. Reuse `sync_directory` from `utils/sync.py` — the copy semantics are
   provider-agnostic.
