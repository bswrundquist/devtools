---
name: terraform-write
description: Use when writing or refactoring Terraform — modules, variables, state moves, imports, provider config — or reviewing a plan before apply. Covers safe refactoring with moved blocks, for_each patterns, and reading plans for hidden destroys.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# Terraform Write

The plan is the contract: every change is written so the plan is small,
readable, and free of surprise destroys.

## Layout

```
modules/<name>/        reusable: main.tf, variables.tf, outputs.tf, README.md
envs/<env>/            root modules per environment — compose modules, hold backend config
```

- Root modules stay thin: wire modules together, no resource sprawl.
- One state per env per domain. A plan that takes minutes or touches hundreds
  of resources means the state is too big — split it.
- Pin everything: `required_version`, provider versions (`~> 5.0`), module
  sources by tag/ref. Unpinned Terraform drifts into surprise plans.

## Variables and Outputs

```hcl
variable "instances" {
  description = "Instances to create, keyed by logical name."
  type = map(object({
    machine_type = string
    zone         = optional(string, "us-central1-a")
  }))
  validation {
    condition     = alltrue([for k, v in var.instances : can(regex("^[a-z][a-z0-9-]*$", k))])
    error_message = "Instance keys must be lowercase kebab-case."
  }
}
```

- Every variable: `type` and `description`. Validation for anything with
  format rules — fail at plan, not at apply.
- No `default` on things that differ per environment; force the caller.
- Outputs only for what other configs/humans consume. `sensitive = true`
  where applicable.

## for_each, not count

```hcl
resource "google_storage_bucket" "this" {
  for_each = var.buckets          # map keyed by stable logical name
  name     = each.value.name
  ...
}
```

`count` indexes by position — removing the first element rebuilds everything
after it. `for_each` keys by name and survives reordering. Only use `count`
for the boolean-existence trick (`count = var.enabled ? 1 : 0`).

## Refactoring Without Destroys

Renames and moves are `moved` blocks — declarative, reviewable, no state
surgery:

```hcl
moved {
  from = google_sql_database_instance.db
  to   = module.database.google_sql_database_instance.this
}
```

Adopting existing infrastructure is an `import` block (1.5+), which shows up
in the plan like any other change:

```hcl
import {
  to = google_storage_bucket.legacy
  id = "my-project/legacy-bucket"
}
```

`terraform state mv/rm` only when blocks can't express it — and then paired
with an immediate plan showing no changes.

## Reading a Plan

Before any apply, scan for:

- **`-/+` (replace)** — the killer. The plan says which attribute forces it
  (`# forces replacement`). Decide: is the replacement acceptable, avoidable
  (e.g. `create_before_destroy`), or a mistake?
- **destroys you didn't intend** — a removed module call, a changed `for_each`
  key, a renamed resource missing its `moved` block.
- **perpetual diffs** — the same attribute changing every plan means provider
  drift or a value computed at apply; fix with `ignore_changes` only as a
  documented last resort.
- Plan output too big to review is itself a finding: split the change.

## Commands

```bash
terraform fmt -recursive && terraform validate
terraform plan -out=tfplan && terraform show tfplan      # review saved plan, apply exactly it
terraform apply tfplan
terraform state list | grep <resource>
terraform plan -refresh-only                             # detect drift without proposing changes
```

## Rules

- Never hand-edit state. `moved`/`import` blocks first, `state mv` second,
  raw edits never.
- Never `-target` in routine work — it's for incident recovery, and each use
  is followed by a full clean plan.
- Apply saved plans (`plan -out` → `apply tfplan`), at least in CI — what was
  reviewed is what runs.
- Every `-/+` replace in a plan is called out and justified before apply.
- No secrets in `.tf` or `.tfvars` — reference a secret manager; state is
  sensitive too, so its backend is encrypted and access-controlled.
- `for_each` keys are stable logical names, never list indexes or values that
  change (that's a rebuild).
- Module interfaces are boring: typed variables in, outputs out, no
  provider config inside reusable modules.
- `ignore_changes` entries carry a comment saying who changes the attribute
  and why that's OK.
