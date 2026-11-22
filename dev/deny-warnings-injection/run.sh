#!/usr/bin/env bash
# Usage: dev/deny-warnings-injection/run.sh
# A crate with `#![deny(warnings)]` must still compile after injection: the raw
# pointer operand's own parentheses are not repeated in the injected macro call
# (`unused_parens` is an error there). Rebuild the autoinj release binary first.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=deny-warnings-injection
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { grep -n "^error" -A5 ".local/verify-$case_name.log" | head -20; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(f"{row['callsite']:<20} {row['caller']:<14} {row['status']}")
E
cat "$actual"
diff -u dev/deny-warnings-injection/expected.txt "$actual" \
    && echo "deny-warnings-injection: injected code is lint-clean"
