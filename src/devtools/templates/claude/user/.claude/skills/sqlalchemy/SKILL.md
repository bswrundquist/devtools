---
name: sqlalchemy
description: Use when creating SQLAlchemy models, defining database schemas, working with ORM relationships, or setting up database connections. Follows convention where each table has an `id` column and foreign keys use `{table_name}_id` pattern.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# SQLAlchemy ORM Models

Expert guidance for creating SQLAlchemy models with consistent conventions and best practices.

## Naming Conventions

### Primary Keys
- Every table has an `id` column as the primary key
- Use `Integer` with `autoincrement=True` by default
- Use `String` or `UUID` for special cases

### Foreign Keys
- Follow the pattern: `{table_name}_id` (singular table name)
- Example: `experiments` table → `experiment_id` foreign key
- Example: `trials` table with reference to `experiments` → `experiment_id` column

### Table Names
- Use plural, lowercase, snake_case
- Examples: `users`, `experiments`, `trial_results`

### Column Names
- Use lowercase, snake_case
- Be descriptive: `created_at`, `is_active`, `email_verified`

## Installation

```bash
# Install with uv (recommended)
uv add sqlalchemy

# With async support
uv add sqlalchemy[asyncio]

# With specific database drivers
uv add sqlalchemy psycopg2-binary  # PostgreSQL
uv add sqlalchemy pymysql           # MySQL
```

## Basic Model Structure

### Single Model
```python
from datetime import datetime
from sqlalchemy import Integer, String, DateTime, Boolean
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column

class Base(DeclarativeBase):
    pass

class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(Integer, primary_key=True, autoincrement=True)
    username: Mapped[str] = mapped_column(String(50), unique=True, nullable=False)
    email: Mapped[str] = mapped_column(String(255), unique=True, nullable=False)
    is_active: Mapped[bool] = mapped_column(Boolean, default=True, nullable=False)
    created_at: Mapped[datetime] = mapped_column(DateTime, default=datetime.utcnow, nullable=False)
```

### Model with Foreign Key
```python
from sqlalchemy import Integer, String, Float, ForeignKey
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship

class Base(DeclarativeBase):
    pass

class Experiment(Base):
    __tablename__ = "experiments"

    id: Mapped[int] = mapped_column(Integer, primary_key=True, autoincrement=True)
    name: Mapped[str] = mapped_column(String(100), nullable=False)
    description: Mapped[str | None] = mapped_column(String(500))
    created_at: Mapped[datetime] = mapped_column(DateTime, default=datetime.utcnow)

    # Relationship (one-to-many)
    trials: Mapped[list["Trial"]] = relationship("Trial", back_populates="experiment")

class Trial(Base):
    __tablename__ = "trials"

    id: Mapped[int] = mapped_column(Integer, primary_key=True, autoincrement=True)
    experiment_id: Mapped[int] = mapped_column(ForeignKey("experiments.id"), nullable=False)
    trial_number: Mapped[int] = mapped_column(Integer, nullable=False)
    score: Mapped[float] = mapped_column(Float, nullable=False)
    created_at: Mapped[datetime] = mapped_column(DateTime, default=datetime.utcnow)

    # Relationship (many-to-one)
    experiment: Mapped["Experiment"] = relationship("Experiment", back_populates="trials")
```

## Common Patterns

### One-to-Many Relationship
```python
class Organization(Base):
    __tablename__ = "organizations"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    name: Mapped[str] = mapped_column(String(100), nullable=False)

    # One organization has many users
    users: Mapped[list["User"]] = relationship("User", back_populates="organization")

class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    username: Mapped[str] = mapped_column(String(50), nullable=False)
    organization_id: Mapped[int] = mapped_column(ForeignKey("organizations.id"), nullable=False)

    # Many users belong to one organization
    organization: Mapped["Organization"] = relationship("Organization", back_populates="users")
```

