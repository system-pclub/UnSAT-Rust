"""Validate an existing DSL observation whose request metadata is unavailable."""
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
    expression = "$1.$mem.pointee.allocation.minimum_size <= $1.$mem.pointee.allocation.size"
    rule = {**source,
            "rule": "Synthetic observation: the allocation request minimum fitting-layout size does not exceed its returned size.",
            "dsl": expression, "synthetic_observation": True,
            "dsl_v2": dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), expression)}
    (here / "allocation-contract-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False, "rules": {"allocation-request-minimum": rule}}, indent=2) + "\n")
    (here / "allocation-contract.studied").write_text("allocation-request-minimum\n")
    print("Validated allocation-request observation; expected backend outcome: eval-error")


if __name__ == "__main__":
    main()
