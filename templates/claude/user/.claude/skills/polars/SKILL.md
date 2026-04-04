---
name: polars
description: Use when working with DataFrames in Python. Prefer Polars over Pandas/NumPy for performance and modern API. Emphasize lazy evaluation with LazyFrames, explicit schemas, and well-defined types. Only convert to Pandas when required by external packages.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Polars

Expert guidance for using Polars as a modern, high-performance DataFrame library.

## Why Polars?

- **Speed**: 5-10x faster than Pandas for most operations
- **Memory Efficient**: Better memory management and lower overhead
- **Lazy Evaluation**: Query optimization before execution
- **Strong Typing**: Explicit schemas prevent errors
- **Modern API**: Consistent, expressive syntax
- **Multi-threaded**: Automatic parallelization
- **No Index Confusion**: No implicit indexes like Pandas

## Core Principles

1. **Lazy by Default** - Use `scan_*` methods and collect once at the end
2. **Explicit Schemas** - Always define data types upfront
3. **Type Safety** - Let the type system catch errors early
4. **Performance** - Proper types = better optimization

## Type System

Polars has a rich, explicit type system. **Always define schemas when reading data.**

### Available Types

```python
import polars as pl

# Numeric types
pl.Int8, pl.Int16, pl.Int32, pl.Int64       # Signed integers
pl.UInt8, pl.UInt16, pl.UInt32, pl.UInt64   # Unsigned integers
pl.Float32, pl.Float64                       # Floating point

# Boolean
pl.Boolean

# String
pl.Utf8  # Unicode strings
pl.Categorical  # For repeated strings (more efficient)

# Temporal types
pl.Date          # Calendar date (no time)
pl.Datetime      # Date + time (with timezone support)
pl.Duration      # Time delta
pl.Time          # Time of day

# Complex types
pl.List(pl.Int64)           # List of integers
pl.Struct([                  # Nested structure
    pl.Field("name", pl.Utf8),
    pl.Field("age", pl.Int64)
])
pl.Object                    # Python objects (avoid when possible)
```

### Define Schema When Reading

**✅ ALWAYS DO THIS** - Define schema explicitly:

```python
# Define schema upfront
schema = {
    "user_id": pl.UInt32,
    "username": pl.Utf8,
    "email": pl.Utf8,
    "age": pl.UInt8,
    "signup_date": pl.Date,
    "is_active": pl.Boolean,
    "balance": pl.Float64,
    "tags": pl.List(pl.Utf8)
}

# Read with schema (lazy)
df = pl.scan_csv("users.csv", schema=schema)

# Read with schema (eager)
df = pl.read_csv("users.csv", schema=schema)

# Parquet (preserves types automatically, but you can override)
df = pl.scan_parquet("users.parquet", schema=schema)
```

**❌ AVOID** - Letting Polars infer types:

```python
# Bad - Types are inferred, may be wrong
df = pl.scan_csv("users.csv")

# Problems:
# - "123" might be read as Int64 instead of Utf8
# - Dates as strings instead of Date type
# - Nullable integers become Float64
# - Poor memory usage
```

### Schema Override (Partial)

```python
# Override specific columns, infer the rest
df = pl.scan_csv(
    "data.csv",
    schema_overrides={
        "user_id": pl.UInt32,
        "signup_date": pl.Date,
        "amount": pl.Float64
    }
)
```

### Inspect Schema

```python
# Check schema
print(df.schema)
# {'user_id': UInt32, 'username': Utf8, 'age': UInt8}

# Check dtypes
print(df.dtypes)
# [UInt32, Utf8, UInt8]

# Get schema as dict
schema_dict = df.schema
```

### Type Casting (Explicit)

```python
# Cast types explicitly
df = df.with_columns([
    pl.col("user_id").cast(pl.UInt32),
    pl.col("age").cast(pl.UInt8),
    pl.col("signup_date").str.strptime(pl.Date, "%Y-%m-%d"),
    pl.col("balance").cast(pl.Float64),
    pl.col("category").cast(pl.Categorical),  # Efficient for repeated values
])
```

### String to Date/Time Parsing

