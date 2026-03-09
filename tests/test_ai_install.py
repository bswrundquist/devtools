"""Tests for devtools ai install logic."""

from pathlib import Path

import pytest
from typer.testing import CliRunner

from devtools.cli import app
from devtools.commands.ai import run_install
from devtools.utils.paths import find_template_root

# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------


@pytest.fixture()
def template_root(tmp_path):
    """Minimal template directory matching the real ai/claude/ layout."""
    # User templates
    user_dir = tmp_path / "ai" / "claude" / "user" / ".claude"
    user_dir.mkdir(parents=True)
    (user_dir / "CLAUDE.md").write_text("# User CLAUDE.md\n")
    skills = user_dir / "skills"
    skills.mkdir()
    (skills / ".gitkeep").write_text("")
    (skills / "example_skill.md").write_text("# Example skill\n")
    agents = user_dir / "agents"
    agents.mkdir()
    (agents / ".gitkeep").write_text("")
    (agents / "example_agent.md").write_text("# Example agent\n")

    # Repo templates
    repo_dir = tmp_path / "ai" / "claude" / "repo" / ".claude"
    repo_dir.mkdir(parents=True)
    (repo_dir / "CLAUDE.md").write_text("# Repo CLAUDE.md\n")
    rules = repo_dir / "rules"
    rules.mkdir()
    (rules / ".gitkeep").write_text("")
    (rules / "example_rule.md").write_text("# Example rule\n")
    repo_skills = repo_dir / "skills"
    repo_skills.mkdir()
    (repo_skills / ".gitkeep").write_text("")
    repo_agents = repo_dir / "agents"
    repo_agents.mkdir()
    (repo_agents / ".gitkeep").write_text("")

    # Needs pyproject.toml so find_template_root can locate it.
    (tmp_path / "pyproject.toml").write_text("[project]\nname = 'test'\n")

    return tmp_path


# ---------------------------------------------------------------------------
# Dry-run tests
# ---------------------------------------------------------------------------


def test_user_install_dry_run(template_root, tmp_path):
    home = tmp_path / "home"

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=True,
        skills=True,
        agents=True,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=True,
        force=False,
    )

    assert "user" in results
    assert Path("CLAUDE.md") in results["user"].copied
    # Dry run: nothing written.
    assert not (home / ".claude" / "CLAUDE.md").exists()


def test_repo_install_dry_run(template_root, tmp_path):
    repo_root = tmp_path / "myrepo"
    repo_root.mkdir()

    results = run_install(
        template_root=template_root,
        user=False,
        repo=True,
        claude_md=True,
        rules=True,
        skills=True,
        agents=True,
        repo_root=repo_root,
        home_dir=None,
        dry_run=True,
        force=False,
    )

    assert "repo" in results
    assert Path("CLAUDE.md") in results["repo"].copied
    assert not (repo_root / ".claude" / "CLAUDE.md").exists()


# ---------------------------------------------------------------------------
# Component toggle tests
# ---------------------------------------------------------------------------


def test_no_skills_excludes_skills_files(template_root, tmp_path):
    home = tmp_path / "home"

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=False,
        agents=True,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=False,
    )

    copied = [str(p) for p in results["user"].copied]
    assert not any("skills" in p for p in copied)
    assert any("agents" in p for p in copied)
    assert not (home / ".claude" / "skills").exists()


def test_no_agents_excludes_agents_files(template_root, tmp_path):
    home = tmp_path / "home"

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=True,
        agents=False,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=False,
    )

    copied = [str(p) for p in results["user"].copied]
    assert not any("agents" in p for p in copied)
    assert any("skills" in p for p in copied)


def test_rules_skipped_on_user_only_scope(template_root, tmp_path, capsys):
    home = tmp_path / "home"

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=True,  # requested, but user-only scope
        skills=True,
        agents=True,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=False,
    )

    # rules never copied for user scope.
    copied = [str(p) for p in results["user"].copied]
    assert not any("rules" in p for p in copied)

    # A note should have been printed.
    captured = capsys.readouterr()
    assert "rules" in captured.out.lower()


def test_rules_installed_for_repo_scope(template_root, tmp_path):
    repo_root = tmp_path / "myrepo"
    repo_root.mkdir()

    results = run_install(
        template_root=template_root,
        user=False,
        repo=True,
        claude_md=True,
        rules=True,
        skills=True,
        agents=True,
        repo_root=repo_root,
        home_dir=None,
        dry_run=False,
        force=False,
    )

    copied = [str(p) for p in results["repo"].copied]
    assert any("rules" in p for p in copied)


# ---------------------------------------------------------------------------
# Overwrite / force tests
# ---------------------------------------------------------------------------


def test_no_overwrite_without_force(template_root, tmp_path):
    home = tmp_path / "home"
    dest_claude = home / ".claude"
    dest_claude.mkdir(parents=True)
    (dest_claude / "CLAUDE.md").write_text("existing content\n")

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=False,
        agents=False,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=False,
    )

    assert Path("CLAUDE.md") in results["user"].conflicts
    assert (dest_claude / "CLAUDE.md").read_text() == "existing content\n"


def test_overwrite_with_force(template_root, tmp_path):
    home = tmp_path / "home"
    dest_claude = home / ".claude"
    dest_claude.mkdir(parents=True)
    (dest_claude / "CLAUDE.md").write_text("existing content\n")

    results = run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=False,
        agents=False,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=True,
    )

    assert Path("CLAUDE.md") in results["user"].copied
    assert (dest_claude / "CLAUDE.md").read_text() == "# User CLAUDE.md\n"


# ---------------------------------------------------------------------------
# Path override tests
# ---------------------------------------------------------------------------


def test_home_dir_override(template_root, tmp_path):
    custom_home = tmp_path / "custom_home"

    run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=False,
        agents=False,
        repo_root=tmp_path,
        home_dir=custom_home,
        dry_run=False,
        force=False,
    )

    assert (custom_home / ".claude" / "CLAUDE.md").exists()


def test_repo_root_override(template_root, tmp_path):
    repo_root = tmp_path / "my_project"
    repo_root.mkdir()

    run_install(
        template_root=template_root,
        user=False,
        repo=True,
        claude_md=True,
        rules=True,
        skills=True,
        agents=True,
        repo_root=repo_root,
        home_dir=None,
        dry_run=False,
        force=False,
    )

    assert (repo_root / ".claude" / "CLAUDE.md").exists()


# ---------------------------------------------------------------------------
# .gitkeep test
# ---------------------------------------------------------------------------


def test_gitkeep_not_installed(template_root, tmp_path):
    home = tmp_path / "home"

    run_install(
        template_root=template_root,
        user=True,
        repo=False,
        claude_md=True,
        rules=False,
        skills=True,
        agents=True,
        repo_root=tmp_path,
        home_dir=home,
        dry_run=False,
        force=False,
    )

    gitkeeps = list((home / ".claude").rglob(".gitkeep"))
    assert gitkeeps == []


# ---------------------------------------------------------------------------
# Error / validation tests
# ---------------------------------------------------------------------------


def test_template_root_not_found(tmp_path):
    # Pass _search_paths to suppress the module-location fallback.
    with pytest.raises(FileNotFoundError, match="devtools repo"):
        find_template_root(start=tmp_path, _search_paths=[tmp_path])


def test_neither_scope_exits_with_error():
    runner = CliRunner()
    result = runner.invoke(app, ["ai", "install", "--no-user", "--no-repo"])
    assert result.exit_code != 0
    assert (
        "at least one" in result.output.lower()
        or "at least one" in (result.stderr if hasattr(result, "stderr") else "").lower()
    )
