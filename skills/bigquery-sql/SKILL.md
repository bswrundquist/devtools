---
name: bigquery-sql
description: Use when writing or optimizing BigQuery SQL — partitioning and clustering choices, cost estimation and control, MERGE/dedup patterns, BQ dialect features (QUALIFY, structs, arrays), and why a query scans more than it should.
tools: Bash, Read, Grep, Glob
---

# BigQuery SQL

You pay per byte scanned (on-demand) or per slot-time — every query is
written against that fact. Pruning is the whole game; the generic sql-write
skill covers style, this covers what BigQuery does differently.

## Cost Before Running

```bash
bq query --dry_run --use_legacy_sql=false 'select ... '   # bytes it WILL scan
```

- Dry-run anything unfamiliar; the estimate is free and exact for on-demand.
- Set guardrails: `maximum_bytes_billed` on ad-hoc sessions.
- `select *` on a columnar store scans every column — name columns, or use
  `select * except(big_json_col)`.
- Find what's expensive: `INFORMATION_SCHEMA.JOBS_BY_PROJECT` (bytes billed
  by user/query over time), `TABLE_STORAGE` for table sizes.
- `LIMIT` does **not** reduce scan. Preview with the free table preview or
  `tablesample system (1 percent)`, never `select * ... limit 10`.

## Partitioning and Clustering

| Choice | Use |
|--------|-----|
| Partition by `DATE(event_ts)` or date column | Time-series facts — the default |
| Ingestion-time partitioning | Loads without a good event date; query via `_PARTITIONTIME` |
| Integer-range partition | Rare; tenant/bucket ids |
| Clustering (up to 4 cols) | High-cardinality filter/join keys *within* partitions — order matters, most-filtered first |

- New fact tables: partition on the date you filter by + cluster on the ids
  you join/filter by, and set `require_partition_filter = true` so nobody
  full-scans by accident.
- Partitions prune only on **direct predicates**. These defeat pruning:
  - `where timestamp_trunc(event_ts, day) = ...` — function on the column
  - `where dt = (select max(dt) from t)` — non-constant (use a scripting
    variable: `declare max_dt date default (select ...);`)
  - joining to a date dimension instead of filtering directly
- Clustering benefits show in bytes scanned; verify with dry-run before/after
  rather than trusting intuition.

## Dialect Worth Using

```sql
-- QUALIFY: filter on window functions without a subquery
select *, row_number() over (partition by user_id order by ts desc) as rn
from events
qualify rn = 1;

-- Arrays/structs are first-class; unnest to rows
select e.user_id, item.sku
from events e, unnest(e.items) as item;

-- safe_ variants return null instead of erroring
select safe_divide(revenue, users), safe_cast(raw as int64);
```

- Dedup grain: `qualify row_number() over (partition by <key> order by
  loaded_at desc) = 1` — the idiomatic latest-record-per-key.
- `DATE(timestamp_col)` is UTC unless you pass a zone: `DATE(ts, 'America/
  Chicago')`. Timezone bugs in partition filters are a classic off-by-one-day.
- Named parameters / `declare` for scripts; `create temp table` beats deeply
  nested CTE pyramids for multi-step transforms.

## Writing Data

```sql
merge target t
using staged s on t.id = s.id and t.dt = s.dt   -- include partition key: enables pruning
when matched then update set ...
when not matched then insert (...) values (...);
```

- MERGE for upserts; include the partition column in the `on` clause or the
  merge scans the whole target.
- Idempotent batch loads: overwrite the partition
  (`insert overwrite`-equivalent via `merge`/partition decorator
  `table$20260701`, or write-truncate on the partition), never blind append.
- Streaming inserts land in a buffer that DML can't touch for up to ~90
  minutes — "merge says done but rows persist" is usually this.

## Debugging a Slow/Expensive Query

1. Dry-run: bytes scanned as expected? No → pruning is broken (see above).
2. Execution details (query plan): find the stage with max slot-time.
   Repartition/shuffle-heavy stages → join on a skewed or high-cardinality
   key; aggregate before joining.
3. Joining two big tables on a string key → consider clustering both on it.
4. Same subquery referenced twice runs twice — CTEs are not materialized;
   use a temp table.

## Rules

- Dry-run before running anything you haven't run today; know the bytes.
- Every new fact table: date partition + clustering + `require_partition_filter`.
- Never a function or subquery on the partition column in a `where`.
- Never `select *` outside exploration; never `limit` as a cost control.
- Dedup with `qualify row_number() = 1` at a stated grain.
- MERGE conditions include the partition key.
- Timezone explicit in every `DATE(ts, ...)` on a timestamp.
- Batch writes are idempotent per partition — reruns must not double-count
  (verify with data-diff on a rerun partition).
