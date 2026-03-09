---
name: pulumi
description: Use when working with Pulumi infrastructure as code — creating stacks, defining resources, configuring cloud infrastructure, debugging deployments, or discussing Pulumi best practices. Supports Python SDK primarily.
tools: Bash, Read, Edit, Write, Grep, Glob
---

# Pulumi

Write and manage infrastructure as code using Pulumi with the Python SDK.

## Core Principles

- **Real code over config** — Use Python's full power (loops, conditionals, functions, classes) to define infrastructure. Avoid copy-paste.
- **Strongly typed** — Use type hints for inputs, outputs, and component args.
- **Explicit over implicit** — Name resources explicitly. Don't rely on auto-naming for resources that need stable names (DNS, storage buckets).
- **Stack isolation** — Each stack (dev, staging, prod) should be independently deployable with its own config.

## Resource Patterns

### Naming Convention
```python
import pulumi

project = pulumi.get_project()
stack = pulumi.get_stack()
prefix = f"{project}-{stack}"

bucket = gcp.storage.Bucket(f"{prefix}-assets", ...)
```

### Component Resources
Group related resources into reusable ComponentResource classes:
```python
class VllmService(pulumi.ComponentResource):
    def __init__(self, name: str, args: VllmServiceArgs, opts=None):
        super().__init__("custom:service:VllmService", name, {}, opts)
        # Define child resources with parent=self
```

### Configuration & Secrets
```python
config = pulumi.Config()
model_name = config.require("model_name")
api_key = config.require_secret("api_key")  # encrypted in state
```

Set config via CLI:
```bash
pulumi config set model_name "meta-llama/Llama-3-8B"
pulumi config set --secret api_key "sk-..."
```

### Outputs & Stack References
```python
# Export outputs
pulumi.export("service_url", service.url)

# Reference from another stack
other = pulumi.StackReference("org/project/prod")
prod_url = other.get_output("service_url")
```

## GCP Patterns (Cloud Run, GPU)

### Cloud Run with GPU
```python
service = gcp.cloudrunv2.Service(
    name,
    template=gcp.cloudrunv2.ServiceTemplateArgs(
        containers=[gcp.cloudrunv2.ServiceTemplateContainerArgs(
            image=image,
            resources=gcp.cloudrunv2.ServiceTemplateContainerResourcesArgs(
                limits={"cpu": "8", "memory": "32Gi", "nvidia.com/gpu": "1"},
            ),
        )],
        node_selector=gcp.cloudrunv2.ServiceTemplateNodeSelectorArgs(
            accelerator="nvidia-l4",
        ),
        scaling=gcp.cloudrunv2.ServiceTemplateScalingArgs(
            min_instance_count=0,
            max_instance_count=1,
        ),
        timeout="900s",
    ),
)
```

### IAM Bindings
```python
# Prefer authoritative bindings for controlled resources
binding = gcp.cloudrunv2.ServiceIamBinding(
    f"{name}-invoker",
    name=service.name,
    location=service.location,
    role="roles/run.invoker",
    members=["allUsers"],
)
```

## Project Structure
```
infra/
├── __main__.py          # Entry point, orchestrates components
├── Pulumi.yaml          # Project definition
├── Pulumi.dev.yaml      # Dev stack config
├── Pulumi.prod.yaml     # Prod stack config
├── components/          # Reusable ComponentResource classes
│   ├── service.py
│   └── network.py
└── requirements.txt     # pulumi, pulumi-gcp, etc.
```

## Commands Reference
```bash
pulumi up              # Preview and deploy
pulumi preview         # Preview only (dry run)
pulumi stack ls        # List stacks
pulumi stack select    # Switch stacks
pulumi config set      # Set config value
pulumi destroy         # Tear down resources
pulumi refresh         # Sync state with cloud
pulumi import          # Import existing resources
```

## Rules

- Always use `pulumi preview` before `pulumi up` to review changes.
- Never hardcode secrets — use `pulumi config set --secret`.
- Use `opts=pulumi.ResourceOptions(parent=self)` for child resources in components.
- Use `opts=pulumi.ResourceOptions(depends_on=[...])` only when Pulumi can't infer dependencies automatically.
- Use `protect=True` for stateful resources (databases, storage buckets) to prevent accidental deletion.
- Use `delete_before_replace=True` for resources that can't have duplicate names.
- Use `ignore_changes` sparingly — it hides drift.
- Tag all resources with project, stack, and environment.
