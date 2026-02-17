#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Heavy build outputs that are safe to regenerate.
HEAVY_PATHS=(
  "target"
  "apps/questionnaire/dist"
  "apps/questionnaire/node_modules/.vite"
  "apps/questionnaire/src-tauri/target"
  "apps/questionnaire/src-tauri/gen"
  "apps/binder/src-tauri/target"
  "apps/sop/src-tauri/target"
)

removed=0
for path in "${HEAVY_PATHS[@]}"; do
  if [[ -e "$path" ]]; then
    rm -rf "$path"
    echo "removed $path"
    removed=1
  fi
done

if [[ "$removed" -eq 0 ]]; then
  echo "no heavy build artifacts found"
fi
