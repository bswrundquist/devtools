---
name: logging-setup
description: Use when setting up logging in a Python service (structlog preferred), configuring JSON output for production, adding request-scoped context to logs, or testing that the right log events are emitted.
tools: Bash, Read, Write, Edit, Grep, Glob
---

# Logging

Set up structured logging for Python applications: structlog for services, stdlib logging for scripts and CLIs.

## Library Choice

| Application | Library | Output |
|-------------|---------|--------|
| Service / API / worker | structlog | JSON in production, colored console in dev |
| Script / CLI | stdlib `logging` | Plain text to stderr |

Don't drag structlog into a 50-line script; don't ship f-string logs from a service.

## Structlog Configuration

Call once at the entrypoint, before anything logs:

```python
import logging
import os

import structlog


def configure_logging(*, json_output: bool | None = None) -> None:
    if json_output is None:
        json_output = os.environ.get("ENV", "dev") not in ("dev", "test")

    shared: list[structlog.typing.Processor] = [
        structlog.contextvars.merge_contextvars,  # pull in request-scoped context
        structlog.processors.add_log_level,
        structlog.processors.TimeStamper(fmt="iso", utc=True),
        structlog.processors.StackInfoRenderer(),
    ]
    if json_output:
        processors = [
            *shared,
            structlog.processors.dict_tracebacks,
            structlog.processors.JSONRenderer(),
        ]
    else:
        processors = [*shared, structlog.dev.ConsoleRenderer(colors=True)]

    structlog.configure(
        processors=processors,
        wrapper_class=structlog.make_filtering_bound_logger(logging.INFO),
        cache_logger_on_first_use=True,
    )
```

Get loggers at module level — this is lazy and safe before configuration:

```python
log = structlog.get_logger()
```

## Request-Scoped Context

Bind per-request fields once in middleware; every log line in that request carries them automatically via `merge_contextvars`:

```python
import uuid

import structlog
from fastapi import FastAPI, Request

app = FastAPI()
log = structlog.get_logger()


@app.middleware("http")
async def logging_middleware(request: Request, call_next):
    structlog.contextvars.clear_contextvars()  # prevent leakage between requests
    structlog.contextvars.bind_contextvars(
        request_id=request.headers.get("x-request-id", str(uuid.uuid4())),
        method=request.method,
        path=request.url.path,
    )
    response = await call_next(request)
    log.info("request_completed", status_code=response.status_code)
    return response
```

Deeper in the stack, bind more context as it becomes known: `structlog.contextvars.bind_contextvars(user_id=user.id)`.

## Event Naming

Events are stable snake_case identifiers you can grep, query, and alert on. Data goes in key-value fields.

```python
# Bad — interpolated message, nothing queryable
log.info(f"User {user_id} uploaded {count} files")

# Good — stable event name, structured fields
log.info("files_uploaded", user_id=user_id, count=count)
```

## Log Levels

| Level | Use for |
|-------|---------|
| DEBUG | Diagnostic detail: cache hits, retry attempts, payload sizes |
| INFO | Business events: `order_created`, `request_completed` — one per meaningful action |
| WARNING | Degraded but handled: retry succeeded, fallback used, deprecated call hit |
| ERROR | Operation failed and needs attention — always with the exception attached |
| CRITICAL | Service cannot continue |

## Testing Log Events

```python
from structlog.testing import capture_logs


def test_upload_emits_event() -> None:
    with capture_logs() as logs:
        upload_files(user_id="u1", paths=["a.csv", "b.csv"])
    events = [e for e in logs if e["event"] == "files_uploaded"]
    assert events == [
        {"event": "files_uploaded", "user_id": "u1", "count": 2, "log_level": "info"}
    ]
```

`capture_logs()` bypasses configured processors, so tests pass regardless of renderer.

## Stdlib Pattern for CLIs

```python
import logging
import sys

logging.basicConfig(
    level=logging.DEBUG if verbose else logging.INFO,
    format="%(asctime)s %(levelname)s %(name)s: %(message)s",
    stream=sys.stderr,
)
log = logging.getLogger(__name__)
```

Logs go to stderr; keep stdout for program output. Wire a `-v` flag to DEBUG.

## Rules

- structlog for services, stdlib for scripts/CLIs. JSON in production, colored console in dev — switch on environment, never hardcode a renderer.
- Never log secrets, tokens, passwords, or PII. No Authorization headers, no full request bodies, no email addresses in fields.
- Event names are snake_case constants; data goes in key-value fields, never f-strings.
- Configure logging exactly once, at the entrypoint. Library code calls `structlog.get_logger()` and never configures.
- `clear_contextvars()` at the start of every request — leaked context misattributes logs to the wrong request.
- Log exceptions with `log.exception("event", ...)` or `exc_info=True`, not `str(exc)` stuffed into a field.
- One `request_completed` INFO line per request; disable duplicate uvicorn access logs (`uvicorn --no-access-log`).
- No INFO logging inside tight loops — aggregate and log once with a count.
- Don't log and re-raise the same exception; log it where it's actually handled.
