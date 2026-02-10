# Codex Prompt (references this doc pack)

You are working in a monorepo project named "compliance-suite". Read and follow the project documents under `/docs/` and `AGENTS.md`.

**Required reading (in order):**
1. `docs/00_OVERVIEW.md`
2. `docs/01_ARCHITECTURE.md`
3. `docs/02_REPO_STRUCTURE.md`
4. `docs/03_DATA_MODEL.md`
5. `docs/04_API_CONTRACTS.md`
6. `docs/05_IMPLEMENTATION_ORDER.md`
7. `docs/06_ERROR_HANDLING.md`
8. `docs/07_TESTING.md`
9. `docs/08_SECURITY_PRIVACY.md`
10. `docs/09_ASSUMPTIONS_DECISIONS.md`
11. `docs/10_DEFINITION_OF_DONE.md`
12. `docs/11_RUNBOOK.md`

**Primary directive:**
Implement the suite following `docs/05_IMPLEMENTATION_ORDER.md`, using git worktrees for parallel slices. Keep `main` green. Every PR must include verification outputs using repo scripts and must update docs/tests/fixtures when needed.

**Execution constraints:**
- Offline-first; do not enable network access.
- Do not introduce new dependencies without strong justification.
- Do not invent verification commands; use repo scripts or add them if missing.

**Start now with Phase 0 and Phase 1.**
- Scaffold repo layout exactly as specified.
- Set up scripts and CI parity.
- Implement Core Phase 1.1–1.4 (storage, evidence fs, audit chain, export engine) with tests and golden fixtures.
- Provide a progress report using the PR checklist format in AGENTS.md.
