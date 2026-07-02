---
name: sql-write
description: Use when writing SQL queries, optimizing query performance, debugging SQL errors, or working with PostgreSQL, MySQL, or SQLite. Covers readable query style, indexing, EXPLAIN analysis, and safe migrations on large tables.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# SQL

Write readable, index-friendly SQL and change schemas without taking production down.

## Query Style

CTEs over nested subqueries, explicit JOINs, explicit columns. Never `SELECT *` in production code — it breaks when columns are added and defeats covering indexes.

```sql
WITH recent_orders AS (
    SELECT o.customer_id, o.total_cents, o.created_at
    FROM orders AS o
    WHERE o.created_at >= now() - interval '30 days'
),
customer_totals AS (
    SELECT customer_id,
           sum(total_cents) AS total_cents,
           count(*)         AS n_orders
    FROM recent_orders
    GROUP BY customer_id
)
SELECT c.id, c.email, ct.total_cents, ct.n_orders
FROM customers AS c
JOIN customer_totals AS ct ON ct.customer_id = c.id
ORDER BY ct.total_cents DESC;
```

- Keywords uppercase or lowercase — pick one per project and stay consistent.
- Alias every table; qualify every column in multi-table queries.
- `JOIN ... ON`, never comma joins. Prefer `LEFT JOIN` + `IS NULL` check over `NOT IN` (which breaks on NULLs).

## EXPLAIN Workflow

```sql
-- Plan only (safe, does not run the query)
EXPLAIN SELECT ...;

-- Actually executes — wrap writes in a transaction and roll back
BEGIN;
EXPLAIN (ANALYZE, BUFFERS) UPDATE ...;
ROLLBACK;
```

What to look for:

| Symptom | Meaning |
|---------|---------|
| `Seq Scan` on a large table | Missing or unusable index |
| `rows=1` estimated vs `rows=500000` actual | Stale stats — run `ANALYZE tablename` |
| `Sort Method: external merge Disk` | Sort spilling — raise `work_mem` or add an index matching `ORDER BY` |
| `Nested Loop` with huge outer row count | Bad join order from misestimation |
| `Filter: ...` removing most rows after an index scan | Index doesn't cover the predicate — extend it |

MySQL: `EXPLAIN ANALYZE SELECT ...` (8.0+). SQLite: `EXPLAIN QUERY PLAN SELECT ...`.

## Index Design

| Need | Index |
|------|-------|
| Equality + range predicates | Composite: equality columns first, range column last: `(tenant_id, status, created_at)` |
| Avoid heap fetches entirely | Covering: `CREATE INDEX ... ON orders (customer_id) INCLUDE (total_cents)` |
| Query only ever hits a slice | Partial: `CREATE INDEX ... ON orders (created_at) WHERE deleted_at IS NULL` |
| Case-insensitive lookup | Expression: `CREATE INDEX ... ON users (lower(email))` — query must use `lower(email) = ...` |
| JSONB / array containment | GIN: `CREATE INDEX ... ON events USING gin (payload jsonb_path_ops)` |

Column order matters: an index on `(a, b)` serves `WHERE a = ?` and `WHERE a = ? AND b = ?`, but not `WHERE b = ?` alone. Don't index low-cardinality columns by themselves.

## Common Pitfalls

- **Implicit casts kill indexes.** `WHERE user_id = '42'` on a `bigint` column, or `WHERE date(created_at) = ...`, forces a scan. Compare same-type values; rewrite as a range: `created_at >= '2026-07-01' AND created_at < '2026-07-02'`.
- **N+1 queries.** One query per row in a loop. Fix with a `JOIN`, `WHERE id = ANY(:ids)`, or your ORM's eager loading.
- **OFFSET pagination.** `OFFSET 100000` reads and discards 100k rows. Use keyset pagination:

```sql
SELECT id, created_at, title
FROM posts
WHERE (created_at, id) < (:last_created_at, :last_id)
ORDER BY created_at DESC, id DESC
LIMIT 20;
```

- **Leading-wildcard LIKE.** `LIKE '%term%'` can't use a btree index — use a trigram index (`pg_trgm`) or full-text search.

## Window Functions

Deduplicate / latest-per-group without self-joins:

```sql
WITH ranked AS (
    SELECT o.*,
           row_number() OVER (PARTITION BY customer_id ORDER BY created_at DESC) AS rn
    FROM orders AS o
)
SELECT id, customer_id, total_cents,
       sum(total_cents) OVER (PARTITION BY customer_id)                          AS customer_total,
       lag(total_cents) OVER (PARTITION BY customer_id ORDER BY created_at)      AS prev_total
FROM ranked
WHERE rn = 1;
```

Prefer `DISTINCT ON (customer_id) ... ORDER BY customer_id, created_at DESC` in Postgres when you only need the latest row.

## Safe Migrations on Large Tables

```sql
SET lock_timeout = '5s';        -- fail fast instead of queueing behind long transactions
SET statement_timeout = '30s';

-- Indexes: never block writes
CREATE INDEX CONCURRENTLY idx_orders_customer ON orders (customer_id);

-- NOT NULL on a big table: validate without a full-table lock
ALTER TABLE orders ADD CONSTRAINT orders_status_nn CHECK (status IS NOT NULL) NOT VALID;
ALTER TABLE orders VALIDATE CONSTRAINT orders_status_nn;  -- takes only a light lock
ALTER TABLE orders ALTER COLUMN status SET NOT NULL;      -- PG12+ uses the validated check

-- Backfills: batch, never one giant UPDATE
UPDATE orders SET status = 'legacy'
WHERE id IN (SELECT id FROM orders WHERE status IS NULL LIMIT 10000);
-- repeat until 0 rows; commit between batches
```

`ADD COLUMN ... DEFAULT ...` is metadata-only in PG11+; adding a column with a **volatile** default (e.g. `now()`) still rewrites the table — add nullable, backfill, then set the default.

## Rules

- No `SELECT *` outside ad-hoc exploration. Ever.
- CTEs for structure; but check the plan — a CTE referenced twice may be materialized.
- Always `EXPLAIN (ANALYZE, BUFFERS)` before and after adding an index; drop indexes that don't change the plan.
- Composite index order: equality first, range last. One good composite beats three single-column indexes.
- Keyset pagination, not `OFFSET`, for anything user-facing.
- Store money as integer cents or `numeric`, never `float`. Store timestamps as `timestamptz`.
- Wrap destructive ad-hoc statements in `BEGIN; ... ROLLBACK;` first and check the row count.
- Every migration on a hot table: `lock_timeout` set, `CONCURRENTLY` for indexes, batched backfills.
- Parameterize all values from application code — string interpolation into SQL is an injection bug, not a style issue.
