---
name: sql
description: Use when writing SQL queries, optimizing SQL, debugging SQL errors, working with SQL databases (PostgreSQL, MySQL, SQLite, etc.), or discussing SQL best practices and performance tuning.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# SQL

Expert guidance for writing clean, performant SQL queries.

## Query Structure Best Practices

### Basic SELECT Structure
```sql
SELECT
    -- Explicit column names (avoid SELECT *)
    user_id,
    username,
    email,
    created_at
FROM users
WHERE
    -- Filter conditions
    active = true
    AND created_at >= '2024-01-01'
ORDER BY created_at DESC
LIMIT 100;
```

### Use CTEs for Readability
```sql
-- Common Table Expressions (CTEs) for complex queries
WITH active_users AS (
    SELECT
        user_id,
        username,
        email
    FROM users
    WHERE active = true
),

recent_orders AS (
    SELECT
        order_id,
        user_id,
        order_date,
        total_amount
    FROM orders
    WHERE order_date >= CURRENT_DATE - INTERVAL '30 days'
)

SELECT
    u.username,
    u.email,
    COUNT(o.order_id) AS order_count,
    SUM(o.total_amount) AS total_spent
FROM active_users u
LEFT JOIN recent_orders o
    ON u.user_id = o.user_id
GROUP BY u.username, u.email
HAVING COUNT(o.order_id) > 0
ORDER BY total_spent DESC;
```

## JOIN Patterns

### INNER JOIN
```sql
-- Only matching rows from both tables
SELECT
    o.order_id,
    o.order_date,
    c.customer_name
FROM orders o
INNER JOIN customers c
    ON o.customer_id = c.customer_id;
```

### LEFT JOIN
```sql
-- All rows from left table, matching rows from right
SELECT
    c.customer_name,
    COALESCE(o.order_count, 0) AS order_count
FROM customers c
LEFT JOIN (
    SELECT
        customer_id,
        COUNT(*) AS order_count
    FROM orders
    GROUP BY customer_id
) o ON c.customer_id = o.customer_id;
```

### Multiple JOINs
```sql
SELECT
    o.order_id,
    c.customer_name,
    p.product_name,
    oi.quantity,
    oi.price
FROM orders o
INNER JOIN customers c
    ON o.customer_id = c.customer_id
INNER JOIN order_items oi
    ON o.order_id = oi.order_id
INNER JOIN products p
    ON oi.product_id = p.product_id
WHERE o.order_date >= '2024-01-01';
```

### Self JOIN
```sql
-- Find employees and their managers
SELECT
    e.employee_name AS employee,
    m.employee_name AS manager
FROM employees e
LEFT JOIN employees m
    ON e.manager_id = m.employee_id;
```

## Window Functions

### ROW_NUMBER
```sql
-- Rank rows within partitions
SELECT
    customer_id,
    order_date,
    total_amount,
    ROW_NUMBER() OVER (
        PARTITION BY customer_id
        ORDER BY order_date DESC
    ) AS order_rank
FROM orders;
```

### RANK and DENSE_RANK
```sql
SELECT
    product_name,
    sales_amount,
    RANK() OVER (ORDER BY sales_amount DESC) AS rank,
    DENSE_RANK() OVER (ORDER BY sales_amount DESC) AS dense_rank
FROM product_sales;
```

### LAG and LEAD
```sql
-- Compare with previous/next rows
SELECT
    date,
    revenue,
    LAG(revenue, 1) OVER (ORDER BY date) AS prev_day_revenue,
    LEAD(revenue, 1) OVER (ORDER BY date) AS next_day_revenue,
    revenue - LAG(revenue, 1) OVER (ORDER BY date) AS daily_change
FROM daily_revenue
ORDER BY date;
```

### Running Totals
```sql
SELECT
    order_date,
    daily_revenue,
    SUM(daily_revenue) OVER (
        ORDER BY order_date
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS running_total
FROM (
    SELECT
        order_date,
        SUM(total_amount) AS daily_revenue
    FROM orders
    GROUP BY order_date
) daily_summary;
```