```python
# Parse date strings with explicit format
df = df.with_columns([
    pl.col("date").str.strptime(pl.Date, "%Y-%m-%d"),
    pl.col("timestamp").str.strptime(pl.Datetime, "%Y-%m-%d %H:%M:%S"),
    pl.col("time").str.strptime(pl.Time, "%H:%M:%S"),
])

# With timezone
df = df.with_columns([
    pl.col("timestamp").str.strptime(
        pl.Datetime("us", "America/New_York"),
        "%Y-%m-%d %H:%M:%S"
    )
])
```

### Categorical Types for Efficiency

```python
# Use Categorical for columns with repeated values
schema = {
    "product_id": pl.UInt32,
    "product_name": pl.Utf8,
    "category": pl.Categorical,  # "Electronics", "Books", etc. (repeated)
    "status": pl.Categorical,     # "active", "pending", "completed" (few unique)
}

df = pl.scan_csv("products.csv", schema=schema)

# Or cast existing column
df = df.with_columns([
    pl.col("category").cast(pl.Categorical)
])

# Benefits:
# - Much lower memory usage
# - Faster string operations
# - Faster groupby operations
```

### Schema Validation

```python
def validate_schema(df: pl.DataFrame, expected_schema: dict) -> None:
    """Validate that DataFrame matches expected schema"""
    actual = df.schema

    for col, expected_type in expected_schema.items():
        if col not in actual:
            raise ValueError(f"Missing column: {col}")
        if actual[col] != expected_type:
            raise TypeError(
                f"Column {col}: expected {expected_type}, got {actual[col]}"
            )

    print("✓ Schema validation passed")

# Usage
expected = {
    "user_id": pl.UInt32,
    "email": pl.Utf8,
    "age": pl.UInt8,
}

validate_schema(df.collect(), expected)
```

## Lazy Evaluation with Explicit Schemas

**Always start with LazyFrame** with explicit schema and only collect at the very end:

```python
import polars as pl

# ✅ GOOD: Lazy evaluation with explicit schema
schema = {
    "user_id": pl.UInt32,
    "age": pl.UInt8,
    "city": pl.Utf8,
    "salary": pl.Float64,
}

df = pl.scan_csv("data.csv", schema=schema)  # LazyFrame with schema
result = (
    df
    .filter(pl.col("age") > 18)
    .group_by("city")
    .agg(pl.col("salary").mean())
    .sort("salary", descending=True)
    .collect()  # Execute once at the end
)

# ❌ BAD: Eager evaluation without schema
df = pl.read_csv("data.csv")  # DataFrame (eager), inferred types
result = (
    df
    .filter(pl.col("age") > 18)  # Executes immediately
    .group_by("city")  # Executes immediately
    # ... lots of intermediate computations
)
```

## Reading Data with Schemas

### Lazy Reading (Preferred) - Always with Schema

```python
# CSV with explicit schema
schema = {
    "id": pl.UInt32,
    "name": pl.Utf8,
    "email": pl.Utf8,
    "age": pl.UInt8,
    "created_at": pl.Date,
    "is_active": pl.Boolean,
}

df = pl.scan_csv("data.csv", schema=schema)

# Parquet (best for large datasets, preserves types)
df = pl.scan_parquet("data.parquet")  # Schema preserved from file

# Multiple files with glob patterns
df = pl.scan_csv("data/*.csv", schema=schema)

# NDJSON with schema
df = pl.scan_ndjson("data.ndjson", schema=schema)

# IPC (Arrow) - schema preserved
df = pl.scan_ipc("data.arrow")

# Schema override (partial)
df = pl.scan_csv(
    "data.csv",
    schema_overrides={
        "id": pl.UInt32,
        "created_at": pl.Date,
    }
)
```

### Eager Reading (When Necessary) - Still Use Schema

```python
# Small datasets that fit in memory
schema = {
    "id": pl.UInt16,
    "value": pl.Float32,
}
df = pl.read_csv("small.csv", schema=schema)

# When you need immediate data inspection
df = pl.read_parquet("data.parquet")

# From Pandas (only when necessary) - specify schema
pandas_df = pd.read_csv("data.csv")
schema = {
    "id": pl.UInt32,
    "amount": pl.Float64,
}
df = pl.from_pandas(pandas_df, schema_overrides=schema)
```

### Reading with Date Parsing

