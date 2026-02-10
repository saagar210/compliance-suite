# Data Models and Persistence

## Persistence strategy
- **SQLite** stores structured records (vault metadata, evidence metadata, controls, mappings, events, SOP versions, answer bank).
- **Filesystem** stores evidence and attachments inside a vault directory.
- All state changes create an **audit event** appended to the event log, including DB writes and file writes.

## IDs and timestamps
- IDs are ULID (lexicographically sortable) or UUIDv7. Choose one and standardize.
- All times in UTC ISO-8601. UI can localize.

## Core entities (canonical)
### Vault
- `vault_id`
- `name`
- `root_path`
- `created_at`
- `encryption_mode` (`none | passphrase`)
- `schema_version`

### EvidenceItem
- `evidence_id`
- `vault_id`
- `filename`
- `relative_path`
- `content_type`
- `byte_size`
- `sha256`
- `source` (`manual_import | generated | extracted`)
- `tags` (string list)
- `created_at`
- `notes` (optional)

### Document
A “document” is a managed file (policy, SOP, etc.) that may have versions.
- `document_id`
- `vault_id`
- `title`
- `doc_type` (`policy | sop | procedure | other`)
- `current_version_id`
- `created_at`

### DocumentVersion
- `version_id`
- `document_id`
- `version_label` (e.g., `v1.2` or ULID)
- `relative_path`
- `sha256`
- `created_at`
- `author` (optional string)
- `change_summary`

### Control (Binder)
- `control_id`
- `framework` (e.g., `SOC2`, `ISO27001`, `CUSTOM`)
- `code` (e.g., `CC6.1`)
- `title`
- `description`
- `owner`
- `cadence` (`monthly|quarterly|annual|ad-hoc`)
- `status` (`not_started|in_progress|implemented|needs_review`)

### ControlEvidenceMapping
- `mapping_id`
- `control_id`
- `evidence_id`
- `notes`
- `created_at`

### AnswerBankEntry (Questionnaire)
- `entry_id`
- `vault_id`
- `question_canonical` (normalized)
- `answer_short`
- `answer_long`
- `evidence_links` (list of evidence_ids)
- `owner`
- `last_reviewed_at`
- `tags`

### QuestionnaireImport
Represents a particular questionnaire file imported.
- `import_id`
- `vault_id`
- `source_filename`
- `source_sha256`
- `imported_at`
- `format` (`xlsx|csv`)
- `column_map` (JSON)
- `status` (`imported|mapped|matched|exported`)

### QuestionnaireQuestion
- `q_id`
- `import_id`
- `row_index`
- `raw_text`
- `normalized_text`
- `answer_cell_ref` (xlsx cell or csv column)
- `status` (`unanswered|suggested|confirmed|skipped`)

### MatchSuggestion
- `suggestion_id`
- `q_id`
- `entry_id`
- `confidence` (0..1)
- `explanation` (JSON: tokens overlap, rules triggered, etc.)
- `created_at`

### SOP / Change Control
#### SOP
- `sop_id`
- `vault_id`
- `title`
- `department`
- `current_version_id`
- `status` (`draft|in_review|approved|published|archived`)

#### SOPVersion
- `sop_version_id`
- `sop_id`
- `sha256`
- `relative_path`
- `created_at`
- `author`
- `diff_base_version_id` (optional)

#### ChangeRequest
- `cr_id`
- `vault_id`
- `target_type` (`sop|policy|control`)
- `target_id`
- `title`
- `description`
- `status` (`draft|submitted|approved|rejected|implemented|verified`)
- `created_at`

#### Approval
- `approval_id`
- `cr_id`
- `approver`
- `decision` (`approved|rejected`)
- `comment`
- `decided_at`

#### TrainingAssignment / Ack
- `assign_id`
- `sop_id`
- `role`
- `assignee`
- `due_at`
- `created_at`

- `ack_id`
- `assign_id`
- `acknowledged_at`
- `quiz_score` (optional)

## Audit events
### Event
- `event_id`
- `vault_id`
- `occurred_at`
- `actor` (string, local user identity)
- `event_type` (enum)
- `payload` (JSON; schema per event_type)
- `prev_hash`
- `hash`

### Required event types (minimum)
- `VaultCreated`, `VaultOpened`
- `EvidenceAdded`, `EvidenceRemoved`
- `DocumentCreated`, `DocumentVersionAdded`
- `ControlCreated`, `ControlUpdated`, `ControlMappedEvidence`
- `AnswerBankEntryCreated`, `AnswerBankEntryUpdated`, `AnswerBankEntryDeleted`
- `QuestionnaireImported`, `QuestionnaireMatched`, `QuestionnaireExported`
- `QuestionnaireColumnMapSet`, `QuestionnaireColumnMapValidated`
- `SOPCreated`, `SOPSubmittedForReview`, `SOPApproved`, `SOPPublished`
- `ExportPackGenerated`
- `LicenseInstalled`, `LicenseValidated`, `LicenseRejected`

## SQLite schema notes
- Keep JSON columns limited and indexed via generated columns where needed.
- Use foreign keys + cascading rules carefully; prefer soft-delete for auditability.
- All writes occur in transactions; file writes are staged and committed with DB transaction semantics (see `06_ERROR_HANDLING.md`).
