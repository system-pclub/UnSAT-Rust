"""Validate existing DSL observations for SIMD arithmetic values and initialization."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))
import dslv2
from dsltrans import Model


def main():
    here = Path(__file__).parent
    sources = json.loads((here / "rules.json").read_text())["rules"]
    rules = {}
    for ruleid in ("rule-322-initialized", "rule-504"):
        source = sources[ruleid]
        expression = source["dsl"]
        rules[ruleid] = {**source, "dsl_v2": dslv2.lower(
            Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), expression)}
        print(f"Validated {ruleid}: {expression}")
    (here / "vector-arithmetic-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False, "rules": rules}, indent=2) + "\n")
    (here / "vector-arithmetic.studied").write_text("\n".join(rules) + "\n")


if __name__ == "__main__":
    main()
