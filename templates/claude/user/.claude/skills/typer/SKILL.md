---
name: typer
description: Use when creating command-line interfaces with Typer. Build well-typed CLIs with type hints, excellent help text, enums for choices, and environment variable integration. Typer provides automatic validation and rich output.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Typer

Expert guidance for building professional, well-typed command-line interfaces with Typer.

## Why Typer?

- **Type hints everywhere** - Automatic validation and conversion
- **Excellent help text** - Auto-generated from docstrings and annotations
- **Rich output** - Built-in integration with Rich library
- **Environment variables** - Easy integration with envvars
- **Enums for choices** - Type-safe options
- **Minimal boilerplate** - Less code than Click or argparse
- **Auto-completion** - Shell completion out of the box

## Core Principles

1. **Type Everything** - Use type hints for all parameters
2. **Document Everything** - Clear help text for all commands and options
3. **Use Enums** - For any fixed set of choices
4. **Environment Variables** - Support envvars for configuration
5. **Validate Early** - Use type system to catch errors
6. **Rich Output** - Use Rich for beautiful terminal output

## Installation

```bash
# Basic installation
pip install typer

# With rich output (recommended)
pip install "typer[all]"

# Or with uv
uv pip install "typer[all]"
```

## Basic CLI Structure

### Simple Command

```python
import typer
from typing import Optional

app = typer.Typer()

def main(
    name: str = typer.Argument(..., help="Name of the user"),
    age: int = typer.Option(25, "--age", "-a", help="Age of the user"),
    email: Optional[str] = typer.Option(None, help="Email address"),
) -> None:
    """
    Greet a user with their information.

    This is a simple example showing basic Typer usage.
    """
    typer.echo(f"Hello {name}!")
    typer.echo(f"Age: {age}")
    if email:
        typer.echo(f"Email: {email}")

if __name__ == "__main__":
    typer.run(main)
```

**Usage:**
```bash
python main.py Alice
python main.py Alice --age 30
python main.py Alice -a 30 --email alice@example.com
```

### Multiple Commands (Recommended Structure)

```python
import typer
from typing import Optional
from enum import Enum

app = typer.Typer(
    name="myapp",
    help="My awesome CLI application",
    add_completion=True,
)

@app.command()
def create(
    name: str = typer.Argument(..., help="Name of the resource"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Enable verbose output"),
) -> None:
    """
    Create a new resource.
    """
    if verbose:
        typer.echo(f"Creating resource: {name}")
    typer.echo(f"✓ Created {name}")

@app.command()
def delete(
    name: str = typer.Argument(..., help="Name of the resource to delete"),
    force: bool = typer.Option(False, "--force", "-f", help="Skip confirmation"),
) -> None:
    """
    Delete a resource.
    """
    if not force:
        confirm = typer.confirm(f"Are you sure you want to delete {name}?")
        if not confirm:
            typer.echo("Cancelled")
            raise typer.Abort()

    typer.echo(f"✓ Deleted {name}")

if __name__ == "__main__":
    app()
```

**Usage:**
```bash
python main.py create myresource
python main.py create myresource --verbose
python main.py delete myresource
python main.py delete myresource --force
```

## Type Hints - Use Everywhere

### Basic Types

```python
import typer
from typing import Optional, List
from pathlib import Path

def command(
    # Strings
    name: str = typer.Argument(...),
    description: Optional[str] = typer.Option(None),

    # Numbers
    count: int = typer.Option(10),
    ratio: float = typer.Option(0.5),

    # Booleans (flags)
    verbose: bool = typer.Option(False, "--verbose", "-v"),
    quiet: bool = typer.Option(False, "--quiet", "-q"),

    # Paths
    input_file: Path = typer.Argument(..., exists=True),
    output_dir: Path = typer.Option(Path("."), dir_okay=True),

    # Lists (multiple values)
    tags: List[str] = typer.Option([], "--tag", "-t"),
) -> None:
    """Example showing all basic types."""
    pass
```