```python
# Parse dates during read
df = pl.scan_csv(
    "data.csv",
    schema_overrides={
        "date": pl.Date,
        "timestamp": pl.Datetime,
    },
    parse_dates=True  # Auto-parse date columns
)

# Or parse after reading with explicit format
df = pl.scan_csv("data.csv").with_columns([
    pl.col("date").str.strptime(pl.Date, "%Y-%m-%d"),
    pl.col("timestamp").str.strptime(pl.Datetime, "%Y-%m-%d %H:%M:%S"),
])
```

## Basic Operations

### Selection and Filtering

```python
# Select columns
df.select([
    pl.col("name"),
    pl.col("age"),
    pl.col("salary")
])

# Or more concisely
df.select(["name", "age", "salary"])

# Select with expressions
df.select([
    pl.col("name"),
    pl.col("age"),
    (pl.col("salary") * 1.1).alias("salary_with_raise")
])

# Filter rows
df.filter(pl.col("age") > 18)

# Multiple conditions
df.filter(
    (pl.col("age") > 18) &
    (pl.col("salary") > 50000)
)

# Filter with complex logic
df.filter(
    pl.col("status").is_in(["active", "pending"]) &
    pl.col("created_at") > pl.datetime(2024, 1, 1)
)
```

### Column Operations

```python
# Add new columns
df.with_columns([
    (pl.col("salary") * 0.2).alias("tax"),
    (pl.col("first_name") + " " + pl.col("last_name")).alias("full_name"),
    pl.col("created_at").dt.year().alias("year")
])

# Rename columns
df.rename({"old_name": "new_name"})

# Drop columns
df.drop(["column1", "column2"])

# Cast types
df.with_columns([
    pl.col("age").cast(pl.Int32),
    pl.col("salary").cast(pl.Float64),
    pl.col("date").str.strptime(pl.Date, "%Y-%m-%d")
])
```

### Aggregations

```python
# Group by and aggregate
df.group_by("department").agg([
    pl.col("salary").mean().alias("avg_salary"),
    pl.col("salary").median().alias("median_salary"),
    pl.col("salary").std().alias("std_salary"),
    pl.col("employee_id").count().alias("num_employees"),
    pl.col("salary").min().alias("min_salary"),
    pl.col("salary").max().alias("max_salary")
])

# Multiple group by columns
df.group_by(["department", "city"]).agg([
    pl.col("salary").mean().alias("avg_salary")
])

# Aggregations without group by
df.select([
    pl.col("salary").mean(),
    pl.col("age").median(),
    pl.col("id").count()
])
```

### Sorting

```python
# Sort by single column
df.sort("salary", descending=True)

# Sort by multiple columns
df.sort(["department", "salary"], descending=[False, True])

# Sort with nulls handling
df.sort("salary", nulls_last=True)
```

## Advanced Operations

### Window Functions

```python
# Rank within groups
df.with_columns([
    pl.col("salary")
      .rank(method="ordinal")
      .over("department")
      .alias("salary_rank")
])

# Row number
df.with_columns([
    pl.col("employee_id")
      .cum_count()
      .over("department")
      .alias("row_num")
])

# Moving average
df.with_columns([
    pl.col("sales")
      .rolling_mean(window_size=7)
      .alias("sales_7day_ma")
])

# Lag and lead
df.with_columns([
    pl.col("price").shift(1).alias("prev_price"),
    pl.col("price").shift(-1).alias("next_price")
])

# Cumulative sum within groups
df.with_columns([
    pl.col("revenue")
      .cum_sum()
      .over("department")
      .alias("cumulative_revenue")
])
```

### Joins

```python
# Inner join
df1.join(df2, on="id", how="inner")

# Left join
df1.join(df2, on="id", how="left")

# Join on multiple columns
df1.join(df2, on=["id", "date"], how="inner")

# Join with different column names
df1.join(df2, left_on="user_id", right_on="id", how="left")

# Join with suffix for duplicate columns
df1.join(df2, on="id", how="inner", suffix="_right")

# Cross join
df1.join(df2, how="cross")
```

### Concatenation

```python
# Vertical concatenation (stack rows)
pl.concat([df1, df2, df3], how="vertical")

# Horizontal concatenation (add columns)
pl.concat([df1, df2], how="horizontal")

# Diagonal concatenation (align by column names)
pl.concat([df1, df2], how="diagonal")
```

