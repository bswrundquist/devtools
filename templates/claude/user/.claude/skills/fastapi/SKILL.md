---
name: fastapi
description: Use when building FastAPI applications with Pydantic schemas, dependency injection, SQLAlchemy integration, external clients, and Typer CLI for deployment. Follows best practices for project structure, async/await, and production deployment.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# FastAPI Development

Expert guidance for building production-ready FastAPI applications with Pydantic, SQLAlchemy, dependency injection, and Typer CLI deployment.

## Installation

```bash
# Core dependencies
uv add fastapi uvicorn[standard]

# With SQLAlchemy
uv add fastapi uvicorn[standard] sqlalchemy

# With async SQLAlchemy
uv add fastapi uvicorn[standard] sqlalchemy[asyncio] asyncpg

# CLI deployment tool
uv add typer

# Common additional packages
uv add pydantic pydantic-settings python-multipart python-jose[cryptography] passlib[bcrypt]

# Development dependencies
uv add --dev pytest pytest-asyncio httpx
```

## Project Structure

```
myapp/
├── src/
│   └── myapp/
│       ├── __init__.py
│       ├── main.py              # FastAPI app instance
│       ├── config.py            # Settings with pydantic-settings
│       ├── cli.py               # Typer CLI for deployment
│       ├── api/
│       │   ├── __init__.py
│       │   ├── deps.py          # Dependency injection
│       │   └── routes/
│       │       ├── __init__.py
│       │       ├── users.py     # User endpoints
│       │       ├── experiments.py
│       │       └── health.py    # Health check
│       ├── schemas/
│       │   ├── __init__.py
│       │   ├── user.py          # Pydantic models for users
│       │   ├── experiment.py    # Pydantic models for experiments
│       │   └── common.py        # Shared schemas (pagination, etc.)
│       ├── models/
│       │   ├── __init__.py
│       │   ├── base.py          # SQLAlchemy Base
│       │   ├── user.py          # SQLAlchemy User model
│       │   └── experiment.py    # SQLAlchemy Experiment model
│       ├── crud/
│       │   ├── __init__.py
│       │   ├── base.py          # Generic CRUD class
│       │   ├── user.py          # User CRUD operations
│       │   └── experiment.py    # Experiment CRUD operations
│       ├── clients/
│       │   ├── __init__.py
│       │   ├── base.py          # Base client class
│       │   ├── stripe.py        # Stripe API client
│       │   ├── sendgrid.py      # SendGrid email client
│       │   └── s3.py            # AWS S3 client
│       ├── database.py          # Database connection
│       └── exceptions.py        # Custom exceptions
├── tests/
│   ├── __init__.py
│   ├── conftest.py              # Pytest fixtures
│   ├── test_api/
│   │   ├── test_users.py
│   │   └── test_experiments.py
│   └── test_clients/
│       └── test_stripe.py
├── alembic/
│   ├── versions/
│   └── env.py
├── alembic.ini
├── pyproject.toml
├── .env.example
└── README.md
```

## Configuration with Pydantic Settings

```python
# src/myapp/config.py
from functools import lru_cache
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
        extra="ignore",
    )

    # App settings
    app_name: str = "MyApp"
    debug: bool = False
    version: str = "1.0.0"
    api_prefix: str = "/api/v1"

    # Server settings
    host: str = "0.0.0.0"
    port: int = 8000
    workers: int = 4
    reload: bool = False

    # Database
    database_url: str
    db_echo: bool = False

    # Security
    secret_key: str
    algorithm: str = "HS256"
    access_token_expire_minutes: int = 30

    # External services
    stripe_api_key: str | None = None
    sendgrid_api_key: str | None = None
    aws_access_key_id: str | None = None
    aws_secret_access_key: str | None = None
    s3_bucket_name: str | None = None

    # CORS
    cors_origins: list[str] = ["http://localhost:3000"]

@lru_cache()
def get_settings() -> Settings:
    return Settings()
```

## Database Setup

