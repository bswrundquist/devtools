---
name: pytest
description: Use when writing unit tests in Python. Use pytest exclusively (not unittest). Keep tests simple, prefer real data over mocks, parameterize tests, and create realistic fake data for comprehensive testing.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# pytest

Expert guidance for writing clean, maintainable unit tests with pytest.

## Core Principles

1. **pytest only** - Never use unittest.TestCase
2. **Keep it simple** - Tests should be easy to read and understand
3. **Avoid mocking** - Use real objects and fake data when possible
4. **Parameterize** - Test multiple scenarios without duplication
5. **Realistic data** - Create fake data that resembles production data
6. **Arrange-Act-Assert** - Clear test structure

## Basic Test Structure

### Simple Test

```python
def test_add_two_numbers():
    # Arrange
    a = 5
    b = 3

    # Act
    result = add(a, b)

    # Assert
    assert result == 8
```

### Test File Organization

```
project/
├── src/
│   └── calculator.py
└── tests/
    ├── __init__.py
    ├── conftest.py          # Shared fixtures
    ├── test_calculator.py   # Tests for calculator module
    └── test_integration.py  # Integration tests
```

**Naming conventions:**
- Test files: `test_*.py` or `*_test.py`
- Test functions: `def test_*()`
- Test classes: `class Test*` (rarely needed)

## Assertions

Use simple assert statements with clear messages:

```python
def test_user_creation():
    user = create_user("alice", "alice@example.com")

    # Simple assertions
    assert user.username == "alice"
    assert user.email == "alice@example.com"
    assert user.is_active is True
    assert user.created_at is not None

    # Assertions with custom messages
    assert len(user.username) > 0, "Username should not be empty"
    assert "@" in user.email, f"Invalid email format: {user.email}"
```

### Common Assertions

```python
# Equality
assert a == b
assert a != b

# Truthiness
assert value
assert not value
assert value is True
assert value is False
assert value is None
assert value is not None

# Membership
assert item in collection
assert item not in collection

# Comparisons
assert a > b
assert a >= b
assert a < b
assert a <= b

# Type checking
assert isinstance(obj, MyClass)
assert type(obj) is MyClass

# Exceptions (use pytest.raises)
with pytest.raises(ValueError):
    invalid_function()

# Exception with message check
with pytest.raises(ValueError, match="Invalid input"):
    invalid_function()

# Collections
assert len(items) == 5
assert all(x > 0 for x in numbers)
assert any(x > 100 for x in numbers)
```

## Parameterization - The Key to Good Tests

Parameterize tests to avoid duplication and test multiple scenarios:

### Basic Parameterization

```python
import pytest

@pytest.mark.parametrize("a,b,expected", [
    (2, 3, 5),
    (0, 0, 0),
    (-1, 1, 0),
    (100, 200, 300),
    (1.5, 2.5, 4.0),
])
def test_add(a, b, expected):
    result = add(a, b)
    assert result == expected
```

### Parametrize with IDs

```python
@pytest.mark.parametrize("input,expected", [
    ("hello", "HELLO"),
    ("WORLD", "WORLD"),
    ("", ""),
    ("123", "123"),
], ids=[
    "lowercase_to_uppercase",
    "already_uppercase",
    "empty_string",
    "numbers",
])
def test_to_uppercase(input, expected):
    result = to_uppercase(input)
    assert result == expected
```

### Parametrize Multiple Arguments

```python
@pytest.mark.parametrize("username", ["alice", "bob", "charlie"])
@pytest.mark.parametrize("email_domain", ["gmail.com", "yahoo.com"])
def test_user_email_combinations(username, email_domain):
    """Tests all combinations: 3 usernames × 2 domains = 6 tests"""
    email = f"{username}@{email_domain}"
    user = create_user(username, email)
    assert user.email == email
```

### Parametrize with pytest.param

```python
@pytest.mark.parametrize("value,expected", [
    pytest.param(10, 100, id="small_number"),
    pytest.param(0, 0, id="zero"),
    pytest.param(-5, 25, id="negative_number"),
    pytest.param(1000000, 1000000000000, marks=pytest.mark.slow, id="large_number"),
])
def test_square(value, expected):
    result = square(value)
    assert result == expected
```

## Fixtures - Reusable Test Data

Fixtures provide reusable setup for tests:

### Basic Fixture

```python
import pytest

@pytest.fixture
def sample_user():
    """Provides a sample user for tests"""
    return {
        "id": 1,
        "username": "alice",
        "email": "alice@example.com",
        "age": 30
    }

def test_user_validation(sample_user):
    assert validate_user(sample_user) is True

def test_user_serialization(sample_user):
    json_str = serialize_user(sample_user)
    assert "alice" in json_str
```

