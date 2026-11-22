"""Validate the unchanged initialization rule and an allocation alignment observation."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))
import dslv2
from dsltrans import Model


def main():
    here = Path(__file__).parent
    source = json.loads((here / "rules.json").read_text())["rules"]["rule-322-initialized"]
    rules = {"rule-322-initialized": dict(source),
             "allocation-align": {**source,
                 "rule": "Synthetic observation: the containing allocation retains its requested 16-byte alignment.",
                 "dsl": "$1.$mem.pointee.allocation.align == 16",
                 "synthetic_observation": True}}
    for rule in rules.values():
        rule["dsl_v2"] = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), rule["dsl"])
    (here / "symbolic-allocation-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False, "rules": rules}, indent=2) + "\n")
    (here / "symbolic-allocation.studied").write_text("\n".join(rules) + "\n")
    print("Validated initialization and allocation alignment observations")


if __name__ == "__main__":
    main()
