#!/usr/bin/env bash
# Usage: dev/wrapper-callee-target/run.sh
# The marker of `NonNull::new_unchecked` (rule 843) sits at a call to the
# crate-local wrapper `Buf::from_raw`, and a core call (`str::as_bytes`) follows
# it in the caller. The rule must be evaluated at the wrapper's inner
# `new_unchecked` call, not at the next core call (which gave an eval-error:
# "pointee of a view that is not a pointer slot").
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=wrapper-callee-target
rm -rf ".local/verify/$case_name"
for rule in step2-core-rule-843 step2-core-rule-659; do
    rm -rf ".local/verify/$case_name"
    ./x verify "benchmark/typerules/$case_name" \
        --rule-dsl dev/wrapper-callee-target/rules.json \
        --studied-rules dev/wrapper-callee-target/studied_rules \
        --rule "$rule" --skip-llm-testcase \
        --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
        || { tail -20 ".local/verify-$case_name.log"; exit 1; }
    cp ".local/verify/$case_name/full-matrix-results.json" ".local/verify-$case_name-$rule.json"
done
python3 - "$case_name" <<'E'
import json, sys
status = {}
for rule in ("step2-core-rule-843", "step2-core-rule-659"):
    data = json.load(open(f".local/verify-{sys.argv[1]}-{rule}.json"))
    for row in data["results"]:
        if row["status"] != "not-in-ir":
            status[(row["caller"], rule)] = row["status"]
for key, value in sorted(status.items()):
    print(f"{key[0]:<24} {key[1]:<20} {value}")
assert "eval-error" not in status.values(), "rule evaluated against the wrong call"
# An arbitrary pointer argument can be null or misaligned.
assert status[("Buf::make_unchecked", "step2-core-rule-843")] == "violation", status
assert status[("read_pair", "step2-core-rule-659")] == "violation", status
print("wrapper-callee-target: rule evaluated at the wrapper's inner callee")
E
