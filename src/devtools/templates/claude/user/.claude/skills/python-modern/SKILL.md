---
name: python-modern
description: Use when writing modern Python code, working with uv package manager, using Pydantic BaseModel, type hints, type checking, or setting up Python projects with modern tooling.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Modern Python Development

Expert guidance for modern Python development with uv, Pydantic, and type hints.

## UV Package Manager

uv is a fast Python package manager written in Rust, replacing pip, pip-tools, and virtual environments.

### Installation
```bash
# Install uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or with pip
pip install uv
```

### Basic Commands
```bash
# Create a new project
uv init my-project
cd my-project

# Install dependencies
uv add requests pandas pydantic

# Install dev dependencies
uv add --dev pytest black ruff mypy

# Install from requirements
uv pip install -r requirements.txt

# Create/sync virtual environment
uv sync

# Run command in environment
uv run python script.py
uv run pytest

# Update dependencies
uv lock --upgrade

# Show installed packages
uv pip list
uv pip show pydantic
```

### Project Structure with uv
```
my-project/
├── pyproject.toml          # Project config (replaces setup.py)
├── uv.lock                 # Locked dependencies
├── .python-version         # Python version pinning
├── README.md
├── src/
│   └── my_package/
│       ├── __init__.py
│       ├── main.py
│       └── models.py
└── tests/
    ├── __init__.py
    └── test_main.py
```

### pyproject.toml with uv
```toml
[project]
name = "my-project"
version = "0.1.0"
description = "A modern Python project"
readme = "README.md"
requires-python = ">=3.11"
dependencies = [
    "pydantic>=2.0.0",
    "pydantic-settings>=2.0.0",
    "requests>=2.31.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.4.0",
    "black>=23.0.0",
    "ruff>=0.1.0",
    "mypy>=1.7.0",
]

[tool.uv]
dev-dependencies = [
    "pytest>=7.4.0",
    "pytest-cov>=4.1.0",
]

[tool.black]
line-length = 100
target-version = ['py311']

[tool.ruff]
line-length = 100
select = ["E", "F", "I", "N", "W", "UP"]
ignore = []

[tool.mypy]
python_version = "3.11"
strict = true
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = true
```

## Type Hints

### Basic Types
```python
from typing import Any

# Primitives
def greet(name: str) -> str:
    return f"Hello, {name}"

def calculate(x: int, y: float) -> float:
    return x + y

def is_valid(flag: bool) -> bool:
    return not flag

def process() -> None:
    """Function that returns nothing"""
    print("Processing...")
```

### Collections
```python
from typing import List, Dict, Set, Tuple, Optional

# Lists
def process_items(items: list[str]) -> list[int]:
    return [len(item) for item in items]

# Dictionaries
def get_config() -> dict[str, Any]:
    return {"host": "localhost", "port": 5432}

# Sets
def unique_values(data: list[int]) -> set[int]:
    return set(data)

# Tuples (fixed size)
def get_coordinates() -> tuple[float, float]:
    return (40.7128, -74.0060)

# Variable-length tuples
def get_values() -> tuple[int, ...]:
    return (1, 2, 3, 4, 5)

# Optional (can be None)
def find_user(user_id: int) -> Optional[str]:
    if user_id == 1:
        return "Alice"
    return None

# Modern optional syntax (Python 3.10+)
def find_user_modern(user_id: int) -> str | None:
    if user_id == 1:
        return "Alice"
    return None
```

### Union Types
```python
from typing import Union

# Old style
def process(value: Union[int, str]) -> str:
    return str(value)

# New style (Python 3.10+)
def process_modern(value: int | str) -> str:
    return str(value)

# Multiple types
def handle_input(data: int | float | str | None) -> str:
    if data is None:
        return "empty"
    return str(data)
```

### Generic Types
```python
from typing import TypeVar, Generic, Protocol

T = TypeVar('T')
K = TypeVar('K')
V = TypeVar('V')

# Generic function
def first(items: list[T]) -> T | None:
    return items[0] if items else None

# Generic class
class Container(Generic[T]):
    def __init__(self, value: T) -> None:
        self.value = value

    def get(self) -> T:
        return self.value

# Usage
int_container = Container[int](42)
str_container = Container[str]("hello")

# Generic mapping
def invert_dict(d: dict[K, V]) -> dict[V, K]:
    return {v: k for k, v in d.items()}
```

### Protocol (Structural Subtyping)
```python
from typing import Protocol

class Drawable(Protocol):
    """Anything with a draw method"""
    def draw(self) -> None: ...

class Circle:
    def draw(self) -> None:
        print("Drawing circle")

class Square:
    def draw(self) -> None:
        print("Drawing square")

def render(shape: Drawable) -> None:
    shape.draw()

# Both work without inheritance
render(Circle())
render(Square())
```

