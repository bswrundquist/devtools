---
name: matplotlib
description: Use when creating data visualizations with matplotlib. Create professional, well-formatted plots with clear titles, properly labeled axes, formatted numbers (commas for thousands, $ for currency), and good aesthetics.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# matplotlib

Expert guidance for creating beautiful, professional data visualizations with matplotlib.

## Core Principles

1. **Clear titles** - Descriptive and informative
2. **Labeled axes** - Always label with units
3. **Formatted numbers** - Commas, currency symbols, percentages
4. **Professional styling** - Use seaborn styles or custom themes
5. **Readable fonts** - Appropriate sizes for all text
6. **Color choices** - Accessible and meaningful
7. **Proper sizing** - Figures should be large enough to read

## Basic Setup

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import seaborn as sns
import numpy as np

# Set style for better-looking plots
plt.style.use('seaborn-v0_8-darkgrid')  # or 'seaborn-v0_8-whitegrid'
sns.set_palette("husl")

# Set default figure size and DPI
plt.rcParams['figure.figsize'] = (12, 6)
plt.rcParams['figure.dpi'] = 100
plt.rcParams['savefig.dpi'] = 300  # High quality for saving

# Font sizes
plt.rcParams['font.size'] = 12
plt.rcParams['axes.titlesize'] = 16
plt.rcParams['axes.labelsize'] = 14
plt.rcParams['xtick.labelsize'] = 12
plt.rcParams['ytick.labelsize'] = 12
plt.rcParams['legend.fontsize'] = 12
```

## Number Formatting

### Thousands with Commas

```python
import matplotlib.ticker as ticker

fig, ax = plt.subplots(figsize=(12, 6))

# Sample data
x = np.arange(2020, 2025)
y = [45000, 52000, 58000, 63000, 71000]

ax.plot(x, y, marker='o', linewidth=2, markersize=8)

# Format y-axis with commas for thousands
ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: f'{int(x):,}'))

ax.set_title('Annual Revenue Growth (2020-2024)', fontsize=16, fontweight='bold', pad=20)
ax.set_xlabel('Year', fontsize=14)
ax.set_ylabel('Revenue', fontsize=14)

plt.tight_layout()
plt.show()
```

### Currency Formatting

```python
# Format as currency with dollar sign
def currency_formatter(x, p):
    if x >= 1_000_000:
        return f'${x/1_000_000:.1f}M'
    elif x >= 1_000:
        return f'${x/1_000:.0f}K'
    else:
        return f'${x:.0f}'

fig, ax = plt.subplots(figsize=(12, 6))

revenue = [1_250_000, 1_480_000, 1_720_000, 2_100_000]
quarters = ['Q1', 'Q2', 'Q3', 'Q4']

ax.bar(quarters, revenue, color='#2ecc71', edgecolor='black', linewidth=1.2)

ax.yaxis.set_major_formatter(ticker.FuncFormatter(currency_formatter))

ax.set_title('Quarterly Revenue Performance', fontsize=16, fontweight='bold', pad=20)
ax.set_xlabel('Quarter', fontsize=14)
ax.set_ylabel('Revenue (USD)', fontsize=14)

# Add value labels on bars
for i, (q, r) in enumerate(zip(quarters, revenue)):
    ax.text(i, r + 50000, currency_formatter(r, None),
            ha='center', va='bottom', fontsize=12, fontweight='bold')

plt.tight_layout()
plt.show()
```

### Percentage Formatting

```python
# Format as percentage
def percent_formatter(x, p):
    return f'{x:.0f}%'

fig, ax = plt.subplots(figsize=(12, 6))

categories = ['Product A', 'Product B', 'Product C', 'Product D']
market_share = [35, 28, 22, 15]

ax.barh(categories, market_share, color='#3498db', edgecolor='black', linewidth=1.2)

ax.xaxis.set_major_formatter(ticker.FuncFormatter(percent_formatter))

ax.set_title('Market Share by Product', fontsize=16, fontweight='bold', pad=20)
ax.set_xlabel('Market Share (%)', fontsize=14)
ax.set_ylabel('Product', fontsize=14)

