#!/usr/bin/env bash
# Usage: dev/nonnull-argument-binding/run.sh
# Verifies that the callee argument bindings injected for a `NonNull` field
# receiver (`&NonNull<T>`) compile, link and run (NonNull::add, rule-512), and
# diffs the status with expected.txt.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=nonnull-argument-binding
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" \
    --rule rule-512 \
    --skip-llm-testcase --ir-output-dir ".local/$case_name-ir" \
    >".local/verify-$case_name.log" 2>&1 || { tail -30 ".local/verify-$case_name.log"; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: (r["callsite"], r["rule"])):
    if row["callsite"].endswith("-as_ref"):
        continue  # as_ref is paired with the add contract only by callee location
    print(f"{row['caller']:<14} {row['callsite']:<26} {row['rule']:<9} {row['status']}")
E
cat "$actual"
diff -u dev/nonnull-argument-binding/expected.txt "$actual" \
    && echo "nonnull-argument-binding: all statuses as expected"
