---
name: pydantic-model
description: Use when defining data models with validation, writing field_validator/model_validator logic, configuring pydantic-settings, or migrating Pydantic v1 code to v2. All examples use v2 APIs.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Pydantic v2

Validate boundary data with Pydantic v2 — v2 APIs only, never v1.

## Models and Field constraints

```python
from pydantic import BaseModel, ConfigDict, Field

class User(BaseModel):
    model_config = ConfigDict(frozen=True, extra="forbid")

    id: int = Field(gt=0)
    name: str = Field(min_length=1, max_length=100)
    email: str = Field(pattern=r"^[^@]+@[^@]+\.[^@]+$")
    tags: list[str] = Field(default_factory=list)
```

## Validators

```python
from datetime import datetime
from typing import Self
from pydantic import BaseModel, field_validator, model_validator

class Booking(BaseModel):
    start: datetime
    end: datetime
    room: str

    @field_validator("room")
    @classmethod
    def normalize_room(cls, v: str) -> str:
        return v.strip().upper()

    @model_validator(mode="after")
    def check_range(self) -> Self:
        if self.end <= self.start:
            raise ValueError("end must be after start")
        return self
```

`field_validator` for single fields, `model_validator(mode="after")` for cross-field checks. Use `mode="before"` only when reshaping raw input before validation.

## computed_field

```python
from pydantic import computed_field

class Rectangle(BaseModel):
    width: float
    height: float

    @computed_field
    @property
    def area(self) -> float:
        return self.width * self.height
```

Included in `model_dump()` and the JSON schema — use for derived values callers need serialized.

## Discriminated unions

```python
from typing import Annotated, Literal

class Card(BaseModel):
    method: Literal["card"]
    last4: str

class Bank(BaseModel):
    method: Literal["bank"]
    iban: str

class Payment(BaseModel):
    source: Annotated[Card | Bank, Field(discriminator="method")]
```

Discriminated unions give O(1) dispatch and precise error messages; a plain union tries every member in order and reports a wall of errors.

## TypeAdapter

Validate non-model types without a wrapper model:

```python
from pydantic import TypeAdapter

users_adapter = TypeAdapter(list[User])
users = users_adapter.validate_json(raw_bytes)
```

## Serialization

```python
user.model_dump()                      # dict, Python objects (datetime stays datetime)
user.model_dump(mode="json")           # dict, JSON-safe primitives only
user.model_dump_json()                 # str
patch.model_dump(exclude_unset=True)   # PATCH payloads: only fields the client sent
```

`exclude_unset=True` is the correct PATCH semantics — it distinguishes "not provided" from "explicitly set to null".

## ORM integration

```python
class UserOut(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: int
    email: str

user_out = UserOut.model_validate(db_user)  # reads attributes, not dict keys
```

## Settings

```python
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_", env_file=".env")

    database_url: str
    debug: bool = False
    workers: int = 4

settings = Settings()  # reads APP_DATABASE_URL, APP_DEBUG, APP_WORKERS from env/.env
```

## v1 → v2 migration

| v1 | v2 |
|----|----|
| `@validator` | `@field_validator` |
| `@root_validator` | `@model_validator` |
| `class Config` | `model_config = ConfigDict(...)` |
| `.dict()` | `.model_dump()` |
| `.json()` | `.model_dump_json()` |
| `.parse_obj()` | `.model_validate()` |
| `.parse_raw()` | `.model_validate_json()` |
| `orm_mode = True` | `from_attributes=True` |

## Rules

- v2 APIs only — if you see `@validator` or `class Config`, migrate it on sight using the table above.
- `extra="forbid"` on inbound API models: typos in payloads should fail loudly, not vanish silently.
- `field_validator` is a classmethod — always stack `@classmethod` directly under it.
- Validators validate or normalize; no I/O, no side effects.
- `default_factory=list` for mutable defaults, never `= []`.
- `exclude_unset=True` for PATCH; `mode="json"` whenever the dict feeds `json.dumps` or a JSON column.
- Use a discriminated union whenever a union has a natural tag field.
- Pydantic at boundaries only — internal data structures use dataclasses.
- Settings classes over scattered `os.environ` reads; one `Settings` per app with an env prefix.
- Reuse `TypeAdapter` instances at module level; never construct them in a hot loop.
