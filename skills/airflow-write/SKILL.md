---
name: airflow-write
description: Use when working with Apache Airflow — creating DAGs, writing tasks, configuring operators, debugging pipelines, or applying scheduling best practices. TaskFlow API first, targeting Airflow 3.x.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Apache Airflow

Write and debug Airflow DAGs — TaskFlow API by default, Airflow 3.x semantics.

## TaskFlow DAG (the default style)

Prefer `@dag`/`@task` over classic operators — data flows through return values (XCom) instead of manual `xcom_push`/`xcom_pull`. Use classic operators only when a provider operator does the whole job (e.g. `S3ToRedshiftOperator`).

```python
from __future__ import annotations

import pendulum
from airflow.sdk import dag, task  # Airflow 2.x: from airflow.decorators import dag, task


@dag(
    schedule="@daily",  # Airflow 2.x called this schedule_interval=
    start_date=pendulum.datetime(2026, 1, 1, tz="UTC"),
    catchup=False,
    default_args={"retries": 3, "retry_exponential_backoff": True,
                  "max_retry_delay": pendulum.duration(minutes=30)},
    tags=["etl"],
)
def daily_sales() -> None:
    @task
    def extract(logical_date: pendulum.DateTime | None = None) -> list[dict]:
        # Idempotent: read exactly this run's date partition, never "latest"
        return fetch_rows(day=logical_date.date())

    @task
    def transform(rows: list[dict]) -> dict:
        return {"total": sum(r["amount"] for r in rows)}

    @task
    def load(summary: dict) -> None:
        upsert_summary(summary)  # upsert keyed on the date — reruns don't duplicate

    load(transform(extract()))


daily_sales()
```

## Airflow 2.x vs 3.x — where it bites

| Concern | 2.x | 3.x |
|---|---|---|
| Schedule parameter | `schedule_interval=` | `schedule=` only |
| Data-aware scheduling | `Dataset` | `Asset` — `schedule=[Asset("s3://bucket/table")]` |
| Run date in task context | `execution_date` | `logical_date` (`execution_date` removed) |
| Decorator imports | `airflow.decorators` | `airflow.sdk` |
| Metadata DB from tasks | direct ORM access possible | blocked — tasks go through the API server |

## No top-level code

The scheduler re-parses every DAG file continuously (default ~every 30 s). Anything at module level executes on every parse.

```python
# BAD — runs on every scheduler parse loop
config = requests.get("https://api.example.com/config").json()

# GOOD — expensive work only at task runtime
@task
def process() -> None:
    engine = create_engine(os.environ["DB_URL"])
```

Keep heavy imports (`pandas`, `torch`) inside task functions too — they slow parsing for the whole deployment.

## Dynamic task mapping

Fan out over runtime data with `.expand()` — one mapped task instance per element.

```python
@task
def list_files() -> list[str]:
    return ["a.csv", "b.csv", "c.csv"]

@task
def process(path: str) -> int:
    return load_file(path)

process.expand(path=list_files())   # 3 mapped task instances at runtime
```

Mix fixed and mapped arguments with `process.partial(dest="warehouse").expand(path=files)`. Cap runaway fan-out with `max_active_tis_per_dag` on the task.

## Sensors: reschedule or deferrable

Never let a sensor occupy a worker slot while it sleeps.

```python
from airflow.providers.standard.sensors.filesystem import FileSensor

wait = FileSensor(
    task_id="wait_for_file",
    filepath="/data/{{ ds }}/input.csv",
    mode="reschedule",       # frees the worker slot between pokes
    poke_interval=300,
    timeout=60 * 60 * 6,
)
```

Prefer `deferrable=True` where the operator supports it — the wait moves to the triggerer and uses no worker slot at all.

## Config: params, Variables, Connections

| Mechanism | Use for | Access |
|---|---|---|
| `params` | Per-run knobs, overridable at trigger time | `params["country"]` from task context |
| Variables | Deployment-level config, small values | `Variable.get("key")` — inside tasks only |
| Connections | Credentials + hosts for external systems | Hooks: `PostgresHook(postgres_conn_id="warehouse")` |

```python
from airflow.sdk import Variable

@task
def run(params: dict | None = None) -> None:
    bucket = Variable.get("reports_bucket")  # runtime only — parse-time calls hit the DB every 30 s
    build_report(country=params["country"], bucket=bucket)
```

Define connections via a secrets backend or env vars (`AIRFLOW_CONN_WAREHOUSE`) — never hardcode credentials in DAG code.

## Testing DAGs

```python
from airflow.models import DagBag

def test_no_import_errors() -> None:
    dag_bag = DagBag(dag_folder="dags/", include_examples=False)
    assert dag_bag.import_errors == {}
    dag = dag_bag.get_dag("daily_sales")
    assert dag is not None and dag.catchup is False
```

Run a whole DAG locally without a scheduler:

```bash
uv run python -c "from dags.daily_sales import daily_sales; daily_sales().test()"
# or: airflow dags test daily_sales 2026-01-01
```

## Rules

- TaskFlow API (`@dag`/`@task`) by default; classic operators only when a provider operator does the entire job.
- Every task must be idempotent and keyed on `logical_date` — rerunning any task instance must produce identical state (upserts, partition overwrites, deterministic paths).
- No top-level code, network calls, DB access, or heavy imports in DAG files — they run every ~30 s at parse time.
- `catchup=False` unless backfill is explicitly wanted; a static, pinned `start_date` — never `datetime.now()`.
- Always set `retries` with `retry_exponential_backoff=True` and a `max_retry_delay` in `default_args`.
- Sensors: `mode="reschedule"` at minimum, `deferrable=True` when available. Never long-timeout `mode="poke"`.
- `Variable.get()` and `params` access belong inside task bodies, never at module level; never hardcode secrets — Connections/Variables come from a secrets backend.
- Keep `dag_id` and `task_id` stable — renaming orphans all run history.
- Put the DagBag import-error test in CI; it catches broken DAGs before the scheduler does.
