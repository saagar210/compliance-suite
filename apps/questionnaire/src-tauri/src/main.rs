// Compliance Suite - Questionnaire Autopilot (Tauri v2)
//
// Phase 2.5: Tauri integration with IPC command handlers

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]

mod app_state;
mod commands;
mod error_map;

use app_state::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize application state
            let app_state = AppState::new();
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Vault commands
            commands::vault::vault_create,
            commands::vault::vault_open,
            commands::vault::vault_close,
            commands::vault::vault_lock,
            // Questionnaire commands
            commands::questionnaire::import_questionnaire,
            commands::questionnaire::get_column_profiles,
            commands::questionnaire::save_column_mapping,
            // Answer bank commands
            commands::answer_bank::answer_bank_create,
            commands::answer_bank::answer_bank_update,
            commands::answer_bank::answer_bank_delete,
            commands::answer_bank::answer_bank_list,
            // Matching commands
            commands::matching::get_matching_suggestions,
            // Export commands
            commands::export::generate_export_pack,
            // License commands
            commands::license::check_license_status,
            commands::license::install_license,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
