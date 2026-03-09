"""Merge-copy logic for devtools install commands."""

from __future__ import annotations

import filecmp
import shutil
from collections.abc import Callable
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class SyncResult:
    """Results from a sync_directory call."""

    copied: list[Path] = field(default_factory=list)
    unchanged: list[Path] = field(default_factory=list)
    conflicts: list[Path] = field(default_factory=list)


def sync_directory(
    source: Path,
    dest: Path,
    *,
    include: Callable[[Path], bool] | None = None,
    dry_run: bool = False,
    force: bool = False,
) -> SyncResult:
    """Merge-copy files from source into dest.

    Rules:
    - Skips .gitkeep files.
    - Creates missing destination directories (unless dry_run).
    - Copies missing files.
    - Skips files whose content is already identical.
    - On conflict (dest exists, content differs):
        - Without force: records as conflict, leaves dest unchanged.
        - With force: overwrites dest.
    - Never deletes destination files.
    """
    result = SyncResult()

    if not source.exists():
        return result

    for src_file in sorted(source.rglob("*")):
        if not src_file.is_file():
            continue

        if src_file.name == ".gitkeep":
            continue

        rel = src_file.relative_to(source)

        if include is not None and not include(rel):
            continue

        dst_file = dest / rel

        if not dst_file.exists():
            if not dry_run:
                dst_file.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(src_file, dst_file)
            result.copied.append(rel)

        elif filecmp.cmp(src_file, dst_file, shallow=False):
            result.unchanged.append(rel)

        else:
            # Conflict: dest exists and content differs.
            if force:
                if not dry_run:
                    shutil.copy2(src_file, dst_file)
                result.copied.append(rel)
            else:
                result.conflicts.append(rel)

    return result


def print_summary(result: SyncResult, *, scope: str) -> None:
    """Print a concise per-file and totals summary of a sync operation."""
    label = f"[{scope}]"

    for path in result.copied:
        print(f"  {label}  copied     {path}")
    for path in result.unchanged:
        print(f"  {label}  unchanged  {path}")
    for path in result.conflicts:
        print(f"  {label}  conflict   {path}  (use --force to overwrite)")

    total = len(result.copied) + len(result.unchanged) + len(result.conflicts)
    if total == 0:
        print(f"  {label}  nothing to install")
    else:
        print(
            f"\n  {label}  {len(result.copied)} copied, "
            f"{len(result.unchanged)} unchanged, "
            f"{len(result.conflicts)} conflicts"
        )
