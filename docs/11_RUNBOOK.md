# Runbook - Compliance Suite

## Prerequisites

### System Requirements
- **Node.js**: v18+ with npm
- **Rust**: Stable toolchain (managed via rust-toolchain.toml)
- **SQLite3**: Required for database operations
- **OS**: Linux, macOS, or Windows

### Required Tools
- `cargo` (Rust package manager)
- `npm` (Node package manager)
- `sqlite3` command-line tool

## Installation & Setup

### 1. Clone Repository
```bash
git clone <repository-url>
cd compliance-suite
```

### 2. Install Dependencies

#### Rust Dependencies
```bash
cargo build
```

#### Node Dependencies
```bash
cd apps/questionnaire
npm install
cd ../..
```

### 3. Build All Components

#### Build Rust Core Library
```bash
cargo build --release -p core
```

#### Build Questionnaire App (Frontend)
```bash
cd apps/questionnaire
npm run build
cd ../..
```

#### Build Tauri Desktop App
```bash
cd apps/questionnaire
npm run tauri build
cd ../..
```

Or use the convenience script:
```bash
bash scripts/build.sh
```

## Running the Application

### Development Mode

#### Start Tauri Development Server
```bash
cd apps/questionnaire
npm run tauri dev
```

Or use the demo script:
```bash
bash scripts/demo.sh
```

This will:
1. Build the Rust core library
2. Launch the Tauri development server
3. Open the desktop application

#### Frontend-Only Development
```bash
cd apps/questionnaire
npm run dev
```

Note: In frontend-only mode, Tauri commands will be mocked.

### Production Mode
After building, the compiled application will be located at:
- **Linux**: `apps/questionnaire/src-tauri/target/release/compliance-questionnaire`
- **macOS**: `apps/questionnaire/src-tauri/target/release/bundle/macos/`
- **Windows**: `apps/questionnaire/src-tauri/target/release/compliance-questionnaire.exe`

## Testing

### Run All Tests
```bash
bash scripts/test.sh
```

### Rust Tests
```bash
cargo test --workspace
```

### Frontend Tests (Vitest)
```bash
cd apps/questionnaire
npm test
```

### Run Specific Test Suites
```bash
# Component tests only
cd apps/questionnaire
npm test -- ImportForm.test.tsx

# E2E tests only
npm test -- e2e/
```

## Linting & Formatting

### Run Linters
```bash
bash scripts/lint.sh
```

This runs:
- Rust: `cargo fmt --check` + `cargo clippy`
- TypeScript: ESLint on questionnaire app

### Auto-Format Code
```bash
bash scripts/format.sh
```

This runs:
- Rust: `cargo fmt`
- TypeScript: Prettier on all `.ts`, `.tsx`, `.json`, `.css` files

## Smoke Tests

### Test 1: Create Vault and Import Questionnaire

1. **Launch Application**
   ```bash
   bash scripts/demo.sh
   ```

2. **Create Vault**
   - On first launch, you'll be prompted to create or open a vault
   - Choose a directory (e.g., `~/compliance-vault`)
   - Enter vault name (e.g., "DemoVault")

3. **Import Questionnaire**
   - Click "Import Questionnaire" from the sidebar
   - Select test file: `fixtures/questionnaires/sample_a.xlsx`
   - Verify import success toast appears

4. **Map Columns**
   - System auto-detects columns with "question", "answer", "notes" labels
   - Verify correct columns are selected
   - Click "Save Mapping & Continue"

5. **Expected Results**
   - Import status shows "mapped"
   - Column profiles display sample data
   - Database contains import record

### Test 2: Answer Bank CRUD Operations

1. **Navigate to Answer Bank**
   - Click "Answer Bank" from sidebar

2. **Create Entry**
   - Click "Add Entry"
   - Fill in form:
     - Question: "What encryption do you use?"
     - Short Answer: "AES-256"
     - Long Answer: "We use AES-256 encryption for all data at rest..."
     - Tags: "security, encryption"
   - Click "Create Entry"

3. **Verify Entry**
   - Entry appears in table
   - Tags are displayed correctly
   - Owner shows current user

4. **Delete Entry**
   - Click "Delete" button
   - Confirm deletion
   - Entry removed from table

### Test 3: Matching Suggestions

1. **Import Questionnaire** (from Test 1)

2. **Navigate to Review Page**
   - View imported questions

3. **Get Suggestions**
   - System automatically suggests matches from answer bank
   - Verify confidence scores are displayed
   - Check that explanations are provided

4. **Accept/Reject Suggestions**
   - Accept a suggestion
   - Verify answer is populated
   - Reject a suggestion
   - Verify question remains unanswered

### Test 4: Export Pack Generation

1. **Complete Import and Mapping** (from Test 1)

2. **Generate Export**
   - Click "Generate Export Pack"
   - Choose output path (e.g., `~/exports/questionnaire-export.zip`)
   - Click "Generate Export"

3. **Verify Export**
   - ZIP file created at specified path
   - Success toast shows file count
   - Export pack contains:
     - Manifest with file hashes
     - Questionnaire data
     - Evidence references (if any)

