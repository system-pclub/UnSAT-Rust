#!/usr/bin/env python3
"""Archive a fourth RustSec supplement: advisories checked through the ./x wrapper."""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("prepare_rustsec", ROOT / "dev/prepare-rustsec.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.CUTOFF = "2027-01-01"

# Pinned to an affected version; the newest affected release unless an older
# one is needed so that the advisory's operation is still present.
CASES = [
    ("bytes", "1.11.0", "RUSTSEC-2026-0007"),
    ("memmap2", "0.9.10", "RUSTSEC-2026-0186"),
    ("redox_uefi_std", "0.1.13", "RUSTSEC-2025-0032"),
    ("stack", "0.3.0", "RUSTSEC-2020-0042"),
    ("rtvm-interpreter", "4.0.0", "RUSTSEC-2025-0131"),
    ("auto_vec", "0.2.0", "RUSTSEC-2026-0133"),
    ("reorder", "1.0.3", "RUSTSEC-2021-0050"),
    ("rdiff", "0.1.2", "RUSTSEC-2021-0094"),
    ("tremor-script", "0.11.5", "RUSTSEC-2021-0111"),
    ("lexical-core", "0.8.5", "RUSTSEC-2023-0086"),
    ("pprof", "0.13.0", "RUSTSEC-2024-0408"),
    ("grcov", "0.8.20", "RUSTSEC-2025-0005"),
    ("ozone", "0.1.0", "RUSTSEC-2020-0022"),
    ("smallvec", "0.6.9", "RUSTSEC-2019-0012"),
    ("toodee", "0.2.4", "RUSTSEC-2025-0062"),
]

cases = [module.prepare(case) for case in CASES]
manifest = {
    "cutoff_exclusive": module.CUTOFF,
    "selection": "Fourth supplement: advisories verified end to end with ./x verify; not a random sample.",
    "cases": cases,
}
(ROOT / "benchmark/rustsec/supplement4-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
for row in cases:
    print(row["advisory"], row["crate"], row["version"], row["issued"])
