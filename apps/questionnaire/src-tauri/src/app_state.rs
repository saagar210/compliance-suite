//! Application State (Phase 2.5)
//!
//! Shared state for the Tauri application, including vault storage
//! and actor context. Managed by Tauri's state management system.

use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Shared application state
///
/// This is managed by Tauri and injected into command handlers.
/// Holds the currently open vault and actor information.
pub struct AppState {
    /// Current vault path (if any vault is open)
    pub vault_path: Arc<Mutex<Option<String>>>,

    /// Current actor (user identifier)
    /// TODO Phase 5: Replace with proper authentication
    pub actor: String,

    /// Request trace ID for debugging
    pub request_id: String,
}

impl AppState {
    /// Create a new application state instance
    pub fn new() -> Self {
        Self {
            vault_path: Arc::new(Mutex::new(None)),
            actor: "user@localhost".to_string(), // Placeholder until Phase 5
            request_id: Uuid::new_v4().to_string(),
        }
    }

    /// Get the current vault path if one is open
    pub fn get_vault_path(&self) -> Option<String> {
        self.vault_path.lock().unwrap().clone()
    }

    /// Set the current vault path
    pub fn set_vault_path(&self, path: Option<String>) {
        *self.vault_path.lock().unwrap() = path;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
