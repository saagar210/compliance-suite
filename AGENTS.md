# AGENTS.md — Compliance Ops Suite

## Operating rules (hard requirements)
- Keep `main` green at all times.
- Use git worktrees: each task gets its own worktree + branch.
- Offline-first: do not enable network access.
- Do not add telemetry.
- Do not add dependencies without explicit justification and review.

## Module boundaries
- `/core` contains domain logic, integrity features, storage, export engine, licensing. No Tauri runtime dependencies.
- `/apps/*/src-tauri` exposes commands and wires UI to core. No deep domain logic.
- `/packages/types` mirrors DTOs and error codes; must stay in sync with Rust.

## Verification gate (required in every PR)
1) Done: what changed + why
2) Files changed
3) Verification: commands run + results (use repo scripts; add scripts if missing)
4) Risks / follow-ups
5) Status: phase + complete/in progress/blocked (+% only if a clear scope exists)
6) Next steps

## Quality bar
- Deterministic export packs with golden tests.
- Tamper-evident audit log with validation tests.
- No silent defaults for required fields.
