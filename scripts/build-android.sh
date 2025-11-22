#!/usr/bin/env bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if ! command -v panpan-apk >/dev/null 2>&1; then
  echo "panpan-apk not found — building locally..."
  (cd "$ROOT/tools/panpan-apk" && cargo build --release)
  export PATH="$ROOT/tools/panpan-apk/target/release:$PATH"
fi

echo "Running panpan-apk for example_crate..."
panpan-apk --crate-path "$ROOT/example_crate" --android-template "$ROOT/android" --release --install
