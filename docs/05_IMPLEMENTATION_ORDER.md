# Implementation Order (with explicit dependencies)

## Summary dependency graph
- Repo scaffold + scripts + CI
  -> Core: storage + migrations
    -> Core: audit events + hash chain (depends on canonical serialization)
    -> Core: evidence filesystem (depends on vault path + hashing util)
    -> Core: export engine (depends on storage + evidence metadata + determinism rules)
      -> App A: questionnaire import (depends on storage + evidence FS optional)
        -> App A: answer bank (depends on storage)
          -> App A: matching (depends on normalization + answer bank)
            -> App A: questionnaire export (depends on importer + mapping + match)
              -> App A: evidence pack export (depends on export engine)
    -> App B: controls + mapping (depends on storage + events)
      -> App B: audit pack by period (depends on export engine + mapping queries)
    -> App C: SOP workflow (depends on document versioning + events)
      -> App C: training acks (depends on SOP status + identity)

## Phase 0 — Scaffold and guardrails (must complete first)
1. Create monorepo layout exactly as in `02_REPO_STRUCTURE.md`.
2. Configure:
   - pnpm workspace
   - Cargo workspace
   - root scripts (lint/test/typecheck/build/format)
3. Add CI workflow that runs the same root scripts.
4. Add fixtures directory with at least:
   - Two sanitized questionnaires (xlsx)
   - Sample evidence docs
   - Golden export expected outputs

**Dependency:** none
**Gate:** All scripts pass on a clean checkout.

## Phase 1 — Core Platform Thin Slice
### 1.1 SQLite + migrations (core/storage)
- Implement DB connection factory, migration runner, schema_version table.
- Create initial schema for: vault, evidence, events, answer_bank (minimum).
- Add migration tests.

**Depends on:** Phase 0
**Gate:** migrations run idempotently, tests pass.

### 1.2 Evidence filesystem (core/storage/evidence_fs)
- Implement:
  - evidence import (copy into vault under content-addressed path or stable folder)
  - sha256 hashing
  - atomic write (temp + rename)
- Record EvidenceAdded event.

**Depends on:** 1.1
**Gate:** import 2 files and verify stored paths + hashes.

### 1.3 Audit event store + hash chain (core/audit)
- Canonical serialization rules (stable field ordering, normalized JSON).
- Hash chaining validator.
- Append events for every state change.
- Add tests:
  - chain validation
  - tamper detection

**Depends on:** 1.1 (for persistence) and canonical serializer
**Gate:** modifying an event payload fails validation.

### 1.4 Export engine (core/export)
- Define determinism policy:
  - stable sort order
  - manifest hashing rules
  - timestamp policy (see `08_SECURITY_PRIVACY.md`)
- Generate zip pack:
  - evidence files
  - `manifest.json`
  - `index.md` or `index.html`
- Golden tests: tree structure and manifest content.

**Depends on:** 1.2 and 1.3
**Gate:** golden export tests pass; pack validates against manifest.

### 1.5 Licensing (core/domain/license + app commands)
- Implement signed license verification (Ed25519) and storage.
- Wire minimal UI: install license file, show status.

**Depends on:** 1.1 and 1.3 (audit events for license changes)
**Gate:** valid license accepted; invalid rejected; events recorded.

## Phase 2 — App A: Questionnaire Autopilot (Money Slice)
### 2.1 Importer + column profiling
- Parse XLSX/CSV.
- Extract questions + candidate answer cells.
- Provide column profile for mapping UI.
- Persist QuestionnaireImport, QuestionnaireQuestion records and events.

**Depends on:** 1.1, 1.3
**Gate:** import sample_a and sample_b reproducibly (same counts).

### 2.2 Column mapping
- UI flow: map “question”, “answer”, optional “notes” columns.
- Store mapping JSON.
- Validate mapping before matching.

**Depends on:** 2.1
**Gate:** invalid mapping produces clear error; valid mapping enables match button.

### 2.3 Answer bank CRUD
- Create/update/list entries.
- Link evidence IDs.
- Add review metadata (owner, last_reviewed).

**Depends on:** 1.1
**Gate:** entry create/edit/link works; events appended.

### 2.4 Matching baseline
- Normalization pipeline (lowercase, punctuation strip, stopwords, domain-specific synonyms table).
- Matching algorithm V1:
  - token overlap + weighted keywords
  - optional simple fuzzy ratio
  - returns top N suggestions with confidence and explanation
- Persist MatchSuggestion and a MatchRun event.

**Depends on:** 2.1, 2.3
**Gate:** at least 60% of sample questions get non-zero suggestions with sensible ranking.

### 2.5 Review + export
- UI review: accept suggestion, edit manually, mark skipped.
- Export XLSX/CSV with answers filled.
- Optional: generate “questionnaire response pack” (export + evidence pack) using core/export.

**Depends on:** 2.2, 2.4, 1.4
**Gate:** end-to-end demo works twice with stable outputs.

## Phase 3 — App B: Compliance Binder (Flagship)
### 3.1 Controls library + CRUD
- Seed SOC2-ish control set (minimal) + allow custom.
- Control listing, status, owner, cadence.
- Events for changes.

**Depends on:** 1.1, 1.3
**Gate:** control create/update/list.

### 3.2 Evidence mapping UI
- Map evidence items to controls.
- Notes per mapping.
- Query “evidence coverage per control.”

**Depends on:** 3.1, 1.2
**Gate:** coverage view correct for sample data.

### 3.3 Audit pack by period
- Pack spec: framework + date range + include mapped evidence.
- Export pack contains index grouped by control with linked evidence.
- Include manifest and log the export event.

**Depends on:** 1.4, 3.2
**Gate:** “Generate Q4 pack” produces deterministic index + manifest.

## Phase 4 — App C: SOP + Change Control
### 4.1 SOP document versioning
- Create SOP, add versions as files.
- Diff viewer (start with text diff for markdown/plaintext; for PDF/docx defer).
- Workflow statuses.

**Depends on:** 1.1, 1.2, 1.3
**Gate:** SOP lifecycle events recorded; version hashes tracked.

### 4.2 Change requests + approvals
- Create CR, approve/reject, status transitions.
- Impact analysis (basic): list linked roles/assignments.

**Depends on:** 4.1
**Gate:** CR state machine enforced; invalid transitions rejected.

### 4.3 Training acknowledgments
- Assign SOP to roles/users.
- Ack tracking and reports.

**Depends on:** 4.1
**Gate:** report shows who is pending vs complete.

## Phase 5 — Polish + hardening
- performance passes for large vaults (thousands of evidence items)
- redaction profiles for exports
- improved matching (optional embeddings later; keep offline)
- release automation and installers
