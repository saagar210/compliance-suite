# Security, Privacy, and Integrity

## Threat model (explicit)
- Assume the user’s machine is trusted at the OS level but files can be modified by other local processes.
- No server-side trust; all integrity must be verifiable locally.
- Users may export packs to share externally; exports should include integrity evidence and avoid sensitive leakage.

## Offline-first policy
- No telemetry.
- No network required for core workflows.
- Any optional online features must be behind explicit user enablement and an allowlist.

## Integrity mechanisms
### Audit log hash chain
- Each event hash is computed over canonical content:
  - fixed field order
  - normalized JSON (no whitespace differences)
  - stable encoding (UTF-8)
- Validation occurs on vault open and before generating export packs.
- If integrity breaks: UI shows a high-visibility warning and blocks exports unless user explicitly overrides (configurable).

### Deterministic exports
Determinism policy must be written and tested:
- Files are added in a stable, sorted order.
- The manifest includes:
  - file paths
  - sha256
  - sizes
- Avoid hashing volatile metadata:
  - do not include OS file mtime in hash
  - if you include timestamps, include them in manifest but not in the hash computation, or canonicalize them (e.g., export time only).

### Deterministic imports (questionnaires)
Questionnaire import results must be repeatable for the same input file bytes:
- Stable (must match on re-import of identical file):
  - `source_sha256`
  - detected column identifiers (CSV header names; XLSX column letters) and their ordering
  - column profiling metrics derived from cell contents (counts, sampled values) for a fixed sampling window
- Variable (allowed to differ run-to-run):
  - `import_id`
  - `imported_at`
  - `actor` (if stored or emitted in audit events)

Validation and mapping must never silently default missing required fields (question/answer columns).

### Optional signed export packs
- For portfolio and high-assurance: sign the manifest with a local key (user-generated) or vendor key.
- This can be Phase 5.

## Encryption-at-rest (phased)
- Interfaces must support `encryption_mode` from day 1.
- Phase 1 can store unencrypted to move fast.
- Phase 5 can implement passphrase-based encryption:
  - Argon2 key derivation
  - AES-GCM for file encryption
  - Key not stored on disk

## PII and redaction
- Provide redaction profiles for exports (Phase 5).
- Default export excludes app logs and internal DB unless explicitly requested.
