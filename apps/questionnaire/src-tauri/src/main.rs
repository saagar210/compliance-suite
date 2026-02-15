// Compliance Suite - Questionnaire Autopilot (Tauri v2)
//
// Phase 2.5: Tauri integration with IPC command handlers

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app_state;
mod commands;
mod error_map;

use app_state::AppState;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize application state
            let app_state = AppState::new();
            app.manage(app_state);
            Ok(())
        })
            // License commands
            commands::license::check_license_status,
            commands::license::install_license,
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