```python
# src/myapp/database.py
from sqlalchemy import create_engine
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker
from sqlalchemy.orm import sessionmaker, Session
from .config import get_settings

settings = get_settings()

# Sync engine
engine = create_engine(
    settings.database_url,
    echo=settings.db_echo,
    pool_pre_ping=True,
)

SessionLocal = sessionmaker(
    bind=engine,
    autocommit=False,
    autoflush=False,
)

# Async engine (if using asyncpg)
async_engine = create_async_engine(
    settings.database_url.replace("postgresql://", "postgresql+asyncpg://"),
    echo=settings.db_echo,
    pool_pre_ping=True,
)

AsyncSessionLocal = async_sessionmaker(
    async_engine,
    class_=AsyncSession,
    expire_on_commit=False,
)
```

## Pydantic Schemas

### Common Schemas
```python
# src/myapp/schemas/common.py
from pydantic import BaseModel, Field

class PaginationParams(BaseModel):
    skip: int = Field(0, ge=0, description="Number of records to skip")
    limit: int = Field(100, ge=1, le=1000, description="Max number of records to return")

class PaginatedResponse(BaseModel):
    items: list
    total: int
    skip: int
    limit: int
    has_more: bool

class MessageResponse(BaseModel):
    message: str

class ErrorResponse(BaseModel):
    detail: str
```

### Entity Schemas
```python
# src/myapp/schemas/user.py
from pydantic import BaseModel, EmailStr, Field, ConfigDict
from datetime import datetime

# Base schema (shared fields)
class UserBase(BaseModel):
    username: str = Field(..., min_length=3, max_length=50)
    email: EmailStr
    is_active: bool = True

# Schema for creating a user (input)
class UserCreate(UserBase):
    password: str = Field(..., min_length=8, max_length=100)

# Schema for updating a user (input, all optional)
class UserUpdate(BaseModel):
    username: str | None = Field(None, min_length=3, max_length=50)
    email: EmailStr | None = None
    is_active: bool | None = None
    password: str | None = Field(None, min_length=8, max_length=100)

# Schema for user in database (output)
class User(UserBase):
    model_config = ConfigDict(from_attributes=True)

    id: int
    created_at: datetime
    updated_at: datetime

# Schema without sensitive data for public response
class UserPublic(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    username: str
    is_active: bool
```

```python
# src/myapp/schemas/experiment.py
from pydantic import BaseModel, Field, ConfigDict
from datetime import datetime

class TrialBase(BaseModel):
    trial_number: int = Field(..., ge=1)
    score: float

class TrialCreate(TrialBase):
    experiment_id: int

class TrialUpdate(BaseModel):
    score: float | None = None

class Trial(TrialBase):
    model_config = ConfigDict(from_attributes=True)

    id: int
    experiment_id: int
    created_at: datetime

class ExperimentBase(BaseModel):
    name: str = Field(..., min_length=1, max_length=100)
    description: str | None = Field(None, max_length=500)

class ExperimentCreate(ExperimentBase):
    pass

class ExperimentUpdate(BaseModel):
    name: str | None = Field(None, min_length=1, max_length=100)
    description: str | None = Field(None, max_length=500)

class Experiment(ExperimentBase):
    model_config = ConfigDict(from_attributes=True)

    id: int
    created_at: datetime
    updated_at: datetime

class ExperimentWithTrials(Experiment):
    trials: list[Trial] = []
```

## Dependency Injection

```python
# src/myapp/api/deps.py
from typing import Annotated, AsyncGenerator
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from sqlalchemy.orm import Session
from sqlalchemy.ext.asyncio import AsyncSession

from ..database import SessionLocal, AsyncSessionLocal
from ..config import get_settings, Settings
from ..models.user import User
from ..crud.user import user_crud

# Settings dependency
def get_settings_dep() -> Settings:
    return get_settings()

# Database dependencies
def get_db() -> Generator[Session, None, None]:
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

async def get_async_db() -> AsyncGenerator[AsyncSession, None]:
    async with AsyncSessionLocal() as session:
        yield session

# Type aliases for cleaner route signatures
DBSession = Annotated[Session, Depends(get_db)]
AsyncDBSession = Annotated[AsyncSession, Depends(get_async_db)]
SettingsDep = Annotated[Settings, Depends(get_settings_dep)]

# Authentication dependencies
security = HTTPBearer()

async def get_current_user(
    db: DBSession,
    credentials: Annotated[HTTPAuthorizationCredentials, Depends(security)],
    settings: SettingsDep,
) -> User:
    """Get current authenticated user from JWT token"""
    from jose import jwt, JWTError

    token = credentials.credentials
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )

    try:
        payload = jwt.decode(token, settings.secret_key, algorithms=[settings.algorithm])
        user_id: int | None = payload.get("sub")
        if user_id is None:
            raise credentials_exception
    except JWTError:
        raise credentials_exception

    user = user_crud.get(db, id=int(user_id))
    if user is None:
        raise credentials_exception

    return user

async def get_current_active_user(
    current_user: Annotated[User, Depends(get_current_user)],
) -> User:
    """Ensure user is active"""
    if not current_user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Inactive user",
        )
    return current_user

# Type aliases for auth dependencies
CurrentUser = Annotated[User, Depends(get_current_user)]
CurrentActiveUser = Annotated[User, Depends(get_current_active_user)]

# Pagination dependency
def get_pagination(
    skip: int = 0,
    limit: int = 100,
) -> dict[str, int]:
    """Pagination parameters"""
    return {"skip": max(0, skip), "limit": min(1000, max(1, limit))}

PaginationDep = Annotated[dict[str, int], Depends(get_pagination)]
```

