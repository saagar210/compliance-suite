#!/usr/bin/env bash
set -euo pipefail

echo "=== Building Compliance Suite ==="

echo ""
echo "Building Rust core library..."
cargo build --workspace

echo ""
echo "Building Tauri desktop app..."
pnpm --dir apps/questionnaire run tauri build

echo ""
echo "=== Build Complete ==="
