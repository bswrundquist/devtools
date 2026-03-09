---
name: altair
description: Use when creating interactive, declarative visualizations with Altair. Create professional charts with proper formatting (commas for thousands, $ for currency), clear titles, labeled axes, and interactive tooltips. Prefer Altair for modern, web-ready visualizations.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Altair

Expert guidance for creating interactive, declarative visualizations with Altair.

## Why Altair?

- **Declarative** - Describe what you want, not how to draw it
- **Interactive by default** - Tooltips, zoom, pan built-in
- **Concise syntax** - Less code than matplotlib
- **Grammar of Graphics** - Consistent, composable API
- **Web-ready** - Outputs to HTML, JSON (Vega-Lite)
- **Beautiful defaults** - Professional look out of the box

## Core Principles

1. **Clear titles** - Descriptive and informative
2. **Labeled axes** - Always label with units
3. **Formatted numbers** - Commas, currency symbols, percentages
4. **Interactive tooltips** - Show details on hover
5. **Professional styling** - Use themes and color schemes
6. **Proper sizing** - Appropriate chart dimensions
7. **Composability** - Layer, concatenate, facet charts

## Basic Setup

```python
import altair as alt
import pandas as pd
import numpy as np

# Enable data transformer for larger datasets
alt.data_transformers.enable('default')

# Set default chart size
alt.themes.enable('default')
```

## Number Formatting

### Currency Formatting

```python
import altair as alt
import pandas as pd

# Sample data
data = pd.DataFrame({
    'quarter': ['Q1', 'Q2', 'Q3', 'Q4'],
    'revenue': [1_250_000, 1_480_000, 1_720_000, 2_100_000]
})

chart = alt.Chart(data).mark_bar(
    color='#2ecc71',
    size=50
).encode(
    x=alt.X('quarter:N', title='Quarter', axis=alt.Axis(labelAngle=0)),
    y=alt.Y('revenue:Q',
            title='Revenue (USD)',
            axis=alt.Axis(format='$,.0f')),  # Currency with commas
    tooltip=[
        alt.Tooltip('quarter:N', title='Quarter'),
        alt.Tooltip('revenue:Q', title='Revenue', format='$,.2f')
    ]
).properties(
    title={
        'text': 'Quarterly Revenue Performance',
        'fontSize': 18,
        'fontWeight': 'bold'
    },
    width=600,
    height=400
)

chart
```

### Thousands with Commas

```python
data = pd.DataFrame({
    'year': [2020, 2021, 2022, 2023, 2024],
    'customers': [45_000, 52_000, 58_000, 63_000, 71_000]
})

chart = alt.Chart(data).mark_line(
    point=True,
    strokeWidth=3
).encode(
    x=alt.X('year:O', title='Year'),
    y=alt.Y('customers:Q',
            title='Number of Customers',
            axis=alt.Axis(format=',d')),  # Thousands with commas
    tooltip=[
        alt.Tooltip('year:O', title='Year'),
        alt.Tooltip('customers:Q', title='Customers', format=',d')
    ]
).properties(
    title='Customer Growth (2020-2024)',
    width=600,
    height=400
)

chart
```

### Percentage Formatting

```python
data = pd.DataFrame({
    'product': ['Product A', 'Product B', 'Product C', 'Product D'],
    'market_share': [0.35, 0.28, 0.22, 0.15]
})

chart = alt.Chart(data).mark_bar(color='#3498db').encode(
    x=alt.X('product:N', title='Product'),
    y=alt.Y('market_share:Q',
            title='Market Share',
            axis=alt.Axis(format='.0%')),  # Percentage format
    tooltip=[
        alt.Tooltip('product:N', title='Product'),
        alt.Tooltip('market_share:Q', title='Market Share', format='.1%')
    ]
).properties(
    title='Market Share by Product',
    width=600,
    height=400
)

chart
```

### Custom Formatting (Millions, Billions)