### Pivoting and Melting

```python
# Pivot (wide format)
df.pivot(
    values="sales",
    index="product",
    columns="month"
)

# Melt (long format)
df.melt(
    id_vars=["id", "name"],
    value_vars=["jan", "feb", "mar"],
    variable_name="month",
    value_name="sales"
)
```

## Expressions: The Heart of Polars

Expressions are composable and optimized:

```python
# Chain multiple operations
df.select([
    pl.col("name").str.to_uppercase(),
    pl.col("salary").filter(pl.col("salary") > 50000).mean(),
    pl.when(pl.col("age") > 65)
      .then(pl.lit("senior"))
      .when(pl.col("age") > 18)
      .then(pl.lit("adult"))
      .otherwise(pl.lit("minor"))
      .alias("age_group")
])

# Use expressions in aggregations
df.group_by("department").agg([
    pl.col("salary").filter(pl.col("salary") > 0).mean().alias("avg_positive_salary"),
    pl.col("bonus").sum().alias("total_bonus"),
    (pl.col("salary") + pl.col("bonus")).max().alias("max_compensation")
])
```

### Conditional Logic

```python
# when/then/otherwise
df.with_columns([
    pl.when(pl.col("score") >= 90)
      .then(pl.lit("A"))
      .when(pl.col("score") >= 80)
      .then(pl.lit("B"))
      .when(pl.col("score") >= 70)
      .then(pl.lit("C"))
      .otherwise(pl.lit("F"))
      .alias("grade")
])

# Using pl.col() for conditionals
df.with_columns([
    (pl.col("salary") * pl.when(pl.col("department") == "Sales")
      .then(1.1)
      .otherwise(1.05))
      .alias("adjusted_salary")
])
```

## String Operations

```python
df.with_columns([
    # Case conversion
    pl.col("name").str.to_lowercase().alias("name_lower"),
    pl.col("name").str.to_uppercase().alias("name_upper"),

    # Extract patterns
    pl.col("email").str.extract(r"@(.+)", 1).alias("domain"),

    # Contains
    pl.col("description").str.contains("urgent").alias("is_urgent"),

    # Replace
    pl.col("phone").str.replace_all("-", "").alias("phone_clean"),

    # Split
    pl.col("full_name").str.split(" ").alias("name_parts"),

    # Length
    pl.col("description").str.len_chars().alias("desc_length"),

    # Strip whitespace
    pl.col("name").str.strip_chars().alias("name_clean"),

    # Padding
    pl.col("id").str.zfill(5).alias("id_padded")  # "123" -> "00123"
])
```

## Date/Time Operations

```python
df.with_columns([
    # Parse strings to dates
    pl.col("date_str").str.strptime(pl.Date, "%Y-%m-%d").alias("date"),

    # Extract date parts
    pl.col("timestamp").dt.year().alias("year"),
    pl.col("timestamp").dt.month().alias("month"),
    pl.col("timestamp").dt.day().alias("day"),
    pl.col("timestamp").dt.hour().alias("hour"),
    pl.col("timestamp").dt.weekday().alias("weekday"),

    # Date arithmetic
    (pl.col("timestamp") + pl.duration(days=7)).alias("next_week"),
    (pl.col("timestamp") - pl.duration(hours=24)).alias("yesterday"),

    # Truncate to period
    pl.col("timestamp").dt.truncate("1d").alias("date_only"),
    pl.col("timestamp").dt.truncate("1h").alias("hour_only"),

    # Format dates
    pl.col("date").dt.strftime("%Y-%m-%d").alias("date_formatted"),

    # Date differences
    (pl.col("end_date") - pl.col("start_date")).dt.total_days().alias("days_diff")
])
```

## Null Handling

```python
# Check for nulls
df.filter(pl.col("value").is_null())
df.filter(pl.col("value").is_not_null())

# Fill nulls
df.with_columns([
    pl.col("value").fill_null(0),
    pl.col("name").fill_null("Unknown"),
    pl.col("value").fill_null(strategy="forward"),  # Forward fill
    pl.col("value").fill_null(strategy="backward"),  # Backward fill
    pl.col("value").fill_null(pl.col("value").mean())  # Fill with mean
])

# Drop nulls
df.drop_nulls()  # Drop any row with any null
df.drop_nulls(subset=["column1", "column2"])  # Drop if nulls in specific columns

# Coalesce (first non-null value)
df.with_columns([
    pl.coalesce([pl.col("email"), pl.col("phone"), pl.lit("No contact")]).alias("contact")
])
```

