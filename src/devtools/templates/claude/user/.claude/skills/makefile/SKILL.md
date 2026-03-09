---
name: makefile
description: Use when creating Makefiles, writing make targets, debugging make commands, or setting up project automation with make.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Makefile

Expert guidance for creating and using Makefiles for project automation.

## Basic Makefile Structure

```makefile
# Variables
PYTHON := python3
SRC_DIR := src
TEST_DIR := tests
VENV := .venv

# Default target (runs when you type 'make')
.DEFAULT_GOAL := help

# Phony targets (not actual files)
.PHONY: help install test clean

## help: Display this help message
help:
	@echo "Available targets:"
	@grep -E '^##' $(MAKEFILE_LIST) | sed 's/##//'

## install: Install dependencies
install:
	$(PYTHON) -m pip install -r requirements.txt

## test: Run tests
test:
	pytest $(TEST_DIR)

## clean: Remove build artifacts
clean:
	rm -rf __pycache__ .pytest_cache *.pyc
```

## Variables

### Simple Variables
```makefile
# := is immediate assignment (evaluated once)
CC := gcc
CFLAGS := -Wall -Wextra -O2
SRC_DIR := src
BUILD_DIR := build

# = is recursive assignment (evaluated each time)
SOURCES = $(wildcard $(SRC_DIR)/*.c)
OBJECTS = $(SOURCES:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)
```

### Automatic Variables
```makefile
# $@ - Target name
# $< - First prerequisite
# $^ - All prerequisites
# $? - Prerequisites newer than target
# $* - Stem of pattern match

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
	# $< is the .c file, $@ is the .o file
```

### Environment Variables
```makefile
# Access environment variables
HOME_DIR := $(HOME)
USER_NAME := $(USER)

# Set environment variable for commands
export DATABASE_URL := postgresql://localhost/mydb

test:
	# DATABASE_URL is available to pytest
	pytest tests/
```

### Conditional Variables
```makefile
# Set default, allow override
ENV ?= dev
DEBUG ?= false

# Conditional assignment
ifeq ($(ENV),prod)
    DATABASE := prod_db
else
    DATABASE := dev_db
endif
```

## Targets and Rules

### Basic Target
```makefile
# Target: Prerequisites
# 	Command (must be indented with TAB)

output.txt: input.txt
	cat input.txt > output.txt
```

### Phony Targets
```makefile
# Targets that don't create files
.PHONY: clean test install deploy

clean:
	rm -rf build/

test:
	pytest tests/

install:
	pip install -r requirements.txt
```

### Multiple Targets
```makefile
# Multiple targets with same recipe
.PHONY: start run serve
start run serve:
	uvicorn main:app --reload
```

### Pattern Rules
```makefile
# Pattern matching with %
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Match any .py file to run pylint
lint-%.py:
	pylint $*.py
```

### Prerequisites Only
```makefile
# Target with dependencies but no commands
.PHONY: all
all: clean install test

# Runs clean, then install, then test
```

## Common Patterns

### Python Project Makefile
```makefile
.PHONY: install dev-install test lint format clean run

# Variables
PYTHON := python3
VENV := .venv
BIN := $(VENV)/bin
SRC := src
TESTS := tests

## install: Install production dependencies
install:
	$(PYTHON) -m venv $(VENV)
	$(BIN)/pip install --upgrade pip
	$(BIN)/pip install -r requirements.txt

## dev-install: Install development dependencies
dev-install: install
	$(BIN)/pip install -r requirements-dev.txt
	$(BIN)/pre-commit install

## test: Run tests with coverage
test:
	$(BIN)/pytest $(TESTS) -v --cov=$(SRC) --cov-report=html

## lint: Check code quality
lint:
	$(BIN)/ruff check $(SRC) $(TESTS)
	$(BIN)/mypy $(SRC)

## format: Format code
format:
	$(BIN)/black $(SRC) $(TESTS)
	$(BIN)/ruff check --fix $(SRC) $(TESTS)

## run: Start the application
run:
	$(BIN)/python -m $(SRC).main

## clean: Remove build artifacts and cache
clean:
	rm -rf $(VENV)
	rm -rf .pytest_cache .coverage htmlcov
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name '*.pyc' -delete
```

### Node.js Project Makefile
```makefile
.PHONY: install dev build test lint clean start

# Variables
NPM := npm
NODE := node
SRC := src
DIST := dist

## install: Install dependencies
install:
	$(NPM) install

## dev: Start development server
dev:
	$(NPM) run dev

## build: Build for production
build:
	$(NPM) run build

## test: Run tests
test:
	$(NPM) test

## lint: Lint code
lint:
	$(NPM) run lint

## format: Format code
format:
	$(NPM) run format

## clean: Clean build artifacts
clean:
	rm -rf $(DIST) node_modules

## start: Start production server
start:
	$(NODE) $(DIST)/index.js
```

