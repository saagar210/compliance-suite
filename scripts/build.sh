#!/usr/bin/env bash
set -euo pipefail

echo "=== Building Compliance Suite ==="

echo ""
echo "Building Rust core library..."
cargo build --workspace

echo ""
echo "Building Questionnaire app (frontend)..."
cd apps/questionnaire
npm run build
cd ../..

echo ""
echo "Building Tauri desktop app..."
cd apps/questionnaire
npm run tauri build
cd ../..

echo ""
echo "=== Build Complete ==="