## List/Array Operations

```python
# Work with list columns
df.with_columns([
    # List length
    pl.col("items").list.len().alias("num_items"),

    # Get first/last
    pl.col("items").list.first().alias("first_item"),
    pl.col("items").list.last().alias("last_item"),

    # Get by index
    pl.col("items").list.get(0).alias("first_item"),

    # Explode (unnest)
    pl.col("items").explode(),

    # Aggregations on lists
    pl.col("scores").list.mean().alias("avg_score"),
    pl.col("scores").list.sum().alias("total_score"),
    pl.col("scores").list.max().alias("max_score"),

    # Contains
    pl.col("tags").list.contains("important").alias("has_important_tag"),

    # Unique values
    pl.col("items").list.unique().alias("unique_items")
])
```

## Performance Optimization

### Query Optimization with Lazy Frames

```python
# Polars optimizes the entire query plan
df = pl.scan_csv("large_file.csv")

result = (
    df
    .filter(pl.col("year") == 2024)  # Predicate pushdown
    .select(["name", "value"])       # Projection pushdown
    .group_by("name")
    .agg(pl.col("value").sum())
    .collect()  # Executes optimized plan
)

# View the optimized plan
query = (
    df
    .filter(pl.col("year") == 2024)
    .select(["name", "value"])
    .group_by("name")
    .agg(pl.col("value").sum())
)
print(query.explain())  # Show execution plan
```

### Streaming for Large Datasets

```python
# Process data in batches (doesn't load full dataset)
result = (
    pl.scan_csv("huge_file.csv")
    .filter(pl.col("value") > 0)
    .group_by("category")
    .agg(pl.col("value").sum())
    .collect(streaming=True)  # Stream processing
)
```

### Parallel Processing

```python
# Polars automatically uses all CPU cores
# No special configuration needed

# Control thread pool size if needed
import os
os.environ["POLARS_MAX_THREADS"] = "8"
```

### Best Practices for Performance

1. **Use Parquet files** for storage (columnar, compressed)
2. **Scan instead of Read** for large files
3. **Filter early** in the query chain
4. **Select only needed columns** before joins/aggregations
5. **Use lazy evaluation** and collect once at the end
6. **Avoid converting to Pandas** until absolutely necessary

## Writing Data

```python
# Lazy write (efficient)
(
    df
    .filter(pl.col("value") > 0)
    .sink_parquet("output.parquet")  # Streams to disk
)

# Eager write
df.collect().write_parquet("output.parquet")

# CSV
df.collect().write_csv("output.csv")

# JSON (line-delimited)
df.collect().write_ndjson("output.ndjson")

# Excel (requires openpyxl)
df.collect().write_excel("output.xlsx")
```

## Converting to Pandas (Last Resort)

Only convert to Pandas when required by external packages:

```python
# ❌ Avoid this pattern
pandas_df = df.collect().to_pandas()
result = some_pandas_operation(pandas_df)

# ✅ Better: do as much as possible in Polars first
result = (
    df
    .filter(...)
    .select(...)
    .group_by(...)
    .agg(...)
    .collect()
    .to_pandas()  # Convert only at the very end
)

# ✅ Even better: check if the package accepts Arrow
result_arrow = df.collect().to_arrow()  # Many packages now accept Arrow

# Convert back from Pandas (if you must)
df = pl.from_pandas(pandas_df)
```

## Common Pandas → Polars Translations

