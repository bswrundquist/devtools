# Personal Claude Instructions

## Security Rules
- NEVER read, cat, display, or access `.env`, `.env.*`, or `.envrc` files
- NEVER run `gopass` or any secret store commands
- NEVER print, echo, or log environment variables or their values
- NEVER include secrets, tokens, or credentials in code, comments, or output
- If you need to reference an env var in code, use the variable name (e.g. `os.environ["DB_URL"]`) — never its value
- Secrets are managed via direnv + gopass — do not attempt to replicate or circumvent this setup