### Path Validation

```python
from pathlib import Path
import typer

def process_file(
    input_file: Path = typer.Argument(
        ...,
        exists=True,           # Must exist
        file_okay=True,        # Must be a file
        dir_okay=False,        # Cannot be a directory
        readable=True,         # Must be readable
        resolve_path=True,     # Convert to absolute path
        help="Input file to process"
    ),
    output_dir: Path = typer.Option(
        Path("."),
        exists=True,
        dir_okay=True,
        file_okay=False,
        writable=True,
        help="Output directory"
    ),
) -> None:
    """
    Process a file and save results to directory.
    """
    typer.echo(f"Processing: {input_file}")
    typer.echo(f"Output to: {output_dir}")
```

## Enums - For Fixed Choices

**ALWAYS use Enums instead of strings for choices.**

```python
from enum import Enum
import typer

class Environment(str, Enum):
    """Deployment environment choices."""
    development = "development"
    staging = "staging"
    production = "production"

class LogLevel(str, Enum):
    """Logging level choices."""
    debug = "debug"
    info = "info"
    warning = "warning"
    error = "error"
    critical = "critical"

class OutputFormat(str, Enum):
    """Output format choices."""
    json = "json"
    yaml = "yaml"
    table = "table"
    csv = "csv"

app = typer.Typer()

@app.command()
def deploy(
    environment: Environment = typer.Option(
        Environment.development,
        "--env", "-e",
        help="Deployment environment",
        case_sensitive=False,
    ),
    log_level: LogLevel = typer.Option(
        LogLevel.info,
        "--log-level", "-l",
        help="Logging level",
    ),
    output_format: OutputFormat = typer.Option(
        OutputFormat.table,
        "--format", "-f",
        help="Output format",
    ),
) -> None:
    """
    Deploy application to specified environment.
    """
    typer.echo(f"Deploying to {environment.value}")
    typer.echo(f"Log level: {log_level.value}")
    typer.echo(f"Output format: {output_format.value}")

if __name__ == "__main__":
    app()
```

**Benefits of Enums:**
- Type-safe (IDE autocomplete)
- Automatic validation
- Clear error messages
- Documented choices in help text
- No typos possible

## Environment Variables

**Support environment variables for all configuration options.**

```python
import typer
from typing import Optional
from enum import Enum
from pathlib import Path

class Environment(str, Enum):
    dev = "dev"
    prod = "prod"

def main(
    # API credentials from env vars
    api_key: str = typer.Option(
        ...,
        "--api-key",
        envvar="API_KEY",  # Read from API_KEY env var
        help="API key (can be set via API_KEY env var)",
    ),
    api_secret: str = typer.Option(
        ...,
        "--api-secret",
        envvar="API_SECRET",
        help="API secret (can be set via API_SECRET env var)",
    ),

    # Configuration with env var fallback
    database_url: str = typer.Option(
        "sqlite:///app.db",
        "--database-url",
        envvar="DATABASE_URL",
        help="Database connection string",
    ),

    # Environment from env var
    environment: Environment = typer.Option(
        Environment.dev,
        "--env",
        envvar="APP_ENV",
        help="Application environment",
    ),

    # Paths from env vars
    config_file: Optional[Path] = typer.Option(
        None,
        "--config",
        envvar="CONFIG_FILE",
        exists=True,
        help="Configuration file path",
    ),

    # Boolean flags from env vars
    debug: bool = typer.Option(
        False,
        "--debug",
        envvar="DEBUG",
        help="Enable debug mode",
    ),

    # Numeric values from env vars
    timeout: int = typer.Option(
        30,
        "--timeout",
        envvar="REQUEST_TIMEOUT",
        help="Request timeout in seconds",
    ),

    # Lists from env vars (comma-separated)
    allowed_hosts: List[str] = typer.Option(
        [],
        "--allowed-host",
        envvar="ALLOWED_HOSTS",
        help="Allowed hosts (comma-separated in env var)",
    ),
) -> None:
    """
    Run application with configuration from CLI or environment variables.

    Environment variables take precedence over defaults but are overridden by CLI args.
    """
    typer.echo(f"Environment: {environment.value}")
    typer.echo(f"Database: {database_url}")
    typer.echo(f"Debug: {debug}")
    typer.echo(f"Timeout: {timeout}s")

if __name__ == "__main__":
    typer.run(main)
```

