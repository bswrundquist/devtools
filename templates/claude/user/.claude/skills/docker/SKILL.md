---
name: docker
description: Use when working with Docker — writing Dockerfiles, debugging containers, optimizing builds, Docker Compose, or GPU container patterns. Covers multi-stage builds, layer caching, and security best practices.
tools: Bash, Read, Edit, Write, Grep, Glob
---

# Docker

Write efficient, secure, and well-structured Dockerfiles and container configurations.

## Dockerfile Best Practices

### Layer Ordering (Cache Optimization)
Order layers from least to most frequently changing:
```dockerfile
# 1. Base image (rarely changes)
FROM python:3.12-slim

# 2. System deps (changes occasionally)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 3. Python deps (changes when requirements change)
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# 4. Application code (changes frequently)
COPY . .
```

### Multi-Stage Builds
Use multi-stage builds to separate build-time from runtime:
```dockerfile
# Build stage
FROM python:3.12 AS builder
WORKDIR /build
COPY requirements.txt .
RUN pip install --no-cache-dir --prefix=/install -r requirements.txt

# Runtime stage
FROM python:3.12-slim
COPY --from=builder /install /usr/local
COPY . /app
WORKDIR /app
CMD ["python", "main.py"]
```

### Security
```dockerfile
# Create non-root user
RUN groupadd -r app && useradd -r -g app app

# Set ownership and switch user
COPY --chown=app:app . /app
USER app

# Use specific image tags, not :latest
FROM python:3.12.4-slim-bookworm
```

### Size Reduction
- Use `-slim` or `-alpine` base images when possible
- Combine `RUN` commands with `&&` to reduce layers
- Use `--no-install-recommends` with apt-get
- Clean up caches: `rm -rf /var/lib/apt/lists/*`, `pip install --no-cache-dir`
- Use `.dockerignore` to exclude unnecessary files

## GPU Container Patterns

### NVIDIA CUDA Base
```dockerfile
FROM nvidia/cuda:12.4.0-runtime-ubuntu22.04

# Install Python in CUDA image
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 python3-pip \
    && rm -rf /var/lib/apt/lists/*
```

### vLLM / ML Serving
```dockerfile
FROM vllm/vllm-openai:latest

# Or build custom with specific CUDA version
FROM nvidia/cuda:12.4.0-devel-ubuntu22.04 AS builder
# ... build wheels ...

FROM nvidia/cuda:12.4.0-runtime-ubuntu22.04
# ... copy wheels, install runtime only ...
```

### GPU Runtime Configuration
```yaml
# docker-compose.yml
services:
  model:
    image: vllm-serve
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    environment:
      - NVIDIA_VISIBLE_DEVICES=all
```

## Docker Compose Patterns

### Development Setup
```yaml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
      target: development  # Multi-stage target
    volumes:
      - .:/app            # Mount source for hot reload
      - /app/node_modules  # Exclude deps from mount
    ports:
      - "8000:8000"
    env_file: .env
```

### Health Checks
```yaml
services:
  api:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

## Debugging Containers

```bash
# Run interactive shell in a new container
docker run -it --rm <image> /bin/bash

# Exec into running container
docker exec -it <container> /bin/bash

# View logs
docker logs -f --tail 100 <container>

# Inspect container details
docker inspect <container>

# Check resource usage
docker stats <container>

# Build with progress output
docker build --progress=plain -t <tag> .
```

## .dockerignore
```
.git
.github
.env
*.pyc
__pycache__
.venv
.mypy_cache
.pytest_cache
tests/
docs/
*.md
Makefile
```

## Rules

- Always use specific image tags, never `:latest` in production Dockerfiles.
- Order layers for maximum cache efficiency.
- Never store secrets in images — use build args, env vars, or mounted secrets.
- Use `.dockerignore` to keep build context small.
- Run as non-root user in production images.
- Use `HEALTHCHECK` for production services.
- Prefer `COPY` over `ADD` unless you need URL fetching or tar extraction.
- Use `ENTRYPOINT` for the main command and `CMD` for default arguments.
- Pin package versions in `apt-get install` for reproducibility when stability matters.
