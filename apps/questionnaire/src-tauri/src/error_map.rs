//! Error Mapping (Phase 2.5)
//!
//! Maps core domain errors to Tauri-compatible error responses.
//! Provides structured error information for frontend error handling.

use core::domain::errors::CoreError;
use serde::{Deserialize, Serialize};

/// Application error DTO for Tauri responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub retryable: bool,
    pub user_action: Option<String>,
}

impl AppErrorDto {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
            retryable: false,
            user_action: None,
        }
    }

    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }

    pub fn with_user_action(mut self, action: &str) -> Self {
        self.user_action = Some(action.to_string());
        self
    }

    pub fn retryable(mut self) -> Self {
        self.retryable = true;
        self
    }
}

/// Map core domain error to Tauri error response
pub fn map_core_error(err: CoreError) -> AppErrorDto {
    AppErrorDto::new(err.code.as_str(), &err.message)
}

/// Convert AppErrorDto to JSON string for Tauri responses
impl std::fmt::Display for AppErrorDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or_else(|_| {
                format!(r#"{{"code":"{}","message":"{}"}}"#, self.code, self.message)
            })
        )
    }
}
