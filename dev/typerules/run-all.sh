#!/usr/bin/env bash
# Usage: dev/typerules/run-all.sh
# Runs every case named in dev/typerules/cases.json (after rebuilding
# rules.json inside the image) and diffs the statuses with expected.txt.
# Build KLEE first with dev/typerules/build-klee.sh when its sources changed.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
docker run --rm --user "$(id -u):$(id -g)" -e PYTHONDONTWRITEBYTECODE=1 \
    -v "$repo:/unsat-rust" -w /unsat-rust unsat-rust:local \
    /opt/x/.venv/bin/python dev/typerules/build_rules.py >/dev/null
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
for case_name in $(python3 -c "
import json
seen = []
for case in json.load(open('dev/typerules/cases.json')):
    if case['case'] not in seen:
        seen.append(case['case'])
print(' '.join(seen))"); do
    dev/typerules/matrix.sh "$case_name" >>"$actual"
done
diff -u dev/typerules/expected.txt "$actual" && echo "typerules: all statuses as expected"
