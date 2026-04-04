---
name: pydantic
description: Use when working with Pydantic v2 — defining models, validators, serializers, settings, and common patterns. Covers field_validator, model_validator, discriminated unions, generic models, computed fields, and pydantic-settings. All examples are v2.
tools: Read, Write, Edit
---

# Pydantic v2

All examples use Pydantic v2 APIs. If you see `from pydantic.v1` or `class Config:` (not `model_config`), that's v1 — migrate it.

## Basic model

```python
from pydantic import BaseModel, Field
from datetime import datetime

class User(BaseModel):
    id: int
    name: str
    email: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    role: str = "user"
```

## Field

```python
from pydantic import BaseModel, Field

class Product(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    price: float = Field(gt=0, description="Price in USD")
    tags: list[str] = Field(default_factory=list)
    sku: str | None = Field(default=None, pattern=r"^[A-Z]{3}-\d{4}$")
    internal_id: str = Field(exclude=True)           # excluded from serialization
    display_name: str = Field(alias="displayName")   # accept alias in input
```

## model_config

```python
from pydantic import BaseModel, ConfigDict

class User(BaseModel):
    model_config = ConfigDict(
        populate_by_name=True,      # accept both alias and field name in input
        str_strip_whitespace=True,  # strip leading/trailing whitespace
        frozen=True,                # immutable (hashable)
        extra="forbid",             # reject unknown fields
        from_attributes=True,       # allow ORM model → Pydantic (replaces orm_mode)
        validate_default=True,      # run validators on default values too
    )
```

## Validators

### field_validator

```python
from pydantic import BaseModel, field_validator

class User(BaseModel):
    email: str
    age: int

    @field_validator("email")
    @classmethod
    def normalize_email(cls, v: str) -> str:
        if "@" not in v:
            raise ValueError("invalid email address")
        return v.lower().strip()

    @field_validator("age")
    @classmethod
    def check_age(cls, v: int) -> int:
        if v < 0:
            raise ValueError("age must be non-negative")
        return v
```

Validate multiple fields with one validator:

```python
@field_validator("first_name", "last_name")
@classmethod
def strip_names(cls, v: str) -> str:
    return v.strip().title()
```

### model_validator

```python
from pydantic import BaseModel, model_validator
from typing import Self

class DateRange(BaseModel):
    start: date
    end: date

    @model_validator(mode="after")
    def check_range(self) -> Self:
        if self.end < self.start:
            raise ValueError("end must be after start")
        return self

    @model_validator(mode="before")   # runs on raw input before field validation
    @classmethod
    def preprocess(cls, data: dict) -> dict:
        # normalize before validation
        if "start_date" in data and "start" not in data:
            data["start"] = data.pop("start_date")
        return data
```

## Serialization

### model_dump

```python
user = User(id=1, name="Alice", email="alice@example.com")

user.model_dump()                        # → dict
user.model_dump(mode="json")             # → JSON-safe dict (e.g., datetimes as strings)
user.model_dump(exclude={"password"})    # → dict without password
user.model_dump(include={"id", "name"}) # → dict with only these fields
user.model_dump(by_alias=True)          # → use aliases as keys
user.model_dump(exclude_unset=True)     # → only fields explicitly set by caller
user.model_dump(exclude_none=True)      # → only non-None fields
user.model_dump_json()                   # → JSON string
```

### field_serializer

```python
from pydantic import BaseModel, field_serializer
from datetime import datetime

class Event(BaseModel):
    name: str
    at: datetime

    @field_serializer("at")
    def serialize_at(self, v: datetime) -> str:
        return v.isoformat()
```

### model_serializer

```python
from pydantic import BaseModel, model_serializer

class Money(BaseModel):
    amount_cents: int
    currency: str

    @model_serializer
    def serialize(self) -> dict:
        return {"amount": self.amount_cents / 100, "currency": self.currency}
```

## Discriminated unions

