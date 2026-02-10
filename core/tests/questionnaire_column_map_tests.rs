use core::domain::errors::{CoreErrorCode, CoreResult};
use core::questionnaire::{self, ColumnMap};
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

fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(rel)
}

fn assert_event_types_contain(db: &SqliteDb, want: &[&str]) -> CoreResult<()> {
    let rows = db.query_rows_tsv("SELECT event_type FROM audit_event ORDER BY seq ASC;")?;
    let got: Vec<String> = rows
        .into_iter()
        .filter_map(|r| r.first().cloned())
        .collect();
    for w in want {
        assert!(got.iter().any(|g| g == w), "missing audit event type {w}");
    }
    Ok(())
}

#[test]
fn csv_column_map_is_persisted_validated_and_audited() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_qna_csv_map")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let imp = questionnaire::import_questionnaire(
        &db,
        &vault_root,
        &fixture_path("fixtures/questionnaires/sample_a.csv"),
        "tester",
    )?;

    let set = questionnaire::set_column_map(
        &db,
        &imp.import_id,
        &ColumnMap {
            question: "Question".to_string(),
            answer: "Answer".to_string(),
            notes: Some("Notes".to_string()),
        },
        "tester",
    )?;
    assert!(set.column_map.is_some());

    let reloaded = questionnaire::load_import(&db, &imp.import_id)?;
    assert_eq!(reloaded.column_map.unwrap().question, "Question");

    let validation = questionnaire::validate_column_map(&db, &imp.import_id, Some("tester"))?;
    assert!(validation.ok);
    assert!(validation.issues.is_empty());

    assert_event_types_contain(
        &db,
        &[
            "VaultCreated",
            "QuestionnaireImported",
            "QuestionnaireColumnMapSet",
            "QuestionnaireColumnMapValidated",
        ],
    )?;

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}

#[test]
fn xlsx_column_map_validation_catches_unknown_columns() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_qna_xlsx_map")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let imp = questionnaire::import_questionnaire(
        &db,
        &vault_root,
        &fixture_path("fixtures/questionnaires/sample_a.xlsx"),
        "tester",
    )?;

    // Persist an invalid map (unknown column ref). Validation should catch it.
    questionnaire::set_column_map(
        &db,
        &imp.import_id,
        &ColumnMap {
            question: "A".to_string(),
            answer: "Z".to_string(),
            notes: None,
        },
        "tester",
    )?;

    let validation = questionnaire::validate_column_map(&db, &imp.import_id, Some("tester"))?;
    assert!(!validation.ok);
    assert!(
        validation
            .issues
            .iter()
            .any(|i| i.code == "UNKNOWN_COLUMN" && i.field.as_deref() == Some("answer")),
        "expected UNKNOWN_COLUMN for answer"
    );

    // Ensure we did not misclassify validation as vault corruption.
    // (Any corruption error should be explicit and not due to missing columns.)
    for issue in &validation.issues {
        assert_ne!(issue.code, CoreErrorCode::CorruptVault.as_str());
    }

    assert_event_types_contain(&db, &["QuestionnaireImported", "QuestionnaireColumnMapSet"])?;

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}