### Moving Average
```sql
SELECT
    date,
    value,
    AVG(value) OVER (
        ORDER BY date
        ROWS BETWEEN 6 PRECEDING AND CURRENT ROW
    ) AS moving_avg_7_day
FROM metrics;
```

## Aggregations

### GROUP BY with Multiple Columns
```sql
SELECT
    DATE_TRUNC('month', order_date) AS month,
    product_category,
    COUNT(*) AS order_count,
    SUM(total_amount) AS total_revenue,
    AVG(total_amount) AS avg_order_value,
    MIN(total_amount) AS min_order,
    MAX(total_amount) AS max_order
FROM orders
WHERE order_date >= '2024-01-01'
GROUP BY
    DATE_TRUNC('month', order_date),
    product_category
HAVING SUM(total_amount) > 10000
ORDER BY month DESC, total_revenue DESC;
```

### ROLLUP and CUBE
```sql
-- ROLLUP: Hierarchical subtotals
SELECT
    region,
    city,
    SUM(sales) AS total_sales
FROM sales_data
GROUP BY ROLLUP(region, city);

-- CUBE: All possible combinations
SELECT
    product_category,
    region,
    SUM(sales) AS total_sales
FROM sales_data
GROUP BY CUBE(product_category, region);
```

### GROUPING SETS
```sql
-- Custom grouping combinations
SELECT
    year,
    quarter,
    product,
    SUM(revenue) AS total_revenue
FROM sales
GROUP BY GROUPING SETS (
    (year, quarter, product),
    (year, quarter),
    (year),
    ()
);
```

## Conditional Logic

### CASE Statements
```sql
SELECT
    order_id,
    total_amount,
    CASE
        WHEN total_amount < 50 THEN 'Small'
        WHEN total_amount < 200 THEN 'Medium'
        WHEN total_amount < 1000 THEN 'Large'
        ELSE 'Extra Large'
    END AS order_size,
    CASE
        WHEN status = 'completed' THEN 1
        ELSE 0
    END AS is_completed
FROM orders;
```

### COALESCE and NULLIF
```sql
SELECT
    customer_id,
    COALESCE(phone, email, 'No contact') AS contact_info,
    NULLIF(discount, 0) AS discount_if_any
FROM customers;
```

## Subqueries

### IN Subquery
```sql
SELECT *
FROM products
WHERE product_id IN (
    SELECT DISTINCT product_id
    FROM order_items
    WHERE order_date >= '2024-01-01'
);
```

### EXISTS
```sql
-- More efficient than IN for large datasets
SELECT c.customer_name
FROM customers c
WHERE EXISTS (
    SELECT 1
    FROM orders o
    WHERE o.customer_id = c.customer_id
    AND o.order_date >= '2024-01-01'
);
```

### Scalar Subquery
```sql
SELECT
    customer_id,
    order_count,
    order_count::float / (
        SELECT AVG(order_count)
        FROM customer_order_counts
    ) AS relative_to_avg
FROM customer_order_counts;
```

## Date/Time Functions

### PostgreSQL
```sql
-- Current date/time
SELECT
    CURRENT_DATE,
    CURRENT_TIMESTAMP,
    NOW();

-- Date arithmetic
SELECT
    order_date,
    order_date + INTERVAL '7 days' AS due_date,
    order_date - INTERVAL '1 month' AS previous_month;

-- Date truncation
SELECT
    DATE_TRUNC('month', order_date) AS month,
    DATE_TRUNC('week', order_date) AS week;

-- Date parts
SELECT
    EXTRACT(YEAR FROM order_date) AS year,
    EXTRACT(MONTH FROM order_date) AS month,
    EXTRACT(DAY FROM order_date) AS day,
    EXTRACT(DOW FROM order_date) AS day_of_week;

-- Age calculation
SELECT
    AGE(CURRENT_DATE, birth_date) AS age;
```

### MySQL
```sql
-- Current date/time
SELECT CURDATE(), NOW();

-- Date arithmetic
SELECT
    DATE_ADD(order_date, INTERVAL 7 DAY) AS due_date,
    DATE_SUB(order_date, INTERVAL 1 MONTH) AS previous_month;

-- Date formatting
SELECT DATE_FORMAT(order_date, '%Y-%m') AS year_month;

-- Date parts
SELECT
    YEAR(order_date) AS year,
    MONTH(order_date) AS month,
    DAY(order_date) AS day,
    DAYOFWEEK(order_date) AS day_of_week;
```

