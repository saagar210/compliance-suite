# Database Schema Reference

This document describes the SQLite database schema used by the Compliance Suite.

## Overview

The compliance vault uses a single SQLite database (`vault.db`) to store all metadata, configurations, and relationships. The actual evidence files are stored in the filesystem, referenced by the database.

**Database Location:** `<vault-root>/vault.db`

**Schema Version:** Managed via migrations (current: v6)

**Foreign Keys:** Enabled via `PRAGMA foreign_keys = ON`

---

## Entity Relationship Diagram

```
┌─────────────┐
│   vault     │ (1)
└──────┬──────┘
       │
       ├──(1:N)─┐
       │        │
       │    ┌───▼──────────────┐
       │    │ evidence_item    │
       │    └──────────────────┘
       │
       ├──(1:N)─┐
       │        │
       │    ┌───▼──────────────────────┐
       │    │ questionnaire_import     │ (1)
       │    └──────────┬───────────────┘
       │               │
       │               └──(1:N)─┐
       │                        │
       │                  ┌─────▼────────────────────────┐
       │                  │questionnaire_import_column   │
       │                  └──────────────────────────────┘
       │
       ├──(1:N)─┐
       │        │
       │    ┌───▼──────────────┐
       │    │  answer_bank     │
       │    └──────────────────┘
       │
       └──(1:N)─┐
                │
            ┌───▼──────────────┐
            │  audit_event     │
            └──────────────────┘
```

---

## Tables

### `vault`

Stores vault metadata. Each vault has exactly one record.

| Column           | Type    | Constraints    | Description                          |
|------------------|---------|----------------|--------------------------------------|
| vault_id         | TEXT    | PRIMARY KEY    | UUID v4                              |
| name             | TEXT    | NOT NULL       | Human-readable vault name            |
| root_path        | TEXT    | NOT NULL       | Absolute path to vault directory     |
| created_at       | TEXT    | NOT NULL       | ISO 8601 timestamp                   |
| encryption_mode  | TEXT    | NOT NULL       | "none" (Phase 3 will add encryption) |

**Indexes:** None (single record)

**Example:**
```sql
INSERT INTO vault (vault_id, name, root_path, created_at, encryption_mode)
VALUES (
  'f47ac10b-58cc-4372-a567-0e02b2c3d479',
  'Security Compliance Vault',
  '/home/user/compliance-vault',
  '2024-02-15T10:30:00Z',
  'none'
);
```

---

### `evidence_item`

Stores metadata for evidence files (documents, screenshots, etc.).

| Column         | Type    | Constraints           | Description                              |
|----------------|---------|-----------------------|------------------------------------------|
| evidence_id    | TEXT    | PRIMARY KEY           | UUID v4                                  |
| vault_id       | TEXT    | NOT NULL, FK(vault)   | Parent vault                             |
| filename       | TEXT    | NOT NULL              | Original filename                        |
| relative_path  | TEXT    | NOT NULL              | Path relative to vault root              |
| content_type   | TEXT    | NOT NULL              | MIME type (e.g., "application/pdf")      |
| byte_size      | INTEGER | NOT NULL              | File size in bytes                       |
| sha256         | TEXT    | NOT NULL              | SHA-256 hash of file content             |
| source         | TEXT    | NOT NULL              | "upload" | "scan" | "import"           |
| tags_json      | TEXT    | NOT NULL              | JSON array of tags                       |
| notes          | TEXT    | NULL                  | Optional notes                           |
| created_at     | TEXT    | NOT NULL              | ISO 8601 timestamp                       |
| deleted_at     | TEXT    | NULL                  | Soft delete timestamp                    |

**Indexes:**
- `idx_evidence_vault` on `vault_id`
- `idx_evidence_sha256` on `sha256`

**Foreign Keys:**
- `vault_id` → `vault(vault_id)`

**Example:**
```sql
INSERT INTO evidence_item (
  evidence_id, vault_id, filename, relative_path, content_type,
  byte_size, sha256, source, tags_json, notes, created_at, deleted_at
) VALUES (
  'a1b2c3d4-e5f6-4789-0123-456789abcdef',
  'f47ac10b-58cc-4372-a567-0e02b2c3d479',
  'security-policy.pdf',
  'evidence/security-policy.pdf',
  'application/pdf',
  245760,
  'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
  'upload',
  '["security", "policy"]',
  'Annual security policy document',
  '2024-02-15T10:45:00Z',
  NULL
);
```

---

### `questionnaire_import`

Tracks imported questionnaire files.

