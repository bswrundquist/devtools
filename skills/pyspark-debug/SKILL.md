---
name: pyspark-debug
description: Use when a PySpark job is slow, failing, or producing wrong results — OOMs, skew, shuffle storms, spills, stuck stages. Covers reading query plans and the Spark UI, then fixing the code. Also the reference for writing efficient PySpark in the first place.
tools: Bash, Read, Grep, Glob
---

# PySpark Debug

Diagnose before touching code: the plan and the Spark UI tell you which of the
five classic problems you have. Guessing at config is how jobs stay slow.

## First Moves

1. Reproduce the symptom's evidence, not the run: get the query plan and the
   Spark UI (or event log) for the failing/slow job.
2. `df.explain(mode="formatted")` — count the `Exchange` nodes. Every Exchange
   is a shuffle; unexplained shuffles are the #1 perf bug.
3. In the UI, find the slowest stage, then look at **task time distribution**:
   max task time >> median task time = skew. All tasks slow = wrong plan or
   under-provisioned.

## Failure Table

| Symptom | Likely cause | Fix |
|---------|-------------|-----|
| Executor OOM / `ExecutorLostFailure` | Skewed key, huge partition, wide row explode | AQE skew join, salt the key, more partitions |
| Driver OOM | `collect()`, `toPandas()`, broadcast of a big table | Aggregate in Spark; check `autoBroadcastJoinThreshold` |
| Stage stuck at 199/200 tasks | Skew — one key holds most rows | Find the key (below), salt or isolate it |
| Massive spill (memory/disk) in UI | Partitions too large for executor memory | Raise `spark.sql.shuffle.partitions`, repartition |
| Thousands of tiny tasks, low CPU | Too many partitions / small-files input | `coalesce` output; compact input files |
| Wrong results after rerun | Non-deterministic UDF, overwrite semantics | Make idempotent; check partition overwrite mode |
| Slow with Python UDFs in plan (`BatchEvalPython`) | Row-at-a-time serialization | Replace with built-ins or `pandas_udf` |

## Find the Skewed Key

```python
(df.groupBy("join_key").count()
   .orderBy(F.desc("count"))
   .show(20))
```

If the top key holds an order of magnitude more rows than the median, that's
the problem. Fixes in order of preference:

1. **AQE** (on by default since 3.2 — verify, don't assume):
   ```python
   spark.conf.set("spark.sql.adaptive.enabled", "true")
   spark.conf.set("spark.sql.adaptive.skewJoin.enabled", "true")
   ```
2. **Broadcast** the small side if it fits: `F.broadcast(dim_df)`.
3. **Salt**: add a random suffix 0..N to the hot key on the big side,
   explode the small side across all N suffixes, join on (key, salt).
4. **Isolate**: union of `hot_keys` join and `rest` join.

## Reading `explain()`

- `Exchange hashpartitioning(...)` — shuffle. Two Exchanges feeding a join =
  sort-merge join; one side Exchange-free = it was already partitioned right.
- `BroadcastHashJoin` — good for small dims. If you see `SortMergeJoin`
  against a table you know is small, Spark misestimated — broadcast manually.
- `Filter` **below** `Scan` (as pushed filters) is what you want. A Filter far
  above the Scan means predicate pushdown failed — often a UDF or a cast in
  the predicate.
- `BatchEvalPython` — a Python UDF blocking codegen and pushdown. Hostile to
  performance; eliminate it.

## Shuffle and Partition Sizing

- Target **100–200 MB per shuffle partition**. `spark.sql.shuffle.partitions`
  default of 200 is wrong for almost every job — size it to
  `total_shuffle_bytes / 128MB`, or let AQE coalesce.
- `repartition(n)` shuffles; `coalesce(n)` doesn't (only merges). Use
  `repartition` to fix skew/parallelism, `coalesce` only to reduce output
  file count.
- Writing: `df.repartition("dt").write.partitionBy("dt")` gives one file per
  partition value instead of `n_tasks × n_values` small files.

## Writing It Right (so you don't debug it later)

- Built-in functions > `pandas_udf` > Python UDF. Check
  `pyspark.sql.functions` first — it almost certainly exists.
- Filter and select **early**; drop columns before joins and shuffles.
- `cache()` only when a DataFrame is reused ≥2 times **and** recomputing is
  expensive; always `unpersist()`. Verify it's actually cached in the UI
  Storage tab — lazy eval means `cache()` alone does nothing.
- No `collect()` on anything unbounded. `toPandas()` only after aggregation,
  with Arrow enabled (`spark.sql.execution.arrow.pyspark.enabled=true`).
- Test locally: `SparkSession.builder.master("local[2]")` with a few hundred
  rows of representative fake data exercises the same plan.

## Rules

- Read the plan/UI first. Config changes without a diagnosis are cargo cult.
- Skew is found by measuring key distribution, never assumed.
- Every `Exchange` in the plan must be explainable; every Python UDF must be
  justified in a comment or removed.
- Never `collect()`/`toPandas()` on unaggregated data.
- Size shuffle partitions to ~128 MB; don't ship the 200 default.
- Reruns must be idempotent — dynamic partition overwrite or explicit
  delete-then-write, never blind append.
- After any fix, compare row counts and a checksum against the pre-fix output
  (see data-diff skill) — faster is worthless if it's also wrong.
