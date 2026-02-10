use core::domain::errors::CoreResult;
use core::export::pack;
use core::storage::db::SqliteDb;
use core::storage::{self, vault_db_path};
use core::util::{fs, zip};
use std::path::{Path, PathBuf};

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

fn read_fixture(rel: &str) -> CoreResult<String> {
    Ok(std::fs::read_to_string(repo_root().join(rel))?)
}

fn write_tree_listing(dir: &Path) -> CoreResult<String> {
    use std::process::Command;
    let out = Command::new("bash")
        .arg("-lc")
        .arg(format!(
            "cd '{}' && find . -type f -print | sed 's#^\\./##' | LC_ALL=C sort",
            dir.to_string_lossy().replace('\'', "'\\''")
        ))
        .output()?;
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[test]
fn export_pack_matches_golden_fixtures_and_validates() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_export")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let e1 = repo_root().join("fixtures/evidence/policy_sample.pdf");
    let e2 = repo_root().join("fixtures/evidence/screenshot_sample.png");

    storage::evidence_add(&db, &vault_root, &e1, "tester")?;
    storage::evidence_add(&db, &vault_root, &e2, "tester")?;

    let out_dir = make_temp_dir("cs_export_out")?;
    let zip_path = out_dir.join("pack.zip");

    pack::generate_pack(&vault_root, &zip_path)?;
    pack::validate_pack(&zip_path)?;

    let extracted = make_temp_dir("cs_export_unpack")?;
    zip::unzip_to_dir(&zip_path, &extracted)?;

    let got_manifest = fs::read_to_string(&extracted.join("manifest.json"))?;
    let got_index = fs::read_to_string(&extracted.join("index.md"))?;
    let got_tree = write_tree_listing(&extracted)?;

    let exp_manifest = read_fixture("fixtures/golden_export/expected_manifest.json")?;
    let exp_index = read_fixture("fixtures/golden_export/expected_index.md")?;
    let exp_tree = read_fixture("fixtures/golden_export/expected_tree.txt")?;

    assert_eq!(got_manifest, exp_manifest);
    assert_eq!(got_index, exp_index);
    assert_eq!(got_tree, exp_tree);

    // Tamper a file and ensure validation fails.
    let tamper_dir = make_temp_dir("cs_export_tamper")?;
    zip::unzip_to_dir(&zip_path, &tamper_dir)?;
    fs::write_string(&tamper_dir.join("index.md"), "tampered\n")?;
    zip::touch_tree_deterministic(&tamper_dir)?;

    let tampered_zip = out_dir.join("pack_tampered.zip");
    zip::zip_dir_deterministic(&tamper_dir, &tampered_zip)?;

    assert!(pack::validate_pack(&tampered_zip).is_err());

    let _ = std::fs::remove_dir_all(&vault_root);
    let _ = std::fs::remove_dir_all(&out_dir);
    let _ = std::fs::remove_dir_all(&extracted);
    let _ = std::fs::remove_dir_all(&tamper_dir);
    Ok(())
}
