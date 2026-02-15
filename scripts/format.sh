#!/usr/bin/env bash
set -euo pipefail

echo "=== Formatting Rust code ==="
cargo fmt --all

echo ""
echo "=== Formatting TypeScript code ==="
cd apps/questionnaire
npx prettier --write "src/**/*.{ts,tsx,js,jsx,json,css}"
cd ../..