### Fixture with Setup and Teardown

```python
@pytest.fixture
def temp_database():
    """Creates a temporary database for testing"""
    # Setup
    db = Database(":memory:")
    db.create_tables()

    yield db  # Provide to test

    # Teardown
    db.close()

def test_insert_user(temp_database):
    user_id = temp_database.insert_user("alice", "alice@example.com")
    assert user_id > 0
```

### Fixture Scopes

```python
# Function scope (default) - runs before each test
@pytest.fixture
def new_list():
    return []

# Module scope - runs once per test module
@pytest.fixture(scope="module")
def database_connection():
    conn = create_connection()
    yield conn
    conn.close()

# Session scope - runs once per test session
@pytest.fixture(scope="session")
def api_client():
    client = APIClient()
    yield client
    client.cleanup()
```

### Parametrized Fixtures

```python
@pytest.fixture(params=["sqlite", "postgres", "mysql"])
def database(request):
    """Tests will run with all database types"""
    db_type = request.param
    db = create_database(db_type)
    yield db
    db.cleanup()

def test_query(database):
    """Runs 3 times - once for each database type"""
    result = database.query("SELECT 1")
    assert result is not None
```

## Creating Fake Data

Use realistic fake data instead of mocks when possible:

### Manual Fake Data

```python
def fake_user(
    username="testuser",
    email="test@example.com",
    age=30,
    is_active=True
):
    """Factory function for fake users"""
    return User(
        username=username,
        email=email,
        age=age,
        is_active=is_active
    )

def test_user_creation():
    user = fake_user(username="alice", age=25)
    assert user.username == "alice"
    assert user.age == 25
```

### Fake Data with Fixtures

```python
@pytest.fixture
def user_factory():
    """Factory fixture for creating test users"""
    def _create_user(**kwargs):
        defaults = {
            "username": "testuser",
            "email": "test@example.com",
            "age": 30,
            "is_active": True,
        }
        defaults.update(kwargs)
        return User(**defaults)
    return _create_user

def test_multiple_users(user_factory):
    alice = user_factory(username="alice", age=25)
    bob = user_factory(username="bob", age=35)

    assert alice.username == "alice"
    assert bob.age == 35
```

### Using Faker Library

```python
from faker import Faker

@pytest.fixture
def fake():
    """Provides Faker instance for generating realistic data"""
    return Faker()

def test_user_with_realistic_data(fake):
    user = User(
        username=fake.user_name(),
        email=fake.email(),
        first_name=fake.first_name(),
        last_name=fake.last_name(),
        address=fake.address(),
        phone=fake.phone_number(),
    )

    assert "@" in user.email
    assert len(user.username) > 0

@pytest.fixture
def fake_users(fake):
    """Generate multiple fake users"""
    return [
        User(
            username=fake.user_name(),
            email=fake.email(),
            age=fake.random_int(18, 80)
        )
        for _ in range(10)
    ]

def test_user_batch_processing(fake_users):
    result = process_users(fake_users)
    assert len(result) == 10
```

### Fake Data Classes

```python
from dataclasses import dataclass, field
from typing import List
import random

@dataclass
class FakeProduct:
    """Fake product data for testing"""
    id: int = field(default_factory=lambda: random.randint(1, 10000))
    name: str = "Test Product"
    price: float = 99.99
    in_stock: bool = True
    categories: List[str] = field(default_factory=lambda: ["Electronics"])

@pytest.fixture
def sample_products():
    return [
        FakeProduct(name="Laptop", price=999.99),
        FakeProduct(name="Mouse", price=29.99),
        FakeProduct(name="Keyboard", price=79.99),
    ]

def test_cart_total(sample_products):
    cart = ShoppingCart()
    for product in sample_products:
        cart.add(product)

    assert cart.total() == 1109.97
```

## When to Use Mocking

**Prefer real objects and fake data**, but use mocks for:

1. External services (APIs, databases in integration tests)
2. Slow operations (file I/O, network calls)
3. Non-deterministic behavior (random, datetime.now())
4. Side effects you want to verify

### pytest-mock (Recommended)

```python
def test_api_call(mocker):
    """Mock external API call"""
    mock_response = {"status": "success", "data": [1, 2, 3]}
    mock_get = mocker.patch("requests.get")
    mock_get.return_value.json.return_value = mock_response

    result = fetch_data_from_api()

    assert result == [1, 2, 3]
    mock_get.assert_called_once()
```

### Mock datetime

