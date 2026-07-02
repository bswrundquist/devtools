---
name: data-quality-checks
description: Use when designing or implementing data quality checks — freshness, volume, schema, uniqueness, referential integrity, distribution — and choosing where they run (dbt tests, pandera, Great Expectations, plain SQL) and what happens when they fail.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Data Quality Checks

A check is only real if it has an owner, a severity, and a defined action on
failure. Coverage without consequences is dashboard theater.

## Check Taxonomy

Cover a table in this order — each tier catches what the previous can't:

| Tier | Check | Catches | Example |
|------|-------|---------|---------|
| 1 | Freshness | Pipeline silently stopped | `max(loaded_at) > now() - interval '6 hours'` |
| 2 | Volume | Partial loads, duplicated loads | today's rows within ±3σ of trailing 28-day same-weekday |
| 3 | Schema | Upstream contract drift | column set + types match expectation |
| 4 | Primary key | Fanout from bad joins, dup loads | `unique` + `not_null` on the grain |
| 5 | Referential integrity | Orphaned facts | every `fct.customer_id` in `dim_customers` |
| 6 | Values/distribution | Semantic breakage with healthy plumbing | null-rate jump, `accepted_values`, amount ≥ 0 |
| 7 | Reconciliation | Cross-system drift | warehouse totals vs source-of-truth totals |

Tiers 1–4 on **every** production table. 5–7 where money, reports, or ML
features depend on the answer.

## Blocking vs Monitoring

Decide per check, explicitly:

- **Blocking** (fails the pipeline, downstream never sees bad data):
  PK violations, schema breaks, empty inputs. Use for anything that would
  *corrupt* downstream state.
- **Monitoring** (data lands, alert fires): freshness lag, volume anomalies,
  distribution drift. Use for anything that needs a human judgment call.

A blocking check on a noisy signal trains people to force-rerun pipelines. A
monitoring alert on a corrupting signal means cleaning up downstream tables
at 2am. Classify carefully.

## Tool Choice

| Situation | Tool |
|-----------|------|
| dbt project exists | dbt tests — no exceptions, keep checks next to models |
| Python pipeline, DataFrame in hand | pandera schema at ingest/egress boundaries |
| Warehouse tables outside dbt | scheduled SQL checks (below) |
| Cross-team, mixed stack, need UI/docs | Great Expectations or Soda — accept the framework weight only at that scale |

Don't introduce a DQ framework to do what nine dbt tests would do.

## Patterns

dbt (see dbt-write skill for testing conventions) — severity separates blocking
from monitoring:

```yaml
- name: fct_payments
  columns:
    - name: payment_id
      tests: [unique, not_null]           # blocking by default
    - name: amount_usd
      tests:
        - dbt_utils.accepted_range:
            min_value: 0
            config: {severity: warn}      # monitoring
```

pandera at a Python boundary:

```python
import pandera.polars as pa

class Payments(pa.DataFrameModel):
    payment_id: str = pa.Field(unique=True)
    amount_usd: float = pa.Field(ge=0)
    status: str = pa.Field(isin=["pending", "settled", "refunded"])

df = Payments.validate(df, lazy=True)     # lazy=True → all failures, not just the first
```

Plain-SQL volume check with a seasonal baseline (compare same weekday, not
yesterday):

```sql
with today as (
  select count(*) as n from events where dt = current_date
),
baseline as (
  select avg(n) as mean, stddev(n) as sd
  from (select dt, count(*) as n from events
        where dt >= current_date - 84
          and extract(dayofweek from dt) = extract(dayofweek from current_date)
        group by dt)
)
select today.n, baseline.mean, baseline.sd,
       abs(today.n - mean) > 3 * sd as anomalous
from today, baseline
```

## Failure Metadata

Every check ships with:

- **Owner** — the team paged or tagged.
- **Severity** — blocking / page / ticket / log.
- **Action** — one line: what the responder does first (link runbook if more).
- **Blast radius** — which downstream tables/dashboards/models consume this.

If you can't write the action line, the check isn't ready to exist.

## Rules

- Tiers 1–4 (freshness, volume, schema, PK) on every production table before
  any fancier checks.
- Every check has owner + severity + first-action. No orphan checks.
- Blocking for corruption, monitoring for judgment calls — never invert.
- Volume baselines respect seasonality: same weekday trailing window, never
  plain yesterday.
- An alert that fires weekly and gets ignored gets fixed or deleted —
  alert fatigue is a data quality failure mode of its own.
- Put checks where the data crosses a boundary (ingest, publish), not
  scattered mid-pipeline.
- Test the check: inject a known violation and confirm it fires before
  trusting it in production.
- New datasets get profiled first (profile-dataset skill) — thresholds
  guessed without a baseline are noise generators.
