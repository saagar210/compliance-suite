# DEFINITIVE IMPLEMENTATION PLAN: COMPLIANCE SUITE
**Version**: 1.0
**Date**: 2026-02-12
**Authority**: Senior Software Engineer / VP of Engineering
**Execution**: By Codex (zero-tolerance for ambiguity)

---

## EXECUTIVE SUMMARY

**Current State**: 55-60% complete (Phase 2.3 done; Phase 3+ waiting)
**Target State**: 100% complete (all 3 apps fully functional)
**Phases**: 7 sequential phases covering matching algorithm → Tauri → React UI → Binder → SOP → Auth/Encryption
**Total Complexity**: Medium-High (24-32 person-days estimated)
**Critical Path**: Phase 2.4 → 2.5 → 2.6 (matching + Tauri + UI must complete in order before Phases 3-4 can start in parallel)

---

# 1. ARCHITECTURE & TECH STACK

## 1.1 CORE DECISIONS

### **Language & Runtime**
- **Backend**: Rust 1.93.0 (with toolchain.toml pin)
- **Frontend**: TypeScript (React 18+)
- **Desktop Framework**: Tauri v2 (Rust core + React webview)
- **Database**: SQLite 3.x (via shell invocation, not library binding)
- **Rationale**:
  - Rust: Type-safe, zero-cost abstractions, no GC, deterministic exports
  - React: Familiar component model, large ecosystem, TanStack Query for IPC
  - Tauri: Lightweight desktop, zero-dependency core, OS-native security
  - SQLite: Embedded, offline-first, no server, portable
  - Shell SQLite: Avoids Rust SQLite library bloat; deterministic output via TSV parsing

### **Dependencies (Minimal)**
- **Rust Core**: `ed25519-dalek` (2.1.1) only
- **Rust Tauri Apps**:
  - `tauri` (v2)
  - `serde` + `serde_json` (for JSON serialization in command responses)
  - `tokio` (async runtime for Tauri commands)
  - `uuid` (for command request tracing)
- **TypeScript**:
  - `@tauri-apps/api` (Tauri IPC client)
  - `react` (18.x)
  - `react-router-dom` (v6, routing)
  - `zustand` (state management)
  - `@tanstack/react-query` (async query caching)
  - `zod` (TypeScript schema validation)
  - `@radix-ui/primitives` (accessible component library)
  - `tailwindcss` (styling)
- **Dev Dependencies**:
  - `vitest` (test runner)
  - `@testing-library/react` (component testing)
  - `@testing-library/user-event` (user interaction simulation)
  - `eslint` + `prettier` (linting/formatting)
  - `typescript` (type checking)

**Rationale**: Minimal direct dependencies; only what Tauri + React IPC require. No heavy ORM, no bloat.

---

## 1.2 MODULE BOUNDARIES & RESPONSIBILITY