### Many-to-Many Relationship
```python
from sqlalchemy import Table, Column, Integer, ForeignKey

# Association table (no class needed)
project_members = Table(
    "project_members",
    Base.metadata,
    Column("project_id", Integer, ForeignKey("projects.id"), primary_key=True),
    Column("user_id", Integer, ForeignKey("users.id"), primary_key=True),
)

class Project(Base):
    __tablename__ = "projects"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    name: Mapped[str] = mapped_column(String(100), nullable=False)

    # Many projects have many users
    members: Mapped[list["User"]] = relationship(
        "User",
        secondary=project_members,
        back_populates="projects"
    )

class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    username: Mapped[str] = mapped_column(String(50), nullable=False)

    # Many users belong to many projects
    projects: Mapped[list["Project"]] = relationship(
        "Project",
        secondary=project_members,
        back_populates="members"
    )
```

### Self-Referential Relationship
```python
class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    username: Mapped[str] = mapped_column(String(50), nullable=False)
    manager_id: Mapped[int | None] = mapped_column(ForeignKey("users.id"))

    # Self-referential: users can have a manager (also a user)
    manager: Mapped["User | None"] = relationship(
        "User",
        remote_side="User.id",
        back_populates="direct_reports"
    )
    direct_reports: Mapped[list["User"]] = relationship(
        "User",
        back_populates="manager"
    )
```

### Cascade Deletes
```python
class Experiment(Base):
    __tablename__ = "experiments"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    name: Mapped[str] = mapped_column(String(100))

    # When experiment is deleted, all trials are also deleted
    trials: Mapped[list["Trial"]] = relationship(
        "Trial",
        back_populates="experiment",
        cascade="all, delete-orphan"
    )

class Trial(Base):
    __tablename__ = "trials"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    experiment_id: Mapped[int] = mapped_column(
        ForeignKey("experiments.id", ondelete="CASCADE"),
        nullable=False
    )

    experiment: Mapped["Experiment"] = relationship("Experiment", back_populates="trials")
```

## Column Types

### Common Column Types
```python
from sqlalchemy import Integer, String, Float, Boolean, DateTime, Date, Time, Text, JSON
from datetime import datetime, date, time

class Example(Base):
    __tablename__ = "examples"

    # Integers
    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    count: Mapped[int] = mapped_column(Integer, default=0)

    # Strings
    name: Mapped[str] = mapped_column(String(100))  # VARCHAR(100)
    description: Mapped[str | None] = mapped_column(Text)  # Unlimited text

    # Floats
    price: Mapped[float] = mapped_column(Float)

    # Boolean
    is_active: Mapped[bool] = mapped_column(Boolean, default=True)

    # Dates and times
    created_at: Mapped[datetime] = mapped_column(DateTime, default=datetime.utcnow)
    birth_date: Mapped[date | None] = mapped_column(Date)
    scheduled_time: Mapped[time | None] = mapped_column(Time)

    # JSON
    metadata_json: Mapped[dict] = mapped_column(JSON, default=dict)
```

### Enums
```python
from enum import Enum as PyEnum
from sqlalchemy import Enum

class StatusEnum(PyEnum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"

class Task(Base):
    __tablename__ = "tasks"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    status: Mapped[StatusEnum] = mapped_column(
        Enum(StatusEnum),
        default=StatusEnum.PENDING,
        nullable=False
    )
```

### UUIDs
```python
from sqlalchemy import String
from sqlalchemy.dialects.postgresql import UUID
import uuid

class Resource(Base):
    __tablename__ = "resources"

    # Using UUID as primary key (PostgreSQL)
    id: Mapped[uuid.UUID] = mapped_column(
        UUID(as_uuid=True),
        primary_key=True,
        default=uuid.uuid4
    )

    # Or using string for portability
    id_str: Mapped[str] = mapped_column(
        String(36),
        primary_key=True,
        default=lambda: str(uuid.uuid4())
    )
```

## Database Connection

