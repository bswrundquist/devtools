---
name: openapi-design
description: Use when designing a new REST API, writing openapi.yaml/json specifications, choosing HTTP methods and status codes, or defining reusable components and schemas.
tools: Bash, Read, Write, Edit, Grep, Glob
---

# OpenAPI

Design REST APIs and write OpenAPI 3.1 specifications that codegen, docs tooling, and linters can all consume.

## Resource Naming

Plural nouns, no verbs in paths. Actions that don't map to CRUD become a sub-resource POST.

| Bad | Good |
|-----|------|
| `GET /getUser/{id}` | `GET /users/{id}` |
| `POST /createOrder` | `POST /orders` |
| `POST /orders/{id}/doCancel` | `POST /orders/{id}/cancel` |

## Methods and Status Codes

| Method | Meaning | Success | Typical errors |
|--------|---------|---------|----------------|
| GET (collection) | List | 200 | 401, 403 |
| GET (item) | Fetch | 200 | 404 |
| POST | Create / action | 201 + `Location` header | 400, 401, 409, 422 |
| PUT | Full replace | 200 | 404, 409, 422 |
| PATCH | Partial update | 200 | 404, 422 |
| DELETE | Remove | 204 (no body) | 404 |

Error semantics: 400 malformed request (unparseable JSON), 401 not authenticated, 403 authenticated but not allowed, 404 not found, 409 state conflict (duplicate, stale version), 422 well-formed but semantically invalid.

## Spec Skeleton

Request and response schemas are separate: clients must not send server-set fields.

```yaml
openapi: 3.1.0
info:
  title: Orders API
  version: 1.0.0
paths:
  /orders:
    post:
      operationId: createOrder
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/OrderCreate"
      responses:
        "201":
          description: Order created
          headers:
            Location: {schema: {type: string}}
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Order"
              example: {id: "6f1c…", status: pending, items: [{sku: "A1", qty: 2}]}
        "422":
          $ref: "#/components/responses/ValidationError"
components:
  schemas:
    OrderCreate:        # request — no server-set fields
      type: object
      required: [items]
      properties:
        items: {type: array, items: {$ref: "#/components/schemas/OrderItem"}}
    Order:              # response — server-set fields added
      allOf:
        - $ref: "#/components/schemas/OrderCreate"
        - type: object
          required: [id, status, created_at]
          properties:
            id: {type: string, format: uuid}
            status: {type: string, enum: [pending, paid, cancelled]}
            created_at: {type: string, format: date-time}
```

## Pagination

Cursor-based, one envelope for every list endpoint. Cursors are opaque — never expose offsets on large tables.

```yaml
parameters:
  Cursor: {name: cursor, in: query, schema: {type: string}}
  Limit: {name: limit, in: query, schema: {type: integer, default: 50, maximum: 200}}
schemas:
  OrderPage:
    type: object
    required: [data, next_cursor, has_more]
    properties:
      data: {type: array, items: {$ref: "#/components/schemas/Order"}}
      next_cursor: {type: [string, "null"], description: "Opaque; pass back as ?cursor="}
      has_more: {type: boolean}
```

## Errors — RFC 9457 problem+json

One error shape for the whole API, content type `application/problem+json`:

```yaml
schemas:
  Problem:
    type: object
    required: [title, status]
    properties:
      type: {type: string, format: uri, default: "about:blank"}
      title: {type: string}
      status: {type: integer}
      detail: {type: string}
      instance: {type: string}
responses:
  ValidationError:
    description: Request failed validation
    content:
      application/problem+json:
        schema:
          $ref: "#/components/schemas/Problem"
```

For field-level details, extend `Problem` via `allOf` with an `errors: [{field, message}]` array.

## Components Organization

- `schemas` — data shapes; suffix request variants (`OrderCreate`, `OrderUpdate`) and keep the bare noun for the read model.
- `responses` — reusable errors: `Unauthorized`, `Forbidden`, `NotFound`, `ValidationError`.
- `parameters` — `Cursor`, `Limit`, shared path params.
- `securitySchemes` — declare once, apply globally with top-level `security`.

## Versioning

| Approach | Example | Verdict |
|----------|---------|---------|
| URL path, major only | `/v1/orders` | Default choice — visible, cacheable, easy to route |
| Header | `Accept: application/vnd.api.v2+json` | Cleaner URLs, harder to test and debug |
| Additive-only, no bump | New optional fields only | Best long-term policy; pair with `/v1` as escape hatch |

## Validation and Linting

Run one of these in CI and treat warnings as errors:

```bash
npx @stoplight/spectral-cli lint openapi.yaml
npx @redocly/cli lint openapi.yaml
```

## Rules

- Use `$ref` for any schema used more than once — never copy-paste inline schemas.
- Separate request schemas from response schemas; `allOf` keeps them DRY.
- Every operation gets a unique camelCase `operationId` — codegen names client methods from it.
- Plural nouns, no verbs in paths; non-CRUD actions are sub-resource POSTs.
- 422 for semantic validation failures, 400 for malformed requests. Never return 200 with an error body.
- All errors are `application/problem+json` referencing the shared `Problem` schema.
- Pick one field-name convention (snake_case) and enforce it everywhere; every schema and response gets at least one `example`.
- Lint the spec in CI with spectral or redocly; a spec that doesn't lint doesn't merge.
