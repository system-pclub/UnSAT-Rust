"""Validate the existing byte-initialization DSL for SIMD lane regressions."""
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
    expression = source["dsl"]
    lowered = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), expression)
    rule = {**source, "dsl_v2": lowered}
    (here / "vector-initialization-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False, "rules": {"rule-322-initialized": rule}}, indent=2) + "\n")
    (here / "vector-initialization.studied").write_text("rule-322-initialized\n")
    print(f"Validated {expression}")


if __name__ == "__main__":
    main()
