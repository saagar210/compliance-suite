use crate::error_map::{map_core_error, AppErrorDto};
use core::questionnaire;
use core::storage::db::SqliteDb;
use core::storage::vault_db_path;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ColumnMapDto {
    pub question: String,
    pub answer: String,
    pub notes: Option<String>,
}

impl From<ColumnMapDto> for questionnaire::ColumnMap {
    fn from(value: ColumnMapDto) -> Self {
        Self {
            question: value.question,
            answer: value.answer,
            notes: value.notes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuestionnaireImportDto {
    pub import_id: String,
    pub vault_id: String,
    pub source_filename: String,
    pub source_sha256: String,
    pub imported_at: String,
    pub format: String,
    pub status: String,
    pub column_map: Option<ColumnMapDto>,
}

impl From<questionnaire::QuestionnaireImport> for QuestionnaireImportDto {
    fn from(value: questionnaire::QuestionnaireImport) -> Self {
        Self {
            import_id: value.import_id,
            vault_id: value.vault_id,
            source_filename: value.source_filename,
            source_sha256: value.source_sha256,
            imported_at: value.imported_at,
            format: value.format,
            status: value.status,
            column_map: value.column_map.map(|m| ColumnMapDto {
                question: m.question,
                answer: m.answer,
                notes: m.notes,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColumnMapValidationIssueDto {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
}

impl From<questionnaire::ColumnMapValidationIssue> for ColumnMapValidationIssueDto {
    fn from(value: questionnaire::ColumnMapValidationIssue) -> Self {
        Self {
            code: value.code,
            message: value.message,
            field: value.field,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColumnMapValidationDto {
    pub ok: bool,
    pub issues: Vec<ColumnMapValidationIssueDto>,
}

impl From<questionnaire::ColumnMapValidation> for ColumnMapValidationDto {
    fn from(value: questionnaire::ColumnMapValidation) -> Self {
        Self {
            ok: value.ok,
            issues: value.issues.into_iter().map(Into::into).collect(),
        }
    }
}

pub fn qna_set_column_map(
    vault_root: &str,
    import_id: &str,
    map: ColumnMapDto,
    actor: &str,
) -> Result<QuestionnaireImportDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;

    let out = questionnaire::set_column_map(&db, import_id, &map.into(), actor)
        .map_err(map_core_error)?;
    Ok(out.into())
}

pub fn qna_validate_column_map(
    vault_root: &str,
    import_id: &str,
    actor: &str,
) -> Result<ColumnMapValidationDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;

    let out =
        questionnaire::validate_column_map(&db, import_id, Some(actor)).map_err(map_core_error)?;
    Ok(out.into())
}
