// Runtime validators for DTOs.
//
// Note: Docs call for Zod-based validation, but the monorepo currently does not
// include Zod as a dependency (and we are offline-first in this phase). These
// lightweight validators keep contracts explicit until UI validation wiring is
// introduced and Zod is adopted via an explicit Decision Log entry.

import type {
  AnswerBankCreateInputDto,
  AnswerBankEntryDto,
  AnswerBankListParamsDto,
  AnswerBankUpdatePatchDto,
  ColumnMapDto,
  ColumnMapValidationDto,
  ColumnMapValidationIssueDto,
  QuestionnaireImportDto,
} from './dto';

function issue(
  code: string,
  message: string,
  field?: ColumnMapValidationIssueDto['field'],
): ColumnMapValidationIssueDto {
  return { code, message, field };
}

export function validateColumnMapDto(value: unknown): ColumnMapValidationDto {
  const issues: ColumnMapValidationIssueDto[] = [];
  const v = value as any;

  if (!v || typeof v !== 'object') {
    return { ok: false, issues: [issue('TYPE_ERROR', 'expected object')] };
  }
  if (typeof v.question !== 'string' || v.question.trim() === '') {
    issues.push(issue('REQUIRED', 'question is required', 'question'));
  }
  if (typeof v.answer !== 'string' || v.answer.trim() === '') {
    issues.push(issue('REQUIRED', 'answer is required', 'answer'));
  }
  if (v.notes != null && typeof v.notes !== 'string') {
    issues.push(issue('TYPE_ERROR', 'notes must be a string', 'notes'));
  }

  return { ok: issues.length === 0, issues };
}

export function isQuestionnaireImportDto(value: unknown): value is QuestionnaireImportDto {
  const v = value as any;
  return (
    !!v &&
    typeof v === 'object' &&
    typeof v.import_id === 'string' &&
    typeof v.vault_id === 'string' &&
    typeof v.source_filename === 'string' &&
    typeof v.source_sha256 === 'string' &&
    typeof v.imported_at === 'string' &&
    typeof v.format === 'string' &&
    typeof v.status === 'string'
  );
}

export function validateAnswerBankCreateInputDto(
  value: unknown,
): ColumnMapValidationDto {
  // Reuse the existing "ok/issues" shape for now (stopgap); UI can map these.
  const issues: ColumnMapValidationIssueDto[] = [];
  const v = value as any;
  if (!v || typeof v !== 'object') {
    return { ok: false, issues: [issue('TYPE_ERROR', 'expected object')] };
  }
  for (const k of ['question_canonical', 'answer_short', 'answer_long', 'owner', 'source']) {
    if (typeof v[k] !== 'string' || v[k].trim() === '') {
      issues.push(issue('REQUIRED', `${k} is required`, k));
    }
  }
  if (!Array.isArray(v.evidence_links)) {
    issues.push(issue('TYPE_ERROR', 'evidence_links must be an array', 'evidence_links'));
  }
  if (!Array.isArray(v.tags)) {
    issues.push(issue('TYPE_ERROR', 'tags must be an array', 'tags'));
  }
  if (v.notes != null && typeof v.notes !== 'string') {
    issues.push(issue('TYPE_ERROR', 'notes must be a string', 'notes'));
  }
  if (v.last_reviewed_at != null && typeof v.last_reviewed_at !== 'string') {
    issues.push(issue('TYPE_ERROR', 'last_reviewed_at must be a string', 'last_reviewed_at'));
  }
  return { ok: issues.length === 0, issues };
}

export function isAnswerBankEntryDto(value: unknown): value is AnswerBankEntryDto {
  const v = value as any;
  return (
    !!v &&
    typeof v === 'object' &&
    typeof v.entry_id === 'string' &&
    typeof v.vault_id === 'string' &&
    typeof v.question_canonical === 'string' &&
    typeof v.answer_short === 'string' &&
    typeof v.answer_long === 'string' &&
    Array.isArray(v.evidence_links) &&
    typeof v.owner === 'string' &&
    Array.isArray(v.tags) &&
    typeof v.source === 'string' &&
    typeof v.content_hash === 'string' &&
    typeof v.created_at === 'string' &&
    typeof v.updated_at === 'string'
  );
}

export function isAnswerBankListParamsDto(value: unknown): value is AnswerBankListParamsDto {
  const v = value as any;
  return (
    !!v &&
    typeof v === 'object' &&
    typeof v.limit === 'number' &&
    typeof v.offset === 'number'
  );
}

export function isAnswerBankUpdatePatchDto(value: unknown): value is AnswerBankUpdatePatchDto {
  // Patch can be partial; just ensure object-ness.
  return !!value && typeof value === 'object';
}
