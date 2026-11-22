"""Validate synthetic trailing-DST observations with the existing DSL checker."""
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
    "dst-valid": "$2.$mem.valid",
    "dst-initialized": "$2.$mem.initialized",
    "dst-size": "$2.$mem.size == 12 or $2.$mem.size == 8 or $2.$mem.size == 20",
    "dst-align": "$2.$mem.align == 4",
}


def main():
    with (ROOT / "rules.csv").open(newline="") as handle:
        root = next(row for row in csv.DictReader(handle)
                    if row["name"] == "const unsafe fn write < T >")
    rules = {}
    for name, expression in EXPRESSIONS.items():
        document = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", API), expression)
        rules[name] = {"path": root["path"], "name": root["name"],
                       "rule": f"Synthetic trailing-DST observation: {name}", "api": API,
                       "dsl": expression, "dsl_v2": document,
                       "synthetic_observation": True}
    here = Path(__file__).parent
    (here / "dst-validity-rules.json").write_text(json.dumps({"include_raw_pointer_deref": False, "rules": rules}, indent=2) + "\n")
    (here / "dst-validity.studied").write_text("\n".join(rules) + "\n")
    print(f"Validated {len(rules)} trailing-DST observations")


if __name__ == "__main__":
    main()
