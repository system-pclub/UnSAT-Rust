#!/usr/bin/env python3
"""Drop KLEE evidence of completed run-all-crates pairs that found no violation.

Usage: dev/run-all-crates-prune.py [--dry-run] [run-dir ...]
       (default: every .local/run-all-crates/run-* directory)

Only archived pairs are touched (the runner writes `.pruned` after it has
archived klee-out). A pair whose status is `violation` or `violation-timeout`
keeps everything. Any other pair loses its klee-out archive, logs and
counterexample; `result.json` stays. Error statuses additionally keep a small
`klee-excerpt.txt` so a failure can still be classified. A job without any
violation also loses its replay-inputs archive (the crate is still in
.local/crates). Every removal is appended to `pruned-nonviolation.tsv` in the
run directory.
"""
from __future__ import annotations

import gzip
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / ".local" / "run-all-crates"
KEEP = {"violation", "violation-timeout"}
ERROR_STATUSES = {"klee-error", "eval-error", "solver-error", "timeout",
                  "reached-timeout", "no-final-result", "unknown"}
PAIR_FILES = ("klee-out.tar.gz", "klee-out.sha256", "klee.log.gz",
              "klee-info.txt.gz", "klee.log", "klee-info.txt",
              "counterexample.ktest")
JOB_FILES = ("replay-inputs.tar.gz", "replay-inputs.sha256")
INTERESTING = re.compile(
    r"ERROR|Assertion|failed to evaluate|execution model|terminating state|"
    r"invalid function pointer|concretiz|unsupported|PLEASE submit", re.I)


def excerpt(pair: Path) -> str:
    log = pair / "klee.log.gz"
    if log.is_file():
        with gzip.open(log, "rt", errors="replace") as handle:
            lines = handle.read().splitlines()
    elif (pair / "klee.log").is_file():
        lines = (pair / "klee.log").read_text(errors="replace").splitlines()
    else:
        return ""
    picked, seen = [], set()
    for line in lines:
        key = re.sub(r"\d+", "N", line)[:160]
        if INTERESTING.search(line) and key not in seen:
            seen.add(key)
            picked.append(line[:400])
        if len(picked) >= 30:
            break
    return "\n".join(["# matching lines", *picked, "# last lines", *lines[-30:]]) + "\n"


def size(path: Path) -> int:
    return path.stat().st_size if path.is_file() else 0


def prune_run(run: Path, dry: bool) -> int:
    freed = 0
    ledger = []
    for job in sorted((run / "jobs").glob("*")):
        # The job still running has no final status yet.
        if not (job / "status").is_file():
            continue
        pairs = [p.parent for p in job.glob("*/*/result.json")]
        statuses = {}
        for pair in pairs:
            try:
                statuses[pair] = json.loads(
                    (pair / "result.json").read_text())["result"]["init_status"]
            except (OSError, ValueError, KeyError):
                statuses[pair] = None
        for pair, status in statuses.items():
            if status in KEEP or status is None or not (pair / ".pruned").exists():
                continue
            if status in ERROR_STATUSES and not (pair / "klee-excerpt.txt").exists():
                text = excerpt(pair)
                if text and not dry:
                    (pair / "klee-excerpt.txt").write_text(text)
            for name in PAIR_FILES:
                file = pair / name
                if file.is_file():
                    freed += size(file)
                    ledger.append(f"{file.relative_to(run)}\t{status}")
                    if not dry:
                        file.unlink()
        if all(s not in KEEP for s in statuses.values()):
            for name in JOB_FILES:
                file = job / name
                if file.is_file():
                    freed += size(file)
                    ledger.append(f"{file.relative_to(run)}\tjob-without-violation")
                    if not dry:
                        file.unlink()
    if ledger and not dry:
        with open(run / "pruned-nonviolation.tsv", "a") as handle:
            handle.write("\n".join(ledger) + "\n")
    return freed


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    dry = "--dry-run" in sys.argv
    runs = [Path(a) for a in args] or sorted(ROOT.glob("run-*"))
    total = 0
    for run in runs:
        freed = prune_run(run, dry)
        total += freed
        if freed:
            print(f"{run.name}: {'would free' if dry else 'freed'} {freed / 2**20:.1f} MiB")
    print(f"total {'would free' if dry else 'freed'}: {total / 2**30:.2f} GiB")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
