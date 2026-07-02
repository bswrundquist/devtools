---
name: pytest-write
description: Use when writing Python unit tests, fixtures, test data, or integration tests. pytest exclusively (never unittest) — simple readable tests, real objects with fake data over mocks, parametrize to remove duplication, Faker for realistic data.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Pytest

Write simple, readable tests with pytest — plain functions, real objects, fake data.

## Test shape

```python
from decimal import Decimal

def test_order_total_includes_shipping() -> None:
    order = Order(items=[Item(name="book", price=Decimal("12.50"))], shipping=Decimal("4.99"))

    total = order.total()

    assert total == Decimal("17.49")
```

Arrange, act, assert. One behavior per test; the name states the behavior. Plain functions — never `unittest.TestCase`.

## Fixtures

| Scope | Use for |
|-------|---------|
| `function` (default) | Almost everything — fresh state per test |
| `module` | Expensive read-only data shared within one file |
| `session` | Containers, DB engines, one-time setup |

Compose small fixtures instead of building big ones:

```python
import pytest
from faker import Faker

@pytest.fixture
def user(faker: Faker) -> User:
    return User(name=faker.name(), email=faker.email())

@pytest.fixture
def order(user: User) -> Order:
    return Order(customer=user, items=[])
```

Put shared fixtures in the nearest `conftest.py`: `tests/conftest.py` for project-wide, `tests/api/conftest.py` for API tests only. Never import from conftest — fixtures resolve by name.

## Built-in fixtures

```python
import json
from pathlib import Path

def test_writes_report(tmp_path: Path) -> None:
    out = tmp_path / "report.json"
    write_report(data={"ok": True}, path=out)
    assert json.loads(out.read_text()) == {"ok": True}

def test_reads_api_url(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("API_URL", "https://test.example.com")
    assert load_config().api_url == "https://test.example.com"
```

`tmp_path` over manual tempfile handling, `monkeypatch` over hand-patching globals — both clean up automatically.

## Parametrize

```python
from datetime import timedelta

@pytest.mark.parametrize(
    ("raw", "expected"),
    [
        ("1h30m", timedelta(hours=1, minutes=30)),
        ("45s", timedelta(seconds=45)),
        ("2d", timedelta(days=2)),
    ],
    ids=["hours-minutes", "seconds-only", "days"],
)
def test_parse_duration(raw: str, expected: timedelta) -> None:
    assert parse_duration(raw=raw) == expected
```

Always pass `ids=` — `test_parse_duration[hours-minutes]` in failure output beats an auto-generated id.

## Realistic fake data

Prefer real objects filled with fake data over mocks. Faker ships a pytest plugin providing the `faker` fixture (`uv add --dev faker`), seeded per test for reproducibility:

```python
def test_invoice_renders_customer_address(faker: Faker) -> None:
    customer = Customer(name=faker.name(), address=faker.address())

    html = render_invoice(customer=customer)

    assert customer.name in html
```

Fake data catches encoding, length, and formatting bugs that `"test"` never will.

## Error paths

```python
def test_rejects_negative_quantity() -> None:
    with pytest.raises(ValueError, match=r"quantity must be positive"):
        Item(name="book", quantity=-3)
```

Always pass `match=` — a bare `pytest.raises(ValueError)` passes on the *wrong* ValueError. Error paths are first-class behavior; test them like happy paths.

## When a mock IS justified

Only at boundaries you don't control: network, clock, randomness, subprocesses.

```python
def test_retries_on_timeout(monkeypatch: pytest.MonkeyPatch) -> None:
    calls: list[str] = []

    def fake_get(url: str) -> Response:
        calls.append(url)
        if len(calls) == 1:
            raise TimeoutError
        return Response(status=200)

    monkeypatch.setattr(client, "get", fake_get)

    assert fetch_with_retry(url="https://api.example.com").status == 200
    assert len(calls) == 2
```

Prefer a small hand-written fake over `MagicMock` — fakes fail loudly on misuse; `MagicMock` happily accepts any call with any arguments.

## Workflow

```bash
uv run pytest -x -q          # stop at first failure, quiet output
uv run pytest --lf           # rerun only last failures
uv run pytest -k "duration"  # filter by name substring
uv run pytest tests/api -q   # scope to a directory
```

## Rules

- pytest exclusively — never `unittest.TestCase`, `setUp`, or `self.assertEqual`.
- Plain `assert`; pytest's assertion rewriting gives rich failure output for free.
- Real objects with fake data over mocks; mock only network/clock/random/subprocess boundaries.
- Parametrize instead of copy-pasting near-identical tests — always with `ids=`.
- `pytest.raises` always with `match=`.
- One behavior per test; the test name is the spec.
- Fixtures are fully typed, return types included.
- No logic in tests — no loops or conditionals around asserts; parametrize instead.
- Keep conftest.py lean: fixtures only, not a helper dumping ground.
- `-x -q` while developing, `--lf` to iterate on failures, full run before commit.
