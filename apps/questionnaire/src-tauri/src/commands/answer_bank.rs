use crate::error_map::{map_core_error, AppErrorDto};
use core::answer_bank;
use core::storage::db::SqliteDb;
use core::storage::vault_db_path;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AnswerBankEntryDto {
    pub entry_id: String,
    pub vault_id: String,
    pub question_canonical: String,
    pub answer_short: String,
    pub answer_long: String,
    pub notes: Option<String>,
    pub evidence_links: Vec<String>,
    pub owner: String,
    pub last_reviewed_at: Option<String>,
    pub tags: Vec<String>,
    pub source: String,
    pub content_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<answer_bank::AnswerBankEntry> for AnswerBankEntryDto {
    fn from(value: answer_bank::AnswerBankEntry) -> Self {
        Self {
            entry_id: value.entry_id,
            vault_id: value.vault_id,
            question_canonical: value.question_canonical,
            answer_short: value.answer_short,
            answer_long: value.answer_long,
            notes: value.notes,
            evidence_links: value.evidence_links,
            owner: value.owner,
            last_reviewed_at: value.last_reviewed_at,
            tags: value.tags,
            source: value.source,
            content_hash: value.content_hash,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnswerBankCreateInputDto {
    pub question_canonical: String,
    pub answer_short: String,
    pub answer_long: String,
    pub notes: Option<String>,
    pub evidence_links: Vec<String>,
    pub owner: String,
    pub last_reviewed_at: Option<String>,
    pub tags: Vec<String>,
    pub source: String,
}

impl From<AnswerBankCreateInputDto> for answer_bank::AnswerBankCreateInput {
    fn from(value: AnswerBankCreateInputDto) -> Self {
        Self {
            question_canonical: value.question_canonical,
            answer_short: value.answer_short,
            answer_long: value.answer_long,
            notes: value.notes,
            evidence_links: value.evidence_links,
            owner: value.owner,
            last_reviewed_at: value.last_reviewed_at,
            tags: value.tags,
            source: value.source,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AnswerBankUpdatePatchDto {
    pub question_canonical: Option<String>,
    pub answer_short: Option<String>,
    pub answer_long: Option<String>,
    pub notes: Option<Option<String>>,
    pub evidence_links: Option<Vec<String>>,
    pub owner: Option<String>,
    pub last_reviewed_at: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
    pub source: Option<String>,
}

impl From<AnswerBankUpdatePatchDto> for answer_bank::AnswerBankUpdatePatch {
    fn from(value: AnswerBankUpdatePatchDto) -> Self {
        Self {
            question_canonical: value.question_canonical,
            answer_short: value.answer_short,
            answer_long: value.answer_long,
            notes: value.notes,
            evidence_links: value.evidence_links,
            owner: value.owner,
            last_reviewed_at: value.last_reviewed_at,
            tags: value.tags,
            source: value.source,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnswerBankListParamsDto {
    pub limit: i64,
    pub offset: i64,
}

impl From<AnswerBankListParamsDto> for answer_bank::ListParams {
    fn from(value: AnswerBankListParamsDto) -> Self {
        Self {
            limit: value.limit,
            offset: value.offset,
        }
    }
}

pub fn ab_create_entry(
    vault_root: &str,
    input: AnswerBankCreateInputDto,
    actor: &str,
) -> Result<AnswerBankEntryDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out = answer_bank::ab_create_entry(&db, input.into(), actor).map_err(map_core_error)?;
    Ok(out.into())
}

pub fn ab_get_entry(vault_root: &str, entry_id: &str) -> Result<AnswerBankEntryDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out = answer_bank::ab_get_entry(&db, entry_id).map_err(map_core_error)?;
    Ok(out.into())
}

pub fn ab_update_entry(
    vault_root: &str,
    entry_id: &str,
    patch: AnswerBankUpdatePatchDto,
    actor: &str,
) -> Result<AnswerBankEntryDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out =
        answer_bank::ab_update_entry(&db, entry_id, patch.into(), actor).map_err(map_core_error)?;
    Ok(out.into())
}

pub fn ab_delete_entry(vault_root: &str, entry_id: &str, actor: &str) -> Result<(), AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    answer_bank::ab_delete_entry(&db, entry_id, actor).map_err(map_core_error)?;
    Ok(())
}

pub fn ab_list_entries(
    vault_root: &str,
    params: AnswerBankListParamsDto,
) -> Result<Vec<AnswerBankEntryDto>, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out = answer_bank::ab_list_entries(&db, params.into()).map_err(map_core_error)?;
    Ok(out.into_iter().map(Into::into).collect())
}

pub fn ab_search_entries(
    vault_root: &str,
    query: &str,
    params: AnswerBankListParamsDto,
) -> Result<Vec<AnswerBankEntryDto>, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out = answer_bank::ab_search_entries(&db, query, params.into()).map_err(map_core_error)?;
    Ok(out.into_iter().map(Into::into).collect())
}

pub fn ab_link_evidence(
    vault_root: &str,
    entry_id: &str,
    evidence_id: &str,
    actor: &str,
) -> Result<AnswerBankEntryDto, AppErrorDto> {
    let root = Path::new(vault_root);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;
    let out =
        answer_bank::ab_link_evidence(&db, entry_id, evidence_id, actor).map_err(map_core_error)?;
    Ok(out.into())
}
