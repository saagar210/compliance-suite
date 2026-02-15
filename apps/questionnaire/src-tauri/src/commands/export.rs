use crate::app_state::AppState;
use crate::error_map::map_core_error;
use core::export::pack;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackDto {
    pub zip_path: String,
    pub manifest_version: i64,
    pub file_count: usize,
}

#[tauri::command]
pub async fn generate_export_pack(
    output_path: String,
    state: State<'_, AppState>,
) -> Result<ExportPackDto, String> {
    let vault_path = state
        .get_vault_path()
        .ok_or_else(|| "No vault open".to_string())?;

    let vault_root = Path::new(&vault_path);
    let out_zip = Path::new(&output_path);

    // Check license feature requirement
    let require_result =
        crate::commands::license::require_export_packs_feature(&vault_path, &state.actor);
    if let Err(e) = require_result {
        return Err(e.to_string());
    }

    let export_pack = pack::generate_pack(vault_root, out_zip).map_err(map_core_error)?;

    Ok(ExportPackDto {
        zip_path: export_pack.zip_path.to_string_lossy().to_string(),
        manifest_version: export_pack.manifest.version,
        file_count: export_pack.manifest.files.len(),
    })
}