### Synchronous Connection
```python
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session

# Create engine
engine = create_engine(
    "postgresql://user:password@localhost:5432/dbname",
    echo=True,  # Log SQL queries
    pool_pre_ping=True,  # Check connection health
)

# Create all tables
Base.metadata.create_all(engine)

# Create session factory
SessionLocal = sessionmaker(bind=engine, autoflush=False, autocommit=False)

# Use session
def get_db() -> Session:
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
```

### Async Connection (SQLAlchemy 2.0+)
```python
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker

# Create async engine
engine = create_async_engine(
    "postgresql+asyncpg://user:password@localhost:5432/dbname",
    echo=True,
)

# Create async session factory
AsyncSessionLocal = async_sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
)

# Use async session
async def get_db() -> AsyncSession:
    async with AsyncSessionLocal() as session:
        yield session
```

## CRUD Operations

### Create
```python
from sqlalchemy.orm import Session

def create_experiment(db: Session, name: str, description: str | None = None) -> Experiment:
    experiment = Experiment(name=name, description=description)
    db.add(experiment)
    db.commit()
    db.refresh(experiment)  # Load the generated id
    return experiment

def create_trial(db: Session, experiment_id: int, trial_number: int, score: float) -> Trial:
    trial = Trial(
        experiment_id=experiment_id,
        trial_number=trial_number,
        score=score,
    )
    db.add(trial)
    db.commit()
    db.refresh(trial)
    return trial
```

### Read
```python
from sqlalchemy import select

def get_experiment(db: Session, experiment_id: int) -> Experiment | None:
    return db.get(Experiment, experiment_id)

def get_experiments(db: Session, skip: int = 0, limit: int = 100) -> list[Experiment]:
    return db.execute(
        select(Experiment).offset(skip).limit(limit)
    ).scalars().all()

def get_trials_by_experiment(db: Session, experiment_id: int) -> list[Trial]:
    return db.execute(
        select(Trial).where(Trial.experiment_id == experiment_id)
    ).scalars().all()

# With relationships (eager loading)
def get_experiment_with_trials(db: Session, experiment_id: int) -> Experiment | None:
    from sqlalchemy.orm import selectinload

    return db.execute(
        select(Experiment)
        .where(Experiment.id == experiment_id)
        .options(selectinload(Experiment.trials))
    ).scalar_one_or_none()
```

### Update
```python
def update_experiment(db: Session, experiment_id: int, name: str) -> Experiment | None:
    experiment = db.get(Experiment, experiment_id)
    if experiment:
        experiment.name = name
        db.commit()
        db.refresh(experiment)
    return experiment

def update_trial_score(db: Session, trial_id: int, score: float) -> Trial | None:
    trial = db.get(Trial, trial_id)
    if trial:
        trial.score = score
        db.commit()
        db.refresh(trial)
    return trial
```

### Delete
```python
def delete_experiment(db: Session, experiment_id: int) -> bool:
    experiment = db.get(Experiment, experiment_id)
    if experiment:
        db.delete(experiment)
        db.commit()
        return True
    return False
```

## Advanced Queries

### Filtering
```python
from sqlalchemy import select, and_, or_

# Simple filter
def get_active_users(db: Session) -> list[User]:
    return db.execute(
        select(User).where(User.is_active == True)
    ).scalars().all()

# Multiple conditions
def search_users(db: Session, username: str | None, is_active: bool) -> list[User]:
    query = select(User)

    conditions = []
    if username:
        conditions.append(User.username.like(f"%{username}%"))
    conditions.append(User.is_active == is_active)

    return db.execute(query.where(and_(*conditions))).scalars().all()

# OR conditions
def get_admin_or_moderator_users(db: Session) -> list[User]:
    return db.execute(
        select(User).where(
            or_(User.role == "admin", User.role == "moderator")
        )
    ).scalars().all()
```