```python
from datetime import datetime

def test_timestamp_creation(mocker):
    """Mock datetime.now() for consistent tests"""
    fixed_time = datetime(2024, 1, 15, 10, 30, 0)
    mocker.patch("datetime.datetime").now.return_value = fixed_time

    record = create_record()

    assert record.created_at == fixed_time
```

### Mock File Operations

```python
def test_read_config(mocker):
    """Mock file reading"""
    mock_open = mocker.patch("builtins.open", mocker.mock_open(
        read_data='{"key": "value"}'
    ))

    config = load_config("config.json")

    assert config["key"] == "value"
    mock_open.assert_called_once_with("config.json", "r")
```

### Partial Mocking (Spy)

```python
def test_with_spy(mocker):
    """Verify function was called while still using real implementation"""
    spy = mocker.spy(MyClass, "my_method")

    obj = MyClass()
    result = obj.my_method(42)

    spy.assert_called_once_with(42)
    assert result is not None  # Real result
```

## Testing Exceptions

```python
import pytest

def test_division_by_zero():
    with pytest.raises(ZeroDivisionError):
        divide(10, 0)

def test_invalid_input_message():
    with pytest.raises(ValueError, match="age must be positive"):
        create_user("alice", age=-5)

def test_exception_details():
    with pytest.raises(ValidationError) as exc_info:
        validate_data({"invalid": "data"})

    assert "field" in str(exc_info.value)
    assert exc_info.value.code == "VALIDATION_ERROR"
```

## Testing Async Code

```python
import pytest

@pytest.mark.asyncio
async def test_async_function():
    result = await fetch_data_async()
    assert result is not None

@pytest.mark.asyncio
async def test_async_with_fixture(async_client):
    response = await async_client.get("/api/users")
    assert response.status_code == 200

@pytest.fixture
async def async_database():
    db = await create_async_db()
    yield db
    await db.close()
```

## Markers - Organize and Control Tests

```python
import pytest

# Mark slow tests
@pytest.mark.slow
def test_large_dataset_processing():
    # ... slow test
    pass

# Mark tests that require database
@pytest.mark.database
def test_user_query():
    # ... database test
    pass

# Skip tests
@pytest.mark.skip(reason="Not implemented yet")
def test_future_feature():
    pass

# Skip conditionally
@pytest.mark.skipif(sys.platform == "win32", reason="Unix only")
def test_unix_feature():
    pass

# Expected failure
@pytest.mark.xfail(reason="Known bug #123")
def test_known_issue():
    pass

# Custom markers (define in pytest.ini)
@pytest.mark.integration
def test_api_integration():
    pass
```

**Run specific markers:**
```bash
pytest -m "not slow"           # Skip slow tests
pytest -m "database"           # Run only database tests
pytest -m "not integration"    # Skip integration tests
```

## conftest.py - Shared Fixtures

Place shared fixtures in `conftest.py`:

```python
# tests/conftest.py
import pytest
from faker import Faker

@pytest.fixture(scope="session")
def fake():
    """Shared Faker instance"""
    return Faker()

@pytest.fixture
def user_factory():
    """Factory for creating test users"""
    def _create(**kwargs):
        defaults = {
            "username": "testuser",
            "email": "test@example.com",
            "is_active": True,
        }
        defaults.update(kwargs)
        return User(**defaults)
    return _create

@pytest.fixture(autouse=True)
def reset_database():
    """Automatically reset database before each test"""
    db.clear()
    yield
    db.close()
```

## Test Organization Patterns

### Test Class for Grouping

```python
class TestUserValidation:
    """Group related tests - no inheritance needed"""

    @pytest.fixture
    def invalid_emails(self):
        return ["invalid", "@example.com", "user@", ""]

    def test_valid_email(self):
        assert validate_email("user@example.com") is True

    @pytest.mark.parametrize("email", [
        "invalid",
        "@example.com",
        "user@",
        "",
    ])
    def test_invalid_emails(self, email):
        assert validate_email(email) is False
```

### Nested Tests (pytest-describe pattern)

```python
def describe_calculator():

    def describe_addition():

        def test_adds_positive_numbers():
            assert add(2, 3) == 5

        def test_adds_negative_numbers():
            assert add(-2, -3) == -5

    def describe_division():

        def test_divides_numbers():
            assert divide(10, 2) == 5

        def test_raises_on_zero():
            with pytest.raises(ZeroDivisionError):
                divide(10, 0)
```

## Practical Examples

### Example 1: Testing a Calculator

