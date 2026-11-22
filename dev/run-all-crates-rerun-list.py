#!/usr/bin/env python3
"""List crates whose job failed from a toolchain problem that has since been fixed.

Usage: dev/run-all-crates-rerun-list.py > .local/rerun-crates.txt

Scans verify.log.gz of every job in every run for the signatures below and
prints the crate directory names whose latest attempt failed that way.
"""
from __future__ import annotations

import gzip
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / ".local" / "run-all-crates"
SIGNATURES = {
    "missing MIRScan report (new cargo layout)":
        re.compile(r"No such file or directory: '[^']*/report\.json'"),
    "stable cargo rejects -Z flags":
        re.compile(r"the `-Z` flag is only accepted on the nightly channel"),
    "derive-generated callsite aborted injection":
        re.compile(r"no call expression found at \d+:\d+ in "),
    "duplicate exported marker symbol":
        re.compile(r"symbol `__klee_target_[^`]*` is already defined"),
    "bindings hoisted out of an unsafe block":
        re.compile(r"error\[E0425\]: cannot find value `[a-z_]+` in this scope(?s:.{0,200})klee_ext_bind"),
    "unsized raw-pointer wrapper":
        re.compile(r"the size for values of type `(str|\[[^`]*\])` cannot be known(?s:.{0,300})raw_pointer_deref"),
    "injected parentheses under deny(warnings)":
        re.compile(r"error: unnecessary parentheses around function argument"),
    "NonNull argument binding":
        re.compile(r"the trait bound `&NonNull<[^>]*>: ArgPtrValue` is not satisfied"),
}


def main() -> int:
    latest: dict[str, tuple[str, str]] = {}
    for run in sorted(ROOT.glob("run-*")):
        for log in sorted((run / "jobs").glob("*/verify.log.gz")):
            source = log.parent / "source.path"
            if not source.is_file():
                continue
            crate = Path(source.read_text().strip()).name
            try:
                text = gzip.open(log, "rt", errors="replace").read()
            except OSError:
                continue
            reason = next((name for name, pattern in SIGNATURES.items()
                           if pattern.search(text)), "")
            latest[crate] = (reason, str(log.parent))
    count = 0
    for crate, (reason, job) in sorted(latest.items()):
        if reason:
            print(f"{crate}\t{reason}")
            count += 1
    print(f"# {count} crates", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
