#!/usr/bin/env python3
"""Archive a third RustSec supplement: std/core unsafe advisories where the initial
RuKLEE stage already reported SAT on the advisory's unsafe operation."""
import importlib.util
import json
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("prepare_rustsec", ROOT / "dev/prepare-rustsec.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

CASES = [
    ("simple-slab", "0.3.2", "RUSTSEC-2020-0039"),
    ("toodee", "0.2.4", "RUSTSEC-2021-0028"),
    ("ordnung", "0.0.1", "RUSTSEC-2020-0038"),
]

cases = [module.prepare(case) for case in CASES]
for row in cases:
    # Keep the dependency resolution that produced the reviewed result.
    scan = ROOT / ".local/rustsec-scan/work" / Path(row["directory"]).name / "Cargo.lock"
    if scan.exists():
        shutil.copy2(scan, ROOT / "benchmark/rustsec/evidence" / row["advisory"] / "Cargo.lock")
manifest = {
    "cutoff_exclusive": "2026-03-01",
    "selection": (
        "Third supplement of known RustSec cases whose advisory operation is a std/core "
        "unsafe call and whose initial RuKLEE stage reported SAT; coverage experiment, "
        "not a random sample."
    ),
    "cases": cases,
}
(ROOT / "benchmark/rustsec/supplement3-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({"cases": len(cases), "manifest": "benchmark/rustsec/supplement3-manifest.json"}))
