#!/usr/bin/env bash
set -euo pipefail
# Preserve every rustc invocation's JSON; upstream overwrites a single fixed path.
if [[ -n "${CULPA_FINDINGS_DIR:-}" ]]; then
    mkdir -p "$CULPA_FINDINGS_DIR"
    crate_name=unknown
    previous=
    for argument in "$@"; do
        if [[ "$previous" == --crate-name ]]; then crate_name="$argument"; fi
        previous="$argument"
    done
    export CULPA_FINDINGS_JSON="$CULPA_FINDINGS_DIR/${crate_name}-$$.json"
fi
exec /usr/local/cargo/bin/culpa-driver "$@"