# Add percentage labels
for i, (cat, share) in enumerate(zip(categories, market_share)):
    ax.text(share + 1, i, f'{share}%', va='center', fontsize=12, fontweight='bold')

plt.tight_layout()
plt.show()
```

### Scientific Notation with Proper Formatting

```python
# Format large numbers in scientific notation
fig, ax = plt.subplots(figsize=(12, 6))

years = np.arange(2015, 2025)
population = np.array([7.3e9, 7.4e9, 7.5e9, 7.6e9, 7.7e9,
                       7.8e9, 7.9e9, 8.0e9, 8.1e9, 8.2e9])

ax.plot(years, population, marker='o', linewidth=2, markersize=8, color='#e74c3c')

# Format y-axis as billions
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'{x/1e9:.1f}B'
))

ax.set_title('World Population Growth (2015-2024)',
             fontsize=16, fontweight='bold', pad=20)
ax.set_xlabel('Year', fontsize=14)
ax.set_ylabel('Population (Billions)', fontsize=14)

plt.grid(True, alpha=0.3)
plt.tight_layout()
plt.show()
```

## Complete Plot Examples

### Line Plot

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

# Create figure and axis
fig, ax = plt.subplots(figsize=(14, 7))

# Sample data
months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
          'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
revenue_2023 = [125_000, 132_000, 145_000, 138_000, 155_000, 162_000,
                178_000, 185_000, 192_000, 201_000, 215_000, 235_000]
revenue_2024 = [145_000, 158_000, 172_000, 165_000, 182_000, 195_000,
                215_000, 228_000, 242_000, 258_000, 275_000, 298_000]

# Plot lines
ax.plot(months, revenue_2023, marker='o', linewidth=2.5, markersize=8,
        label='2023', color='#3498db')
ax.plot(months, revenue_2024, marker='s', linewidth=2.5, markersize=8,
        label='2024', color='#2ecc71')

# Format y-axis with currency
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Titles and labels
ax.set_title('Monthly Revenue Comparison: 2023 vs 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Month', fontsize=14, fontweight='bold')
ax.set_ylabel('Revenue (USD)', fontsize=14, fontweight='bold')

# Legend
ax.legend(loc='upper left', fontsize=12, frameon=True, shadow=True)

# Grid
ax.grid(True, alpha=0.3, linestyle='--')

# Rotate x-axis labels if needed
plt.xticks(rotation=45, ha='right')

plt.tight_layout()
plt.show()
```

### Bar Chart

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

fig, ax = plt.subplots(figsize=(14, 7))

# Data
products = ['Product A', 'Product B', 'Product C', 'Product D', 'Product E']
sales = [245_000, 312_000, 198_000, 275_000, 423_000]

# Color gradient
colors = plt.cm.viridis(np.linspace(0.3, 0.9, len(products)))

# Create bars
bars = ax.bar(products, sales, color=colors, edgecolor='black', linewidth=1.5)

# Format y-axis
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Add value labels on top of bars
for bar in bars:
    height = bar.get_height()
    ax.text(bar.get_x() + bar.get_width()/2., height,
            f'${height/1000:.0f}K',
            ha='center', va='bottom', fontsize=12, fontweight='bold')

# Titles and labels
ax.set_title('Product Sales Performance - Q4 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Product', fontsize=14, fontweight='bold')
ax.set_ylabel('Sales (USD)', fontsize=14, fontweight='bold')

# Grid (only horizontal for bar charts)
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)  # Grid behind bars

plt.tight_layout()
plt.show()
```

### Horizontal Bar Chart (Better for Long Labels)

```python
fig, ax = plt.subplots(figsize=(12, 8))

# Data with longer names
departments = [
    'Sales & Marketing',
    'Product Development',
    'Customer Support',
    'Engineering',
    'Human Resources',
    'Finance & Accounting'
]
budget = [850_000, 1_200_000, 450_000, 1_500_000, 320_000, 280_000]

