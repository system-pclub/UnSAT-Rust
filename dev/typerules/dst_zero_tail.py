"""Validate existing DSL observations for aligned, zero-sized DST elements."""
import csv
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))
import dslv2
from dsltrans import Model

API = "unsafe fn write<T>(dst: *mut T, src: T)"
EXPRESSIONS = {
    "zst-valid": "$2.$mem.valid",
    "zst-initialized": "$2.$mem.initialized",
    "zst-size": "$2.$mem.size == 16",
    "zst-align": "$2.$mem.align == 16",
    "zst-wrong-size": "$2.$mem.size == 32",
}


def main():
    with (ROOT / "rules.csv").open(newline="") as handle:
        root = next(row for row in csv.DictReader(handle)
                    if row["name"] == "const unsafe fn write < T >")
    rules = {}
    for name, expression in EXPRESSIONS.items():
        lowered = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", API), expression)
        rules[name] = {"path": root["path"], "name": root["name"],
                       "rule": f"Synthetic zero-sized DST observation: {name}",
                       "api": API, "dsl": expression, "dsl_v2": lowered,
                       "synthetic_observation": True}
    here = Path(__file__).parent
    (here / "dst-zero-tail-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False, "rules": rules}, indent=2) + "\n")
    (here / "dst-zero-tail.studied").write_text("\n".join(rules) + "\n")
    print(f"Validated {len(rules)} zero-sized DST observations")


if __name__ == "__main__":
    main()
