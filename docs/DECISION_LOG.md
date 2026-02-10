# Decision Log

This log records decisions that affect architecture, portability, determinism, integrity, and dependency policy.

## 2026-02-10 — Dependency policy and shell-out boundary
- Decision:
  - Shell-outs are permitted in Phase 1 only behind a single abstraction boundary: `core/util/shell.rs`.
  - All shell-outs must perform a capabilities check (presence + basic version/behavior expectations).
  - New Rust dependencies require a Decision Log entry with rationale and tradeoffs.
  - Windows blockers will be removed in priority order by replacing shell-outs with minimal Rust crates.
- Rationale:
  - Current implementation uses `sqlite3`, `shasum`, `zip/unzip`, and `find/touch` for a fast offline-first slice.
  - A single boundary prevents shell-out sprawl and makes later portability work mechanical.
- Tradeoffs:
  - Shell-outs reduce portability (especially Windows) and can hide OS-level differences.
  - Native crates increase dependency surface area and supply-chain risk.
- Revisit trigger:
  - Any work targeting Windows, or any CI failures due to missing system tools.
  - Before release 0.1 if distribution targets expand beyond macOS.

## 2026-02-10 — Platform scope for release 0.1
- Decision:
  - Release 0.1 targets macOS only.
  - Vault format, audit chain hashing, and export pack contents remain platform-invariant.
- Rationale:
  - Current Phase 1 uses macOS-friendly system tools and `/dev/urandom` for ULID generation.
  - Narrowing the initial platform reduces rework while core formats remain stable.
- Tradeoffs:
  - Windows users are deferred.
  - Some portability decisions (native SQLite/ZIP/HASH) are postponed but must be tracked.
- Revisit trigger:
  - If Windows support is required for any near-term customer or App A distribution.

## 2026-02-10 — Proposed minimal crates for Windows unblock (not yet adopted)
- Decision:
  - Not adopted yet (proposal only).
  - When Windows support becomes a target, replace shell-outs in this order:
    1. Hashing: `sha2` + `digest` (remove `shasum`)
    2. ZIP: `zip` crate (remove `zip/unzip` shell-outs)
    3. SQLite: `rusqlite` + `libsqlite3-sys` (remove `sqlite3` shell-out)
    4. IDs: `ulid` or `uuid` (remove `/dev/urandom` ULID implementation)
    5. JSON: `serde_json` (replace minimal JSON parsing where it becomes complex)
- Rationale:
  - These crates directly remove portability blockers and high-cost process spawning.
- Tradeoffs:
  - Adds dependency footprint and requires careful version pinning.
- Revisit trigger:
  - Any planned Windows deliverable or CI matrix expansion.

## 2026-02-10 — Add Ed25519 verification dependency
- Decision:
  - Add `ed25519-dalek` to `core/` to implement Ed25519 license signature verification.
- Rationale:
  - Phase 1.5 requires signed offline licenses with Ed25519 verification.
  - Implementing Ed25519 safely without a well-maintained crate is not reasonable.
- Tradeoffs:
  - Adds a crypto dependency (supply-chain and update management considerations).
- Revisit trigger:
  - If we adopt a broader crypto strategy (for example `ring`) or need FIPS-aligned primitives.

## 2026-02-10 — Development license fixtures policy
- Decision:
  - Keep `fixtures/licenses/*.json` strictly for automated tests and local development.
  - Do not ship fixtures in production bundles.
  - Replace `core/src/domain/license.rs` embedded public key before release.
- Rationale:
  - Tests require a known-good signed payload without storing any private signing material.
- Tradeoffs:
  - Anyone with access to fixtures can use the dev license unless the public key is rotated.
- Revisit trigger:
  - Before any external distribution or monetization release.

## 2026-02-10 — Add explicit missing capability error code
- Decision:
  - Introduce `MISSING_CAPABILITY` as a stable error code for missing required local tooling (for example `sqlite3`, `shasum`, `zip`, `unzip`, `bash`).
- Rationale:
  - Missing OS tools is a common offline-first failure mode and should not be conflated with corrupt vault data.
  - A dedicated code allows UI/runbooks to show clear “install tool X” guidance.
- Tradeoffs:
  - Adds another stable error code to keep in sync across Rust core and TS DTO mirrors.
- Revisit trigger:
  - When we replace shell-outs with native crates (capability errors should map to dependency presence/config instead).

## 2026-02-10 — Phase 2 dependency + determinism policy clarifications
- Decision:
  - No new dependencies (Rust or TypeScript) without a Decision Log entry (rationale + tradeoffs) and same-PR updates to affected docs/tests/fixtures.
  - TypeScript runtime validation is currently implemented as a temporary stopgap in `packages/types/src/validators.ts` (no Zod yet). Adoption of Zod is deferred until we explicitly approve a dependency update.
  - Persisted JSON blobs that participate in integrity or deterministic workflows must be canonicalized:
    - Stable key ordering (sorted keys) and stable encoding (UTF-8)
    - No inclusion of volatile fields (timestamps, actor) in any hashed/canonical content used for integrity or deterministic exports
- Rationale:
  - Phase 2 introduced persisted questionnaire mapping/profiling JSON; without explicit canonicalization rules, cross-run diffs become noisy and integrity checks become ambiguous.
  - Offline-first constraints require that we are deliberate about dependency introduction (especially runtime validators for UI contracts).
- Tradeoffs:
  - Stopgap validators are less ergonomic than Zod schemas and can diverge if not maintained with discipline.
  - Canonicalization rules can add friction when adding new fields; mitigated by tests and clear DTO mirroring.
- Revisit trigger:
  - When UI form validation is implemented (candidate: adopt Zod with an explicit dependency decision).
  - When export packs start including questionnaire artifacts that depend on mapping/profiling content.
