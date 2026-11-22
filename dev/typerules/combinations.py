"""Validate existing-syntax DSL combinations and emit observation test metadata.

These are synthetic evaluator contracts, independent of the standard API rules.
Run inside the repository image before ./x verify with combination-rules.json.
"""
import json
import csv
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))
import dslv2
from dsltrans import Model

API = "unsafe fn write(dst: *mut ([u8; 4], usize, i64, char), src: ([u8; 4], usize, i64, char))"
RULES = {
    "obs-list": "$2[0][$2[1]] == (11 if $2[1] == 0 else 22 if $2[1] == 1 else 33 if $2[1] == 2 else 44)",
    "obs-floor": "$2[2] % 3 >= 0 and $2[2] % 3 < 3 and ($2[2] // 3) * 3 + $2[2] % 3 == $2[2] and $2[2] % -3 <= 0 and $2[2] % -3 > -3 and ($2[2] // -3) * -3 + $2[2] % -3 == $2[2]",
    "obs-floor-boundary": "-7 // 3 == -3 and -7 % 3 == 2 and 7 // -3 == -3 and 7 % -3 == -2 and $2[2] // -1 == -$2[2] and $2[2] % -1 == 0",
    "obs-bigint": f"({2**240} + $2[2]) - {2**240} == $2[2] and ($2[2] * $2[2] * $2[2]) // 1 == $2[2] * $2[2] * $2[2]",
    "obs-filter": "all([b < 128 and b.$mem.initialized for b in $2[0] if b < 128]) and len([b for b in $2[0] if b < 128]) <= len($2[0]) and (any([b < 128 for b in $2[0]]) == (len([b for b in $2[0] if b < 128]) > 0))",
    "obs-view": "($2[0][$2[1]].$mem.size if $2[1] == 0 else $2[0][3].$mem.size) == 1 and ($2[0][$2[1]].$mem.initialized if $2[1] == 0 else $2[0][3].$mem.initialized)",
    "obs-unicode": "($2[3] if $2[1] == 0 else \"β\") == \"β\" and len(\"β\") == 2 and len($2[3]) >= 1 and len($2[3]) <= 4",
    "obs-membership": "$2[0][$2[1]] in $2[0] and $2[0][$2[1]] in [b for b in $2[0]]",
    "obs-div-guard": "($2[2] // $2[2] == 1) if $2[2] != 0 else True",
    "obs-chain": "0 <= $2[2] < 4 > $2[0][$2[2]]",
}


def main():
    with (ROOT / "rules.csv").open(newline="") as handle:
        root = next(row for row in csv.DictReader(handle)
                    if row["name"] == "const unsafe fn write < T >")
    rules = {}
    for name, expression in RULES.items():
        document = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", API), expression)
        rules[name] = {"path": root["path"], "name": root["name"],
                       "rule": f"Synthetic evaluator observation: {name}", "api": API,
                       "dsl": expression, "dsl_v2": document, "synthetic_observation": True}
    here = Path(__file__).parent
    (here / "combination-rules.json").write_text(json.dumps({"rules": rules}, indent=2) + "\n")
    (here / "combinations.studied").write_text("\n".join(rules) + "\n")
    print(f"Validated {len(rules)} existing-syntax DSL combinations")


if __name__ == "__main__":
    main()
