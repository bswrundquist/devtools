---
name: verify-codebase
description: Use when the user wants to verify the codebase is complete and in working order. Re-examines the full project for missed files, broken logic, untested paths, stale references, and build/test health.
tools: Bash, Read, Grep, Glob, Agent
---

# Verify Codebase

ultrathink

Re-examine the entire codebase from scratch to ensure nothing was missed and everything is in working order. This is a full-project audit — not a quick scan.

## Process

1. **Map the project structure** - Use Glob and directory listings to build a complete picture of every source file, config file, and test file. Note anything that looks orphaned, misplaced, or missing.

2. **Trace all entry points** - Identify every binary, CLI command, API endpoint, or public interface. For each one, trace the code path from entry to exit. Verify that:
   - All referenced modules, functions, and types actually exist
   - Imports resolve correctly
   - No dead code or unreachable branches

3. **Verify build health** - Run the build (`cargo build`, `make`, `npm run build`, etc.) and confirm it succeeds with zero errors and zero warnings. If there are warnings, flag them.

4. **Run the full test suite** - Execute all tests and confirm they pass. For any failures:
   - Read the failing test to understand what it expects
   - Read the implementation to find the mismatch
   - Report the root cause, not just the symptom

5. **Check for gaps** - Look for:
   - Source files with no corresponding tests
   - Config or schema definitions that are out of sync with the code
   - TODO/FIXME/HACK comments that indicate unfinished work
   - Functions or types that are defined but never called
   - Stale references to renamed or removed code
   - Missing error handling at system boundaries

6. **Validate integration points** - For each place the code interacts with external systems (filesystem, network, shell commands, databases):
   - Confirm the interface is correct (right arguments, right return types)
   - Confirm error cases are handled
   - Confirm idempotency where expected

7. **Cross-reference documentation and config** - Check that:
   - CLI help text matches actual behavior
   - Config schemas match what the code actually reads
   - Any README or doc files are consistent with current state

## Output Format

Present findings as a structured report:

### Status
- **Build**: pass/fail (with details if fail)
- **Tests**: X passed, Y failed, Z skipped (with failure details)
- **Warnings**: list any compiler/linter warnings

### Issues Found
For each issue, provide:
- **File and line**: exact location
- **Severity**: critical (breaks functionality), moderate (incorrect behavior in edge case), minor (cosmetic or cleanup)
- **Description**: what's wrong
- **Fix**: specific suggestion

### Verified Working
Briefly list the major components confirmed to be correct, so the user knows what was checked.

### Recommendations
Any improvements spotted during review that are not bugs but would strengthen the codebase.
