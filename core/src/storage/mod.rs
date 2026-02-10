pub mod db;
pub mod evidence_fs;
pub mod tx;

use crate::audit::canonical::CanonicalJson;
use crate::audit::validator;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::domain::ids::Ulid;
use crate::domain::time::DETERMINISTIC_TIMESTAMP_UTC;
use crate::storage::db::SqliteDb;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Vault {
    pub vault_id: String,
    pub name: String,
    pub root_path: PathBuf,
    pub encryption_mode: String,
    pub schema_version: i64,
}

#[derive(Debug, Clone)]
pub struct EvidenceItem {
    pub evidence_id: String,
    pub vault_id: String,
    pub filename: String,
    pub relative_path: String,
    pub content_type: String,
    pub byte_size: i64,
    pub sha256: String,
    pub source: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub notes: Option<String>,
}

pub fn vault_db_path(vault_root: &Path) -> PathBuf {
    vault_root.join("vault.sqlite")
}

pub fn vault_create(vault_root: &Path, name: &str, actor: &str) -> CoreResult<Vault> {
    crate::util::fs::ensure_dir(vault_root)?;
    crate::util::fs::ensure_dir(&vault_root.join("evidence"))?;

    let db_path = vault_db_path(vault_root);
    let db = SqliteDb::new(&db_path);
    db.migrate()?;

    let vault_id = Ulid::new()?.to_string();
    let created_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();

    let event_sql = build_event_insert_sql(&db, &vault_id, actor, "VaultCreated", {
        let mut o = CanonicalJson::object();
        o.insert("vault_id", CanonicalJson::String(vault_id.clone()));
        o.insert("name", CanonicalJson::String(name.to_string()));
        o
    })?;
    let vault_insert = format!(
        "INSERT INTO vault (vault_id, name, root_path, created_at, encryption_mode) VALUES ({}, {}, {}, {}, {});",
        db.q(&vault_id),
        db.q(name),
        db.q(&vault_root.to_string_lossy()),
        db.q(&created_at),
        db.q("none"),
    );
    let script = format!("BEGIN;\n{}\n{}\nCOMMIT;", vault_insert, event_sql);
    db.exec_batch(&script)?;

    let schema_version = db.schema_version()?;
    Ok(Vault {
        vault_id,
        name: name.to_string(),
        root_path: vault_root.to_path_buf(),
        encryption_mode: "none".to_string(),
        schema_version,
    })
}

pub fn vault_open(vault_root: &Path) -> CoreResult<Vault> {
    let db_path = vault_db_path(vault_root);
    if !db_path.exists() {
        return Err(CoreError::new(
            CoreErrorCode::NotFound,
            "vault.sqlite not found",
        ));
    }
    let db = SqliteDb::new(&db_path);
    db.migrate()?;

    // Validate audit chain on open.
    validator::validate_chain(&db)?;

    let rows = db.query_rows_tsv(
        "SELECT vault_id, name, root_path, created_at, encryption_mode FROM vault LIMIT 1;",
    )?;
    if rows.is_empty() {
        return Err(CoreError::new(
            CoreErrorCode::CorruptVault,
            "missing vault row",
        ));
    }
    let r = &rows[0];
    let schema_version = db.schema_version()?;
    Ok(Vault {
        vault_id: r[0].clone(),
        name: r[1].clone(),
        root_path: PathBuf::from(r[2].clone()),
        encryption_mode: r[4].clone(),
        schema_version,
    })
}

