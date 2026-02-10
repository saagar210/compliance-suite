use crate::audit::hasher;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::fs;
use std::path::Path;

pub struct ImportedEvidence {
    pub relative_path: String,
    pub sha256: String,
    pub byte_size: i64,
    pub content_type: String,
}

pub fn import_evidence_file(vault_root: &Path, src_file: &Path) -> CoreResult<ImportedEvidence> {
    if !src_file.exists() {
        return Err(CoreError::new(
            CoreErrorCode::NotFound,
            "source evidence file not found",
        ));
    }

    let tmp_dir = vault_root.join(".staging");
    fs::ensure_dir(&tmp_dir)?;

    let filename = src_file
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "evidence".to_string());

    let tmp_dst = tmp_dir.join(&filename);
    fs::atomic_copy_to(src_file, &tmp_dst)?;

    let sha256 = hasher::sha256_hex_file(&tmp_dst)?;
    let meta = std::fs::metadata(&tmp_dst)?;
    let byte_size = meta.len() as i64;

    let prefix = &sha256[0..2];
    let safe_filename = sanitize_filename(&filename);
    let rel = format!("evidence/{}/{}_{}", prefix, sha256, safe_filename);

    let final_path = vault_root.join(&rel);
    if !final_path.exists() {
        fs::atomic_copy_to(&tmp_dst, &final_path)?;
    }

    // best-effort cleanup
    let _ = std::fs::remove_file(&tmp_dst);

    Ok(ImportedEvidence {
        relative_path: rel,
        sha256,
        byte_size,
        content_type: content_type_for(&safe_filename),
    })
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c == '/' || c == '\\' { '_' } else { c })
        .collect()
}

fn content_type_for(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".pdf") {
        "application/pdf".to_string()
    } else if lower.ends_with(".png") {
        "image/png".to_string()
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else {
        "application/octet-stream".to_string()
    }
}
