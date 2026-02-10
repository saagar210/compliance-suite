use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn sha256_hex_file(path: &Path) -> CoreResult<String> {
    let out = Command::new("shasum")
        .args(["-a", "256", path.to_string_lossy().as_ref()])
        .output()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

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
    let mut child = Command::new("shasum")
        .args(["-a", "256"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    {
        use std::io::Write;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| CoreError::new(CoreErrorCode::IoError, "shasum stdin unavailable"))?;
        stdin.write_all(bytes)?;
    }

    let out = child
        .wait_with_output()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

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