**Usage:**
```bash
# Using CLI arguments
python main.py --api-key abc123 --api-secret secret123

# Using environment variables
export API_KEY=abc123
export API_SECRET=secret123
export APP_ENV=prod
export DEBUG=true
python main.py

# Mix of both (CLI overrides env vars)
export API_KEY=abc123
python main.py --api-secret secret123 --env prod
```

### Environment Variable Naming Convention

```python
# Good naming conventions for env vars
# - UPPERCASE_WITH_UNDERSCORES
# - Prefix with app name for clarity

class Config:
    """Centralized configuration with env vars."""

    # Database
    DB_HOST: str = typer.Option(..., envvar="MYAPP_DB_HOST")
    DB_PORT: int = typer.Option(5432, envvar="MYAPP_DB_PORT")
    DB_NAME: str = typer.Option("myapp", envvar="MYAPP_DB_NAME")

    # API
    API_KEY: str = typer.Option(..., envvar="MYAPP_API_KEY")
    API_URL: str = typer.Option(..., envvar="MYAPP_API_URL")

    # Features
    ENABLE_CACHE: bool = typer.Option(True, envvar="MYAPP_ENABLE_CACHE")
    CACHE_TTL: int = typer.Option(3600, envvar="MYAPP_CACHE_TTL")
```

## Rich Help Text

### Command Documentation

```python
import typer
from typing import Optional

app = typer.Typer()

@app.command()
def process(
    input_file: str = typer.Argument(
        ...,
        help="Path to the input file to process",
        metavar="FILE",  # Shows as FILE in help instead of INPUT_FILE
    ),
    output_file: Optional[str] = typer.Option(
        None,
        "--output", "-o",
        help="Path to save processed output. Defaults to stdout if not provided.",
        metavar="FILE",
    ),
    workers: int = typer.Option(
        4,
        "--workers", "-w",
        help="Number of parallel workers to use. Higher values = faster processing.",
        min=1,
        max=32,
    ),
    verbose: bool = typer.Option(
        False,
        "--verbose", "-v",
        help="Enable verbose output with detailed progress information.",
    ),
) -> None:
    """
    Process input file and generate output.

    This command reads the input file, processes it using the specified number
    of workers, and writes the result to the output file (or stdout).

    Examples:

        # Process file with default settings
        $ myapp process input.txt

        # Process with custom output file
        $ myapp process input.txt --output result.txt

        # Use 8 workers for faster processing
        $ myapp process input.txt -w 8 -o result.txt
    """
    if verbose:
        typer.echo(f"Processing {input_file} with {workers} workers...")

    # Processing logic here
    typer.echo("✓ Done")

if __name__ == "__main__":
    app()
```

### Application-Level Help

```python
import typer

app = typer.Typer(
    name="myapp",
    help="MyApp - A powerful CLI tool for data processing",
    epilog="For more information, visit https://myapp.example.com/docs",
    add_completion=True,
)

# Commands go here...

if __name__ == "__main__":
    app()
```

## Complete Example: Well-Typed CLI

