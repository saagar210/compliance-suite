use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const DETERMINISTIC_TOUCH_ARG: &str = "200001010000";

pub fn touch_tree_deterministic(root: &Path) -> CoreResult<()> {
    // Use system `find` + `touch` for portability within our target dev envs.
    // This sets mtimes in the staging directory to a fixed value to keep zip output stable.
    let status = Command::new("bash")
        .arg("-lc")
        .arg(format!(
            "cd {} && find . -type f -print0 | xargs -0 touch -t {}",
            shell_escape(root),
            DETERMINISTIC_TOUCH_ARG
        ))
        .status()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    if !status.success() {
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            "touch failed for staging tree",
        ));
    }
    Ok(())
}

pub fn zip_dir_deterministic(staging_dir: &Path, out_zip: &Path) -> CoreResult<()> {
    // Build a stable, sorted file list and feed it to zip via stdin (-@).
    let file_list = list_files_sorted(staging_dir)?;

    let mut child = Command::new("zip")
        .current_dir(staging_dir)
        .args(["-X", "-q", out_zip.to_string_lossy().as_ref(), "-@"]) // -X excludes extra attrs
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| CoreError::new(CoreErrorCode::IoError, "zip stdin unavailable"))?;
        for rel in file_list {
            use std::io::Write;
            writeln!(stdin, "{}", rel)?;
        }
    }

    let out = child
        .wait_with_output()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            format!("zip failed: {}", stderr.trim()),
        ));
    }

    Ok(())
}

pub fn unzip_to_dir(zip_path: &Path, out_dir: &Path) -> CoreResult<()> {
    std::fs::create_dir_all(out_dir)?;
    let out = Command::new("unzip")
        .args([
            "-qq",
            zip_path.to_string_lossy().as_ref(),
            "-d",
            out_dir.to_string_lossy().as_ref(),
        ])
        .output()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            format!("unzip failed: {}", stderr.trim()),
        ));
    }

    Ok(())
}

fn list_files_sorted(staging_dir: &Path) -> CoreResult<Vec<String>> {
    let out = Command::new("bash")
        .arg("-lc")
        .arg(format!(
            "cd {} && find . -type f -print | sed 's#^\\./##' | LC_ALL=C sort",
            shell_escape(staging_dir)
        ))
        .output()
        .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            format!("find/sort failed: {}", stderr.trim()),
        ));
    }

    let s = String::from_utf8_lossy(&out.stdout);
    let mut files = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if !line.is_empty() {
            files.push(line.to_string());
        }
    }
    Ok(files)
}

fn shell_escape(p: &Path) -> String {
    // Minimal shell escaping for paths in our controlled environment.
    let s = p.to_string_lossy();
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[allow(dead_code)]
fn join(root: &Path, rel: &str) -> PathBuf {
    root.join(rel)
}
