use crate::domain::ids::Ulid;

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub event_id: Ulid,
    pub vault_id: String,
    pub occurred_at: String,
    pub actor: String,
    pub event_type: String,
    pub payload_json: String,
    pub prev_hash: String,
    pub hash: String,
}