```python
data = pd.DataFrame({
    'year': [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024],
    'population': [7.3e9, 7.4e9, 7.5e9, 7.6e9, 7.7e9,
                   7.8e9, 7.9e9, 8.0e9, 8.1e9, 8.2e9]
})

chart = alt.Chart(data).mark_line(
    point=alt.OverlayMarkDef(size=100),
    strokeWidth=3,
    color='#e74c3c'
).encode(
    x=alt.X('year:O', title='Year'),
    y=alt.Y('population:Q',
            title='Population (Billions)',
            axis=alt.Axis(format='.1s')),  # SI prefix (7.3G becomes 7.3B)
    tooltip=[
        alt.Tooltip('year:O', title='Year'),
        alt.Tooltip('population:Q', title='Population', format='.2s')
    ]
).properties(
    title='World Population Growth',
    width=700,
    height=400
)

chart
```

## Format Specifiers Reference

```python
# Common format patterns for axis and tooltips

# Numbers
'.0f'    # No decimal places: 1234
',.0f'   # Thousands separator: 1,234
',.2f'   # Two decimals with separator: 1,234.56
'.2s'    # SI prefix with 2 decimals: 1.23k, 4.56M, 7.89B

# Currency
'$,.0f'  # Currency with commas: $1,234
'$,.2f'  # Currency with cents: $1,234.56

# Percentages
'.0%'    # Percentage, no decimals: 45%
'.1%'    # Percentage, 1 decimal: 45.5%
'.2%'    # Percentage, 2 decimals: 45.67%

# Custom via formatType
# Use 'formatType': 'number' or 'time' for more control
```

## Complete Chart Examples

### Line Chart

```python
import altair as alt
import pandas as pd

# Sample data
data = pd.DataFrame({
    'month': ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
              'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
    'revenue_2023': [125_000, 132_000, 145_000, 138_000, 155_000, 162_000,
                     178_000, 185_000, 192_000, 201_000, 215_000, 235_000],
    'revenue_2024': [145_000, 158_000, 172_000, 165_000, 182_000, 195_000,
                     215_000, 228_000, 242_000, 258_000, 275_000, 298_000]
})

# Reshape for Altair (long format)
data_long = pd.melt(
    data,
    id_vars=['month'],
    value_vars=['revenue_2023', 'revenue_2024'],
    var_name='year',
    value_name='revenue'
)
data_long['year'] = data_long['year'].str.replace('revenue_', '')

# Create chart
chart = alt.Chart(data_long).mark_line(
    point=True,
    strokeWidth=3
).encode(
    x=alt.X('month:N',
            title='Month',
            sort=['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
                  'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']),
    y=alt.Y('revenue:Q',
            title='Revenue (USD)',
            axis=alt.Axis(format='$,.0f')),
    color=alt.Color('year:N',
                    title='Year',
                    scale=alt.Scale(scheme='category10')),
    tooltip=[
        alt.Tooltip('month:N', title='Month'),
        alt.Tooltip('year:N', title='Year'),
        alt.Tooltip('revenue:Q', title='Revenue', format='$,.0f')
    ]
).properties(
    title={
        'text': 'Monthly Revenue Comparison: 2023 vs 2024',
        'fontSize': 18,
        'fontWeight': 'bold',
        'anchor': 'start'
    },
    width=800,
    height=400
).configure_axis(
    labelFontSize=12,
    titleFontSize=14,
    titleFontWeight='bold'
).configure_legend(
    titleFontSize=12,
    labelFontSize=11
)

chart
```

### Bar Chart with Value Labels

```python
data = pd.DataFrame({
    'product': ['Product A', 'Product B', 'Product C', 'Product D', 'Product E'],
    'sales': [245_000, 312_000, 198_000, 275_000, 423_000]
})

# Base bar chart
bars = alt.Chart(data).mark_bar(
    color='#3498db',
    cornerRadiusTopLeft=5,
    cornerRadiusTopRight=5
).encode(
    x=alt.X('product:N', title='Product', axis=alt.Axis(labelAngle=0)),
    y=alt.Y('sales:Q', title='Sales (USD)', axis=alt.Axis(format='$,.0f')),
    tooltip=[
        alt.Tooltip('product:N', title='Product'),
        alt.Tooltip('sales:Q', title='Sales', format='$,.0f')
    ]
)

# Text labels on bars
text = bars.mark_text(
    align='center',
    baseline='bottom',
    dy=-5,
    fontSize=12,
    fontWeight='bold'
).encode(
    text=alt.Text('sales:Q', format='$,.0f')
)

# Combine
chart = (bars + text).properties(
    title={
        'text': 'Product Sales Performance - Q4 2024',
        'fontSize': 18,
        'fontWeight': 'bold'
    },
    width=700,
    height=400
).configure_axis(
    labelFontSize=12,
    titleFontSize=14,
    titleFontWeight='bold'
)

chart
```