```python
"""
MyApp CLI - A complete example showing best practices.
"""
import typer
from typing import Optional, List
from enum import Enum
from pathlib import Path
from rich.console import Console
from rich.table import Table

# Initialize console for rich output
console = Console()

# Define enums for choices
class Environment(str, Enum):
    """Deployment environment."""
    development = "development"
    staging = "staging"
    production = "production"

class LogLevel(str, Enum):
    """Logging verbosity level."""
    debug = "debug"
    info = "info"
    warning = "warning"
    error = "error"

class OutputFormat(str, Enum):
    """Output format for results."""
    json = "json"
    yaml = "yaml"
    table = "table"

# Create app
app = typer.Typer(
    name="myapp",
    help="MyApp - Professional CLI application",
    add_completion=True,
)

# Global options (can be used across commands)
state = {"verbose": False}

@app.callback()
def main(
    verbose: bool = typer.Option(
        False,
        "--verbose", "-v",
        envvar="MYAPP_VERBOSE",
        help="Enable verbose output",
    ),
) -> None:
    """
    MyApp CLI - Manage your application from the command line.

    Set global options here that apply to all commands.
    """
    state["verbose"] = verbose

@app.command()
def deploy(
    environment: Environment = typer.Option(
        Environment.development,
        "--env", "-e",
        envvar="MYAPP_ENV",
        help="Target environment for deployment",
        case_sensitive=False,
    ),
    config_file: Path = typer.Option(
        Path("config.yaml"),
        "--config", "-c",
        envvar="MYAPP_CONFIG",
        exists=True,
        file_okay=True,
        dir_okay=False,
        readable=True,
        help="Path to configuration file",
    ),
    dry_run: bool = typer.Option(
        False,
        "--dry-run",
        help="Simulate deployment without making changes",
    ),
    skip_tests: bool = typer.Option(
        False,
        "--skip-tests",
        help="Skip running tests before deployment",
    ),
    tags: List[str] = typer.Option(
        [],
        "--tag", "-t",
        help="Tags to apply to deployment (can be used multiple times)",
    ),
) -> None:
    """
    Deploy application to specified environment.

    This command deploys your application with the provided configuration.
    By default, tests are run before deployment.

    Examples:

        # Deploy to production
        $ myapp deploy --env production

        # Dry run to staging
        $ myapp deploy -e staging --dry-run

        # Deploy with tags
        $ myapp deploy -e prod -t v1.2.0 -t hotfix
    """
    if state["verbose"]:
        console.print(f"[bold blue]Deploying to {environment.value}[/bold blue]")
        console.print(f"Config: {config_file}")
        console.print(f"Dry run: {dry_run}")
        console.print(f"Skip tests: {skip_tests}")
        if tags:
            console.print(f"Tags: {', '.join(tags)}")

    if dry_run:
        console.print("[yellow]DRY RUN - No changes will be made[/yellow]")

    # Deployment logic here...
    console.print(f"[green]✓ Successfully deployed to {environment.value}[/green]")

@app.command()
def status(
    environment: Environment = typer.Option(
        Environment.development,
        "--env", "-e",
        envvar="MYAPP_ENV",
        help="Environment to check status",
    ),
    output_format: OutputFormat = typer.Option(
        OutputFormat.table,
        "--format", "-f",
        help="Output format",
    ),
) -> None:
    """
    Check deployment status for an environment.

    Shows current deployment information including version, status, and uptime.
    """
    # Mock data
    status_data = {
        "version": "1.2.3",
        "status": "running",
        "uptime": "5d 3h 22m",
        "requests": "1,234,567",
    }

    if output_format == OutputFormat.table:
        table = Table(title=f"Status - {environment.value}")
        table.add_column("Metric", style="cyan")
        table.add_column("Value", style="green")

        for key, value in status_data.items():
            table.add_row(key.title(), value)

        console.print(table)

    elif output_format == OutputFormat.json:
        import json
        console.print(json.dumps(status_data, indent=2))

    elif output_format == OutputFormat.yaml:
        console.print("version:", status_data["version"])
        console.print("status:", status_data["status"])
        # ... etc

@app.command()
def logs(
    environment: Environment = typer.Argument(
        ...,
        help="Environment to fetch logs from",
    ),
    lines: int = typer.Option(
        100,
        "--lines", "-n",
        envvar="MYAPP_LOG_LINES",
        help="Number of log lines to display",
        min=1,
        max=10000,
    ),
    follow: bool = typer.Option(
        False,
        "--follow", "-f",
        help="Follow log output (like tail -f)",
    ),
    level: LogLevel = typer.Option(
        LogLevel.info,
        "--level", "-l",
        help="Minimum log level to display",
    ),
) -> None:
    """
    Display application logs from specified environment.

    Examples:

        # Show last 100 lines from production
        $ myapp logs production

        # Follow logs in real-time
        $ myapp logs prod --follow

        # Show only errors
        $ myapp logs prod --level error -n 50
    """
    console.print(f"Fetching logs from {environment.value}...")
    console.print(f"Lines: {lines}, Level: {level.value}, Follow: {follow}")
    # Log fetching logic here...

@app.command()
def config(
    action: str = typer.Argument(
        ...,
        help="Action to perform: get, set, list",
    ),
    key: Optional[str] = typer.Argument(
        None,
        help="Configuration key",
    ),
    value: Optional[str] = typer.Argument(
        None,
        help="Configuration value (for 'set' action)",
    ),
    global_config: bool = typer.Option(
        False,
        "--global",
        help="Use global configuration instead of project config",
    ),
) -> None:
    """
    Manage application configuration.

    Examples:

        # List all config values
        $ myapp config list

        # Get a specific value
        $ myapp config get api.endpoint

        # Set a value
        $ myapp config set api.timeout 30

        # Set global value
        $ myapp config set api.key abc123 --global
    """
    if action == "list":
        console.print("[bold]Configuration:[/bold]")
        # List config logic...
    elif action == "get":
        if not key:
            console.print("[red]Error: key required for 'get' action[/red]")
            raise typer.Exit(1)
        console.print(f"{key}: <value>")
    elif action == "set":
        if not key or not value:
            console.print("[red]Error: key and value required for 'set' action[/red]")
            raise typer.Exit(1)
        scope = "global" if global_config else "project"
        console.print(f"[green]✓ Set {key}={value} ({scope})[/green]")

if __name__ == "__main__":
    app()
```

