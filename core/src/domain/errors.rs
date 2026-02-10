use std::fmt;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreErrorCode {
    ValidationError,
    NotFound,
    Conflict,
    PermissionDenied,
    IoError,
    MissingCapability,
    DbError,
    MigrationRequired,
    CorruptVault,
    HashMismatch,
    ExportFailed,
    ImportFailed,
    LicenseRequired,
    LicenseInvalid,
    UnsupportedFormat,
    InternalError,
}

impl CoreErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreErrorCode::ValidationError => "VALIDATION_ERROR",
            CoreErrorCode::NotFound => "NOT_FOUND",
            CoreErrorCode::Conflict => "CONFLICT",
            CoreErrorCode::PermissionDenied => "PERMISSION_DENIED",
            CoreErrorCode::IoError => "IO_ERROR",
            CoreErrorCode::MissingCapability => "MISSING_CAPABILITY",
            CoreErrorCode::DbError => "DB_ERROR",
            CoreErrorCode::MigrationRequired => "MIGRATION_REQUIRED",
            CoreErrorCode::CorruptVault => "CORRUPT_VAULT",
            CoreErrorCode::HashMismatch => "HASH_MISMATCH",
            CoreErrorCode::ExportFailed => "EXPORT_FAILED",
            CoreErrorCode::ImportFailed => "IMPORT_FAILED",
            CoreErrorCode::LicenseRequired => "LICENSE_REQUIRED",
            CoreErrorCode::LicenseInvalid => "LICENSE_INVALID",
            CoreErrorCode::UnsupportedFormat => "UNSUPPORTED_FORMAT",
            CoreErrorCode::InternalError => "INTERNAL_ERROR",
        }
    }
}

#[derive(Debug)]
pub struct CoreError {
    pub code: CoreErrorCode,
    pub message: String,
}

impl CoreError {
    pub fn new(code: CoreErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(CoreErrorCode::InternalError, message)
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for CoreError {}

impl From<std::io::Error> for CoreError {
    fn from(value: std::io::Error) -> Self {
        CoreError::new(CoreErrorCode::IoError, value.to_string())
    }
}
