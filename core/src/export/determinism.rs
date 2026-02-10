use crate::domain::time::DETERMINISTIC_TIMESTAMP_UTC;

pub fn deterministic_generated_at() -> &'static str {
    DETERMINISTIC_TIMESTAMP_UTC
}
