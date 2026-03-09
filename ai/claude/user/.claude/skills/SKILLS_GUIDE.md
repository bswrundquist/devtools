# Claude Code Skills Guide

## What are Skills?

Skills are specialized capabilities in Claude Code that provide domain-specific expertise and functionality. When you invoke a skill, it loads additional context and instructions to help with specific types of tasks.

## How to Invoke Skills

There are two ways to invoke skills:

### 1. Slash Command (User)
You can type a slash command directly in the CLI:
```
/commit
/push
/git-analyze
/keybindings-help
/airflow
/sql
/polars
/pytest
/matplotlib
/altair
/typer
```

### 2. Natural Language (Automatic)
Simply describe what you want to do, and Claude will automatically invoke the relevant skill:
```
"Help me write a SQL query"
"I need to create an Airflow DAG"
"How do I customize my keybindings?"
```

## Available Skills

### commit
**Purpose**: Create well-structured git commits using Conventional Commits

**Use when:**
- The user asks to commit changes
- The user says "commit this", "make a commit", etc.

**Key principles:**
- Uses Conventional Commits format (`feat`, `fix`, `refactor`, `chore`, etc.)
- Descriptions explain what was done, not just which files changed
- Splits into multiple commits when changes are logically distinct
- Commits in logical order (refactors first, then features, then tests)

**Examples:**
- "Commit these changes"
- "Create a commit for this work"
- "/commit"

---

### git-analyze
**Purpose**: Comprehensive analysis of a git repository's recent activity, direction, and health

**Use when:**
- Understanding what's been happening in a codebase
- Identifying major initiatives, common bugs, and roadmap signals
- Reviewing contributor activity and project health
- Onboarding to a new project
- Preparing status reports or project summaries

**Sections covered:**
- Recent activity summary (pace, hot areas)
- Major initiatives & themes (3-7 workstreams)
- Common bugs & fragile areas
- Major changes & breaking work
- Roadmap signals (unmerged branches, WIP, new deps)
- Contributor analysis (6mo / 3mo / 1mo / 2wk windows)
- Contribution patterns (small vs large, cadence, distribution)

**Examples:**
- "Analyze this repo"
- "What's been going on in this codebase?"
- "Who are the major contributors?"
- "What's on the roadmap?"
- "/git-analyze"

---

### push
**Purpose**: Push local commits to the remote repository safely

**Use when:**
- The user asks to push commits
- The user says "push", "push this", "push to remote", etc.

**Key principles:**
- Shows what will be pushed before pushing
- Sets upstream tracking if needed
- Never force pushes to main/master
- Suggests `git pull --rebase` on divergence instead of force push

**Examples:**
- "Push my changes"
- "Push to remote"
- "/push"

---

### keybindings-help
**Purpose**: Customize keyboard shortcuts and keybindings

**Use when:**
- Customizing keyboard shortcuts
- Rebinding keys
- Adding chord bindings (multi-key combinations)
- Modifying `~/.claude/keybindings.json`

**Examples:**
- "Rebind ctrl+s to a different action"
- "Add a chord shortcut for running tests"
- "Change the submit key to cmd+enter"
- "How do I customize my keybindings?"

---

### airflow
**Purpose**: Work with Apache Airflow for workflow orchestration

**Use when:**
- Creating DAGs (Directed Acyclic Graphs)
- Writing Airflow tasks
- Configuring Airflow operators
- Debugging Airflow pipelines
- Discussing Airflow best practices and scheduling

**Examples:**
- "Create an Airflow DAG for ETL pipeline"
- "Help me debug this Airflow task"
- "What's the best way to schedule a daily job in Airflow?"
- "Write a PythonOperator task"

---

### sql
**Purpose**: Work with SQL queries and databases

**Use when:**
- Writing SQL queries
- Optimizing SQL performance
- Debugging SQL errors
- Working with SQL databases (PostgreSQL, MySQL, SQLite, etc.)
- Discussing SQL best practices and performance tuning

**Examples:**
- "Write a SQL query to join these tables"
- "Optimize this slow query"
- "Debug this SQL syntax error"
- "How do I create an index in PostgreSQL?"

---

### dbt
**Purpose**: Work with dbt (data build tool)

**Use when:**
- Creating dbt models
- Writing dbt SQL
- Configuring dbt projects
- Debugging dbt runs
- Creating tests
- Discussing dbt best practices and project structure

**Examples:**
- "Create a dbt model for customer analytics"
- "Help me write a dbt test"
- "Debug this dbt compilation error"
- "What's the best way to structure a dbt project?"

---

### makefile
**Purpose**: Create and work with Makefiles

**Use when:**
- Creating Makefiles
- Writing make targets
- Debugging make commands
- Setting up project automation with make

**Examples:**
- "Create a Makefile for this Python project"
- "Add a test target to my Makefile"
- "Debug this make error"
- "How do I use variables in make?"

---

### typer
**Purpose**: Build professional command-line interfaces with Typer

