"""Path resolution utilities for devtools."""

from __future__ import annotations

from pathlib import Path


def get_template_root() -> Path:
    """Return the path to the bundled Claude templates directory.

    Templates live at ``src/devtools/templates/`` in the source tree and are
    bundled into the installed wheel, so this works whether the package is
    installed via ``uvx``, ``pip``, or run directly from a checkout.
    """
    return Path(__file__).parent.parent / "templates"
