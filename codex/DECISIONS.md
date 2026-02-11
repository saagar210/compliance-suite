# DECISIONS

## 2026-02-10T23:41:00Z — Pagination validation scope
- Decision: validate `answer_bank::ListParams` in core list/search functions with `VALIDATION_ERROR` for invalid values.
- Alternatives considered:
  - Coerce invalid values (rejected: silent defaults violate repo quality bar).
  - Add a new error code for pagination issues (rejected: unnecessary contract expansion).
- Consequence: invalid callers now fail fast and deterministically; valid behavior unchanged.
