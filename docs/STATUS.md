# Project Status

_Last updated: 2026-02-10_

## Current phase + slice
- Phase 1.5 Licensing

## What is done
- Phase 0 scaffold and guardrails (workspace layout, scripts, CI parity)
- Phase 1.1 SQLite + migrations (core)
- Phase 1.2 Evidence filesystem import (core)
- Phase 1.3 Audit event store + hash chain validation (core)
- Phase 1.4 Deterministic export packs + golden tests (core)

## What is in progress
- Phase 1.5 Licensing (signed license verification, storage, audit events, feature gating)

## Next steps (ordered)
1. Decide and document dependency policy (shell-out boundary + Windows unblock plan)
2. Decide and document platform scope for release 0.1
3. Implement licensing verification + storage + audit events in `core/`
4. Implement app command gating + DTOs mirrored in `packages/types`
5. Add tests + fixtures for valid/invalid licensing flows

## Verification summary
- Last run (2026-02-10): PASS
- Commands: `pnpm lint`, `pnpm typecheck`, `pnpm test`, `pnpm build`
