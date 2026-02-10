# Compliance Ops Suite

Local-first compliance suite monorepo.

## Structure
- `core/`: Rust domain + storage + audit chain + deterministic export packs
- `apps/*`: desktop apps (Tauri + React) that call into `core/`
- `packages/*`: shared TypeScript packages (DTOs, UI)

## Verification
Run root scripts (these are what CI runs):
- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- `pnpm format`

See `docs/11_RUNBOOK.md` for smoke flows.
