#!/usr/bin/env bash
# Hourly, serial development maintenance requested by the repository owner.
# The crate verifier itself continues to use --skip-llm-testcase.
set -euo pipefail
repo="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
state="$repo/.local/klee-maintenance"
mkdir -p "$state"
exec 8>"$state/maintenance.lock"
flock -n 8 || exit 0
if [[ -e "$state/manual-active" ]]; then
    printf '%s skipped: interactive maintenance active\n' "$(date -u +%FT%TZ)" >> "$state/timer.log"
    exit 0
fi
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
job="$state/$stamp"
mkdir "$job"
printf '%s\n' "$job" > "$state/latest"
printf 'running\n' > "$job/status"
printf '%s started %s\n' "$(date -u +%FT%TZ)" "$job" >> "$state/timer.log"
codex_bin="${CODEX_BIN:-/home/sxia/.local/bin/codex}"
set +e
"$codex_bin" exec -C "$repo" --sandbox danger-full-access \
    -c 'approval_policy="never"' --color never \
    -o "$job/summary.md" - < dev/run-all-crates-maintenance-prompt.txt \
    > "$job/agent.log" 2>&1
result=$?
set -e
printf '%s\n' "$result" > "$job/exit-code"
if (( result == 0 )); then printf 'finished\n' > "$job/status"; else printf 'failed\n' > "$job/status"; fi
printf '%s ended exit=%s %s\n' "$(date -u +%FT%TZ)" "$result" "$job" >> "$state/timer.log"
exit "$result"
