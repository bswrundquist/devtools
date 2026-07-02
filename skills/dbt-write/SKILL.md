---
name: dbt-write
description: Use when creating dbt models, writing dbt SQL, configuring dbt projects, debugging dbt runs, or writing dbt tests. Covers layering, sources, materializations, tests, and node selection.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# dbt

Build maintainable dbt projects: layered models, tested contracts, incremental where it pays.

## Layering

| Layer | Prefix | Purpose | Materialization |
|-------|--------|---------|-----------------|
| staging | `stg_` | 1:1 with source tables — rename, cast, light cleanup only | `view` |
| intermediate | `int_` | Reusable joins/pivots between staging and marts | `view` or `ephemeral` |
| marts | `fct_` / `dim_` | Business-facing facts and dimensions | `table` or `incremental` |

Staging models select from `source()` only. Everything downstream selects from `ref()` only. No model ever references a raw table by name — that breaks lineage, `--select` graphs, and environment swapping.

```sql
-- models/staging/shop/stg_shop__orders.sql
select
    id                             as order_id,
    customer_id,
    cast(total as numeric(12, 2)) as total_amount,
    created_at                     as ordered_at
from {{ source('shop', 'orders') }}
```

## Sources and Freshness

```yaml
# models/staging/shop/_shop__sources.yml
sources:
  - name: shop
    database: raw
    schema: shop
    freshness:
      warn_after: {count: 12, period: hour}
      error_after: {count: 24, period: hour}
    loaded_at_field: _loaded_at
    tables:
      - name: orders
      - name: customers
```

Check with `dbt source freshness` — wire it into CI so stale pipelines fail loudly, not silently.

## Materializations

Default to `view`; promote to `table` when downstream queries are slow; go `incremental` only when a full rebuild is measurably too expensive.

```sql
{{ config(
    materialized='incremental',
    unique_key='event_id',
    on_schema_change='append_new_columns',
) }}

select event_id, user_id, event_type, event_at
from {{ ref('stg_app__events') }}
{% if is_incremental() %}
  -- lookback window absorbs late-arriving data
  where event_at > (select max(event_at) - interval '3 days' from {{ this }})
{% endif %}
```

Always set `unique_key` for incremental models or you will get duplicates on reruns. Test with `dbt run --select my_model --full-refresh` after any logic change.

## Tests

Generic tests in YAML on every mart's primary key at minimum:

```yaml
models:
  - name: fct_orders
    columns:
      - name: order_id
        tests: [unique, not_null]
      - name: status
        tests:
          - accepted_values:
              values: ['placed', 'shipped', 'returned']
      - name: customer_id
        tests:
          - relationships:
              to: ref('dim_customers')
              field: customer_id
```

Singular tests for business logic — a query that returns failing rows:

```sql
-- tests/assert_no_negative_order_totals.sql
select order_id, total_amount
from {{ ref('fct_orders') }}
where total_amount < 0
```

## Documentation

```yaml
models:
  - name: fct_orders
    description: '{{ doc("fct_orders") }}'
```

```md
{% docs fct_orders %}
One row per order. Grain: order_id. Excludes test orders (`is_test = true`).
{% enddocs %}
```

Document grain and exclusions — those are the two things the next person cannot guess.

## Commands

Prefer `dbt build` — it runs and tests each node in DAG order, so a failing test stops bad data from feeding downstream models. `dbt run` then `dbt test` validates only after everything already ran.

```bash
dbt build --select +fct_orders          # model, all upstream, and their tests
dbt build --select state:modified+ --defer --state prod-artifacts/   # CI: only what changed
dbt compile --select fct_orders          # inspect rendered SQL in target/compiled/
dbt source freshness
dbt run --select fct_orders --full-refresh
```

| Selector | Meaning |
|----------|---------|
| `+model` | model and all upstream |
| `model+` | model and all downstream |
| `+model+` | full lineage through model |
| `@model` | model, downstream, and their upstreams |
| `state:modified` | changed vs `--state` manifest (CI slim runs) |
| `tag:nightly`, `path:models/marts` | by tag / by path |

## Debugging

1. `dbt debug` — connection and profile problems.
2. `dbt compile --select bad_model`, then read `target/compiled/.../bad_model.sql` and run it directly in the warehouse. Jinja errors show at compile; SQL errors show at run.
3. Test failures: `dbt build --store-failures`, then query the failures table it prints.
4. Wrong incremental results: `--full-refresh` the model; if that fixes it, the `is_incremental()` predicate or `unique_key` is wrong.

## Rules

- `source()` only in staging; `ref()` everywhere else. Zero hardcoded table names.
- One staging model per source table; no joins in staging.
- Every model's primary key gets `unique` + `not_null` tests. No exceptions.
- Use `dbt build`, not `run` + `test`, so tests gate downstream models.
- Incremental models: always `unique_key`, always a late-data lookback window, always retest with `--full-refresh`.
- Keep business logic out of BI tools — if a metric is defined in a dashboard, move it into a mart.
- Name models so the file name equals the table name equals the `ref()` argument.
- CI runs `dbt build --select state:modified+` against production artifacts — never rebuild the whole DAG per PR.
- Don't `select *` from sources in staging; list columns so schema drift fails loudly at compile time.
