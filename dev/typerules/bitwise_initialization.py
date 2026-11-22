"""Validate the existing synthetic byte-initialization observation."""
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
    lowered = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), source["dsl"])
    (here / "bitwise-initialization-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False,
         "rules": {"rule-322-initialized": {**source, "dsl_v2": lowered}}}, indent=2) + "\n")
    (here / "bitwise-initialization.studied").write_text("rule-322-initialized\n")
    print("Validated", source["dsl"])


if __name__ == "__main__":
    main()