| Column            | Type    | Constraints           | Description                              |
|-------------------|---------|-----------------------|------------------------------------------|
| import_id         | TEXT    | PRIMARY KEY           | UUID v4                                  |
| vault_id          | TEXT    | NOT NULL, FK(vault)   | Parent vault                             |
| source_filename   | TEXT    | NOT NULL              | Original filename                        |
| source_sha256     | TEXT    | NOT NULL              | SHA-256 hash of source file              |
| imported_at       | TEXT    | NOT NULL              | ISO 8601 timestamp                       |
| format            | TEXT    | NOT NULL              | "csv" | "xlsx"                          |
| status            | TEXT    | NOT NULL, DEFAULT     | "imported" | "mapped" | "processed"    |
| column_map_json   | TEXT    | NULL                  | JSON object with column mapping          |

**Indexes:**
- `idx_qna_import_vault` on `vault_id`

**Foreign Keys:**
- `vault_id` → `vault(vault_id)`

**Status Flow:**
1. `imported` - File uploaded, columns detected
2. `mapped` - User has mapped question/answer columns
3. `processed` - Matching/processing complete (Phase 3)

**Example:**
```sql
INSERT INTO questionnaire_import (
  import_id, vault_id, source_filename, source_sha256, imported_at,
  format, status, column_map_json
) VALUES (
  'b2c3d4e5-f6a7-4890-1234-567890bcdef0',
  'f47ac10b-58cc-4372-a567-0e02b2c3d479',
  'security-questionnaire.xlsx',
  'a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456',
  '2024-02-15T11:00:00Z',
  'xlsx',
  'mapped',
  '{"question":"A","answer":"B","notes":"C"}'
);
```

---

### `questionnaire_import_column`

Stores column metadata for imported questionnaires.

| Column          | Type    | Constraints                  | Description                          |
|-----------------|---------|------------------------------|--------------------------------------|
| import_id       | TEXT    | NOT NULL, FK, PK composite   | Parent import                        |
| col_ref         | TEXT    | NOT NULL, PK composite       | Column reference ("A", "B", etc.)    |
| ordinal         | INTEGER | NOT NULL                     | Column position (0-based)            |
| label           | TEXT    | NOT NULL                     | Column header/label                  |
| non_empty_count | INTEGER | NOT NULL                     | Count of non-empty cells             |
| sample_json     | TEXT    | NOT NULL                     | JSON array of sample values          |

**Primary Key:** `(import_id, col_ref)`

**Indexes:**
- `idx_qna_cols_import` on `import_id`

**Foreign Keys:**
- `import_id` → `questionnaire_import(import_id)` ON DELETE CASCADE

**Example:**
```sql
INSERT INTO questionnaire_import_column (
  import_id, col_ref, ordinal, label, non_empty_count, sample_json
) VALUES (
  'b2c3d4e5-f6a7-4890-1234-567890bcdef0',
  'A',
  0,
  'Question',
  50,
  '["What is your encryption policy?","How do you handle breaches?","What backup procedures?"]'
);
```

---

### `answer_bank`

Stores reusable answers for common compliance questions.

| Column              | Type    | Constraints           | Description                              |
|---------------------|---------|-----------------------|------------------------------------------|
| entry_id            | TEXT    | PRIMARY KEY           | UUID v4                                  |
| vault_id            | TEXT    | NOT NULL, FK(vault)   | Parent vault                             |
| question_canonical  | TEXT    | NOT NULL              | Canonical question text                  |
| answer_short        | TEXT    | NOT NULL              | Brief answer (1-2 sentences)             |
| answer_long         | TEXT    | NOT NULL              | Detailed answer                          |
| notes               | TEXT    | NULL                  | Optional notes                           |
| evidence_links_json | TEXT    | NOT NULL              | JSON array of evidence_ids               |
| owner               | TEXT    | NOT NULL              | Entry owner/author                       |
| last_reviewed_at    | TEXT    | NULL                  | ISO 8601 timestamp                       |
| tags_json           | TEXT    | NOT NULL              | JSON array of tags                       |
| source              | TEXT    | NOT NULL, DEFAULT     | "manual" | "import" | "suggestion"    |
| content_hash        | TEXT    | NOT NULL, DEFAULT     | SHA-256 of canonical content             |
| created_at          | TEXT    | NOT NULL, DEFAULT     | ISO 8601 timestamp                       |
| updated_at          | TEXT    | NOT NULL, DEFAULT     | ISO 8601 timestamp                       |

**Indexes:**
- `idx_answer_bank_content_hash` on `content_hash`
- `idx_answer_bank_question` on `question_canonical`
- `idx_answer_bank_vault_question` on `(vault_id, question_canonical, entry_id)`