```python
# Pandas
df = pd.read_csv("data.csv")
result = df[df["age"] > 18].groupby("city")["salary"].mean()

# Polars (Eager)
df = pl.read_csv("data.csv")
result = df.filter(pl.col("age") > 18).group_by("city").agg(pl.col("salary").mean())

# Polars (Lazy - Preferred)
df = pl.scan_csv("data.csv")
result = (
    df
    .filter(pl.col("age") > 18)
    .group_by("city")
    .agg(pl.col("salary").mean())
    .collect()
)

# Pandas: df.head()
# Polars: df.head() or df.collect().head() for lazy

# Pandas: df.shape
# Polars: df.shape or (df.height, df.width)

# Pandas: df.columns
# Polars: df.columns

# Pandas: df.dtypes
# Polars: df.dtypes or df.schema

# Pandas: df.describe()
# Polars: df.describe()

# Pandas: df.isna()
# Polars: df.null_count() or df.select(pl.all().is_null())

# Pandas: df.fillna(0)
# Polars: df.fill_null(0)

# Pandas: df.dropna()
# Polars: df.drop_nulls()

# Pandas: df.apply(func)
# Polars: Use expressions instead of apply

# Pandas: df.merge(df2)
# Polars: df.join(df2)

# Pandas: df.concat([df1, df2])
# Polars: pl.concat([df1, df2])

# Pandas: df.pivot_table()
# Polars: df.pivot()
```

## Avoid NumPy When Possible

```python
# ❌ Don't convert to NumPy for operations
numpy_array = df["column"].to_numpy()
result = np.mean(numpy_array)

# ✅ Use Polars expressions
result = df.select(pl.col("column").mean())

# ❌ Avoid NumPy array operations
df["result"] = np.where(df["value"] > 0, df["value"] * 2, 0)

# ✅ Use Polars when/then
df = df.with_columns([
    pl.when(pl.col("value") > 0)
      .then(pl.col("value") * 2)
      .otherwise(0)
      .alias("result")
])
```

## Common Patterns

### ETL Pipeline with Explicit Schema

```python
# Define schema upfront
raw_schema = {
    "product_id": pl.UInt32,
    "date": pl.Date,
    "quantity": pl.UInt32,
    "price": pl.Float64,
    "value": pl.Float64,
}

# Extract, Transform, Load - all lazy with schema
result = (
    pl.scan_csv("raw_data/*.csv", schema=raw_schema)
    # Filter bad data
    .filter(pl.col("value").is_not_null())
    .filter(pl.col("value") > 0)
    # Transform
    .with_columns([
        (pl.col("quantity") * pl.col("price")).alias("total")
    ])
    # Aggregate
    .group_by("product_id")
    .agg([
        pl.col("total").sum().alias("total_revenue"),
        pl.col("quantity").sum().alias("total_quantity")
    ])
    # Load
    .sink_parquet("output/aggregated.parquet")
)
```

### Time Series Analysis with Schema

```python
# Define schema with proper temporal types
schema = {
    "timestamp": pl.Datetime("us"),  # Microsecond precision
    "value": pl.Float64,
    "sensor_id": pl.UInt16,
}

df = (
    pl.scan_csv("timeseries.csv", schema=schema)
    .sort("timestamp")
    .with_columns([
        # Rolling statistics
        pl.col("value").rolling_mean(window_size=24).alias("moving_avg_24h"),
        pl.col("value").rolling_std(window_size=24).alias("rolling_std"),

        # Lag features
        pl.col("value").shift(1).alias("prev_value"),
        pl.col("value").shift(24).alias("value_24h_ago"),

        # Change calculations
        (pl.col("value") - pl.col("value").shift(1)).alias("change"),
        ((pl.col("value") / pl.col("value").shift(1)) - 1).alias("pct_change")
    ])
    .collect()
)
```

### Deduplication

```python
# Keep first occurrence
df = df.unique(subset=["user_id"], keep="first")

# Keep last occurrence
df = df.unique(subset=["user_id"], keep="last")

# Keep based on criteria (most recent)
df = (
    df
    .sort("timestamp", descending=True)
    .unique(subset=["user_id"], keep="first")
)
```

## Schema Patterns and Guidelines

### Choosing the Right Integer Type

```python
# Memory-efficient schema with appropriate integer types
schema = {
    # IDs - use unsigned integers
    "user_id": pl.UInt32,          # 0 to 4.3B users
    "product_id": pl.UInt32,       # 0 to 4.3B products
    "transaction_id": pl.UInt64,   # 0 to 18 quintillion transactions

    # Small integers
    "age": pl.UInt8,               # 0-255 (ages)
    "quantity": pl.UInt16,         # 0-65,535 (order quantities)
    "status_code": pl.UInt8,       # 0-255 (HTTP status codes)

    # Counters and metrics
    "page_views": pl.UInt32,       # Page view counts
    "revenue_cents": pl.Int64,     # Revenue in cents (avoid Float for money!)

    # Flags and booleans
    "is_active": pl.Boolean,
    "is_verified": pl.Boolean,
}
```

