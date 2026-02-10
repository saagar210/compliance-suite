use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn ensure_dir(path: &Path) -> CoreResult<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn atomic_copy_to(path_src: &Path, path_dst: &Path) -> CoreResult<()> {
    let parent = path_dst
        .parent()
        .ok_or_else(|| CoreError::new(CoreErrorCode::IoError, "destination has no parent"))?;
    ensure_dir(parent)?;

    let tmp_path = tmp_sibling(path_dst);

    let mut src = fs::File::open(path_src)?;
    let mut tmp = fs::File::create(&tmp_path)?;

    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = src.read(&mut buf)?;
        if n == 0 {
            break;
        }
        tmp.write_all(&buf[..n])?;
    }

    tmp.flush()?;
    tmp.sync_all()?;

    fs::rename(&tmp_path, path_dst)?;
    Ok(())
}

pub fn read_to_string(path: &Path) -> CoreResult<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_string(path: &Path, contents: &str) -> CoreResult<()> {
    let parent = path
        .parent()
        .ok_or_else(|| CoreError::new(CoreErrorCode::IoError, "path has no parent"))?;
    ensure_dir(parent)?;

    let tmp_path = tmp_sibling(path);

    let mut f = fs::File::create(&tmp_path)?;
    f.write_all(contents.as_bytes())?;
    f.flush()?;
    f.sync_all()?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn tmp_sibling(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "tmp".to_string());
    let tmp_name = format!("{}.tmp", file_name);
    path.with_file_name(tmp_name)
}
