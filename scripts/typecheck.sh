#!/usr/bin/env bash
set -euo pipefail

# Placeholder typecheck for the TS workspace. Once UI work starts, this should
# run package-level typechecks (tsc) via pnpm -r.

# Keep Rust compilation healthy as the primary early signal.
cargo check --workspace