## String Functions

```sql
SELECT
    -- Concatenation
    CONCAT(first_name, ' ', last_name) AS full_name,
    first_name || ' ' || last_name AS full_name_alt,  -- PostgreSQL

    -- Case conversion
    UPPER(email) AS email_upper,
    LOWER(email) AS email_lower,
    INITCAP(name) AS name_capitalized,  -- PostgreSQL

    -- Trimming
    TRIM(name) AS trimmed,
    LTRIM(name) AS left_trimmed,
    RTRIM(name) AS right_trimmed,

    -- Substring
    SUBSTRING(phone FROM 1 FOR 3) AS area_code,  -- PostgreSQL
    SUBSTR(phone, 1, 3) AS area_code_alt,  -- MySQL/SQLite

    -- Pattern matching
    email LIKE '%@gmail.com' AS is_gmail,
    email ~ '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$' AS valid_email,  -- PostgreSQL regex

    -- Replace
    REPLACE(phone, '-', '') AS phone_no_dashes,

    -- Length
    LENGTH(description) AS description_length,
    CHAR_LENGTH(description) AS char_count  -- PostgreSQL

FROM users;
```

## Performance Optimization

### Use Indexes
```sql
-- Create indexes on frequently queried columns
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_orders_customer_date ON orders(customer_id, order_date);

-- Unique index
CREATE UNIQUE INDEX idx_users_username ON users(username);

-- Partial index (PostgreSQL)
CREATE INDEX idx_active_users ON users(email) WHERE active = true;

-- Full-text search index (PostgreSQL)
CREATE INDEX idx_products_search ON products USING gin(to_tsvector('english', name || ' ' || description));
```

### Analyze Query Plans
```sql
-- PostgreSQL
EXPLAIN ANALYZE
SELECT * FROM orders
WHERE customer_id = 123;

-- Look for:
-- - Seq Scan (bad on large tables)
-- - Index Scan (good)
-- - Nested Loop vs Hash Join
-- - High execution time
```

### Avoid SELECT *
```sql
-- Bad: Retrieves all columns
SELECT * FROM large_table;

-- Good: Explicit columns
SELECT id, name, email FROM large_table;
```

### Use JOINs Instead of Subqueries (Sometimes)
```sql
-- Subquery (can be slow)
SELECT *
FROM customers
WHERE customer_id IN (
    SELECT customer_id FROM orders WHERE total > 1000
);

-- JOIN (often faster)
SELECT DISTINCT c.*
FROM customers c
INNER JOIN orders o ON c.customer_id = o.customer_id
WHERE o.total > 1000;
```

### Limit Result Sets
```sql
-- Always use LIMIT for exploratory queries
SELECT * FROM large_table LIMIT 100;

-- Pagination
SELECT * FROM products
ORDER BY product_id
LIMIT 20 OFFSET 100;  -- Page 6 (20 per page)
```

## Common Table Expressions (CTEs) vs Subqueries

### Recursive CTE
```sql
-- Generate hierarchical data (org chart, categories, etc.)
WITH RECURSIVE employee_hierarchy AS (
    -- Base case: top-level employees
    SELECT
        employee_id,
        employee_name,
        manager_id,
        1 AS level
    FROM employees
    WHERE manager_id IS NULL

    UNION ALL

    -- Recursive case: employees with managers
    SELECT
        e.employee_id,
        e.employee_name,
        e.manager_id,
        eh.level + 1
    FROM employees e
    INNER JOIN employee_hierarchy eh
        ON e.manager_id = eh.employee_id
)

SELECT * FROM employee_hierarchy
ORDER BY level, employee_name;
```

## Transactions

```sql
-- Begin transaction
BEGIN;

-- Multiple operations
UPDATE accounts SET balance = balance - 100 WHERE account_id = 1;
UPDATE accounts SET balance = balance + 100 WHERE account_id = 2;
INSERT INTO transactions (from_account, to_account, amount) VALUES (1, 2, 100);

-- Commit or rollback
COMMIT;
-- or
ROLLBACK;
```

