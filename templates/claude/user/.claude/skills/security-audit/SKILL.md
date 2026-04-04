---
name: security-audit
description: Use when the user wants to scan their codebase for security vulnerabilities. Checks for OWASP top 10 patterns, hardcoded secrets, injection flaws, auth issues, and insecure configurations.
tools: Bash, Read, Grep, Glob, Agent
---

# Security Audit

Scan the codebase for common security vulnerabilities and misconfigurations.

## Process

1. **Scope** - Determine what to scan: entire repo, specific directories, or specific concern areas.
2. **Automated scans** - Run available tools (bandit, semgrep, trufflehog, etc.) if installed.
3. **Manual pattern scan** - Search for known vulnerability patterns across the codebase.
4. **Configuration review** - Check config files, Dockerfiles, CI configs for security issues.
5. **Report findings** - Organized by severity with specific file:line references and remediation.

## Vulnerability Patterns to Scan

### 1. Hardcoded Secrets
Search for:
```
# Patterns to grep for
password\s*=\s*["']
api_key\s*=\s*["']
secret\s*=\s*["']
token\s*=\s*["']
AWS_ACCESS_KEY
PRIVATE.KEY
-----BEGIN (RSA |EC |DSA )?PRIVATE KEY-----
```
Check: `.env` files committed, secrets in config files, API keys in source code.

### 2. SQL Injection
Search for:
```python
# Dangerous: string formatting in queries
f"SELECT * FROM users WHERE id = {user_id}"
"SELECT * FROM users WHERE id = " + user_id
cursor.execute("SELECT * FROM users WHERE id = %s" % user_id)

# Safe: parameterized queries
cursor.execute("SELECT * FROM users WHERE id = %s", (user_id,))
```

### 3. Command Injection
Search for:
```python
os.system(user_input)
subprocess.call(user_input, shell=True)
subprocess.Popen(cmd, shell=True)
eval(user_input)
exec(user_input)
```

### 4. Path Traversal
Search for:
```python
open(user_provided_path)
os.path.join(base, user_input)  # without validation
send_file(user_input)
```
Check: Is `../` blocked? Is the resolved path validated against an allowed base directory?

### 5. Insecure Deserialization
Search for:
```python
pickle.loads(untrusted_data)
yaml.load(data)  # should be yaml.safe_load
marshal.loads()
```

### 6. Authentication & Authorization
Check for:
- Endpoints without auth decorators/middleware
- JWT tokens without expiration
- Password storage without hashing (or weak hashing like MD5/SHA1)
- Missing CSRF protection on state-changing endpoints
- Session tokens in URLs

### 7. Sensitive Data Exposure
Check for:
- Logging of passwords, tokens, or PII
- Error messages that leak stack traces or internal paths
- Debug mode enabled in production configs
- Sensitive fields included in API responses (password hashes, internal IDs)

### 8. Insecure Configuration
Check:
- `DEBUG = True` in production settings
- `CORS: *` (overly permissive)
- `ALLOWED_HOSTS = ['*']`
- TLS/SSL verification disabled (`verify=False`)
- Default credentials in config files
- Dockerfile running as root without USER directive

### 9. Dependency Vulnerabilities
```bash
# Python
pip-audit 2>/dev/null || echo "pip-audit not installed"
safety check 2>/dev/null || echo "safety not installed"

# Node
npm audit 2>/dev/null
```

### 10. Docker & Infrastructure
- Base images using `:latest` tag
- Running as root in container
- Secrets passed as build args (visible in image history)
- Exposed ports that shouldn't be public
- `.dockerignore` missing `.env`, `.git`, credentials

## Automated Tools (Use If Available)

```bash
# Python security linter
bandit -r . -f json 2>/dev/null

# Secret scanner
trufflehog filesystem . --json 2>/dev/null
gitleaks detect --source . 2>/dev/null

# General pattern matching
semgrep --config auto . 2>/dev/null
```

If none are installed, rely on manual grep patterns and code review.

## Output Format

### 🔴 Critical
Exploitable vulnerabilities that could lead to data breach, RCE, or privilege escalation.

### 🟠 High
Vulnerabilities that are exploitable under certain conditions or with additional information.

### 🟡 Medium
Issues that weaken security posture but aren't directly exploitable.

### 🔵 Informational
Best practice deviations and hardening opportunities.

For each finding:
```
**[SEVERITY]** Category — Brief description
**File**: path/to/file.py:42
**Code**: `the vulnerable code`
**Risk**: What could an attacker do with this?
**Fix**: Specific remediation with code example
```

### Summary
- Total findings by severity
- Top 3 priorities to fix first
- Overall security posture assessment

## Rules

- Never log, print, or expose actual secret values found — just note their location.
- Differentiate between test/example code and production code. Hardcoded test credentials in test files are lower severity.
- Check `.gitignore` — if `.env` is not ignored, flag it as critical.
- Consider the deployment context: a local-only tool has different risk than a public-facing API.
- Don't flag commented-out code as vulnerable unless it could be uncommented.
- If no automated tools are installed, recommend the most relevant one to install and proceed with manual analysis.
