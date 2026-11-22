#!/usr/bin/env bash
# Usage: dev/callchain-multi-field/run.sh
# Verifies every copy_nonoverlapping callsite (rule-958) of
# benchmark/typerules/callchain-multi-field and diffs the verdict per caller
# with expected.txt. Build KLEE first (see dev/callchain-field-dependency.md).
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=callchain-multi-field
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --rule rule-958 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { tail -30 ".local/verify-$case_name.log"; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, pathlib, sys
root = pathlib.Path(".local/verify") / sys.argv[1]
rows = []
for result in sorted(root.glob("*/rule-958/result.json")):
    data = json.loads(result.read_text())
    caller = data["callsite"]["caller"]
    status = data["result"]["init_status"]
    rows.append((caller, status))
for caller, status in sorted(rows):
    print(f"{caller:<22} {status}")
E
# Only the violation / no-violation split is the contract; the raw init_status
# (e.g. low-confidence-sat when the control chain stays unresolved) is printed.
normalize() { sed -E 's/^(\S+)\s+(violation).*/\1 \2/; t; s/^(\S+)\s+.*/\1 no-violation/' "$1"; }
cat "$actual"
diff -u <(normalize dev/callchain-multi-field/expected.txt) <(normalize "$actual") \
    && echo "callchain-multi-field: all verdicts as expected"
