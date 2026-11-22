#!/usr/bin/env bash
# Usage: dev/macro-duplicate-callsite/run.sh
# A generic caller (impl<T> Drop for Holder<T>) with two unsafe callsites is
# instantiated for two types in the lib test. Injection must not export a marker
# symbol from it (one definition per instantiation) nor stack several exports on
# one function. Passes when the crate injects, links and every callsite is run.
# Rebuild the autoinj release binary first: x uses tools/autoinj/target/release.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=macro-duplicate-callsite
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --rule rule-512 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { grep -n "^error" -A6 ".local/verify-$case_name.log" | head -30; exit 1; }
if grep -q "export_name" "crates_inj/$case_name/src/main.rs"; then
    echo "generic caller exported a marker symbol"; exit 1
fi
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(f"{row['callsite']:<20} {row['caller']}")
E
cat "$actual"
diff -u dev/macro-duplicate-callsite/expected.txt "$actual" \
    && echo "macro-duplicate-callsite: all callsites injected and run"
