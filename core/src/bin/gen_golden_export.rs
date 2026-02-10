use core::export::pack;
use core::storage::db::SqliteDb;
use core::storage::{self, vault_db_path};
use core::util::{fs, zip};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn make_temp_dir(prefix: &str) -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), ts));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn tree_listing(dir: &Path) -> String {
    use std::process::Command;
    let out = Command::new("bash")
        .arg("-lc")
        .arg(format!(
            "cd '{}' && find . -type f -print | sed 's#^\\./##' | LC_ALL=C sort",
            dir.to_string_lossy().replace('\'', "'\\''")
        ))
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn main() {
    let repo = repo_root();
    let fixtures = repo.join("fixtures");

    let vault_root = make_temp_dir("cs_golden_vault");
    storage::vault_create(&vault_root, "GoldenVault", "tester").unwrap();

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate().unwrap();

    let e1 = fixtures.join("evidence/policy_sample.pdf");
    let e2 = fixtures.join("evidence/screenshot_sample.png");

    storage::evidence_add(&db, &vault_root, &e1, "tester").unwrap();
    storage::evidence_add(&db, &vault_root, &e2, "tester").unwrap();

    let out_dir = make_temp_dir("cs_golden_out");
    let zip_path = out_dir.join("pack.zip");
    pack::generate_pack(&vault_root, &zip_path).unwrap();

    let extracted = make_temp_dir("cs_golden_unpack");
    zip::unzip_to_dir(&zip_path, &extracted).unwrap();

    let manifest = fs::read_to_string(&extracted.join("manifest.json")).unwrap();
    let index = fs::read_to_string(&extracted.join("index.md")).unwrap();
    let tree = tree_listing(&extracted);

    let golden_dir = fixtures.join("golden_export");
    std::fs::create_dir_all(&golden_dir).unwrap();

    fs::write_string(&golden_dir.join("expected_manifest.json"), &manifest).unwrap();
    fs::write_string(&golden_dir.join("expected_index.md"), &index).unwrap();
    fs::write_string(&golden_dir.join("expected_tree.txt"), &tree).unwrap();

    println!("Wrote golden fixtures to {}", golden_dir.display());
}
