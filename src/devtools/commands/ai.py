"""AI tooling commands."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
from typing import Optional

import typer

from devtools.utils.paths import get_template_root
from devtools.utils.sync import SyncResult, print_summary, sync_directory

app = typer.Typer(help="AI tooling commands.", no_args_is_help=True)


def _make_filter(
    *,
    claude_md: bool,
    rules: bool,
    skills: bool,
    agents: bool,
) -> Callable[[Path], bool]:
    """Return an include predicate for sync_directory based on component flags."""

    def _include(rel: Path) -> bool:
        first = rel.parts[0]
        if first == "CLAUDE.md":
            return claude_md
        if first == "rules":
            return rules
        if first == "skills":
            return skills
        if first == "agents":
            return agents
        # Include unknown paths by default so future additions aren't silently dropped.
        return True

    return _include


def run_install(
    *,
    template_root: Path,
    user: bool,
    repo: bool,
    claude_md: bool,
    rules: bool,
    skills: bool,
    agents: bool,
    repo_root: Path,
    home_dir: Path | None,
    dry_run: bool,
    force: bool,
) -> dict[str, SyncResult]:
    """Core install logic. Returns results keyed by scope ("user", "repo").

    Extracted from the CLI command for testability — call this directly in
    tests, passing temp directories for template_root, home_dir, and repo_root.
    """
    if rules and user and not repo:
        print("Note: --rules only applies to repo scope. Skipping rules for user install.")

    prefix = "[dry-run] " if dry_run else ""
    results: dict[str, SyncResult] = {}

    if user:
        effective_home = home_dir if home_dir is not None else Path.home()
        src = template_root / "claude" / "user" / ".claude"
        dst = effective_home / ".claude"
        include = _make_filter(claude_md=claude_md, rules=False, skills=skills, agents=agents)

        print(f"\n{prefix}Installing user scope")
        print(f"  source: {src}")
        print(f"  dest:   {dst}")
        result = sync_directory(src, dst, include=include, dry_run=dry_run, force=force)
        print_summary(result, scope="user")
        results["user"] = result

    if repo:
        src = template_root / "claude" / "repo" / ".claude"
        dst = repo_root.resolve() / ".claude"
        include = _make_filter(claude_md=claude_md, rules=rules, skills=skills, agents=agents)

        print(f"\n{prefix}Installing repo scope")
        print(f"  source: {src}")
        print(f"  dest:   {dst}")
        result = sync_directory(src, dst, include=include, dry_run=dry_run, force=force)
        print_summary(result, scope="repo")
        results["repo"] = result

    return results


@app.command("install")
def install(
    user: bool = typer.Option(True, "--user/--no-user", help="Install to user scope (~/.claude/)."),
    repo: bool = typer.Option(
        False,
        "--repo/--no-repo",
        help="Install to repo scope (.claude/ in repo root).",
    ),
    claude_md: bool = typer.Option(True, "--claude-md/--no-claude-md", help="Install CLAUDE.md."),
    rules: bool = typer.Option(
        True, "--rules/--no-rules", help="Install rules/ (repo scope only)."
    ),
    skills: bool = typer.Option(True, "--skills/--no-skills", help="Install skills/."),
    agents: bool = typer.Option(True, "--agents/--no-agents", help="Install agents/."),
    repo_root: Path = typer.Option(
        Path("."), "--repo-root", help="Repo root for repo-scope install."
    ),
    home_dir: Optional[Path] = typer.Option(
        None, "--home-dir", help="Override home directory (default: ~). Useful for testing."
    ),
    dry_run: bool = typer.Option(
        False, "--dry-run", help="Show what would happen without making any changes."
    ),
    force: bool = typer.Option(
        False, "--force", help="Overwrite existing files that differ from the template."
    ),
) -> None:
    """Install Claude Code files from the devtools templates."""
    if not user and not repo:
        typer.echo("Error: at least one of --user or --repo must be selected.", err=True)
        raise typer.Exit(1)

    run_install(
        template_root=get_template_root(),
        user=user,
        repo=repo,
        claude_md=claude_md,
        rules=rules,
        skills=skills,
        agents=agents,
        repo_root=repo_root,
        home_dir=home_dir,
        dry_run=dry_run,
        force=force,
    )
