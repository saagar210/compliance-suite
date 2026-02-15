use crate::app_state::AppState;
use crate::error_map::map_core_error;
use core::answer_bank;
use core::questionnaire::matching::{MatchSuggestion, MatchingEngine};
use core::storage::db::SqliteDb;
use core::storage::vault_db_path;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSuggestionDto {
    pub answer_bank_entry_id: String,
    pub score: f64,
    pub normalized_question: String,
    pub normalized_answer: String,
    pub confidence_explanation: String,
}

impl From<MatchSuggestion> for MatchSuggestionDto {
    fn from(value: MatchSuggestion) -> Self {
        Self {
            answer_bank_entry_id: value.answer_bank_entry_id,
            score: value.score,
            normalized_question: value.normalized_question,
            normalized_answer: value.normalized_answer,
            confidence_explanation: value.confidence_explanation,
        }
    }
}

#[tauri::command]
pub async fn get_matching_suggestions(
    question: String,
    top_n: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<MatchSuggestionDto>, String> {
    let vault_path = state
        .get_vault_path()
        .ok_or_else(|| "No vault open".to_string())?;

    let root = Path::new(&vault_path);
    let db = SqliteDb::new(&vault_db_path(root));
    db.migrate().map_err(map_core_error)?;

    // Load all answer bank entries for the vault
    let params = answer_bank::ListParams {
        limit: 10000, // Large limit to get all entries
        offset: 0,
    };
    let entries = answer_bank::ab_list_entries(&db, params).map_err(map_core_error)?;

    // Create matching engine with answer bank
    let engine = MatchingEngine::new(entries);

    // Get suggestions (default to top 5 if not specified)
    let suggestions = engine
        .get_suggestions(&question, top_n.unwrap_or(5))
        .map_err(map_core_error)?;

    Ok(suggestions.into_iter().map(Into::into).collect())
}
