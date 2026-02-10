# Integration Points and Testing Strategy

## Testing pyramid
1. **Rust core unit tests**: domain invariants, audit chain, hashing, determinism functions.
2. **Rust integration tests**: SQLite migrations + export pack generation on fixtures.
3. **Golden tests**: export pack structure and manifest content compared to fixture outputs.
4. **UI tests**: component/unit tests for forms and state stores (light).
5. **End-to-end smoke scripts**: scripted demo run with sample fixtures.

## Integration points
### IP1: UI ↔ Tauri commands
- Test: command-level tests in Rust where possible.
- Smoke: UI flows that invoke commands and assert rendered results.

### IP2: Command layer ↔ core crate
- Test: compile-time boundaries + unit tests at core.
- Add a small set of integration tests that call the same core APIs used by commands.

### IP3: Storage ↔ filesystem evidence
- Test: import evidence, verify hashes and paths, simulate interruption (fail mid-copy), ensure cleanup.

### IP4: Export engine ↔ manifest verification
- Test: generate pack, validate pack, tamper one file, ensure validation fails.

### IP5: Questionnaire pipeline (import → match → export)
- Test: fixture questionnaires; verify counts and known expected matches (snapshot expectations).

## CI workflow (minimum)
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- Rust: `cargo test -p core` (and app crates if needed)

## Fixture policy
- Only sanitized documents. No real client data.
- Every fixture has a README describing what it tests.
- Golden export expectations updated only when export format changes intentionally.

## Smoke runbook
See `11_RUNBOOK.md` for step-by-step smoke commands and expected outputs.
