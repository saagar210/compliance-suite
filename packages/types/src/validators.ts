// Runtime validators for DTOs.
//
// Note: Docs call for Zod-based validation, but the monorepo currently does not
// include Zod as a dependency (and we are offline-first in this phase). These
// lightweight validators keep contracts explicit until UI validation wiring is
// introduced and Zod is adopted via an explicit Decision Log entry.

import type {
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

