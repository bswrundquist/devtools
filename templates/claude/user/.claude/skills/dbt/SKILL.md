---
name: dbt
description: Use when working with dbt (data build tool), creating dbt models, writing dbt SQL, configuring dbt projects, debugging dbt runs, creating tests, or discussing dbt best practices and project structure.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# dbt

Expert guidance for working with dbt (data build tool) projects.

## Core dbt Concepts

### Project Structure
```
dbt_project/
├── dbt_project.yml          # Project configuration
├── profiles.yml             # Connection profiles (in ~/.dbt/)
├── packages.yml             # dbt package dependencies
├── models/                  # SQL models
│   ├── staging/            # Raw data transformations
│   ├── intermediate/       # Business logic
│   └── marts/              # Final analytics tables
├── tests/                   # Custom data tests
├── macros/                  # Reusable SQL snippets
├── snapshots/              # Type-2 slowly changing dimensions
├── analyses/               # Ad-hoc queries
└── seeds/                  # Static CSV data

```

## Common dbt Commands

```bash
# Development
dbt debug                           # Test connection
dbt deps                           # Install dependencies
dbt compile                        # Compile SQL without running
dbt run                            # Run all models
dbt run --select model_name        # Run specific model
dbt run --select +model_name       # Run model and upstream dependencies
dbt run --select model_name+       # Run model and downstream dependencies
dbt run --select tag:daily         # Run models with tag
dbt run --full-refresh             # Rebuild incremental models from scratch

# Testing
dbt test                           # Run all tests
dbt test --select model_name       # Test specific model
dbt test --select test_type:generic # Run only generic tests

# Documentation
dbt docs generate                  # Generate documentation
dbt docs serve                     # Serve docs locally

# Cleaning
dbt clean                          # Remove compiled files
dbt build                          # Run, test, and snapshot in order
```

## Model Best Practices

### Staging Models (models/staging/)
```sql
-- stg_orders.sql
-- Purpose: Light transformation of raw data
-- Naming: stg_<source>__<entity>.sql

with source as (
    select * from {{ source('raw', 'orders') }}
),

renamed as (
    select
        order_id,
        customer_id,
        order_date,
        status,
        _loaded_at as loaded_at
    from source
)

select * from renamed
```

### Intermediate Models (models/intermediate/)
```sql
-- int_orders_with_customers.sql
-- Purpose: Business logic and joins
-- Naming: int_<entity>_<verb>.sql

with orders as (
    select * from {{ ref('stg_orders') }}
),

customers as (
    select * from {{ ref('stg_customers') }}
),

joined as (
    select
        orders.order_id,
        orders.order_date,
        customers.customer_name,
        customers.customer_email
    from orders
    inner join customers
        on orders.customer_id = customers.customer_id
)

select * from joined
```

### Marts Models (models/marts/)
```sql
-- fct_orders.sql
-- Purpose: Final analytics-ready tables
-- Naming: fct_<process>.sql or dim_<entity>.sql

{{
    config(
        materialized='incremental',
        unique_key='order_id',
        tags=['daily']
    )
}}

with orders as (
    select * from {{ ref('int_orders_with_customers') }}
)

select
    order_id,
    customer_name,
    order_date,
    {{ get_order_status('status') }} as order_status,
    current_timestamp() as dbt_updated_at
from orders

{% if is_incremental() %}
    where order_date > (select max(order_date) from {{ this }})
{% endif %}
```

## Materialization Strategies

### View (default)
```yaml
# In model SQL or schema.yml
{{ config(materialized='view') }}
```
- Fast to build
- Always fresh data
- Slower queries

### Table
```yaml
{{ config(materialized='table') }}
```
- Slower to build
- Faster queries
- Rebuilt on each run

### Incremental
```yaml
{{
    config(
        materialized='incremental',
        unique_key='id',
        on_schema_change='fail'
    )
}}

select * from source
{% if is_incremental() %}
    where updated_at > (select max(updated_at) from {{ this }})
{% endif %}
```
- Only processes new/changed rows
- Requires unique_key
- Fast for large datasets

### Ephemeral
```yaml
{{ config(materialized='ephemeral') }}
```
- CTEs only, not materialized
- Used for reusable logic

## Testing

