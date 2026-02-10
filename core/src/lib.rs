//! Compliance Suite core platform.
//!
//! Phase 1 provides: SQLite-backed storage, evidence filesystem, tamper-evident
//! audit log hash chain, and deterministic export packs.

pub mod prelude;

pub mod audit;
pub mod domain;
pub mod export;
pub mod storage;
pub mod util;