### Docker Project Makefile
```makefile
.PHONY: build up down logs shell test clean

# Variables
COMPOSE := docker-compose
IMAGE := myapp
TAG := latest

## build: Build Docker images
build:
	$(COMPOSE) build

## up: Start containers
up:
	$(COMPOSE) up -d

## down: Stop containers
down:
	$(COMPOSE) down

## logs: Show container logs
logs:
	$(COMPOSE) logs -f

## shell: Open shell in app container
shell:
	$(COMPOSE) exec app /bin/bash

## test: Run tests in container
test:
	$(COMPOSE) exec app pytest tests/

## clean: Remove containers and volumes
clean:
	$(COMPOSE) down -v
	docker rmi $(IMAGE):$(TAG)

## ps: Show running containers
ps:
	$(COMPOSE) ps
```

### Database Makefile
```makefile
.PHONY: db-create db-migrate db-rollback db-seed db-reset

# Variables
DB_URL := postgresql://localhost/mydb
MIGRATIONS := migrations

## db-create: Create database
db-create:
	createdb mydb

## db-migrate: Run migrations
db-migrate:
	alembic upgrade head

## db-rollback: Rollback last migration
db-rollback:
	alembic downgrade -1

## db-seed: Seed database with sample data
db-seed:
	python scripts/seed_db.py

## db-reset: Drop and recreate database
db-reset: db-drop db-create db-migrate db-seed

## db-drop: Drop database (CAUTION)
db-drop:
	dropdb mydb
```

## Advanced Features

### Conditional Execution
```makefile
.PHONY: deploy

# Conditional based on variable
deploy:
ifeq ($(ENV),prod)
	@echo "Deploying to production..."
	./deploy-prod.sh
else
	@echo "Not in production mode"
	@exit 1
endif
```

### Multi-line Commands
```makefile
.PHONY: setup

# Use backslash for multi-line
setup:
	pip install \
		django \
		requests \
		pytest

# Or use semicolons
setup-alt:
	cd frontend; \
	npm install; \
	npm run build
```

### Silent Commands
```makefile
.PHONY: greet

# @ suppresses command echoing
greet:
	@echo "Hello, World!"
	# This line is not echoed, only output is shown

# Without @
greet-verbose:
	echo "Hello, World!"
	# Shows: echo "Hello, World!"
	#        Hello, World!
```

### Error Handling
```makefile
.PHONY: test-all

# Continue on error with -
test-all:
	-pytest tests/unit
	-pytest tests/integration
	@echo "Tests completed (some may have failed)"

# Stop on error (default behavior)
test-strict:
	pytest tests/unit
	pytest tests/integration
```

### Including Other Makefiles
```makefile
# Include other makefiles
include config.mk
include docker.mk

# Optional include (no error if missing)
-include local.mk
```

### Functions
```makefile
# wildcard - Find files matching pattern
SOURCES := $(wildcard src/*.c)

# patsubst - Pattern substitution
OBJECTS := $(patsubst src/%.c,build/%.o,$(SOURCES))

# shell - Execute shell command
CURRENT_DATE := $(shell date +%Y-%m-%d)
GIT_COMMIT := $(shell git rev-parse --short HEAD)

# foreach - Loop over list
DIRS := build dist logs
create-dirs:
	$(foreach dir,$(DIRS),mkdir -p $(dir);)

# if - Conditional
DEBUG := true
CFLAGS := $(if $(DEBUG),-g -O0,-O2)
```

### Automatic Dependency Generation
```makefile
# Generate header dependencies
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

# Include generated dependency files
-include $(OBJECTS:.o=.d)
```

## Self-Documenting Makefile

```makefile
.PHONY: help
.DEFAULT_GOAL := help

# Colors
BLUE := \033[0;34m
GREEN := \033[0;32m
RESET := \033[0m

## help: Show this help message
help:
	@echo "$(BLUE)Available targets:$(RESET)"
	@grep -E '^## [a-zA-Z_-]+:' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = "## |:"}; {printf "  $(GREEN)%-15s$(RESET) %s\n", $$2, $$3}'

## install: Install all dependencies
install:
	pip install -r requirements.txt

## test: Run test suite
test:
	pytest tests/

## deploy: Deploy to production
deploy:
	./deploy.sh
```

## Real-World Example: Full-Stack Project

