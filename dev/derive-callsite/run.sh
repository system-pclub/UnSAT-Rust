#!/usr/bin/env bash
# Usage: dev/derive-callsite/run.sh
# zerocopy's derive macros generate unsafe calls that are reported at the
# `#[derive]` line. autoinj used to abort the whole crate ("no call expression
# found"); those callsites are now skipped with a warning and the others run.
# Rebuild the autoinj release binary first (x runs tools/autoinj/target/release).
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=derive-callsite
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --rule rule-512 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { grep -n "^Error\|^error" -A4 ".local/verify-$case_name.log" | head -20; exit 1; }
grep -q "skipping callsite src-main-rs-5-21" ".local/verify-$case_name.log" \
    || { echo "derive callsite was not skipped"; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(row["callsite"])
E
cat "$actual"
diff -u dev/derive-callsite/expected.txt "$actual" \
    && echo "derive-callsite: derive-generated callsites skipped, crate analysed"
