#!/usr/bin/env bash
# Usage: dev/rawptr-unsized-deref/run.sh
# Injects and verifies the raw-pointer dereferences of
# benchmark/typerules/rawptr-unsized-deref (fat pointers to str and [u8]) and
# diffs the status per caller with expected.txt.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=rawptr-unsized-deref
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { tail -30 ".local/verify-$case_name.log"; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["caller"]):
    print(f"{row['caller']:<20} {row['status']}")
E
cat "$actual"
diff -u dev/rawptr-unsized-deref/expected.txt "$actual" \
    && echo "rawptr-unsized-deref: all statuses as expected"
