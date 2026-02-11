use core::answer_bank::{self, AnswerBankCreateInput, AnswerBankUpdatePatch, ListParams};
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
fn answer_bank_create_update_delete_and_list_are_deterministic_and_audited() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_answer_bank")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let created = answer_bank::ab_create_entry(
        &db,
        AnswerBankCreateInput {
            question_canonical: "  Do you have a policy?  \r\n".to_string(),
            answer_short: " Yes ".to_string(),
            answer_long: "Yes\r\nWe do.\r\n".to_string(),
            notes: Some("  note \r\n".to_string()),
            evidence_links: vec!["ev2".to_string(), "ev1".to_string(), "ev1".to_string()],
            owner: " alice ".to_string(),
            last_reviewed_at: None,
            tags: vec![
                "b".to_string(),
                "a".to_string(),
                " ".to_string(),
                "a".to_string(),
            ],
            source: "manual".to_string(),
        },
        "tester",
    )?;

    // Canonicalization expectations.
    assert_eq!(created.question_canonical, "Do you have a policy?");
    assert_eq!(created.answer_short, "Yes");
    assert_eq!(created.answer_long, "Yes\nWe do.");
    assert_eq!(created.notes.as_deref(), Some("note"));
    assert_eq!(created.owner, "alice");
    assert_eq!(created.tags, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(
        created.evidence_links,
        vec!["ev1".to_string(), "ev2".to_string()]
    );
    assert_eq!(created.source, "manual");
    assert_eq!(created.created_at, "2000-01-01T00:00:00Z");
    assert_eq!(created.updated_at, "2000-01-01T00:00:00Z");
    assert_eq!(created.content_hash.len(), 64);

    let listed = answer_bank::ab_list_entries(
        &db,
        ListParams {
            limit: 100,
            offset: 0,
        },
    )?;
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].entry_id, created.entry_id);

    // Add a second entry that should sort before the first by question_canonical.
    let created2 = answer_bank::ab_create_entry(
        &db,
        AnswerBankCreateInput {
            question_canonical: "A question".to_string(),
            answer_short: "A".to_string(),
            answer_long: "A long".to_string(),
            notes: None,
            evidence_links: vec![],
            owner: "alice".to_string(),
            last_reviewed_at: None,
            tags: vec![],
            source: "manual".to_string(),
        },
        "tester",
    )?;

    let listed2 = answer_bank::ab_list_entries(
        &db,
        ListParams {
            limit: 100,
            offset: 0,
        },
    )?;
    assert_eq!(listed2.len(), 2);
    assert_eq!(listed2[0].entry_id, created2.entry_id);
    assert_eq!(listed2[1].entry_id, created.entry_id);

    // Update changes should recompute the hash deterministically.
    let patch = AnswerBankUpdatePatch {
        answer_long: Some("Yes\nWe do.\n(Updated)".to_string()),
        ..Default::default()
    };
    let updated = answer_bank::ab_update_entry(&db, &created.entry_id, patch, "tester")?;
    assert_ne!(updated.content_hash, created.content_hash);

    // Search should be deterministic (ordered by question_canonical, entry_id).
    let searched = answer_bank::ab_search_entries(
        &db,
        "policy",
        ListParams {
            limit: 100,
            offset: 0,
        },
    )?;
    assert_eq!(searched.len(), 1);
    assert_eq!(searched[0].entry_id, created.entry_id);

    // Delete should remove the row and append an audit event.
    answer_bank::ab_delete_entry(&db, &created.entry_id, "tester")?;
    let err = answer_bank::ab_get_entry(&db, &created.entry_id).unwrap_err();
    assert_eq!(err.code, CoreErrorCode::NotFound);

    assert_event_types_contain(
        &db,
        &[
            "AnswerBankEntryCreated",
            "AnswerBankEntryUpdated",
            "AnswerBankEntryDeleted",
        ],
    )?;

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}

#[test]
fn answer_bank_validation_errors_are_stable() -> CoreResult<()> {
    let vault_root = make_temp_dir("cs_answer_bank_validation")?;
    storage::vault_create(&vault_root, "TestVault", "tester")?;

    let db = SqliteDb::new(&vault_db_path(&vault_root));
    db.migrate()?;

    let err = answer_bank::ab_create_entry(
        &db,
        AnswerBankCreateInput {
            question_canonical: " ".to_string(),
            answer_short: "a".to_string(),
            answer_long: "b".to_string(),
            notes: None,
            evidence_links: vec![],
            owner: "x".to_string(),
            last_reviewed_at: None,
            tags: vec![],
            source: "manual".to_string(),
        },
        "tester",
    )
    .unwrap_err();
    assert_eq!(err.code, CoreErrorCode::ValidationError);

    let err = answer_bank::ab_list_entries(
        &db,
        ListParams {
            limit: 0,
            offset: 0,
        },
    )
    .unwrap_err();
    assert_eq!(err.code, CoreErrorCode::ValidationError);

    let err = answer_bank::ab_search_entries(
        &db,
        "anything",
        ListParams {
            limit: 10,
            offset: -1,
        },
    )
    .unwrap_err();
    assert_eq!(err.code, CoreErrorCode::ValidationError);

    let _ = std::fs::remove_dir_all(&vault_root);
    Ok(())
}
