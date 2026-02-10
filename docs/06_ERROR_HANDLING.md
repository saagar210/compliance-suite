# Error Handling Strategy and Edge Cases

## Error taxonomy (stable)
All errors exposed to UI must map to `AppErrorDto` with a stable `code`.
Recommended codes:
- `VALIDATION_ERROR`
- `NOT_FOUND`
- `CONFLICT`
- `PERMISSION_DENIED`
- `IO_ERROR`
- `MISSING_CAPABILITY`
- `DB_ERROR`
- `MIGRATION_REQUIRED`
- `CORRUPT_VAULT`
- `HASH_MISMATCH`
- `EXPORT_FAILED`
- `IMPORT_FAILED`
- `LICENSE_INVALID`
- `UNSUPPORTED_FORMAT`
- `INTERNAL_ERROR`

## Capability errors (required tooling)
If a required local tool is missing (for example `sqlite3`, `shasum`, `zip`, `unzip`, `bash`):
- Return `MISSING_CAPABILITY` (not `CORRUPT_VAULT` / not a “corrupt file” classification).
- Include a clear message naming the missing tool (for example: "required tool missing: unzip").

## Core principles
1. **Fail fast on invalid input** (Zod in UI + Rust validation in core).
2. **No silent defaults** when values should be required.
3. **Atomicity for writes**:
   - File writes: write to temp, fsync, rename.
   - DB writes: transaction.
   - Combined operations: stage file first, then commit DB, then finalize (or use a two-phase approach).
4. **Audit everything**: record events for attempted and successful operations where appropriate.

## Edge cases by subsystem

### Vault open / migration
- Vault directory missing or moved.
- Schema version too old or too new.
- Partial migration from previous crash.
**Handling:** `MIGRATION_REQUIRED` or `CORRUPT_VAULT`, with safe recovery steps.

### Evidence import
- Duplicate filenames (must avoid overwrite).
- Same file imported twice (dedupe policy required).
- Large files (stream hashing).
- Unsupported file types.
**Handling:** dedupe by hash; store under content-addressed path; return `UNSUPPORTED_FORMAT` only if you truly cannot store.

### Audit log hash chain
- Tampered DB rows or event file (if stored separately).
- Clock skew (occurred_at monotonicity issues).
**Handling:** validate chain on open; if broken, mark vault as “integrity compromised” and require explicit user acknowledgement to proceed.

### Export packs
- Non-determinism from timestamps or OS ordering.
- Missing referenced evidence files.
- Zip creation failure or out-of-disk.
**Handling:** deterministic sort; exclude volatile metadata from hash; validate existence before export; `EXPORT_FAILED` with actionable message.

### Questionnaire import
- XLSX with merged cells, hidden sheets, multi-table layouts.
- CSV with inconsistent quoting/encoding.
- Missing question column.
**Handling:** robust parser with clear diagnostics; require user mapping; return `IMPORT_FAILED` with row/sheet context.

### Matching
- Low confidence matches should not be auto-applied without review.
- Domain-specific synonyms and abbreviations.
**Handling:** require manual acceptance; show explanation; store overrides.

### SOP workflow
- Invalid status transitions.
- Approver identity unknown.
**Handling:** enforce state machine; treat identity as local profile (config) with clear UI.

### Licensing
- Clock/date spoofing should not break offline license unless you enforce expiry.
- License file tampering.
**Handling:** verify signature; if expired, degrade gracefully and show a clear block on premium features.

## Logging and observability (local-only)
- Use structured logs (Rust tracing).
- Provide a “diagnostics bundle” export (no PII by default).
