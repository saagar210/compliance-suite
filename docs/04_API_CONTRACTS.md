# API Contracts and State Management

## Command boundary
UI never touches DB/files directly. UI calls Tauri commands. Commands call core.
Commands return DTOs only (no internal structs). DTOs are mirrored in `/packages/types`.

## Tauri invoke conventions
- Each command returns `Result<T, AppErrorDto>`.
- `AppErrorDto` is stable and includes:
  - `code` (string enum)
  - `message` (human)
  - `details` (optional JSON)
  - `retryable` (bool)
  - `user_action` (optional string)

## Core command set (v1)
### Vault / Workspace
- `vault_create(name, path, encryption_mode, passphrase?) -> VaultDto`
- `vault_open(path, passphrase?) -> VaultDto`
- `vault_list_recent() -> VaultSummaryDto[]`
- `vault_close() -> void`

### Evidence
- `evidence_add(files[], tags?, notes?) -> EvidenceDto[]`
- `evidence_list(vault_id, filters?) -> EvidenceDto[]`
- `evidence_get_metadata(evidence_id) -> EvidenceDto`
- `evidence_remove(evidence_id) -> void` (soft-delete recommended)

### Export packs
- `export_generate_pack(vault_id, pack_spec) -> ExportPackDto`
- `export_validate_pack(path) -> ExportValidationDto`

`pack_spec` includes:
- `type`: `audit_pack | questionnaire_pack | custom`
- `date_range?`
- `control_ids?`
- `evidence_ids?`
- `redaction_profile?`
- `include_index`: bool
- `include_manifest`: bool

### Questionnaire Autopilot
- `qna_import(file_path) -> QuestionnaireImportDto`
- `qna_get_columns(import_id) -> ColumnProfileDto`
- `qna_set_column_map(import_id, map) -> QuestionnaireImportDto`
- `qna_validate_column_map(import_id) -> ColumnMapValidationDto`
- `qna_list_questions(import_id, filters?) -> QuestionDto[]`
- `qna_run_matching(import_id, params?) -> MatchRunDto`
- `qna_accept_suggestion(q_id, suggestion_id) -> QuestionDto`
- `qna_set_manual_answer(q_id, answer) -> QuestionDto`
- `qna_export(import_id, export_path, options?) -> ExportResultDto`

### Answer bank
- `ab_list_entries(params) -> AnswerBankEntryDto[]`
- `ab_create_entry(input) -> AnswerBankEntryDto`
- `ab_get_entry(entry_id) -> AnswerBankEntryDto`
- `ab_update_entry(entry_id, patch) -> AnswerBankEntryDto`
- `ab_delete_entry(entry_id) -> void`
- `ab_search_entries(query, params) -> AnswerBankEntryDto[]`
- `ab_link_evidence(entry_id, evidence_id) -> AnswerBankEntryDto`

### Binder
- `control_list(framework?) -> ControlDto[]`
- `control_create(control) -> ControlDto`
- `control_update(control_id, patch) -> ControlDto`
- `control_map_evidence(control_id, evidence_id, notes?) -> MappingDto`
- `binder_generate_audit_pack(vault_id, date_range, framework?) -> ExportPackDto`

### SOP + Change Control
- `sop_create(sop) -> SopDto`
- `sop_add_version(sop_id, file_path, summary) -> SopDto`
- `sop_submit_for_review(sop_id) -> SopDto`
- `sop_approve(sop_id, approver, comment?) -> SopDto`
- `sop_publish(sop_id) -> SopDto`
- `cr_create(change_request) -> ChangeRequestDto`
- `cr_approve(cr_id, approver, comment?) -> ApprovalDto`
- `training_assign(sop_id, role, assignee, due_at?) -> TrainingAssignmentDto`
- `training_ack(assign_id, quiz_score?) -> TrainingAckDto`

## UI state management
- **Zustand** stores: current vault, selected import, filters, selection sets, modal/panel state.
- **TanStack Query** queries: evidence list, controls list, questions list, answer bank list.
- Mutations call Tauri commands and invalidate queries.
- Shared validation via Zod schemas in `/packages/types`.

## API versioning
- `get_app_info() -> { api_version, app_version, schema_version }`
- If `schema_version` mismatch: UI shows “migration required” and triggers `vault_migrate()` flow.
