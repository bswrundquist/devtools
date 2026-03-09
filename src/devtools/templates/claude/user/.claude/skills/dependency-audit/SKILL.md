---
name: dependency-audit
description: Use when the user wants to audit project dependencies for security vulnerabilities, outdated packages, license issues, or unused dependencies. Scans requirements.txt, pyproject.toml, or package.json.
tools: Bash, Read, Grep, Glob, Agent
---

# Dependency Audit

Analyze project dependencies for security, freshness, licensing, and bloat.

## Process

1. **Find dependency files** - Look for `requirements.txt`, `pyproject.toml`, `setup.py`, `setup.cfg`, `Pipfile`, `package.json`, `poetry.lock`, `uv.lock`.
2. **Security scan** - Run `pip-audit` (Python) or `npm audit` (Node) if available. If not installed, check PyPI/npm advisory databases manually using web search.
3. **Freshness check** - For each dependency, check the installed version against the latest available version. Flag packages more than 1 major version behind.
4. **License check** - Identify dependency licenses. Flag any copyleft (GPL, AGPL) or unknown licenses that might conflict with the project's license.
5. **Unused dependency detection** - Cross-reference declared dependencies with actual imports in the codebase. Flag packages that are declared but never imported.
6. **Report findings** - Organized by category with actionable recommendations.

## Security

### What to flag
- Dependencies with known CVEs (Critical and High severity)
- Packages that are abandoned (no release in 2+ years, archived repo)
- Packages with very low download counts (possible typosquatting)
- Pinned versions with known vulnerabilities
- Missing hash verification in requirements files

### Tools to try (in order)
```bash
# Python
pip-audit                          # Best option
safety check                      # Alternative
pip list --outdated                # Freshness only

# Node
npm audit
npx auditjs
```

## Freshness

Categorize each dependency:
- **Current** — Latest or within one minor version
- **Behind** — One or more minor versions behind
- **Outdated** — One or more major versions behind
- **Abandoned** — No release in 2+ years

## License Compatibility

Flag these license types:
- 🔴 **GPL/AGPL** — Copyleft, may require your code to be open-sourced
- 🟡 **LGPL** — Copyleft for modifications to the library itself
- 🟢 **MIT/BSD/Apache/ISC** — Permissive, generally safe
- 🔴 **Unknown/None** — No license declared, legally risky

## Unused Dependencies

```bash
# Check what's actually imported in the codebase
# Compare against what's declared in requirements
```

Cross-reference:
1. Parse all `import X` and `from X import` statements in the codebase
2. Map import names to package names (they often differ: `PIL` → `Pillow`, `cv2` → `opencv-python`)
3. Flag declared packages with no matching imports

## Output Format

### 🔴 Security Vulnerabilities
| Package | Version | CVE | Severity | Fixed In |
|---------|---------|-----|----------|----------|

### 🟡 Outdated Packages
| Package | Current | Latest | Behind By |
|---------|---------|--------|-----------|

### ⚖️ License Concerns
| Package | License | Risk |
|---------|---------|------|

### 🗑️ Potentially Unused
| Package | Declared In | Reason |
|---------|-------------|--------|

### Recommendations
Prioritized list of actions: what to update, what to remove, what to replace.

## Rules

- Always run automated tools first before manual analysis.
- Don't flag dev/test dependencies as unused if they're used in test files or CI.
- Be careful with import-to-package-name mapping — many packages have different import names.
- Consider transitive dependencies — a package may be unused directly but required by another package.
- If `pip-audit` or similar tools aren't installed, inform the user and suggest installing them, then proceed with manual analysis.