## CRUD Base Class

```python
# src/myapp/crud/base.py
from typing import Generic, TypeVar, Type, Any
from pydantic import BaseModel
from sqlalchemy.orm import Session
from sqlalchemy import select

from ..models.base import Base

ModelType = TypeVar("ModelType", bound=Base)
CreateSchemaType = TypeVar("CreateSchemaType", bound=BaseModel)
UpdateSchemaType = TypeVar("UpdateSchemaType", bound=BaseModel)

class CRUDBase(Generic[ModelType, CreateSchemaType, UpdateSchemaType]):
    def __init__(self, model: Type[ModelType]):
        self.model = model

    def get(self, db: Session, id: int) -> ModelType | None:
        return db.get(self.model, id)

    def get_multi(
        self, db: Session, *, skip: int = 0, limit: int = 100
    ) -> list[ModelType]:
        return db.execute(
            select(self.model).offset(skip).limit(limit)
        ).scalars().all()

    def create(self, db: Session, *, obj_in: CreateSchemaType) -> ModelType:
        obj_data = obj_in.model_dump()
        db_obj = self.model(**obj_data)
        db.add(db_obj)
        db.commit()
        db.refresh(db_obj)
        return db_obj

    def update(
        self,
        db: Session,
        *,
        db_obj: ModelType,
        obj_in: UpdateSchemaType | dict[str, Any]
    ) -> ModelType:
        if isinstance(obj_in, dict):
            update_data = obj_in
        else:
            update_data = obj_in.model_dump(exclude_unset=True)

        for field, value in update_data.items():
            setattr(db_obj, field, value)

        db.add(db_obj)
        db.commit()
        db.refresh(db_obj)
        return db_obj

    def delete(self, db: Session, *, id: int) -> ModelType | None:
        obj = db.get(self.model, id)
        if obj:
            db.delete(obj)
            db.commit()
        return obj

    def count(self, db: Session) -> int:
        from sqlalchemy import func
        return db.execute(select(func.count(self.model.id))).scalar_one()
```

```python
# src/myapp/crud/user.py
from sqlalchemy.orm import Session
from sqlalchemy import select
from passlib.context import CryptContext

from .base import CRUDBase
from ..models.user import User
from ..schemas.user import UserCreate, UserUpdate

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

class CRUDUser(CRUDBase[User, UserCreate, UserUpdate]):
    def get_by_email(self, db: Session, *, email: str) -> User | None:
        return db.execute(
            select(User).where(User.email == email)
        ).scalar_one_or_none()

    def get_by_username(self, db: Session, *, username: str) -> User | None:
        return db.execute(
            select(User).where(User.username == username)
        ).scalar_one_or_none()

    def create(self, db: Session, *, obj_in: UserCreate) -> User:
        db_obj = User(
            username=obj_in.username,
            email=obj_in.email,
            hashed_password=pwd_context.hash(obj_in.password),
            is_active=obj_in.is_active,
        )
        db.add(db_obj)
        db.commit()
        db.refresh(db_obj)
        return db_obj

    def authenticate(self, db: Session, *, email: str, password: str) -> User | None:
        user = self.get_by_email(db, email=email)
        if not user:
            return None
        if not pwd_context.verify(password, user.hashed_password):
            return None
        return user

user_crud = CRUDUser(User)
```