### Callable Types
```python
from typing import Callable

# Function that takes a function
def apply_twice(func: Callable[[int], int], value: int) -> int:
    return func(func(value))

def double(x: int) -> int:
    return x * 2

result = apply_twice(double, 5)  # 20

# With multiple parameters
def process(callback: Callable[[str, int], bool]) -> None:
    callback("test", 42)
```

### TypedDict
```python
from typing import TypedDict

class User(TypedDict):
    id: int
    name: str
    email: str
    active: bool

def create_user(data: User) -> User:
    # Validate and return
    return data

user: User = {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com",
    "active": True,
}
```

## Pydantic BaseModel

Pydantic provides data validation and settings management using Python type hints.

### Basic Model
```python
from pydantic import BaseModel, Field, field_validator
from datetime import datetime

class User(BaseModel):
    id: int
    username: str = Field(..., min_length=3, max_length=50)
    email: str = Field(..., pattern=r'^[\w\.-]+@[\w\.-]+\.\w+$')
    age: int = Field(..., ge=0, le=150)
    is_active: bool = True
    created_at: datetime = Field(default_factory=datetime.now)
    tags: list[str] = []
    metadata: dict[str, Any] = {}

# Usage
user = User(
    id=1,
    username="alice",
    email="alice@example.com",
    age=30,
)

# Access fields
print(user.username)  # alice
print(user.model_dump())  # Dict representation
print(user.model_dump_json())  # JSON string

# Validation happens automatically
try:
    invalid_user = User(id=1, username="ab", email="invalid", age=200)
except ValidationError as e:
    print(e.errors())
```

### Field Validators
```python
from pydantic import BaseModel, field_validator, model_validator

class Product(BaseModel):
    name: str
    price: float
    discount: float = 0.0
    final_price: float | None = None

    @field_validator('price')
    @classmethod
    def price_must_be_positive(cls, v: float) -> float:
        if v <= 0:
            raise ValueError('Price must be positive')
        return v

    @field_validator('discount')
    @classmethod
    def discount_must_be_valid(cls, v: float) -> float:
        if not 0 <= v <= 1:
            raise ValueError('Discount must be between 0 and 1')
        return v

    @model_validator(mode='after')
    def calculate_final_price(self) -> 'Product':
        if self.final_price is None:
            self.final_price = self.price * (1 - self.discount)
        return self
```

### Nested Models
```python
from pydantic import BaseModel

class Address(BaseModel):
    street: str
    city: str
    country: str
    zip_code: str

class Company(BaseModel):
    name: str
    address: Address
    employees: list[str]

class Person(BaseModel):
    name: str
    age: int
    address: Address
    company: Company | None = None

# Create nested structure
person = Person(
    name="Alice",
    age=30,
    address=Address(
        street="123 Main St",
        city="New York",
        country="USA",
        zip_code="10001",
    ),
    company=Company(
        name="Acme Corp",
        address=Address(
            street="456 Corp Ave",
            city="New York",
            country="USA",
            zip_code="10002",
        ),
        employees=["Bob", "Charlie"],
    ),
)
```

### Model Configuration
```python
from pydantic import BaseModel, ConfigDict

class User(BaseModel):
    model_config = ConfigDict(
        str_strip_whitespace=True,  # Strip whitespace from strings
        validate_assignment=True,   # Validate on attribute assignment
        frozen=False,               # Allow mutations (True for immutable)
        populate_by_name=True,      # Allow population by field name
        arbitrary_types_allowed=False,
    )

    id: int
    username: str
    email: str
```

### Settings Management
```python
from pydantic_settings import BaseSettings, SettingsConfigDict
from functools import lru_cache

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file='.env',
        env_file_encoding='utf-8',
        case_sensitive=False,
    )

    # Automatically loaded from environment variables
    app_name: str = "MyApp"
    debug: bool = False
    database_url: str
    api_key: str
    redis_host: str = "localhost"
    redis_port: int = 6379
    allowed_hosts: list[str] = ["localhost"]

# Cache settings (load once)
@lru_cache()
def get_settings() -> Settings:
    return Settings()

# Usage
settings = get_settings()
print(settings.database_url)
```

### JSON Schema and OpenAPI
```python
from pydantic import BaseModel

class Item(BaseModel):
    name: str
    description: str | None = None
    price: float
    tax: float | None = None

# Generate JSON schema
schema = Item.model_json_schema()
print(schema)

# Use with FastAPI (automatic OpenAPI docs)
from fastapi import FastAPI

app = FastAPI()

@app.post("/items/")
def create_item(item: Item) -> Item:
    return item
```

