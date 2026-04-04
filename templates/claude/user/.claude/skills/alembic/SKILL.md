---
name: alembic
description: Use when working with Alembic database migrations — creating, running, testing, and troubleshooting migrations with SQLAlchemy. Covers autogenerate, data migrations, multiple heads, and common pitfalls.
tools: Bash, Read, Write, Edit, Grep, Glob
---

# Alembic

Manage database schema migrations with Alembic and SQLAlchemy.

## Setup

```bash
pip install alembic
alembic init alembic   # creates alembic/ dir and alembic.ini
```

### env.py — wire up your models

The generated `alembic/env.py` must import your models so autogenerate can detect changes:

```python
# alembic/env.py
import os
from myapp.db import Base       # your DeclarativeBase
from myapp import models        # import all models so they register on Base  # noqa: F401

config.set_main_option("sqlalchemy.url", os.environ["DATABASE_URL"])
target_metadata = Base.metadata
```

For **async engines**:

```python
from sqlalchemy.ext.asyncio import async_engine_from_config
import asyncio

def run_migrations_online() -> None:
    connectable = async_engine_from_config(
        config.get_section(config.config_ini_section, {}),
        prefix="sqlalchemy.",
    )

    async def do_run() -> None:
        async with connectable.connect() as connection:
            await connection.run_sync(run_migrations)

    asyncio.run(do_run())
```

### alembic.ini — don't hardcode the URL

```ini
# Leave blank — set it from the environment in env.py
sqlalchemy.url =
```

### Naming convention

Set this on your `MetaData` so autogenerate produces consistent constraint names:

```python
from sqlalchemy import MetaData

convention = {
    "ix": "ix_%(column_0_label)s",
    "uq": "uq_%(table_name)s_%(column_0_name)s",
    "ck": "ck_%(table_name)s_%(constraint_name)s",
    "fk": "fk_%(table_name)s_%(column_0_name)s_%(referred_table_name)s",
    "pk": "pk_%(table_name)s",
}
metadata = MetaData(naming_convention=convention)
Base = declarative_base(metadata=metadata)
```

## Common commands

```bash
# Generate a migration from model changes
alembic revision --autogenerate -m "add status column to users"

# Create an empty migration (for data migrations or manual DDL)
alembic revision -m "backfill display_name"

# Apply all pending migrations
alembic upgrade head

# Apply to a specific revision
alembic upgrade abc123

# Downgrade one step
alembic downgrade -1

# Downgrade to base (empty schema)
alembic downgrade base

# Show current revision(s)
alembic current

# Show full history
alembic history --verbose

# Check what autogenerate would detect without writing a file
alembic check

# Show the SQL that would run without executing
alembic upgrade head --sql
```

## Autogenerate

Autogenerate compares your SQLAlchemy models to the live DB schema and generates the diff.

**Detects:**
- Tables added / removed
- Columns added / removed / type changed (if `compare_type=True` in env.py)
- Indexes added / removed
- Unique constraints added / removed
- Foreign keys added / removed

**Does NOT detect automatically:**
- Stored procedures, triggers, functions
- Column default changes (often)
- Sequence changes
- PostgreSQL ENUMs (partial support)
- Table/column comments

Enable type comparison in `env.py`:
```python
context.configure(
    connection=connection,
    target_metadata=target_metadata,
    compare_type=True,        # detect column type changes
    compare_server_default=True,  # detect server_default changes
)
```

**Always review the generated migration before applying it.** Autogenerate makes mistakes — especially with inherited models, views, and dialect-specific features.

## Data migrations

Add a column, backfill data, then make it non-nullable:

```python
import sqlalchemy as sa
from alembic import op

def upgrade() -> None:
    # 1. Add as nullable
    op.add_column("users", sa.Column("display_name", sa.String(255), nullable=True))

    # 2. Backfill
    op.execute("UPDATE users SET display_name = username")

    # 3. Make non-nullable
    op.alter_column("users", "display_name", nullable=False)

def downgrade() -> None:
    op.drop_column("users", "display_name")
```

For complex backfills with Python logic:

```python
from sqlalchemy.orm import Session

def upgrade() -> None:
    bind = op.get_bind()
    session = Session(bind=bind)

    rows = session.execute(sa.text("SELECT id, first_name, last_name FROM users")).fetchall()
    for row in rows:
        session.execute(
            sa.text("UPDATE users SET display_name = :name WHERE id = :id"),
            {"name": f"{row.first_name} {row.last_name}", "id": row.id},
        )
    session.commit()
```

## Multiple heads

Occurs when two migrations both descend from the same parent (parallel feature branches):

```bash
# Check for multiple heads
alembic heads

# Merge them
alembic merge heads -m "merge feature-a and feature-b migrations"
```

The generated merge migration:
```python
down_revision = ("abc123", "def456")  # both heads as parents
```

## Testing migrations

Test that upgrade → downgrade → upgrade is idempotent:

```python
from alembic.command import downgrade, upgrade
from alembic.config import Config

def test_migrations_roundtrip(tmp_db_url: str) -> None:
    cfg = Config("alembic.ini")
    cfg.set_main_option("sqlalchemy.url", tmp_db_url)

    upgrade(cfg, "head")
    downgrade(cfg, "base")
    upgrade(cfg, "head")   # must succeed cleanly
```

## Common pitfalls

### Non-nullable column on a populated table

Never do this in one step — it will fail if rows exist:

```python
# WRONG
op.add_column("users", sa.Column("status", sa.String(), nullable=False))

# RIGHT: add nullable → backfill → constrain
op.add_column("users", sa.Column("status", sa.String(), nullable=True))
op.execute("UPDATE users SET status = 'active'")
op.alter_column("users", "status", nullable=False)
```

### server_default vs default

`default` is Python-side (SQLAlchemy only). `server_default` is DB-side and visible to migrations:

```python
# autogenerate can detect server_default changes
Column("created_at", DateTime, server_default=func.now())

# autogenerate ignores this
Column("created_at", DateTime, default=datetime.utcnow)
```

### PostgreSQL ENUM types

Autogenerate doesn't reliably handle ENUM creation and deletion:

```python
def upgrade() -> None:
    status_enum = sa.Enum("active", "inactive", name="user_status")
    status_enum.create(op.get_bind(), checkfirst=True)
    op.add_column("users", sa.Column("status", status_enum))

def downgrade() -> None:
    op.drop_column("users", "status")
    sa.Enum(name="user_status").drop(op.get_bind(), checkfirst=True)
```

### Large table operations (PostgreSQL)

Some operations lock the table. Use `CONCURRENTLY` where possible:

```python
def upgrade() -> None:
    # Don't use op.create_index for concurrent — use raw SQL
    op.execute("CREATE INDEX CONCURRENTLY ix_users_email ON users (email)")

def downgrade() -> None:
    op.execute("DROP INDEX CONCURRENTLY ix_users_email")
```

Wrap in `with op.get_context().autocommit_block():` since `CONCURRENTLY` can't run in a transaction.

## Rules

- Always read generated migrations before applying. Autogenerate is a starting point, not a final answer.
- Never edit a migration that's already been applied to a shared environment (staging, production).
- Always implement `downgrade()` — even if you never use it, it's essential for testing.
- One logical change per migration — don't bundle unrelated schema changes.
- Don't import ORM models directly in migration files — they may change. Use `op.execute()` with raw SQL or `sa.table()` for data migrations.
- Keep env.py's `DATABASE_URL` out of version control — always read from environment.
