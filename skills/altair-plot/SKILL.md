---
name: altair-plot
description: Use when creating interactive, web-ready charts, dashboards with linked charts, or Grammar-of-Graphics visualization work. Altair charts are declarative, composable, and interactive by default.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Altair

Declarative, interactive visualizations: describe what to show, not how to draw it.

## Baseline Chart

Every chart ships with a real title, labeled axes, formatted numbers, and tooltips. Interactivity is the default, not an add-on.

```python
import altair as alt
import polars as pl

df = pl.read_parquet("sales.parquet")  # Altair accepts Polars frames directly

chart = (
    alt.Chart(df, title="Monthly Revenue by Region")
    .mark_line(point=True)
    .encode(
        x=alt.X("month:T", title="Month", axis=alt.Axis(format="%b %Y")),
        y=alt.Y("revenue:Q", title="Revenue (USD)", axis=alt.Axis(format="$,.0f")),
        color=alt.Color("region:N", title="Region"),
        tooltip=[
            alt.Tooltip("month:T", title="Month", format="%B %Y"),
            alt.Tooltip("region:N", title="Region"),
            alt.Tooltip("revenue:Q", title="Revenue", format="$,.0f"),
        ],
    )
    .properties(width=640, height=360)
    .interactive()   # pan + zoom
)
chart.save("revenue.html")
```

Type codes matter: `:Q` quantitative, `:T` temporal, `:N` nominal, `:O` ordinal. Getting these wrong is the #1 cause of broken axes and alphabetical "date" ordering.

## Number and Date Formatting

Raw `1234567.891` on an axis is unacceptable. Use d3-format strings in `axis=`/`format=`:

| Value | Format | Renders as |
|-------|--------|------------|
| Count | `,.0f` | 1,234,568 |
| Currency | `$,.0f` | $1,234,568 |
| Percentage (0–1 data) | `.1%` | 12.3% |
| Compact/SI | `.2s` | 1.2M |
| Two decimals | `,.2f` | 1,234,567.89 |
| Month + year (temporal) | `%b %Y` | Jul 2026 |

Apply the same format in both the axis and the tooltip — mismatched precision between them looks broken.

## Tidy Data

Altair wants long/tidy data: one row per observation, one column per variable. Unpivot wide frames before charting.

```python
long = wide.unpivot(index="month", variable_name="region", value_name="revenue")
```

If the source frame is large, aggregate in Polars first — don't ship 500k rows into a browser. For >5000 rows that must pass through, enable `alt.data_transformers.enable("vegafusion")`.

## Composition

Operators, in precedence order: `+` layer, `|` hconcat, `&` vconcat.

```python
base = alt.Chart(df).encode(x=alt.X("month:T", title="Month"))

line = base.mark_line().encode(y=alt.Y("revenue:Q", title="Revenue (USD)", axis=alt.Axis(format="$,.0f")))
target = base.mark_rule(strokeDash=[4, 4], color="gray").encode(y=alt.datum(1_000_000))

dashboard = (line + target) & base.mark_bar().encode(
    y=alt.Y("n_orders:Q", title="Orders", axis=alt.Axis(format=",.0f"))
)
```

Small multiples via facet — never a loop that builds N separate charts:

```python
chart.facet(facet=alt.Facet("region:N", title=None), columns=3)
```

## Selections and Cross-Filtering

Linked charts are Altair's superpower. One param, shared across charts:

```python
region_sel = alt.selection_point(fields=["region"], bind="legend")
brush = alt.selection_interval(encodings=["x"])

overview = (
    alt.Chart(df, title="Revenue Over Time (drag to filter)")
    .mark_area()
    .encode(x="month:T", y=alt.Y("sum(revenue):Q", axis=alt.Axis(format="$,.0f")))
    .add_params(brush)
    .properties(height=120)
)

detail = (
    alt.Chart(df, title="Revenue by Region")
    .mark_bar()
    .encode(
        x=alt.X("sum(revenue):Q", title="Revenue (USD)", axis=alt.Axis(format="$,.0f")),
        y=alt.Y("region:N", sort="-x", title=None),
        opacity=alt.condition(region_sel, alt.value(1.0), alt.value(0.25)),
        tooltip=[alt.Tooltip("sum(revenue):Q", title="Revenue", format="$,.0f")],
    )
    .transform_filter(brush)
    .add_params(region_sel)
)

(overview & detail).save("dashboard.html")
```

Use `alt.param(bind=alt.binding_select(...))` for dropdown-driven filtering.

## Output

- `chart.save("chart.html")` — self-contained interactive page; the default deliverable.
- `chart.save("chart.png", scale_factor=2.0)` — static export (needs `uv add vl-convert-python`).
- In notebooks the chart renders inline; no save needed.

## Rules

- Every chart: descriptive title, `title=` on every axis (with units), formatted numbers, tooltips. No exceptions.
- Interactive by default — add `.interactive()` or explicit params unless the target is a static PNG in a PDF.
- Declare encodings; never precompute pixel positions or colors row-by-row.
- Type suffixes (`:Q/:T/:N/:O`) on every shorthand field — implicit inference will eventually betray you.
- Long/tidy data in; `unpivot` wide frames before charting.
- Aggregate large data in Polars before the chart; VegaFusion only when the browser must see many rows.
- Compose with `+`, `|`, `&`, and `facet` — one spec, not N glued images.
- Sort categorical axes deliberately (`sort="-x"` or an explicit list); alphabetical order is rarely the story.
- Bind color to meaning; use `bind="legend"` selections instead of ten separate filtered charts.
- Percentages stored as 0–1 get `.1%` formatting — don't multiply by 100 in the data.
