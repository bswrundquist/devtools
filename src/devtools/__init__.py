"""Install versioned Claude Code config (skills, agents, starter CLAUDE.md) into ~/.claude."""

from __future__ import annotations

import argparse
import filecmp
import os
import shutil
from importlib.resources import files
from pathlib import Path

__version__ = "1.0.0"

# Directories synced into ~/.claude. Files are added or left alone — an
# existing file that differs is never overwritten unless --force is given.
SYNCED_DIRS = ("skills", "agents")


def _data_root() -> Path:
    """Locate the bundled config, whether installed as a wheel or run from a checkout."""
    packaged = Path(str(files("devtools"))) / "_data"
    if packaged.is_dir():
        return packaged
    repo = Path(__file__).resolve().parents[2]
    if (repo / "skills").is_dir():
        return repo
    raise SystemExit("error: bundled skills not found (broken install?)")


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        prog="devtools",
        description="Install Claude Code skills and agents into ~/.claude. "
        "Adds new files and leaves everything already there untouched; "
        "nothing is ever deleted.",
    )
    parser.add_argument(
        "--dest",
        default=os.environ.get("CLAUDE_DIR") or str(Path.home() / ".claude"),
        help="install target (default: ~/.claude, or $CLAUDE_DIR if set)",
    )
    parser.add_argument(
        "--dry-run", action="store_true", help="show what would change without writing"
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="overwrite files that differ from the shipped version "
        "(default: report and leave them alone)",
    )
    parser.add_argument("--version", action="version", version=__version__)
    args = parser.parse_args(argv)

    data = _data_root()
    dest = Path(args.dest)
    added = updated = unchanged = skipped = 0

    def put(src: Path, dst: Path, rel: str) -> None:
        nonlocal added, updated, unchanged, skipped
        if not dst.exists():
            print(f"  + {rel}")
            if not args.dry_run:
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(src, dst)
            added += 1
        elif not dst.is_file():
            raise SystemExit(f"error: {dst} exists but is not a regular file")
        elif filecmp.cmp(src, dst, shallow=False):
            unchanged += 1
        elif args.force:
            print(f"  ~ {rel}")
            if not args.dry_run:
                shutil.copyfile(src, dst)
            updated += 1
        else:
            print(f"  ! {rel} differs, left alone (--force to overwrite)")
            skipped += 1

    if args.dry_run:
        print("dry run — nothing will be written")
    print(f"installing to {dest}")

    for dirname in SYNCED_DIRS:
        srcdir = data / dirname
        if not srcdir.is_dir():
            continue
        for src in sorted(p for p in srcdir.rglob("*") if p.is_file()):
            if src.name == ".DS_Store":
                continue
            rel = f"{dirname}/{src.relative_to(srcdir)}"
            put(src, dest / rel, rel)

    claude_md = dest / "CLAUDE.md"
    if claude_md.exists():
        print("  = CLAUDE.md already exists, left untouched")
    else:
        put(data / "templates" / "user" / "CLAUDE.md", claude_md, "CLAUDE.md")

    print(
        f"done: {added} added, {updated} updated, {unchanged} unchanged, {skipped} skipped"
    )
