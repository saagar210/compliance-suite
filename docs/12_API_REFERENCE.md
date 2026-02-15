# API Reference - Tauri Commands

This document provides a complete reference for all Tauri commands available in the Compliance Suite Questionnaire application.

## Table of Contents

1. [Vault Commands](#vault-commands)
2. [Questionnaire Commands](#questionnaire-commands)
3. [Answer Bank Commands](#answer-bank-commands)
4. [Matching Commands](#matching-commands)
5. [Export Commands](#export-commands)
6. [License Commands](#license-commands)
7. [Error Handling](#error-handling)

---

## Vault Commands

### `vault_create`

Creates a new compliance vault at the specified path.

**Parameters:**
```typescript
{
  path: string;      // Absolute path where vault will be created
  name: string;      // Human-readable vault name
}
```

**Returns:**
```typescript
{
  vault_id: string;           // Unique vault identifier (UUID)
  name: string;               // Vault name
  root_path: string;          // Absolute path to vault root
  encryption_mode: string;    // "none" | "aes256" (Phase 3)
  schema_version: number;     // Database schema version
}
```

**Errors:**
- `"Vault already exists at path"` - Vault directory already exists
- `"Permission denied"` - Insufficient permissions to create directory
- `"Invalid path"` - Path is invalid or inaccessible

**Example:**
```typescript
import { invoke } from "@tauri-apps/api/core";

const vault = await invoke("vault_create", {
  path: "/home/user/compliance-vault",
  name: "Security Compliance Vault"
});

console.log(`Created vault: ${vault.vault_id}`);
```

---

### `vault_open`

Opens an existing vault at the specified path.

**Parameters:**
```typescript
{
  path: string;  // Absolute path to vault directory
}
```

**Returns:**
```typescript
{
  vault_id: string;
  name: string;
  root_path: string;
  encryption_mode: string;
  schema_version: number;
}
```

**Errors:**
- `"Vault not found"` - No vault exists at path
- `"Corrupt vault"` - Vault database is corrupted
- `"Schema version mismatch"` - Vault schema is incompatible

**Example:**
```typescript
const vault = await invoke("vault_open", {
  path: "/home/user/compliance-vault"
});
```

---

### `vault_close`

Closes the currently open vault.

**Parameters:** None

**Returns:** `null`

**Example:**
```typescript
await invoke("vault_close");
```

---

### `vault_lock`

Locks the vault (closes it in Phase 2, encrypts in Phase 3).

**Parameters:** None

**Returns:** `null`

**Example:**
```typescript
await invoke("vault_lock");
```

---

## Questionnaire Commands

### `import_questionnaire`

Imports a questionnaire file (Excel or CSV) into the vault.

**Parameters:**
```typescript
{
  file_path: string;  // Absolute path to .xlsx or .csv file
}
```

**Returns:**
```typescript
{
  import_id: string;          // Unique import identifier (UUID)
  vault_id: string;           // Parent vault ID
  source_filename: string;    // Original filename
  source_sha256: string;      // SHA-256 hash of source file
  imported_at: string;        // ISO 8601 timestamp
  format: string;             // "csv" | "xlsx"
  status: string;             // "imported" | "mapped" | "processed"
  column_map: {               // null until columns are mapped
    question: string;
    answer: string;
    notes?: string;
  } | null;
}
```

**Errors:**
- `"No vault open"` - Must open/create vault first
- `"File not found"` - Specified file doesn't exist
- `"Unsupported format"` - File is not .xlsx or .csv
- `"Failed to parse file"` - File is corrupted or invalid

**Example:**
```typescript
const importData = await invoke("import_questionnaire", {
  file_path: "/path/to/questionnaire.xlsx"
});

console.log(`Imported ${importData.import_id}`);
```

---

### `get_column_profiles`

Retrieves column profiles for an imported questionnaire to assist with mapping.

**Parameters:**
```typescript
{
  import_id: string;  // Import ID from import_questionnaire
}
```

**Returns:**
```typescript
Array<{
  col_ref: string;          // Column reference (e.g., "A", "B" for Excel)
  ordinal: number;          // Column position (0-based)
  label: string;            // Column header/label
  non_empty_count: number;  // Number of non-empty cells
  sample: string[];         // Sample values (up to 5)
}>
```

**Errors:**
- `"No vault open"`
- `"Import not found"` - Invalid import_id
- `"Import already mapped"` - Columns already mapped

**Example:**
```typescript
const profiles = await invoke("get_column_profiles", {
  import_id: "abc-123-def"
});

profiles.forEach(profile => {
  console.log(`${profile.label}: ${profile.sample.slice(0, 2).join(", ")}...`);
});
```

---

### `save_column_mapping`

Saves the column mapping for an imported questionnaire.

**Parameters:**
```typescript
{
  import_id: string;
  column_map: {
    question: string;    // Column reference for questions
    answer: string;      // Column reference for answers
    notes?: string;      // Optional column reference for notes
  };
}
```

**Returns:**
```typescript
{
  import_id: string;
  vault_id: string;
  source_filename: string;
  source_sha256: string;
  imported_at: string;
  format: string;
  status: string;        // Updated to "mapped"
  column_map: {
    question: string;
    answer: string;
    notes?: string;
  };
}
```

**Errors:**
- `"No vault open"`
- `"Import not found"`
- `"Invalid column mapping"` - Missing required fields
- `"Column not found"` - Referenced column doesn't exist

**Example:**
```typescript
const updated = await invoke("save_column_mapping", {
  import_id: "abc-123-def",
  column_map: {
    question: "A",
    answer: "B",
    notes: "C"
  }
});

console.log(`Mapping saved, status: ${updated.status}`);
```

---

## Answer Bank Commands

### `answer_bank_create`

Creates a new answer bank entry.

**Parameters:**
```typescript
{
  input: {
    question_canonical: string;    // Canonical question text
    answer_short: string;          // Brief answer (1-2 sentences)
    answer_long: string;           // Detailed answer
    notes?: string;                // Optional notes
    evidence_links: string[];      // Array of evidence_ids
    owner: string;                 // Entry owner/author
    last_reviewed_at?: string;     // ISO 8601 timestamp
    tags: string[];                // Tags for categorization
    source: string;                // "manual" | "import" | "suggestion"
  }
}
```

**Returns:**
```typescript
{
  entry_id: string;              // Unique entry ID (UUID)
  vault_id: string;
  question_canonical: string;
  answer_short: string;
  answer_long: string;
  notes?: string;
  evidence_links: string[];
  owner: string;
  last_reviewed_at?: string;
  tags: string[];
  source: string;
  content_hash: string;          // SHA-256 of canonical content
  created_at: string;            // ISO 8601 timestamp
  updated_at: string;            // ISO 8601 timestamp
}
```

**Errors:**
- `"No vault open"`
- `"Invalid input"` - Missing required fields
- `"Duplicate entry"` - Entry with same content_hash exists

**Example:**
```typescript
const entry = await invoke("answer_bank_create", {
  input: {
    question_canonical: "What encryption do you use?",
    answer_short: "AES-256 encryption",
    answer_long: "We use AES-256 encryption for all data at rest...",
    notes: "Review annually",
    evidence_links: [],
    owner: "security-team",
    tags: ["security", "encryption"],
    source: "manual"
  }
});
```

---

### `answer_bank_list`

Lists answer bank entries with pagination.

**Parameters:**
```typescript
{
  limit: number;   // Maximum entries to return (default: 20)
  offset: number;  // Number of entries to skip (default: 0)
}
```

**Returns:**
```typescript
Array<{
  entry_id: string;
  vault_id: string;
  question_canonical: string;
  answer_short: string;
  answer_long: string;
  notes?: string;
  evidence_links: string[];
  owner: string;
  last_reviewed_at?: string;
  tags: string[];
  source: string;
  content_hash: string;
  created_at: string;
  updated_at: string;
}>
```

**Errors:**
- `"No vault open"`
- `"Invalid pagination parameters"` - limit or offset out of range

**Example:**
```typescript
// Get first 20 entries
const entries = await invoke("answer_bank_list", {
  limit: 20,
  offset: 0
});

// Get next 20 entries
const nextEntries = await invoke("answer_bank_list", {
  limit: 20,
  offset: 20
});
```

---

### `answer_bank_update`

Updates an existing answer bank entry.

**Parameters:**
```typescript
{
  entry_id: string;
  patch: {
    question_canonical?: string;
    answer_short?: string;
    answer_long?: string;
    notes?: string | null;
    evidence_links?: string[];
    owner?: string;
    last_reviewed_at?: string | null;
    tags?: string[];
    source?: string;
  }
}
```

**Returns:**
```typescript
{
  entry_id: string;
  // ... full entry with updated fields
  updated_at: string;  // Updated to current timestamp
}
```

**Errors:**
- `"No vault open"`
- `"Entry not found"`
- `"Invalid patch"` - Invalid field values

**Example:**
```typescript
const updated = await invoke("answer_bank_update", {
  entry_id: "entry-123",
  patch: {
    answer_short: "Updated brief answer",
    tags: ["security", "encryption", "updated"]
  }
});
```

---

### `answer_bank_delete`

Deletes an answer bank entry.

**Parameters:**
```typescript
{
  entry_id: string;
}
```

**Returns:** `null`

**Errors:**
- `"No vault open"`
- `"Entry not found"`

**Example:**
```typescript
await invoke("answer_bank_delete", {
  entry_id: "entry-123"
});
```

---

## Matching Commands

### `get_matching_suggestions`

Gets answer suggestions for a question using the matching algorithm.

**Parameters:**
```typescript
{
  question: string;      // Question text to match
  top_n?: number;        // Number of suggestions to return (default: 5)
}
```

**Returns:**
```typescript
Array<{
  answer_bank_entry_id: string;     // Matched entry ID
  score: number;                     // Confidence score (0.0 - 1.0)
  normalized_question: string;       // Normalized question text
  normalized_answer: string;         // Normalized answer text
  confidence_explanation: string;    // Explanation of match confidence
}>
```

**Errors:**
- `"No vault open"`
- `"Answer bank is empty"` - No entries to match against

**Example:**
```typescript
const suggestions = await invoke("get_matching_suggestions", {
  question: "What is your data encryption policy?",
  top_n: 3
});

suggestions.forEach(s => {
  console.log(`Match: ${s.score.toFixed(2)} - ${s.confidence_explanation}`);
});
```

---

## Export Commands

### `generate_export_pack`

Generates an export pack (ZIP file) containing questionnaire data and evidence.

**Parameters:**
```typescript
{
  output_path: string;  // Absolute path where ZIP will be created
}
```

**Returns:**
```typescript
{
  zip_path: string;         // Path to created ZIP file
  manifest_version: number; // Manifest format version
  file_count: number;       // Number of files in pack
}
```

**Errors:**
- `"No vault open"`
- `"License required"` - EXPORT_PACKS feature not licensed
- `"Permission denied"` - Cannot write to output path
- `"Disk full"` - Insufficient disk space

**Example:**
```typescript
const exportPack = await invoke("generate_export_pack", {
  output_path: "/home/user/exports/compliance-export.zip"
});

console.log(`Created export with ${exportPack.file_count} files`);
```

---

## License Commands

### `check_license_status`

Checks the current license status for the vault.

**Parameters:** None

**Returns:**
```typescript
{
  installed: boolean;                 // License file present
  valid: boolean;                     // License is valid
  license_id?: string;                // License identifier
  features: string[];                 // Enabled features
  verification_status?: string;       // Verification details
}
```

**Example:**
```typescript
const status = await invoke("check_license_status");

if (status.valid && status.features.includes("EXPORT_PACKS")) {
  console.log("Export packs feature enabled");
}
```

---

### `install_license`

Installs a license file into the vault.

**Parameters:**
```typescript
{
  license_path: string;  // Path to license.json file
}
```

**Returns:**
```typescript
{
  installed: boolean;
  valid: boolean;
  license_id?: string;
  features: string[];
  verification_status?: string;
}
```

**Errors:**
- `"No vault open"`
- `"License file not found"`
- `"Invalid license format"`
- `"License expired"`
- `"License signature invalid"`

**Example:**
```typescript
const status = await invoke("install_license", {
  license_path: "/path/to/license.json"
});

console.log(`Installed license: ${status.license_id}`);
console.log(`Features: ${status.features.join(", ")}`);
```

---

## Error Handling

All Tauri commands return errors as strings. The frontend should handle errors appropriately.

### Common Error Patterns

```typescript
try {
  const result = await invoke("vault_create", { path, name });
  // Handle success
} catch (error) {
  // error is a string
  if (error.includes("already exists")) {
    // Handle duplicate vault
  } else if (error.includes("Permission denied")) {
    // Handle permission error
  } else {
    // Handle generic error
    console.error("Unexpected error:", error);
  }
}
```

### Error Categories

1. **Vault Errors**
   - `"No vault open"` - Operation requires an open vault
   - `"Vault not found"` - Vault doesn't exist at path
   - `"Corrupt vault"` - Vault database is corrupted

2. **Resource Errors**
   - `"[Resource] not found"` - Entity doesn't exist
   - `"Duplicate entry"` - Entity already exists

3. **Validation Errors**
   - `"Invalid [field]"` - Input validation failed
   - `"Missing required field"` - Required field not provided

4. **Permission Errors**
   - `"Permission denied"` - Insufficient file system permissions
   - `"License required"` - Feature requires valid license

5. **System Errors**
   - `"File not found"` - Referenced file doesn't exist
   - `"Disk full"` - Insufficient disk space
   - `"Database error"` - SQLite operation failed

### Best Practices

1. **Always check for open vault** before vault-dependent operations
2. **Validate inputs** on the frontend before invoking commands
3. **Handle errors gracefully** with user-friendly messages
4. **Log errors** for debugging and support
5. **Use TypeScript types** from `@packages/types` for type safety

### Example: Robust Error Handling

```typescript
import { invoke } from "@tauri-apps/api/core";

async function safeImport(filePath: string) {
  try {
    const result = await invoke("import_questionnaire", { file_path: filePath });
    return { success: true, data: result };
  } catch (error) {
    const errorMsg = String(error);

    if (errorMsg.includes("No vault open")) {
      return {
        success: false,
        error: "Please open or create a vault first"
      };
    } else if (errorMsg.includes("File not found")) {
      return {
        success: false,
        error: "The selected file could not be found"
      };
    } else if (errorMsg.includes("Unsupported format")) {
      return {
        success: false,
        error: "Please select an Excel (.xlsx) or CSV (.csv) file"
      };
    } else {
      return {
        success: false,
        error: `Import failed: ${errorMsg}`
      };
    }
  }
}
```

---

## Type Definitions

For complete TypeScript type definitions, see:
- `packages/types/src/index.ts` - Frontend types
- Command DTOs in Rust source match these types exactly

All DTOs use:
- ISO 8601 timestamps for dates
- UUIDs (v4) for identifiers
- JSON for complex fields (tags, evidence_links, etc.)