**Foreign Keys:**
- `vault_id` → `vault(vault_id)`

**Example:**
```sql
INSERT INTO answer_bank (
  entry_id, vault_id, question_canonical, answer_short, answer_long,
  notes, evidence_links_json, owner, last_reviewed_at, tags_json,
  source, content_hash, created_at, updated_at
) VALUES (
  'c3d4e5f6-a7b8-4901-2345-678901cdef01',
  'f47ac10b-58cc-4372-a567-0e02b2c3d479',
  'What encryption do you use for data at rest?',
  'AES-256 encryption',
  'All data at rest is encrypted using AES-256 encryption. This includes databases, file storage, and backups. Keys are rotated quarterly and stored in a hardware security module (HSM).',
  'Review annually',
  '["a1b2c3d4-e5f6-4789-0123-456789abcdef"]',
  'security-team',
  '2024-01-15T00:00:00Z',
  '["security","encryption","data-protection"]',
  'manual',
  'd4e5f6a7b8c9012345678901234567890abcdef123456789abcdef0123456789',
  '2024-02-15T12:00:00Z',
  '2024-02-15T12:00:00Z'
);
```

---

### `audit_event`

Immutable audit log of all vault operations.

| Column       | Type    | Constraints              | Description                          |
|--------------|---------|--------------------------|--------------------------------------|
| seq          | INTEGER | PRIMARY KEY AUTOINCREMENT| Sequential event number              |
| event_id     | TEXT    | NOT NULL                 | UUID v4                              |
| vault_id     | TEXT    | NOT NULL, FK(vault)      | Parent vault                         |
| occurred_at  | TEXT    | NOT NULL                 | ISO 8601 timestamp                   |
| actor        | TEXT    | NOT NULL                 | User/system that triggered event     |
| event_type   | TEXT    | NOT NULL                 | Event type code                      |
| payload_json | TEXT    | NOT NULL                 | JSON event details                   |
| prev_hash    | TEXT    | NOT NULL                 | Hash of previous event (chain)       |
| hash         | TEXT    | NOT NULL                 | Hash of this event                   |

**Indexes:**
- `idx_audit_vault_seq` on `(vault_id, seq)`

**Foreign Keys:**
- `vault_id` → `vault(vault_id)`

**Event Types:**
- `VAULT_CREATED`
- `EVIDENCE_ADDED`
- `QUESTIONNAIRE_IMPORTED`
- `COLUMN_MAPPING_SAVED`
- `ANSWER_BANK_CREATED`
- `ANSWER_BANK_UPDATED`
- `ANSWER_BANK_DELETED`
- `EXPORT_PACK_GENERATED`
- `LICENSE_INSTALLED`

**Hash Chain:**
Each event's `hash` is computed from:
```
SHA-256(prev_hash + event_id + occurred_at + actor + event_type + payload_json)
```

This creates a tamper-evident chain of events.

**Example:**
```sql
INSERT INTO audit_event (
  event_id, vault_id, occurred_at, actor, event_type, payload_json,
  prev_hash, hash
) VALUES (
  'e5f6a7b8-c901-4234-5678-901cdef01234',
  'f47ac10b-58cc-4372-a567-0e02b2c3d479',
  '2024-02-15T12:30:00Z',
  'admin@example.com',
  'ANSWER_BANK_CREATED',
  '{"entry_id":"c3d4e5f6-a7b8-4901-2345-678901cdef01","question":"What encryption..."}',
  'f1e2d3c4b5a6978890abcdef1234567890abcdef1234567890abcdef123456',
  '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef'
);
```

---

## Migration Order

Migrations are applied sequentially in filename order:

1. **0001_init.sql** - Initial schema: vault, evidence_item, audit_event, answer_bank (basic)
2. **0002_add_license.sql** - Add license table (Phase 3)
3. **0003_license_verification.sql** - Add license verification fields
4. **0004_questionnaire_import.sql** - Add questionnaire_import and questionnaire_import_column tables
5. **0005_answer_bank_crud.sql** - Add notes, source, content_hash, created_at, updated_at to answer_bank
6. **0006_matching.sql** - Add matching result tables (Phase 3)

**Migration Strategy:**
- Migrations are idempotent (use `IF NOT EXISTS`, `IF EXISTS`)
- Each migration updates `schema_version` table
- Foreign keys are enforced
- Migrations run automatically on vault open

---

## Indexes

All indexes are created during migrations for optimal query performance.

### Performance Considerations

1. **Evidence Lookups:**
   - By vault: `idx_evidence_vault`
   - By hash (deduplication): `idx_evidence_sha256`

