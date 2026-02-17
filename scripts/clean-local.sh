#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

"$ROOT_DIR/scripts/clean-heavy.sh"

# Full local reproducible cleanup: dependency installs + local package-manager cache.
FULL_PATHS=(
  "node_modules"
  "apps/questionnaire/node_modules"
  "apps/binder/node_modules"
  "apps/sop/node_modules"
  "packages/ui/node_modules"
  "packages/types/node_modules"
  ".pnpm-store"
)

removed=0
for path in "${FULL_PATHS[@]}"; do
  if [[ -e "$path" ]]; then
    rm -rf "$path"
    echo "removed $path"
    removed=1
  fi
done

if [[ "$removed" -eq 0 ]]; then
  echo "no additional local caches found"
fi
