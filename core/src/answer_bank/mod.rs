//! Answer Bank (Phase 2.3)
//!
//! Stores canonical Q/A entries for questionnaire matching and export.
//! All writes append audit events and use deterministic canonicalization rules
//! for tags and content hashing.

use crate::audit::canonical::CanonicalJson;
use crate::audit::validator;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::domain::ids::Ulid;
use crate::domain::time::DETERMINISTIC_TIMESTAMP_UTC;
use crate::storage::db::SqliteDb;

#[derive(Debug, Clone)]
pub struct AnswerBankEntry {
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

#[derive(Debug, Clone)]
pub struct AnswerBankCreateInput {
    pub question_canonical: String,
    pub answer_short: String,
    pub answer_long: String,
    pub notes: Option<String>,
    pub evidence_links: Vec<String>,
    pub owner: String,
    pub last_reviewed_at: Option<String>,
    pub tags: Vec<String>,
    pub source: String, // 'manual' | 'import' | 'match' (free string, validated non-empty)
}

#[derive(Debug, Clone, Default)]
pub struct AnswerBankUpdatePatch {
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

#[derive(Debug, Clone)]
pub struct ListParams {
    pub limit: i64,
    pub offset: i64,
}

pub fn ab_create_entry(
    db: &SqliteDb,
    input: AnswerBankCreateInput,
    actor: &str,
) -> CoreResult<AnswerBankEntry> {
    validator::validate_chain(db)?;

    let vault_id = load_vault_id(db)?;

    let question_canonical =
        normalize_text_required("question_canonical", &input.question_canonical)?;
    let answer_short = normalize_text_required("answer_short", &input.answer_short)?;
    let answer_long = normalize_text_required("answer_long", &input.answer_long)?;
    let notes = input
        .notes
        .map(|s| normalize_text_optional(&s))
        .filter(|s| !s.is_empty());
    let owner = normalize_text_required("owner", &input.owner)?;
    let source = normalize_text_required("source", &input.source)?;

    let tags = normalize_tags(&input.tags);
    let evidence_links = normalize_ids(&input.evidence_links);

    let tags_json =
        CanonicalJson::Array(tags.iter().cloned().map(CanonicalJson::String).collect()).to_string();
    let evidence_json = CanonicalJson::Array(
        evidence_links
            .iter()
            .cloned()
            .map(CanonicalJson::String)
            .collect(),
    )
    .to_string();

    let content_hash = compute_content_hash(
        &question_canonical,
        &answer_short,
        &answer_long,
        notes.as_deref(),
        &tags,
        &source,
    )?;

    let entry_id = Ulid::new()?.to_string();
    let created_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();
    let updated_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();

    // NOTE: SqliteDb uses a simple TSV-oriented reader that is not safe for
    // embedded tabs/newlines. Store user text with lightweight escaping and
    // unescape on read.
    let insert_sql = format!(
        "INSERT INTO answer_bank (entry_id, vault_id, question_canonical, answer_short, answer_long, evidence_links_json, owner, last_reviewed_at, tags_json, notes, source, content_hash, created_at, updated_at) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});",
        db.q(&entry_id),
        db.q(&vault_id),
        db.q(&escape_db_text(&question_canonical)),
        db.q(&escape_db_text(&answer_short)),
        db.q(&escape_db_text(&answer_long)),
        db.q(&evidence_json),
        db.q(&escape_db_text(&owner)),
        match &input.last_reviewed_at {
            Some(s) => db.q(s),
            None => "NULL".to_string(),
        },
        db.q(&tags_json),
        match &notes {
            Some(s) => db.q(&escape_db_text(s)),
            None => "NULL".to_string(),
        },
        db.q(&escape_db_text(&source)),
        db.q(&content_hash),
        db.q(&created_at),
        db.q(&updated_at),
    );

    let event_sql =
        crate::storage::build_event_insert_sql(db, &vault_id, actor, "AnswerBankEntryCreated", {
            let mut o = CanonicalJson::object();
            o.insert("entry_id", CanonicalJson::String(entry_id.clone()));
            o.insert("content_hash", CanonicalJson::String(content_hash.clone()));
            o
        })?;

    db.exec_batch(&format!("BEGIN;\n{}\n{}\nCOMMIT;", insert_sql, event_sql))?;

    ab_get_entry(db, &entry_id)
}

pub fn ab_get_entry(db: &SqliteDb, entry_id: &str) -> CoreResult<AnswerBankEntry> {
    let rows = db.query_rows_tsv(&format!(
        "SELECT entry_id, vault_id, question_canonical, answer_short, answer_long, IFNULL(notes,''), evidence_links_json, owner, IFNULL(last_reviewed_at,''), tags_json, source, content_hash, created_at, updated_at FROM answer_bank WHERE entry_id={} LIMIT 1;",
        db.q(entry_id)
    ))?;
    if rows.is_empty() {
        return Err(CoreError::new(
            CoreErrorCode::NotFound,
            "answer bank entry not found",
        ));
    }
    let r = &rows[0];
    if r.len() < 14 {
        return Err(CoreError::new(
            CoreErrorCode::CorruptVault,
            "unexpected answer_bank row",
        ));
    }

    let notes = if r[5].trim().is_empty() {
        None
    } else {
        Some(unescape_db_text(&r[5]))
    };
    let last_reviewed_at = if r[8].trim().is_empty() {
        None
    } else {
        Some(r[8].clone())
    };

    Ok(AnswerBankEntry {
        entry_id: r[0].clone(),
        vault_id: r[1].clone(),
        question_canonical: unescape_db_text(&r[2]),
        answer_short: unescape_db_text(&r[3]),
        answer_long: unescape_db_text(&r[4]),
        notes,
        evidence_links: parse_string_array_json(&r[6])?,
        owner: unescape_db_text(&r[7]),
        last_reviewed_at,
        tags: parse_string_array_json(&r[9])?,
        source: unescape_db_text(&r[10]),
        content_hash: r[11].clone(),
        created_at: r[12].clone(),
        updated_at: r[13].clone(),
    })
}

pub fn ab_update_entry(
    db: &SqliteDb,
    entry_id: &str,
    patch: AnswerBankUpdatePatch,
    actor: &str,
) -> CoreResult<AnswerBankEntry> {
    validator::validate_chain(db)?;

    let before = ab_get_entry(db, entry_id)?;

    let question_canonical = patch
        .question_canonical
        .map(|s| normalize_text_required("question_canonical", &s))
        .transpose()?
        .unwrap_or_else(|| before.question_canonical.clone());
    let answer_short = patch
        .answer_short
        .map(|s| normalize_text_required("answer_short", &s))
        .transpose()?
        .unwrap_or_else(|| before.answer_short.clone());
    let answer_long = patch
        .answer_long
        .map(|s| normalize_text_required("answer_long", &s))
        .transpose()?
        .unwrap_or_else(|| before.answer_long.clone());

    let notes = match patch.notes {
        Some(v) => v
            .map(|s| normalize_text_optional(&s))
            .filter(|s| !s.is_empty()),
        None => before.notes.clone(),
    };

    let evidence_links = match patch.evidence_links {
        Some(v) => normalize_ids(&v),
        None => before.evidence_links.clone(),
    };
    let owner = patch
        .owner
        .map(|s| normalize_text_required("owner", &s))
        .transpose()?
        .unwrap_or_else(|| before.owner.clone());
    let last_reviewed_at = match patch.last_reviewed_at {
        Some(v) => v,
        None => before.last_reviewed_at.clone(),
    };
    let tags = match patch.tags {
        Some(v) => normalize_tags(&v),
        None => before.tags.clone(),
    };
    let source = patch
        .source
        .map(|s| normalize_text_required("source", &s))
        .transpose()?
        .unwrap_or_else(|| before.source.clone());

    let tags_json =
        CanonicalJson::Array(tags.iter().cloned().map(CanonicalJson::String).collect()).to_string();
    let evidence_json = CanonicalJson::Array(
        evidence_links
            .iter()
            .cloned()
            .map(CanonicalJson::String)
            .collect(),
    )
    .to_string();

    let content_hash = compute_content_hash(
        &question_canonical,
        &answer_short,
        &answer_long,
        notes.as_deref(),
        &tags,
        &source,
    )?;

    let updated_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();

    let update_sql = format!(
        "UPDATE answer_bank SET question_canonical={}, answer_short={}, answer_long={}, evidence_links_json={}, owner={}, last_reviewed_at={}, tags_json={}, notes={}, source={}, content_hash={}, updated_at={} WHERE entry_id={};",
        db.q(&escape_db_text(&question_canonical)),
        db.q(&escape_db_text(&answer_short)),
        db.q(&escape_db_text(&answer_long)),
        db.q(&evidence_json),
        db.q(&escape_db_text(&owner)),
        match &last_reviewed_at {
            Some(s) => db.q(s),
            None => "NULL".to_string(),
        },
        db.q(&tags_json),
        match &notes {
            Some(s) => db.q(&escape_db_text(s)),
            None => "NULL".to_string(),
        },
        db.q(&escape_db_text(&source)),
        db.q(&content_hash),
        db.q(&updated_at),
        db.q(entry_id),
    );

    let after_for_diff = AnswerBankEntry {
        entry_id: before.entry_id.clone(),
        vault_id: before.vault_id.clone(),
        question_canonical: question_canonical.clone(),
        answer_short: answer_short.clone(),
        answer_long: answer_long.clone(),
        notes: notes.clone(),
        evidence_links: evidence_links.clone(),
        owner: owner.clone(),
        last_reviewed_at: last_reviewed_at.clone(),
        tags: tags.clone(),
        source: source.clone(),
        content_hash: content_hash.clone(),
        created_at: before.created_at.clone(),
        updated_at: updated_at.clone(),
    };
    let changed_fields = compute_changed_fields(&before, &after_for_diff);

    let event_sql = crate::storage::build_event_insert_sql(
        db,
        &before.vault_id,
        actor,
        "AnswerBankEntryUpdated",
        {
            let mut o = CanonicalJson::object();
            o.insert("entry_id", CanonicalJson::String(entry_id.to_string()));
            o.insert("content_hash", CanonicalJson::String(content_hash.clone()));
            o.insert(
                "changed_fields",
                CanonicalJson::Array(
                    changed_fields
                        .into_iter()
                        .map(CanonicalJson::String)
                        .collect(),
                ),
            );
            o
        },
    )?;

    db.exec_batch(&format!("BEGIN;\n{}\n{}\nCOMMIT;", update_sql, event_sql))?;

    ab_get_entry(db, entry_id)
}

pub fn ab_delete_entry(db: &SqliteDb, entry_id: &str, actor: &str) -> CoreResult<()> {
    validator::validate_chain(db)?;

    let before = ab_get_entry(db, entry_id)?;

    let delete_sql = format!("DELETE FROM answer_bank WHERE entry_id={};", db.q(entry_id));
    let event_sql = crate::storage::build_event_insert_sql(
        db,
        &before.vault_id,
        actor,
        "AnswerBankEntryDeleted",
        {
            let mut o = CanonicalJson::object();
            o.insert("entry_id", CanonicalJson::String(entry_id.to_string()));
            o.insert(
                "content_hash",
                CanonicalJson::String(before.content_hash.clone()),
            );
            o
        },
    )?;

    db.exec_batch(&format!("BEGIN;\n{}\n{}\nCOMMIT;", delete_sql, event_sql))?;
    Ok(())
}

pub fn ab_list_entries(db: &SqliteDb, params: ListParams) -> CoreResult<Vec<AnswerBankEntry>> {
    validate_list_params(&params)?;
    let vault_id = load_vault_id(db)?;
    let rows = db.query_rows_tsv(&format!(
        "SELECT entry_id FROM answer_bank WHERE vault_id={} ORDER BY question_canonical ASC, entry_id ASC LIMIT {} OFFSET {};",
        db.q(&vault_id),
        params.limit,
        params.offset
    ))?;
    let mut out = Vec::new();
    for r in rows {
        if let Some(id) = r.first() {
            out.push(ab_get_entry(db, id)?);
        }
    }
    Ok(out)
}

pub fn ab_search_entries(
    db: &SqliteDb,
    query: &str,
    params: ListParams,
) -> CoreResult<Vec<AnswerBankEntry>> {
    validate_list_params(&params)?;
    let vault_id = load_vault_id(db)?;
    let q = normalize_text_optional(query);
    if q.is_empty() {
        return ab_list_entries(db, params);
    }
    // Basic LIKE search. Deterministic ordering, no ranking yet.
    let like = format!("%{}%", q.replace('%', "\\%").replace('_', "\\_"));
    let rows = db.query_rows_tsv(&format!(
        "SELECT entry_id FROM answer_bank WHERE vault_id={} AND (question_canonical LIKE {} ESCAPE '\\' OR answer_short LIKE {} ESCAPE '\\' OR answer_long LIKE {} ESCAPE '\\') ORDER BY question_canonical ASC, entry_id ASC LIMIT {} OFFSET {};",
        db.q(&vault_id),
        db.q(&like),
        db.q(&like),
        db.q(&like),
        params.limit,
        params.offset
    ))?;
    let mut out = Vec::new();
    for r in rows {
        if let Some(id) = r.first() {
            out.push(ab_get_entry(db, id)?);
        }
    }
    Ok(out)
}

pub fn ab_link_evidence(
    db: &SqliteDb,
    entry_id: &str,
    evidence_id: &str,
    actor: &str,
) -> CoreResult<AnswerBankEntry> {
    let mut patch = AnswerBankUpdatePatch::default();
    let mut before = ab_get_entry(db, entry_id)?;
    before.evidence_links.push(evidence_id.to_string());
    patch.evidence_links = Some(before.evidence_links);
    ab_update_entry(db, entry_id, patch, actor)
}

fn load_vault_id(db: &SqliteDb) -> CoreResult<String> {
    db.query_optional_string("SELECT vault_id FROM vault LIMIT 1;")?
        .ok_or_else(|| CoreError::new(CoreErrorCode::CorruptVault, "missing vault row"))
}

fn normalize_text_required(field: &str, s: &str) -> CoreResult<String> {
    let out = normalize_text_optional(s);
    if out.is_empty() {
        Err(CoreError::new(
            CoreErrorCode::ValidationError,
            format!("{field} is required"),
        ))
    } else {
        Ok(out)
    }
}

fn normalize_text_optional(s: &str) -> String {
    // Minimal canonicalization:
    // - trim leading/trailing whitespace
    // - normalize CRLF/CR -> LF
    let trimmed = s.trim();
    trimmed.replace("\r\n", "\n").replace('\r', "\n")
}

fn escape_db_text(s: &str) -> String {
    // Keep it reversible and deterministic. Order matters.
    s.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape_db_text(s: &str) -> String {
    // Reverse of escape_db_text.
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.peek().copied() {
            Some('n') => {
                let _ = chars.next();
                out.push('\n');
            }
            Some('t') => {
                let _ = chars.next();
                out.push('\t');
            }
            Some('\\') => {
                let _ = chars.next();
                out.push('\\');
            }
            _ => out.push('\\'),
        }
    }
    out
}

fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = tags
        .iter()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

fn normalize_ids(ids: &[String]) -> Vec<String> {
    let mut out: Vec<String> = ids
        .iter()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

fn compute_content_hash(
    question_canonical: &str,
    answer_short: &str,
    answer_long: &str,
    notes: Option<&str>,
    tags: &[String],
    source: &str,
) -> CoreResult<String> {
    let mut s = String::new();
    s.push_str(question_canonical);
    s.push('\n');
    s.push_str(answer_short);
    s.push('\n');
    s.push_str(answer_long);
    s.push('\n');
    if let Some(n) = notes {
        s.push_str(n);
    }
    s.push('\n');
    for (i, t) in tags.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(t);
    }
    s.push('\n');
    s.push_str(source);
    crate::audit::hasher::sha256_hex_bytes(s.as_bytes())
}

fn parse_string_array_json(s: &str) -> CoreResult<Vec<String>> {
    let v = crate::util::json::JsonValue::parse(s)?;
    let arr = v.as_array()?;
    let mut out = Vec::new();
    for vv in arr {
        match vv {
            crate::util::json::JsonValue::String(s) => out.push(s.clone()),
            _ => {
                return Err(CoreError::new(
                    CoreErrorCode::CorruptVault,
                    "expected string array",
                ))
            }
        }
    }
    Ok(out)
}

fn compute_changed_fields(before: &AnswerBankEntry, after: &AnswerBankEntry) -> Vec<String> {
    let mut fields = Vec::new();
    if before.question_canonical != after.question_canonical {
        fields.push("question_canonical".to_string());
    }
    if before.answer_short != after.answer_short {
        fields.push("answer_short".to_string());
    }
    if before.answer_long != after.answer_long {
        fields.push("answer_long".to_string());
    }
    if before.notes != after.notes {
        fields.push("notes".to_string());
    }
    if before.evidence_links != after.evidence_links {
        fields.push("evidence_links".to_string());
    }
    if before.owner != after.owner {
        fields.push("owner".to_string());
    }
    if before.last_reviewed_at != after.last_reviewed_at {
        fields.push("last_reviewed_at".to_string());
    }
    if before.tags != after.tags {
        fields.push("tags".to_string());
    }
    if before.source != after.source {
        fields.push("source".to_string());
    }
    if before.content_hash != after.content_hash {
        fields.push("content_hash".to_string());
    }
    fields.sort();
    fields
}

fn validate_list_params(params: &ListParams) -> CoreResult<()> {
    if params.limit <= 0 {
        return Err(CoreError::new(
            CoreErrorCode::ValidationError,
            "limit must be > 0",
        ));
    }
    if params.offset < 0 {
        return Err(CoreError::new(
            CoreErrorCode::ValidationError,
            "offset must be >= 0",
        ));
    }
    Ok(())
}
