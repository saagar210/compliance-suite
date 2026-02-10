-- 0001_init.sql

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS vault (
  vault_id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  root_path TEXT NOT NULL,
  created_at TEXT NOT NULL,
  encryption_mode TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS evidence_item (
  evidence_id TEXT PRIMARY KEY,
  vault_id TEXT NOT NULL,
  filename TEXT NOT NULL,
  relative_path TEXT NOT NULL,
  content_type TEXT NOT NULL,
  byte_size INTEGER NOT NULL,
  sha256 TEXT NOT NULL,
  source TEXT NOT NULL,
  tags_json TEXT NOT NULL,
  created_at TEXT NOT NULL,
  notes TEXT NULL,
  deleted_at TEXT NULL,
  FOREIGN KEY(vault_id) REFERENCES vault(vault_id)
);

CREATE INDEX IF NOT EXISTS idx_evidence_vault ON evidence_item(vault_id);
CREATE INDEX IF NOT EXISTS idx_evidence_sha256 ON evidence_item(sha256);

CREATE TABLE IF NOT EXISTS audit_event (
  seq INTEGER PRIMARY KEY AUTOINCREMENT,
  event_id TEXT NOT NULL,
  vault_id TEXT NOT NULL,
  occurred_at TEXT NOT NULL,
  actor TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  prev_hash TEXT NOT NULL,
  hash TEXT NOT NULL,
  FOREIGN KEY(vault_id) REFERENCES vault(vault_id)
);

CREATE INDEX IF NOT EXISTS idx_audit_vault_seq ON audit_event(vault_id, seq);

CREATE TABLE IF NOT EXISTS answer_bank (
  entry_id TEXT PRIMARY KEY,
  vault_id TEXT NOT NULL,
  question_canonical TEXT NOT NULL,
  answer_short TEXT NOT NULL,
  answer_long TEXT NOT NULL,
  evidence_links_json TEXT NOT NULL,
  owner TEXT NOT NULL,
  last_reviewed_at TEXT NULL,
  tags_json TEXT NOT NULL,
  FOREIGN KEY(vault_id) REFERENCES vault(vault_id)
);