```python
import pytest

@pytest.mark.parametrize("a,b,expected", [
    (5, 3, 8),
    (0, 0, 0),
    (-1, 1, 0),
    (100, -50, 50),
    (1.5, 2.5, 4.0),
])
def test_add(a, b, expected):
    assert add(a, b) == expected

@pytest.mark.parametrize("a,b,expected", [
    (10, 2, 5.0),
    (7, 2, 3.5),
    (-10, 2, -5.0),
    (0, 5, 0.0),
])
def test_divide(a, b, expected):
    assert divide(a, b) == expected

def test_divide_by_zero():
    with pytest.raises(ZeroDivisionError):
        divide(10, 0)
```

### Example 2: Testing User Service

```python
import pytest
from faker import Faker

@pytest.fixture
def fake():
    return Faker()

@pytest.fixture
def user_service():
    return UserService()

@pytest.fixture
def sample_users(fake):
    """Generate realistic test users"""
    return [
        {
            "username": fake.user_name(),
            "email": fake.email(),
            "age": fake.random_int(18, 80),
            "city": fake.city(),
        }
        for _ in range(5)
    ]

def test_create_user(user_service, fake):
    username = fake.user_name()
    email = fake.email()

    user = user_service.create_user(username, email)

    assert user.username == username
    assert user.email == email
    assert user.id is not None

def test_find_user_by_email(user_service, fake):
    email = fake.email()
    created_user = user_service.create_user(fake.user_name(), email)

    found_user = user_service.find_by_email(email)

    assert found_user.id == created_user.id
    assert found_user.email == email

def test_bulk_create(user_service, sample_users):
    created = user_service.bulk_create(sample_users)

    assert len(created) == 5
    assert all(user.id is not None for user in created)

@pytest.mark.parametrize("age,expected_category", [
    (17, "minor"),
    (18, "adult"),
    (30, "adult"),
    (65, "adult"),
    (66, "senior"),
])
def test_age_category(user_service, fake, age, expected_category):
    user = user_service.create_user(fake.user_name(), fake.email(), age=age)

    assert user.age_category == expected_category
```

### Example 3: Testing Data Processing Pipeline

```python
import pytest

@pytest.fixture
def sample_transactions():
    """Realistic transaction data"""
    return [
        {"id": 1, "amount": 100.0, "status": "completed", "user_id": 1},
        {"id": 2, "amount": 250.0, "status": "completed", "user_id": 1},
        {"id": 3, "amount": 50.0, "status": "pending", "user_id": 2},
        {"id": 4, "amount": 500.0, "status": "completed", "user_id": 2},
        {"id": 5, "amount": 75.0, "status": "failed", "user_id": 3},
    ]

def test_filter_completed(sample_transactions):
    completed = filter_by_status(sample_transactions, "completed")

    assert len(completed) == 3
    assert all(t["status"] == "completed" for t in completed)

def test_calculate_user_totals(sample_transactions):
    totals = calculate_user_totals(sample_transactions)

    assert totals[1] == 350.0  # User 1: 100 + 250
    assert totals[2] == 550.0  # User 2: 50 + 500
    assert totals[3] == 75.0   # User 3: 75

def test_pipeline(sample_transactions):
    result = (
        Pipeline(sample_transactions)
        .filter_by_status("completed")
        .group_by_user()
        .calculate_totals()
        .execute()
    )

    assert len(result) == 2
    assert result[1]["total"] == 350.0
    assert result[2]["total"] == 500.0
```

### Example 4: Testing with Realistic Data Files

```python
import pytest
import json
from pathlib import Path

@pytest.fixture
def sample_data_file(tmp_path):
    """Create temporary JSON file with test data"""
    data = {
        "users": [
            {"id": 1, "name": "Alice", "email": "alice@example.com"},
            {"id": 2, "name": "Bob", "email": "bob@example.com"},
        ]
    }

    file_path = tmp_path / "users.json"
    file_path.write_text(json.dumps(data))

    return file_path

def test_load_users_from_file(sample_data_file):
    users = load_users(sample_data_file)

    assert len(users) == 2
    assert users[0]["name"] == "Alice"
    assert users[1]["email"] == "bob@example.com"
```

## pytest Configuration

### pytest.ini

```ini
[pytest]
# Test discovery patterns
python_files = test_*.py *_test.py
python_classes = Test*
python_functions = test_*

# Markers
markers =
    slow: marks tests as slow
    integration: integration tests
    database: tests requiring database
    unit: unit tests

# Output options
addopts =
    -v                  # Verbose
    --strict-markers    # Error on unknown markers
    --tb=short          # Shorter traceback format
    -ra                 # Show summary of all test outcomes

# Coverage (if using pytest-cov)
[coverage:run]
source = src
omit = */tests/*

[coverage:report]
exclude_lines =
    pragma: no cover
    def __repr__
    raise AssertionError
    raise NotImplementedError
```

