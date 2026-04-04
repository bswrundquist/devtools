---
name: openapi
description: Use when designing REST APIs or writing OpenAPI 3.x specifications — paths, operations, parameters, request bodies, responses, components, security schemes, and API design principles.
tools: Read, Write, Edit, Bash
---

# OpenAPI

Design REST APIs and write OpenAPI 3.x specifications.

## Spec structure

```yaml
openapi: "3.1.0"
info:
  title: My API
  version: "1.0.0"
  description: |
    What this API does and who it's for.
  contact:
    email: api@example.com

servers:
  - url: https://api.example.com/v1
    description: Production
  - url: https://staging.api.example.com/v1
    description: Staging

paths:
  /users/{user_id}:
    get:
      summary: Get a user
      operationId: get_user
      tags: [users]
      parameters:
        - $ref: '#/components/parameters/UserId'
      responses:
        "200":
          description: User found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        "404":
          $ref: '#/components/responses/NotFound'
      security:
        - BearerAuth: []

components:
  schemas: {}
  parameters: {}
  responses: {}
  securitySchemes: {}
```

## Paths and operations

```yaml
paths:
  /users:
    get:
      summary: List users
      operationId: list_users
      tags: [users]
      parameters:
        - name: page
          in: query
          schema:
            type: integer
            minimum: 1
            default: 1
        - name: page_size
          in: query
          schema:
            type: integer
            minimum: 1
            maximum: 100
            default: 20
        - name: status
          in: query
          schema:
            type: string
            enum: [active, inactive, pending]
      responses:
        "200":
          description: Paginated list of users
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/UserPage'

    post:
      summary: Create a user
      operationId: create_user
      tags: [users]
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUser'
      responses:
        "201":
          description: User created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        "409":
          description: Email already exists
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
        "422":
          $ref: '#/components/responses/UnprocessableEntity'

  /users/{user_id}:
    parameters:
      - $ref: '#/components/parameters/UserId'

    get:
      summary: Get a user
      operationId: get_user
      tags: [users]
      responses:
        "200":
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        "404":
          $ref: '#/components/responses/NotFound'

    patch:
      summary: Update a user
      operationId: update_user
      tags: [users]
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/UpdateUser'
      responses:
        "200":
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        "404":
          $ref: '#/components/responses/NotFound'

    delete:
      summary: Delete a user
      operationId: delete_user
      tags: [users]
      responses:
        "204":
          description: Deleted
        "404":
          $ref: '#/components/responses/NotFound'
```

## Schemas (components)

```yaml
components:
  schemas:
    User:
      type: object
      required: [id, email, name, created_at]
      properties:
        id:
          type: string
          format: uuid
          readOnly: true
          example: "550e8400-e29b-41d4-a716-446655440000"
        email:
          type: string
          format: email
          example: "alice@example.com"
        name:
          type: string
          minLength: 1
          maxLength: 255
          example: "Alice Smith"
        role:
          type: string
          enum: [admin, member, viewer]
          default: member
        created_at:
          type: string
          format: date-time
          readOnly: true

    CreateUser:
      type: object
      required: [email, name]
      properties:
        email:
          type: string
          format: email
        name:
          type: string
          minLength: 1
          maxLength: 255
        role:
          type: string
          enum: [admin, member, viewer]
          default: member

    UpdateUser:
      type: object
      # All fields optional for PATCH
      properties:
        name:
          type: string
          minLength: 1
          maxLength: 255
        role:
          type: string
          enum: [admin, member, viewer]

    UserPage:
      type: object
      required: [items, total, page, page_size]
      properties:
        items:
          type: array
          items:
            $ref: '#/components/schemas/User'
        total:
          type: integer
          example: 142
        page:
          type: integer
          example: 1
        page_size:
          type: integer
          example: 20

    Error:
      type: object
      required: [code, message]
      properties:
        code:
          type: string
          example: "EMAIL_ALREADY_EXISTS"
        message:
          type: string
          example: "A user with this email already exists"
        details:
          type: object
          additionalProperties: true

  parameters:
    UserId:
      name: user_id
      in: path
      required: true
      schema:
        type: string
        format: uuid
      example: "550e8400-e29b-41d4-a716-446655440000"

  responses:
    NotFound:
      description: Resource not found
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/Error'
          example:
            code: NOT_FOUND
            message: "User not found"

    UnprocessableEntity:
      description: Validation error
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/Error'

  securitySchemes:
    BearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT

    ApiKeyAuth:
      type: apiKey
      in: header
      name: X-API-Key

    OAuth2:
      type: oauth2
      flows:
        authorizationCode:
          authorizationUrl: https://auth.example.com/authorize
          tokenUrl: https://auth.example.com/token
          scopes:
            read:users: Read user data
            write:users: Create and update users
```

