#!/usr/bin/env bash
# Usage: dev/typerules/matrix.sh <case> [extra ./x verify args]
# Verifies every callsite of benchmark/typerules/<case> against the rules in
# dev/typerules/rules.json selected by callee location, without PoC generation,
# then prints one status per (caller, rule): violation | verified | other.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
case_name="$1"; shift
cd "$repo"
# Stale control-chain JSON from an earlier run would be read as this run's verdict.
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" \
    --rule-dsl dev/typerules/rules.json --studied-rules dev/typerules/studied_rules \
    --skip-llm-testcase --compose-loop-bound 1 "$@" >".local/verify-$case_name.log" 2>&1 \
    || { tail -30 ".local/verify-$case_name.log"; exit 1; }
python3 - "$case_name" <<'E'
import json, sys
name = sys.argv[1]
data = json.load(open(f".local/verify/{name}/full-matrix-results.json"))
for row in data["results"]:
    print(f"{name}: {row['caller']:<24} {row['rule']:<9} {row['status']}")
E