### Horizontal Bar Chart

```python
data = pd.DataFrame({
    'department': [
        'Sales & Marketing',
        'Product Development',
        'Customer Support',
        'Engineering',
        'Human Resources',
        'Finance & Accounting'
    ],
    'budget': [850_000, 1_200_000, 450_000, 1_500_000, 320_000, 280_000]
})

# Sort by budget
data = data.sort_values('budget')

chart = alt.Chart(data).mark_bar(
    color='#2ecc71'
).encode(
    y=alt.Y('department:N',
            title='Department',
            sort='-x'),  # Sort by x descending
    x=alt.X('budget:Q',
            title='Budget (USD)',
            axis=alt.Axis(format='$,.0f')),
    tooltip=[
        alt.Tooltip('department:N', title='Department'),
        alt.Tooltip('budget:Q', title='Budget', format='$,.2f')
    ]
).properties(
    title='Department Budget Allocation - FY 2024',
    width=700,
    height=400
)

chart
```

### Grouped Bar Chart

```python
data = pd.DataFrame({
    'quarter': ['Q1', 'Q2', 'Q3', 'Q4'] * 3,
    'region': ['East'] * 4 + ['West'] * 4 + ['Central'] * 4,
    'revenue': [425_000, 478_000, 512_000, 589_000,  # East
                398_000, 445_000, 502_000, 612_000,  # West
                312_000, 358_000, 391_000, 445_000]  # Central
})

chart = alt.Chart(data).mark_bar().encode(
    x=alt.X('quarter:N', title='Quarter'),
    y=alt.Y('revenue:Q', title='Revenue (USD)', axis=alt.Axis(format='$,.0f')),
    color=alt.Color('region:N',
                    title='Region',
                    scale=alt.Scale(scheme='category10')),
    xOffset='region:N',  # Group bars
    tooltip=[
        alt.Tooltip('quarter:N', title='Quarter'),
        alt.Tooltip('region:N', title='Region'),
        alt.Tooltip('revenue:Q', title='Revenue', format='$,.0f')
    ]
).properties(
    title='Regional Revenue by Quarter - 2024',
    width=700,
    height=400
)

chart
```

### Stacked Bar Chart

```python
data = pd.DataFrame({
    'month': ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'] * 3,
    'channel': ['Online'] * 6 + ['Retail'] * 6 + ['Wholesale'] * 6,
    'sales': [125_000, 138_000, 145_000, 152_000, 168_000, 178_000,  # Online
              215_000, 198_000, 205_000, 212_000, 225_000, 238_000,  # Retail
              98_000, 105_000, 112_000, 118_000, 125_000, 132_000]   # Wholesale
})

chart = alt.Chart(data).mark_bar().encode(
    x=alt.X('month:N',
            title='Month',
            sort=['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun']),
    y=alt.Y('sales:Q', title='Sales (USD)', axis=alt.Axis(format='$,.0f')),
    color=alt.Color('channel:N',
                    title='Sales Channel',
                    scale=alt.Scale(scheme='category10')),
    tooltip=[
        alt.Tooltip('month:N', title='Month'),
        alt.Tooltip('channel:N', title='Channel'),
        alt.Tooltip('sales:Q', title='Sales', format='$,.0f')
    ]
).properties(
    title='Sales by Channel - First Half 2024',
    width=700,
    height=400
)

chart
```

### Scatter Plot