### pyproject.toml

```toml
[tool.pytest.ini_options]
testpaths = ["tests"]
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]

markers = [
    "slow: marks tests as slow",
    "integration: integration tests",
]

addopts = [
    "-v",
    "--strict-markers",
    "--tb=short",
]
```

## Running Tests

```bash
# Run all tests
pytest

# Run specific file
pytest tests/test_calculator.py

# Run specific test
pytest tests/test_calculator.py::test_add

# Run tests matching pattern
pytest -k "test_user"

# Run with markers
pytest -m "not slow"
pytest -m "integration"

# Verbose output
pytest -v

# Show print statements
pytest -s

# Stop on first failure
pytest -x

# Show locals in tracebacks
pytest -l

# Run last failed tests
pytest --lf

# Run failed tests first, then others
pytest --ff

# Parallel execution (requires pytest-xdist)
pytest -n auto
```

## Essential pytest Plugins

```bash
# Install common plugins
pip install pytest pytest-cov pytest-mock pytest-asyncio pytest-xdist faker

# Or with uv
uv pip install pytest pytest-cov pytest-mock pytest-asyncio pytest-xdist faker
```

**Recommended plugins:**
- `pytest-cov` - Coverage reporting
- `pytest-mock` - Mocking support
- `pytest-asyncio` - Async test support
- `pytest-xdist` - Parallel test execution
- `faker` - Generate realistic fake data

## Best Practices Summary

1. **Keep tests simple** - Easy to read and understand
2. **Use parametrize** - Avoid test duplication
3. **Create realistic fake data** - Better than hardcoded values
4. **Avoid mocking when possible** - Use real objects with fake data
5. **One assertion per test** - Or related assertions
6. **Clear test names** - Describe what is being tested
7. **Use fixtures** - For reusable setup
8. **Arrange-Act-Assert** - Clear structure
9. **Test behavior, not implementation** - Tests should survive refactoring
10. **Fast tests** - Keep unit tests fast (<100ms each)

## Common Patterns

### Factory Pattern

```python
class UserFactory:
    """Factory for creating test users"""

    @staticmethod
    def create(**kwargs):
        defaults = {
            "username": "testuser",
            "email": "test@example.com",
            "age": 30,
            "is_active": True,
        }
        defaults.update(kwargs)
        return User(**defaults)

    @staticmethod
    def create_batch(count=5, **kwargs):
        return [UserFactory.create(**kwargs) for _ in range(count)]

@pytest.fixture
def user_factory():
    return UserFactory

def test_with_factory(user_factory):
    users = user_factory.create_batch(10, is_active=True)
    assert len(users) == 10
    assert all(u.is_active for u in users)
```

### Builder Pattern

```python
class UserBuilder:
    """Builder for creating complex test users"""

    def __init__(self):
        self._username = "testuser"
        self._email = "test@example.com"
        self._age = 30
        self._roles = []

    def with_username(self, username):
        self._username = username
        return self

    def with_email(self, email):
        self._email = email
        return self

    def with_roles(self, *roles):
        self._roles = list(roles)
        return self

    def build(self):
        return User(
            username=self._username,
            email=self._email,
            age=self._age,
            roles=self._roles,
        )

def test_admin_user():
    user = (
        UserBuilder()
        .with_username("admin")
        .with_roles("admin", "moderator")
        .build()
    )

    assert user.has_role("admin")
    assert user.has_role("moderator")
```

## Anti-Patterns to Avoid

1. **Don't use unittest.TestCase** - Use plain functions
2. **Don't over-mock** - Prefer real objects with fake data
3. **Don't test implementation details** - Test behavior
4. **Don't share state between tests** - Each test should be independent
5. **Don't make tests depend on order** - Tests should run in any order
6. **Don't use time.sleep()** - Tests should be fast and deterministic
7. **Don't put logic in tests** - Tests should be simple and obvious

## Quick Reference

```python
# Basic test
def test_something():
    assert result == expected

# Parametrized test
@pytest.mark.parametrize("input,expected", [(1, 2), (3, 4)])
def test_param(input, expected):
    assert func(input) == expected

# Fixture
@pytest.fixture
def data():
    return {"key": "value"}

# Exception testing
with pytest.raises(ValueError):
    func()

# Mock (when necessary)
def test_mock(mocker):
    mocker.patch("module.function")

# Async test
@pytest.mark.asyncio
async def test_async():
    result = await async_func()
```
