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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn license_install_valid_records_events() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_license_ok")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let license_path = repo_root().join("fixtures/licenses/valid_license.json");
    let st = storage::license_install_from_path(&db, &vault_root, &license_path, "tester")?;

    assert!(st.installed);
    assert!(st.valid);
    assert!(st.features.iter().any(|f| f == "EXPORT_PACKS"));

    let events = db.query_rows_tsv("SELECT event_type FROM audit_event ORDER BY seq ASC;")?;
    let flat: Vec<String> = events.into_iter().map(|r| r[0].clone()).collect();

    assert!(flat.iter().any(|e| e == "LicenseInstalled"));
    assert!(flat.iter().any(|e| e == "LicenseValidated"));

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}

#[test]
fn license_install_invalid_is_rejected_but_logged() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_license_bad")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let license_path = repo_root().join("fixtures/licenses/invalid_license.json");
    let err =
        storage::license_install_from_path(&db, &vault_root, &license_path, "tester").unwrap_err();
    assert_eq!(err.code, CoreErrorCode::LicenseInvalid);

    let rows = db.query_rows_tsv(
        "SELECT verification_status FROM license_install ORDER BY installed_at DESC LIMIT 1;",
    )?;
    assert!(!rows.is_empty());

    let events = db.query_rows_tsv("SELECT event_type FROM audit_event ORDER BY seq ASC;")?;
    let flat: Vec<String> = events.into_iter().map(|r| r[0].clone()).collect();

    assert!(flat.iter().any(|e| e == "LicenseInstalled"));
    assert!(flat.iter().any(|e| e == "LicenseRejected"));

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}