## External Clients

### Base Client
```python
# src/myapp/clients/base.py
from abc import ABC, abstractmethod
import httpx
from typing import Any

class BaseClient(ABC):
    """Base class for external API clients"""

    def __init__(self, api_key: str | None = None, base_url: str | None = None):
        self.api_key = api_key
        self.base_url = base_url
        self._client: httpx.AsyncClient | None = None

    async def __aenter__(self):
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()

    async def connect(self) -> None:
        """Initialize the HTTP client"""
        headers = self._get_headers()
        self._client = httpx.AsyncClient(
            base_url=self.base_url,
            headers=headers,
            timeout=30.0,
        )

    async def close(self) -> None:
        """Close the HTTP client"""
        if self._client:
            await self._client.aclose()
            self._client = None

    @abstractmethod
    def _get_headers(self) -> dict[str, str]:
        """Get headers for requests"""
        pass

    async def _request(
        self,
        method: str,
        endpoint: str,
        **kwargs: Any
    ) -> httpx.Response:
        """Make HTTP request"""
        if not self._client:
            await self.connect()

        response = await self._client.request(method, endpoint, **kwargs)
        response.raise_for_status()
        return response
```

### Stripe Client
```python
# src/myapp/clients/stripe.py
from typing import Any
from .base import BaseClient

class StripeClient(BaseClient):
    """Stripe API client"""

    def __init__(self, api_key: str):
        super().__init__(api_key=api_key, base_url="https://api.stripe.com/v1")

    def _get_headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/x-www-form-urlencoded",
        }

    async def create_customer(self, email: str, name: str) -> dict[str, Any]:
        """Create a Stripe customer"""
        response = await self._request(
            "POST",
            "/customers",
            data={"email": email, "name": name},
        )
        return response.json()

    async def create_payment_intent(
        self,
        amount: int,
        currency: str = "usd",
        customer_id: str | None = None,
    ) -> dict[str, Any]:
        """Create a payment intent"""
        data = {"amount": amount, "currency": currency}
        if customer_id:
            data["customer"] = customer_id

        response = await self._request("POST", "/payment_intents", data=data)
        return response.json()

    async def get_customer(self, customer_id: str) -> dict[str, Any]:
        """Get customer details"""
        response = await self._request("GET", f"/customers/{customer_id}")
        return response.json()
```

### SendGrid Client
```python
# src/myapp/clients/sendgrid.py
from typing import Any
from .base import BaseClient

class SendGridClient(BaseClient):
    """SendGrid email client"""

    def __init__(self, api_key: str):
        super().__init__(api_key=api_key, base_url="https://api.sendgrid.com/v3")

    def _get_headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
        }

    async def send_email(
        self,
        to_email: str,
        subject: str,
        html_content: str,
        from_email: str = "noreply@example.com",
    ) -> dict[str, Any]:
        """Send an email"""
        payload = {
            "personalizations": [{"to": [{"email": to_email}]}],
            "from": {"email": from_email},
            "subject": subject,
            "content": [{"type": "text/html", "value": html_content}],
        }

        response = await self._request("POST", "/mail/send", json=payload)
        return {"status": "sent", "status_code": response.status_code}
```

### Client Dependencies
```python
# src/myapp/api/deps.py (additions)
from ..clients.stripe import StripeClient
from ..clients.sendgrid import SendGridClient

async def get_stripe_client(settings: SettingsDep) -> StripeClient:
    """Get Stripe client"""
    if not settings.stripe_api_key:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Stripe not configured",
        )
    return StripeClient(api_key=settings.stripe_api_key)

async def get_sendgrid_client(settings: SettingsDep) -> SendGridClient:
    """Get SendGrid client"""
    if not settings.sendgrid_api_key:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="SendGrid not configured",
        )
    return SendGridClient(api_key=settings.sendgrid_api_key)

StripeClientDep = Annotated[StripeClient, Depends(get_stripe_client)]
SendGridClientDep = Annotated[SendGridClient, Depends(get_sendgrid_client)]
```

## API Routes

### Health Check
```python
# src/myapp/api/routes/health.py
from fastapi import APIRouter
from ...schemas.common import MessageResponse

router = APIRouter()

@router.get("/health", response_model=MessageResponse)
async def health_check():
    """Health check endpoint"""
    return {"message": "OK"}
```

