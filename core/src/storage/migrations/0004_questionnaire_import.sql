-- 0004_questionnaire_import.sql

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS questionnaire_import (
  import_id TEXT PRIMARY KEY,
  vault_id TEXT NOT NULL,
  source_filename TEXT NOT NULL,
  source_sha256 TEXT NOT NULL,
  imported_at TEXT NOT NULL,
  format TEXT NOT NULL, -- 'csv' | 'xlsx'
  status TEXT NOT NULL DEFAULT 'imported',
  column_map_json TEXT NULL,
  FOREIGN KEY(vault_id) REFERENCES vault(vault_id)
);

CREATE INDEX IF NOT EXISTS idx_qna_import_vault ON questionnaire_import(vault_id);

CREATE TABLE IF NOT EXISTS questionnaire_import_column (
  import_id TEXT NOT NULL,
  col_ref TEXT NOT NULL, -- CSV header name or XLSX column letter
  ordinal INTEGER NOT NULL, -- stable column order as detected at import
  label TEXT NOT NULL,   -- UI label (often same as col_ref)
  non_empty_count INTEGER NOT NULL,
  sample_json TEXT NOT NULL, -- JSON array of strings (canonicalized)
  PRIMARY KEY(import_id, col_ref),
  FOREIGN KEY(import_id) REFERENCES questionnaire_import(import_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_qna_cols_import ON questionnaire_import_column(import_id);