### Joins
```python
from sqlalchemy import select

def get_trials_with_experiment_name(db: Session) -> list[tuple[Trial, str]]:
    return db.execute(
        select(Trial, Experiment.name)
        .join(Experiment, Trial.experiment_id == Experiment.id)
    ).all()

# Using relationship
def get_high_score_trials(db: Session, min_score: float) -> list[Trial]:
    return db.execute(
        select(Trial)
        .join(Trial.experiment)
        .where(Trial.score >= min_score)
        .order_by(Trial.score.desc())
    ).scalars().all()
```

### Aggregations
```python
from sqlalchemy import func, select

def count_trials_by_experiment(db: Session, experiment_id: int) -> int:
    return db.execute(
        select(func.count(Trial.id)).where(Trial.experiment_id == experiment_id)
    ).scalar_one()

def get_average_score(db: Session, experiment_id: int) -> float:
    return db.execute(
        select(func.avg(Trial.score)).where(Trial.experiment_id == experiment_id)
    ).scalar_one()

def get_experiment_stats(db: Session, experiment_id: int) -> dict:
    result = db.execute(
        select(
            func.count(Trial.id).label("count"),
            func.avg(Trial.score).label("avg_score"),
            func.max(Trial.score).label("max_score"),
            func.min(Trial.score).label("min_score"),
        ).where(Trial.experiment_id == experiment_id)
    ).one()

    return {
        "count": result.count,
        "avg_score": result.avg_score,
        "max_score": result.max_score,
        "min_score": result.min_score,
    }
```

## Indexes and Constraints

```python
from sqlalchemy import Index, UniqueConstraint, CheckConstraint

class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    username: Mapped[str] = mapped_column(String(50), nullable=False)
    email: Mapped[str] = mapped_column(String(255), nullable=False)
    organization_id: Mapped[int] = mapped_column(ForeignKey("organizations.id"))
    age: Mapped[int | None] = mapped_column(Integer)

    # Table-level constraints
    __table_args__ = (
        # Unique constraint
        UniqueConstraint("username", "organization_id", name="uq_user_org"),

        # Check constraint
        CheckConstraint("age >= 0 AND age <= 150", name="ck_age_range"),

        # Composite index
        Index("ix_user_org_email", "organization_id", "email"),
    )

# Single column index (in mapped_column)
class Trial(Base):
    __tablename__ = "trials"

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    experiment_id: Mapped[int] = mapped_column(
        ForeignKey("experiments.id"),
        index=True,  # Create index on this column
        nullable=False
    )
    created_at: Mapped[datetime] = mapped_column(DateTime, index=True)
```

## Mixins for Common Patterns

```python
from datetime import datetime
from sqlalchemy import Integer, DateTime
from sqlalchemy.orm import Mapped, mapped_column, DeclarativeBase

class Base(DeclarativeBase):
    pass

# Mixin for id column
class IdMixin:
    id: Mapped[int] = mapped_column(Integer, primary_key=True, autoincrement=True)

# Mixin for timestamps
class TimestampMixin:
    created_at: Mapped[datetime] = mapped_column(
        DateTime,
        default=datetime.utcnow,
        nullable=False
    )
    updated_at: Mapped[datetime] = mapped_column(
        DateTime,
        default=datetime.utcnow,
        onupdate=datetime.utcnow,
        nullable=False
    )

# Use mixins
class User(IdMixin, TimestampMixin, Base):
    __tablename__ = "users"

    username: Mapped[str] = mapped_column(String(50), nullable=False)
    email: Mapped[str] = mapped_column(String(255), nullable=False)

class Experiment(IdMixin, TimestampMixin, Base):
    __tablename__ = "experiments"

    name: Mapped[str] = mapped_column(String(100), nullable=False)
```

## Alembic Migrations

```bash
# Install Alembic
uv add alembic

# Initialize Alembic
alembic init alembic

# Edit alembic.ini - set sqlalchemy.url
# Edit alembic/env.py - import your Base and models

# Create migration
alembic revision --autogenerate -m "Create users table"

# Apply migrations
alembic upgrade head

# Downgrade
alembic downgrade -1

# Check current version
alembic current

# View history
alembic history
```

