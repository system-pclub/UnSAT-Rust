#!/usr/bin/env bash
# Usage: dev/typerules/verify.sh <case> <callsite-id> <rule-id> [extra ./x verify args]
# Runs one callsite/rule pair of benchmark/typerules/<case> with the lowered DSL
# in dev/typerules/rules.json, stopping after KLEE compose verification.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
case_name="$1"; callsite="$2"; rule="$3"; shift 3
cd "$repo"
./x verify "benchmark/typerules/$case_name" --callsite "$callsite" --rule "$rule" \
    --rule-dsl dev/typerules/rules.json --studied-rules dev/typerules/studied_rules \
    --skip-llm-testcase --compose-loop-bound 1 "$@"
