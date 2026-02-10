# Assumptions, Judgment Calls, and How to Change Them

## Assumptions
1. **Desktop stack:** Tauri v2 + React + TypeScript.
2. **Persistence:** SQLite for metadata/relations; filesystem for evidence bytes.
3. **Offline-only:** no cloud sync in the initial scope.
4. **Identity:** single local user identity configured in app settings (name/email) for approvals and audit actor field.
5. **Questionnaire formats:** XLSX + CSV only for initial release.
6. **Determinism:** export pack determinism defined as stable file content + path order; export timestamps are treated as metadata (not part of file hash).

## Judgment calls made (and why)
- **Hybrid build order**: thin core + money slice in parallel to avoid a “core cathedral”.
- **Signed offline licensing**: best fit for offline-first monetization and robustness.
- **Answer matching V1**: heuristic approach first; embedding-based matching deferred until needed and must remain offline.

## Change protocol (to avoid scope death)
- Any change to:
  - persistence model
  - determinism rules
  - event schema
  - command DTOs
  requires:
  - a migration plan
  - updated golden tests
  - updated DTO mirror in `/packages/types`

## Deferred intentionally
- Cloud sync
- Third-party integrations
- Advanced OCR
- Full PDF/DOCX diff