**Use when:**
- Creating CLI applications in Python
- Building command-line tools
- Need type-safe argument parsing
- Want automatic help text generation
- Building tools with subcommands

**Key principles:**
- Type hints everywhere (automatic validation)
- Excellent help text from docstrings
- Use enums for fixed choices
- Support environment variables for all config
- Rich output with colors and tables
- Validate input with type system

**Examples:**
- "Create a CLI tool with Typer"
- "Add a command with environment variable support"
- "Create subcommands for database management"
- "Add enum choices for output format"

---

### python-modern
**Purpose**: Write modern Python code with best practices

**Use when:**
- Writing modern Python code
- Working with uv package manager
- Using Pydantic BaseModel
- Implementing type hints
- Type checking
- Setting up Python projects with modern tooling

**Examples:**
- "Create a Pydantic model for this API"
- "Set up a Python project with uv"
- "Add type hints to this function"
- "How do I configure mypy for strict type checking?"

---

### altair
**Purpose**: Create interactive, declarative visualizations with Altair

**Use when:**
- Creating interactive web-ready visualizations
- Need declarative, concise visualization code
- Want built-in tooltips and interactivity
- Prefer Grammar of Graphics approach
- Building dashboards with linked charts

**Key principles:**
- Declarative API (describe what, not how)
- Interactive by default (tooltips, zoom, pan)
- Proper number formatting (commas, currency, percentages)
- Clear titles and labeled axes
- Long format (tidy) data preferred
- Composable charts (layer, concatenate, facet)

**Examples:**
- "Create an interactive bar chart with Altair"
- "Make a scatter plot with tooltips showing details"
- "Build a dashboard with linked charts"
- "Create a time series with a range selector"

---

### matplotlib
**Purpose**: Create professional, well-formatted data visualizations

**Use when:**
- Creating plots and charts with matplotlib
- Visualizing data
- Making graphs for reports or presentations
- Need properly formatted axes and labels

**Key principles:**
- Clear, descriptive titles
- Properly labeled axes with units
- Format numbers (commas for thousands, $ for currency, % for percentages)
- Professional styling and colors
- Readable fonts and proper sizing
- Include legends and grids where appropriate

**Examples:**
- "Create a bar chart showing revenue by quarter"
- "Plot this time series with proper date formatting"
- "Make a scatter plot with currency formatting on the axes"
- "Create a professional-looking dashboard with subplots"

---

### pytest
**Purpose**: Write unit tests with pytest (not unittest)

**Use when:**
- Writing unit tests in Python
- Setting up test fixtures
- Creating test data
- Testing functions and classes
- Writing integration tests

**Key principles:**
- Use pytest exclusively (not unittest.TestCase)
- Keep tests simple and readable
- Prefer real objects with fake data over mocks
- Parameterize tests to avoid duplication
- Use realistic fake data (Faker library)

**Examples:**
- "Write tests for this function"
- "Create parametrized tests for these scenarios"
- "Add fixtures for test data"
- "Generate fake data for testing"

---

### polars
**Purpose**: Work with DataFrames using Polars instead of Pandas/NumPy

**Use when:**
- Working with DataFrames in Python
- Processing large datasets efficiently
- Performing data transformations and aggregations
- ETL pipelines and data analysis
- Need high performance and memory efficiency

**Key principles:**
- Always prefer LazyFrames with `scan_*` methods
- Use lazy evaluation and only `.collect()` at the end
- Avoid Pandas/NumPy unless required by external packages
- Convert to Pandas only when absolutely necessary

**Examples:**
- "Read and process this CSV with Polars"
- "Aggregate this data using lazy evaluation"
- "Convert this Pandas code to Polars"
- "Join these datasets efficiently"

---

## Tips

1. **Automatic Detection**: You don't always need to explicitly invoke skills. Claude will automatically use the relevant skill when it detects you're working on a related task.

2. **Combine with Tools**: Skills work alongside other Claude Code tools like file reading, editing, and bash commands.

3. **Multiple Skills**: You can work with multiple skills in the same session as your tasks change.

4. **Context Aware**: Skills have deep domain knowledge and follow best practices for their respective technologies.

## Examples in Practice

### Example 1: Data Pipeline
```
User: "I need to create a data pipeline that extracts from PostgreSQL and loads into a data warehouse"

Claude will likely invoke:
- sql (for SQL queries)
- polars (for data processing)
- airflow (for orchestration)
- dbt (for transformations)
```

### Example 2: Project Setup
```
User: "Set up a new Python project with modern tooling and automation"

Claude will likely invoke:
- python-modern (for project setup)
- makefile (for automation)
```

### Example 3: Configuration
```
User: "I want to change my keyboard shortcuts"

Claude will invoke:
- keybindings-help (for keybindings customization)
```

---

## Need Help?

- Type `/help` for general Claude Code help
- Report issues at https://github.com/anthropics/claude-code/issues
