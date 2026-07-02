---
name: polars-write
description: Use when working with DataFrames in Python, processing large datasets, or building transformations, aggregations, and ETL pipelines. Prefers Polars LazyFrames over Pandas/NumPy.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Polars

DataFrames done right: lazy, expression-based, and fast. Polars replaces Pandas/NumPy for all data work.

## Lazy First — Always

Start every pipeline with a `scan_*` method, build the whole query lazily, and call `.collect()` exactly once at the end. The optimizer pushes filters and column pruning down to the file reader — eager `read_*` throws that away.

```python
import polars as pl

result: pl.DataFrame = (
    pl.scan_parquet("data/events/*.parquet")   # LazyFrame — nothing is read yet
    .filter(pl.col("event_date") >= pl.date(2026, 1, 1))
    .select("customer_id", "event_type", "amount", "event_date")
    .collect()                                  # single collect at the end
)
```

| Source | Lazy (use this) | Eager (avoid) |
|--------|-----------------|----------------|
| Parquet | `pl.scan_parquet` | `pl.read_parquet` |
| CSV | `pl.scan_csv` | `pl.read_csv` |
| NDJSON | `pl.scan_ndjson` | `pl.read_json` |
| Database | `pl.read_database_uri(...).lazy()` | — |

Inspect a plan with `lf.explain()`; debug intermediate steps with `lf.head(20).collect()` — never collect the full frame just to look at it.

## Expressions, Not Loops

All logic goes through expressions on `pl.col`. Never iterate rows; avoid `map_elements` (it drops to Python speed) unless no expression exists.

```python
lf = pl.scan_parquet("data/orders/*.parquet").with_columns(
    revenue_usd=pl.col("amount") * pl.col("fx_rate"),
    order_month=pl.col("ordered_at").dt.truncate("1mo"),
    tier=(
        pl.when(pl.col("amount") >= 1_000).then(pl.lit("high"))
        .when(pl.col("amount") >= 100).then(pl.lit("mid"))
        .otherwise(pl.lit("low"))
    ),
    email_domain=pl.col("email").str.split("@").list.last(),
)
```

Column-family helpers: `pl.all()`, `pl.col(pl.Float64)`, `pl.col("^sales_.*$")`, `.name.suffix("_clean")`.

## Group By / Aggregate

```python
summary = (
    lf.group_by("customer_id", "tier")
    .agg(
        total_revenue=pl.col("revenue_usd").sum(),
        n_orders=pl.len(),
        avg_order=pl.col("revenue_usd").mean(),
        first_order=pl.col("ordered_at").min(),
        big_orders=pl.col("revenue_usd").filter(pl.col("revenue_usd") > 500).count(),
    )
    .sort("total_revenue", descending=True)
    .collect()
)
```

Window functions without leaving the pipeline: `pl.col("revenue_usd").sum().over("customer_id")`. Time-based grouping: `group_by_dynamic(index_column="ordered_at", every="1w")`.

## Joins

```python
enriched = orders_lf.join(
    customers_lf,
    on="customer_id",
    how="left",          # inner | left | full | semi | anti | cross
    validate="m:1",      # fail fast on unexpected duplicates
).join_asof(
    fx_rates_lf.sort("rate_date"),
    left_on="ordered_at",
    right_on="rate_date",
    strategy="backward",
)
```

Use `how="semi"` / `how="anti"` for existence filtering instead of joining and dropping columns. Always pass `validate=` when you believe a join is 1:1 or m:1 — silent fan-out is the classic data bug.

## Larger-than-RAM: Streaming

```python
# Streaming collect for big aggregations
big = pl.scan_parquet("s3://bucket/events/**/*.parquet")
top = big.group_by("user_id").agg(pl.len()).collect(engine="streaming")

# Or never materialize at all — write straight to disk
big.filter(pl.col("status") == "ok").sink_parquet("clean_events.parquet")
```

`sink_parquet` / `sink_csv` execute the lazy plan and stream to disk without holding the result in memory.

## The Pandas Boundary

Do NOT use Pandas or NumPy for data manipulation. Convert only when an external package demands it, at the last possible moment:

```python
import polars as pl

df = lf.collect()
model.fit(X=df.select("f1", "f2").to_numpy(), y=df["label"].to_numpy())
legacy_lib.process(frame=df.to_pandas())          # only at this boundary
back = pl.from_pandas(legacy_result)              # and convert back immediately
```

## Rules

- `scan_*` + one `.collect()` at the end. If a pipeline has multiple collects, refactor it.
- No Pandas, no NumPy, unless a third-party API requires it — convert with `.to_pandas()` / `.to_numpy()` at that boundary only, then come back to Polars.
- Expressions over `map_elements`; `map_elements` over row loops; row loops never.
- Name aggregations with keyword arguments (`total=pl.col("x").sum()`), not `.alias()` chains.
- Pass `validate=` on every join where cardinality matters; use `semi`/`anti` joins for filtering.
- Set dtypes at scan time (`scan_csv(..., schema_overrides={"id": pl.Int64})`) — don't cast after the fact.
- For larger-than-RAM work: `collect(engine="streaming")` or `sink_parquet`, not chunked manual loops.
- Parquet over CSV for any intermediate storage; CSV only at ingestion/export edges.
- `pl.len()` for row counts in aggregations, `pl.lit()` for literals in `when/then` — bare strings mean column names.
- Keep pipelines as single fluent chains wrapped in parentheses; assign a typed `pl.DataFrame` / `pl.LazyFrame` at meaningful checkpoints only.