### User Routes
```python
# src/myapp/api/routes/users.py
from fastapi import APIRouter, HTTPException, status
from typing import Any

from ...api.deps import DBSession, CurrentActiveUser, PaginationDep
from ...schemas.user import User, UserCreate, UserUpdate, UserPublic
from ...schemas.common import PaginatedResponse
from ...crud.user import user_crud

router = APIRouter(prefix="/users", tags=["users"])

@router.post("/", response_model=User, status_code=status.HTTP_201_CREATED)
def create_user(*, db: DBSession, user_in: UserCreate) -> Any:
    """Create new user"""
    # Check if email already exists
    user = user_crud.get_by_email(db, email=user_in.email)
    if user:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Email already registered",
        )

    # Check if username already exists
    user = user_crud.get_by_username(db, username=user_in.username)
    if user:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Username already taken",
        )

    user = user_crud.create(db, obj_in=user_in)
    return user

@router.get("/", response_model=list[UserPublic])
def list_users(
    db: DBSession,
    pagination: PaginationDep,
    current_user: CurrentActiveUser,
) -> Any:
    """List all users (requires authentication)"""
    users = user_crud.get_multi(db, **pagination)
    return users

@router.get("/me", response_model=User)
def get_current_user_info(current_user: CurrentActiveUser) -> Any:
    """Get current user info"""
    return current_user

@router.get("/{user_id}", response_model=UserPublic)
def get_user(*, db: DBSession, user_id: int) -> Any:
    """Get user by ID"""
    user = user_crud.get(db, id=user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found",
        )
    return user

@router.patch("/{user_id}", response_model=User)
def update_user(
    *,
    db: DBSession,
    user_id: int,
    user_in: UserUpdate,
    current_user: CurrentActiveUser,
) -> Any:
    """Update user (only own profile or admin)"""
    user = user_crud.get(db, id=user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found",
        )

    # Check permissions (only allow updating own profile for now)
    if user.id != current_user.id:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not enough permissions",
        )

    user = user_crud.update(db, db_obj=user, obj_in=user_in)
    return user

@router.delete("/{user_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_user(
    *,
    db: DBSession,
    user_id: int,
    current_user: CurrentActiveUser,
) -> None:
    """Delete user"""
    user = user_crud.get(db, id=user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found",
        )

    # Check permissions
    if user.id != current_user.id:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not enough permissions",
        )

    user_crud.delete(db, id=user_id)
```

### Experiment Routes with Client Usage
```python
# src/myapp/api/routes/experiments.py
from fastapi import APIRouter, HTTPException, status, BackgroundTasks
from typing import Any

from ...api.deps import DBSession, CurrentActiveUser, SendGridClientDep
from ...schemas.experiment import (
    Experiment,
    ExperimentCreate,
    ExperimentUpdate,
    ExperimentWithTrials,
)
from ...crud.experiment import experiment_crud

router = APIRouter(prefix="/experiments", tags=["experiments"])

async def send_experiment_created_email(
    email_client: SendGridClient,
    user_email: str,
    experiment_name: str,
) -> None:
    """Background task to send email notification"""
    await email_client.send_email(
        to_email=user_email,
        subject=f"Experiment Created: {experiment_name}",
        html_content=f"<p>Your experiment <strong>{experiment_name}</strong> has been created.</p>",
    )

@router.post("/", response_model=Experiment, status_code=status.HTTP_201_CREATED)
async def create_experiment(
    *,
    db: DBSession,
    experiment_in: ExperimentCreate,
    current_user: CurrentActiveUser,
    email_client: SendGridClientDep,
    background_tasks: BackgroundTasks,
) -> Any:
    """Create new experiment"""
    experiment = experiment_crud.create(db, obj_in=experiment_in)

    # Send email notification in background
    async with email_client:
        background_tasks.add_task(
            send_experiment_created_email,
            email_client,
            current_user.email,
            experiment.name,
        )

    return experiment

@router.get("/", response_model=list[Experiment])
def list_experiments(
    db: DBSession,
    skip: int = 0,
    limit: int = 100,
) -> Any:
    """List all experiments"""
    experiments = experiment_crud.get_multi(db, skip=skip, limit=limit)
    return experiments

@router.get("/{experiment_id}", response_model=ExperimentWithTrials)
def get_experiment(*, db: DBSession, experiment_id: int) -> Any:
    """Get experiment by ID with trials"""
    experiment = experiment_crud.get_with_trials(db, id=experiment_id)
    if not experiment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Experiment not found",
        )
    return experiment

@router.patch("/{experiment_id}", response_model=Experiment)
def update_experiment(
    *,
    db: DBSession,
    experiment_id: int,
    experiment_in: ExperimentUpdate,
) -> Any:
    """Update experiment"""
    experiment = experiment_crud.get(db, id=experiment_id)
    if not experiment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Experiment not found",
        )

    experiment = experiment_crud.update(db, db_obj=experiment, obj_in=experiment_in)
    return experiment

@router.delete("/{experiment_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_experiment(*, db: DBSession, experiment_id: int) -> None:
    """Delete experiment"""
    experiment = experiment_crud.get(db, id=experiment_id)
    if not experiment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Experiment not found",
        )

    experiment_crud.delete(db, id=experiment_id)
```

