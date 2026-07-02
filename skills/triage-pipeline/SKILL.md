---
name: triage-pipeline
description: Use when a data pipeline run failed or produced suspect output — a failed Airflow DAG, a stuck job, a table that looks wrong. Systematic triage to code/data/infra/upstream, impact assessment, and safe recovery (retry vs fix-forward vs backfill).
tools: Bash, Read, Grep, Glob
---

# Triage Pipeline

Classify before you fix: code, data, infra, or upstream. The retry button is
not a diagnosis — retrying a data problem just fails slower, and retrying a
non-idempotent task can corrupt output.

## Sequence

### 1. Find the First Failure

The alert usually points at a *downstream* casualty. Walk up the DAG to the
first failed/anomalous task — that's the crime scene; everything after is
collateral.

```bash
airflow dags list-runs -d <dag_id> --state failed -o table
airflow tasks states-for-dag-run <dag_id> <run_id>
```

### 2. Read the Actual Error

Get the failing task's log and find the root exception (usually above the
final stack trace — orchestrator noise buries it). Classify:

| Signal | Class |
|--------|-------|
| Stack trace in transform logic, works on yesterday's data | Code (recent deploy?) |
| Schema/type/constraint error, key error on a column | Data — upstream changed shape |
| OOM, disk, timeout, connection refused, quota/429 | Infra/platform |
| Empty input, "0 rows", upstream freshness check failed | Upstream |
| Task never started / stuck queued | Orchestrator (pool slots, paused, scheduler) |

### 3. Check What Changed

Failures have causes; "it just broke" means you haven't found it yet.

- **Code**: deploys/merges to the pipeline repo since the last green run.
- **Data**: input volume/schema today vs last green run (profile-dataset for
  a quick look; data-diff to compare runs).
- **Infra**: platform incidents, quota changes, cluster events, credential
  or token expiry (a classic for "worked yesterday, nothing changed").
- **Calendar**: month-end volume, DST, holidays — date-shaped breakage.

### 4. Assess Blast Radius Before Fixing

- Which downstream tasks/tables already ran on bad or partial input?
- Did the failing task **partially write** before dying?
- Who consumes the output (dashboards, ML features, exports) and have they
  already read today's bad data?

Post a one-line status to the owning channel *before* the deep dive: what's
broken, what's affected, that you're on it. Silent triage breeds duplicate
investigations.

### 5. Recover Deliberately

| Situation | Action |
|-----------|--------|
| Transient infra (OOM once, network blip) and task is idempotent | Retry, watch it |
| Task partially wrote and is not idempotent | Clean up output *first*, then rerun |
| Code bug | Fix forward, rerun from first failure, then rerun affected downstream |
| Bad upstream data | Block: quarantine/flag the input, get producer fix or backfill, then rerun |
| Output was wrong but run was green | Fix logic, then targeted backfill of affected partitions |

Backfills: state the exact partition range, verify idempotency (overwrite
semantics, not append), estimate cost/duration, run oldest-first, and
data-diff a repaired partition against expectations before declaring done.

## Wrap-Up Format

```
incident: dag=orders_daily run=2026-07-01 root cause found
what:     stg_payments failed 04:12Z; 3 downstream tasks failed, fct_orders stale
class:    upstream — payments API export dropped `currency`, task schema check (correctly) blocked
impact:   fct_orders + 2 dashboards stale ~6h; no bad data landed (blocked at staging)
fix:      producer restored column 09:30Z; reran stg_payments → downstream; all green
prevent:  currency covered by contract? (data-contracts-check) — filed JIRA-123
```

"No bad data landed" or "bad data reached X for Y hours" must appear
explicitly — it's the sentence stakeholders actually need.

## Rules

- Walk to the *first* failure; never debug a downstream casualty.
- Classify (code/data/infra/upstream) before touching anything.
- No blind retries: know why it failed, or know the task is idempotent and
  the failure transient — otherwise you're gambling with output state.
- Check for partial writes before every rerun.
- Announce early; update when the class changes; close with the wrap-up.
- A green rerun doesn't end the incident — verifying the *output* does.
- Every incident ends with one prevention item (check, contract, alert, or
  runbook line), else you'll triage the same thing next month.
