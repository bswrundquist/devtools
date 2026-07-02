---
name: data-diff
description: Use when comparing two tables or datasets — before/after a pipeline change, source vs destination after a migration, dev vs prod models — to prove they match or pinpoint exactly where they don't. Row counts, per-column checksums, then row-level drill-down.
tools: Bash, Read, Grep, Glob
---

# Data Diff

Coarse to fine: counts, then column checksums, then row-level diff on the
narrowed slice. Never start row-by-row on the whole table.

## Level 0 — Establish Comparability

Before comparing anything, confirm: same grain, same filter window, same
timezone on the partition column, and a primary key. Half of all "diffs" are
two queries that don't mean the same thing. If there's no PK, derive the
grain first — a diff without a key can only compare aggregates.

## Level 1 — Counts by Partition

```sql
select coalesce(a.dt, b.dt) as dt, a.n as n_a, b.n as n_b, a.n - b.n as delta
from (select dt, count(*) as n from table_a group by dt) a
full outer join (select dt, count(*) as n from table_b group by dt) b using (dt)
where a.n is distinct from b.n
order by dt
```

By partition, not in total — offsetting differences (+50 in Monday, −50 in
Tuesday) hide in totals and instantly localize in partitions.

## Level 2 — Key Overlap and Column Checksums

Key overlap — which rows exist where:

```sql
select
  count(*) filter (where b.id is null) as only_in_a,
  count(*) filter (where a.id is null) as only_in_b,
  count(*) filter (where a.id is not null and b.id is not null) as in_both
from table_a a full outer join table_b b using (id)
```

Per-column checksums over the intersection find *which columns* differ
without moving row data:

```sql
-- BigQuery; for Postgres use sum(hashtext(col::text))
select
  sum(farm_fingerprint(cast(amount as string)))   as h_amount,
  sum(farm_fingerprint(status))                   as h_status,
  countif(amount is null)                          as nulls_amount
from table_x
where dt between @start and @end
```

Run per table, compare per column. Matching hash + matching null count =
column verified; move on. Only mismatched columns proceed to Level 3.

## Level 3 — Row-Level on the Narrowed Slice

```sql
select a.id, a.amount as amount_a, b.amount as amount_b
from table_a a join table_b b using (id)
where a.amount is distinct from b.amount    -- null-safe; <> drops null rows
limit 100
```

Then characterize the pattern before concluding: all diffs in one partition
(bad backfill)? one enum value (mapping change)? constant offset (timezone,
rounding)? random scatter (non-determinism)?

## False-Positive Traps

| Trap | Symptom | Fix |
|------|---------|-----|
| Float representation | Tiny scattered numeric diffs | `abs(a-b) > 1e-9`, or round to the column's real precision |
| Timestamp precision/tz | Constant offset (1h, 5h) or µs noise | Compare in UTC, truncate to the coarser precision |
| Null vs empty string | String columns "differ" invisibly | `nullif(col, '')` both sides |
| Non-deterministic source | Diff changes on every run | Snapshot both sides first, diff the snapshots |
| Unordered arrays/JSON | Same content, different bytes | Sort/canonicalize before hashing |

## Report Format

```
data-diff: table_a vs table_b (dt 2026-06-01..2026-06-30)
  rows:        12,401,332 vs 12,401,340  (+8, all in dt=2026-06-14)
  keys:        8 only in b, 0 only in a
  columns:     amount ✓  status ✗ (14,203 rows)  changed_at ✓ ...
  status diff: all 'returned' → 'refunded'; first seen dt=2026-06-14
  verdict:     NOT equivalent — enum rename + 8 late rows; both traced to <cause>
```

Always: scope, counts, key overlap, per-column verdict, characterized
pattern, and a verdict with the *reason*, not just the numbers.

## Rules

- Coarse to fine — counts, checksums, then rows. Row-level first wastes time
  and money on big tables.
- Partition every comparison; totals hide offsetting errors.
- `is distinct from`, never `<>`, when nulls exist (BigQuery: compare with
  `coalesce` or `IS NOT DISTINCT FROM` where supported).
- Numeric/timestamp comparisons state their tolerance explicitly.
- Snapshot moving sources before diffing.
- Sample and characterize mismatches — "14,203 rows differ" is not a
  finding; "all `returned` became `refunded` after the 06-14 deploy" is.
- Cost-check checksum queries on huge tables: restrict to recent partitions
  first, widen only if they disagree.
- The diff ends with a verdict: equivalent (within stated tolerance) or not,
  and if not, why.
