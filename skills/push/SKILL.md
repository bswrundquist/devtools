---
name: push
description: Use when the user asks to push commits to the remote repository. Checks branch status, confirms the remote and branch, and pushes safely.
tools: Bash
---

# Push

Push local commits to the remote repository safely.

## Process

1. **Check status** - Run `git status` and `git log --oneline @{u}..HEAD` (if tracking branch exists) to see what will be pushed.
2. **Verify branch** - Confirm the current branch name and its upstream tracking branch. If no upstream is set, use `git push -u origin <branch>`.
3. **Push** - Run `git push`. Never force push unless the user explicitly asks for it.

## Rules

- **Never force push to main/master** - Warn the user and refuse even if asked.
- **Always show what will be pushed** before pushing (commits, branch, remote).
- If the push is rejected due to divergence, inform the user and suggest `git pull --rebase` rather than force pushing.
- If there is no remote tracking branch, set one with `-u`.
- If there are uncommitted changes, inform the user before pushing.