4. **Validate Pack Integrity**
   ```bash
   unzip -l ~/exports/questionnaire-export.zip
   # Should show manifest.json and data files
   ```

## Demo Script (Recordable)

This script demonstrates a complete workflow for a demo or tutorial video:

### Setup
```bash
bash scripts/demo.sh
```

### Workflow

1. **Create Vault** (00:00 - 00:30)
   - Choose directory: `~/demo-vault`
   - Name: "Security Compliance Vault"
   - Click "Create Vault"

2. **Import Questionnaire** (00:30 - 01:00)
   - Navigate to "Import Questionnaire"
   - Select `fixtures/questionnaires/sample_a.xlsx`
   - Click "Import Questionnaire"
   - Show success notification

3. **Map Columns** (01:00 - 01:30)
   - Review auto-detected columns:
     - Question → Column A
     - Answer → Column B
     - Notes → Column C
   - Click "Save Mapping & Continue"

4. **Build Answer Bank** (01:30 - 03:00)
   - Navigate to "Answer Bank"
   - Create 3 sample entries:

     **Entry 1:**
     - Question: "What is your data encryption policy?"
     - Short: "AES-256 encryption"
     - Long: "All data at rest is encrypted using AES-256..."
     - Tags: "security, encryption, data"

     **Entry 2:**
     - Question: "How do you handle data breaches?"
     - Short: "Incident response plan"
     - Long: "We have a comprehensive incident response plan..."
     - Tags: "security, incident, compliance"

     **Entry 3:**
     - Question: "What backup procedures do you have?"
     - Short: "Daily automated backups"
     - Long: "Daily automated backups to encrypted cloud storage..."
     - Tags: "backup, disaster-recovery"

5. **Review Matching Suggestions** (03:00 - 04:00)
   - Navigate to "Review"
   - Show matching suggestions for imported questions
   - Highlight confidence scores
   - Explain confidence explanations
   - Accept 2 suggestions
   - Manually override 1 suggestion

6. **Generate Export Pack** (04:00 - 05:00)
   - Click "Generate Export Pack"
   - Choose output: `~/demo-export.zip`
   - Show export progress
   - Verify success: "Export pack created with N files"

7. **Validate Export** (05:00 - 05:30)
   ```bash
   unzip -l ~/demo-export.zip
   cat manifest.json  # Show file hashes
   ```

## Troubleshooting

### Common Issues

#### "No vault open" Error
- **Cause**: Application started without a vault
- **Solution**: Create or open a vault from the welcome screen

#### Import Fails: "Failed to parse file"
- **Cause**: File format not supported or corrupted
- **Solution**: Ensure file is `.xlsx` or `.csv` format
- **Verify**: Open file in Excel/Calc to confirm it's valid

#### SQLite3 Not Found
- **Cause**: `sqlite3` command not in PATH
- **Solution**:
  - macOS: `brew install sqlite`
  - Ubuntu/Debian: `apt-get install sqlite3`
  - Windows: Download from sqlite.org

#### Build Fails: "error: linker `cc` not found"
- **Cause**: C compiler not installed
- **Solution**:
  - Ubuntu/Debian: `apt-get install build-essential`
  - macOS: `xcode-select --install`
  - Windows: Install Visual Studio Build Tools

#### Frontend Tests Fail: "Cannot find module"
- **Cause**: Node modules not installed
- **Solution**:
  ```bash
  cd apps/questionnaire
  npm install
  ```

### Debug Mode

Enable verbose logging:
```bash
export RUST_LOG=debug
bash scripts/demo.sh
```

### Database Inspection

Inspect vault database directly:
```bash
sqlite3 ~/compliance-vault/vault.db
.schema  # Show table schemas
SELECT * FROM vault;  # Show vault info
SELECT * FROM questionnaire_import;  # Show imports
```

## Deployment

### Building for Distribution

#### Linux
```bash
cd apps/questionnaire
npm run tauri build
# Output: src-tauri/target/release/bundle/appimage/
```

#### macOS
```bash
cd apps/questionnaire
npm run tauri build
# Output: src-tauri/target/release/bundle/dmg/
```

#### Windows
```bash
cd apps/questionnaire
npm run tauri build
# Output: src-tauri/target/release/bundle/msi/
```

### Configuration

Application configuration is stored in:
- **Vault metadata**: `<vault-root>/vault.db`
- **Tauri config**: `apps/questionnaire/src-tauri/tauri.conf.json`

## Performance Benchmarks

Expected performance on reference hardware (Intel i5, 8GB RAM):
- **Vault creation**: < 1 second
- **Import 1000-row questionnaire**: 2-5 seconds
- **Column mapping save**: < 500ms
- **Matching suggestions (per question)**: < 100ms
- **Export pack generation**: 3-10 seconds (depends on file count)

## Support

For issues or questions:
1. Check this runbook for common solutions
2. Review `docs/` directory for detailed documentation
3. Search existing GitHub issues
4. Create a new issue with logs and reproduction steps
