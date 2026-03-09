"""Root CLI entry point for devtools."""

from __future__ import annotations

import typer

from devtools.commands.ai import app as ai_app

app = typer.Typer(
    name="devtools",
    help="Developer tooling CLI.",
    no_args_is_help=True,
)

app.add_typer(ai_app, name="ai")
