"""Python-validate CStr DST observations; source step2 rules remain untouched."""
import csv
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))
import dslv2
from dsltrans import Model

API = "impl core::ffi::c_str::CStr { const unsafe fn from_ptr<'a>(ptr: *const i8) -> &'a core::ffi::c_str::CStr }"
EXPRESSIONS = {
    "dst-layout": "$3.$mem.size == $3.inner.$mem.size and $3.$mem.align == 1 and $3.inner.$mem.align == 1 and $3.$mem.end == $3.$mem.start + $3.inner.$mem.size",
    "dst-three-bytes": "$3.inner.$mem.size == 3",
}

def main():
    with (ROOT / "rules.csv").open(newline="") as handle:
        root = next(row for row in csv.DictReader(handle)
                    if "c_str.rs" in row["path"] and "from_ptr" in row["name"])
    rules = {}
    for name, expression in EXPRESSIONS.items():
        document = dslv2.lower(Model(ROOT / "tools/x/generated/dsl/schema.json", API), expression)
        rules[name] = {"path": root["path"], "name": root["name"],
                       "rule": f"Synthetic DST observation: {name}", "api": API,
                       "dsl": expression, "dsl_v2": document,
                       "synthetic_observation": True}
    here = Path(__file__).parent
    (here / "dst-layout-rules.json").write_text(json.dumps({"include_raw_pointer_deref": False, "rules": rules}, indent=2) + "\n")
    (here / "dst-layout.studied").write_text("\n".join(rules) + "\n")
    print(f"Validated {len(rules)} DST observations")

if __name__ == "__main__":
    main()