## Security

Apply globally or per-operation:

```yaml
# Global — applies to all operations unless overridden
security:
  - BearerAuth: []

paths:
  /public/health:
    get:
      security: []   # Override: no auth required
      summary: Health check
```

## Discriminated unions (polymorphism)

```yaml
schemas:
  Animal:
    oneOf:
      - $ref: '#/components/schemas/Cat'
      - $ref: '#/components/schemas/Dog'
    discriminator:
      propertyName: type
      mapping:
        cat: '#/components/schemas/Cat'
        dog: '#/components/schemas/Dog'

  Cat:
    type: object
    required: [type, indoor]
    properties:
      type:
        type: string
        enum: [cat]
      indoor:
        type: boolean

  Dog:
    type: object
    required: [type, breed]
    properties:
      type:
        type: string
        enum: [dog]
      breed:
        type: string
```

## Common formats

| `format` | Meaning |
|---|---|
| `date-time` | ISO 8601: `2024-01-15T12:00:00Z` |
| `date` | `2024-01-15` |
| `uuid` | UUID v4 |
| `email` | Email address |
| `uri` | URI |
| `password` | Hint to UIs to mask the field |
| `binary` | File upload |
| `int64` | 64-bit integer |

## API design principles

### URLs
- Use nouns for resources, not verbs: `/users` not `/getUsers`
- Plural nouns for collections: `/users`, `/orders`
- Nest for ownership: `/users/{id}/orders` (not `/orders?user_id=...` unless filtering a top-level collection)
- Avoid deep nesting (max 2 levels): `/users/{id}/orders/{order_id}` not `/users/{id}/orders/{order_id}/items/{item_id}/reviews`

### HTTP methods
- `GET` — read only, idempotent, cacheable
- `POST` — create a resource, or trigger an action
- `PUT` — full replacement of a resource
- `PATCH` — partial update (fields can be omitted)
- `DELETE` — remove a resource

### Status codes
- `200` — success with body
- `201` — created (POST that creates)
- `204` — success without body (DELETE, some PUT/PATCH)
- `400` — bad request (malformed syntax)
- `401` — unauthenticated
- `403` — authenticated but unauthorized
- `404` — not found
- `409` — conflict (e.g., duplicate)
- `422` — validation error (valid syntax, invalid semantics)
- `429` — rate limited
- `500` — server error

### Naming conventions
- snake_case for field names in JSON bodies
- kebab-case for URL path segments
- SCREAMING_SNAKE_CASE for error codes
- `operationId` in camelCase or snake_case — be consistent

### Versioning
- URL versioning (`/v1/`) is simplest and most explicit
- Header versioning (`Accept: application/vnd.api+json;version=1`) is REST-purer but harder to test
- Don't version individual endpoints — version the whole API

## Tools

```bash
# Validate a spec
pip install openapi-spec-validator
openapi-spec-validator openapi.yaml

# Generate a HTML reference doc
npx @redocly/cli preview-docs openapi.yaml

# Generate client code
npx @openapitools/openapi-generator-cli generate \
  -i openapi.yaml -g python -o client/

# Lint for best practices
npx @redocly/cli lint openapi.yaml
```

## Rules

- Use `$ref` for any schema used more than once — never duplicate inline definitions.
- Every operation needs a unique `operationId` — code generators use it as the function name.
- Document every response status code your API actually returns.
- Include `example` values in schemas — they appear in docs and make the spec self-documenting.
- Separate request schemas (`CreateUser`) from response schemas (`User`) — they're rarely identical.
- `readOnly: true` on fields set by the server (id, created_at). `writeOnly: true` on password fields.
- Design for the consumer: what does a client developer need to know to use this correctly?
