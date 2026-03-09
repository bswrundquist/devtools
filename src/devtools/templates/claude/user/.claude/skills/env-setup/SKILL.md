---
name: env-setup
description: Use when setting up environment variable management — .env files, pydantic-settings, .env.example, environment-specific config, and secrets hygiene. Covers patterns for development, testing, and production.
tools: Read, Write, Edit, Bash
---

# Environment Setup

Manage configuration and secrets cleanly across development, test, and production environments.

## Core pattern: pydantic-settings

The standard approach for Python services — validates config at startup, gives clear errors for missing required values:

```bash
uv add pydantic-settings
```

```python
# config.py
from functools import lru_cache
from pydantic import PostgresDsn, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
        extra="ignore",           # silently ignore unknown env vars
    )

    # Required — will raise at startup if missing
    database_url: PostgresDsn
    secret_key: str

    # Optional with defaults
    debug: bool = False
    log_level: str = "INFO"
    allowed_hosts: list[str] = ["localhost"]
    cors_origins: list[str] = []

    # Derived
    @property
    def is_production(self) -> bool:
        return not self.debug


@lru_cache
def get_settings() -> Settings:
    return Settings()
```

Usage:

```python
from config import get_settings

settings = get_settings()
print(settings.database_url)
```

The `@lru_cache` means settings are parsed once per process. In tests, call `get_settings.cache_clear()` after patching env vars.

## .env files

### .env (local development — never commit)

```bash
# .env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/myapp
SECRET_KEY=dev-only-not-a-real-secret
DEBUG=true
LOG_LEVEL=DEBUG
```

### .env.example (commit this — the canonical reference)

```bash
# .env.example
# Copy to .env and fill in your values.
# Never put real secrets here.

DATABASE_URL=postgresql://user:password@localhost:5432/myapp
SECRET_KEY=                        # required — generate with: openssl rand -hex 32
DEBUG=false
LOG_LEVEL=INFO
ALLOWED_HOSTS=["localhost"]
```

### .gitignore

```
.env
.env.local
.env.*.local
*.env
```

Never commit `.env`. Always commit `.env.example`.

## Environment-specific config

### Approach 1: Single .env, override per environment

Load a base `.env` then an environment-specific override:

```python
from pydantic_settings import BaseSettings, SettingsConfigDict
import os

class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        # Load base, then override with env-specific
        env_file=(
            ".env",
            f".env.{os.getenv('APP_ENV', 'development')}",
        ),
    )
```

Files: `.env`, `.env.development`, `.env.staging`, `.env.test`

### Approach 2: env_prefix per service

```python
class DatabaseSettings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="DB_")
    url: str
    pool_size: int = 5

class RedisSettings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="REDIS_")
    url: str
    max_connections: int = 10
```

Env vars: `DB_URL`, `DB_POOL_SIZE`, `REDIS_URL`, etc.

## Testing with environment variables

### pytest with monkeypatch

```python
from functools import lru_cache
from config import get_settings, Settings

def test_debug_mode(monkeypatch):
    monkeypatch.setenv("DEBUG", "true")
    monkeypatch.setenv("DATABASE_URL", "postgresql://localhost/test")
    monkeypatch.setenv("SECRET_KEY", "test-secret")

    get_settings.cache_clear()
    settings = get_settings()

    assert settings.debug is True
    get_settings.cache_clear()   # clean up for next test
```

### pytest fixture

```python
# conftest.py
import pytest
from config import get_settings

@pytest.fixture(autouse=True)
def clear_settings_cache():
    get_settings.cache_clear()
    yield
    get_settings.cache_clear()

@pytest.fixture
def test_settings(monkeypatch):
    monkeypatch.setenv("DATABASE_URL", "postgresql://localhost/test")
    monkeypatch.setenv("SECRET_KEY", "test-secret-key")
    monkeypatch.setenv("DEBUG", "false")
    get_settings.cache_clear()
    return get_settings()
```

### Override settings in FastAPI

```python
# For FastAPI dependency injection
from fastapi import Depends
from config import Settings, get_settings

def get_test_settings() -> Settings:
    return Settings(
        database_url="postgresql://localhost/test",
        secret_key="test-secret",
        debug=True,
    )

app.dependency_overrides[get_settings] = get_test_settings
```

## Secrets management

### Development: .env file

Sufficient for local development. Ensure `.env` is in `.gitignore`.

### Production options

**Environment variables directly** (simplest, works everywhere):
```bash
export DATABASE_URL=postgresql://prod-host/myapp
export SECRET_KEY=real-production-secret
```

**AWS Secrets Manager** (fetch at startup):
```python
import boto3
import json

def load_secrets_from_aws(secret_name: str) -> dict:
    client = boto3.client("secretsmanager", region_name="us-east-1")
    response = client.get_secret_value(SecretId=secret_name)
    return json.loads(response["SecretString"])

# In settings or app startup
secrets = load_secrets_from_aws("myapp/production")
os.environ.update(secrets)
settings = Settings()
```

**HashiCorp Vault**:
```python
import hvac

client = hvac.Client(url="https://vault.example.com", token=os.environ["VAULT_TOKEN"])
secrets = client.secrets.kv.v2.read_secret_version(path="myapp/production")
os.environ.update(secrets["data"]["data"])
```

## Secret rotation hygiene

- **Rotate secrets regularly** — especially after personnel changes
- **Never hardcode secrets** — not even in "temporary" scripts
- **Use short-lived tokens** — prefer OIDC/workload identity over long-lived API keys in CI
- **Audit access** — log when secrets are accessed in production
- **Separate secrets per environment** — dev, staging, prod each have their own

## Generating secrets

```bash
# Secret key (Django, FastAPI, etc.)
openssl rand -hex 32

# UUID-based
python -c "import uuid; print(uuid.uuid4())"

# URL-safe base64
python -c "import secrets; print(secrets.token_urlsafe(32))"
```

## Common patterns

### FastAPI with settings

```python
from fastapi import FastAPI, Depends
from config import Settings, get_settings

app = FastAPI()

@app.get("/")
async def root(settings: Settings = Depends(get_settings)):
    return {"debug": settings.debug}
```

### Validate at startup (fail fast)

```python
# main.py
from config import get_settings

# Parse settings immediately at import time — crash fast if config is wrong
settings = get_settings()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000)
```

### Docker / container environments

Don't use `.env` files in containers — pass env vars directly:

```yaml
# docker-compose.yml
services:
  api:
    environment:
      DATABASE_URL: postgresql://db/myapp
      SECRET_KEY: ${SECRET_KEY}   # from host environment
```

```dockerfile
# Dockerfile — don't copy .env into the image
COPY pyproject.toml uv.lock ./
RUN uv sync --frozen --no-dev
COPY src/ ./src/
# No COPY .env — pass via docker run -e or Kubernetes secrets
```

## Rules

- Never commit `.env`. Always commit `.env.example`.
- Never log secrets, even at DEBUG level.
- Use `pydantic-settings` for any service — it validates at startup and gives useful errors for missing values.
- Use `@lru_cache` on `get_settings()` — parse once, reuse everywhere. Clear the cache in tests.
- Fail fast: parse settings at application startup, not lazily on first use.
- Keep secrets out of Docker images — pass them at runtime via environment variables.
- Use different secrets in different environments — never reuse production secrets in dev or staging.
- Document every required env var in `.env.example` with a comment explaining what it's for.
