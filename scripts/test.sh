#!/usr/bin/env bash
set -euo pipefail

echo "=== Running All Tests ==="

echo ""
echo "Running Rust core tests..."
cargo test -p core

echo ""
echo "Running Questionnaire app tests (Vitest)..."
pnpm --dir apps/questionnaire exec vitest run

echo ""
echo "=== All Tests Complete ==="