### Custom Types
```python
from pydantic import BaseModel, field_validator
from typing import Annotated
from pydantic.functional_validators import AfterValidator

def validate_positive(v: int) -> int:
    if v <= 0:
        raise ValueError('must be positive')
    return v

PositiveInt = Annotated[int, AfterValidator(validate_positive)]

class Product(BaseModel):
    name: str
    quantity: PositiveInt
    price: PositiveInt

# Usage
product = Product(name="Widget", quantity=10, price=100)  # OK
# product = Product(name="Widget", quantity=-5, price=100)  # ValidationError
```

### Dataclass Alternative
```python
from pydantic.dataclasses import dataclass
from pydantic import Field

@dataclass
class User:
    id: int
    username: str = Field(min_length=3)
    email: str
    is_active: bool = True

# Works like dataclass but with validation
user = User(id=1, username="alice", email="alice@example.com")
```

## Type Checking with mypy

```bash
# Install mypy
uv add --dev mypy

# Run mypy
uv run mypy src/

# With specific config
uv run mypy --strict src/
```

### mypy Configuration
```toml
# pyproject.toml
[tool.mypy]
python_version = "3.11"
strict = true
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = true
disallow_any_generics = true
disallow_subclassing_any = true
disallow_untyped_calls = true
disallow_incomplete_defs = true
check_untyped_defs = true
no_implicit_optional = true
warn_redundant_casts = true
warn_unused_ignores = true
warn_no_return = true
```

## Linting with Ruff

```bash
# Install ruff
uv add --dev ruff

# Check code
uv run ruff check src/

# Fix issues automatically
uv run ruff check --fix src/

# Format code
uv run ruff format src/
```

### Ruff Configuration
```toml
# pyproject.toml
[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = [
    "E",   # pycodestyle errors
    "W",   # pycodestyle warnings
    "F",   # pyflakes
    "I",   # isort
    "N",   # pep8-naming
    "UP",  # pyupgrade
    "B",   # flake8-bugbear
    "C4",  # flake8-comprehensions
    "SIM", # flake8-simplify
]
ignore = []

[tool.ruff.lint.isort]
known-first-party = ["my_package"]
```

## Testing with pytest

```python
# tests/test_models.py
import pytest
from pydantic import ValidationError
from my_package.models import User

def test_user_creation():
    user = User(
        id=1,
        username="alice",
        email="alice@example.com",
        age=30,
    )
    assert user.username == "alice"
    assert user.age == 30

def test_user_validation():
    with pytest.raises(ValidationError) as exc_info:
        User(id=1, username="ab", email="invalid", age=200)

    errors = exc_info.value.errors()
    assert len(errors) == 3  # username, email, age

def test_user_json_serialization():
    user = User(id=1, username="alice", email="alice@example.com", age=30)
    json_str = user.model_dump_json()
    assert "alice" in json_str

    # Deserialize
    user2 = User.model_validate_json(json_str)
    assert user2 == user
```

## Best Practices

1. **Use type hints everywhere**: Functions, variables, class attributes
2. **Pydantic for data validation**: Use BaseModel for data that crosses boundaries (API, DB, config)
3. **Use uv for dependencies**: Faster than pip, handles virtual environments
4. **Run mypy in strict mode**: Catch type errors early
5. **Use ruff for linting**: Fast, replaces multiple tools
6. **Type narrow with isinstance**: Help type checker understand flow
7. **Avoid Any**: Use specific types or Protocols
8. **Use | for unions**: Modern syntax (Python 3.10+)
9. **Field validation in Pydantic**: Use validators for complex business logic
10. **Settings from environment**: Use pydantic-settings for config

## Common Patterns

### Dependency Injection with Pydantic
```python
from pydantic import BaseModel

class Database:
    def query(self, sql: str) -> list[dict]:
        return []

class Config(BaseModel):
    db_url: str

class Service:
    def __init__(self, config: Config, db: Database) -> None:
        self.config = config
        self.db = db

    def get_users(self) -> list[dict]:
        return self.db.query("SELECT * FROM users")
```

### Result Type Pattern
```python
from typing import Generic, TypeVar
from pydantic import BaseModel

T = TypeVar('T')
E = TypeVar('E')

class Ok(BaseModel, Generic[T]):
    value: T

class Err(BaseModel, Generic[E]):
    error: E

Result = Ok[T] | Err[E]

def divide(a: int, b: int) -> Result[float, str]:
    if b == 0:
        return Err(error="Division by zero")
    return Ok(value=a / b)

# Usage
result = divide(10, 2)
if isinstance(result, Ok):
    print(f"Result: {result.value}")
else:
    print(f"Error: {result.error}")
```

## When Helping Users

1. **Check Python version**: Modern features require 3.10+
2. **Suggest uv**: Faster than pip/poetry
3. **Add type hints**: Always include types
4. **Use Pydantic for validation**: Not manual if/else checks
5. **Configure mypy strict mode**: Catch more errors
6. **Format with ruff**: Consistent code style
