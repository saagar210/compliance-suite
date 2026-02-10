# Architecture Decisions and Rationale

## Executive summary
Build a monorepo containing:
- A **shared Rust core** crate (`core/`) that implements the vault domain: storage, audit events, export packs, licensing.
- Three **Tauri apps** in `apps/*` that consume core via Rust crate imports and expose a stable command API to the UI.
- A shared TypeScript package for **types**, **validation**, and **UI primitives**.

This yields a “product suite” with clean boundaries:
- Domain logic lives in Rust core and is unit tested.
- UI is thin: renders state and calls commands; no business rules duplicated in TS.
- Apps remain separate deliverables but reuse the same hardened core.

## Architecture diagram (logical)

UI (React/TS)  <—invoke—>  Tauri Commands (Rust)  —calls—>  Core Platform (Rust crates)
     |                                  |                               |
 Zustand/TanStack Query            Error mapping                      SQLite / Filesystem
     |
 View models + forms

## Key decisions (with rationale)

### D1. Monorepo with shared core
**Decision:** Single repository with `/core` + `/apps/*` + `/packages/*`.
**Rationale:** Reuse, consistent quality gates, and shared release tooling. Avoids copy/paste drift.
**Tradeoff:** Monorepo complexity. Mitigated by strict module boundaries and CI scopes.

### D2. Shared Rust core crate(s)
**Decision:** Core business logic in Rust, independent from Tauri runtime.
**Rationale:** Portable across apps, easier to test, strongest “portfolio flex” for integrity features.
**Tradeoff:** More Rust surface area. Mitigated by narrow APIs and “command layer” wrappers.

### D3. SQLite as the primary store
**Decision:** Structured data in SQLite, binary evidence stored as files in the vault directory.
**Rationale:** SQLite is local-first, reliable, queryable, and works cross-platform.
**Tradeoff:** Schema migrations. Mitigated by a migration system and versioned schemas.

### D4. Evidence files stored outside SQLite
**Decision:** Evidence bytes as files; SQLite stores metadata + hashes + paths.
**Rationale:** Avoids DB bloat and simplifies deterministic export pack generation.
**Tradeoff:** Need atomic file operations. Mitigated by temp files + rename and checksums.

### D5. Tamper-evident audit log implemented as append-only hash chain
**Decision:** Append-only event store: each event includes `prev_hash` and `hash` over canonical event content.
**Rationale:** Provides integrity without needing a server. Easy to validate and demo.
**Tradeoff:** Not a legal signature alone. Mitigated by optional signed exports and “trust boundaries” doc.

### D6. Deterministic export packs
**Decision:** Export engine produces stable, ordered bundles with a manifest of hashes and a human-readable index.
**Rationale:** Repeatability + audit credibility; prevents “what changed?” ambiguity.
**Tradeoff:** Must define determinism policy (timestamps, ordering). See `08_SECURITY_PRIVACY.md`.

### D7. Offline licensing via signed license payloads
**Decision:** Use an embedded public key to verify a license payload signed by you (Ed25519).
**Rationale:** Offline-first monetization with strong integrity. Also a portfolio flex.
**Tradeoff:** Key management and upgrade path. Mitigated by versioned license schema.

### D8. UI state management
**Decision:** UI uses:
- **Zustand** for local UI state (filters, selections, modal state)
- **TanStack Query** for command calls that load data (cache + invalidation)
- **Zod** for client-side input validation matching shared types

**Rationale:** Predictable, minimal boilerplate, easy to test.
**Tradeoff:** Another dependency layer. Mitigated by keeping domain logic in Rust.

## Module boundaries (non-negotiable)
- `/core` contains all domain rules and data integrity logic. No UI imports.
- `/apps/*/src-tauri` contains app-specific commands and wiring; *no* deep business logic.
- `/packages/types` holds shared TypeScript types that mirror Rust DTOs (generated or maintained with discipline).
- UI components must not compute compliance status; they only display state returned by commands.

## Versioning policy
- Tauri command DTOs are versioned with a `api_version` constant and include `schema_version` where relevant.
- Database schema has a `schema_version` table; migrations are deterministic and idempotent.