## Pivoting Data

### CASE-based Pivot
```sql
SELECT
    product_id,
    SUM(CASE WHEN month = 'Jan' THEN sales ELSE 0 END) AS jan_sales,
    SUM(CASE WHEN month = 'Feb' THEN sales ELSE 0 END) AS feb_sales,
    SUM(CASE WHEN month = 'Mar' THEN sales ELSE 0 END) AS mar_sales
FROM monthly_sales
GROUP BY product_id;
```

### CROSSTAB (PostgreSQL)
```sql
-- Enable tablefunc extension first
CREATE EXTENSION IF NOT EXISTS tablefunc;

SELECT *
FROM crosstab(
    'SELECT product_id, month, sales FROM monthly_sales ORDER BY 1,2',
    'SELECT DISTINCT month FROM monthly_sales ORDER BY 1'
) AS ct(product_id int, jan numeric, feb numeric, mar numeric);
```

## Data Quality Checks

```sql
-- Find duplicates
SELECT
    email,
    COUNT(*) AS count
FROM users
GROUP BY email
HAVING COUNT(*) > 1;

-- Find nulls
SELECT COUNT(*) AS null_email_count
FROM users
WHERE email IS NULL;

-- Find outliers
WITH stats AS (
    SELECT
        AVG(order_amount) AS mean,
        STDDEV(order_amount) AS stddev
    FROM orders
)
SELECT o.*
FROM orders o, stats
WHERE o.order_amount > stats.mean + (3 * stats.stddev)
   OR o.order_amount < stats.mean - (3 * stats.stddev);

-- Check referential integrity
SELECT o.customer_id
FROM orders o
LEFT JOIN customers c ON o.customer_id = c.customer_id
WHERE c.customer_id IS NULL;
```

## Database-Specific Features

### PostgreSQL JSON
```sql
-- Query JSON columns
SELECT
    user_id,
    preferences->>'theme' AS theme,
    preferences->'notifications'->>'email' AS email_notifications
FROM users
WHERE preferences @> '{"premium": true}';

-- JSON aggregation
SELECT
    customer_id,
    jsonb_agg(
        jsonb_build_object(
            'order_id', order_id,
            'total', total_amount
        )
    ) AS orders
FROM orders
GROUP BY customer_id;
```

### PostgreSQL Arrays
```sql
-- Array operations
SELECT
    tag,
    COUNT(*) AS post_count
FROM posts,
UNNEST(tags) AS tag
GROUP BY tag;

-- Array containment
SELECT * FROM posts
WHERE tags @> ARRAY['sql', 'database'];
```

## Best Practices

1. **Explicit Column Names**: Avoid `SELECT *`
2. **Use CTEs**: For complex queries, improve readability
3. **Index Foreign Keys**: Always index columns used in JOINs
4. **Parameterize Queries**: Prevent SQL injection (use `$1`, `?`, etc.)
5. **Use Transactions**: For multi-statement operations
6. **Avoid N+1 Queries**: Use JOINs or subqueries instead of loops
7. **Use EXPLAIN**: Understand query performance
8. **Consistent Naming**: snake_case for columns, tables
9. **Comment Complex Logic**: Explain non-obvious queries
10. **Test with Production-like Data**: Different data volumes affect performance

## Common Anti-Patterns to Avoid

1. **SELECT * in production code**
2. **No indexes on JOIN/WHERE columns**
3. **Using DISTINCT to fix duplicates** (fix the root cause)
4. **Implicit column names in INSERT** (specify columns)
5. **String concatenation for SQL** (use parameters)
6. **Multiple queries in loop** (use single query with JOIN)
7. **Not using LIMIT on exploratory queries**

## When Helping Users

1. **Identify the database**: PostgreSQL, MySQL, SQLite have different syntax
2. **Check for indexes**: Ask about indexes on large tables
3. **Suggest EXPLAIN**: For slow queries
4. **Use CTEs for clarity**: Break complex queries into logical steps
5. **Format consistently**: Consistent indentation and capitalization