### Common Schema Patterns

#### E-commerce Schema

```python
orders_schema = {
    "order_id": pl.UInt64,
    "user_id": pl.UInt32,
    "product_id": pl.UInt32,
    "quantity": pl.UInt16,
    "price_cents": pl.UInt32,         # Store money as cents (integers)
    "order_date": pl.Date,
    "order_timestamp": pl.Datetime("us"),
    "status": pl.Categorical,          # "pending", "shipped", "delivered"
    "payment_method": pl.Categorical,  # "credit", "debit", "paypal"
    "is_gift": pl.Boolean,
    "notes": pl.Utf8,
}
```

#### Analytics/Events Schema

```python
events_schema = {
    "event_id": pl.UInt64,
    "user_id": pl.UInt32,
    "session_id": pl.Utf8,
    "event_type": pl.Categorical,      # "click", "view", "purchase"
    "timestamp": pl.Datetime("us"),
    "page_url": pl.Utf8,
    "referrer": pl.Utf8,
    "user_agent": pl.Utf8,
    "country_code": pl.Categorical,    # "US", "UK", "CA" (limited values)
    "device_type": pl.Categorical,     # "mobile", "desktop", "tablet"
    "duration_ms": pl.UInt32,
    "metadata": pl.Utf8,               # JSON string if needed
}
```

#### Time Series Schema

```python
timeseries_schema = {
    "sensor_id": pl.UInt16,
    "timestamp": pl.Datetime("ns"),    # Nanosecond precision
    "temperature": pl.Float32,         # Lower precision ok for sensors
    "humidity": pl.Float32,
    "pressure": pl.Float32,
    "location": pl.Categorical,        # "warehouse_a", "warehouse_b"
    "is_anomaly": pl.Boolean,
}
```

#### Financial Data Schema

```python
financial_schema = {
    "transaction_id": pl.UInt64,
    "account_id": pl.UInt32,
    "amount_cents": pl.Int64,          # Use cents/smallest unit for money!
    "currency": pl.Categorical,        # "USD", "EUR", "GBP"
    "transaction_type": pl.Categorical,# "debit", "credit"
    "posted_date": pl.Date,
    "transaction_timestamp": pl.Datetime("us"),
    "merchant_category": pl.Categorical,
    "description": pl.Utf8,
    "is_pending": pl.Boolean,
}
```

### Type Selection Guidelines

```python
# Strings
pl.Utf8          # Free-form text (names, descriptions, URLs)
pl.Categorical   # Repeated values (status, category, country code)
                 # Rule: < 50% unique values → use Categorical

# Integers - choose smallest that fits
pl.UInt8         # 0 to 255
pl.UInt16        # 0 to 65,535
pl.UInt32        # 0 to ~4.3 billion
pl.UInt64        # 0 to ~18 quintillion
pl.Int8/16/32/64 # When you need negative numbers

# Floats
pl.Float32       # Lower precision (6-7 digits) - sensors, approximations
pl.Float64       # Higher precision (15-16 digits) - scientific, financial ratios

# Money - ALWAYS use integers (cents)
"price_cents": pl.UInt32    # ✅ Store as cents
"price": pl.Float64         # ❌ Float precision errors!

# Dates and Times
pl.Date                     # Just the date (no time)
pl.Datetime("ms")          # Millisecond precision
pl.Datetime("us")          # Microsecond precision (default)
pl.Datetime("ns")          # Nanosecond precision
pl.Time                    # Time of day only
pl.Duration                # Time deltas

# Temporal with timezone
pl.Datetime("us", "UTC")
pl.Datetime("us", "America/New_York")
```

### Schema Validation Example

