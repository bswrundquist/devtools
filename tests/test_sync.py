"""Tests for the merge-copy sync logic."""

from pathlib import Path

from devtools.utils.sync import SyncResult, sync_directory


def test_copy_missing_file(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    (src / "file.md").write_text("hello\n")

    result = sync_directory(src, dst)

    assert Path("file.md") in result.copied
    assert (dst / "file.md").read_text() == "hello\n"


def test_skip_unchanged_file(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()
    (src / "file.md").write_text("hello\n")
    (dst / "file.md").write_text("hello\n")

    result = sync_directory(src, dst)

    assert Path("file.md") in result.unchanged
    assert result.copied == []
    assert result.conflicts == []


def test_conflict_without_force(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()
    (src / "file.md").write_text("new content\n")
    (dst / "file.md").write_text("old content\n")

    result = sync_directory(src, dst, force=False)

    assert Path("file.md") in result.conflicts
    assert result.copied == []
    assert (dst / "file.md").read_text() == "old content\n"


def test_force_overwrites_conflict(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()
    (src / "file.md").write_text("new content\n")
    (dst / "file.md").write_text("old content\n")

    result = sync_directory(src, dst, force=True)

    assert Path("file.md") in result.copied
    assert result.conflicts == []
    assert (dst / "file.md").read_text() == "new content\n"


def test_gitkeep_is_skipped(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    (src / ".gitkeep").write_text("")
    (src / "real.md").write_text("content\n")

    result = sync_directory(src, dst)

    copied_names = [p.name for p in result.copied]
    assert ".gitkeep" not in copied_names
    assert "real.md" in copied_names
    assert not (dst / ".gitkeep").exists()


def test_dry_run_does_not_write(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    (src / "file.md").write_text("hello\n")

    result = sync_directory(src, dst, dry_run=True)

    assert Path("file.md") in result.copied
    assert not (dst / "file.md").exists()
    assert not dst.exists()


def test_creates_missing_subdirectories(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    (src / "subdir").mkdir()
    (src / "subdir" / "nested.md").write_text("content\n")

    result = sync_directory(src, dst)

    assert Path("subdir/nested.md") in result.copied
    assert (dst / "subdir" / "nested.md").read_text() == "content\n"


def test_missing_source_returns_empty_result(tmp_path):
    src = tmp_path / "nonexistent"
    dst = tmp_path / "dst"

    result = sync_directory(src, dst)

    assert result == SyncResult()


def test_include_filter_excludes_files(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    (src / "CLAUDE.md").write_text("md\n")
    skills = src / "skills"
    skills.mkdir()
    (skills / "tool.md").write_text("skill\n")

    # Exclude skills.
    result = sync_directory(src, dst, include=lambda rel: rel.parts[0] != "skills")

    copied_names = [str(p) for p in result.copied]
    assert "CLAUDE.md" in copied_names
    assert not any("skills" in n for n in copied_names)


def test_never_deletes_destination_files(tmp_path):
    src = tmp_path / "src"
    dst = tmp_path / "dst"
    src.mkdir()
    dst.mkdir()
    (dst / "extra.md").write_text("keep me\n")
    (src / "file.md").write_text("new\n")

    sync_directory(src, dst)

    assert (dst / "extra.md").exists()
