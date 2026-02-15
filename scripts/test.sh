#!/usr/bin/env bash
set -euo pipefail

echo "=== Running All Tests ==="

echo ""
echo "Running Rust core tests..."
cargo test -p core

echo ""
echo "Running Questionnaire app tests (Vitest)..."
cd apps/questionnaire
npm test -- --run
cd ../..

echo ""
echo "=== All Tests Complete ==="
