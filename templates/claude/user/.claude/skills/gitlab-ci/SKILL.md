---
name: gitlab-ci
description: Use when writing or debugging GitLab CI/CD pipelines — .gitlab-ci.yml structure, stages, rules, needs, caching, artifacts, environments, merge request pipelines, and Docker-based jobs.
tools: Read, Write, Edit, Bash, Glob
---

# GitLab CI/CD

Write and debug `.gitlab-ci.yml` pipelines.

## Basic structure

```yaml
# .gitlab-ci.yml

stages:
  - lint
  - test
  - build
  - deploy

default:
  image: python:3.12-slim
  before_script:
    - pip install uv
    - uv sync

variables:
  UV_SYSTEM_PYTHON: "1"
  PIP_CACHE_DIR: "$CI_PROJECT_DIR/.cache/pip"

lint:
  stage: lint
  script:
    - uv run ruff check .
    - uv run ruff format --check .

test:
  stage: test
  script:
    - uv run pytest --cov

build:
  stage: build
  script:
    - uv build
  artifacts:
    paths:
      - dist/
    expire_in: 1 week
```

## Variables

```yaml
variables:
  # Project-level defaults
  APP_ENV: staging
  DATABASE_URL: "postgresql://localhost/mydb"

job:
  variables:
    # Job-level override
    APP_ENV: test
  script:
    - echo $APP_ENV   # "test"
```

Secret variables are set in: Project → Settings → CI/CD → Variables.

Access in scripts:
```yaml
deploy:
  script:
    - echo $SECRET_KEY    # masked in logs if marked sensitive
    - ./deploy.sh
  environment: production
```

### Predefined variables (commonly used)

| Variable | Value |
|---|---|
| `$CI_COMMIT_SHA` | Full commit SHA |
| `$CI_COMMIT_SHORT_SHA` | First 8 chars of SHA |
| `$CI_COMMIT_REF_NAME` | Branch or tag name |
| `$CI_COMMIT_BRANCH` | Branch name (not set for tags) |
| `$CI_DEFAULT_BRANCH` | Default branch (usually `main`) |
| `$CI_PIPELINE_ID` | Pipeline ID |
| `$CI_MERGE_REQUEST_IID` | MR number (only in MR pipelines) |
| `$CI_PROJECT_NAME` | Project name |
| `$CI_REGISTRY` | GitLab container registry hostname |
| `$CI_REGISTRY_IMAGE` | Full registry path for this project |
| `$CI_REGISTRY_USER` | Registry auth username |
| `$CI_REGISTRY_PASSWORD` | Registry auth token |

## Rules

`rules:` replaces the deprecated `only:` / `except:`. It's evaluated top-to-bottom; first match wins.

```yaml
deploy-staging:
  script: ./deploy.sh staging
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
      when: on_success
    - when: never      # don't run otherwise

deploy-production:
  script: ./deploy.sh production
  rules:
    - if: $CI_COMMIT_TAG =~ /^v\d+\.\d+\.\d+$/   # version tags
      when: manual     # requires manual trigger
    - when: never

test:
  script: pytest
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
```

Common `if` conditions:
```yaml
if: $CI_PIPELINE_SOURCE == "merge_request_event"   # MR pipeline
if: $CI_PIPELINE_SOURCE == "schedule"               # scheduled pipeline
if: $CI_PIPELINE_SOURCE == "web"                    # manual trigger via UI
if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH         # main/master
if: $CI_COMMIT_TAG                                  # any tag
if: $CI_MERGE_REQUEST_TARGET_BRANCH_NAME == "main"  # MR targeting main
```

## Needs (DAG pipelines)

`needs:` lets jobs run as soon as their dependencies finish, ignoring stage order:

```yaml
stages: [build, test, deploy]

build-app:
  stage: build
  script: uv build
  artifacts:
    paths: [dist/]

build-docker:
  stage: build
  script: docker build .

test-unit:
  stage: test
  needs: [build-app]     # starts as soon as build-app finishes
  script: pytest tests/unit

test-integration:
  stage: test
  needs: [build-app, build-docker]  # waits for both
  script: pytest tests/integration

deploy:
  stage: deploy
  needs: [test-unit, test-integration]
  script: ./deploy.sh
```

## Cache

```yaml
default:
  cache:
    key:
      files:
        - uv.lock         # cache key based on lock file hash
    paths:
      - .cache/uv/
      - .venv/
    policy: pull-push     # pull at start, push at end (default)

# Pull-only for jobs that don't change deps
test:
  cache:
    policy: pull
  script: pytest
```

Cache scope:
```yaml
cache:
  key: "$CI_COMMIT_REF_SLUG"    # per branch
  key: "$CI_JOB_NAME"           # per job
  key: "global"                  # shared across all jobs/branches
```

## Artifacts

