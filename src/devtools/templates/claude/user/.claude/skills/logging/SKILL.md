---
name: logging
description: Use when setting up or improving logging in Python applications — structlog for structured/JSON logging, stdlib logging configuration, log levels, context variables, production vs development output, and testing logs.
tools: Read, Write, Edit
---

# Logging

Set up logging that is useful in production (structured JSON) and readable in development. Prefer structlog over stdlib for anything non-trivial.

## structlog (preferred)

structlog produces structured logs that work as human-readable text in dev and JSON in production.

```bash
pip install structlog
```

### Basic setup

```python
# logging_config.py
import logging
import sys
import structlog

def configure_logging(*, json: bool = False, level: str = "INFO") -> None:
    """Call once at application startup."""

    shared_processors = [
        structlog.contextvars.merge_contextvars,   # include bound context
        structlog.stdlib.add_log_level,
        structlog.stdlib.add_logger_name,
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.StackInfoRenderer(),
    ]

    if json:
        processors = shared_processors + [
            structlog.processors.dict_tracebacks,
            structlog.processors.JSONRenderer(),
        ]
    else:
        processors = shared_processors + [
            structlog.dev.ConsoleRenderer(),        # colored, readable
        ]

    structlog.configure(
        processors=processors,
        wrapper_class=structlog.make_filtering_bound_logger(
            logging.getLevelName(level)
        ),
        context_class=dict,
        logger_factory=structlog.PrintLoggerFactory(),
        cache_logger_on_first_use=True,
    )

    # Also configure stdlib so third-party libraries' logs are captured
    logging.basicConfig(
        format="%(message)s",
        stream=sys.stdout,
        level=level,
    )
```

### Usage

```python
import structlog

log = structlog.get_logger()

# Basic logging
log.info("user_created", user_id=123, email="alice@example.com")
log.warning("rate_limit_exceeded", user_id=123, limit=100)
log.error("payment_failed", order_id=456, reason="card_declined")

# Bind context for a block
log = log.bind(request_id="req-abc", user_id=123)
log.info("processing_started")
log.info("processing_complete", duration_ms=42)

# Exception logging
try:
    result = risky_operation()
except Exception:
    log.exception("operation_failed", operation="risky_operation")
    raise
```

### Context variables (for request-scoped context)

Bind context once per request; all subsequent log calls in that request include it:

```python
import structlog

# FastAPI / Starlette middleware
from starlette.middleware.base import BaseHTTPMiddleware

class LoggingMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request, call_next):
        structlog.contextvars.clear_contextvars()
        structlog.contextvars.bind_contextvars(
            request_id=request.headers.get("X-Request-ID", str(uuid.uuid4())),
            path=request.url.path,
            method=request.method,
        )
        response = await call_next(request)
        structlog.contextvars.bind_contextvars(status_code=response.status_code)
        log.info("request_completed")
        return response
```

### Application startup

```python
# main.py
import os
from logging_config import configure_logging

configure_logging(
    json=os.getenv("APP_ENV") == "production",
    level=os.getenv("LOG_LEVEL", "INFO"),
)
```

## stdlib logging

For simpler scripts or projects that can't add dependencies:

### Basic config

```python
import logging
import sys

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s %(name)s %(message)s",
    stream=sys.stdout,
)

log = logging.getLogger(__name__)
```

### JSON output with stdlib

```python
import json
import logging

class JSONFormatter(logging.Formatter):
    def format(self, record: logging.LogRecord) -> str:
        return json.dumps({
            "ts": self.formatTime(record, self.datefmt),
            "level": record.levelname,
            "logger": record.name,
            "msg": record.getMessage(),
            "exc": self.formatException(record.exc_info) if record.exc_info else None,
        })

handler = logging.StreamHandler()
handler.setFormatter(JSONFormatter())
logging.root.addHandler(handler)
logging.root.setLevel(logging.INFO)
```

### Logger hierarchy

