"""Validate the existing synthetic bit-reversal initialization observation."""
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
    for rule_id in ("rule-322-initialized", "rule-504"):
        source = sources[rule_id]
        lowered = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", source["api"]), source["dsl"])
        rules[rule_id] = {**source, "dsl_v2": lowered}
    (here / "bit-reverse-initialization-rules.json").write_text(json.dumps(
        {"include_raw_pointer_deref": False,
         "rules": rules}, indent=2) + "\n")
    (here / "bit-reverse-initialization.studied").write_text("\n".join(rules) + "\n")
    for rule_id, rule in rules.items():
        print("Validated", rule_id, rule["dsl"])


if __name__ == "__main__":
    main()