```python
np.random.seed(42)
data = pd.DataFrame({
    'marketing_spend': np.random.uniform(10_000, 100_000, 50),
    'revenue': np.random.uniform(50_000, 400_000, 50),
    'region': np.random.choice(['North', 'South', 'East', 'West'], 50)
})

# Add size column
data['deal_size'] = np.random.uniform(1000, 50000, 50)

chart = alt.Chart(data).mark_circle(
    opacity=0.7
).encode(
    x=alt.X('marketing_spend:Q',
            title='Marketing Spend (USD)',
            axis=alt.Axis(format='$,.0f'),
            scale=alt.Scale(zero=False)),
    y=alt.Y('revenue:Q',
            title='Revenue (USD)',
            axis=alt.Axis(format='$,.0f'),
            scale=alt.Scale(zero=False)),
    size=alt.Size('deal_size:Q',
                  title='Deal Size',
                  scale=alt.Scale(range=[100, 1000]),
                  legend=None),
    color=alt.Color('region:N',
                    title='Region',
                    scale=alt.Scale(scheme='category10')),
    tooltip=[
        alt.Tooltip('marketing_spend:Q', title='Marketing Spend', format='$,.0f'),
        alt.Tooltip('revenue:Q', title='Revenue', format='$,.0f'),
        alt.Tooltip('deal_size:Q', title='Deal Size', format='$,.0f'),
        alt.Tooltip('region:N', title='Region')
    ]
).properties(
    title='Marketing Spend vs Revenue by Region',
    width=700,
    height=500
)

chart
```

### Area Chart

```python
data = pd.DataFrame({
    'date': pd.date_range('2024-01-01', periods=365, freq='D'),
    'revenue': np.cumsum(np.random.randn(365) * 1000) + 100_000
})

chart = alt.Chart(data).mark_area(
    line=True,
    color='#3498db',
    opacity=0.5
).encode(
    x=alt.X('date:T', title='Date', axis=alt.Axis(format='%b %Y')),
    y=alt.Y('revenue:Q',
            title='Cumulative Revenue (USD)',
            axis=alt.Axis(format='$,.0f')),
    tooltip=[
        alt.Tooltip('date:T', title='Date', format='%Y-%m-%d'),
        alt.Tooltip('revenue:Q', title='Revenue', format='$,.0f')
    ]
).properties(
    title='Cumulative Revenue - 2024',
    width=800,
    height=400
)

chart
```

### Histogram

```python
np.random.seed(42)
data = pd.DataFrame({
    'transaction_amount': np.random.lognormal(mean=4, sigma=0.8, size=1000) * 100
})

chart = alt.Chart(data).mark_bar(
    color='#2ecc71',
    opacity=0.7
).encode(
    x=alt.X('transaction_amount:Q',
            title='Transaction Amount (USD)',
            bin=alt.Bin(maxbins=30),
            axis=alt.Axis(format='$,.0f')),
    y=alt.Y('count()', title='Frequency (Number of Transactions)'),
    tooltip=[
        alt.Tooltip('transaction_amount:Q', title='Amount Range', format='$,.0f', bin=True),
        alt.Tooltip('count()', title='Count')
    ]
).properties(
    title='Distribution of Transaction Amounts',
    width=700,
    height=400
)

chart
```

### Box Plot

```python
np.random.seed(42)
data = pd.DataFrame({
    'region': np.repeat(['North', 'South', 'East', 'West', 'Central'], 100),
    'sales': np.concatenate([
        np.random.normal(50_000, 15_000, 100),
        np.random.normal(45_000, 12_000, 100),
        np.random.normal(55_000, 18_000, 100),
        np.random.normal(48_000, 14_000, 100),
        np.random.normal(52_000, 16_000, 100)
    ])
})

chart = alt.Chart(data).mark_boxplot(
    size=50,
    ticks=True
).encode(
    x=alt.X('region:N', title='Region'),
    y=alt.Y('sales:Q',
            title='Sales (USD)',
            axis=alt.Axis(format='$,.0f')),
    color=alt.Color('region:N', legend=None, scale=alt.Scale(scheme='category10')),
    tooltip=[
        alt.Tooltip('region:N', title='Region'),
        alt.Tooltip('median(sales):Q', title='Median Sales', format='$,.0f'),
        alt.Tooltip('mean(sales):Q', title='Mean Sales', format='$,.0f')
    ]
).properties(
    title='Sales Distribution by Region - 2024',
    width=700,
    height=400
)

chart
```

