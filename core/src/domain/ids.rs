use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use std::fmt;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

/// ULID implementation (26 chars, Crockford base32).
///
/// We avoid external dependencies in Phase 1; randomness is sourced from `/dev/urandom`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ulid(pub [u8; 16]);

impl Ulid {
    pub fn new() -> CoreResult<Self> {
        let now_ms: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CoreError::new(CoreErrorCode::InternalError, e.to_string()))?
            .as_millis() as u64;

        if now_ms >> 48 != 0 {
            return Err(CoreError::new(
                CoreErrorCode::InternalError,
                "timestamp overflow for ULID",
            ));
        }

        let mut bytes = [0u8; 16];
        // 48-bit timestamp big-endian
        bytes[0] = ((now_ms >> 40) & 0xFF) as u8;
        bytes[1] = ((now_ms >> 32) & 0xFF) as u8;
        bytes[2] = ((now_ms >> 24) & 0xFF) as u8;
        bytes[3] = ((now_ms >> 16) & 0xFF) as u8;
        bytes[4] = ((now_ms >> 8) & 0xFF) as u8;
        bytes[5] = (now_ms & 0xFF) as u8;

        // 80 bits randomness
        let mut rnd = [0u8; 10];
        std::fs::File::open("/dev/urandom")?.read_exact(&mut rnd)?;
        bytes[6..].copy_from_slice(&rnd);

        Ok(Ulid(bytes))
    }
}

impl fmt::Debug for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", encode_ulid(self.0))
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", encode_ulid(self.0))
    }
}

const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn encode_ulid(bytes: [u8; 16]) -> String {
    // ULID encodes 128 bits into 26 base32 chars (130 bits), leading 2 bits are zero.
    let mut out = [0u8; 26];
    let mut buffer: u32 = 0;
    let mut bits: u8 = 0;
    let mut idx = 0usize;

    for b in bytes {
        buffer = (buffer << 8) | (b as u32);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            let v = ((buffer >> bits) & 0x1F) as usize;
            out[idx] = CROCKFORD[v];
            idx += 1;
            // Keep only remaining bits to avoid accumulator growth beyond 32 bits.
            if bits == 0 {
                buffer = 0;
            } else {
                buffer &= (1u32 << bits) - 1;
            }
        }
    }

    if bits > 0 {
        let v = ((buffer << (5 - bits)) & 0x1F) as usize;
        out[idx] = CROCKFORD[v];
        idx += 1;
    }

    debug_assert_eq!(idx, 26, "ULID encoding produced unexpected length");
    String::from_utf8_lossy(&out).to_string()
}