## Validation and Error Handling

### Custom Validation

```python
import typer
from typing import Optional

def validate_email(email: str) -> str:
    """Validate email format."""
    if "@" not in email or "." not in email:
        raise typer.BadParameter("Invalid email format")
    return email

def validate_port(port: int) -> int:
    """Validate port number."""
    if not (1 <= port <= 65535):
        raise typer.BadParameter("Port must be between 1 and 65535")
    return port

def command(
    email: str = typer.Option(
        ...,
        "--email",
        callback=validate_email,
        help="Email address",
    ),
    port: int = typer.Option(
        8080,
        "--port",
        callback=validate_port,
        help="Port number",
    ),
) -> None:
    """Command with validation."""
    typer.echo(f"Email: {email}")
    typer.echo(f"Port: {port}")
```

### Range Validation

```python
def command(
    workers: int = typer.Option(
        4,
        "--workers",
        min=1,
        max=32,
        help="Number of workers (1-32)",
    ),
    timeout: float = typer.Option(
        30.0,
        "--timeout",
        min=0.1,
        max=300.0,
        help="Timeout in seconds (0.1-300)",
    ),
) -> None:
    """Command with range validation."""
    pass
```

### Error Handling

```python
import typer
from pathlib import Path

def process_file(
    input_file: Path = typer.Argument(..., exists=True),
) -> None:
    """Process a file with error handling."""
    try:
        with open(input_file) as f:
            content = f.read()
            # Process content...

    except PermissionError:
        typer.secho(
            f"Error: Permission denied reading {input_file}",
            fg=typer.colors.RED,
            err=True,
        )
        raise typer.Exit(1)

    except Exception as e:
        typer.secho(
            f"Error: {e}",
            fg=typer.colors.RED,
            err=True,
        )
        raise typer.Exit(1)

    typer.secho("✓ Success", fg=typer.colors.GREEN)
```

## Interactive Prompts

