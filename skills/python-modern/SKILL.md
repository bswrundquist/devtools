---
name: python-modern
description: Use when writing modern Python, setting up projects with modern tooling, or configuring type hints and type checking. Covers Python 3.12+ syntax, strict mypy, ruff, src/ layout, and uv.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Modern Python

Write fully typed Python 3.12+ with modern syntax and a strict, uv-based toolchain.

## 3.12+ Syntax

PEP 695 type aliases and generics — no `TypeVar` boilerplate:

```python
from pathlib import Path
from typing import Self

type UserId = int
type JSON = dict[str, "JSON"] | list["JSON"] | str | int | float | bool | None

class Repository[T]:
    def __init__(self, items: list[T]) -> None:
        self._items = items

    def first(self) -> T | None:
        return self._items[0] if self._items else None

class Config:
    @classmethod
    def from_file(cls, path: Path) -> Self:  # Self, not "Config" — subclass-safe
        ...
```

Use `X | None`, never `Optional[X]`. Use builtin generics (`list[int]`, `dict[str, str]`), never `typing.List`.

## pathlib, not os.path

```python
from pathlib import Path

config = Path.home() / ".config" / "app" / "settings.toml"
config.parent.mkdir(parents=True, exist_ok=True)
text = config.read_text(encoding="utf-8")
sources = list(Path("src").rglob("*.py"))
```

## Dataclasses vs Pydantic

| Tool | When |
|------|------|
| `@dataclass(slots=True, frozen=True)` | Internal data from trusted code — fast, zero deps |
| Pydantic `BaseModel` | Boundary data: API payloads, config files, env vars — anything needing validation/serialization |

```python
from dataclasses import dataclass

@dataclass(slots=True, frozen=True)
class Point:
    x: float
    y: float
```

Don't validate trusted internal data with Pydantic; don't hand-parse untrusted data into dataclasses.

## Enums and match for sum types

```python
from enum import StrEnum, auto

class JobState(StrEnum):
    PENDING = auto()
    RUNNING = auto()
    DONE = auto()
    FAILED = auto()

def describe(*, state: JobState, exit_code: int | None) -> str:
    match state, exit_code:
        case JobState.DONE, 0:
            return "succeeded"
        case JobState.FAILED, int(code):
            return f"failed with exit code {code}"
        case _:
            return state.value
```

`match` earns its place for structural decomposition over sum types and tagged data. For flat value dispatch, a dict lookup or if/elif is clearer.

## Project layout

```
myproj/
├── pyproject.toml
├── uv.lock
├── .python-version
├── src/
│   └── myproj/
│       ├── __init__.py
│       └── py.typed
└── tests/
    └── test_core.py
```

`src/` layout forces tests to run against the installed package, not the checkout — import bugs surface immediately.

## pyproject.toml skeleton

```toml
[project]
name = "myproj"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[dependency-groups]
dev = ["pytest>=8", "mypy>=1.13", "ruff>=0.8"]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.mypy]
strict = true
warn_unreachable = true

[tool.ruff]
line-length = 100
target-version = "py312"

[tool.ruff.lint]
select = ["E", "F", "I", "UP", "B", "SIM", "PTH", "RUF"]

[tool.pytest.ini_options]
testpaths = ["tests"]
```

## Toolchain

```bash
uv init --lib myproj && cd myproj
uv add httpx
uv add --dev pytest mypy ruff
uv run ruff format . && uv run ruff check --fix .
uv run mypy src
uv run pytest -q
```

## Rules

- Type everything: parameters, returns (`-> None` included), class attributes. `mypy --strict` must pass.
- Functions with 2+ parameters take keyword-only params: `def f(*, a: int, b: str) -> None`. Call with keywords: `f(a=1, b="x")`.
- `X | None` over `Optional[X]`; builtin generics over `typing.List`; PEP 695 `type` / `class C[T]` over `TypeVar`.
- `pathlib.Path` everywhere — `os.path` is banned (ruff `PTH` enforces it).
- Dataclasses for internal data, Pydantic at boundaries — never both for the same type.
- `StrEnum` / `IntEnum` over bare `Enum` when values get serialized or compared to primitives.
- `match` only for structural patterns; it is not a switch replacement for flat value checks.
- `src/` layout always; ship a `py.typed` marker in every published package.
- uv for everything — no pip, no poetry, no requirements.txt.
- ruff replaces black + isort + flake8: run `ruff format` then `ruff check --fix`, in that order.
