---
name: data-contracts-check
description: Use when defining a data contract between a producer and consumers, checking a dataset or schema change against an existing contract, or deciding whether a change is breaking. Covers contract contents, versioning, and CI enforcement.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Data Contracts Check

A contract is an interface, enforced where the producer changes things — not
a wiki page consumers discover after the incident.

## What a Contract Contains

```yaml
# contracts/orders_events.yaml
dataset: analytics.orders_events
owner: team-checkout            # who consumers page
version: 2.1.0
grain: one row per order status change (order_id, changed_at)
sla:
  freshness: 2h                 # max lag, load-to-available
  availability_by: "07:00 UTC"  # for batch consumers
schema:
  - {name: order_id,   type: string, nullable: false}
  - {name: status,     type: string, nullable: false,
     enum: [placed, paid, shipped, returned]}
  - {name: amount_usd, type: numeric, nullable: false, checks: ["≥ 0"]}
  - {name: changed_at, type: timestamp, nullable: false}
consumers:
  - team-finance (revenue reporting)
  - team-ml (churn features)
```

Grain and semantics matter more than column types — a type mismatch fails
loudly on its own; a silent grain change (suddenly two rows per status)
corrupts every downstream aggregate while every check stays green.

## Breaking or Not

| Change | Verdict |
|--------|---------|
| Add nullable column | Non-breaking |
| Add enum value | **Breaking** — consumer `case` statements silently drop it |
| Widen type (int → numeric) | Usually non-breaking; check consumer tooling |
| Rename / drop / narrow column, tighten nullability | Breaking |
| Change grain or dedup logic | Breaking, and the worst kind — announce loudly |
| Freshness SLA loosened | Breaking for schedule-dependent consumers |

Breaking path: bump major version → notify listed consumers → parallel-run
old and new (`orders_events_v3` alongside) → migrate consumers → deprecate on
a stated date. Never break in place.

## Checking Against the Contract

Producer CI — diff the proposed schema against the contract before merge.
With dbt, the contract lives in the model YAML and `dbt build` enforces it:

```yaml
models:
  - name: orders_events
    config:
      contract: {enforced: true}    # build fails on schema mismatch
    columns:
      - name: order_id
        data_type: string
        constraints: [{type: not_null}]
```

Runtime — the SLA and semantic clauses become data quality checks
(data-quality-checks skill) tagged with the contract version: freshness ≤ 2h,
PK unique on the grain, enum membership on `status`.

Checking an actual dataset against a contract file, report per clause:
**pass / fail / not-checkable**, with failing rows sampled. Not-checkable
clauses (undocumented semantics, missing loaded_at) are findings too — an
uncheckable contract is a wish.

## Adopting Contracts on an Existing Pipeline

1. Write the contract for what the data **actually is today** (profile it
   first — data-profile skill), not what it should be.
2. Enforce in producer CI as warn-only; watch a week of violations.
3. Fix or renegotiate the violating clauses, then flip to enforcing.
4. Only then tighten toward what consumers actually need.

Starting from aspiration instead of reality produces a contract that's red
from day one and gets ignored forever.

## Rules

- Every contract names an owner and its consumers — a contract nobody is
  accountable for is documentation, and stale documentation at that.
- Grain is stated explicitly in every contract. Grain changes are always
  breaking.
- Enforce at the producer's CI, monitor at runtime — consumers should never
  be the detection mechanism.
- Adding an enum value is breaking. The consumer's `else` branch disagrees
  with your intuition.
- Breaking changes ship as new versioned datasets with parallel-run;
  in-place mutation of a contracted dataset is an incident, not a change.
- Contract files live in the producer's repo, versioned with the code that
  fulfills them.
- Check reports distinguish fail from not-checkable — and treat both as
  work items.