## Main FastAPI App

```python
# src/myapp/main.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from .config import get_settings
from .api.routes import users, experiments, health
from .exceptions import AppException

settings = get_settings()

app = FastAPI(
    title=settings.app_name,
    version=settings.version,
    debug=settings.debug,
    docs_url=f"{settings.api_prefix}/docs",
    redoc_url=f"{settings.api_prefix}/redoc",
    openapi_url=f"{settings.api_prefix}/openapi.json",
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Exception handlers
@app.exception_handler(AppException)
async def app_exception_handler(request, exc: AppException):
    return JSONResponse(
        status_code=exc.status_code,
        content={"detail": exc.detail},
    )

# Include routers
app.include_router(health.router, prefix=settings.api_prefix)
app.include_router(users.router, prefix=settings.api_prefix)
app.include_router(experiments.router, prefix=settings.api_prefix)

@app.get("/")
async def root():
    """Root endpoint"""
    return {
        "app": settings.app_name,
        "version": settings.version,
        "docs": f"{settings.api_prefix}/docs",
    }

@app.on_event("startup")
async def startup_event():
    """Run on startup"""
    print(f"Starting {settings.app_name} v{settings.version}")

@app.on_event("shutdown")
async def shutdown_event():
    """Run on shutdown"""
    print(f"Shutting down {settings.app_name}")
```

## Typer CLI for Deployment

