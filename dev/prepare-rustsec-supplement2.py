#!/usr/bin/env python3
"""Archive a second DSL-selected RustSec coverage supplement."""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("prepare_rustsec", ROOT / "dev/prepare-rustsec.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

CASES = [
    ("orx-pinned-vec", "3.20.0", "RUSTSEC-2025-0106"),
    ("tanton_engine", "1.0.0", "RUSTSEC-2025-0031"),
    ("xmas-elf", "0.9.1", "RUSTSEC-2025-0018"),
    ("slab", "0.4.10", "RUSTSEC-2025-0047"),
]

cases = [module.prepare(case) for case in CASES]
manifest = {
    "cutoff_exclusive": "2026-03-01",
    "selection": (
        "Second supplement of known RustSec cases selected only after the frozen DSL "
        "bound the advisory operation; coverage experiment over published cases."
    ),
    "cases": cases,
}
(ROOT / "benchmark/rustsec/supplement2-manifest.json").write_text(
    json.dumps(manifest, indent=2) + "\n"
)
print(json.dumps({"cases": len(cases), "manifest": "benchmark/rustsec/supplement2-manifest.json"}))
