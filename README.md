# Compliance Ops Suite

Local-first compliance suite monorepo.

## Quickstart
1. Install dependencies:
   `pnpm install`
2. Run verification:
   `pnpm lint`
   `pnpm typecheck`
   `pnpm test`
   `pnpm build`

## Structure
- `core/`: Rust domain + storage + audit chain + deterministic export packs
- `apps/*`: desktop apps (Tauri + React) that call into `core/`
- `packages/*`: shared TypeScript packages (DTOs, UI)
