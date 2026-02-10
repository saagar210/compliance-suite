use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::shell;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SqliteDb {
    path: PathBuf,
}

impl SqliteDb {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn q(&self, s: &str) -> String {
        // Basic SQL string quoting for our controlled inputs.
        format!("'{}'", s.replace('\'', "''"))
    }

    pub fn exec_batch(&self, sql: &str) -> CoreResult<()> {
        let caps = shell::capabilities();
        caps.require_sqlite3()?;

        let cmd = format!("PRAGMA foreign_keys=ON; {}", sql);
        let out = shell::run_capture(
            "sqlite3",
            &[
                "-batch",
                "-bail",
                self.path.to_string_lossy().as_ref(),
                cmd.as_str(),
            ],
        )?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(CoreError::new(
                CoreErrorCode::DbError,
                format!("sqlite3 exec failed: {}", stderr.trim()),
            ));
        }
        Ok(())
    }

    pub fn query_rows_tsv(&self, sql: &str) -> CoreResult<Vec<Vec<String>>> {
        // Use a tab separator to reduce collisions.
        let caps = shell::capabilities();
        caps.require_sqlite3()?;

        let cmd = format!("PRAGMA foreign_keys=ON; {}", sql);
        let out = shell::run_capture(
            "sqlite3",
            &[
                "-batch",
                "-bail",
                "-noheader",
                "-separator",
                "\t",
                self.path.to_string_lossy().as_ref(),
                cmd.as_str(),
            ],
        )?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(CoreError::new(
                CoreErrorCode::DbError,
                format!("sqlite3 query failed: {}", stderr.trim()),
            ));
        }

        let s = String::from_utf8_lossy(&out.stdout);
        let mut rows = Vec::new();
        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }
            rows.push(line.split('\t').map(|c| c.to_string()).collect());
        }
        Ok(rows)
    }

    pub fn query_optional_string(&self, sql: &str) -> CoreResult<Option<String>> {
        let rows = self.query_rows_tsv(sql)?;
        if rows.is_empty() || rows[0].is_empty() {
            Ok(None)
        } else {
            Ok(Some(rows[0][0].clone()))
        }
    }

    pub fn schema_version(&self) -> CoreResult<i64> {
        let rows = self.query_rows_tsv(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version';",
        )?;
        if rows.is_empty() {
            return Ok(0);
        }

        let rows = self.query_rows_tsv("SELECT version FROM schema_version LIMIT 1;")?;
        if rows.is_empty() {
            return Ok(0);
        }
        let v: i64 = rows[0][0]
            .parse()
            .map_err(|_| CoreError::new(CoreErrorCode::CorruptVault, "invalid schema_version"))?;
        Ok(v)
    }

    pub fn migrate(&self) -> CoreResult<()> {
        let current = self.schema_version()?;

        let migrations_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("storage")
            .join("migrations");

        let mut files: Vec<_> = fs::read_dir(&migrations_dir)
            .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "sql").unwrap_or(false))
            .collect();
        files.sort();

        let mut version = current;
        for path in files {
            let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            // parse leading numeric prefix
            let mig_ver: i64 = fname.split('_').next().unwrap_or("0").parse().unwrap_or(0);

            if mig_ver <= version {
                continue;
            }

            let sql = fs::read_to_string(&path)
                .map_err(|e| CoreError::new(CoreErrorCode::IoError, e.to_string()))?;

            let set_version = format!("INSERT INTO schema_version (version) VALUES ({});", mig_ver);
            let script = format!(
                "BEGIN;\n{}\n{}\n{}\n{}\nCOMMIT;",
                sql,
                "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL);",
                "DELETE FROM schema_version;",
                set_version
            );
            self.exec_batch(&script)?;
            version = mig_ver;
        }

        Ok(())
    }
}
