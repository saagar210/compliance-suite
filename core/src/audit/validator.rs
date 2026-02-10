use crate::audit::hasher;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::storage::db::SqliteDb;

const GENESIS_PREV_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

pub fn validate_chain(db: &SqliteDb) -> CoreResult<()> {
    let rows = db.query_rows_tsv(
        "SELECT seq, event_id, vault_id, occurred_at, actor, event_type, payload_json, prev_hash, hash FROM audit_event ORDER BY seq ASC;",
    )?;

    let mut prev = GENESIS_PREV_HASH.to_string();
    for row in rows {
        if row.len() != 9 {
            return Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "unexpected audit row shape",
            ));
        }
        let seq = &row[0];
        let event_id = &row[1];
        let vault_id = &row[2];
        let occurred_at = &row[3];
        let actor = &row[4];
        let event_type = &row[5];
        let payload_json = &row[6];
        let prev_hash = &row[7];
        let hash = &row[8];

        if prev_hash != &prev {
            return Err(CoreError::new(
                CoreErrorCode::HashMismatch,
                format!("prev_hash mismatch at seq {}", seq),
            ));
        }

        let canonical = canonical_event_string(
            event_id,
            vault_id,
            occurred_at,
            actor,
            event_type,
            payload_json,
            prev_hash,
        );
        let computed = hasher::sha256_hex_bytes(canonical.as_bytes())?;
        if &computed != hash {
            return Err(CoreError::new(
                CoreErrorCode::HashMismatch,
                format!("hash mismatch at seq {}", seq),
            ));
        }

        prev = computed;
    }

    Ok(())
}

pub fn canonical_event_string(
    event_id: &str,
    vault_id: &str,
    occurred_at: &str,
    actor: &str,
    event_type: &str,
    payload_json: &str,
    prev_hash: &str,
) -> String {
    // Fixed field order; newline separators.
    // Payload is expected to already be canonical JSON.
    format!(
        "event_id={}\nvault_id={}\noccurred_at={}\nactor={}\nevent_type={}\npayload={}\nprev_hash={}\n",
        event_id, vault_id, occurred_at, actor, event_type, payload_json, prev_hash
    )
}