```yaml
build:
  script: uv build
  artifacts:
    paths:
      - dist/
      - reports/
    reports:
      junit: reports/junit.xml     # shows test results in MR UI
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
    expire_in: 30 days
    when: always                   # keep artifacts even on failure

test:
  needs:
    - job: build
      artifacts: true    # download artifacts from build job
  script:
    - ls dist/           # available from build job
```

## Environments and deployments

```yaml
deploy-staging:
  script: ./deploy.sh staging
  environment:
    name: staging
    url: https://staging.example.com
    on_stop: stop-staging    # job to call on "stop environment"

stop-staging:
  script: ./teardown.sh staging
  environment:
    name: staging
    action: stop
  when: manual
  rules:
    - if: $CI_MERGE_REQUEST_IID
      when: manual
```

## Docker in pipelines

### Build and push to GitLab registry

```yaml
build-image:
  image: docker:27
  services:
    - docker:27-dind
  variables:
    DOCKER_TLS_CERTDIR: "/certs"
  before_script:
    - docker login -u $CI_REGISTRY_USER -p $CI_REGISTRY_PASSWORD $CI_REGISTRY
  script:
    - docker build -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA .
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA
    - docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA $CI_REGISTRY_IMAGE:latest
    - docker push $CI_REGISTRY_IMAGE:latest
```

### Kaniko (no Docker daemon required)

```yaml
build-image:
  image:
    name: gcr.io/kaniko-project/executor:v1.23.0-debug
    entrypoint: [""]
  script:
    - /kaniko/executor
      --context $CI_PROJECT_DIR
      --dockerfile $CI_PROJECT_DIR/Dockerfile
      --destination $CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA
      --cache=true
```

## Include and extends

Split large configs:

```yaml
# .gitlab-ci.yml
include:
  - local: .gitlab/ci/test.yml
  - local: .gitlab/ci/deploy.yml
  - project: my-group/shared-ci
    ref: main
    file: /templates/python.yml
```

Reuse job templates:

```yaml
# Template (name starts with dot — not run directly)
.deploy-template:
  image: alpine:3.20
  before_script:
    - apk add --no-cache curl
  script:
    - ./deploy.sh $ENVIRONMENT

deploy-staging:
  extends: .deploy-template
  variables:
    ENVIRONMENT: staging
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

deploy-production:
  extends: .deploy-template
  variables:
    ENVIRONMENT: production
  rules:
    - if: $CI_COMMIT_TAG
      when: manual
```

## Merge request pipelines

Run pipelines on MR events (separate from branch pipelines):

```yaml
workflow:
  rules:
    - if: $CI_MERGE_REQUEST_IID      # MR pipeline
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH   # main branch pipeline
    - if: $CI_COMMIT_TAG             # tag pipeline

# Now use rules in jobs:
test:
  script: pytest
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
```

## Common full pipeline

```yaml
stages: [lint, test, build, deploy]

default:
  image: python:3.12-slim
  interruptible: true   # cancel superseded pipelines

workflow:
  rules:
    - if: $CI_MERGE_REQUEST_IID
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
    - if: $CI_COMMIT_TAG

variables:
  PIP_CACHE_DIR: "$CI_PROJECT_DIR/.cache/pip"

.python-setup:
  before_script:
    - pip install uv
    - uv sync --frozen

lint:
  extends: .python-setup
  stage: lint
  script:
    - uv run ruff check .
    - uv run mypy src/
  cache:
    key: { files: [uv.lock] }
    paths: [.cache/pip, .venv]
    policy: pull

test:
  extends: .python-setup
  stage: test
  script:
    - uv run pytest --cov --cov-report=xml --junitxml=report.xml
  coverage: '/TOTAL.*\s+(\d+%)$/'
  artifacts:
    reports:
      junit: report.xml
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
  cache:
    key: { files: [uv.lock] }
    paths: [.cache/pip, .venv]
    policy: pull

build:
  extends: .python-setup
  stage: build
  script: uv build
  artifacts:
    paths: [dist/]
    expire_in: 1 week
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
    - if: $CI_COMMIT_TAG

deploy-staging:
  stage: deploy
  needs: [test, build]
  script: ./deploy.sh staging
  environment:
    name: staging
    url: https://staging.example.com
  rules:
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

deploy-production:
  stage: deploy
  needs: [test, build]
  script: ./deploy.sh production
  environment:
    name: production
    url: https://example.com
  when: manual
  rules:
    - if: $CI_COMMIT_TAG =~ /^v\d+/
```

## Rules

- Use `rules:` not `only:`/`except:` — the latter is deprecated.
- Always set `interruptible: true` on jobs that should be cancelled when a newer pipeline runs for the same branch.
- Set `expire_in` on artifacts — they consume storage indefinitely otherwise.
- Use `needs:` to run jobs in parallel as soon as their dependencies finish; don't rely on stage ordering for performance.
- Set `policy: pull` on cache for jobs that don't install new dependencies — they don't need to push.
- Don't hardcode secrets in `.gitlab-ci.yml`. Use CI/CD variables with the "Masked" option.
- Pin image tags: `image: python:3.12-slim` not `python:latest`.