pub fn evidence_add(
    db: &SqliteDb,
    vault_root: &Path,
    src_file: &Path,
    actor: &str,
) -> CoreResult<EvidenceItem> {
    // Callers should prefer validating on open; keep a cheap re-check here to fail fast.
    validator::validate_chain(db)?;
    let vault = load_vault_row(db, vault_root)?;

    let imported = evidence_fs::import_evidence_file(vault_root, src_file)?;

    let evidence_id = Ulid::new()?.to_string();
    let filename = src_file
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "evidence".to_string());

    let created_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();

    let event_sql = build_event_insert_sql(db, &vault.vault_id, actor, "EvidenceAdded", {
        let mut o = CanonicalJson::object();
        o.insert("evidence_id", CanonicalJson::String(evidence_id.clone()));
        o.insert(
            "relative_path",
            CanonicalJson::String(imported.relative_path.clone()),
        );
        o.insert("sha256", CanonicalJson::String(imported.sha256.clone()));
        o.insert("byte_size", CanonicalJson::Number(imported.byte_size));
        o.insert("filename", CanonicalJson::String(filename.clone()));
        o
    })?;
    let evidence_insert = format!(
        "INSERT INTO evidence_item (evidence_id, vault_id, filename, relative_path, content_type, byte_size, sha256, source, tags_json, created_at, notes, deleted_at) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, NULL, NULL);",
        db.q(&evidence_id),
        db.q(&vault.vault_id),
        db.q(&filename),
        db.q(&imported.relative_path),
        db.q(&imported.content_type),
        imported.byte_size,
        db.q(&imported.sha256),
        db.q("manual_import"),
        db.q("[]"),
        db.q(&created_at),
    );
    let script = format!("BEGIN;\n{}\n{}\nCOMMIT;", evidence_insert, event_sql);
    db.exec_batch(&script)?;

    Ok(EvidenceItem {
        evidence_id,
        vault_id: vault.vault_id,
        filename,
        relative_path: imported.relative_path,
        content_type: imported.content_type,
        byte_size: imported.byte_size,
        sha256: imported.sha256,
        source: "manual_import".to_string(),
        tags: vec![],
        created_at,
        notes: None,
    })
}

fn load_vault_row(db: &SqliteDb, vault_root: &Path) -> CoreResult<Vault> {
    let rows = db.query_rows_tsv(
        "SELECT vault_id, name, root_path, created_at, encryption_mode FROM vault LIMIT 1;",
    )?;
    if rows.is_empty() || rows[0].len() < 5 {
        return Err(CoreError::new(
            CoreErrorCode::CorruptVault,
            "missing vault row",
        ));
    }
    let r = &rows[0];
    let schema_version = db.schema_version()?;
    let root = if r[2].is_empty() {
        vault_root.to_path_buf()
    } else {
        PathBuf::from(r[2].clone())
    };
    Ok(Vault {
        vault_id: r[0].clone(),
        name: r[1].clone(),
        root_path: root,
        encryption_mode: r[4].clone(),
        schema_version,
    })
}

fn build_event_insert_sql(
    db: &SqliteDb,
    vault_id: &str,
    actor: &str,
    event_type: &str,
    payload: CanonicalJson,
) -> CoreResult<String> {
    use crate::audit::hasher;

    let event_id = Ulid::new()?.to_string();
    let occurred_at = DETERMINISTIC_TIMESTAMP_UTC.to_string();
    let payload_json = payload.to_string();

    let prev_hash = db
        .query_optional_string("SELECT hash FROM audit_event ORDER BY seq DESC LIMIT 1;")?
        .unwrap_or_else(|| {
            "0000000000000000000000000000000000000000000000000000000000000000".to_string()
        });

    let canonical = crate::audit::validator::canonical_event_string(
        &event_id,
        vault_id,
        &occurred_at,
        actor,
        event_type,
        &payload_json,
        &prev_hash,
    );
    let hash = hasher::sha256_hex_bytes(canonical.as_bytes())?;

    Ok(format!(
        "INSERT INTO audit_event (event_id, vault_id, occurred_at, actor, event_type, payload_json, prev_hash, hash) VALUES ({}, {}, {}, {}, {}, {}, {}, {});",
        db.q(&event_id),
        db.q(vault_id),
        db.q(&occurred_at),
        db.q(actor),
        db.q(event_type),
        db.q(&payload_json),
        db.q(&prev_hash),
        db.q(&hash),
    ))
}
