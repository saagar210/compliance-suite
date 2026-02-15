// DTO definitions mirrored from Rust. This is intentionally minimal in Phase 0/1.

export type VaultDto = {
  vault_id: string;
  name: string;
  root_path: string;
  created_at: string;
  encryption_mode: 'none' | 'passphrase';
  schema_version: number;
};

export type EvidenceDto = {
  evidence_id: string;
  vault_id: string;
  filename: string;
  relative_path: string;
  content_type: string;
  byte_size: number;
  sha256: string;
  source: 'manual_import' | 'generated' | 'extracted';
  tags: string[];
  created_at: string;
  notes?: string;
};

export type LicenseStatusDto = {
  installed: boolean;
  valid: boolean;
  license_id?: string;
  features: string[];
  verification_status?: string;
};

// Phase 2 (Questionnaire Autopilot)
export type ColumnMapDto = {
  question: string;
  answer: string;
  notes?: string;
};

export type QuestionnaireImportDto = {
  import_id: string;
  vault_id: string;
  source_filename: string;
  source_sha256: string;
  imported_at: string;
  format: 'csv' | 'xlsx';
  status: 'imported' | 'mapped' | string;
  column_map?: ColumnMapDto;
};

export type ColumnMapValidationIssueDto = {
  code: string;
  message: string;
  field?: 'question' | 'answer' | 'notes' | string;
};

export type ColumnMapValidationDto = {
  ok: boolean;
  issues: ColumnMapValidationIssueDto[];
};

// Phase 2.3 (Answer Bank)
export type AnswerBankEntryDto = {
  entry_id: string;
  vault_id: string;
  question_canonical: string;
  answer_short: string;
  answer_long: string;
  notes?: string;
  evidence_links: string[];
  owner: string;
  last_reviewed_at?: string;
  tags: string[];
  source: 'manual' | 'import' | 'match' | string;
  content_hash: string;
  created_at: string;
  updated_at: string;
};

export type AnswerBankCreateInputDto = {
  question_canonical: string;
  answer_short: string;
  answer_long: string;
  notes?: string;
  evidence_links: string[];
  owner: string;
  last_reviewed_at?: string;
  tags: string[];
  source: 'manual' | 'import' | 'match' | string;
};

export type AnswerBankUpdatePatchDto = Partial<
  Omit<AnswerBankCreateInputDto, 'evidence_links'> & {
    evidence_links: string[];
  }
> & {
  // Explicitly allow clearing notes/last_reviewed_at by passing null (mapped to None).
  notes?: string | null;
  last_reviewed_at?: string | null;
};

export type AnswerBankListParamsDto = {
  limit: number;
  offset: number;
};

// Phase 2.4 (Matching Algorithm)
export type MatchSuggestionDto = {
  id: string;
  answer_bank_entry_id: string;
  answer_preview: string;  // First 200 chars of answer
  score: number;           // 0.0 - 1.0
  confidence_explanation: string;
  normalized_question?: string;
  normalized_answer?: string;
};

export type MatchingInputDto = {
  question: string;
  vault_id: string;
  top_n?: number;
};

// Phase 2 (Extended): Column profiling for import
export type ColumnProfileDto = {
  column_index: number;
  inferred_type: 'question' | 'answer' | 'notes' | 'unknown';
  sample_values: string[];
  validation_issues?: string[];
};

export type QuestionnaireImportWithProfilesDto = QuestionnaireImportDto & {
  column_count: number;
  question_count: number;
  column_profiles: ColumnProfileDto[];
};

// Export pack DTOs
export type ExportPackDto = {
  id: string;
  vault_id: string;
  export_type: string;
  file_name: string;
  file_size_bytes: number;
  file_hash: string;
  manifest: {
    export_date: string;
    version: string;
    entry_count: number;
  };
  downloaded: boolean;
  created_at: string;
};

export type ExportFilterDto = {
  include_evidence?: boolean;
  include_audit_trail?: boolean;
  questionnaire_id?: string;
};
