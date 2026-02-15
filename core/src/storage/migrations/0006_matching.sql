-- Migration 0006: Matching Algorithm
-- Adds match_suggestion table to track questionnaire answer matches

CREATE TABLE IF NOT EXISTS match_suggestion (
    id TEXT PRIMARY KEY,                    -- ULID
    vault_id TEXT NOT NULL,                 -- FK to vault
    question_id TEXT NOT NULL,              -- Question being matched
    answer_bank_entry_id TEXT NOT NULL,     -- FK to answer_bank_entry
    score REAL NOT NULL,                    -- 0.0 - 1.0 confidence score
    normalized_question TEXT NOT NULL,      -- Normalized for reproducibility
    normalized_answer TEXT NOT NULL,        -- Normalized for reproducibility
    confidence_explanation TEXT,            -- Human-readable reason
    accepted BOOLEAN NOT NULL DEFAULT 0,    -- User accepted this suggestion?
    accepted_at TIMESTAMP,                  -- When user accepted
    created_at TIMESTAMP NOT NULL,          -- UTC timestamp
    updated_at TIMESTAMP NOT NULL,          -- UTC timestamp
    FOREIGN KEY(vault_id) REFERENCES vault(id),
    FOREIGN KEY(answer_bank_entry_id) REFERENCES answer_bank_entry(id)
);

CREATE INDEX IF NOT EXISTS idx_match_suggestion_vault
ON match_suggestion(vault_id);

CREATE INDEX IF NOT EXISTS idx_match_suggestion_question
ON match_suggestion(question_id);

CREATE INDEX IF NOT EXISTS idx_match_suggestion_score
ON match_suggestion(score DESC);

-- Track schema version
INSERT OR REPLACE INTO schema_version(version, applied_at)
VALUES(6, datetime('now'));
