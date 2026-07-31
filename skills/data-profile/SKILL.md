---
name: data-profile
description: Use when meeting an unfamiliar table or dataset — produce a structured profile (grain, per-column stats, coverage, red flags) before building on it, writing quality checks for it, or debugging it. First move on any new data.
tools: Bash, Read, Grep, Glob
---

# Profile Dataset

The goal is not statistics, it's answers: what's the grain, can I trust the
keys, where are the gaps, what will bite me. Profile before you build.

## Sequence

### 1. Shape and Grain

```sql
select count(*) as rows, count(distinct id) as distinct_ids from t;
```

Test the *claimed* grain: if `count(*) > count(distinct <claimed_key>)`,
find out why before anything else — everything downstream depends on it.

```sql
select id, count(*) as n from t group by id having count(*) > 1
order by n desc limit 10;   -- then eyeball a duplicated id's full rows
```

### 2. Per-Column Profile

For each column: null rate, distinct count, min/max, top values.

```sql
select
  count(*) as n,
  countif(status is null) / count(*)          as null_rate_status,
  count(distinct status)                       as distinct_status,
  min(created_at) as min_created, max(created_at) as max_created
from t;

select status, count(*) as n from t group by status order by n desc limit 15;
```

In Python with the data in hand, `df.describe()` in polars plus per-column
`value_counts` on low-cardinality columns covers the same ground faster than
hand-writing SQL per column.

### 3. Time Coverage

```sql
select date_trunc(dt, month) as month, count(*) as n
from t group by 1 order by 1;
```

Look for: gaps (missing loads), cliffs (schema/logic change dates — a column
that's 100% null before 2024-03 isn't "60% null", it's "added 2024-03"),
ramps (backfill vs organic), and the true freshness (`max(loaded_at)` vs
what the docs claim).

### 4. Relationships (when a table references others)

```sql
select count(*) filter (where d.id is null) as orphans
from fct f left join dim d on f.dim_id = d.id;
```

## Red-Flag Table

| Observation | Suspicion |
|-------------|-----------|
| Claimed key has duplicates | Wrong grain doc, or a fanout join upstream |
| Column ~99% one value | Dead flag, or default masking missing data |
| Constant column | Dead — or environment-specific (all rows from one source) |
| Null rate cliff at a date | Column added/removed then; segment all stats by that date |
| `min(date)` = 1970-01-01 / max in the future | Sentinel values polluting every aggregate |
| Numeric with negative min where impossible | Sign convention (refunds?) — ask, don't assume |
| String col with 2 distinct casings of same value | No normalization upstream; joins will miss |
| Row count cliff/ramp | Backfill boundary or lost source — find the event |

## Output Format

```
profile: analytics.orders_events (queried 2026-07-02)
grain:    claimed order_id+changed_at — HOLDS (0 dups over 12.4M rows)
coverage: 2023-01-04 → 2026-07-01, gap 2024-11-02..04 (3 days, ~40k rows)
freshness: max loaded_at 2026-07-02 05:14 UTC (~2h lag)

column        type       nulls   distinct  notes
order_id      string     0%      8.1M      key component
status        string     0%      5         'returned' appears only after 2024-06
amount_usd    numeric    0.02%   —         min -840.00 (refunds?), max 92k
changed_at    timestamp  0%      —         3% rows at exactly 00:00:00 (batch artifact?)

red flags:
  1. 3-day gap Nov 2024 — confirmed incident or silent loss?
  2. negative amounts undocumented — sign convention needs an owner answer
suggested checks: unique(order_id, changed_at); freshness < 6h; amount ≥ -X pending #2
```

Every profile ends with red flags ranked by risk and the checks this profile
justifies (feed data-quality-checks).

## Rules

- Verify the grain first; every other number is uninterpretable without it.
- Never profile only the latest partition — cliffs and gaps live in history.
  On huge tables: full scan on time-bucketed counts, sample only the
  per-column value stats, and label sampled numbers as sampled.
- Distinguish "always been 60% null" from "100% null until March" — segment
  by time before reporting any rate.
- Hunt sentinels (1970-01-01, 9999-12-31, -1, 'N/A', '') explicitly; they
  silently poison min/max/avg.
- Eyeball 5–10 raw rows. Aggregates lie about things a human spots instantly.
- Red flags are questions for the data owner, not accusations — "negative
  amounts: convention or bug?"
- End with suggested checks; a profile that doesn't produce checks was
  tourism.