2. **Questionnaire Imports:**
   - By vault: `idx_qna_import_vault`
   - Columns by import: `idx_qna_cols_import`

3. **Answer Bank:**
   - By content hash (deduplication): `idx_answer_bank_content_hash`
   - By question (search): `idx_answer_bank_question`
   - By vault and question (composite): `idx_answer_bank_vault_question`

4. **Audit Log:**
   - By vault and sequence (chronological): `idx_audit_vault_seq`

---

## Data Types & Formats

### Timestamps
All timestamps use ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`

**Example:** `2024-02-15T12:30:45Z`

### UUIDs
All identifiers use UUID v4 (random):

**Example:** `f47ac10b-58cc-4372-a567-0e02b2c3d479`

### JSON Fields

JSON fields are stored as TEXT and parsed by the application.

**tags_json:**
```json
["security", "encryption", "compliance"]
```

**evidence_links_json:**
```json
["evidence-id-1", "evidence-id-2"]
```

**column_map_json:**
```json
{
  "question": "A",
  "answer": "B",
  "notes": "C"
}
```

**sample_json:**
```json
["Sample value 1", "Sample value 2", "Sample value 3"]
```

### Hashes

All hashes use SHA-256, represented as 64-character hex strings.

**Example:** `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

---

## Constraints & Rules

### Foreign Keys

Foreign keys are enabled and enforced:
```sql
PRAGMA foreign_keys = ON;
```

Deleting a vault will cascade delete:
- All evidence items
- All questionnaire imports (and their columns)
- All answer bank entries
- All audit events

### Soft Deletes

Evidence items use soft deletes via `deleted_at` timestamp. Deleted items:
- Have `deleted_at` set to current timestamp
- Are excluded from normal queries
- Remain in database for audit purposes

### Content Hashing

Answer bank entries use `content_hash` for deduplication:
```
SHA-256(question_canonical + answer_short + answer_long)
```

This prevents duplicate entries with identical content.

---

## Query Examples

### Get all active evidence for a vault
```sql
SELECT * FROM evidence_item
WHERE vault_id = ?
  AND deleted_at IS NULL
ORDER BY created_at DESC;
```

### Get column profiles for an import
```sql
SELECT col_ref, ordinal, label, non_empty_count, sample_json
FROM questionnaire_import_column
WHERE import_id = ?
ORDER BY ordinal;
```

### Search answer bank by keyword
```sql
SELECT * FROM answer_bank
WHERE vault_id = ?
  AND (
    question_canonical LIKE '%' || ? || '%'
    OR answer_short LIKE '%' || ? || '%'
    OR answer_long LIKE '%' || ? || '%'
  )
ORDER BY updated_at DESC
LIMIT 20 OFFSET 0;
```

### Get recent audit events
```sql
SELECT * FROM audit_event
WHERE vault_id = ?
ORDER BY seq DESC
LIMIT 100;
```

### Verify audit chain integrity
```sql
WITH RECURSIVE chain AS (
  SELECT seq, hash, prev_hash FROM audit_event
  WHERE vault_id = ? AND seq = 1
  UNION ALL
  SELECT e.seq, e.hash, e.prev_hash
  FROM audit_event e
  JOIN chain c ON e.prev_hash = c.hash
  WHERE e.vault_id = ?
)
SELECT COUNT(*) as valid_events FROM chain;
```

---

## Backup & Recovery

### Backup Strategy

1. **Database Backup:**
   ```bash
   sqlite3 vault.db ".backup vault-backup.db"
   ```

2. **Full Vault Backup:**
   ```bash
   tar -czf vault-backup.tar.gz /path/to/vault-root/
   ```

### Recovery

1. **Database Recovery:**
   ```bash
   sqlite3 vault.db ".restore vault-backup.db"
   ```

2. **Corruption Check:**
   ```bash
   sqlite3 vault.db "PRAGMA integrity_check;"
   ```

### Data Integrity

- Foreign keys prevent orphaned records
- Audit log provides tamper evidence
- Content hashes verify data integrity
- Soft deletes preserve history

---

## Future Enhancements (Phase 3)

1. **Encryption:**
   - Add `encryption_key_id` to vault table
   - Add `encrypted_data` BLOB field to evidence_item
   - Add `encryption_metadata_json` to vault table

2. **Matching Results:**
   - Add `matching_result` table
   - Link questionnaire rows to answer bank entries
   - Store confidence scores and explanations

3. **Advanced Search:**
   - Add full-text search indexes (FTS5)
   - Add trigram indexes for fuzzy matching

4. **Versioning:**
   - Add `entry_version` table for answer bank history
   - Track all changes with diffs
