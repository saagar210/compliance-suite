#!/usr/bin/env bash
set -euo pipefail

# Lint is currently focused on Rust formatting + clippy to keep main green.
# If clippy is not available in an environment, this should fail loudly.

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