```python
import typer
from typing import Optional

def setup() -> None:
    """Interactive setup wizard."""
    # Simple prompt
    name = typer.prompt("What's your name?")

    # Prompt with default
    email = typer.prompt("Email", default="user@example.com")

    # Hidden input (for passwords)
    password = typer.prompt("Password", hide_input=True)

    # Confirmation prompt
    confirm = typer.prompt(
        "Confirm password",
        hide_input=True,
        confirmation_prompt=True,
    )

    # Yes/no confirmation
    should_continue = typer.confirm("Do you want to continue?")
    if not should_continue:
        raise typer.Abort()

    # Optional prompt (can be empty)
    optional_value: Optional[str] = typer.prompt(
        "API key (optional)",
        default="",
        show_default=False,
    )

    typer.echo(f"Setup complete for {name}!")
```

## Progress Bars

```python
import typer
import time

def process() -> None:
    """Show progress bar."""
    items = range(100)

    with typer.progressbar(
        items,
        label="Processing items",
        show_eta=True,
        show_percent=True,
    ) as progress:
        for item in progress:
            # Process item...
            time.sleep(0.1)

    typer.echo("✓ Done")
```

## Subcommands (Command Groups)

```python
import typer

app = typer.Typer()

# Database subcommands
db_app = typer.Typer(help="Database management commands")
app.add_typer(db_app, name="db")

@db_app.command()
def migrate() -> None:
    """Run database migrations."""
    typer.echo("Running migrations...")

@db_app.command()
def seed() -> None:
    """Seed database with sample data."""
    typer.echo("Seeding database...")

@db_app.command()
def reset() -> None:
    """Reset database to initial state."""
    if typer.confirm("This will delete all data. Continue?"):
        typer.echo("Resetting database...")

# User subcommands
user_app = typer.Typer(help="User management commands")
app.add_typer(user_app, name="user")

@user_app.command()
def create(
    username: str = typer.Argument(...),
    email: str = typer.Option(..., "--email"),
) -> None:
    """Create a new user."""
    typer.echo(f"Creating user: {username}")

@user_app.command()
def delete(
    username: str = typer.Argument(...),
) -> None:
    """Delete a user."""
    typer.echo(f"Deleting user: {username}")

if __name__ == "__main__":
    app()
```

**Usage:**
```bash
python main.py db migrate
python main.py db seed
python main.py user create alice --email alice@example.com
```

## Best Practices

### 1. Always Use Type Hints

```python
# ✅ Good - Fully typed
def command(
    name: str = typer.Argument(...),
    count: int = typer.Option(10),
    verbose: bool = typer.Option(False),
) -> None:
    """Well-typed command."""
    pass

# ❌ Bad - No types
def command(name, count=10, verbose=False):
    """Untyped command."""
    pass
```

### 2. Use Enums for Choices

```python
from enum import Enum

# ✅ Good - Type-safe enum
class Format(str, Enum):
    json = "json"
    yaml = "yaml"

def export(format: Format = typer.Option(Format.json)) -> None:
    pass

# ❌ Bad - String choices
def export(format: str = typer.Option("json")) -> None:
    if format not in ["json", "yaml"]:  # Manual validation needed
        raise ValueError("Invalid format")
```

### 3. Support Environment Variables

```python
# ✅ Good - Env var support
def command(
    api_key: str = typer.Option(
        ...,
        "--api-key",
        envvar="API_KEY",
        help="API key (or set API_KEY env var)",
    ),
) -> None:
    pass

# ❌ Bad - No env var support
def command(
    api_key: str = typer.Option(..., "--api-key"),
) -> None:
    pass
```

### 4. Provide Rich Help Text

```python
# ✅ Good - Detailed help
@app.command()
def deploy(
    env: str = typer.Argument(..., help="Target environment (dev/staging/prod)"),
) -> None:
    """
    Deploy application to specified environment.

    This command builds and deploys your application. It will:
    - Run tests
    - Build Docker image
    - Push to registry
    - Deploy to Kubernetes

    Examples:
        $ myapp deploy staging
        $ myapp deploy prod --skip-tests
    """
    pass

# ❌ Bad - Minimal help
def deploy(env: str) -> None:
    """Deploy."""
    pass
```