### Pie/Donut Chart

```python
data = pd.DataFrame({
    'category': ['Product Sales', 'Services', 'Subscriptions', 'Licensing', 'Other'],
    'value': [2_450_000, 1_230_000, 890_000, 450_000, 180_000]
})

# Calculate percentages and total
data['percentage'] = data['value'] / data['value'].sum()
total = data['value'].sum()

# Pie chart
base = alt.Chart(data).encode(
    theta=alt.Theta('value:Q', stack=True),
    color=alt.Color('category:N',
                    title='Category',
                    scale=alt.Scale(scheme='category10')),
    tooltip=[
        alt.Tooltip('category:N', title='Category'),
        alt.Tooltip('value:Q', title='Revenue', format='$,.0f'),
        alt.Tooltip('percentage:Q', title='Percentage', format='.1%')
    ]
)

pie = base.mark_arc(
    outerRadius=150,
    innerRadius=60,  # Creates donut chart
    stroke='white',
    strokeWidth=2
)

# Add text labels
text = base.mark_text(
    radius=180,
    size=12,
    fontWeight='bold'
).encode(
    text=alt.Text('percentage:Q', format='.1%')
)

chart = (pie + text).properties(
    title={
        'text': f'Revenue Breakdown by Category - Total: ${total:,.0f}',
        'fontSize': 18,
        'fontWeight': 'bold'
    },
    width=500,
    height=500
)

chart
```

### Heatmap

```python
# Sample data - sales by day and hour
hours = list(range(24))
days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
data = pd.DataFrame([
    {'hour': h, 'day': d, 'sales': np.random.randint(1000, 10000)}
    for h in hours for d in days
])

chart = alt.Chart(data).mark_rect().encode(
    x=alt.X('hour:O', title='Hour of Day'),
    y=alt.Y('day:N', title='Day of Week', sort=days),
    color=alt.Color('sales:Q',
                    title='Sales (USD)',
                    scale=alt.Scale(scheme='viridis')),
    tooltip=[
        alt.Tooltip('day:N', title='Day'),
        alt.Tooltip('hour:O', title='Hour'),
        alt.Tooltip('sales:Q', title='Sales', format='$,.0f')
    ]
).properties(
    title='Sales Heatmap by Day and Hour',
    width=800,
    height=300
).configure_axis(
    labelFontSize=11
)

chart
```

## Layering Charts

### Line with Confidence Band

```python
data = pd.DataFrame({
    'x': range(100),
    'y': np.cumsum(np.random.randn(100)),
    'lower': np.cumsum(np.random.randn(100)) - 10,
    'upper': np.cumsum(np.random.randn(100)) + 10
})

# Confidence band
band = alt.Chart(data).mark_area(
    opacity=0.3,
    color='lightblue'
).encode(
    x='x:Q',
    y='lower:Q',
    y2='upper:Q'
)

# Line
line = alt.Chart(data).mark_line(
    color='darkblue',
    strokeWidth=2
).encode(
    x=alt.X('x:Q', title='Time Period'),
    y=alt.Y('y:Q', title='Value'),
    tooltip=['x:Q', 'y:Q']
)

chart = (band + line).properties(
    title='Forecast with Confidence Interval',
    width=700,
    height=400
)

chart
```

### Bar Chart with Average Line

