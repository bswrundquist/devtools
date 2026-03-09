"""Path resolution utilities for devtools."""

from __future__ import annotations

from pathlib import Path


def find_template_root(
    start: Path | None = None,
    *,
    _search_paths: list[Path] | None = None,
) -> Path:
    """Locate the devtools repo root by walking upward from start (or CWD).

    A valid root contains both ``pyproject.toml`` and ``ai/claude/``.

    Also walks upward from this source file's location so that the command
    works when run via ``uv run devtools`` from within the repo checkout.

    Args:
        start: Where to start walking. Defaults to CWD.
        _search_paths: Override the list of root candidates (for testing only).
                       Defaults to [start-or-CWD, module-file-location].

    Raises FileNotFoundError if no valid root is found.
    """

    def _walk_up(path: Path) -> Path | None:
        for current in [path.resolve(), *path.resolve().parents]:
            if (current / "pyproject.toml").exists() and (current / "ai" / "claude").exists():
                return current
        return None

    candidates = (
        _search_paths
        if _search_paths is not None
        else [
            start or Path.cwd(),
            Path(__file__),
        ]
    )

    for candidate in candidates:
        result = _walk_up(candidate)
        if result:
            return result

    raise FileNotFoundError(
        "Could not find the devtools repo root. "
        "Run this command from within a devtools checkout "
        "(a directory tree containing both pyproject.toml and ai/claude/)."
    )
