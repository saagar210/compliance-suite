#!/usr/bin/env bash
set -euo pipefail

echo "=== Formatting Rust code ==="
cargo fmt --all

echo ""
echo "=== Formatting TypeScript code ==="
pnpm --dir apps/questionnaire exec prettier --write "src/**/*.{ts,tsx,js,jsx,json,css}"
