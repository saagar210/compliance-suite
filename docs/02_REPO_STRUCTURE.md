# Repository Structure and Module Boundaries (Exact)

## Root layout (target)
```
compliance-suite/
  AGENTS.md
  README.md
  pnpm-workspace.yaml
  package.json
  pnpm-lock.yaml
  rust-toolchain.toml
  Cargo.toml                    # Rust workspace
  .github/workflows/ci.yml

  core/
    Cargo.toml
    src/
      lib.rs
      prelude.rs

      domain/                   # Pure domain types + invariants
        mod.rs
        ids.rs
        time.rs
        events.rs
        errors.rs
        license.rs

      storage/                  # SQLite + filesystem store
        mod.rs
        db.rs
        migrations/
          0001_init.sql
          0002_add_license.sql
        evidence_fs.rs
        tx.rs                    # transactional helpers

      audit/                    # event log + hash chain validation
        mod.rs
        hasher.rs
        canonical.rs            # canonical serialization for hashing
        validator.rs

      export/                   # deterministic export pack generation
        mod.rs
        manifest.rs
        index.rs
        pack.rs
        determinism.rs

      questionnaire/            # domain + matching (app can reuse later)
        mod.rs
        normalize.rs
        matcher.rs
        answer_bank.rs
        io_xlsx.rs              # XLSX parsing/writing (if kept in core)
        io_csv.rs

      sop/                      # SOP domain primitives
        mod.rs
        workflow.rs
        diff.rs

      binder/                   # controls + mappings primitives
        mod.rs
        controls.rs
        mapping.rs

      util/
        mod.rs
        fs.rs
        zip.rs
        redact.rs

    tests/
      audit_chain_tests.rs
      export_pack_golden_tests.rs
      migrations_tests.rs

  packages/
    types/
      package.json
      src/
        index.ts
        dto.ts                  # DTO definitions mirrored from Rust
        errors.ts               # error codes
        validators.ts           # Zod schemas
    ui/
      package.json
      src/
        components/
        hooks/
        styles/

  apps/
    questionnaire/
      package.json
      src/
        app/
        features/
        routes/
        state/                  # Zustand stores
        api/                    # thin wrapper over Tauri invoke
        components/
      src-tauri/
        Cargo.toml
        src/
          main.rs
          commands/
            mod.rs
            vault.rs
            questionnaire.rs
            export.rs
            license.rs
          error_map.rs
          app_state.rs

    binder/
      (same structure as questionnaire app)
    sop/
      (same structure as questionnaire app)

  fixtures/
    questionnaires/
      sample_a.xlsx
      sample_b.xlsx
      column_map_a.json
      column_map_b.json
    evidence/
      policy_sample.pdf
      screenshot_sample.png
    golden_export/
      expected_manifest.json
      expected_index.md
      expected_tree.txt

  docs/
    00_OVERVIEW.md
    01_ARCHITECTURE.md
    02_REPO_STRUCTURE.md
    03_DATA_MODEL.md
    04_API_CONTRACTS.md
    05_IMPLEMENTATION_ORDER.md
    06_ERROR_HANDLING.md
    07_TESTING.md
    08_SECURITY_PRIVACY.md
    09_ASSUMPTIONS_DECISIONS.md
    10_DEFINITION_OF_DONE.md
    11_RUNBOOK.md
```

## Boundaries enforced by review
- `core/` must not depend on any Tauri crate.
- `apps/*/src-tauri` may depend on `core` but must not include deep domain logic.
- `packages/types` mirrors DTOs and errors. If DTOs change, both Rust and TS must be updated in the same PR.

## Workspace config
- `pnpm-workspace.yaml` includes `apps/*` and `packages/*`
- `Cargo.toml` defines a Rust workspace with members: `core`, and each app’s `src-tauri` crate.

## Scripts (must exist at root)
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- `pnpm format`

Each app may also define scoped scripts; root scripts orchestrate all.
