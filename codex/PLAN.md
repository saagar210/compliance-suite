# DELTA PLAN

## A) Executive Summary
### Current state (repo-grounded)
- Monorepo root scripts (`pnpm lint/typecheck/test/build/format`) delegate to shell scripts in `scripts/` and currently focus verification on Rust workspace health.
- Domain logic is concentrated in `core/`, with app crates in `apps/*/src-tauri` acting as wrappers.
- Answer bank CRUD/search/list behavior is implemented in `core/src/answer_bank/mod.rs` and wrapped by app DTO conversion layers.
- Error taxonomy is centralized in Rust (`core/src/domain/errors.rs`) and mirrored in TS package policy docs.
- Deterministic behavior exists for ordering and canonicalization (e.g., tags/evidence normalization), but pagination parameters are passed through directly to SQL queries.
- Existing tests validate answer bank CRUD determinism and stable validation errors for create flows (`core/tests/answer_bank_tests.rs`).

### Key risks
- Pagination currently accepts unconstrained values (`limit`, `offset`) which may permit silent invalid behavior (negative limits/offsets) rather than explicit validation errors.
- Invalid pagination inputs can make downstream behavior ambiguous across SQLite contexts and wrappers.
- Current tests do not lock pagination validation behavior, creating regression risk.

### Improvement themes (prioritized)
1. Enforce explicit pagination input validation in core answer bank APIs.
2. Add regression tests for invalid pagination inputs and stable error taxonomy.
3. Keep app/core boundaries unchanged (no new DTO fields, no new error codes).

## B) Constraints & Invariants (Repo-derived)
### Explicit invariants
- Keep domain logic in `core/`; app command layers remain thin wrappers.
- Keep stable error code mapping (reuse `VALIDATION_ERROR`, avoid introducing new codes for this delta).
- Preserve deterministic ordering contract for list/search: `question_canonical ASC, entry_id ASC`.
- No dependency additions for this change.

### Implicit invariants (inferred)
- Audit chain and export determinism must remain unaffected by answer bank read-path changes.
- Existing CRUD and search behavior should remain unchanged for valid inputs.

### Non-goals
- No schema migration changes.
- No API signature changes in `apps/*/src-tauri` DTOs.
- No UI changes.

## C) Proposed Changes by Theme (Prioritized)
### Theme 1 — Validate pagination arguments in core
- Current approach: `ab_list_entries`/`ab_search_entries` interpolate `params.limit` and `params.offset` directly into SQL.
- Proposed change: introduce a small internal validator to enforce `limit > 0` and `offset >= 0`, returning `VALIDATION_ERROR` with explicit messages.
- Why: aligns with no-silent-default guidance and makes invalid inputs deterministic.
- Tradeoffs: stricter behavior may reject previously tolerated invalid values; preferred for integrity.
- Scope boundary: only answer bank pagination calls.
- Migration approach: additive validation with tests; no data migration.

### Theme 2 — Regression test coverage for pagination validation
- Current approach: tests cover create/update/delete determinism and a single create validation failure.
- Proposed change: add tests asserting invalid list/search params fail with `VALIDATION_ERROR`.
- Why: protects contract stability and prevents future regressions.
- Tradeoffs: slightly longer test runtime.
- Scope boundary: `core/tests/answer_bank_tests.rs` only.
- Migration approach: no migration required.

## D) File/Module Delta (Exact)
### ADD
- `codex/SESSION_LOG.md` — run log.
- `codex/PLAN.md` — this plan.
- `codex/DECISIONS.md` — judgment calls.
- `codex/CHECKPOINTS.md` — resumable checkpoints.
- `codex/VERIFICATION.md` — command evidence.
- `codex/CHANGELOG_DRAFT.md` — delivery draft.

### MODIFY
- `core/src/answer_bank/mod.rs` — pagination validator + call sites.
- `core/tests/answer_bank_tests.rs` — pagination validation tests.

### REMOVE/DEPRECATE
- None.

### Boundary rules
- Allowed: `core/tests` depending on `core` APIs.
- Forbidden: app-layer business logic additions, schema edits, new dependencies.

## E) Data Models & API Contracts (Delta)
- Current definitions: `ListParams` in `core/src/answer_bank/mod.rs`; DTO mirrors in app command modules.
- Proposed change: behavioral contract only (parameter validation), no struct shape change.
- Compatibility: backward-compatible for valid clients; invalid inputs now fail deterministically.
- Persisted data migrations: none.
- Versioning strategy: no version bump required since public shape is unchanged.

## F) Implementation Sequence (Dependency-Explicit)
1. **S1: Add core pagination validation**
   - Objective: reject invalid list/search params early.
   - Files: `core/src/answer_bank/mod.rs`
   - Preconditions: baseline tests green.
   - Dependencies: none.
   - Verification: `cargo test -p core answer_bank`.
   - Rollback: remove validator + call sites to restore prior behavior.
2. **S2: Add regression tests**
   - Objective: enforce stable validation behavior for invalid params.
   - Files: `core/tests/answer_bank_tests.rs`
   - Preconditions: S1 compiled.
   - Dependencies: S1.
   - Verification: `cargo test -p core answer_bank_tests`.
   - Rollback: remove added tests if invalid assumptions found.
3. **S3: Full suite confirmation**
   - Objective: ensure repo remains green.
   - Files: none (verification only).
   - Preconditions: S1/S2 pass.
   - Dependencies: S1/S2.
   - Verification: `pnpm lint && pnpm typecheck && pnpm test && pnpm build`.
   - Rollback: revert offending commit if full suite regresses.

## G) Error Handling & Edge Cases
- Current pattern: `CoreErrorCode::ValidationError` for invalid required fields.
- Proposed improvement: apply same pattern to invalid pagination (`limit <= 0`, `offset < 0`).
- Edge cases:
  - `limit = 0` rejected.
  - `limit < 0` rejected.
  - `offset < 0` rejected.
  - valid positive/zero combinations continue unchanged.
- Tests:
  - list/search both reject invalid params with stable code.

## H) Integration & Testing Strategy
- Integration points: app command wrappers continue to call core list/search unchanged.
- Unit/integration tests modified: `core/tests/answer_bank_tests.rs`.
- DoD:
  - New validation present and covered.
  - Baseline and final verification green.
  - Session artifacts updated for resumability.

## I) Assumptions & Judgment Calls
### Assumptions
- Invalid pagination should be treated as input validation failure, not coerced.
- Existing clients can comply with strict positive limit/nonnegative offset.

### Judgment calls
- Chose `VALIDATION_ERROR` reuse (vs introducing a new code) to avoid contract expansion and keep TS mirrors stable.
- Chose minimal scope in `core/answer_bank` only, deferring cross-module pagination normalization to future work.
