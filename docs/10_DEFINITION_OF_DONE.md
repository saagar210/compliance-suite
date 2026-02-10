# Definition of Done (Acceptance Criteria)

## Global quality gates
- Root scripts pass: `pnpm lint`, `pnpm typecheck`, `pnpm test`, `pnpm build`.
- Core has unit and integration tests for integrity and determinism.
- A new engineer can follow `11_RUNBOOK.md` and reproduce the demo.

## Core Platform
- Vault create/open/close works.
- Evidence import stores file deterministically and records audit events.
- Audit chain validates and detects tampering.
- Export packs generated deterministically and validate against manifest.
- Licensing verification works (valid/invalid paths).

## App A — Questionnaire Autopilot
- Import XLSX/CSV and map columns.
- Populate question list with status tracking.
- Answer bank CRUD with evidence linking.
- Matching suggests top 3 answers with confidence + explanation.
- User can accept suggestion or override.
- Export produces a completed questionnaire file.
- Optional evidence pack export is available and validated.

## App B — Compliance Binder
- Control list CRUD and status.
- Map evidence to controls.
- Generate audit pack for period/framework with index by control + manifest.
- Audit events for mappings and exports.

## App C — SOP Builder + Change Control
- SOP create, add versions, publish workflow.
- Change requests with approval state machine.
- Training assignment and acknowledgment tracking.
- Audit events for state transitions.

## Demo deliverable (2 minutes)
- Create vault → import questionnaire → auto match → review → export questionnaire → generate evidence pack → validate pack.