### Generic Tests (in schema.yml)
```yaml
version: 2

models:
  - name: fct_orders
    description: "Order facts table"
    columns:
      - name: order_id
        description: "Primary key"
        tests:
          - unique
          - not_null

      - name: customer_id
        description: "Foreign key to customers"
        tests:
          - not_null
          - relationships:
              to: ref('dim_customers')
              field: customer_id

      - name: order_status
        description: "Order status"
        tests:
          - accepted_values:
              values: ['pending', 'shipped', 'delivered', 'cancelled']

      - name: order_total
        description: "Total order amount"
        tests:
          - not_null
          - dbt_utils.expression_is_true:
              expression: ">= 0"
```

### Custom Tests (in tests/)
```sql
-- tests/assert_positive_revenue.sql
-- Returns rows that fail the test

select
    order_id,
    revenue
from {{ ref('fct_orders') }}
where revenue < 0
```

## Macros

### Simple Macro
```sql
-- macros/cents_to_dollars.sql
{% macro cents_to_dollars(column_name) %}
    ({{ column_name }} / 100.0)::decimal(10, 2)
{% endmacro %}

-- Usage in model:
select {{ cents_to_dollars('amount_cents') }} as amount_dollars
```

### Macro with Jinja Logic
```sql
-- macros/generate_schema_name.sql
{% macro generate_schema_name(custom_schema_name, node) -%}
    {%- set default_schema = target.schema -%}
    {%- if custom_schema_name is none -%}
        {{ default_schema }}
    {%- else -%}
        {{ default_schema }}_{{ custom_schema_name | trim }}
    {%- endif -%}
{%- endmacro %}
```

## Sources

```yaml
# models/staging/sources.yml
version: 2

sources:
  - name: raw
    database: raw_database
    schema: public
    tables:
      - name: orders
        description: "Raw orders from production DB"
        freshness:
          warn_after: {count: 12, period: hour}
          error_after: {count: 24, period: hour}
        loaded_at_field: _loaded_at
        columns:
          - name: order_id
            tests:
              - unique
              - not_null

      - name: customers
        description: "Raw customer data"
```

## Configuration

### dbt_project.yml
```yaml
name: 'my_project'
version: '1.0.0'
config-version: 2

profile: 'my_profile'

model-paths: ["models"]
analysis-paths: ["analyses"]
test-paths: ["tests"]
seed-paths: ["seeds"]
macro-paths: ["macros"]
snapshot-paths: ["snapshots"]

target-path: "target"
clean-targets:
  - "target"
  - "dbt_packages"

models:
  my_project:
    staging:
      +materialized: view
      +tags: ['staging']

    intermediate:
      +materialized: ephemeral
      +tags: ['intermediate']

    marts:
      +materialized: table
      +tags: ['marts']
```

### profiles.yml (~/.dbt/profiles.yml)
```yaml
my_profile:
  target: dev
  outputs:
    dev:
      type: postgres
      host: localhost
      port: 5432
      user: my_user
      password: my_password
      dbname: dev_database
      schema: dbt_dev
      threads: 4

    prod:
      type: postgres
      host: prod.example.com
      port: 5432
      user: prod_user
      password: "{{ env_var('DBT_PASSWORD') }}"
      dbname: prod_database
      schema: analytics
      threads: 8
```

## Snapshots (SCD Type 2)

```sql
-- snapshots/customers_snapshot.sql
{% snapshot customers_snapshot %}

{{
    config(
      target_schema='snapshots',
      unique_key='customer_id',
      strategy='timestamp',
      updated_at='updated_at'
    )
}}

select * from {{ source('raw', 'customers') }}

{% endsnapshot %}
```

## Debugging Tips

1. **Compiled SQL**: Check `target/compiled/` to see actual SQL
2. **Run logs**: Check `target/run_results.json` for execution details
3. **Use --debug flag**: `dbt run --debug` for verbose output
4. **Profile specific model**: Use `--select` to isolate issues
5. **Check refs**: Ensure `{{ ref() }}` references are correct

## Common Patterns

### Date Spine
```sql
-- Generate date series for time-based analysis
{{ dbt_utils.date_spine(
    datepart="day",
    start_date="cast('2020-01-01' as date)",
    end_date="cast('2024-12-31' as date)"
) }}
```

### Surrogate Keys
```sql
-- Generate unique keys from multiple columns
{{ dbt_utils.generate_surrogate_key(['order_id', 'line_item_id']) }}
```

## When Helping Users

1. **Always check project structure**: Read `dbt_project.yml` first
2. **Follow naming conventions**: staging (stg_), intermediate (int_), facts (fct_), dimensions (dim_)
3. **Use refs not sources in models**: `{{ ref('model') }}` for dbt models
4. **Add tests**: Every model should have basic tests
5. **Document models**: Add descriptions in schema.yml
6. **Choose right materialization**: Consider data volume and freshness needs
