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
