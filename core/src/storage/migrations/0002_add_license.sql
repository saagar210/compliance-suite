-- 0002_add_license.sql

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS license_install (
  license_id TEXT PRIMARY KEY,
  vault_id TEXT NOT NULL,
  installed_at TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  FOREIGN KEY(vault_id) REFERENCES vault(vault_id)
);
