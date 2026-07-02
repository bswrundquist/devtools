---
name: model-monitoring
description: Use when setting up or debugging monitoring for a deployed ML model — feature/prediction drift, delayed-label performance tracking, feature pipeline quality, retraining triggers, and what to alert on. Data quality applied to the ML serving path.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Model Monitoring

Labels arrive late or never; drift is the early-warning system, and feature
pipeline quality is where most "model" incidents actually start. Monitor the
inputs hardest.

## The Four Layers

| Layer | Question | Signals | Latency |
|-------|----------|---------|---------|
| Operational | Is it serving? | error rate, p99 latency, throughput, null-prediction rate | seconds |
| Feature quality | Are inputs sane? | null rates, out-of-range, missing entities, feature freshness, train/serve schema match | minutes |
| Drift | Does today look like training? | PSI/KS per feature, prediction distribution shift | hours–days |
| Performance | Is it still right? | task metric on matured labels; proxy metrics meanwhile | days–weeks |

Most teams build 3 before 2. Backwards: a broken feature join degrades the
model *today* and is *fixable today* — drift tells you to investigate,
feature quality tells you what broke.

## Log at Serve Time (or nothing else works)

Per prediction: model+feature version, entity id, timestamp, **feature values
as served**, prediction (score, not just decision). Sample under high volume,
but never sample to zero. This table *is* the monitoring system — drift jobs,
label joins, and every incident investigation read from it. Recomputing
features later from source tells you what features *should have been*, which
is exactly the discrepancy you're trying to catch (train/serve skew).

## Drift That Doesn't Cry Wolf

PSI per feature, serving window vs training reference:

```python
def psi(expected: pl.Series, actual: pl.Series, bins: int = 10) -> float:
    cuts = expected.qcut(bins, include_breaks=True)  # bins from REFERENCE quantiles
    e = expected.cut(cuts).value_counts(normalize=True)
    a = actual.cut(cuts).value_counts(normalize=True)  # aligned, ε-smoothed
    return float(((a - e) * (a / e).log()).sum())
```

Conventional read: <0.1 stable, 0.1–0.25 investigate, >0.25 significant.
Treat as defaults to tune per feature, not laws.

- Reference = **training distribution**, fixed until retrain. A rolling
  reference adapts to gradual drift and never fires.
- Comparison window matches traffic seasonality (week vs week, not Tuesday
  vs training).
- Rank alerts by drift × feature importance — PSI 0.3 on the top feature is
  a page; on feature #47 it's a note.
- Prediction-distribution drift is the single best aggregate signal: cheap,
  label-free, and it summarizes all input drift the model actually responds
  to.

## Performance With Late Labels

- Join predictions to labels **by prediction time** as labels mature; a
  "last 30 days accuracy" metric silently measures only old traffic.
- Define per model: label maturity (when is a label final?), proxy metrics
  until then (click-through, downstream acceptance, human-override rate),
  and the correlation you've checked between proxy and true metric.
- Slice everything: overall metrics hide a collapsed segment (one region,
  one product line, new users). Alert on worst-slice, not just mean.
- Compare against the deployed baseline *and* a dumb baseline — a model
  drifting toward "predict the majority class" can look stable on accuracy.

## Retraining Triggers

Retraining is the response to *diagnosed* drift, not to any red number:

1. Alert fires → check feature quality first (most "drift" is a broken join
   or a stale feature — fix the pipeline, don't retrain on corrupt data).
2. Genuine world-change → retrain on recent data, evaluate on the *newest*
   matured slice (not a random split — that hides recency problems).
3. New model must beat deployed on the current-window eval before promotion;
   shadow or canary if the stakes justify it.
4. Log the trigger → action → outcome. A retrain that didn't recover the
   metric is a finding, not a checkbox.

## Rules

- No serve-time logging of features + score + versions → no monitoring.
  Build that first.
- Feature quality checks (nulls, ranges, freshness, schema vs training) are
  tier one — most model incidents are data incidents wearing a costume.
- Drift reference is the training distribution, fixed; rolling references
  self-blind.
- Weight drift alerts by feature importance; alert on prediction drift
  always.
- Performance joins labels by prediction time and reports by slice, with
  worst-slice alerting.
- Every alert carries its first diagnostic action ("check feature X null
  rate in serving logs"), or it will be ignored by week three.
- Never retrain as a reflex; diagnose whether the pipeline broke or the
  world changed.
- Silence is a failure mode: alert on the monitoring jobs *not running*,
  and on the score-logging volume dropping — a dead monitor looks exactly
  like a healthy model.
