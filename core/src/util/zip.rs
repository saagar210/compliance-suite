use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::shell;
use std::path::{Path, PathBuf};

pub const DETERMINISTIC_TOUCH_ARG: &str = "200001010000";

pub fn touch_tree_deterministic(root: &Path) -> CoreResult<()> {
    let caps = shell::capabilities();
    caps.require_bash()?;

    // Set mtimes to a fixed value to reduce zip output variance.
    let out = shell::run_bash_script(
        &format!(
            "find . -type f -print0 | xargs -0 touch -t {}",
            DETERMINISTIC_TOUCH_ARG
        ),
        root,
    )?;

    if !out.status.success() {
        return Err(CoreError::new(
            CoreErrorCode::IoError,
            "touch failed for staging tree",
        ));
    }

    Ok(())
}

pub fn zip_dir_deterministic(staging_dir: &Path, out_zip: &Path) -> CoreResult<()> {
    let caps = shell::capabilities();
    caps.require_zip()?;

    // Build a stable, sorted file list and feed it to zip via stdin (-@).
    let file_list = list_files_sorted(staging_dir)?;

    let mut stdin = String::new();
    for rel in file_list {
        stdin.push_str(&rel);
        stdin.push('\n');
    }

    let out = shell::run_capture_stdin_in(
        "zip",
        &["-X", "-q", out_zip.to_string_lossy().as_ref(), "-@"],
        stdin.as_bytes(),
        staging_dir,
    )?;

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
    let caps = shell::capabilities();
    caps.require_unzip()?;

    std::fs::create_dir_all(out_dir)?;

    let out = shell::run_capture(
        "unzip",
        &[
            "-qq",
            zip_path.to_string_lossy().as_ref(),
            "-d",
            out_dir.to_string_lossy().as_ref(),
        ],
    )?;

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
    let caps = shell::capabilities();
    caps.require_bash()?;

    let out = shell::run_bash_script(
        "find . -type f -print | sed 's#^\\./##' | LC_ALL=C sort",
        staging_dir,
    )?;

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

#[allow(dead_code)]
fn join(root: &Path, rel: &str) -> PathBuf {
    root.join(rel)
}