```python
# root logger
logging.getLogger()

# module-level loggers (recommended)
log = logging.getLogger(__name__)   # e.g., "myapp.services.auth"

# Set levels selectively
logging.getLogger("myapp").setLevel(logging.DEBUG)
logging.getLogger("myapp.db").setLevel(logging.WARNING)
logging.getLogger("sqlalchemy.engine").setLevel(logging.WARNING)  # silence noisy lib
```

### Config dict (for apps with multiple handlers)

```python
import logging.config

LOGGING = {
    "version": 1,
    "disable_existing_loggers": False,
    "formatters": {
        "json": {"()": "myapp.logging.JSONFormatter"},
        "simple": {"format": "%(levelname)s %(name)s: %(message)s"},
    },
    "handlers": {
        "console": {
            "class": "logging.StreamHandler",
            "formatter": "json",
            "stream": "ext://sys.stdout",
        },
    },
    "root": {
        "level": "INFO",
        "handlers": ["console"],
    },
    "loggers": {
        "sqlalchemy.engine": {"level": "WARNING"},
        "uvicorn.access": {"level": "WARNING"},
    },
}

logging.config.dictConfig(LOGGING)
```

## Log levels

| Level | When to use |
|---|---|
| `DEBUG` | Detailed diagnostic info, safe to enable locally |
| `INFO` | Normal operation events: requests, jobs started/completed, significant state changes |
| `WARNING` | Unexpected situation that's handled: retrying, falling back, deprecated usage |
| `ERROR` | Something failed and requires attention; the operation did not complete |
| `CRITICAL` | System-level failure; application may not be able to continue |

**Never use print() for application logging.** Use the appropriate level.

## What to log

**Log:**
- Request received / completed (with status and duration)
- Background job started / completed / failed
- External API calls (request + response status, duration)
- State transitions that matter (order placed, payment processed, user created)
- Errors with full context
- Significant config at startup (not secrets)

**Don't log:**
- Passwords, tokens, API keys, PII (email, name, SSN) in plain text
- Internal loop iterations or every function call (use DEBUG sparingly)
- Redundant info already in the log record (level, timestamp are automatic)
- Stack traces for expected errors (e.g., 404 not found — just log the path)

## Testing logs

### structlog

```python
import structlog.testing

def test_logs_on_failure():
    with structlog.testing.capture_logs() as logs:
        process_order(order_id=123)

    assert len(logs) == 1
    assert logs[0]["event"] == "order_failed"
    assert logs[0]["order_id"] == 123
    assert logs[0]["log_level"] == "error"
```

### stdlib

```python
def test_logs_warning(caplog):
    with caplog.at_level(logging.WARNING, logger="myapp.service"):
        risky_operation()

    assert "rate_limit" in caplog.text
    assert caplog.records[0].levelno == logging.WARNING
```

## Anti-patterns

```python
# BAD: string formatting before the log call (wasteful if level is filtered)
log.debug("Processing user: " + str(user_id))

# GOOD: lazy — structlog formats only if the level is active
log.debug("processing_user", user_id=user_id)

# BAD: logging secrets
log.info("auth_success", password=password, token=token)

# GOOD: log only safe identifiers
log.info("auth_success", user_id=user_id)

# BAD: bare except that swallows errors silently
try:
    process()
except Exception:
    pass

# GOOD: always log before re-raising or ignoring
try:
    process()
except Exception:
    log.exception("process_failed", context=ctx)
    raise

# BAD: using print() in application code
print(f"Got request: {request}")

# GOOD
log.info("request_received", path=request.path, method=request.method)
```

## Rules

- Use structlog for any application that runs as a service. Use stdlib only for scripts and CLIs.
- Always use structured fields (`log.info("event", key=value)`) — not string formatting.
- Never log secrets, tokens, passwords, or PII.
- Set the log level from an environment variable (`LOG_LEVEL`), defaulting to `INFO`.
- In production, output JSON. In development, output human-readable colored text.
- Silence noisy third-party loggers (`sqlalchemy.engine`, `httpx`, `uvicorn.access`) at WARNING level.
- Use context variables (structlog) or LoggerAdapter (stdlib) to bind request-scoped context once, not on every log call.