```python
# src/myapp/cli.py
import typer
import uvicorn
from pathlib import Path
from typing import Optional

from .config import get_settings

app = typer.Typer(
    name="myapp",
    help="MyApp CLI - Deploy and manage the FastAPI application",
    add_completion=False,
)

@app.command()
def serve(
    host: str = typer.Option(
        None,
        "--host",
        "-h",
        help="Host to bind to (default: from config)",
    ),
    port: int = typer.Option(
        None,
        "--port",
        "-p",
        help="Port to bind to (default: from config)",
    ),
    workers: int = typer.Option(
        None,
        "--workers",
        "-w",
        help="Number of worker processes (default: from config)",
    ),
    reload: bool = typer.Option(
        False,
        "--reload",
        "-r",
        help="Enable auto-reload on code changes (dev only)",
    ),
    log_level: str = typer.Option(
        "info",
        "--log-level",
        "-l",
        help="Log level (debug, info, warning, error, critical)",
    ),
    access_log: bool = typer.Option(
        True,
        "--access-log/--no-access-log",
        help="Enable/disable access log",
    ),
):
    """
    Start the FastAPI server with uvicorn.

    Examples:
        # Use defaults from config
        myapp serve

        # Custom port and enable reload
        myapp serve --port 8080 --reload

        # Production with 4 workers
        myapp serve --workers 4 --no-access-log

        # Debug mode
        myapp serve --reload --log-level debug
    """
    settings = get_settings()

    # Use CLI args or fall back to config
    final_host = host or settings.host
    final_port = port or settings.port
    final_workers = workers or settings.workers if not reload else 1

    typer.echo(f"🚀 Starting {settings.app_name} v{settings.version}")
    typer.echo(f"📍 Server: http://{final_host}:{final_port}")
    typer.echo(f"📚 Docs: http://{final_host}:{final_port}{settings.api_prefix}/docs")
    typer.echo(f"👷 Workers: {final_workers}")
    typer.echo(f"🔄 Reload: {reload}")
    typer.echo(f"📊 Log level: {log_level}")

    uvicorn.run(
        "myapp.main:app",
        host=final_host,
        port=final_port,
        workers=final_workers,
        reload=reload,
        log_level=log_level,
        access_log=access_log,
    )

@app.command()
def dev():
    """
    Start server in development mode (reload enabled, debug logging).

    Equivalent to: serve --reload --log-level debug --workers 1
    """
    settings = get_settings()

    typer.echo("🔧 Starting in DEVELOPMENT mode")
    typer.echo(f"📍 Server: http://{settings.host}:{settings.port}")
    typer.echo(f"📚 Docs: http://{settings.host}:{settings.port}{settings.api_prefix}/docs")

    uvicorn.run(
        "myapp.main:app",
        host=settings.host,
        port=settings.port,
        reload=True,
        log_level="debug",
    )

@app.command()
def prod(
    workers: int = typer.Option(
        None,
        "--workers",
        "-w",
        help="Number of worker processes (default: from config)",
    ),
):
    """
    Start server in production mode (no reload, optimized settings).

    Uses multiple workers for better performance.
    """
    settings = get_settings()
    final_workers = workers or settings.workers

    typer.echo("🚀 Starting in PRODUCTION mode")
    typer.echo(f"📍 Server: http://{settings.host}:{settings.port}")
    typer.echo(f"👷 Workers: {final_workers}")

    uvicorn.run(
        "myapp.main:app",
        host=settings.host,
        port=settings.port,
        workers=final_workers,
        log_level="warning",
        access_log=False,
    )

@app.command()
def info():
    """Display application configuration and environment info."""
    settings = get_settings()

    typer.echo("ℹ️  Application Information")
    typer.echo(f"\nApp Name: {settings.app_name}")
    typer.echo(f"Version: {settings.version}")
    typer.echo(f"Debug: {settings.debug}")
    typer.echo(f"\n🌐 Server Settings")
    typer.echo(f"Host: {settings.host}")
    typer.echo(f"Port: {settings.port}")
    typer.echo(f"Workers: {settings.workers}")
    typer.echo(f"API Prefix: {settings.api_prefix}")
    typer.echo(f"\n🗄️  Database")
    typer.echo(f"URL: {settings.database_url.split('@')[-1] if '@' in settings.database_url else 'Not configured'}")
    typer.echo(f"Echo SQL: {settings.db_echo}")
    typer.echo(f"\n🔐 Security")
    typer.echo(f"Algorithm: {settings.algorithm}")
    typer.echo(f"Token Expiry: {settings.access_token_expire_minutes} minutes")
    typer.echo(f"\n🌍 CORS Origins")
    for origin in settings.cors_origins:
        typer.echo(f"  - {origin}")

@app.command()
def db_create():
    """Create all database tables (for development only, use Alembic in production)."""
    from .database import engine
    from .models.base import Base

    typer.echo("Creating database tables...")
    Base.metadata.create_all(engine)
    typer.echo("✅ Tables created successfully")

@app.command()
def db_drop():
    """Drop all database tables (DANGER: This will delete all data!)."""
    if typer.confirm("⚠️  This will DELETE ALL DATA. Are you sure?"):
        from .database import engine
        from .models.base import Base

        typer.echo("Dropping all tables...")
        Base.metadata.drop_all(engine)
        typer.echo("✅ Tables dropped")
    else:
        typer.echo("Aborted")

def main():
    """Entry point for CLI"""
    app()

if __name__ == "__main__":
    main()
```

## Setup Entry Point

```toml
# pyproject.toml
[project]
name = "myapp"
version = "1.0.0"
dependencies = [
    "fastapi>=0.109.0",
    "uvicorn[standard]>=0.27.0",
    "sqlalchemy>=2.0.0",
    "pydantic>=2.0.0",
    "pydantic-settings>=2.0.0",
    "typer>=0.9.0",
    "python-jose[cryptography]>=3.3.0",
    "passlib[bcrypt]>=1.7.4",
    "python-multipart>=0.0.6",
    "httpx>=0.26.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.4.0",
    "pytest-asyncio>=0.21.0",
    "httpx>=0.26.0",
    "alembic>=1.13.0",
]

[project.scripts]
myapp = "myapp.cli:main"

[tool.uv]
dev-dependencies = [
    "pytest>=7.4.0",
    "pytest-asyncio>=0.21.0",
]
```

