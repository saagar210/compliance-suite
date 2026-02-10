-- 0003_license_verification.sql

PRAGMA foreign_keys = ON;

ALTER TABLE license_install ADD COLUMN signature_hex TEXT NULL;
ALTER TABLE license_install ADD COLUMN verification_status TEXT NULL; -- 'valid' | 'invalid'
ALTER TABLE license_install ADD COLUMN verified_at TEXT NULL;

CREATE INDEX IF NOT EXISTS idx_license_vault_installed ON license_install(vault_id, installed_at);
