#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FASTCHECK_DIR="$ROOT_DIR/tools/fastcheck"
CRATES_DIR="$ROOT_DIR/.local/crates"

if [[ ! -d "$CRATES_DIR" ]]; then
    echo "missing crates directory: $CRATES_DIR" >&2
    exit 1
fi

cargo build --release --manifest-path "$FASTCHECK_DIR/Cargo.toml"
"$FASTCHECK_DIR/target/release/fastcheck" "$@" "$CRATES_DIR" > fastcheck.log