## CLI Usage

```bash
# Install the package
uv pip install -e .

# Development mode (auto-reload)
myapp dev

# Production mode (multi-worker)
myapp prod

# Custom settings
myapp serve --host 0.0.0.0 --port 8080 --workers 4

# Development with custom port
myapp serve --port 3000 --reload --log-level debug

# Production with specific workers
myapp prod --workers 8

# View configuration
myapp info

# Database commands (dev only)
myapp db-create
myapp db-drop

# Help
myapp --help
myapp serve --help
```

## Testing

```python
# tests/conftest.py
import pytest
from fastapi.testclient import TestClient
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from sqlalchemy.pool import StaticPool

from myapp.main import app
from myapp.database import Base
from myapp.api.deps import get_db

# In-memory SQLite for tests
SQLALCHEMY_TEST_DATABASE_URL = "sqlite:///:memory:"

engine = create_engine(
    SQLALCHEMY_TEST_DATABASE_URL,
    connect_args={"check_same_thread": False},
    poolclass=StaticPool,
)
TestingSessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

@pytest.fixture
def db():
    """Create a fresh database for each test"""
    Base.metadata.create_all(bind=engine)
    db = TestingSessionLocal()
    try:
        yield db
    finally:
        db.close()
        Base.metadata.drop_all(bind=engine)

@pytest.fixture
def client(db):
    """Test client with database override"""
    def override_get_db():
        try:
            yield db
        finally:
            pass

    app.dependency_overrides[get_db] = override_get_db
    with TestClient(app) as test_client:
        yield test_client
    app.dependency_overrides.clear()
```

```python
# tests/test_api/test_users.py
from fastapi.testclient import TestClient

def test_create_user(client: TestClient):
    response = client.post(
        "/api/v1/users/",
        json={
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123",
        },
    )
    assert response.status_code == 201
    data = response.json()
    assert data["username"] == "testuser"
    assert data["email"] == "test@example.com"
    assert "id" in data

def test_get_user(client: TestClient):
    # Create user first
    create_response = client.post(
        "/api/v1/users/",
        json={
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123",
        },
    )
    user_id = create_response.json()["id"]

    # Get user
    response = client.get(f"/api/v1/users/{user_id}")
    assert response.status_code == 200
    data = response.json()
    assert data["username"] == "testuser"
```

## Best Practices

1. **Use Pydantic for all input/output**: Define separate Create, Update, and Response schemas
2. **Dependency injection**: Use FastAPI's Depends for database sessions, auth, clients
3. **Type hints everywhere**: Use Annotated[] for cleaner dependency signatures
4. **Separate concerns**: Models (SQLAlchemy), schemas (Pydantic), CRUD (operations), routes (API)
5. **External clients in dedicated folder**: Keep API clients organized and testable
6. **Use Typer for CLI**: Professional deployment interface with uvicorn
7. **Async when possible**: Use async for I/O-bound operations (database, HTTP clients)
8. **Background tasks**: Use BackgroundTasks for non-blocking operations (emails, etc.)
9. **Proper error handling**: Use HTTPException with appropriate status codes
10. **Settings from environment**: Use pydantic-settings for configuration
11. **Use Alembic for migrations**: Never use create_all() in production
12. **Test with pytest**: Use TestClient and in-memory database

## Common Patterns

### Pagination
```python
from fastapi import Query

@router.get("/items/")
def list_items(
    skip: int = Query(0, ge=0),
    limit: int = Query(100, ge=1, le=1000),
):
    return {"skip": skip, "limit": limit}
```

### File Upload
```python
from fastapi import UploadFile, File

@router.post("/upload/")
async def upload_file(file: UploadFile = File(...)):
    contents = await file.read()
    return {"filename": file.filename, "size": len(contents)}
```

### WebSocket
```python
from fastapi import WebSocket

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    while True:
        data = await websocket.receive_text()
        await websocket.send_text(f"Echo: {data}")
```

### Response Models with Nested Data
```python
@router.get("/experiments/{id}", response_model=ExperimentWithTrials)
def get_experiment_with_nested(db: DBSession, id: int):
    # Pydantic automatically handles nested models
    return experiment_crud.get_with_trials(db, id=id)
```
