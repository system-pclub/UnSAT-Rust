#!/usr/bin/env bash
# Usage: dev/toolchain-file-crate/run.sh
# benchmark/typerules/toolchain-file-crate pins `channel = "stable"` in
# rust-toolchain.toml. The analysis must still use the repository toolchain: the
# MIRScan report is written (it used to be missing when cargo picked a newer
# target layout) and the `-Z` test build is accepted. Rebuild mirscan first.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo"
case_name=toolchain-file-crate
rm -rf ".local/verify/$case_name"
./x verify "benchmark/typerules/$case_name" --rule rule-512 --skip-llm-testcase \
    --ir-output-dir ".local/$case_name-ir" >".local/verify-$case_name.log" 2>&1 \
    || { tail -12 ".local/verify-$case_name.log"; exit 1; }
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
python3 - "$case_name" >"$actual" <<'E'
import json, sys
data = json.load(open(f".local/verify/{sys.argv[1]}/full-matrix-results.json"))
for row in sorted(data["results"], key=lambda r: r["callsite"]):
    print(f"{row['callsite']:<20} {row['caller']}")
E
cat "$actual"
diff -u dev/toolchain-file-crate/expected.txt "$actual" \
    && echo "toolchain-file-crate: analysed with the repository toolchain"