### Alembic env.py Setup
```python
# alembic/env.py
from logging.config import fileConfig
from sqlalchemy import engine_from_config, pool
from alembic import context

# Import your models
from myapp.models import Base
from myapp.database import DATABASE_URL

config = context.config
config.set_main_option("sqlalchemy.url", DATABASE_URL)

target_metadata = Base.metadata

def run_migrations_online():
    connectable = engine_from_config(
        config.get_section(config.config_ini_section),
        prefix="sqlalchemy.",
        poolclass=pool.NullPool,
    )

    with connectable.connect() as connection:
        context.configure(connection=connection, target_metadata=target_metadata)
        with context.begin_transaction():
            context.run_migrations()

run_migrations_online()
```

## Project Structure

```
myproject/
├── alembic/
│   ├── versions/
│   │   └── 001_create_users.py
│   ├── env.py
│   └── script.py.mako
├── src/
│   └── myapp/
│       ├── __init__.py
│       ├── database.py        # Engine, sessionmaker
│       ├── models/
│       │   ├── __init__.py
│       │   ├── base.py        # Base class, mixins
│       │   ├── user.py        # User model
│       │   └── experiment.py  # Experiment, Trial models
│       ├── crud/
│       │   ├── __init__.py
│       │   ├── user.py        # User CRUD operations
│       │   └── experiment.py  # Experiment CRUD operations
│       └── schemas/
│           ├── __init__.py
│           ├── user.py        # Pydantic schemas for User
│           └── experiment.py  # Pydantic schemas for Experiment
├── alembic.ini
├── pyproject.toml
└── README.md
```

## Best Practices

1. **Use Mapped[] and mapped_column()**: Modern SQLAlchemy 2.0 style with type hints
2. **Follow naming conventions**: `{table_name}_id` for foreign keys, plural table names
3. **Always use autoincrement=True**: For integer primary keys
4. **Use relationship() for both sides**: Define bidirectional relationships with `back_populates`
5. **Set nullable appropriately**: Be explicit about which columns allow NULL
6. **Use indexes**: Add indexes on foreign keys and frequently queried columns
7. **Eager load relationships**: Use `selectinload()` or `joinedload()` to avoid N+1 queries
8. **Use transactions**: Commit or rollback as a unit of work
9. **Use Alembic**: Manage schema changes with migrations, not `create_all()`
10. **Separate concerns**: Models in one place, CRUD in another, schemas (Pydantic) separate

## Common Mistakes to Avoid

1. **Forgetting ForeignKey**: Always specify `ForeignKey()` on foreign key columns
2. **Not using back_populates**: Bidirectional relationships need both sides defined
3. **Lazy loading in loops**: Causes N+1 queries; use eager loading
4. **Forgetting db.commit()**: Changes aren't persisted without commit
5. **Not handling None**: Use `| None` type hints for nullable columns
6. **Using create_all() in production**: Use Alembic migrations instead
7. **Not setting cascade**: Orphaned records when parent is deleted
8. **Hardcoding connection strings**: Use environment variables

## Integration with Pydantic

```python
from pydantic import BaseModel, ConfigDict
from datetime import datetime

# Pydantic schemas for API
class TrialCreate(BaseModel):
    experiment_id: int
    trial_number: int
    score: float

class TrialResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    experiment_id: int
    trial_number: int
    score: float
    created_at: datetime

class ExperimentResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    name: str
    description: str | None
    created_at: datetime
    trials: list[TrialResponse] = []

# Convert SQLAlchemy model to Pydantic
def get_experiment_response(db: Session, experiment_id: int) -> ExperimentResponse:
    experiment = get_experiment_with_trials(db, experiment_id)
    return ExperimentResponse.model_validate(experiment)
```
