---
name: pipeline-status
description: Use when the user wants a point-in-time status check of what's currently running — Airflow DAGs, Kubernetes workloads, long-running batch/Spark/BigQuery jobs, active alerts. A snapshot for "what's going on right now", not an incident investigation.
tools: Bash, Read, Grep, Glob
argument-hint: [airflow|k8s|jobs|all]
---

# Pipeline Status

A fast operational snapshot of what's running right now, across orchestration, compute, and batch jobs. This is a status board, not an investigation — if something in the snapshot looks broken, hand off to `/pipeline-triage` rather than digging in here.

## Arguments

`$ARGUMENTS` — optional scope filter: `airflow`, `k8s`, `jobs`, or `all` (default).

## Process

Check which CLIs are actually configured before running a section (`command -v airflow`, `command -v kubectl`, `command -v bq`/`gcloud`) — skip a section cleanly and say so if its CLI is missing, rather than failing the whole check.

### 1. Orchestration (Airflow)

```bash
airflow dags list-runs --state running -o table
airflow dags list-runs --state failed --start-date "$(date -v-1d +%F)" -o table   # last 24h
```

Flag anything running well past its usual duration, and any failed run in the last 24h that hasn't clearly been triaged already.

### 2. Kubernetes workloads

```bash
kubectl get pods --all-namespaces --field-selector=status.phase!=Running,status.phase!=Succeeded
kubectl get pods --all-namespaces | awk '$5 ~ /^[0-9]+$/ && $5+0 > 3 {print}'   # high restart counts
kubectl get jobs --all-namespaces -o wide
kubectl get cronjobs --all-namespaces
kubectl top pods --all-namespaces --sort-by=memory 2>/dev/null | head -10
```

Flag: pods stuck in `Pending`/`CrashLoopBackOff`/`ImagePullBackOff`, jobs not completed within a reasonable multiple of their usual runtime, cronjobs with no recent successful run.

### 3. Long-running / batch jobs

```bash
# BigQuery jobs currently running
bq ls -j -a --max_results=20 --format=prettyjson | grep -E '"state"|"jobId"|"startTime"'

# Spark-on-k8s, if the operator is installed
kubectl get sparkapplications --all-namespaces 2>/dev/null

# Vertex AI pipeline runs
gcloud ai pipeline-jobs list --region=<region> --filter="state=PIPELINE_STATE_RUNNING" 2>/dev/null
```

Flag anything running well past a normal duration for that job, or queued without progress.

### 4. Alerts

If Alertmanager/Grafana is reachable via CLI or a configured API endpoint, pull currently-firing alerts. Otherwise name the dashboard(s) the user should check instead of guessing at alert state.

## Output Format

```markdown
## Pipeline Status — <timestamp>

**Airflow:** 2 running (orders_daily on schedule, backfill_july ~40min in, normal ~35min) · 1 failed in last 24h (stg_payments, 04:12Z — not yet triaged)
**Kubernetes:** all pods healthy · cronjob `nightly-export` has no successful run since yesterday 22:00
**Batch/Spark/BQ:** 1 BQ job running 25min (job abc123, normal <5min — worth a look)
**Alerts:** none firing (Alertmanager) — or: not checked, no reachable endpoint configured

**Needs attention:**
- `stg_payments` Airflow failure — unresolved, consider `/pipeline-triage`
- BQ job abc123 running far past normal — check if stuck
```

## Rules

- This is a snapshot: state "all clear" explicitly for a healthy section — don't just omit it.
- Never guess DAG names, namespaces, or job IDs — only report what the CLI actually returned.
- Skip a section cleanly (and say so) when its CLI isn't installed/authenticated in this environment.
- Anything in "needs attention" gets a next step, usually `/pipeline-triage` for a genuine failure — this skill doesn't diagnose or fix.
- Keep it to one screen — meant to be read in ten seconds, not studied.