```python
data = pd.DataFrame({
    'category': ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
    'value': [45, 67, 52, 89, 61, 73, 55, 68]
})

# Calculate average
avg_value = data['value'].mean()

# Bars
bars = alt.Chart(data).mark_bar(color='#3498db').encode(
    x=alt.X('category:N', title='Category'),
    y=alt.Y('value:Q', title='Value'),
    tooltip=['category:N', 'value:Q']
)

# Average line
avg_line = alt.Chart(pd.DataFrame({'y': [avg_value]})).mark_rule(
    color='red',
    strokeWidth=2,
    strokeDash=[5, 5]
).encode(
    y='y:Q'
)

# Text for average
avg_text = alt.Chart(pd.DataFrame({
    'x': ['H'],
    'y': [avg_value],
    'label': [f'Average: {avg_value:.1f}']
})).mark_text(
    align='left',
    dx=5,
    dy=-5,
    fontSize=12,
    fontWeight='bold',
    color='red'
).encode(
    x='x:N',
    y='y:Q',
    text='label:N'
)

chart = (bars + avg_line + avg_text).properties(
    title='Values by Category with Average',
    width=700,
    height=400
)

chart
```

## Concatenation (Multiple Charts)

### Horizontal Concatenation

```python
# Chart 1
chart1 = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q'
).properties(
    title='Chart 1',
    width=300,
    height=300
)

# Chart 2
chart2 = alt.Chart(data).mark_line(point=True).encode(
    x='category:N',
    y='value:Q'
).properties(
    title='Chart 2',
    width=300,
    height=300
)

# Combine horizontally
combined = chart1 | chart2

combined
```

### Vertical Concatenation

```python
# Combine vertically
combined = chart1 & chart2

combined
```

### Dashboard Grid

```python
# Create 2x2 dashboard
row1 = chart1 | chart2
row2 = chart3 | chart4
dashboard = row1 & row2

dashboard
```

## Faceting (Small Multiples)

### Column Facet

```python
data = pd.DataFrame({
    'month': ['Jan', 'Feb', 'Mar', 'Apr'] * 3,
    'region': ['North'] * 4 + ['South'] * 4 + ['West'] * 4,
    'sales': [100, 120, 115, 130, 95, 105, 110, 125, 105, 115, 108, 120]
})

chart = alt.Chart(data).mark_bar().encode(
    x=alt.X('month:N', title='Month'),
    y=alt.Y('sales:Q', title='Sales'),
    color='region:N',
    column=alt.Column('region:N', title='Region')  # Create columns
).properties(
    width=200,
    height=300,
    title='Sales by Region (Faceted)'
)

chart
```

### Row Facet

```python
chart = alt.Chart(data).mark_line(point=True).encode(
    x='month:N',
    y='sales:Q',
    color='region:N',
    row=alt.Row('region:N', title='Region')  # Create rows
).properties(
    width=500,
    height=150
)

chart
```

### Grid Facet

```python
chart = alt.Chart(data).mark_bar().encode(
    x='month:N',
    y='sales:Q',
    facet=alt.Facet('region:N', columns=2)  # Grid with 2 columns
).properties(
    width=250,
    height=200
)

chart
```

## Interactive Features

### Tooltips (Always Include)

```python
chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    tooltip=[
        alt.Tooltip('category:N', title='Category'),
        alt.Tooltip('value:Q', title='Value', format='$,.0f'),
        alt.Tooltip('value:Q', title='Percentage', format='.1%')  # Multiple formats
    ]
)
```

### Selection and Highlighting

```python
# Click selection
selection = alt.selection_point(fields=['category'])

chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    color=alt.condition(
        selection,
        alt.Color('category:N', legend=None),
        alt.value('lightgray')
    ),
    opacity=alt.condition(selection, alt.value(1), alt.value(0.3))
).add_params(
    selection
).properties(
    title='Click to highlight a category',
    width=600,
    height=400
)

chart
```

### Interval Selection (Zoom/Brush)

```python
brush = alt.selection_interval(encodings=['x'])

chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    color=alt.condition(brush, alt.value('#3498db'), alt.value('lightgray'))
).add_params(
    brush
).properties(
    title='Brush to select range',
    width=600,
    height=400
)

chart
```

### Linked Charts

```python
brush = alt.selection_interval()

# Chart 1 with selection
chart1 = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    color=alt.condition(brush, alt.Color('category:N'), alt.value('lightgray'))
).add_params(
    brush
).properties(width=300, height=300)

# Chart 2 filters based on selection
chart2 = alt.Chart(data).mark_bar().encode(
    x='subcategory:N',
    y='value:Q'
).transform_filter(
    brush
).properties(width=300, height=300)

# Combine
linked = chart1 | chart2

linked
```

