# CHANGELOG DRAFT

## Theme: Answer bank pagination hardening
### What changed
- Added explicit validation for answer bank pagination parameters (`limit`, `offset`) in core list/search flows.
- Added regression tests proving invalid pagination is rejected with stable `VALIDATION_ERROR` codes.
- Added resumable codex planning/logging artifacts for this execution.

### Why
- Enforces the repo's “no silent defaults for required fields” quality bar for list/search inputs.
- Reduces ambiguity and cross-environment variation from unconstrained SQL pagination inputs.

### User-visible behavior
- Calls using invalid pagination now return a validation failure instead of relying on DB behavior.

### Risks / follow-ups
- If any existing caller relied on negative limits/offsets, it must be corrected upstream.
