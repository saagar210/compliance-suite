#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LEAN_TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/compliance-suite-lean.XXXXXX")"

cleanup() {
  local rc=$?
  rm -rf "$LEAN_TMP_ROOT"
  "$ROOT_DIR/scripts/clean-heavy.sh" >/dev/null 2>&1 || true
  exit "$rc"
}

trap 'exit 130' INT
trap 'exit 143' TERM
trap cleanup EXIT

export CARGO_TARGET_DIR="$LEAN_TMP_ROOT/target"
export CARGO_BUILD_TARGET_DIR="$CARGO_TARGET_DIR"
export VITE_CACHE_DIR="$LEAN_TMP_ROOT/vite-cache"

mkdir -p "$VITE_CACHE_DIR"

echo "lean dev cache root: $LEAN_TMP_ROOT"
echo "cargo target dir: $CARGO_TARGET_DIR"
echo "vite cache dir: $VITE_CACHE_DIR"

echo "starting questionnaire app in lean mode..."
cd "$ROOT_DIR"
pnpm --dir apps/questionnaire run tauri dev
