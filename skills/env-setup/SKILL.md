---
name: env-setup
description: Use when setting up .env files and pydantic-settings, creating .env.example, managing configuration across dev/staging/production, or testing with different configs. Enforces strict secrets hygiene.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Environment & Secrets Setup

Manage configuration with pydantic-settings; keep secrets out of code, logs, and git.

## Settings module

```python
# src/app/settings.py
from functools import lru_cache

from pydantic import Field, PostgresDsn, SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict


class DatabaseSettings(BaseSettings):
    url: PostgresDsn
    pool_size: int = 5


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        env_nested_delimiter="__",   # DB__URL maps to db.url, DB__POOL_SIZE to db.pool_size
        extra="ignore",
    )

    env: str = Field(default="dev", pattern="^(dev|staging|production)$")
    debug: bool = False
    api_key: SecretStr               # repr/logs show '**********', never the value
    db: DatabaseSettings


@lru_cache
def get_settings() -> Settings:
    return Settings()                # parse once at first call, reuse everywhere
```

Validation runs at startup — a missing or malformed variable fails fast with a clear pydantic error instead of a 3 a.m. `AttributeError` deep in a request.

```python
from app.settings import get_settings

settings = get_settings()
engine = create_engine(str(settings.db.url))
client = ApiClient(key=settings.api_key.get_secret_value())  # unwrap only at the point of use
```

## .env.example committed — .env never

```bash
# .env.example — placeholders only, committed to git, one line per variable the app reads
ENV=dev
DEBUG=true
API_KEY=changeme
DB__URL=postgresql+psycopg://user:password@localhost:5432/app
DB__POOL_SIZE=5
```

```gitignore
# .gitignore
.env
.env.*
!.env.example
```

Onboarding is `cp .env.example .env`, then fill in real values.

## Environments (dev / staging / production)

Real environment variables always beat `.env` file values — that is the deployment mechanism. In staging/production don't ship `.env` files at all; inject variables via the platform (K8s secrets, ECS task definition, CI/CD variables). The same `Settings` class works unchanged everywhere.

```python
model_config = SettingsConfigDict(
    # Later files win; real env vars beat both
    env_file=(".env", f".env.{os.environ.get('ENV', 'dev')}"),
)
```

## Testing with different configs

```python
# A) monkeypatch env vars — must clear the lru_cache
def test_debug_mode(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("DEBUG", "true")
    monkeypatch.setenv("API_KEY", "test-key")
    get_settings.cache_clear()
    assert get_settings().debug is True


# B) construct directly — no env needed, best for unit tests
def make_test_settings(**overrides: object) -> Settings:
    defaults: dict[str, object] = {
        "api_key": "test-key",
        "db": {"url": "postgresql+psycopg://localhost:5432/test"},
    }
    return Settings(_env_file=None, **{**defaults, **overrides})


# C) FastAPI dependency override
app.dependency_overrides[get_settings] = lambda: make_test_settings(debug=True)
```

If any test mutates env vars, add an autouse fixture calling `get_settings.cache_clear()` so cached state never leaks between tests.

## direnv integration

`.envrc` loads variables automatically on `cd` and pairs with a secret store so real values never sit in a file:

```bash
# .envrc — direnv loads on cd; secrets pulled from the store at load time
dotenv_if_exists .env
export API_KEY="$(gopass show -o myproject/api_key)"
```

Run `direnv allow` after every `.envrc` edit. Treat `.envrc` as sensitive — same handling as `.env`.

To debug config, check which variable NAMES are set — never dump values:

```bash
env | cut -d= -f1 | grep '^DB__' | sort   # names only, values never printed
```

## Rules

- Never commit `.env`; always commit `.env.example` with placeholder values covering every variable the app reads. Keep it in sync when adding settings.
- Validate at startup with pydantic-settings — no raw `os.environ.get()` calls scattered through the codebase.
- `@lru_cache` on `get_settings()` — parse once, reuse everywhere; `cache_clear()` in tests that touch env vars.
- `SecretStr` for every secret; unwrap with `.get_secret_value()` only at the point of use — never in logs, repr, or error messages.
- NEVER read, cat, display, or otherwise access the contents of `.env`, `.env.*`, or `.envrc` files — not even "just to check a value". Ask the user to verify their local file themselves.
- NEVER run `gopass` or any other secret-store command.
- NEVER print, echo, or log environment variable values. Reference variables by NAME only — `os.environ["DB_URL"]` in code, `$DB_URL` in shell, `DB_URL` in prose.
- Secrets on this machine are managed via direnv + gopass — write examples and templates, but do not execute, replicate, or circumvent that setup.
- When debugging, verify which variables are SET (names via `env | cut -d= -f1`), never what they contain.
- `extra="ignore"` in `SettingsConfigDict` — a shared shell environment contains unrelated variables; don't fail on them.
