#!/usr/bin/env python3
"""Prepare a DSL-selected supplement of already published RustSec cases."""
import concurrent.futures
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("prepare_rustsec", ROOT / "dev/prepare-rustsec.py")
prepare = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prepare)

CASES = [
    ("fast-able", "1.11.17", "RUSTSEC-2025-0063"),
    ("shaman", "0.1.0", "RUSTSEC-2025-0113"),
    ("scanner", "0.1.0", "RUSTSEC-2025-0033"),
    ("anon-vec", "0.1.1", "RUSTSEC-2025-0039"),
    ("fast_id_map", "0.1.0", "RUSTSEC-2025-0034"),
]

if __name__ == "__main__":
    with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:
        rows = list(pool.map(prepare.prepare, CASES))
    data = {"cutoff_exclusive": "2026-03-01", "selection": "Supplement of known RustSec cases selected for unsafe APIs already covered by the frozen DSL; coverage experiment, not new vulnerability discovery. The original ten-case set remains separate.", "cases": rows}
    (ROOT / "benchmark/rustsec/supplement-manifest.json").write_text(json.dumps(data, indent=2) + "\n")
    for row in rows:
        print(row["advisory"], row["crate"], row["version"], row["issued"])
