# Compliance Ops Suite (Local-first) — Implementation Plan Pack
_Last updated: 2026-02-10_

## Suite components
1. **Core Platform (shared)**: Vault, Storage, Tamper-evident Audit Log, Export Pack Engine, Licensing
2. **App A: Security Questionnaire Autopilot** (money driver)
3. **App B: Compliance Binder** (flagship)
4. **App C: SOP Builder + Change Control** (sticky retention)

## Primary objectives
- **Money-soon**: Ship App A first with an end-to-end workflow: import → match → review → export → evidence pack.
- **Portfolio-flex**: Provide cryptographic integrity (tamper-evident audit log + signed export manifests), deterministic exports, and clean architecture boundaries.

## Non-negotiables
- Offline-first (no network assumptions).
- Deterministic export packs (documented determinism policy).
- Append-only audit log for all state-changing actions.
- Every change passes repo-defined verification gates (lint/test/typecheck/build).

## What “done” means (high level)
- App A works reliably on at least **two** real-world questionnaire templates.
- Binder can generate an “Audit Pack” for a selected period with a credible index and manifest.
- SOP tool supports Draft → Review → Approve → Publish, with training acknowledgments and an audit trail.

## Intended stack (assumptions)
These docs assume the following (adjust if you choose differently):
- Desktop apps built with **Tauri v2** (Rust backend) + **React + TypeScript** frontend.
- Local persistence in **SQLite**.
- Optional encryption-at-rest is supported by interfaces from day 1; implementation can be phased.

See `09_ASSUMPTIONS_DECISIONS.md` for explicit assumptions and how to change them safely.