## Theming and Styling

### Built-in Themes

```python
# Available themes
alt.themes.names()

# Enable theme
alt.themes.enable('default')      # Default Vega-Lite theme
alt.themes.enable('opaque')       # Opaque background
alt.themes.enable('dark')         # Dark theme
alt.themes.enable('fivethirtyeight')  # FiveThirtyEight style
alt.themes.enable('ggplot2')      # ggplot2 style
alt.themes.enable('latimes')      # LA Times style
alt.themes.enable('urbaninstitute')  # Urban Institute style
alt.themes.enable('vox')          # Vox style
```

### Custom Theme

```python
def custom_theme():
    return {
        'config': {
            'view': {'strokeWidth': 0},
            'axis': {
                'labelFontSize': 12,
                'titleFontSize': 14,
                'titleFontWeight': 'bold',
                'grid': True,
                'gridOpacity': 0.3
            },
            'title': {
                'fontSize': 18,
                'fontWeight': 'bold',
                'anchor': 'start'
            },
            'legend': {
                'titleFontSize': 12,
                'labelFontSize': 11
            }
        }
    }

# Register and enable
alt.themes.register('custom', custom_theme)
alt.themes.enable('custom')
```

### Color Schemes

```python
# Built-in schemes
# Categorical: 'category10', 'category20', 'tableau10', 'tableau20'
# Sequential: 'blues', 'greens', 'greys', 'oranges', 'purples', 'reds'
# Diverging: 'blueorange', 'brownbluegreen', 'purplegreen', 'redblue'
# Perceptual: 'viridis', 'magma', 'inferno', 'plasma'

chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    color=alt.Color('category:N',
                    scale=alt.Scale(scheme='tableau10'))  # Use color scheme
)
```

### Custom Colors

```python
# Define custom color palette
custom_colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd']

chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q',
    color=alt.Color('category:N',
                    scale=alt.Scale(range=custom_colors))
)
```

## Saving Charts

```python
# Save as HTML (interactive)
chart.save('chart.html')

# Save as PNG (static, requires altair_saver)
chart.save('chart.png', scale_factor=2.0)  # Higher resolution

# Save as SVG (vector, requires altair_saver)
chart.save('chart.svg')

# Save as JSON (Vega-Lite spec)
chart.save('chart.json')

# Get Vega-Lite spec as dict
spec = chart.to_dict()
```

## Configuration Methods

### Global Configuration

```python
chart = alt.Chart(data).mark_bar().encode(
    x='category:N',
    y='value:Q'
).configure_mark(
    color='#3498db',
    opacity=0.8
).configure_axis(
    labelFontSize=12,
    titleFontSize=14,
    titleFontWeight='bold',
    grid=True,
    gridOpacity=0.3
).configure_title(
    fontSize=18,
    fontWeight='bold',
    anchor='start',
    offset=20
).configure_legend(
    titleFontSize=12,
    labelFontSize=11,
    strokeColor='gray',
    padding=10
).configure_view(
    strokeWidth=0  # Remove border
)
```

## Best Practices

### 1. Always Format Numbers

```python
# ✅ Good - Formatted
y=alt.Y('revenue:Q',
        title='Revenue (USD)',
        axis=alt.Axis(format='$,.0f'))

# ❌ Bad - No formatting
y='revenue:Q'
```

### 2. Include Descriptive Tooltips

```python
# ✅ Good - Informative tooltips
tooltip=[
    alt.Tooltip('month:N', title='Month'),
    alt.Tooltip('revenue:Q', title='Revenue', format='$,.0f'),
    alt.Tooltip('growth:Q', title='Growth Rate', format='.1%')
]

# ❌ Bad - Default tooltip
tooltip=True
```

### 3. Use Proper Titles

```python
# ✅ Good - Descriptive with context
title={
    'text': 'Quarterly Revenue Performance - 2024 vs 2023',
    'fontSize': 18,
    'fontWeight': 'bold'
}

# ❌ Bad - Generic
title='Revenue'
```

