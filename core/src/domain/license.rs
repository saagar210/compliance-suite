use crate::audit::canonical::CanonicalJson;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::json::JsonValue;

pub const LICENSE_VERIFICATION_STATUS_VALID: &str = "valid";
pub const LICENSE_VERIFICATION_STATUS_INVALID: &str = "invalid";

// Vendor public key (Ed25519) as hex (32 bytes => 64 hex chars).
//
// For development/testing, this is a repo-controlled public key. Replace for release.
pub const VENDOR_PUBLIC_KEY_HEX: &str =
    "613d659a7e2acf99913c44dde6a1e192795b88f322e2387947eee18635aa417f";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicensePayload {
    pub license_id: String,
    pub issued_to: String,
    pub issued_at: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicenseFile {
    pub payload: LicensePayload,
    pub signature_hex: String,
}

impl LicensePayload {
    pub fn to_canonical_json(&self) -> CanonicalJson {
        let mut o = CanonicalJson::object();
        o.insert(
            "features",
            CanonicalJson::Array(
                self.features
                    .iter()
                    .map(|f| CanonicalJson::String(f.clone()))
                    .collect(),
            ),
        );
        o.insert("issued_at", CanonicalJson::String(self.issued_at.clone()));
        o.insert("issued_to", CanonicalJson::String(self.issued_to.clone()));
        o.insert("license_id", CanonicalJson::String(self.license_id.clone()));
        o
    }

    pub fn to_canonical_string(&self) -> String {
        self.to_canonical_json().encode()
    }

    pub fn parse_canonical_json_str(s: &str) -> CoreResult<Self> {
        let v = JsonValue::parse(s)?;
        let obj = v.as_object()?;

        let license_id = obj.get_string("license_id")?;
        let issued_to = obj.get_string("issued_to")?;
        let issued_at = obj.get_string("issued_at")?;

        let feats = obj.get_array("features")?;
        let mut features = Vec::new();
        for f in feats {
            features.push(f.as_string()?);
        }

        Ok(LicensePayload {
            license_id,
            issued_to,
            issued_at,
            features,
        })
    }
}

impl LicenseFile {
    pub fn parse_json_str(s: &str) -> CoreResult<LicenseFile> {
        let v = JsonValue::parse(s)?;
        let obj = v.as_object()?;

        let license_id = obj.get_string("license_id")?;
        let issued_to = obj.get_string("issued_to")?;
        let issued_at = obj.get_string("issued_at")?;

        let feats = obj.get_array("features")?;
        let mut features = Vec::new();
        for f in feats {
            features.push(f.as_string()?);
        }

        let signature_hex = obj.get_string("signature_hex")?;

        Ok(LicenseFile {
            payload: LicensePayload {
                license_id,
                issued_to,
                issued_at,
                features,
            },
            signature_hex,
        })
    }

    pub fn payload_canonical(&self) -> String {
        self.payload.to_canonical_string()
    }
}

pub fn verify_license(license: &LicenseFile) -> CoreResult<()> {
    let pubkey = decode_hex_32(VENDOR_PUBLIC_KEY_HEX).map_err(|e| {
        CoreError::new(
            CoreErrorCode::InternalError,
            format!("invalid embedded public key: {e}"),
        )
    })?;

    verify_license_with_pubkey(license, pubkey)
}

pub fn verify_license_with_pubkey(license: &LicenseFile, pubkey: [u8; 32]) -> CoreResult<()> {
    let sig = decode_hex_64(&license.signature_hex)
        .map_err(|e| CoreError::new(CoreErrorCode::LicenseInvalid, e))?;

    let msg = license.payload_canonical();

    let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&pubkey)
        .map_err(|e| CoreError::new(CoreErrorCode::LicenseInvalid, e.to_string()))?;

    let signature = ed25519_dalek::Signature::from_bytes(&sig);

    verifying_key
        .verify_strict(msg.as_bytes(), &signature)
        .map_err(|_| CoreError::new(CoreErrorCode::LicenseInvalid, "invalid signature"))?;

    Ok(())
}

fn decode_hex_32(s: &str) -> Result<[u8; 32], String> {
    let v = decode_hex(s)?;
    if v.len() != 32 {
        return Err(format!("expected 32 bytes hex, got {}", v.len()));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&v);
    Ok(out)
}

fn decode_hex_64(s: &str) -> Result<[u8; 64], String> {
    let v = decode_hex(s)?;
    if v.len() != 64 {
        return Err(format!("expected 64 bytes hex, got {}", v.len()));
    }
    let mut out = [0u8; 64];
    out.copy_from_slice(&v);
    Ok(out)
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) {
        return Err("hex must have even length".to_string());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let hi = from_hex_nibble(bytes[i])?;
        let lo = from_hex_nibble(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn from_hex_nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err("invalid hex".to_string()),
    }
}