### 5. Validate Input

```python
# ✅ Good - Validation
def command(
    port: int = typer.Option(
        8080,
        min=1,
        max=65535,
        help="Port number (1-65535)",
    ),
    workers: int = typer.Option(
        4,
        min=1,
        max=32,
        help="Number of workers (1-32)",
    ),
) -> None:
    pass

# ❌ Bad - No validation
def command(port: int = 8080, workers: int = 4) -> None:
    # Could receive invalid values
    pass
```

### 6. Use Rich Output

```python
from rich.console import Console
from rich.table import Table

console = Console()

# ✅ Good - Rich output
def list_users() -> None:
    table = Table(title="Users")
    table.add_column("ID", style="cyan")
    table.add_column("Name", style="green")
    table.add_row("1", "Alice")
    table.add_row("2", "Bob")
    console.print(table)

# ❌ Bad - Plain output
def list_users() -> None:
    print("ID | Name")
    print("1  | Alice")
    print("2  | Bob")
```

## Project Structure

```
myapp/
├── pyproject.toml
├── README.md
├── myapp/
│   ├── __init__.py
│   ├── __main__.py          # Entry point
│   ├── cli.py               # CLI definition
│   ├── commands/
│   │   ├── __init__.py
│   │   ├── deploy.py        # Deploy command
│   │   ├── logs.py          # Logs command
│   │   └── config.py        # Config command
│   ├── models.py            # Enums and types
│   └── utils.py             # Helper functions
└── tests/
    └── test_cli.py
```

**myapp/__main__.py:**
```python
from myapp.cli import app

if __name__ == "__main__":
    app()
```

**myapp/cli.py:**
```python
import typer
from myapp.commands import deploy, logs, config

app = typer.Typer(name="myapp", help="MyApp CLI")

app.add_typer(deploy.app, name="deploy")
app.add_typer(logs.app, name="logs")
app.add_typer(config.app, name="config")
```

**myapp/models.py:**
```python
from enum import Enum

class Environment(str, Enum):
    dev = "dev"
    staging = "staging"
    prod = "prod"

class LogLevel(str, Enum):
    debug = "debug"
    info = "info"
    warning = "warning"
    error = "error"
```

## Testing

```python
from typer.testing import CliRunner
from myapp.cli import app

runner = CliRunner()

def test_deploy_command():
    result = runner.invoke(app, ["deploy", "--env", "staging"])
    assert result.exit_code == 0
    assert "staging" in result.stdout

def test_deploy_invalid_env():
    result = runner.invoke(app, ["deploy", "--env", "invalid"])
    assert result.exit_code != 0

def test_help():
    result = runner.invoke(app, ["--help"])
    assert result.exit_code == 0
    assert "MyApp CLI" in result.stdout
```

## Quick Reference

```python
import typer
from typing import Optional, List
from enum import Enum
from pathlib import Path

class Format(str, Enum):
    json = "json"
    yaml = "yaml"

app = typer.Typer()

@app.command()
def example(
    # Required argument
    name: str = typer.Argument(..., help="Name"),

    # Optional argument with default
    count: int = typer.Argument(10, help="Count"),

    # Option with short flag
    verbose: bool = typer.Option(False, "--verbose", "-v"),

    # Option with env var
    api_key: str = typer.Option(
        ...,
        "--api-key",
        envvar="API_KEY",
    ),

    # Enum choice
    format: Format = typer.Option(Format.json, "--format", "-f"),

    # Path with validation
    input_file: Path = typer.Option(
        ...,
        exists=True,
        file_okay=True,
        dir_okay=False,
    ),

    # List option
    tags: List[str] = typer.Option([], "--tag", "-t"),

    # Number with range
    workers: int = typer.Option(4, min=1, max=32),
) -> None:
    """Example command showing all common patterns."""
    pass

if __name__ == "__main__":
    app()
```
