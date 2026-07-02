---
name: matplotlib-plot
description: Use when creating plots and charts with matplotlib or visualizing data for reports. Produces professionally styled, clearly labeled figures using the object-oriented API.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Matplotlib

Publication-quality static figures: object-oriented API, labeled everything, formatted numbers.

## Object-Oriented API — Always

Use `fig, ax = plt.subplots()` and call methods on `ax`/`fig`. Never the `plt.plot()`/`plt.title()` state machine — it breaks with multiple axes and hides which figure you're mutating.

```python
import matplotlib.pyplot as plt
from matplotlib.ticker import StrMethodFormatter

fig, ax = plt.subplots(figsize=(10, 6), constrained_layout=True)
ax.plot(months, revenue, marker="o", linewidth=2, label="Revenue")
ax.plot(months, target, linestyle="--", color="gray", label="Target")

ax.set_title("Monthly Revenue vs Target, 2026", fontsize=14, fontweight="bold")
ax.set_xlabel("Month")
ax.set_ylabel("Revenue (USD)")
ax.yaxis.set_major_formatter(StrMethodFormatter("${x:,.0f}"))
ax.legend(frameon=False)
ax.grid(axis="y", alpha=0.3)
ax.spines[["top", "right"]].set_visible(False)

fig.savefig("revenue.png", dpi=300, bbox_inches="tight")
plt.close(fig)  # free memory in scripts/loops
```

`constrained_layout=True` at creation time replaces `tight_layout()` calls and handles multi-axes figures correctly.

## Number Formatting

Raw `1200000.0` tick labels are unacceptable. Format every numeric axis:

```python
from matplotlib.ticker import FuncFormatter, PercentFormatter, StrMethodFormatter

ax.yaxis.set_major_formatter(StrMethodFormatter("{x:,.0f}"))       # 1,200,000
ax.yaxis.set_major_formatter(StrMethodFormatter("${x:,.0f}"))      # $1,200,000
ax.yaxis.set_major_formatter(PercentFormatter(xmax=1.0, decimals=1))  # 0.123 -> 12.3%
ax.yaxis.set_major_formatter(FuncFormatter(lambda x, _: f"${x / 1e6:.1f}M"))  # $1.2M
```

| Data | Formatter |
|------|-----------|
| Counts | `StrMethodFormatter("{x:,.0f}")` |
| Currency | `StrMethodFormatter("${x:,.0f}")` |
| Proportions (0–1) | `PercentFormatter(xmax=1.0, decimals=1)` |
| Millions/billions | `FuncFormatter` with `/1e6` + `"M"` suffix |

Match precision in annotations: `ax.annotate(f"${value:,.0f}", ...)`.

## Date Axes

Never let matplotlib auto-pick date ticks for report figures:

```python
import matplotlib.dates as mdates

ax.xaxis.set_major_locator(mdates.MonthLocator(interval=3))
ax.xaxis.set_major_formatter(mdates.DateFormatter("%b %Y"))
ax.xaxis.set_minor_locator(mdates.MonthLocator())
fig.autofmt_xdate(rotation=30, ha="right")
```

For mixed-scale automatic ticking: `mdates.ConciseDateFormatter(ax.xaxis.get_major_locator())`.

## Multi-Panel Figures

```python
fig, axes = plt.subplots(nrows=2, ncols=2, figsize=(12, 8), sharex=True, constrained_layout=True)
for ax, (region, sub) in zip(axes.flat, df.group_by("region", maintain_order=True)):
    ax.plot(sub["month"], sub["revenue"])
    ax.set_title(str(region[0]))
    ax.yaxis.set_major_formatter(StrMethodFormatter("${x:,.0f}"))
fig.suptitle("Revenue by Region, 2026", fontsize=15, fontweight="bold")
fig.supylabel("Revenue (USD)")
```

`sharex`/`sharey` keep panels comparable — use them whenever panels show the same quantity.

## Professional Styling

```python
plt.rcParams.update({
    "figure.dpi": 120,
    "font.size": 11,
    "axes.titlesize": 14,
    "axes.labelsize": 12,
    "axes.spines.top": False,
    "axes.spines.right": False,
    "axes.grid": True,
    "grid.alpha": 0.3,
    "legend.frameon": False,
})
```

- Set rcParams once at the top of the script, not per figure.
- Horizontal bar charts for long category names; sort bars by value, not alphabetically.
- Color: one accent color for the message, gray for context series. Colorblind-safe cycle: `plt.style.use("tableau-colorblind10")`.
- Direct-label lines with `ax.annotate` when a legend would force eye travel.

## Saving

```python
fig.savefig("figure.png", dpi=300, bbox_inches="tight")   # raster: docs, slides
fig.savefig("figure.svg", bbox_inches="tight")             # vector: web, print
fig.savefig("figure.pdf", bbox_inches="tight")             # vector: LaTeX reports
```

`dpi=300` minimum for anything printed or embedded in a report; the default 100 looks fuzzy.

## Rules

- `fig, ax = plt.subplots(...)` always; zero calls to the pyplot state machine (`plt.plot`, `plt.title`, `plt.xlabel`).
- Every figure: descriptive title (what + when), both axes labeled with units, formatted tick labels. No exceptions.
- `constrained_layout=True` at figure creation; never bare `plt.show()` cropping labels.
- Thousands separators, currency symbols, and percent signs via formatters — never pre-format data into strings.
- Date axes get explicit `mdates` locator + formatter.
- Save with `dpi=300, bbox_inches="tight"`; `plt.close(fig)` in loops and long-running scripts.
- Legends only when there are multiple series; `frameon=False`; consider direct labels instead.
- Grid on the value axis only, `alpha≈0.3`; drop top/right spines.
- Bar charts start at zero. Line charts may not — but say so in the title or a note.
- If the deliverable is interactive or web-based, use Altair instead — matplotlib is for static report/print output.