```makefile
.PHONY: help install dev build test lint format clean docker-up docker-down migrate

# Variables
PYTHON := python3
NPM := npm
COMPOSE := docker-compose
BACKEND := backend
FRONTEND := frontend
VENV := $(BACKEND)/.venv
BIN := $(VENV)/bin

.DEFAULT_GOAL := help

## help: Display this help
help:
	@echo "Available commands:"
	@grep -E '^##' $(MAKEFILE_LIST) | sed 's/##/  /'

## install: Install all dependencies
install: install-backend install-frontend

## install-backend: Install Python dependencies
install-backend:
	cd $(BACKEND) && $(PYTHON) -m venv .venv
	$(BIN)/pip install --upgrade pip
	$(BIN)/pip install -r $(BACKEND)/requirements.txt

## install-frontend: Install Node dependencies
install-frontend:
	cd $(FRONTEND) && $(NPM) install

## dev: Start development servers
dev:
	@echo "Starting backend and frontend..."
	$(COMPOSE) up -d postgres redis
	$(MAKE) -j2 dev-backend dev-frontend

dev-backend:
	cd $(BACKEND) && $(BIN)/uvicorn main:app --reload --port 8000

dev-frontend:
	cd $(FRONTEND) && $(NPM) run dev

## build: Build for production
build: build-backend build-frontend

build-backend:
	cd $(BACKEND) && $(BIN)/pip install --no-cache-dir -r requirements.txt

build-frontend:
	cd $(FRONTEND) && $(NPM) run build

## test: Run all tests
test: test-backend test-frontend

test-backend:
	cd $(BACKEND) && $(BIN)/pytest tests/ -v --cov

test-frontend:
	cd $(FRONTEND) && $(NPM) test

## lint: Lint all code
lint: lint-backend lint-frontend

lint-backend:
	cd $(BACKEND) && $(BIN)/ruff check src/ && $(BIN)/mypy src/

lint-frontend:
	cd $(FRONTEND) && $(NPM) run lint

## format: Format all code
format: format-backend format-frontend

format-backend:
	cd $(BACKEND) && $(BIN)/black src/ tests/

format-frontend:
	cd $(FRONTEND) && $(NPM) run format

## migrate: Run database migrations
migrate:
	cd $(BACKEND) && $(BIN)/alembic upgrade head

## seed: Seed database with sample data
seed:
	cd $(BACKEND) && $(BIN)/python scripts/seed.py

## docker-up: Start Docker containers
docker-up:
	$(COMPOSE) up -d

## docker-down: Stop Docker containers
docker-down:
	$(COMPOSE) down

## docker-logs: Show Docker logs
docker-logs:
	$(COMPOSE) logs -f

## clean: Remove build artifacts
clean:
	rm -rf $(BACKEND)/.venv
	rm -rf $(BACKEND)/.pytest_cache
	rm -rf $(FRONTEND)/node_modules
	rm -rf $(FRONTEND)/dist
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name '*.pyc' -delete

## reset: Full reset (clean + install)
reset: clean install
```

## Best Practices

1. **Use `.PHONY`**: Declare non-file targets as phony
2. **Self-documenting**: Add help target with descriptions
3. **Variables at top**: Define configuration variables at the beginning
4. **Use `:=` for speed**: Immediate assignment is faster than `=`
5. **Tab indentation**: Commands must be indented with tabs, not spaces
6. **Silent by default**: Use `@` to suppress command echoing
7. **Error handling**: Use `-` prefix to ignore errors when appropriate
8. **Default goal**: Set `.DEFAULT_GOAL` for better UX
9. **Organized sections**: Group related targets together
10. **Cross-platform**: Use `$(RM)` instead of `rm -f` for portability

## Common Pitfalls

1. **Spaces instead of tabs**: Commands must use tab indentation
2. **Forgetting `.PHONY`**: Non-file targets should be declared phony
3. **Recursive variables**: Use `:=` to avoid infinite recursion
4. **Unquoted variables**: Quote paths with spaces
5. **Missing dependencies**: Specify all prerequisites

## Debugging Makefiles

```bash
# Show what make would do without executing
make -n target

# Print variables
make -p

# Debug why target is rebuilt
make -d target

# Show commands as they're executed
make -v target

# Check specific variable
make print-VAR_NAME

# Add to Makefile for debugging:
print-%:
	@echo $* = $($*)
# Usage: make print-PYTHON
```

## Tips for Writing Makefiles

1. **Start simple**: Begin with basic targets, add complexity as needed
2. **Group related tasks**: Organize targets logically
3. **Use variables**: Make configuration easy to change
4. **Document everything**: Add help text for each target
5. **Test incrementally**: Test each target as you write it
6. **Consider dependencies**: Ensure targets run in correct order
7. **Make idempotent**: Targets should be safe to run multiple times

## When Helping Users

1. **Check for tabs**: Ensure commands use tab indentation
2. **Suggest `.PHONY`**: For non-file targets
3. **Add help target**: Make Makefile self-documenting
4. **Use variables**: For paths, commands, and configuration
5. **Test commands**: Verify make targets work as expected
