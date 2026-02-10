use crate::audit::hasher;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::export::index;
use crate::export::manifest::{ExportManifest, ManifestFile};
use crate::storage::db::SqliteDb;
use crate::storage::{vault_db_path, EvidenceItem};
use crate::util::{fs, zip};
use std::path::{Path, PathBuf};

pub struct ExportPack {
    pub zip_path: PathBuf,
    pub manifest: ExportManifest,
}

pub fn generate_pack(vault_root: &Path, out_zip: &Path) -> CoreResult<ExportPack> {
    let db = SqliteDb::new(&vault_db_path(vault_root));
    db.migrate()?;

    crate::audit::validator::validate_chain(&db)?;

    let evidence = load_evidence(&db)?;

    let staging = make_temp_dir("cs_export_staging")?;

    // Copy evidence files into staging under their relative paths.
    for e in &evidence {
        let src = vault_root.join(&e.relative_path);
        let dst = staging.join(&e.relative_path);
        fs::atomic_copy_to(&src, &dst)?;
    }

    // Write index.md
    let index_md = index::render_index_md(&evidence)?;
    fs::write_string(&staging.join("index.md"), &index_md)?;

    // Build manifest entries (excluding manifest.json itself).
    let mut files = Vec::new();

    // index
    let idx_path = staging.join("index.md");
    let idx_meta = std::fs::metadata(&idx_path)?;
    files.push(ManifestFile {
        path: "index.md".to_string(),
        sha256: hasher::sha256_hex_file(&idx_path)?,
        size: idx_meta.len() as i64,
    });

    for e in &evidence {
        let p = staging.join(&e.relative_path);
        let meta = std::fs::metadata(&p)?;
        files.push(ManifestFile {
            path: e.relative_path.clone(),
            sha256: hasher::sha256_hex_file(&p)?,
            size: meta.len() as i64,
        });
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));

    let manifest = ExportManifest { version: 1, files };

    fs::write_string(&staging.join("manifest.json"), &manifest.to_json_string())?;

    zip::touch_tree_deterministic(&staging)?;

    if let Some(parent) = out_zip.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // ensure output doesn't already exist with stale content
    if out_zip.exists() {
        std::fs::remove_file(out_zip)?;
    }

    zip::zip_dir_deterministic(&staging, out_zip)?;

    // best-effort cleanup
    let _ = std::fs::remove_dir_all(&staging);

    Ok(ExportPack {
        zip_path: out_zip.to_path_buf(),
        manifest,
    })
}

pub fn validate_pack(zip_path: &Path) -> CoreResult<()> {
    let out_dir = make_temp_dir("cs_export_validate")?;
    zip::unzip_to_dir(zip_path, &out_dir)?;

    let manifest_path = out_dir.join("manifest.json");
    let manifest_str = fs::read_to_string(&manifest_path)?;
    let manifest = ExportManifest::from_json_str(&manifest_str)?;

    for f in &manifest.files {
        let p = out_dir.join(&f.path);
        if !p.exists() {
            return Err(CoreError::new(
                CoreErrorCode::HashMismatch,
                format!("missing file {}", f.path),
            ));
        }
        let sha = hasher::sha256_hex_file(&p)?;
        if sha != f.sha256 {
            return Err(CoreError::new(
                CoreErrorCode::HashMismatch,
                format!("hash mismatch for {}", f.path),
            ));
        }
    }

    let _ = std::fs::remove_dir_all(&out_dir);
    Ok(())
}

fn load_evidence(db: &SqliteDb) -> CoreResult<Vec<EvidenceItem>> {
    let rows = db.query_rows_tsv(
        "SELECT evidence_id, vault_id, filename, relative_path, content_type, byte_size, sha256, source, tags_json, created_at, notes FROM evidence_item WHERE deleted_at IS NULL ORDER BY relative_path ASC;",
    )?;

    let mut out = Vec::new();
    for r in rows {
        if r.len() < 11 {
            return Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "unexpected evidence row",
            ));
        }
        out.push(EvidenceItem {
            evidence_id: r[0].clone(),
            vault_id: r[1].clone(),
            filename: r[2].clone(),
            relative_path: r[3].clone(),
            content_type: r[4].clone(),
            byte_size: r[5].parse().unwrap_or(0),
            sha256: r[6].clone(),
            source: r[7].clone(),
            tags: vec![],
            created_at: r[9].clone(),
            notes: if r[10].is_empty() {
                None
            } else {
                Some(r[10].clone())
            },
        });
    }
    Ok(out)
}

fn make_temp_dir(prefix: &str) -> CoreResult<PathBuf> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| CoreError::new(CoreErrorCode::InternalError, e.to_string()))?
        .as_millis();

    let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), ts));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
