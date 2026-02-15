#!/usr/bin/env bash
set -euo pipefail

echo "=== Compliance Suite Demo ==="

echo ""
echo "Building Rust core library..."
cargo build -p core

echo ""
echo "Launching Questionnaire app (dev mode)..."
cd apps/questionnaire
npm run tauri dev