# Sort by budget (optional)
sorted_data = sorted(zip(departments, budget), key=lambda x: x[1])
departments, budget = zip(*sorted_data)

# Create horizontal bars
colors = plt.cm.RdYlGn(np.linspace(0.3, 0.9, len(departments)))
bars = ax.barh(departments, budget, color=colors, edgecolor='black', linewidth=1.5)

# Format x-axis
ax.xaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1_000_000:.1f}M'
))

# Add value labels
for i, (dept, budg) in enumerate(zip(departments, budget)):
    ax.text(budg + 30000, i, f'${budg/1_000_000:.2f}M',
            va='center', fontsize=11, fontweight='bold')

# Titles and labels
ax.set_title('Department Budget Allocation - FY 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Budget (USD)', fontsize=14, fontweight='bold')
ax.set_ylabel('Department', fontsize=14, fontweight='bold')

# Grid
ax.grid(True, axis='x', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

plt.tight_layout()
plt.show()
```

### Grouped Bar Chart

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

fig, ax = plt.subplots(figsize=(14, 7))

# Data
quarters = ['Q1', 'Q2', 'Q3', 'Q4']
revenue_east = [425_000, 478_000, 512_000, 589_000]
revenue_west = [398_000, 445_000, 502_000, 612_000]
revenue_central = [312_000, 358_000, 391_000, 445_000]

# Bar positions
x = np.arange(len(quarters))
width = 0.25

# Create bars
bars1 = ax.bar(x - width, revenue_east, width, label='East Region',
               color='#3498db', edgecolor='black', linewidth=1.2)
bars2 = ax.bar(x, revenue_west, width, label='West Region',
               color='#2ecc71', edgecolor='black', linewidth=1.2)
bars3 = ax.bar(x + width, revenue_central, width, label='Central Region',
               color='#e74c3c', edgecolor='black', linewidth=1.2)

# Format y-axis
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Titles and labels
ax.set_title('Regional Revenue by Quarter - 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Quarter', fontsize=14, fontweight='bold')
ax.set_ylabel('Revenue (USD)', fontsize=14, fontweight='bold')
ax.set_xticks(x)
ax.set_xticklabels(quarters)

# Legend
ax.legend(loc='upper left', fontsize=12, frameon=True, shadow=True)

# Grid
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

plt.tight_layout()
plt.show()
```

### Stacked Bar Chart

```python
fig, ax = plt.subplots(figsize=(14, 7))

# Data
months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun']
online_sales = [125_000, 138_000, 145_000, 152_000, 168_000, 178_000]
retail_sales = [215_000, 198_000, 205_000, 212_000, 225_000, 238_000]
wholesale_sales = [98_000, 105_000, 112_000, 118_000, 125_000, 132_000]

# Create stacked bars
ax.bar(months, online_sales, label='Online', color='#3498db',
       edgecolor='black', linewidth=1.2)
ax.bar(months, retail_sales, bottom=online_sales, label='Retail',
       color='#2ecc71', edgecolor='black', linewidth=1.2)
ax.bar(months, wholesale_sales,
       bottom=np.array(online_sales) + np.array(retail_sales),
       label='Wholesale', color='#f39c12', edgecolor='black', linewidth=1.2)

# Format y-axis
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Titles and labels
ax.set_title('Sales by Channel - First Half 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Month', fontsize=14, fontweight='bold')
ax.set_ylabel('Sales (USD)', fontsize=14, fontweight='bold')

# Legend
ax.legend(loc='upper left', fontsize=12, frameon=True, shadow=True)

# Grid
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

plt.tight_layout()
plt.show()
```

### Scatter Plot

```python
fig, ax = plt.subplots(figsize=(12, 8))

# Data
np.random.seed(42)
marketing_spend = np.random.uniform(10_000, 100_000, 50)
revenue = marketing_spend * np.random.uniform(2.5, 4.5, 50) + np.random.normal(0, 20_000, 50)

# Create scatter plot
scatter = ax.scatter(marketing_spend, revenue,
                     s=100, alpha=0.6, c=revenue,
                     cmap='viridis', edgecolors='black', linewidth=1)

# Format axes
ax.xaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Add trend line
z = np.polyfit(marketing_spend, revenue, 1)
p = np.poly1d(z)
ax.plot(marketing_spend, p(marketing_spend),
        "r--", linewidth=2, label=f'Trend: y = {z[0]:.2f}x + {z[1]:,.0f}')

# Titles and labels
ax.set_title('Marketing Spend vs Revenue Correlation',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Marketing Spend (USD)', fontsize=14, fontweight='bold')
ax.set_ylabel('Revenue (USD)', fontsize=14, fontweight='bold')

# Colorbar
cbar = plt.colorbar(scatter, ax=ax)
cbar.set_label('Revenue Level', fontsize=12, fontweight='bold')
cbar.ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Legend and grid
ax.legend(fontsize=12, frameon=True, shadow=True)
ax.grid(True, alpha=0.3, linestyle='--')

plt.tight_layout()
plt.show()
```

### Pie Chart

```python
fig, ax = plt.subplots(figsize=(12, 8))

# Data
categories = ['Product Sales', 'Services', 'Subscriptions', 'Licensing', 'Other']
values = [2_450_000, 1_230_000, 890_000, 450_000, 180_000]

# Colors
colors = ['#3498db', '#2ecc71', '#f39c12', '#e74c3c', '#9b59b6']

# Create pie chart
wedges, texts, autotexts = ax.pie(
    values,
    labels=categories,
    autopct=lambda pct: f'{pct:.1f}%\n(${values[int(pct/100*len(values))]/1_000_000:.2f}M)',
    startangle=90,
    colors=colors,
    explode=(0.05, 0, 0, 0, 0),  # Explode first slice
    shadow=True,
    textprops={'fontsize': 11, 'fontweight': 'bold'}
)

# Make percentage text white for better contrast
for autotext in autotexts:
    autotext.set_color('white')
    autotext.set_fontsize(10)

# Title
ax.set_title('Revenue Breakdown by Category - 2024',
             fontsize=18, fontweight='bold', pad=20)

# Add total in center
total = sum(values)
ax.text(0, 0, f'Total\n${total/1_000_000:.1f}M',
        ha='center', va='center',
        fontsize=14, fontweight='bold',
        bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))

plt.tight_layout()
plt.show()
```

### Histogram

```python
fig, ax = plt.subplots(figsize=(12, 7))

# Generate sample data (e.g., customer transaction amounts)
np.random.seed(42)
transactions = np.random.lognormal(mean=4, sigma=0.8, size=1000) * 100

# Create histogram
n, bins, patches = ax.hist(transactions, bins=30, edgecolor='black',
                           linewidth=1.2, color='#3498db', alpha=0.7)

# Color bars by height (gradient effect)
cm = plt.cm.viridis
norm = plt.Normalize(vmin=n.min(), vmax=n.max())
for i, patch in enumerate(patches):
    patch.set_facecolor(cm(norm(n[i])))

# Format x-axis as currency
ax.xaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x:,.0f}'
))

# Format y-axis with commas
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'{int(x):,}'
))

# Add mean and median lines
mean_val = np.mean(transactions)
median_val = np.median(transactions)

ax.axvline(mean_val, color='red', linestyle='--', linewidth=2,
           label=f'Mean: ${mean_val:,.0f}')
ax.axvline(median_val, color='green', linestyle='--', linewidth=2,
           label=f'Median: ${median_val:,.0f}')

# Titles and labels
ax.set_title('Distribution of Customer Transaction Amounts',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Transaction Amount (USD)', fontsize=14, fontweight='bold')
ax.set_ylabel('Frequency (Number of Transactions)', fontsize=14, fontweight='bold')

# Legend
ax.legend(fontsize=12, frameon=True, shadow=True)

# Grid
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

plt.tight_layout()
plt.show()
```

### Box Plot

```python
fig, ax = plt.subplots(figsize=(14, 7))

# Sample data - sales by region
np.random.seed(42)
regions = ['North', 'South', 'East', 'West', 'Central']
data = [
    np.random.normal(50000, 15000, 100),
    np.random.normal(45000, 12000, 100),
    np.random.normal(55000, 18000, 100),
    np.random.normal(48000, 14000, 100),
    np.random.normal(52000, 16000, 100),
]

# Create box plot
bp = ax.boxplot(data, labels=regions, patch_artist=True,
                notch=True, showmeans=True,
                boxprops=dict(facecolor='#3498db', alpha=0.7),
                medianprops=dict(color='red', linewidth=2),
                meanprops=dict(marker='D', markerfacecolor='green',
                              markeredgecolor='black', markersize=8),
                whiskerprops=dict(linewidth=1.5),
                capprops=dict(linewidth=1.5))

# Format y-axis as currency
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Titles and labels
ax.set_title('Sales Distribution by Region - 2024',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Region', fontsize=14, fontweight='bold')
ax.set_ylabel('Sales (USD)', fontsize=14, fontweight='bold')

# Add legend for mean and median
from matplotlib.patches import Patch
legend_elements = [
    Patch(facecolor='red', label='Median'),
    plt.Line2D([0], [0], marker='D', color='w',
               markerfacecolor='green', markeredgecolor='black',
               markersize=8, label='Mean')
]
ax.legend(handles=legend_elements, fontsize=12, frameon=True, shadow=True)

# Grid
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

plt.tight_layout()
plt.show()
```

### Subplots - Multiple Charts

```python
fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(16, 12))

# Define currency formatter
currency_fmt = ticker.FuncFormatter(lambda x, p: f'${x/1000:.0f}K')

# Plot 1: Line chart
months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun']
revenue = [125_000, 138_000, 145_000, 152_000, 168_000, 178_000]
ax1.plot(months, revenue, marker='o', linewidth=2.5, markersize=8, color='#3498db')
ax1.yaxis.set_major_formatter(currency_fmt)
ax1.set_title('Monthly Revenue Trend', fontsize=14, fontweight='bold', pad=10)
ax1.set_ylabel('Revenue (USD)', fontsize=12, fontweight='bold')
ax1.grid(True, alpha=0.3)

# Plot 2: Bar chart
products = ['A', 'B', 'C', 'D']
sales = [245_000, 312_000, 198_000, 275_000]
ax2.bar(products, sales, color='#2ecc71', edgecolor='black', linewidth=1.2)
ax2.yaxis.set_major_formatter(currency_fmt)
ax2.set_title('Product Sales', fontsize=14, fontweight='bold', pad=10)
ax2.set_ylabel('Sales (USD)', fontsize=12, fontweight='bold')
ax2.grid(True, axis='y', alpha=0.3)

# Plot 3: Pie chart
categories = ['Online', 'Retail', 'Wholesale']
values = [45, 35, 20]
colors = ['#3498db', '#2ecc71', '#f39c12']
ax3.pie(values, labels=categories, autopct='%1.1f%%', colors=colors,
        startangle=90, textprops={'fontsize': 11, 'fontweight': 'bold'})
ax3.set_title('Sales Channel Distribution', fontsize=14, fontweight='bold', pad=10)

# Plot 4: Horizontal bar
regions = ['North', 'South', 'East', 'West']
customers = [1250, 980, 1450, 1120]
ax4.barh(regions, customers, color='#e74c3c', edgecolor='black', linewidth=1.2)
ax4.xaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: f'{int(x):,}'))
ax4.set_title('Customer Count by Region', fontsize=14, fontweight='bold', pad=10)
ax4.set_xlabel('Number of Customers', fontsize=12, fontweight='bold')
ax4.grid(True, axis='x', alpha=0.3)

# Overall title
fig.suptitle('Q4 2024 Business Dashboard', fontsize=20, fontweight='bold', y=0.995)

plt.tight_layout()
plt.show()
```

## Styling and Aesthetics

### Available Styles

```python
# See all available styles
print(plt.style.available)

# Popular styles
plt.style.use('seaborn-v0_8-darkgrid')
plt.style.use('seaborn-v0_8-whitegrid')
plt.style.use('seaborn-v0_8-dark')
plt.style.use('ggplot')
plt.style.use('fivethirtyeight')
plt.style.use('bmh')
```

### Custom Style with Seaborn

```python
import seaborn as sns

# Set seaborn style
sns.set_style("whitegrid")
sns.set_context("talk")  # or "paper", "notebook", "poster"

# Set color palette
sns.set_palette("husl")  # or "Set2", "pastel", "dark", "colorblind"
```

### Professional Color Palettes

```python
# Qualitative (for categories)
colors_professional = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd']
colors_corporate = ['#003f5c', '#58508d', '#bc5090', '#ff6361', '#ffa600']
colors_pastel = ['#a8dadc', '#457b9d', '#1d3557', '#f1faee', '#e63946']

# Sequential (for gradients)
colors_blues = plt.cm.Blues(np.linspace(0.3, 0.9, 5))
colors_greens = plt.cm.Greens(np.linspace(0.3, 0.9, 5))

# Diverging (for showing deviation)
colors_diverging = plt.cm.RdYlGn(np.linspace(0.1, 0.9, 5))
```

### Custom rcParams Configuration

```python
# Save this in a script or notebook for consistent styling
plt.rcParams.update({
    # Figure
    'figure.figsize': (12, 6),
    'figure.dpi': 100,
    'savefig.dpi': 300,
    'figure.facecolor': 'white',

    # Fonts
    'font.size': 12,
    'font.family': 'sans-serif',
    'font.sans-serif': ['Arial', 'DejaVu Sans'],

    # Axes
    'axes.titlesize': 16,
    'axes.titleweight': 'bold',
    'axes.labelsize': 14,
    'axes.labelweight': 'bold',
    'axes.spines.top': False,
    'axes.spines.right': False,
    'axes.grid': True,
    'axes.axisbelow': True,

    # Ticks
    'xtick.labelsize': 12,
    'ytick.labelsize': 12,
    'xtick.direction': 'out',
    'ytick.direction': 'out',

    # Legend
    'legend.fontsize': 12,
    'legend.frameon': True,
    'legend.shadow': True,

    # Grid
    'grid.alpha': 0.3,
    'grid.linestyle': '--',

    # Lines
    'lines.linewidth': 2,
    'lines.markersize': 8,
})
```

## Annotations and Labels

### Adding Text Annotations

```python
fig, ax = plt.subplots(figsize=(12, 7))

months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun']
revenue = [125_000, 138_000, 145_000, 152_000, 168_000, 178_000]

ax.plot(months, revenue, marker='o', linewidth=2.5, markersize=8, color='#3498db')

# Annotate highest point
max_idx = np.argmax(revenue)
ax.annotate(
    f'Peak: ${revenue[max_idx]:,}',
    xy=(max_idx, revenue[max_idx]),
    xytext=(max_idx - 1, revenue[max_idx] + 10000),
    arrowprops=dict(facecolor='red', shrink=0.05, width=2),
    fontsize=12,
    fontweight='bold',
    bbox=dict(boxstyle='round,pad=0.5', facecolor='yellow', alpha=0.7)
)

ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, p: f'${x/1000:.0f}K'))
ax.set_title('Revenue with Peak Annotation', fontsize=16, fontweight='bold', pad=20)
ax.grid(True, alpha=0.3)

plt.tight_layout()
plt.show()
```

### Adding Reference Lines

```python
fig, ax = plt.subplots(figsize=(12, 7))

# Data
performance = [85, 92, 78, 95, 88, 91, 87, 94]
employees = ['Alice', 'Bob', 'Charlie', 'Diana', 'Eve', 'Frank', 'Grace', 'Henry']

ax.barh(employees, performance, color='#3498db', edgecolor='black', linewidth=1.2)

# Add target line
target = 90
ax.axvline(target, color='red', linestyle='--', linewidth=2,
           label=f'Target: {target}%')

# Add average line
avg = np.mean(performance)
ax.axvline(avg, color='green', linestyle='--', linewidth=2,
           label=f'Average: {avg:.1f}%')

ax.set_title('Employee Performance vs Target', fontsize=16, fontweight='bold', pad=20)
ax.set_xlabel('Performance Score (%)', fontsize=14, fontweight='bold')
ax.legend(fontsize=12, frameon=True, shadow=True)
ax.grid(True, axis='x', alpha=0.3)

plt.tight_layout()
plt.show()
```

## Saving Figures

```python
# Save with high DPI for publications
fig.savefig('revenue_chart.png', dpi=300, bbox_inches='tight',
            facecolor='white', edgecolor='none')

# Save as PDF (vector format - scales without quality loss)
fig.savefig('revenue_chart.pdf', bbox_inches='tight')

# Save as SVG (for web, editable in Illustrator)
fig.savefig('revenue_chart.svg', bbox_inches='tight')

# Save with transparent background
fig.savefig('revenue_chart.png', dpi=300, bbox_inches='tight',
            transparent=True)
```

## Common Formatting Functions

### Reusable Formatters

```python
import matplotlib.ticker as ticker

def currency_formatter(x, p):
    """Format as currency with appropriate suffix"""
    if abs(x) >= 1_000_000_000:
        return f'${x/1_000_000_000:.1f}B'
    elif abs(x) >= 1_000_000:
        return f'${x/1_000_000:.1f}M'
    elif abs(x) >= 1_000:
        return f'${x/1_000:.0f}K'
    else:
        return f'${x:.0f}'

def number_formatter(x, p):
    """Format numbers with commas"""
    return f'{int(x):,}'

def percent_formatter(x, p):
    """Format as percentage"""
    return f'{x:.1f}%'

def millions_formatter(x, p):
    """Format as millions"""
    return f'{x/1_000_000:.1f}M'

def thousands_formatter(x, p):
    """Format as thousands"""
    return f'{x/1_000:.0f}K'

# Usage
ax.yaxis.set_major_formatter(ticker.FuncFormatter(currency_formatter))
ax.xaxis.set_major_formatter(ticker.FuncFormatter(number_formatter))
```

## Time Series Formatting

```python
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime, timedelta
import numpy as np

# Generate date range
start_date = datetime(2024, 1, 1)
dates = [start_date + timedelta(days=x) for x in range(365)]
values = np.cumsum(np.random.randn(365) * 1000) + 100_000

fig, ax = plt.subplots(figsize=(14, 7))

# Plot
ax.plot(dates, values, linewidth=2, color='#3498db')

# Format x-axis as dates
ax.xaxis.set_major_formatter(mdates.DateFormatter('%b %Y'))
ax.xaxis.set_major_locator(mdates.MonthLocator(interval=2))

# Format y-axis as currency
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'
))

# Rotate date labels
plt.xticks(rotation=45, ha='right')

# Titles and labels
ax.set_title('Daily Revenue - 2024', fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('Date', fontsize=14, fontweight='bold')
ax.set_ylabel('Revenue (USD)', fontsize=14, fontweight='bold')

ax.grid(True, alpha=0.3, linestyle='--')

plt.tight_layout()
plt.show()
```

## Best Practices Checklist

### Every Plot Should Have:

1. **Clear, descriptive title**
   - What is being shown
   - Time period if applicable
   - Context (e.g., "Q4 2024", "Year-over-Year")

2. **Labeled axes with units**
   - X-axis label
   - Y-axis label
   - Units in parentheses (USD), (%), (Units), etc.

3. **Formatted numbers**
   - Commas for thousands: 1,000 not 1000
   - Currency symbols: $1,000 not 1000
   - Percentages: 45% not 0.45
   - Appropriate precision: $1.2M not $1,234,567

4. **Readable fonts**
   - Title: 16-18pt, bold
   - Axis labels: 14pt, bold
   - Tick labels: 12pt
   - Legend: 12pt

5. **Appropriate colors**
   - Use colorblind-friendly palettes
   - Consistent colors for same categories
   - Good contrast

6. **Legend (when needed)**
   - Clear labels
   - Positioned to not overlap data
   - Use `frameon=True, shadow=True` for readability

7. **Grid (optional but recommended)**
   - Light alpha (0.3)
   - Behind data (`ax.set_axisbelow(True)`)
   - Horizontal only for bar charts

8. **Proper sizing**
   - Figure size appropriate for content
   - DPI 100 for display, 300 for print
   - Use `tight_layout()` to prevent label cutoff

## Complete Example: Professional Report Chart

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import numpy as np

# Set style
plt.style.use('seaborn-v0_8-whitegrid')

# Create figure
fig, ax = plt.subplots(figsize=(14, 8))

# Data
quarters = ['Q1\n2023', 'Q2\n2023', 'Q3\n2023', 'Q4\n2023',
            'Q1\n2024', 'Q2\n2024', 'Q3\n2024', 'Q4\n2024']
revenue = [1_245_000, 1_389_000, 1_512_000, 1_678_000,
           1_823_000, 2_012_000, 2_156_000, 2_387_000]
target = [1_200_000, 1_350_000, 1_500_000, 1_650_000,
          1_800_000, 1_950_000, 2_100_000, 2_250_000]

# Plot bars
x = np.arange(len(quarters))
width = 0.35

bars1 = ax.bar(x - width/2, revenue, width, label='Actual Revenue',
               color='#2ecc71', edgecolor='black', linewidth=1.5)
bars2 = ax.bar(x + width/2, target, width, label='Target',
               color='#3498db', edgecolor='black', linewidth=1.5, alpha=0.7)

# Add value labels on bars
for i, (bar, val) in enumerate(zip(bars1, revenue)):
    ax.text(bar.get_x() + bar.get_width()/2, val + 30000,
            f'${val/1_000_000:.2f}M',
            ha='center', va='bottom', fontsize=10, fontweight='bold')

# Format y-axis
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1_000_000:.1f}M'
))