```python
from typing import Annotated, Literal, Union
from pydantic import BaseModel, Field

class Cat(BaseModel):
    type: Literal["cat"]
    indoor: bool

class Dog(BaseModel):
    type: Literal["dog"]
    breed: str

Pet = Annotated[Union[Cat, Dog], Field(discriminator="type")]

class Owner(BaseModel):
    name: str
    pet: Pet

# Routes to Cat or Dog based on the "type" field value
owner = Owner.model_validate({"name": "Alice", "pet": {"type": "cat", "indoor": True}})
assert isinstance(owner.pet, Cat)
```

## Generic models

```python
from pydantic import BaseModel
from typing import Generic, TypeVar

T = TypeVar("T")

class Page(BaseModel, Generic[T]):
    items: list[T]
    total: int
    page: int
    page_size: int

# Use directly with a type parameter
page = Page[User](items=[...], total=100, page=1, page_size=20)
```

## Computed fields

```python
from pydantic import BaseModel, computed_field

class Rectangle(BaseModel):
    width: float
    height: float

    @computed_field
    @property
    def area(self) -> float:
        return self.width * self.height
```

## pydantic-settings

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        env_prefix="APP_",          # reads APP_DATABASE_URL, APP_SECRET_KEY, etc.
        case_sensitive=False,
        extra="ignore",
    )

    database_url: str
    secret_key: str
    debug: bool = False
    allowed_hosts: list[str] = ["localhost"]

# Singleton — parse once, reuse everywhere
from functools import lru_cache

@lru_cache
def get_settings() -> Settings:
    return Settings()
```

Override in tests:

```python
def test_something(monkeypatch):
    monkeypatch.setenv("APP_DATABASE_URL", "sqlite:///:memory:")
    get_settings.cache_clear()
    settings = get_settings()
    assert settings.database_url == "sqlite:///:memory:"
```

## Common patterns

### ORM integration

```python
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column
from pydantic import BaseModel, ConfigDict

class UserORM(Base):
    __tablename__ = "users"
    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str]

class UserSchema(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    name: str

# ORM → Pydantic
db_user = session.get(UserORM, 1)
schema = UserSchema.model_validate(db_user)
```

### PATCH endpoint (partial updates)

```python
class UserUpdate(BaseModel):
    name: str | None = None
    email: str | None = None
    role: str | None = None

def update_user(user_id: int, data: UserUpdate) -> None:
    # exclude_unset=True: only fields the caller explicitly provided
    updates = data.model_dump(exclude_unset=True)
    db.query(UserORM).filter_by(id=user_id).update(updates)
```

### Nested validation errors

```python
from pydantic import ValidationError

try:
    User(id="not-an-int", email="bad")
except ValidationError as e:
    print(e.error_count())   # number of errors
    print(e.errors())        # list of error dicts with loc, msg, type
    # loc is a tuple indicating the path: ("email",), ("address", "zip"), etc.
```

## V1 → V2 migration quick reference

| V1 | V2 |
|---|---|
| `@validator` | `@field_validator` |
| `@root_validator` | `@model_validator` |
| `class Config:` | `model_config = ConfigDict(...)` |
| `orm_mode = True` | `from_attributes=True` |
| `.dict()` | `.model_dump()` |
| `.json()` | `.model_dump_json()` |
| `.schema()` | `.model_json_schema()` |
| `parse_obj()` | `model_validate()` |
| `parse_raw()` | `model_validate_json()` |

## Rules

- Use `model_dump(exclude_unset=True)` for PATCH payloads — it only includes what the caller actually provided.
- Use `Field(default_factory=list)` for mutable defaults — never `Field(default=[])`.
- Prefer `model_validator(mode="after")` for cross-field validation; use `mode="before"` only for input normalization.
- `from_attributes=True` enables ORM-to-schema conversion — set it in `model_config`, not per-field.
- `frozen=True` makes models hashable and immutable — good for use as dict keys or in sets.
- When receiving untrusted input (APIs, CLI), always use `model_validate()` — never construct directly with `Model(**dict)` if you want validation to run on nested objects from raw dicts.