```python
from typing import Dict
import polars as pl

def create_validated_dataframe(
    file_path: str,
    schema: Dict[str, pl.DataType],
    required_columns: set[str] | None = None
) -> pl.DataFrame:
    """
    Read CSV with schema validation
    """
    # Read with explicit schema
    df = pl.read_csv(file_path, schema=schema)

    # Validate schema matches
    actual_schema = df.schema
    for col, expected_type in schema.items():
        if col not in actual_schema:
            raise ValueError(f"Missing required column: {col}")
        if actual_schema[col] != expected_type:
            raise TypeError(
                f"Column '{col}': expected {expected_type}, got {actual_schema[col]}"
            )

    # Validate required columns
    if required_columns:
        missing = required_columns - set(df.columns)
        if missing:
            raise ValueError(f"Missing required columns: {missing}")

    return df

# Usage
schema = {
    "user_id": pl.UInt32,
    "email": pl.Utf8,
    "created_at": pl.Date,
}

df = create_validated_dataframe(
    "users.csv",
    schema=schema,
    required_columns={"user_id", "email"}
)
```

### Memory Usage Comparison

```python
import polars as pl

# Bad schema - wastes memory
bad_schema = {
    "id": pl.Int64,           # Could be UInt32 → 2x memory waste
    "age": pl.Int64,          # Could be UInt8 → 8x memory waste
    "status": pl.Utf8,        # Could be Categorical → 10-100x waste
}

# Good schema - memory efficient
good_schema = {
    "id": pl.UInt32,          # 4 bytes instead of 8
    "age": pl.UInt8,          # 1 byte instead of 8
    "status": pl.Categorical, # Shared string pool
}

# Memory saved on 1M rows:
# id: (8-4) * 1M = 4 MB saved
# age: (8-1) * 1M = 7 MB saved
# status: ~90% reduction = 100+ MB saved (depending on cardinality)
# Total: 110+ MB saved on 1M rows!
```

## Best Practices Summary

### Schema and Type Best Practices

1. **ALWAYS Define Schema**: Never let Polars infer types - define them explicitly
   ```python
   # ✅ Good
   schema = {"id": pl.UInt32, "value": pl.Float64}
   df = pl.scan_csv("data.csv", schema=schema)

   # ❌ Bad
   df = pl.scan_csv("data.csv")  # Types inferred
   ```

2. **Use Appropriate Integer Types**: Save memory with smaller types
   - `pl.UInt8` for 0-255 (status codes, categories)
   - `pl.UInt16` for 0-65,535 (small IDs)
   - `pl.UInt32` for 0-4B (user IDs, product IDs)
   - `pl.Int8/16/32/64` when you need negative numbers

3. **Use Categorical for Repeated Strings**: Much more efficient
   ```python
   # Status: "active", "pending", "completed" → use pl.Categorical
   # Country codes, product categories, etc. → use pl.Categorical
   ```

4. **Parse Dates Properly**: Don't leave dates as strings
   ```python
   # ✅ Good
   schema = {"date": pl.Date, "timestamp": pl.Datetime}

   # Or parse explicitly
   .with_columns([pl.col("date").str.strptime(pl.Date, "%Y-%m-%d")])
   ```

5. **Validate Schema After Reading**:
   ```python
   df = pl.scan_csv("data.csv", schema=expected_schema)
   print(df.schema)  # Verify types are correct
   ```

### General Best Practices

6. **Start Lazy**: Use `scan_*` methods instead of `read_*`
7. **End Lazy**: Call `.collect()` only once at the very end
8. **Filter Early**: Apply filters before joins and aggregations
9. **Select Smart**: Only select columns you need
10. **Use Expressions**: Leverage Polars' expression API, avoid Python loops
11. **Avoid Pandas**: Only convert to Pandas when absolutely required
12. **Avoid NumPy**: Use Polars operations instead of NumPy arrays
13. **Use Parquet**: Best format for large datasets (preserves types!)
14. **Stream Big Data**: Use `streaming=True` for huge datasets

## When to Use Pandas

Only use Pandas when:
- A library explicitly requires Pandas DataFrames (scikit-learn, some plotting libraries)
- You need functionality not yet in Polars
- Working with tiny datasets where performance doesn't matter

**Always do as much processing in Polars as possible before converting.**

## Installation

```bash
# Basic installation
pip install polars

# With optional dependencies
pip install polars[numpy,pandas,pyarrow]

# Or with uv (modern Python package manager)
uv pip install polars
```

## Resources

- [Polars Documentation](https://pola-rs.github.io/polars/)
- [Polars GitHub](https://github.com/pola-rs/polars)
- [User Guide](https://pola-rs.github.io/polars-book/)