```
┌─────────────────────────────────────────────────────────────────┐
│ COMPLIANCE SUITE MONOREPO                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ CORE (Rust) — Pure Domain Logic, No Tauri Deps         │   │
│  ├─────────────────────────────────────────────────────────┤   │
│  │ ✓ Domain types (ID, errors, time)                      │   │
│  │ ✓ Storage (SQLite driver, evidence FS)                 │   │
│  │ ✓ Audit (hash chain, canonical JSON)                   │   │
│  │ ✓ Export (deterministic pack generation)               │   │
│  │ ✓ Questionnaire (importer, column mapping)             │   │
│  │ ✓ Answer Bank (CRUD, determinism)                      │   │
│  │ ✓ Matching (NEW: scoring algorithm)                    │   │
│  │ ✓ Licensing (Ed25519 verification)                     │   │
│  │ ✓ Util (fs, zip, json, shell, redact)                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                         ↓ (via Tauri bridge)                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ TAURI APPS (3) — IPC + Command Handlers (Minimal Logic) │   │
│  ├─────────────────────────────────────────────────────────┤   │
│  │ App A: Questionnaire Autopilot                         │   │
│  │  └─ Commands: vault, answer_bank, export, matching    │   │
│  │ App B: Compliance Binder                               │   │
│  │  └─ Commands: controls, evidence_mapping, binder_pack │   │
│  │ App C: SOP Builder                                     │   │
│  │  └─ Commands: sop_crud, approval_chain, training      │   │
│  └─────────────────────────────────────────────────────────┘   │
│                         ↑ (IPC invoke)                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ REACT FRONTENDS (3) — UI + State Management             │   │
│  ├─────────────────────────────────────────────────────────┤   │
│  │ Pages: Import, Map, Review, Export                     │   │
│  │ State: Zustand (import state, answer bank, UI)         │   │
│  │ API: @tauri-apps/api (IPC wrapper)                     │   │
│  │ Styles: Tailwind + Radix UI                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ PACKAGES (Shared TypeScript)                            │   │
│  ├─────────────────────────────────────────────────────────┤   │
│  │ types/dto.ts — DTO definitions (mirrors Rust)          │   │
│  │ types/errors.ts — Error codes                          │   │
│  │ types/validators.ts — Zod schemas                      │   │
│  │ ui/ — Shared React components (future)                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Responsibility Model**:
- **Core** owns ALL business logic; ZERO Tauri dependencies; 100% testable standalone
- **Tauri apps** own ONLY command dispatch, error mapping, state serialization; call core
- **React frontends** own ONLY UI/UX, forms, routing; invoke Tauri commands, never touch core
- **Packages/types** own ONLY DTOs and validation; used by all layers

**Module Boundaries (Enforced)**:
- ❌ Tauri code cannot import core domain logic directly (use commands)
- ❌ React cannot import Tauri code (use IPC)
- ❌ Core cannot depend on Tauri or React
- ✓ All cross-layer communication via DTOs (serializable)

---

# 2. FILE STRUCTURE (COMPLETE)

## 2.1 FULL DIRECTORY TREE

```
/home/user/compliance-suite/
├── Cargo.toml                              # Rust workspace root
├── Cargo.lock                              # Locked Rust deps
├── package.json                            # Root (pnpm workspace)
├── pnpm-lock.yaml                          # Locked Node deps
├── pnpm-workspace.yaml                     # pnpm workspace config
├── rust-toolchain.toml                     # Rust 1.93.0 pin
├── tsconfig.json                           # TypeScript root config
│
├── .github/
│   └── workflows/
│       └── ci.yml                          # GitHub Actions (lint/test/build)
│
├── core/                                   # ✓ EXISTING (Phase 1 + 2.3)
│   ├── Cargo.toml                          # deps: ed25519-dalek
│   ├── src/
│   │   ├── lib.rs                          # Re-exports
│   │   ├── prelude.rs                      # Common imports
│   │   ├── domain/
│   │   │   ├── mod.rs
│   │   │   ├── ids.rs                      # ULID generation
│   │   │   ├── errors.rs                   # CoreErrorCode enum
│   │   │   ├── time.rs                     # Deterministic timestamps
│   │   │   └── license.rs                  # Ed25519 verification
│   │   ├── storage/
│   │   │   ├── mod.rs                      # VaultStorage (main interface)
│   │   │   ├── db.rs                       # SqliteDb (shell-based)
│   │   │   ├── evidence_fs.rs              # Evidence filesystem
│   │   │   ├── migrations/
│   │   │   │   ├── 0001_init.sql
│   │   │   │   ├── 0002_add_license.sql
│   │   │   │   ├── 0003_license_verification.sql
│   │   │   │   ├── 0004_questionnaire_import.sql
│   │   │   │   ├── 0005_answer_bank_crud.sql
│   │   │   │   └── 0006_matching.sql         # NEW for Phase 2.4
│   │   │   └── lib.rs
│   │   ├── audit/
│   │   │   ├── mod.rs
│   │   │   ├── validator.rs
│   │   │   ├── hasher.rs
│   │   │   └── canonical.rs
│   │   ├── export/
│   │   │   ├── mod.rs
│   │   │   ├── pack.rs
│   │   │   ├── manifest.rs
│   │   │   └── index.rs
│   │   ├── questionnaire/
│   │   │   ├── mod.rs
│   │   │   ├── xlsx.rs
│   │   │   ├── csv.rs
│   │   │   ├── matching.rs                 # NEW for Phase 2.4
│   │   │   └── serde.rs
│   │   ├── answer_bank/
│   │   │   ├── mod.rs
│   │   │   └── canonicalize.rs
│   │   ├── binder/
│   │   │   ├── mod.rs                      # Phase 3
│   │   │   └── controls.rs                 # NEW for Phase 3
│   │   ├── sop/
│   │   │   ├── mod.rs                      # Phase 4
│   │   │   └── workflows.rs                # NEW for Phase 4
│   │   ├── util/
│   │   │   ├── mod.rs
│   │   │   ├── fs.rs
│   │   │   ├── zip.rs
│   │   │   ├── json.rs
│   │   │   ├── shell.rs
│   │   │   └── redact.rs
│   │   └── auth/                           # Phase 5
│   │       ├── mod.rs
│   │       └── encryption.rs
│   └── tests/
│       ├── migrations_tests.rs
│       ├── audit_chain_tests.rs
│       ├── license_tests.rs
│       ├── export_pack_golden_tests.rs
│       ├── questionnaire_column_map_tests.rs
│       ├── answer_bank_tests.rs
│       ├── matching_tests.rs                # NEW for Phase 2.4
│       ├── binder_controls_tests.rs         # NEW for Phase 3
│       ├── sop_workflow_tests.rs            # NEW for Phase 4
│       └── fixtures/                        # ✓ EXISTING (test data)
│           ├── questionnaires/
│           ├── evidence/
│           ├── golden_export/
│           ├── licenses/
│           └── matching_baseline/           # NEW for Phase 2.4
│
├── apps/
│   ├── questionnaire/                      # App A (Phases 2.4-2.7)
│   │   ├── Cargo.toml
│   │   ├── tsconfig.json
│   │   ├── tailwind.config.js              # NEW for Phase 2.6
│   │   ├── vite.config.ts                  # NEW for Phase 2.6
│   │   ├── vitest.config.ts                # NEW for Phase 2.7
│   │   ├── src/
│   │   │   ├── main.tsx                    # React entry point
│   │   │   ├── App.tsx                     # Root component (NEW for Phase 2.6)
│   │   │   ├── index.css                   # Global styles (NEW for Phase 2.6)
│   │   │   ├── api/
│   │   │   │   └── tauri.ts                # Tauri command wrappers (NEW for Phase 2.5)
│   │   │   ├── state/
│   │   │   │   ├── index.ts                # Zustand store exports (NEW for Phase 2.6)
│   │   │   │   ├── importStore.ts          # Import state (NEW for Phase 2.6)
│   │   │   │   ├── answerBankStore.ts      # Answer bank state (NEW for Phase 2.6)
│   │   │   │   └── uiStore.ts              # UI state (loading, errors) (NEW for Phase 2.6)
│   │   │   ├── components/
│   │   │   │   ├── ui/                     # Radix UI wrappers (NEW for Phase 2.6)
│   │   │   │   │   ├── Button.tsx
│   │   │   │   │   ├── Dialog.tsx
│   │   │   │   │   ├── Form.tsx
│   │   │   │   │   ├── Input.tsx
│   │   │   │   │   ├── Table.tsx
│   │   │   │   │   └── Toast.tsx
│   │   │   │   ├── features/
│   │   │   │   │   ├── ImportForm.tsx      # File upload (NEW for Phase 2.6)
│   │   │   │   │   ├── ColumnMapTable.tsx  # Column mapping UI (NEW for Phase 2.6)
│   │   │   │   │   ├── AnswerBankTable.tsx # Answer bank CRUD (NEW for Phase 2.6)
│   │   │   │   │   ├── MatchingResults.tsx # Matching suggestions (NEW for Phase 2.6)
│   │   │   │   │   └── ExportDialog.tsx    # Export pack generation (NEW for Phase 2.6)
│   │   │   │   └── layout/
│   │   │   │       ├── Sidebar.tsx         # NEW for Phase 2.6
│   │   │   │       └── Header.tsx          # NEW for Phase 2.6
│   │   │   ├── routes/
│   │   │   │   ├── index.ts                # Route definitions (NEW for Phase 2.6)
│   │   │   │   ├── Import.tsx              # Page 1 (NEW for Phase 2.6)
│   │   │   │   ├── Map.tsx                 # Page 2 (NEW for Phase 2.6)
│   │   │   │   ├── AnswerBank.tsx          # Page 3 (NEW for Phase 2.6)
│   │   │   │   ├── Review.tsx              # Page 4 (NEW for Phase 2.6)
│   │   │   │   └── Export.tsx              # Page 5 (NEW for Phase 2.6)
│   │   │   ├── hooks/
│   │   │   │   ├── useVault.ts             # Vault lifecycle (NEW for Phase 2.5)
│   │   │   │   ├── useImport.ts            # Import state & queries (NEW for Phase 2.6)
│   │   │   │   ├── useAnswerBank.ts        # Answer bank mutations (NEW for Phase 2.6)
│   │   │   │   ├── useMatching.ts          # Matching queries (NEW for Phase 2.6)
│   │   │   │   └── useTauriInvoke.ts       # Tauri error handling (NEW for Phase 2.6)
│   │   │   ├── utils/
│   │   │   │   ├── validators.ts           # Form validation (NEW for Phase 2.6)
│   │   │   │   ├── formatters.ts           # Display formatting (NEW for Phase 2.6)
│   │   │   │   └── error.ts                # Error messages (NEW for Phase 2.6)
│   │   │   └── __tests__/
│   │   │       ├── ImportForm.test.tsx     # NEW for Phase 2.7
│   │   │       ├── ColumnMapTable.test.tsx # NEW for Phase 2.7
│   │   │       ├── AnswerBankTable.test.tsx# NEW for Phase 2.7
│   │   │       └── e2e/
│   │   │           └── questionnaire_flow.test.tsx # NEW for Phase 2.7
│   │   ├── src-tauri/
│   │   │   ├── Cargo.toml                  # deps: tauri, serde, tokio
│   │   │   ├── tauri.conf.json             # Tauri window config (NEW for Phase 2.5)
│   │   │   └── src/
│   │   │       ├── main.rs                 # Tauri setup + window (NEW for Phase 2.5)
│   │   │       ├── app_state.rs            # AppState struct (NEW for Phase 2.5)
│   │   │       ├── error_map.rs            # CoreError → TauriError mapping (NEW for Phase 2.5)
│   │   │       ├── commands/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── vault.rs            # Commands: create, open, close, lock (NEW for Phase 2.5)
│   │   │       │   ├── evidence.rs         # Commands: add_file, list (NEW for Phase 2.5)
│   │   │       │   ├── answer_bank.rs      # Commands: create, read, update, delete, list (NEW for Phase 2.5)
│   │   │       │   ├── matching.rs         # Commands: get_suggestions (NEW for Phase 2.5)
│   │   │       │   ├── export.rs           # Commands: generate_pack (NEW for Phase 2.5)
│   │   │       │   └── license.rs          # Commands: check_status (NEW for Phase 2.5)
│   │   │       └── __tests__/
│   │   │           └── command_tests.rs    # NEW for Phase 2.7
│   │   └── package.json
│   │
│   ├── binder/                             # App B (Phase 3)
│   │   ├── Cargo.toml
│   │   ├── tsconfig.json
│   │   ├── tailwind.config.js              # NEW for Phase 3
│   │   ├── vite.config.ts                  # NEW for Phase 3
│   │   ├── src/
│   │   │   ├── main.tsx
│   │   │   ├── App.tsx                     # NEW for Phase 3
│   │   │   ├── api/
│   │   │   │   └── tauri.ts                # NEW for Phase 3
│   │   │   ├── state/
│   │   │   │   ├── controlStore.ts         # NEW for Phase 3
│   │   │   │   ├── evidenceStore.ts        # NEW for Phase 3
│   │   │   │   └── uiStore.ts              # NEW for Phase 3
│   │   │   ├── components/ (full UI tree) # NEW for Phase 3
│   │   │   ├── routes/ (5+ pages)          # NEW for Phase 3
│   │   │   └── hooks/                      # NEW for Phase 3
│   │   ├── src-tauri/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs                 # NEW for Phase 3
│   │   │       ├── commands/
│   │   │       │   ├── controls.rs         # NEW for Phase 3
│   │   │       │   ├── evidence_mapping.rs # NEW for Phase 3
│   │   │       │   └── binder_pack.rs      # NEW for Phase 3
│   │   └── package.json
│   │
│   └── sop/                                # App C (Phase 4)
│       ├── Cargo.toml
│       ├── tsconfig.json
│       ├── tailwind.config.js              # NEW for Phase 4
│       ├── vite.config.ts                  # NEW for Phase 4
│       ├── src/ (full structure)            # NEW for Phase 4
│       ├── src-tauri/ (commands)            # NEW for Phase 4
│       └── package.json
│
├── packages/
│   ├── types/                              # ✓ EXISTING (partial)
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── src/
│   │       ├── index.ts                    # Re-exports
│   │       ├── dto.ts                      # NEW additions for Phase 2.4+
│   │       │   ├── MatchSuggestionDto
│   │       │   ├── BinderControlDto (Phase 3)
│   │       │   ├── SopDto (Phase 4)
│   │       │   └── UserDto (Phase 5)
│   │       ├── errors.ts                   # ✓ EXISTING (add new codes)
│   │       ├── validators.ts               # ✓ EXISTING (extend)
│   │       │   ├── matchingInputSchema
│   │       │   ├── controlSchema (Phase 3)
│   │       │   └── sopSchema (Phase 4)
│   │       └── __tests__/
│   │           ├── dto.test.ts             # NEW for Phase 2.7
│   │           └── validators.test.ts      # NEW for Phase 2.7
│   │
│   └── ui/                                 # Shared React components (Phase 5+)
│       ├── package.json
│       ├── tsconfig.json
│       └── src/
│           ├── index.ts
│           └── components/ (TBD)
│
├── scripts/
│   ├── lint.sh                             # ✓ EXISTING
│   ├── build.sh                            # ✓ EXISTING (NEW: add Tauri build)
│   ├── test.sh                             # ✓ EXISTING (NEW: add Vitest)
│   ├── typecheck.sh                        # NEW for Phase 2.5
│   ├── format.sh                           # ✓ EXISTING
│   ├── verify-determinism.sh               # ✓ EXISTING
│   └── demo.sh                             # NEW for Phase 2.7 (launch Tauri dev)
│
├── docs/                                   # ✓ EXISTING
│   ├── 00_OVERVIEW.md
│   ├── 01_ARCHITECTURE.md
│   ├── 02_REPO_STRUCTURE.md
│   ├── 03_DATA_MODEL.md
│   ├── 04_API_CONTRACTS.md
│   ├── 05_IMPLEMENTATION_ORDER.md
│   ├── 06_ERROR_HANDLING.md
│   ├── 07_TESTING.md
│   ├── 08_SECURITY_PRIVACY.md
│   ├── 09_ASSUMPTIONS_DECISIONS.md
│   ├── 10_DEFINITION_OF_DONE.md
│   ├── 11_RUNBOOK.md                      # NEW for Phase 2.7
│   ├── 12_API_REFERENCE.md                # NEW for Phase 2.5
│   └── 13_DATABASE_SCHEMA.md               # NEW for Phase 2.4
│
├── fixtures/                               # ✓ EXISTING (add new)
│   ├── questionnaires/
│   ├── evidence/
│   ├── golden_export/
│   ├── licenses/
│   ├── matching_baseline/                  # NEW for Phase 2.4
│   ├── binder_controls/                    # NEW for Phase 3
│   └── sop_workflows/                      # NEW for Phase 4
│
├── .gitignore                              # ✓ EXISTING
├── README.md                               # ✓ EXISTING
├── AGENTS.md                               # ✓ EXISTING
└── IMPLEMENTATION_PLAN.md                  # THIS FILE
```

---

## 2.2 KEY FILES: PURPOSE & INTERDEPENDENCIES

### **CORE RUST FILES** (By Creation Order)

| File | Purpose | Created Phase | Depends On | Enables |
|------|---------|---------------|-----------|---------|
| `core/src/questionnaire/matching.rs` | Token-overlap scoring algorithm | 2.4 | questionnaire/mod.rs, answer_bank/mod.rs | Phase 2.5 Tauri |
| `core/src/storage/migrations/0006_matching.sql` | DB schema for match suggestions | 2.4 | 0005_answer_bank_crud.sql | Phase 2.4 tests |
| `core/tests/matching_tests.rs` | Unit + golden tests for matching | 2.4 | matching.rs, fixtures | Phase 2.5 |
| `apps/questionnaire/src-tauri/Cargo.toml` | Add tauri, serde, tokio | 2.5 | root Cargo.toml | Phase 2.5 main.rs |
| `apps/questionnaire/src-tauri/tauri.conf.json` | Window config (size, title, dev) | 2.5 | — | Phase 2.5 main.rs |
| `apps/questionnaire/src-tauri/src/main.rs` | Tauri setup, window, command registry | 2.5 | Cargo.toml, tauri.conf.json | Phase 2.5 complete |
| `apps/questionnaire/src-tauri/src/app_state.rs` | Shared state (vault, actor, config) | 2.5 | core module | Phase 2.5 commands |
| `apps/questionnaire/src-tauri/src/error_map.rs` | CoreError → Tauri response | 2.5 | core::domain::errors | Phase 2.5 commands |
| `apps/questionnaire/src-tauri/src/commands/{vault,evidence,answer_bank,matching,export,license}.rs` | 6 command modules + handlers | 2.5 | app_state, error_map, core | Phase 2.6 API |
| `packages/types/src/dto.ts` (extend) | Add MatchSuggestionDto, etc. | 2.4-5 | — | Phase 2.6 |
| `packages/types/src/validators.ts` (extend) | Add matchingInputSchema | 2.4-5 | zod | Phase 2.6 |

### **REACT FRONTEND FILES** (By Creation Order)

| File | Purpose | Created Phase | Depends On | Enables |
|------|---------|---------------|-----------|---------|
| `apps/questionnaire/src/main.tsx` | React entry point | 2.6 | index.html | React boot |
| `apps/questionnaire/src/App.tsx` | Root component + router | 2.6 | react-router | routes |
| `apps/questionnaire/src/api/tauri.ts` | Tauri IPC wrapper functions | 2.6 | @tauri-apps/api | hooks |
| `apps/questionnaire/src/state/{import,answerBank,ui}Store.ts` | Zustand stores (3 files) | 2.6 | zustand | components |
| `apps/questionnaire/src/hooks/use{Vault,Import,AnswerBank,Matching,TauriInvoke}.ts` | 5 custom hooks | 2.6 | API + stores | components |
| `apps/questionnaire/src/routes/{Import,Map,AnswerBank,Review,Export}.tsx` | 5 page components | 2.6 | hooks, stores | demo |
| `apps/questionnaire/src/components/features/{ImportForm,ColumnMapTable,AnswerBankTable,MatchingResults,ExportDialog}.tsx` | 5 feature components | 2.6 | routes, hooks | Page functionality |
| `apps/questionnaire/src/components/ui/{Button,Dialog,Form,Input,Table,Toast}.tsx` | 6 Radix UI wrappers | 2.6 | @radix-ui, tailwind | feature components |
| `apps/questionnaire/__tests__/*.test.tsx` | Component + integration tests | 2.7 | vitest, testing-lib | Phase 2.7 complete |

---

## 2.3 IMPORT/DEPENDENCY GRAPH

```
┌─────────────────────────────────────────────────────────────────┐
│ REACT COMPONENTS (questionnaire/src/components)                 │
│ ├─ ImportForm.tsx                                               │
│ │  └─ useImport() → importStore → API.invokeImport() → main.rs │
│ ├─ ColumnMapTable.tsx                                           │
│ │  └─ useImport() → importStore → API.invokeColumnMap() → cmd  │
│ ├─ AnswerBankTable.tsx                                          │
│ │  └─ useAnswerBank() → answerBankStore → API.invoke...() → cmd│
│ ├─ MatchingResults.tsx                                          │
│ │  └─ useMatching() → API.getMatches() → cmd::matching         │
│ └─ ExportDialog.tsx                                             │
│    └─ API.generateExport() → cmd::export → core::export::pack  │
│                                                                  │
│ ZUSTAND STORES                                                   │
│ ├─ importStore (import state + setters)                         │
│ ├─ answerBankStore (answer bank list + cache)                  │
│ └─ uiStore (loading, toasts, modals)                           │
│                                                                  │
│ TAURI COMMANDS                                                   │
│ ├─ vault_create() → core::storage::VaultStorage::create()      │
│ ├─ vault_open() → core::storage::VaultStorage::open()          │
│ ├─ import_xlsx() → core::questionnaire::import_xlsx()          │
│ ├─ get_column_map() → core::questionnaire::profile_columns()   │
│ ├─ save_column_map() → core::questionnaire::save_column_map()  │
│ ├─ answer_bank_create() → core::answer_bank::create()          │
│ ├─ answer_bank_update() → core::answer_bank::update()          │
│ ├─ answer_bank_list() → core::answer_bank::list()              │
│ ├─ get_matching_suggestions() → core::questionnaire::match...()│
│ ├─ generate_export_pack() → core::export::pack::generate()     │
│ └─ check_license() → core::domain::license::verify()           │
│                                                                  │
│ CORE (Rust)                                                      │
│ ├─ core::questionnaire::matching::score()                      │
│ ├─ core::questionnaire::import_xlsx()                          │
│ ├─ core::questionnaire::profile_columns()                      │
│ ├─ core::answer_bank::{create,read,update,delete,list}()      │
│ ├─ core::export::pack::generate()                              │
│ ├─ core::storage::VaultStorage                                 │
│ │  └─ core::storage::db::SqliteDb (shell invocation)           │
│ ├─ core::audit::Validator (hash chain)                         │
│ └─ core::domain::license::verify_ed25519()                     │
│                                                                  │
│ DATABASE (SQLite)                                                │
│ ├─ vault, evidence_item, audit_event, answer_bank              │
│ ├─ questionnaire_import, column_map, match_suggestion          │
│ └─ license_verification                                         │
└─────────────────────────────────────────────────────────────────┘
```

---

# 3. DATA MODELS & API CONTRACTS

## 3.1 DATABASE SCHEMAS

### **MIGRATION 0006: MATCHING** (Phase 2.4)
```sql
-- File: core/src/storage/migrations/0006_matching.sql

CREATE TABLE IF NOT EXISTS match_suggestion (
    id TEXT PRIMARY KEY,                    -- ULID
    vault_id TEXT NOT NULL,                 -- FK to vault
    question_id TEXT NOT NULL,              -- FK to questionnaire_import_question
    answer_bank_entry_id TEXT NOT NULL,     -- FK to answer_bank_entry
    score REAL NOT NULL,                    -- 0.0 - 1.0
    normalized_question TEXT NOT NULL,      -- For reproducibility
    normalized_answer TEXT NOT NULL,        -- For reproducibility
    confidence_explanation TEXT,            -- Human-readable reason
    accepted BOOLEAN NOT NULL DEFAULT 0,    -- User accepted this suggestion?
    accepted_at TIMESTAMP,                  -- When user accepted
    created_at TIMESTAMP NOT NULL,          -- UTC timestamp
    updated_at TIMESTAMP NOT NULL,          -- UTC timestamp
    FOREIGN KEY(vault_id) REFERENCES vault(id),
    FOREIGN KEY(question_id) REFERENCES questionnaire_import_question(id),
    FOREIGN KEY(answer_bank_entry_id) REFERENCES answer_bank_entry(id)
);

CREATE INDEX idx_match_suggestion_vault ON match_suggestion(vault_id);
CREATE INDEX idx_match_suggestion_question ON match_suggestion(question_id);
CREATE INDEX idx_match_suggestion_score ON match_suggestion(score DESC);

-- Track schema version
INSERT OR REPLACE INTO schema_version(version, applied_at) VALUES(6, datetime('now'));
```

### **BINDER CONTROLS** (Phase 3)
```sql
-- File: core/src/storage/migrations/0007_binder_controls.sql

CREATE TABLE IF NOT EXISTS control_framework (
    id TEXT PRIMARY KEY,                    -- ULID
    vault_id TEXT NOT NULL,                 -- FK to vault
    name TEXT NOT NULL,                     -- "SOC 2", "ISO 27001"
    description TEXT,
    version TEXT,                           -- Framework version
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY(vault_id) REFERENCES vault(id)
);

CREATE TABLE IF NOT EXISTS control_definition (
    id TEXT PRIMARY KEY,                    -- ULID
    framework_id TEXT NOT NULL,             -- FK to control_framework
    control_id TEXT NOT NULL,               -- "CC6.1", "A.5.1.1"
    description TEXT NOT NULL,
    requirements TEXT NOT NULL,             -- JSON array of strings
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(framework_id) REFERENCES control_framework(id)
);

CREATE TABLE IF NOT EXISTS control_evidence (
    id TEXT PRIMARY KEY,                    -- ULID
    control_id TEXT NOT NULL,               -- FK to control_definition
    evidence_id TEXT NOT NULL,              -- FK to evidence_item
    linked_by TEXT NOT NULL,                -- actor ID
    linked_at TIMESTAMP NOT NULL,
    FOREIGN KEY(control_id) REFERENCES control_definition(id),
    FOREIGN KEY(evidence_id) REFERENCES evidence_item(id)
);

-- Audit events already capture all changes via audit_event table
```

### **SOP WORKFLOWS** (Phase 4)
```sql
-- File: core/src/storage/migrations/0008_sop_workflows.sql

CREATE TABLE IF NOT EXISTS sop_document (
    id TEXT PRIMARY KEY,                    -- ULID
    vault_id TEXT NOT NULL,                 -- FK to vault
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,                   -- ENUM: draft, review, approved, published
    owner_id TEXT NOT NULL,                 -- actor ID
    content_version INT DEFAULT 0,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    published_at TIMESTAMP,
    FOREIGN KEY(vault_id) REFERENCES vault(id)
);

CREATE TABLE IF NOT EXISTS sop_approval_chain (
    id TEXT PRIMARY KEY,                    -- ULID
    sop_id TEXT NOT NULL,                   -- FK to sop_document
    approver_id TEXT NOT NULL,              -- actor ID
    status TEXT NOT NULL,                   -- ENUM: pending, approved, rejected
    decision_at TIMESTAMP,
    decision_reason TEXT,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY(sop_id) REFERENCES sop_document(id)
);

CREATE TABLE IF NOT EXISTS training_acknowledgment (
    id TEXT PRIMARY KEY,                    -- ULID
    sop_id TEXT NOT NULL,                   -- FK to sop_document
    user_id TEXT NOT NULL,                  -- actor ID
    acknowledged_at TIMESTAMP NOT NULL,
    acknowledged_version INT NOT NULL,      -- content_version when acknowledged
    FOREIGN KEY(sop_id) REFERENCES sop_document(id)
);
```

---

## 3.2 API CONTRACTS (Tauri Commands)

### **VAULT COMMANDS**

**1. vault_create**
```rust
// Tauri Command
#[tauri::command]
async fn vault_create(
    path: String,
    name: String,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<VaultDto, String>

// Request (JSON)
{
  "path": "/home/user/.compliance/vault1",
  "name": "Q4 Audit 2025",
  "actor": "alice@company.com"
}

// Response (200 OK)
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "name": "Q4 Audit 2025",
  "path": "/home/user/.compliance/vault1",
  "created_at": "2025-02-12T14:30:00Z",
  "locked": false,
  "license_status": "valid"
}

// Error (400/500)
{
  "code": "VAULT_ALREADY_EXISTS" | "INVALID_PATH" | "IO_ERROR",
  "message": "Human-readable error"
}
```

**2. vault_open**
```rust
#[tauri::command]
async fn vault_open(
    path: String,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<VaultDto, String>
```

**3. vault_close**
```rust
#[tauri::command]
async fn vault_close(
    vault_id: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String>
```

**4. vault_lock**
```rust
#[tauri::command]
async fn vault_lock(
    vault_id: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String>
```

---

### **QUESTIONNAIRE COMMANDS**

**5. import_questionnaire**
```rust
#[tauri::command]
async fn import_questionnaire(
    vault_id: String,
    file_path: String,  // Full path (after user selects in dialog)
    name: String,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<QuestionnaireImportDto, String>

// Response (200 OK)
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "name": "SOC 2 Template",
  "file_hash": "abc123def456",
  "column_count": 5,
  "question_count": 50,
  "column_profiles": [
    {
      "column_index": 0,
      "inferred_type": "question",
      "sample_values": ["1.1 Access Controls", "1.2 Authentication"]
    },
    // ... 4 more columns
  ],
  "imported_at": "2025-02-12T14:30:00Z"
}

// Error (400/500)
{
  "code": "INVALID_FILE_FORMAT" | "FILE_NOT_FOUND" | "VAULT_NOT_FOUND",
  "message": "..."
}
```

**6. get_column_profiles**
```rust
#[tauri::command]
async fn get_column_profiles(
    questionnaire_id: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<ColumnProfileDto>, String>

// Response
[
  {
    "column_index": 0,
    "inferred_type": "question" | "answer" | "notes" | "unknown",
    "sample_values": ["1.1 Access", "1.2 Auth", ...],
    "validation_issues": ["Contains HTML tags", "Too long"]
  },
  // ... more columns
]
```

**7. save_column_mapping**
```rust
#[tauri::command]
async fn save_column_mapping(
    questionnaire_id: String,
    mapping: ColumnMapDto,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String>

// Request (ColumnMapDto)
{
  "question_column": 0,
  "answer_column": 1,
  "notes_column": 2,
  "validation_rules": {
    "require_answer": true,
    "min_answer_length": 10
  }
}

// Response
()
```

---

### **ANSWER BANK COMMANDS**

**8. answer_bank_create**
```rust
#[tauri::command]
async fn answer_bank_create(
    vault_id: String,
    entry: CreateAnswerBankEntryDto,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<AnswerBankEntryDto, String>

// Request
{
  "question_tag": "access-controls",
  "answer": "We implement role-based access control (RBAC) using...",
  "notes": "Updated for Q4 audit; approved by security team",
  "evidence_ids": ["01ARZ3NDEKTSV4RRFFQ69G5FAV", "01ARZ3NDEKTSV4RRFFQ69G5FAW"],
  "owner": "alice@company.com"
}

// Response (201 Created)
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "question_tag": "access-controls",
  "answer": "We implement...",
  "notes": "Updated for Q4...",
  "content_hash": "sha256:abc123...",
  "evidence_count": 2,
  "owner": "alice@company.com",
  "created_at": "2025-02-12T14:30:00Z",
  "updated_at": "2025-02-12T14:30:00Z"
}
```

**9. answer_bank_update**
```rust
#[tauri::command]
async fn answer_bank_update(
    entry_id: String,
    update: UpdateAnswerBankEntryDto,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<AnswerBankEntryDto, String>
```

**10. answer_bank_delete**
```rust
#[tauri::command]
async fn answer_bank_delete(
    entry_id: String,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String>
```

**11. answer_bank_list**
```rust
#[tauri::command]
async fn answer_bank_list(
    vault_id: String,
    query: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AnswerBankEntryDto>, String>

// Response
[
  {
    "id": "...",
    "question_tag": "access-controls",
    "answer": "We implement...",
    "evidence_count": 2,
    "owner": "alice@company.com",
    "created_at": "..."
  },
  // ... more entries
]
```

---

### **MATCHING COMMANDS**

**12. get_matching_suggestions**
```rust
#[tauri::command]
async fn get_matching_suggestions(
    question: String,
    vault_id: String,
    top_n: Option<usize>,
    state: tauri::State<'_, AppState>
) -> Result<Vec<MatchSuggestionDto>, String>

// Request
{
  "question": "How do you implement access control?",
  "vault_id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "top_n": 3
}

// Response (200 OK)
[
  {
    "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
    "answer_bank_entry_id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
    "answer_preview": "We implement role-based access control...",
    "score": 0.92,
    "confidence_explanation": "92% token overlap: 'access', 'control', 'implement'"
  },
  {
    "id": "01ARZ3NDEKTSV4RRFFQ69G5FAW",
    "answer_bank_entry_id": "01ARZ3NDEKTSV4RRFFQ69G5FAW",
    "answer_preview": "Multi-factor authentication guards systems...",
    "score": 0.68,
    "confidence_explanation": "68% token overlap: 'access', 'systems'"
  },
  {
    "id": "01ARZ3NDEKTSV4RRFFQ69G5FAX",
    "answer_bank_entry_id": "01ARZ3NDEKTSV4RRFFQ69G5FAX",
    "answer_preview": "Identity management integrates SSO...",
    "score": 0.55,
    "confidence_explanation": "55% token overlap: 'systems', 'management'"
  }
]

// Error (400/500)
{
  "code": "VAULT_NOT_FOUND" | "INVALID_QUESTION",
  "message": "..."
}
```

---

### **EXPORT COMMANDS**

**13. generate_export_pack**
```rust
#[tauri::command]
async fn generate_export_pack(
    vault_id: String,
    export_type: String,  // "questionnaire_answers" | "evidence_binder" | "audit_report"
    filters: Option<ExportFilterDto>,
    actor: String,
    state: tauri::State<'_, AppState>
) -> Result<ExportPackDto, String>

// Response (200 OK)
{
  "id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "vault_id": "01ARZ3NDEKTSV4RRFFQ69G5FAV",
  "export_type": "questionnaire_answers",
  "file_name": "compliance_export_2025-02-12T143000Z.zip",
  "file_size_bytes": 1024000,
  "file_hash": "sha256:abc123...",
  "manifest": {
    "export_date": "2025-02-12T14:30:00Z",
    "version": "1.0",
    "entry_count": 50
  },
  "downloaded": false,
  "created_at": "2025-02-12T14:30:00Z"
}
```

---

### **LICENSE COMMANDS**

**14. check_license_status**
```rust
#[tauri::command]
async fn check_license_status(
    state: tauri::State<'_, AppState>
) -> Result<LicenseStatusDto, String>

// Response (200 OK)
{
  "status": "valid" | "invalid" | "expired",
  "vendor": "Compliance Suite Inc.",
  "expiration_date": "2025-12-31",
  "features": ["questionnaire", "binder", "sop"],
  "verified_at": "2025-02-12T14:30:00Z"
}

// Error (400)
{
  "code": "LICENSE_NOT_FOUND" | "INVALID_SIGNATURE",
  "message": "..."
}
```

---

## 3.3 TYPE DEFINITIONS (TypeScript)

### **NEW DTOs for Phase 2.4+**

```typescript
// File: packages/types/src/dto.ts (additions)

// MATCHING
export interface MatchSuggestionDto {
  id: string;
  answer_bank_entry_id: string;
  answer_preview: string;        // First 200 chars of answer
  score: number;                 // 0.0 - 1.0
  confidence_explanation: string;
  normalized_question?: string;
  normalized_answer?: string;
}

export interface MatchingInputDto {
  question: string;
  vault_id: string;
  top_n?: number;
}

// QUESTIONNAIRE
export interface QuestionnaireImportDto {
  id: string;
  name: string;
  file_hash: string;
  column_count: number;
  question_count: number;
  column_profiles: ColumnProfileDto[];
  imported_at: string;
}

export interface ColumnProfileDto {
  column_index: number;
  inferred_type: "question" | "answer" | "notes" | "unknown";
  sample_values: string[];
  validation_issues?: string[];
}

export interface ColumnMapDto {
  question_column: number;
  answer_column: number;
  notes_column?: number;
  validation_rules?: {
    require_answer?: boolean;
    min_answer_length?: number;
    max_answer_length?: number;
  };
}

// ANSWER BANK
export interface CreateAnswerBankEntryDto {
  question_tag: string;
  answer: string;
  notes?: string;
  evidence_ids?: string[];
  owner: string;
}

export interface UpdateAnswerBankEntryDto {
  answer?: string;
  notes?: string;
  evidence_ids?: string[];
}

export interface AnswerBankEntryDto {
  id: string;
  question_tag: string;
  answer: string;
  notes?: string;
  content_hash: string;
  evidence_count: number;
  owner: string;
  created_at: string;
  updated_at: string;
}

// VAULT
export interface VaultDto {
  id: string;
  name: string;
  path: string;
  created_at: string;
  locked: boolean;
  license_status: "valid" | "invalid" | "expired";
}

// EXPORT
export interface ExportPackDto {
  id: string;
  vault_id: string;
  export_type: string;
  file_name: string;
  file_size_bytes: number;
  file_hash: string;
  manifest: {
    export_date: string;
    version: string;
    entry_count: number;
  };
  downloaded: boolean;
  created_at: string;
}

// LICENSE
export interface LicenseStatusDto {
  status: "valid" | "invalid" | "expired";
  vendor: string;
  expiration_date: string;
  features: string[];
  verified_at: string;
}
```

### **VALIDATION SCHEMAS (Zod)**

```typescript
// File: packages/types/src/validators.ts (additions)

import { z } from "zod";

export const MatchingInputSchema = z.object({
  question: z.string().min(5).max(1000),
  vault_id: z.string().ulid(),
  top_n: z.number().int().min(1).max(10).optional().default(3),
});

export const ColumnMapSchema = z.object({
  question_column: z.number().int().min(0).max(100),
  answer_column: z.number().int().min(0).max(100),
  notes_column: z.number().int().min(0).max(100).optional(),
  validation_rules: z.object({
    require_answer: z.boolean().optional(),
    min_answer_length: z.number().int().min(0).optional(),
    max_answer_length: z.number().int().min(1).optional(),
  }).optional(),
});

export const CreateAnswerBankEntrySchema = z.object({
  question_tag: z.string().min(1).max(100),
  answer: z.string().min(10).max(10000),
  notes: z.string().max(5000).optional(),
  evidence_ids: z.array(z.string().ulid()).optional(),
  owner: z.string().email(),
});
```

---

## 3.4 STATE SHAPE (Zustand)

```typescript
// File: apps/questionnaire/src/state/importStore.ts

import { create } from "zustand";

export interface ImportState {
  // UI state
  currentStep: "import" | "map" | "review" | "export";
  loading: boolean;
  error: string | null;

  // Questionnaire data
  questionnaireId: string | null;
  questionnaireName: string | null;
  columnProfiles: ColumnProfileDto[] | null;
  columnMapping: ColumnMapDto | null;

  // File selection
  selectedFilePath: string | null;
  fileHash: string | null;

  // Actions
  setStep: (step: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  importFile: (path: string, name: string) => Promise<void>;
  setColumnMapping: (mapping: ColumnMapDto) => void;
  reset: () => void;
}

export const useImportStore = create<ImportState>((set) => ({
  currentStep: "import",
  loading: false,
  error: null,
  questionnaireId: null,
  questionnaireName: null,
  columnProfiles: null,
  columnMapping: null,
  selectedFilePath: null,
  fileHash: null,

  setStep: (step) => set({ currentStep: step as ImportState["currentStep"] }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),

  importFile: async (path: string, name: string) => {
    set({ loading: true, error: null });
    try {
      const dto = await invokeImportQuestionnaire(path, name);
      set({
        questionnaireId: dto.id,
        questionnaireName: dto.name,
        columnProfiles: dto.column_profiles,
        fileHash: dto.file_hash,
        currentStep: "map",
      });
    } catch (err) {
      set({ error: err.message });
    } finally {
      set({ loading: false });
    }
  },

  setColumnMapping: (mapping) => set({ columnMapping: mapping }),
  reset: () => set({
    currentStep: "import",
    loading: false,
    error: null,
    questionnaireId: null,
    questionnaireName: null,
    columnProfiles: null,
    columnMapping: null,
    selectedFilePath: null,
    fileHash: null,
  }),
}));

// ========================================

// File: apps/questionnaire/src/state/answerBankStore.ts

export interface AnswerBankState {
  entries: AnswerBankEntryDto[];
  selectedEntryId: string | null;
  loading: boolean;
  error: string | null;

  // Actions
  setEntries: (entries: AnswerBankEntryDto[]) => void;
  addEntry: (entry: AnswerBankEntryDto) => void;
  updateEntry: (id: string, update: Partial<AnswerBankEntryDto>) => void;
  deleteEntry: (id: string) => void;
  selectEntry: (id: string | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useAnswerBankStore = create<AnswerBankState>((set) => ({
  entries: [],
  selectedEntryId: null,
  loading: false,
  error: null,

  setEntries: (entries) => set({ entries }),
  addEntry: (entry) => set((state) => ({
    entries: [...state.entries, entry]
  })),
  updateEntry: (id, update) => set((state) => ({
    entries: state.entries.map((e) => e.id === id ? { ...e, ...update } : e)
  })),
  deleteEntry: (id) => set((state) => ({
    entries: state.entries.filter((e) => e.id !== id)
  })),
  selectEntry: (id) => set({ selectedEntryId: id }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
}));

// ========================================

// File: apps/questionnaire/src/state/uiStore.ts

export interface UIState {
  toasts: Toast[];
  modals: { [key: string]: boolean };
  sidebarOpen: boolean;

  // Actions
  addToast: (toast: Toast) => void;
  removeToast: (id: string) => void;
  openModal: (key: string) => void;
  closeModal: (key: string) => void;
  setSidebarOpen: (open: boolean) => void;
}

export interface Toast {
  id: string;
  message: string;
  type: "info" | "success" | "warning" | "error";
  duration: number;
}

export const useUIStore = create<UIState>((set) => ({
  toasts: [],
  modals: {},
  sidebarOpen: true,

  addToast: (toast) => set((state) => ({
    toasts: [...state.toasts, toast]
  })),
  removeToast: (id) => set((state) => ({
    toasts: state.toasts.filter((t) => t.id !== id)
  })),
  openModal: (key) => set((state) => ({
    modals: { ...state.modals, [key]: true }
  })),
  closeModal: (key) => set((state) => ({
    modals: { ...state.modals, [key]: false }
  })),
  setSidebarOpen: (open) => set({ sidebarOpen: open }),
}));
```

---

# 4. IMPLEMENTATION STEPS (NUMBERED & SEQUENTIAL)

## PHASE 2.4: MATCHING ALGORITHM

### **Step 1: Create Database Migration for Matching**
- **Exact files**: `core/src/storage/migrations/0006_matching.sql`
- **Changes required**:
  - Create `match_suggestion` table (13 columns: id, vault_id, question_id, answer_bank_entry_id, score, normalized_question, normalized_answer, confidence_explanation, accepted, accepted_at, created_at, updated_at)
  - Create indexes on vault_id, question_id, score
  - Update `schema_version` to 6
  - **Code pattern**: Copy structure from 0005_answer_bank_crud.sql (same FK + audit patterns)
- **Prerequisites**: Phase 1 migrations 0001-0005 complete
- **Unlocked after**: Core tests can run matching tests
- **Complexity**: Low

### **Step 2: Implement Matching Algorithm in Rust Core**
- **Exact files**: `core/src/questionnaire/matching.rs` (NEW)
- **Changes required**:
  ```rust
  pub struct MatchingEngine {
    answer_bank: Vec<AnswerBankEntry>,
  }

  impl MatchingEngine {
    pub fn new(answer_bank: Vec<AnswerBankEntry>) -> Self { ... }

    /// Normalize text: lowercase, remove punctuation, split into tokens
    fn normalize(text: &str) -> Vec<String> {
      // Implementation: lowercase → remove [.,!?;:'"()—-] → split on whitespace
    }

    /// Score question against answer: token overlap / total unique tokens
    fn score_tokens(q_tokens: &[String], a_tokens: &[String]) -> f64 {
      // (intersection count) / (union count)
    }

    /// Get top-N suggestions
    pub fn get_suggestions(&self, question: &str, top_n: usize) -> Vec<Suggestion> {
      // 1. normalize(question) → q_tokens
      // 2. For each answer_bank entry:
      //    a. normalize(answer) → a_tokens
      //    b. score = score_tokens(q_tokens, a_tokens)
      //    c. Push (score, entry_id, normalized_q, normalized_a)
      // 3. Sort by score descending
      // 4. Take top_n
      // 5. Return with confidence explanation
    }
  }

  pub struct Suggestion {
    pub answer_bank_entry_id: String,
    pub score: f64,                      // 0.0 - 1.0
    pub normalized_question: String,
    pub normalized_answer: String,
    pub confidence_explanation: String,  // "92% token overlap: 'access', 'control'"
  }
  ```
- **Prerequisites**: answer_bank.rs complete; fixtures with sample answers ready
- **Unlocked after**: Core tests pass; Phase 2.5 can wire commands
- **Complexity**: Low

### **Step 3: Add Matching Tests to Core**
- **Exact files**: `core/tests/matching_tests.rs` (NEW)
- **Changes required**:
  ```rust
  #[test]
  fn test_normalize_removes_punctuation() { ... }

  #[test]
  fn test_token_overlap_scoring() {
    // q_tokens: ["access", "control"]
    // a_tokens: ["access", "control", "rbac"]
    // score should be 2/3 ≈ 0.67
  }

  #[test]
  fn test_top_n_suggestions() {
    // Load fixtures/matching_baseline/answers.json
    // Query: "How do you implement access control?"
    // Verify: top 3 suggestions ranked by score
    // Verify: score > 0.5 for top result
  }

  #[test]
  fn test_golden_matching_results() {
    // Load fixtures/matching_baseline/test_cases.json
    // For each case: { question, expected_top_answer_id, min_score }
    // Verify each case produces correct ranking
  }
  ```
- **Fixtures needed**:
  - `fixtures/matching_baseline/answers.json` (20 pre-written answers)
  - `fixtures/matching_baseline/test_cases.json` (10 Q/A pairs with expected rankings)
- **Prerequisites**: matching.rs implemented
- **Unlocked after**: Core tests pass; CI green
- **Complexity**: Low

### **Step 4: Update packages/types for MatchSuggestionDto**
- **Exact files**: `packages/types/src/dto.ts`, `packages/types/src/validators.ts`
- **Changes required**:
  - Add MatchSuggestionDto interface (as specified in section 3.3)
  - Add MatchingInputSchema validation (Zod)
  - Export both from index.ts
- **Prerequisites**: Zod already available
- **Unlocked after**: Phase 2.5 can use for command responses
- **Complexity**: Low

---

## PHASE 2.5: TAURI INTEGRATION & COMMAND WIRING

### **Step 5: Add Tauri Dependencies**
- **Exact files**: `apps/questionnaire/src-tauri/Cargo.toml`
- **Changes required**:
  ```toml
  [dependencies]
  core = { path = "../../core" }
  tauri = { version = "2.0", features = ["shell-open", "process"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  tokio = { version = "1.0", features = ["full"] }
  uuid = { version = "1.0", features = ["v4", "serde"] }
  ```
- **Prerequisites**: None (independent)
- **Unlocked after**: Step 6 (main.rs)
- **Complexity**: Low

### **Step 6: Create Tauri Configuration & Main Entry Point**
- **Exact files**:
  - `apps/questionnaire/src-tauri/tauri.conf.json` (NEW)
  - `apps/questionnaire/src-tauri/src/main.rs` (REPLACE scaffolding)
- **Changes required**:
  ```json
  // tauri.conf.json
  {
    "productName": "Compliance Suite - Questionnaire",
    "version": "0.1.0",
    "identifier": "com.compliancesuite.questionnaire",
    "build": {
      "beforeDevCommand": "npm run dev",
      "devUrl": "http://localhost:5173",
      "beforeBuildCommand": "npm run build",
      "frontendDist": "../src",
      "features": ["shell-open"]
    },
    "app": {
      "windows": [
        {
          "title": "Compliance Suite - Questionnaire Autopilot",
          "width": 1200,
          "height": 800,
          "resizable": true,
          "fullscreen": false
        }
      ],
      "security": {
        "csp": "default-src 'self'"
      }
    }
  }
  ```
  ```rust
  // main.rs (complete rewrite)
  use tauri::{Manager, Window};
  use std::sync::{Arc, Mutex};

  mod app_state;
  mod error_map;
  mod commands;

  use app_state::AppState;

  fn main() {
    tauri::Builder::default()
      .setup(|app| {
        let app_state = AppState::new();
        app.manage(app_state);
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        commands::vault::vault_create,
        commands::vault::vault_open,
        commands::vault::vault_close,
        commands::vault::vault_lock,
        commands::questionnaire::import_questionnaire,
        commands::questionnaire::get_column_profiles,
        commands::questionnaire::save_column_mapping,
        commands::answer_bank::answer_bank_create,
        commands::answer_bank::answer_bank_update,
        commands::answer_bank::answer_bank_delete,
        commands::answer_bank::answer_bank_list,
        commands::matching::get_matching_suggestions,
        commands::export::generate_export_pack,
        commands::license::check_license_status,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
  ```
- **Prerequisites**: Cargo.toml updated (Step 5), core module available
- **Unlocked after**: Commands can be wired
- **Complexity**: Low

### **Step 7: Create AppState & Error Mapping**
- **Exact files**:
  - `apps/questionnaire/src-tauri/src/app_state.rs` (NEW)
  - `apps/questionnaire/src-tauri/src/error_map.rs` (NEW)
- **Changes required**:
  ```rust
  // app_state.rs
  use core::storage::VaultStorage;
  use std::sync::{Arc, Mutex};
  use uuid::Uuid;

  pub struct AppState {
    pub vault: Arc<Mutex<Option<VaultStorage>>>,
    pub actor: String,
    pub request_id: String,
  }

  impl AppState {
    pub fn new() -> Self {
      Self {
        vault: Arc::new(Mutex::new(None)),
        actor: "user@localhost".to_string(),  // TODO: Phase 5 - user auth
        request_id: Uuid::new_v4().to_string(),
      }
    }
  }

  // error_map.rs
  use core::domain::errors::CoreErrorCode;
  use serde::Serialize;

  #[derive(Serialize)]
  pub struct TauriError {
    pub code: String,
    pub message: String,
  }

  pub fn map_core_error(err: CoreErrorCode, message: String) -> TauriError {
    TauriError {
      code: format!("{:?}", err),  // Convert enum to string
      message,
    }
  }
  ```
- **Prerequisites**: core::domain::errors available
- **Unlocked after**: Commands (Steps 8+)
- **Complexity**: Low

### **Step 8: Wire 6 Command Modules**
- **Exact files** (CREATE & FILL each):
  - `apps/questionnaire/src-tauri/src/commands/vault.rs`
  - `apps/questionnaire/src-tauri/src/commands/questionnaire.rs` (update scaffolding)
  - `apps/questionnaire/src-tauri/src/commands/answer_bank.rs` (update scaffolding)
  - `apps/questionnaire/src-tauri/src/commands/matching.rs` (NEW)
  - `apps/questionnaire/src-tauri/src/commands/export.rs`
  - `apps/questionnaire/src-tauri/src/commands/license.rs` (update scaffolding)
- **Changes required** (example for vault.rs):
  ```rust
  use tauri::State;
  use core::storage::VaultStorage;
  use crate::app_state::AppState;
  use crate::error_map::map_core_error;

  #[tauri::command]
  pub async fn vault_create(
    path: String,
    name: String,
    state: State<'_, AppState>,
  ) -> Result<VaultDto, String> {
    match VaultStorage::create(&path, &name, &state.actor) {
      Ok(vault) => {
        *state.vault.lock().unwrap() = Some(vault.clone());
        Ok(VaultDto {
          id: vault.id().to_string(),
          name: vault.name().to_string(),
          path: vault.path().to_string(),
          created_at: vault.created_at().to_string(),
          locked: vault.locked(),
          license_status: check_license(&vault).await,
        })
      },
      Err(e) => Err(map_core_error(e.code, e.message).to_string()),
    }
  }

  // Similar for vault_open, vault_close, vault_lock
  ```
  - Each command:
    - Takes state parameter
    - Calls core module function
    - Maps errors via error_map
    - Returns Result<DtoType, String>
- **Prerequisites**: app_state, error_map, core modules available
- **Unlocked after**: Commands callable from React
- **Complexity**: Medium

### **Step 9: Update commands/mod.rs**
- **Exact files**: `apps/questionnaire/src-tauri/src/commands/mod.rs`
- **Changes required**:
  ```rust
  pub mod vault;
  pub mod questionnaire;
  pub mod answer_bank;
  pub mod matching;
  pub mod export;
  pub mod license;
  ```
- **Prerequisites**: All 6 command files created (Step 8)
- **Unlocked after**: main.rs can register all commands
- **Complexity**: Low

### **Step 10: Test Tauri IPC**
- **Exact files**: `apps/questionnaire/src-tauri/__tests__/command_tests.rs` (NEW)
- **Changes required**:
  ```rust
  #[tokio::test]
  async fn test_vault_create_command() {
    // Create AppState
    let state = AppState::new();
    // Call vault_create with test path
    // Verify response DTO structure
  }

  #[tokio::test]
  async fn test_error_mapping() {
    // Trigger invalid path error
    // Verify error DTO returned with correct code
  }
  ```
- **Prerequisites**: All commands wired (Step 8)
- **Unlocked after**: `tauri dev` launches successfully
- **Complexity**: Medium

---

## PHASE 2.6: REACT UI & STATE MANAGEMENT

### **Step 11: Setup React + Tooling**
- **Exact files**:
  - `apps/questionnaire/Cargo.toml` (update: add build deps)
  - `apps/questionnaire/package.json` (update: add deps, scripts)
  - `apps/questionnaire/tsconfig.json` (update or create)
  - `apps/questionnaire/vite.config.ts` (NEW)
  - `apps/questionnaire/tailwind.config.js` (NEW)
- **Changes required**:
  ```json
  // package.json (in apps/questionnaire)
  {
    "name": "questionnaire-app",
    "version": "0.1.0",
    "type": "module",
    "scripts": {
      "dev": "vite",
      "build": "vite build",
      "preview": "vite preview",
      "test": "vitest",
      "lint": "eslint src --ext ts,tsx"
    },
    "dependencies": {
      "@tauri-apps/api": "^2.0.0",
      "react": "^18.2.0",
      "react-dom": "^18.2.0",
      "react-router-dom": "^6.0.0",
      "zustand": "^4.0.0",
      "@tanstack/react-query": "^5.0.0",
      "zod": "^3.22.0",
      "@radix-ui/react-dialog": "^1.1.0",
      "@radix-ui/react-form": "^0.0.0",
      "@radix-ui/react-primitive": "^1.0.0",
      "tailwindcss": "^3.0.0"
    },
    "devDependencies": {
      "typescript": "^5.0.0",
      "vite": "^5.0.0",
      "vitest": "^1.0.0",
      "@testing-library/react": "^14.0.0",
      "@testing-library/user-event": "^14.0.0",
      "eslint": "^8.0.0",
      "prettier": "^3.0.0"
    }
  }
  ```
- **Prerequisites**: Tauri commands wired (Phase 2.5 complete)
- **Unlocked after**: React can compile
- **Complexity**: Low

### **Step 12: Create React Entry Point & Root Component**
- **Exact files**:
  - `apps/questionnaire/src/main.tsx` (NEW)
  - `apps/questionnaire/src/App.tsx` (NEW)
  - `apps/questionnaire/src/index.css` (NEW)
  - `apps/questionnaire/index.html` (NEW)
- **Changes required**:
  ```typescript
  // main.tsx
  import React from "react";
  import ReactDOM from "react-dom/client";
  import App from "./App";
  import "./index.css";

  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );

  // App.tsx
  import { BrowserRouter, Routes, Route } from "react-router-dom";
  import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
  import ImportPage from "./routes/Import";
  import MapPage from "./routes/Map";
  import AnswerBankPage from "./routes/AnswerBank";
  import ReviewPage from "./routes/Review";
  import ExportPage from "./routes/Export";

  const queryClient = new QueryClient();

  export default function App() {
    return (
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<ImportPage />} />
            <Route path="/map" element={<MapPage />} />
            <Route path="/answer-bank" element={<AnswerBankPage />} />
            <Route path="/review" element={<ReviewPage />} />
            <Route path="/export" element={<ExportPage />} />
          </Routes>
        </BrowserRouter>
      </QueryClientProvider>
    );
  }
  ```
- **Prerequisites**: package.json updated (Step 11)
- **Unlocked after**: Zustand stores, API wrapper
- **Complexity**: Low

### **Step 13: Create Zustand Stores (3 files)**
- **Exact files**:
  - `apps/questionnaire/src/state/importStore.ts`
  - `apps/questionnaire/src/state/answerBankStore.ts`
  - `apps/questionnaire/src/state/uiStore.ts`
- **Changes required**: (See section 3.4 for full implementations)
- **Prerequisites**: Zustand available, DTOs imported
- **Unlocked after**: Custom hooks
- **Complexity**: Low

### **Step 14: Create Tauri IPC Wrapper**
- **Exact files**: `apps/questionnaire/src/api/tauri.ts` (NEW)
- **Changes required**:
  ```typescript
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    VaultDto, ColumnProfileDto, AnswerBankEntryDto,
    MatchSuggestionDto, ExportPackDto, LicenseStatusDto
  } from "@packages/types";

  export async function invokeVaultCreate(
    path: string,
    name: string
  ): Promise<VaultDto> {
    return invoke("vault_create", { path, name });
  }

  export async function invokeImportQuestionnaire(
    vault_id: string,
    file_path: string,
    name: string
  ): Promise<QuestionnaireImportDto> {
    return invoke("import_questionnaire", { vault_id, file_path, name });
  }

  export async function invokeGetMatchingSuggestions(
    question: string,
    vault_id: string,
    top_n?: number
  ): Promise<MatchSuggestionDto[]> {
    return invoke("get_matching_suggestions", { question, vault_id, top_n });
  }

  // ... similar for all 14 commands
  ```
- **Prerequisites**: Tauri commands wired (Phase 2.5 complete)
- **Unlocked after**: Custom hooks
- **Complexity**: Low

### **Step 15: Create Custom Hooks (5 files)**
- **Exact files**:
  - `apps/questionnaire/src/hooks/useVault.ts`
  - `apps/questionnaire/src/hooks/useImport.ts`
  - `apps/questionnaire/src/hooks/useAnswerBank.ts`
  - `apps/questionnaire/src/hooks/useMatching.ts`
  - `apps/questionnaire/src/hooks/useTauriInvoke.ts`
- **Changes required** (example useImport.ts):
  ```typescript
  import { useCallback, useState } from "react";
  import { useImportStore } from "../state/importStore";
  import { invokeImportQuestionnaire, invokeGetColumnProfiles } from "../api/tauri";

  export function useImport() {
    const store = useImportStore();
    const [error, setError] = useState<string | null>(null);

    const importFile = useCallback(async (path: string, name: string) => {
      store.setLoading(true);
      try {
        const dto = await invokeImportQuestionnaire(path, name);
        store.setEntries(dto.column_profiles);
        store.setError(null);
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        setError(msg);
        store.setError(msg);
      } finally {
        store.setLoading(false);
      }
    }, [store]);

    return { importFile, error, ...store };
  }
  ```
- **Prerequisites**: Zustand stores, API wrapper complete
- **Unlocked after**: Page components
- **Complexity**: Medium

### **Step 16: Create Page Components (5 files)**
- **Exact files**:
  - `apps/questionnaire/src/routes/Import.tsx`
  - `apps/questionnaire/src/routes/Map.tsx`
  - `apps/questionnaire/src/routes/AnswerBank.tsx`
  - `apps/questionnaire/src/routes/Review.tsx`
  - `apps/questionnaire/src/routes/Export.tsx`
- **Changes required** (example Import.tsx):
  ```typescript
  import { useState } from "react";
  import { useNavigate } from "react-router-dom";
  import { useImportStore } from "../state/importStore";
  import ImportForm from "../components/features/ImportForm";
  import { Sidebar } from "../components/layout/Sidebar";

  export default function ImportPage() {
    const navigate = useNavigate();
    const store = useImportStore();

    const handleImportComplete = () => {
      store.setStep("map");
      navigate("/map");
    };

    return (
      <div className="flex h-screen">
        <Sidebar />
        <div className="flex-1 p-8">
          <h1 className="text-3xl font-bold mb-6">Import Questionnaire</h1>
          <ImportForm onComplete={handleImportComplete} />
        </div>
      </div>
    );
  }
  ```
- **Prerequisites**: Custom hooks, Zustand stores
- **Unlocked after**: Feature components
- **Complexity**: Medium

### **Step 17: Create Feature Components (5 files)**
- **Exact files**:
  - `apps/questionnaire/src/components/features/ImportForm.tsx`
  - `apps/questionnaire/src/components/features/ColumnMapTable.tsx`
  - `apps/questionnaire/src/components/features/AnswerBankTable.tsx`
  - `apps/questionnaire/src/components/features/MatchingResults.tsx`
  - `apps/questionnaire/src/components/features/ExportDialog.tsx`
- **Changes required** (example ImportForm.tsx):
  ```typescript
  import { useState } from "react";
  import { open } from "@tauri-apps/api/dialog";
  import Button from "../ui/Button";
  import Input from "../ui/Input";
  import { useImport } from "../../hooks/useImport";

  interface ImportFormProps {
    onComplete: () => void;
  }

  export default function ImportForm({ onComplete }: ImportFormProps) {
    const { importFile, loading, error } = useImport();
    const [name, setName] = useState("");

    const handleSelectFile = async () => {
      const filePath = await open({
        filters: [{ name: "Excel/CSV", extensions: ["xlsx", "csv"] }],
      });
      if (filePath) {
        await importFile(filePath as string, name);
        onComplete();
      }
    };

    return (
      <div className="space-y-4">
        <Input
          label="Questionnaire Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <Button onClick={handleSelectFile} disabled={loading}>
          {loading ? "Importing..." : "Select File"}
        </Button>
        {error && <p className="text-red-500">{error}</p>}
      </div>
    );
  }
  ```
- **Prerequisites**: UI components (Step 18), hooks
- **Unlocked after**: Demo ready
- **Complexity**: Medium

### **Step 18: Create Radix UI Wrapper Components (6 files)**
- **Exact files**:
  - `apps/questionnaire/src/components/ui/Button.tsx`
  - `apps/questionnaire/src/components/ui/Dialog.tsx`
  - `apps/questionnaire/src/components/ui/Form.tsx`
  - `apps/questionnaire/src/components/ui/Input.tsx`
  - `apps/questionnaire/src/components/ui/Table.tsx`
  - `apps/questionnaire/src/components/ui/Toast.tsx`
- **Changes required** (example Button.tsx):
  ```typescript
  import * as React from "react";
  import { cva, type VariantProps } from "class-variance-authority";

  const buttonVariants = cva(
    "inline-flex items-center justify-center px-4 py-2 rounded-md font-medium",
    {
      variants: {
        variant: {
          default: "bg-blue-600 text-white hover:bg-blue-700",
          outline: "border border-gray-300 bg-white hover:bg-gray-50",
        },
      },
      defaultVariants: {
        variant: "default",
      },
    }
  );

  interface ButtonProps
    extends React.ButtonHTMLAttributes<HTMLButtonElement>,
      VariantProps<typeof buttonVariants> {}

  const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
    ({ className, variant, ...props }, ref) => (
      <button
        ref={ref}
        className={buttonVariants({ variant, className })}
        {...props}
      />
    )
  );

  export default Button;
  ```
- **Prerequisites**: Tailwind configured, Radix UI installed
- **Unlocked after**: Feature components
- **Complexity**: Low

### **Step 19: Create Layout Components (2 files)**
- **Exact files**:
  - `apps/questionnaire/src/components/layout/Sidebar.tsx`
  - `apps/questionnaire/src/components/layout/Header.tsx`
- **Changes required**: Navigation, progress indicator, help text
- **Prerequisites**: UI components, Router
- **Unlocked after**: Pages complete
- **Complexity**: Low

---

## PHASE 2.7: FRONTEND TESTING & POLISH

### **Step 20: Setup Vitest & React Testing Library**
- **Exact files**: `apps/questionnaire/vitest.config.ts` (NEW)
- **Changes required**:
  ```typescript
  import { defineConfig } from "vitest/config";
  import react from "@vitejs/plugin-react";

  export default defineConfig({
    plugins: [react()],
    test: {
      globals: true,
      environment: "jsdom",
      setupFiles: ["./src/__tests__/setup.ts"],
    },
  });
  ```
- **Prerequisites**: vitest + @testing-library/react installed
- **Unlocked after**: Tests can run
- **Complexity**: Low

### **Step 21: Write Component Tests (4 files)**
- **Exact files**:
  - `apps/questionnaire/src/__tests__/ImportForm.test.tsx`
  - `apps/questionnaire/src/__tests__/ColumnMapTable.test.tsx`
  - `apps/questionnaire/src/__tests__/AnswerBankTable.test.tsx`
  - `apps/questionnaire/src/__tests__/ExportDialog.test.tsx`
- **Changes required**:
  ```typescript
  import { render, screen, userEvent } from "@testing-library/react";
  import ImportForm from "../components/features/ImportForm";

  describe("ImportForm", () => {
    it("should render import button", () => {
      const onComplete = vi.fn();
      render(<ImportForm onComplete={onComplete} />);
      expect(screen.getByText(/Select File/i)).toBeInTheDocument();
    });

    it("should call onComplete when file imported", async () => {
      const onComplete = vi.fn();
      const user = userEvent.setup();
      render(<ImportForm onComplete={onComplete} />);
      // Mock file selection
      // Verify onComplete called
    });
  });
  ```
- **Prerequisites**: Vitest configured; mocked Tauri API
- **Unlocked after**: CI tests pass
- **Complexity**: Medium

### **Step 22: Write Integration/E2E Test**
- **Exact files**: `apps/questionnaire/src/__tests__/e2e/questionnaire_flow.test.tsx` (NEW)
- **Changes required**:
  ```typescript
  describe("Questionnaire Flow E2E", () => {
    it("should complete full workflow: import → map → review → export", async () => {
      // 1. Render App (full router + stores)
      // 2. Mock Tauri commands
      // 3. User imports file
      // 4. User maps columns
      // 5. System suggests matches (from matching algorithm)
      // 6. User accepts/rejects suggestions
      // 7. User exports pack
      // 8. Verify export file created
    });
  });
  ```
- **Prerequisites**: Component tests passing
- **Unlocked after**: Phase 2.7 complete
- **Complexity**: High

### **Step 23: Add TypeScript Linting & Formatting**
- **Exact files**:
  - `.eslintrc.json` (NEW, root level)
  - `.prettierrc.json` (NEW, root level)
  - Update `scripts/lint.sh` and `scripts/format.sh`
- **Changes required**:
  ```json
  // .eslintrc.json
  {
    "env": {
      "browser": true,
      "es2021": true
    },
    "extends": [
      "eslint:recommended",
      "plugin:react/recommended",
      "plugin:@typescript-eslint/recommended"
    ],
    "parserOptions": {
      "ecmaVersion": "latest",
      "sourceType": "module"
    },
    "rules": {
      "react/react-in-jsx-scope": "off"
    }
  }
  ```
- **Prerequisites**: eslint, prettier installed
- **Unlocked after**: CI enforces standards
- **Complexity**: Low

### **Step 24: Create Runbook & Documentation**
- **Exact files**:
  - `docs/11_RUNBOOK.md` (NEW)
  - `docs/12_API_REFERENCE.md` (NEW)
  - `docs/13_DATABASE_SCHEMA.md` (NEW)
- **Changes required**:
  - **11_RUNBOOK.md**: How to build, run `tauri dev`, test, package
  - **12_API_REFERENCE.md**: Full Tauri command reference (params, responses, errors)
  - **13_DATABASE_SCHEMA.md**: ERD, migration order, indices
- **Prerequisites**: All code complete
- **Unlocked after**: Team can onboard
- **Complexity**: Low

### **Step 25: Demo Ready - Update Build Scripts**
- **Exact files**:
  - `scripts/build.sh` (update)
  - `scripts/demo.sh` (NEW)
  - Update root `package.json`
- **Changes required**:
  ```bash
  # scripts/demo.sh
  #!/bin/bash
  set -e
  echo "=== Compliance Suite Demo ==="
  echo "Building Rust core..."
  cargo build -p core
  echo "Launching Questionnaire app (dev mode)..."
  cd apps/questionnaire
  npm run dev
  ```
- **Prerequisites**: All steps 1-24 complete
- **Unlocked after**: Demo launchable
- **Complexity**: Low

---

# 5. ERROR HANDLING

## 5.1 FAILURE MODES & RECOVERY BY PHASE

### **Phase 2.4: Matching Algorithm**

| Failure Mode | Prevention | Recovery |
|--------------|-----------|----------|
| Empty answer bank | Validate answer_bank has ≥1 entry before matching | Return empty suggestions list; UI shows "no suggestions available" |
| Score calculation NaN | Use `saturating_add` for token counts; normalize by union length | Default to 0.0 score; log anomaly |
| Database migration fails | Run migrations idempotent; test 0006 against prior states | Roll back to 0005; user retries with `core::storage::migrate()` |
| Unicode normalization issues | Use `str::to_lowercase()` for ASCII; handle combining chars | Normalize to NFD before token split |

### **Phase 2.5: Tauri Integration**

| Failure Mode | Prevention | Recovery |
|--------------|-----------|----------|
| Vault locked when command runs | Acquire read lock before command; check locked status | Return `VAULT_LOCKED` error; UI shows "Vault is locked" toast |
| File not found on import | Validate file path exists before passing to core | Return `FILE_NOT_FOUND` error with suggested path |
| Command serialization fails | Validate DTO fields non-null; use Zod validators | Return `INVALID_DTO` error; log serde error |
| IPC message too large | Paginate results if > 1MB; stream large exports | Return paginated response; frontend fetches next page |
| AppState mutex poisoned | Never panic while holding lock; use unwrap_or_default | Return `INTERNAL_ERROR`; restart Tauri app |

### **Phase 2.6: React UI**

| Failure Mode | Prevention | Recovery |
|--------------|-----------|----------|
| Zustand state out of sync | Keep state derived from server responses; don't duplicate | Refresh state via `useEffect` on mount; show "Sync" button |
| File upload fails | Validate file size (< 100MB); check MIME type | Show error toast with reason; allow retry |
| Tauri command timeout | Set 30s timeout on all invokes; show loading state | Show "Request timed out" error; allow user to retry |
| React Router state loss | Use query params + localStorage for recovery | Restore last page + query from localStorage on mount |

### **Phase 2.7: Testing**

| Failure Mode | Prevention | Recovery |
|--------------|-----------|----------|
| Flaky tests (timing) | Use `waitFor()` instead of `sleep()`; mock async operations | Re-run tests 3x; flag as flaky in CI if still failing |
| Mocked Tauri API calls wrong | Verify mock signatures match Tauri v2 API | Update mocks; run against real Tauri app in integration mode |
| Test data inconsistent | Use fixtures from `/fixtures/matching_baseline/` | Regenerate fixtures; commit new golden files |

---

## 5.2 INVALID INPUT VALIDATION

### **Frontend Validation (Zod Schemas)**
```typescript
// Before any Tauri invoke
const input = MatchingInputSchema.parse(userInput);
// If invalid, Zod throws; catch in component
try {
  await invokeGetMatches(input);
} catch (err) {
  if (err instanceof z.ZodError) {
    showToast(err.errors[0].message);
  }
}
```

### **Tauri Command Validation**
```rust
// In each command, validate input before calling core
if path.is_empty() {
  return Err("path cannot be empty".to_string());
}
if !std::path::Path::new(&path).exists() {
  return Err(format!("path does not exist: {}", path));
}
// Call core
```

### **Core Input Validation**
```rust
// In core functions
pub fn import_questionnaire(path: &str, name: &str) -> Result<ImportResult, CoreError> {
  if path.is_empty() {
    return Err(CoreError::InvalidInput("path empty".to_string()));
  }
  if name.len() < 1 || name.len() > 256 {
    return Err(CoreError::InvalidInput("name length 1-256".to_string()));
  }
  // Process
}
```

---

## 5.3 NETWORK & SERVICE FAILURES

**All operations are local (no network)**, so failures are:
- File I/O: No such file, permission denied → Return `IO_ERROR` with path
- Database: Corruption, locked → Return `DATABASE_ERROR`; suggest recovery
- State corruption: Hash chain invalid → Return `AUDIT_CHAIN_INVALID`; don't allow mutations until fixed

---

## 5.4 LOGGING & MONITORING POINTS

```rust
// Core
info!("vault_create: path={}, actor={}", path, actor);
error!("import failed: {}", err);
debug!("matching scored {} answers in {}ms", count, elapsed);

// Tauri
info!("command: vault_create request_id={}", state.request_id);
error!("command failed: code={}, message={}", err.code, err.message);

// React
console.log("useImport: importFile started");
console.error("ImportForm: Tauri invoke failed", err);
```

---

# 6. TESTING STRATEGY

## 6.1 UNIT TESTS

### **Rust Core (Phase 2.4+)**

| Module | What to Test | Fixtures |
|--------|--------------|----------|
| `questionnaire/matching.rs` | Token normalization, scoring, top-N ranking | `/fixtures/matching_baseline/answers.json` (20 answers) |
| `questionnaire/matching.rs` | Edge cases: empty question, unicode, stopwords | Test cases in golden fixtures |
| Database migrations | Schema evolution idempotent | Run 0001-0006 sequentially; verify schema |

### **React Components (Phase 2.7)**

| Component | What to Test | Mocks |
|-----------|--------------|-------|
| `ImportForm` | Renders correctly; file dialog opens; calls API | Mock `tauri invoke` |
| `ColumnMapTable` | Renders columns; allows mapping; calls save API | Mock API responses |
| `AnswerBankTable` | CRUD operations; pagination; search | Mock API |
| Hooks | `useImport`, `useAnswerBank`, `useMatching` fetch data | Mock API |

---

## 6.2 INTEGRATION TESTS

### **Tauri Commands**

```rust
#[tokio::test]
async fn test_vault_create_to_import() {
  // 1. Create vault
  // 2. Import questionnaire
  // 3. Verify questionnaire in DB
  // 4. Verify audit events recorded
}
```

### **Core + Tauri**

```typescript
// E2E: React → Tauri → Core → Database
test("questionnaire import flow", async () => {
  // 1. User selects file in ImportForm
  // 2. Tauri command runs core::import_questionnaire
  // 3. Database updated
  // 4. React component shows column profiles
  // 5. User maps columns
  // 6. Column mapping saved to DB
});
```

---

## 6.3 VERIFICATION CHECKLIST (Per Step)

| Step | Verification Command | Expected |
|------|----------------------|----------|
| 1 | `cargo test -p core -- migration_tests` | ✓ All migrations apply idempotent |
| 2 | `cargo test -p core -- matching` | ✓ Scoring correct; top-N ranked |
| 4 | `cargo test -p questionnaire` | ✓ All unit tests pass |
| 5 | `cargo build --manifest-path apps/questionnaire/src-tauri/Cargo.toml` | ✓ Compiles |
| 6 | `cd apps/questionnaire && npm run dev` | ✓ Tauri window launches |
| 8 | `curl -X POST http://localhost:8080 -d '{"method":"vault_create",...}'` | ✓ Command responds |
| 11 | `cd apps/questionnaire && npm install && npm run build` | ✓ React builds |
| 12 | `cd apps/questionnaire && npm run dev` | ✓ React dev server on :5173 |
| 16 | `cd apps/questionnaire && npm run test` | ✓ All component tests pass |
| 20 | `cd apps/questionnaire && npm run test -- --ui` | ✓ Vitest UI launches |

---

# 7. EXPLICIT ASSUMPTIONS

## 7.1 DATA ASSUMPTIONS

- **Answer Bank**: ≥ 1 answer exists before matching (error if empty)
- **Questions**: Text-only; no embedded HTML, binary, or special encoding
- **File Paths**: UTF-8 encoded; absolute paths only (no relative paths)
- **Database**: SQLite file fits on disk; no external DB required
- **Vault**: Single vault open at a time per AppState (no multi-vault sessions)

## 7.2 USER BEHAVIOR ASSUMPTIONS

- **Actor Field**: Provided by user at vault creation; no dynamic auth provider
- **File Selection**: User has OS file dialog access (Tauri requirement)
- **Network**: None required; all operations local
- **Export**: User manually downloads files; no automatic uploading

## 7.3 SYSTEM CONSTRAINTS

- **Memory**: Answer bank fits in RAM (~100MB per 100k entries)
- **Disk**: Export packs can be large (1GB+); sufficient free space assumed
- **CPU**: Matching algorithm O(n*m) where n=questions, m=answers; acceptable for <100k answers
- **UI**: Desktop resolution ≥ 1024x768 assumed

## 7.4 EXTERNAL DEPENDENCIES

| Dependency | Version | Critical | Fallback |
|------------|---------|----------|----------|
| ed25519-dalek | 2.1.1 | Yes (license verification) | None (must have) |
| tauri | v2 | Yes (desktop framework) | None (must have) |
| React | 18+ | Yes (UI) | None (must have) |
| SQLite | 3.x | Yes (data persistence) | None (must have) |

---

# 8. QUALITY GATE

## APPROVAL CHECKLIST

- [x] **Logical Flow**: Each step follows from previous (no circular dependencies)
- [x] **Actionability**: Every step can be executed without clarification questions
- [x] **Completeness**: All 25 steps account for full Phase 2.4-2.7 implementation
- [x] **Testing**: Each step has clear verification criteria
- [x] **Error Handling**: Failure modes documented; recovery strategies provided
- [x] **API Contracts**: All 14 Tauri commands fully specified (request/response/errors)
- [x] **Database Schema**: Migrations 0006-0008 complete with relationships
- [x] **File Structure**: Every file to be created/modified listed with purpose
- [x] **Dependencies**: All npm/cargo packages specified with rationale
- [x] **Assumptions**: Implicit assumptions made explicit

## RISKS & MITIGATIONS

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Tauri IPC serialization incompatibility | Low | High | Mock Tauri API early (Step 10); test with real Tauri (Phase 2.7) |
| React Router state loss on refresh | Medium | Medium | Use query params + localStorage for recovery |
| Matching algorithm too slow (O(n²)) | Low | High | Profile early (Phase 2.4); optimize token indexing if needed |
| SQLite shell invocation fragile | Medium | Medium | Add migration to `rusqlite` crate in Phase 3 if issues arise |
| Timezone issues in audit timestamps | Low | Low | Use UTC everywhere; document in 03_DATA_MODEL.md |

---

## JUDGMENT CALLS MADE

1. **Shell-based SQLite**: Kept for Phase 2 MVP; flagged for refactor in Phase 3 with async Tauri commands
2. **Zustand over Redux**: Simpler state model for 3 stores; trade-off: less middleware flexibility
3. **No encryption-at-rest yet**: Deferred to Phase 5; okay for internal-only MVP
4. **Token-overlap matching (not ML)**: Deterministic, testable, offline; no ML framework bloat
5. **File I/O validation at 3 layers** (React + Tauri + Core): Paranoid but safe for compliance app
6. **Fixtures in-repo, not generated**: Golden tests more reliable; fixtures versioned with code

---

## SIGN-OFF

**STATUS**: ✅ **APPROVED**

This plan is unambiguous, complete, and executable. Every step has clear prerequisites, deliverables, and verification criteria. No clarifying questions required before implementation.

**Plan prepared**: 2026-02-12 15:30 UTC
**Authority**: Senior Software Engineer / VP of Engineering
**Codex Ready**: Yes

---

**NEXT ACTION**: Begin Phase 2.4, Step 1 (Create Database Migration for Matching)

