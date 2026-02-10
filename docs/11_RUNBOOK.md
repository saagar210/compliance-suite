# Runbook (Smoke Tests + Demo Script)

## Prerequisites
- Node + pnpm installed (repo specifies versions)
- Rust toolchain installed (via rust-toolchain.toml)
- No network required

## Setup
1. Clone repo
2. Install deps:
   - `pnpm install`
3. Build everything:
   - `pnpm build`

## Smoke: Core integrity
1. Create a vault (via app UI or CLI if provided)
2. Import 2 evidence files from `fixtures/evidence`
3. Generate export pack (audit pack or custom pack)
4. Validate pack:
   - ensure manifest exists
   - run `export_validate_pack` command if provided
5. Tamper test:
   - modify one exported file
   - re-run validation
   - expect failure (`HASH_MISMATCH`)

## Smoke: Questionnaire round-trip
1. Open Questionnaire app
2. Import `fixtures/questionnaires/sample_a.xlsx`
3. Map columns using `fixtures/questionnaires/column_map_a.json` as guidance
4. Run matching
5. Accept a few suggestions + override one manually
6. Export filled questionnaire to a temp folder
7. Generate questionnaire response pack (if enabled)

Expected:
- export file exists and is filled
- pack includes index + manifest + evidence references
- audit log shows operations in order

## Demo script (recordable)
- Create vault (name: DemoVault)
- Add evidence: policy_sample.pdf + screenshot_sample.png
- Import sample_a.xlsx
- Run matching and show confidence/explanations
- Export filled spreadsheet
- Generate response pack
- Validate pack and show manifest hashes
