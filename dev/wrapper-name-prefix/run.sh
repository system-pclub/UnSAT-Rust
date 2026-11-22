#!/usr/bin/env bash
# Usage: dev/wrapper-name-prefix/run.sh
# The report's callee is `Box::from_raw`, reached inside the wrapper
# `Holder::from_raw_parts` whose name starts with it. autoinj used to find the
# callee name inside the statement by substring, concluded that the call was
# there, then failed to match it ("no call expression found") and aborted the
# whole crate. The marker is now placed at the wrapper's call.
# Rebuild the autoinj release binary first (x runs tools/autoinj/target/release).
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=wrapper-name-prefix
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" \
    --rule-dsl dev/wrapper-name-prefix/rules.json \
    --studied-rules dev/wrapper-name-prefix/studied_rules \
    --rule step2-alloc-rule-1126 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { grep -n "^Error" -A3 ".local/verify-$case_name.log" | head -12; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(f"{row['callsite']:<20} {row['caller']}")
E
cat "$actual"
diff -u dev/wrapper-name-prefix/expected.txt "$actual" \
    && echo "wrapper-name-prefix: wrapper callsite injected and analysed"
