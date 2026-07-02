---
name: vertex-pipelines-write
description: Use when building or debugging Vertex AI Pipelines with KFP v2 — writing components and pipeline DSL, parameters vs artifacts, caching behavior, resource specs, compiling and submitting jobs, scheduling, and diagnosing failed runs.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Vertex Pipelines Write

Components are plain Python functions that must stay testable without any
pipeline machinery; the DSL only wires them. Logic in the DAG is logic you
can't unit test.

## Components

```python
from kfp import dsl

@dsl.component(
    base_image="python:3.12-slim",
    packages_to_install=["polars==1.9.0", "google-cloud-bigquery==3.25.0"],  # pinned
)
def transform(
    src_table: str,
    out_data: dsl.Output[dsl.Dataset],
    min_date: str = "2020-01-01",
) -> None:
    import polars as pl            # imports INSIDE the function — it runs in its own container
    ...
    df.write_parquet(out_data.path)
```

- Pin `packages_to_install` exactly, or builds drift and caching lies.
- Keep the decorated function a thin shell: parse inputs, call a regular
  function from your package, write outputs. The regular function gets unit
  tests (pytest-write skill).
- Components with heavy shared deps → one custom `base_image` built in CI
  instead of 40-line install lists per component.

## Parameters vs Artifacts

| | Parameters | Artifacts |
|---|---|---|
| What | str/int/float/bool/list/dict | Files/dirs: `Dataset`, `Model`, `Metrics` |
| Passed as | values in the DAG spec | GCS paths under `pipeline_root` |
| Use for | config, table names, dates, flags | anything with size or lineage |

Never pass data *contents* as parameters, and never smuggle GCS paths as
string parameters when an artifact fits — artifacts get lineage, metadata,
and UI visibility; strings get nothing.

## Pipeline

```python
@dsl.pipeline(name="churn-training")
def pipeline(src_table: str, min_date: str):
    t = transform(src_table=src_table, min_date=min_date)
    t.set_cpu_limit("4").set_memory_limit("16G")

    train = train_model(data=t.outputs["out_data"])
    train.set_accelerator_type("NVIDIA_TESLA_T4").set_accelerator_limit(1)

    with dsl.If(train.outputs["auc"] > 0.8):     # v2 condition
        register(model=train.outputs["model"])
```

Set resources per task — defaults are small, and an OOM in a component
surfaces as an unhelpful task crash. Data-dependent branching belongs in
`dsl.If`/`dsl.ParallelFor`, not Python `if` (Python control flow runs at
*compile* time, on placeholder objects — a classic silent wrong-DAG bug).

## Caching — Decide, Don't Discover

Cache hit = same component spec + image + inputs. Two traps:

- **Stale hits**: image tag unchanged (`:latest`) but contents changed, or a
  component reads a BQ table — the table isn't an input, so yesterday's
  result is "valid". Anything reading external state: disable its cache
  (`task.set_caching_options(False)`) or pass a date/snapshot parameter that
  changes when the data does.
- **Surprise misses**: timestamps or `uuid` in parameter defaults kill
  caching pipeline-wide.

Rule: every task is either deterministic-in-its-inputs (cacheable) or
explicitly uncached.

## Compile, Run, Schedule

```python
from kfp import compiler
from google.cloud import aiplatform

compiler.Compiler().compile(pipeline, "pipeline.yaml")   # commit this? no — build in CI

aiplatform.init(project=PROJECT, location="us-central1")
job = aiplatform.PipelineJob(
    display_name="churn-training",
    template_path="pipeline.yaml",
    pipeline_root=f"gs://{BUCKET}/pipeline-root",
    parameter_values={"src_table": "...", "min_date": "2026-01-01"},
)
job.submit(service_account=PIPELINE_SA)      # dedicated SA, least privilege
```

- Always pass an explicit `service_account` — the default compute SA either
  has too much access or mysteriously lacks BQ/GCS permissions.
- Recurring runs: `aiplatform.PipelineJobSchedule` (cron) — parameterize by
  logical date, don't compute `today()` inside components (breaks reruns and
  caching).
- CI: compile on every PR (compilation catches wiring/type errors), submit
  a smoke run on a tiny dataset before promoting the template.

## Debugging a Failed Run

1. UI → failed task → **Logs** (Cloud Logging, scoped to the task). The real
   error is usually the last Python traceback, above KFP teardown noise.
2. Task never started, "internal error" or pending forever → quota
   (GPUs/CPUs in region) or SA permissions (`iam.serviceAccounts.actAs`,
   GCS access to `pipeline_root`).
3. Exit code 137 / "task terminated" → OOM: raise `set_memory_limit`, check
   input size growth first (profile-dataset).
4. Wrong results with green run → check for a stale cache hit before
   blaming the code: was this task cached from before the fix/data change?
5. Reproduce outside the pipeline: run the component's inner function
   locally on the same inputs — if it fails locally, debug there, not
   through 10-minute pipeline iterations.

## Rules

- Component = thin shell over a unit-tested plain function; imports inside;
  packages pinned.
- Artifacts for data, parameters for config — no GCS-paths-as-strings.
- Python control flow in a pipeline body is a compile-time bug; use
  `dsl.If`/`dsl.ParallelFor`.
- Every task has explicit resources and an explicit caching decision;
  external-state readers are never cached.
- Explicit least-privilege service account per pipeline; never the default
  compute SA.
- Logical dates come in as parameters — components never call `now()`.
- Compile in CI; smoke-run before promoting; debug component logic locally,
  not by resubmitting pipelines.
