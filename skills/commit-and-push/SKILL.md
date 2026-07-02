---
name: commit-and-push
description: Use when the user asks to commit and push changes. Commits using conventional commits format, then pushes to the remote without asking for confirmation.
tools: Bash, Skill
---

# Commit and Push

Commit changes and push to the remote repository in one step.

## Process

1. **Commit** - Invoke the `/commit` skill to create well-structured commits.
2. **Push** - Invoke the `/push` skill to push commits to the remote. Do NOT ask for confirmation before pushing — push immediately.

## Rules

- Always commit first using the commit skill, then push using the push skill.
- Never ask the user to confirm the push — push automatically after committing.
- Never force push to main/master.
