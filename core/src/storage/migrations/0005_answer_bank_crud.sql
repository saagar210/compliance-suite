-- 0005_answer_bank_crud.sql

PRAGMA foreign_keys = ON;

-- Phase 2.3 evolves the existing answer_bank table with a few additional fields
-- to support deterministic CRUD + content hashing.

ALTER TABLE answer_bank ADD COLUMN notes TEXT NULL;
ALTER TABLE answer_bank ADD COLUMN source TEXT NOT NULL DEFAULT 'manual';
ALTER TABLE answer_bank ADD COLUMN content_hash TEXT NOT NULL DEFAULT '';
ALTER TABLE answer_bank ADD COLUMN created_at TEXT NOT NULL DEFAULT '2000-01-01T00:00:00Z';
ALTER TABLE answer_bank ADD COLUMN updated_at TEXT NOT NULL DEFAULT '2000-01-01T00:00:00Z';

CREATE INDEX IF NOT EXISTS idx_answer_bank_content_hash ON answer_bank(content_hash);
CREATE INDEX IF NOT EXISTS idx_answer_bank_question ON answer_bank(question_canonical);
CREATE INDEX IF NOT EXISTS idx_answer_bank_vault_question ON answer_bank(vault_id, question_canonical, entry_id);

