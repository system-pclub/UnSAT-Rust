#!/usr/bin/env python3
"""Summarize the latest run-all-crates run: progress and failure classes.

Usage: dev/run-all-crates-status.py [run-dir]
"""
from __future__ import annotations

import collections
import gzip
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / ".local" / "run-all-crates"
run = Path(sys.argv[1]) if len(sys.argv) > 1 else Path((ROOT / "latest").read_text().strip())

print(run.name, "status:", (run / "status").read_text().strip(),
      "current-index:", (run / "current-index").read_text().strip())
rows = [line.split("\t") for line in (run / "progress.tsv").read_text().splitlines()[1:]]
print("jobs done:", len(rows), dict(collections.Counter(r[2] for r in rows)))

ERROR = re.compile(
    r"^(?:error(?:\[E\d+\])?:[^\n]{0,110}|Error:[^\n]{0,110}|"
    r"[^\n]*is already defined|[^\n]*Failed to link[^\n]{0,80})", re.M)
classes: collections.Counter = collections.Counter()
example: dict[str, int] = {}
for index, code, status in rows:
    if status == "finished":
        continue
    log = run / "jobs" / f"{int(index):06d}" / "verify.log.gz"
    try:
        text = gzip.open(log, "rt", errors="replace").read()
    except OSError:
        continue
    found = ERROR.findall(text)
    key = re.sub(r"[`'\"][^`'\"]*[`'\"]", "X", found[-1])[:100] if found else (
        "outer timeout" if code == "124" else f"exit {code} (no error line)")
    classes[key] += 1
    example.setdefault(key, int(index))
for key, count in classes.most_common(15):
    print(f"{count:4d}  {key}  (e.g. job {example[key]})")

pair_status: collections.Counter = collections.Counter()
for result in run.glob("jobs/*/*/*/result.json"):
    try:
        pair_status[json.loads(result.read_text())["result"]["init_status"]] += 1
    except (OSError, ValueError, KeyError):
        pass
print("pair statuses:", dict(pair_status.most_common()))