# Titles and labels
ax.set_title('Quarterly Revenue Performance: Actual vs Target\n2023-2024 Comparison',
             fontsize=20, fontweight='bold', pad=20)
ax.set_xlabel('Quarter', fontsize=14, fontweight='bold')
ax.set_ylabel('Revenue (Millions USD)', fontsize=14, fontweight='bold')
ax.set_xticks(x)
ax.set_xticklabels(quarters, fontsize=11)

# Legend
ax.legend(loc='upper left', fontsize=13, frameon=True, shadow=True)

# Grid
ax.grid(True, axis='y', alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

# Add growth annotation
total_growth = ((revenue[-1] - revenue[0]) / revenue[0]) * 100
ax.text(0.98, 0.97, f'Total Growth: {total_growth:.1f}%',
        transform=ax.transAxes,
        fontsize=14, fontweight='bold',
        bbox=dict(boxstyle='round', facecolor='yellow', alpha=0.7),
        ha='right', va='top')

# Adjust layout
plt.tight_layout()

# Save
plt.savefig('quarterly_revenue_report.png', dpi=300, bbox_inches='tight',
            facecolor='white')

plt.show()
```

## Quick Reference Template

```python
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

# Setup
fig, ax = plt.subplots(figsize=(12, 7))

# Your plot here
ax.plot(x, y)  # or ax.bar(), ax.scatter(), etc.

# Format axes
ax.yaxis.set_major_formatter(ticker.FuncFormatter(
    lambda x, p: f'${x/1000:.0f}K'  # Adjust as needed
))

# Titles and labels
ax.set_title('Your Descriptive Title Here',
             fontsize=18, fontweight='bold', pad=20)
ax.set_xlabel('X Axis Label (Units)', fontsize=14, fontweight='bold')
ax.set_ylabel('Y Axis Label (Units)', fontsize=14, fontweight='bold')

# Optional: Legend
ax.legend(fontsize=12, frameon=True, shadow=True)

# Optional: Grid
ax.grid(True, alpha=0.3, linestyle='--')
ax.set_axisbelow(True)

# Finalize
plt.tight_layout()
plt.show()
```