### 4. Label Axes with Units

```python
# ✅ Good - Clear labels with units
x=alt.X('date:T', title='Date'),
y=alt.Y('revenue:Q', title='Revenue (USD)')

# ❌ Bad - No context
x='date:T',
y='revenue:Q'
```

### 5. Set Appropriate Sizes

```python
# ✅ Good - Explicit sizing
.properties(
    width=700,
    height=400
)

# Consider:
# - Single chart: 600-800 width, 400-500 height
# - Dashboard: 300-400 per chart
# - Mobile: 300-400 width
```

### 6. Use Long Format Data

```python
# ✅ Good - Long format (tidy data)
data = pd.DataFrame({
    'month': ['Jan', 'Feb'] * 2,
    'metric': ['revenue', 'revenue', 'cost', 'cost'],
    'value': [100, 110, 50, 55]
})

# ❌ Avoid - Wide format (requires pivot_longer)
data = pd.DataFrame({
    'month': ['Jan', 'Feb'],
    'revenue': [100, 110],
    'cost': [50, 55]
})
```

## Common Patterns

### Time Series with Range Selector

```python
source = pd.DataFrame({
    'date': pd.date_range('2024-01-01', periods=365),
    'value': np.cumsum(np.random.randn(365))
})

brush = alt.selection_interval(encodings=['x'])

# Main chart
main = alt.Chart(source).mark_line().encode(
    x=alt.X('date:T', title='Date', scale=alt.Scale(domain=brush)),
    y=alt.Y('value:Q', title='Value')
).properties(width=700, height=300)

# Selector chart
selector = alt.Chart(source).mark_area().encode(
    x='date:T',
    y='value:Q'
).add_params(brush).properties(width=700, height=60)

# Combine
chart = main & selector

chart
```

### Comparison with Percentage Change

```python
data = pd.DataFrame({
    'category': ['A', 'B', 'C', 'D'],
    'current': [100, 150, 120, 180],
    'previous': [90, 140, 130, 160]
})

data['change'] = ((data['current'] - data['previous']) / data['previous'])

chart = alt.Chart(data).mark_bar().encode(
    x=alt.X('category:N', title='Category'),
    y=alt.Y('change:Q',
            title='Change from Previous Period',
            axis=alt.Axis(format='.1%')),
    color=alt.condition(
        alt.datum.change > 0,
        alt.value('#2ecc71'),  # Green for positive
        alt.value('#e74c3c')   # Red for negative
    ),
    tooltip=[
        alt.Tooltip('category:N', title='Category'),
        alt.Tooltip('previous:Q', title='Previous', format=',.0f'),
        alt.Tooltip('current:Q', title='Current', format=',.0f'),
        alt.Tooltip('change:Q', title='Change', format='.1%')
    ]
).properties(
    title='Performance Change by Category',
    width=600,
    height=400
)

chart
```

## Quick Reference Template

```python
import altair as alt
import pandas as pd

# Create data
data = pd.DataFrame({
    'category': ['A', 'B', 'C'],
    'value': [100, 150, 120]
})

# Create chart
chart = alt.Chart(data).mark_bar().encode(
    x=alt.X('category:N', title='Category Name'),
    y=alt.Y('value:Q',
            title='Value (Units)',
            axis=alt.Axis(format=',.0f')),  # Format numbers
    tooltip=[
        alt.Tooltip('category:N', title='Category'),
        alt.Tooltip('value:Q', title='Value', format=',.0f')
    ]
).properties(
    title={
        'text': 'Descriptive Title Here',
        'fontSize': 18,
        'fontWeight': 'bold'
    },
    width=700,
    height=400
).configure_axis(
    labelFontSize=12,
    titleFontSize=14,
    titleFontWeight='bold'
)

# Display or save
chart
# chart.save('chart.html')
```

## Installation

```bash
# Basic installation
pip install altair

# With recommended dependencies
pip install altair altair_saver vega_datasets

# Or with uv
uv pip install altair
```
