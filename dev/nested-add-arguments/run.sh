#!/usr/bin/env bash
# Usage: dev/nested-add-arguments/run.sh
# An #[inline(always)] helper whose body uses `add`/`offset_from` yields a "root"
# marker at its call. Binding that marker's arguments used to be hoisted in front
# of the enclosing `unsafe` block, naming `src`/`dst` before their `let`
# (E0425). The crate must inject and link; unplaceable markers are skipped.
# Rebuild the autoinj release binary first (x runs tools/autoinj/target/release).
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=nested-add-arguments
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --rule rule-512 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { grep -n "^error" -A6 ".local/verify-$case_name.log" | head -30; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(row["callsite"])
E
cat "$actual"
diff -u dev/nested-add-arguments/expected.txt "$actual" \
    && echo "nested-add-arguments: crate injected, linked and run"
