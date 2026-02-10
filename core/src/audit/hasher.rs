use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::shell;
use std::path::Path;

pub fn sha256_hex_file(path: &Path) -> CoreResult<String> {
    let caps = shell::capabilities();
    caps.require_shasum()?;

    let out = shell::run_capture("shasum", &["-a", "256", path.to_string_lossy().as_ref()])?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            format!("shasum failed: {}", stderr.trim()),
        ));
    }

    parse_shasum_output(&out.stdout)
}

pub fn sha256_hex_bytes(bytes: &[u8]) -> CoreResult<String> {
    let caps = shell::capabilities();
    caps.require_shasum()?;

    let out = shell::run_capture_stdin("shasum", &["-a", "256"], bytes)?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            format!("shasum failed: {}", stderr.trim()),
        ));
    }

    parse_shasum_output(&out.stdout)
}

fn parse_shasum_output(stdout: &[u8]) -> CoreResult<String> {
    let s = String::from_utf8_lossy(stdout);
    let first = s
        .split_whitespace()
        .next()
        .ok_or_else(|| CoreError::new(CoreErrorCode::InternalError, "invalid shasum output"))?;

    // sanity check length (sha256 hex)
    if first.len() != 64 {
        return Err(CoreError::new(
            CoreErrorCode::InternalError,
            format!("unexpected sha256 length: {}", first.len()),
        ));
    }

    Ok(first.to_string())
}
