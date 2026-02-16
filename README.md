# Compliance Ops Suite

Local-first compliance suite monorepo.

## Quickstart
1. Install dependencies:
   `pnpm install`
2. Start the desktop app (normal dev):
   `pnpm dev`
3. Start the desktop app in low-disk lean mode:
   `pnpm dev:lean`
4. Run verification:
   `pnpm lint`
   `pnpm typecheck`
   `pnpm test`
   `pnpm build`

## Dev Modes
- Normal dev (`pnpm dev`):
  - Fastest repeated startups.
  - Uses persistent local build outputs (`target/`, `apps/questionnaire/node_modules/.vite`).
- Lean dev (`pnpm dev:lean`):
  - Uses temporary cache locations for Rust and Vite build outputs.
  - Automatically removes heavy build artifacts when the app exits.
  - Keeps dependency installs (`node_modules`, global Cargo/pnpm caches) so restarts stay reasonable.

## Cleanup Commands
- Heavy artifacts only (safe daily cleanup):
  - `pnpm clean:heavy`
  - Removes build outputs such as `target/`, `apps/questionnaire/dist`, and Vite/Tauri generated artifacts.
- Full local reproducible cleanup:
  - `pnpm clean:local`
  - Includes heavy artifacts plus local dependency installs (`node_modules`) and local pnpm store (`.pnpm-store`), all of which can be recreated.

## Structure
- `core/`: Rust domain + storage + audit chain + deterministic export packs
- `apps/*`: desktop apps (Tauri + React) that call into `core/`
- `packages/*`: shared TypeScript packages (DTOs, UI)
