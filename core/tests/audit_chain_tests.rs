use core::audit::hasher;
use core::audit::validator;
use core::domain::errors::{CoreErrorCode, CoreResult};
use core::storage::db::SqliteDb;
use core::storage::{self, vault_db_path};
use std::path::PathBuf;

fn make_temp_dir(prefix: &str) -> CoreResult<PathBuf> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), ts));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[test]
fn audit_chain_validates_and_detects_tampering() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_audit")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let e1 = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("fixtures/evidence/policy_sample.pdf");
    let e2 = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("fixtures/evidence/screenshot_sample.png");

    let ev1 = storage::evidence_add(&db, &vault_root, &e1, "tester")?;
    let ev2 = storage::evidence_add(&db, &vault_root, &e2, "tester")?;

    // Evidence files are stored under the recorded relative paths and match the recorded hashes.
    let stored1 = vault_root.join(&ev1.relative_path);
    let stored2 = vault_root.join(&ev2.relative_path);
    assert!(stored1.exists());
    assert!(stored2.exists());
    assert_eq!(hasher::sha256_hex_file(&stored1)?, ev1.sha256);
    assert_eq!(hasher::sha256_hex_file(&stored2)?, ev2.sha256);

    validator::validate_chain(&db)?;

    // Tamper with an event payload and ensure validation fails.
    db.exec_batch("UPDATE audit_event SET payload_json='{}' WHERE seq=2;")?;

    let err = validator::validate_chain(&db).unwrap_err();
    assert_eq!(err.code, CoreErrorCode::HashMismatch);

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}
