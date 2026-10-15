#!/usr/bin/env python3
"""Publish explicitly reviewed cells, preserving every unrelated CSV byte."""
import csv
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ORDER = ("semgrep", "lockbud", "prusti", "rudra", "mirai", "mirchecker")


def main():
    summary = json.loads((ROOT / "eval/rust-comparison/summary.json").read_text())
    cells = summary["cells"]
    names = {"fast-float-0.2.0": "fast-float", "falcon_capstone-0.5.2": "falcon_capstone"}
    assert len(cells) == len(names) * len(ORDER)
    expected = {(crate, tool) for crate in names for tool in ORDER}
    assert {(c["crate"], c["tool"]) for c in cells} == expected
    by_crate = {crate: {} for crate in names}
    for cell in cells:
        assert cell["status"] in ("completed", "unsupported", "oom", "timeout")
        assert cell["reason"] and cell["runs"]
        runs = [json.loads((ROOT / "eval/rust-comparison" / path).read_text()) for path in cell["runs"]]
        assert all(r["tool"] == cell["tool"] and r["crate"] == cell["crate"] for r in runs)
        if cell["status"] == "completed":
            assert all(r["status"] == "completed" for r in runs)
            assert isinstance(cell["tp"], int) and isinstance(cell["fp"], int)
            assert cell["tp"] >= 0 and cell["fp"] >= 0
            values = [str(cell["tp"]), str(cell["fp"])]
        else:
            assert cell["tp"] is None and cell["fp"] is None
            values = ["N/A", "N/A"]
        by_crate[cell["crate"]][cell["tool"]] = values
    replacements = {names[crate]: [v for tool in ORDER for v in by_crate[crate][tool]] for crate in names}
    path = ROOT / "eval/RustComparison.csv"
    original = path.read_bytes()
    lines = original.splitlines(keepends=True)
    changed = []
    for i, line in enumerate(lines):
        row = next(csv.reader([line.decode()]))
        if row and row[0] in replacements:
            assert len(row) >= 21
            before = row[:]
            row[9:21] = replacements[row[0]]
            assert row[:9] == before[:9] and row[21:] == before[21:]
            ending = "\r\n" if line.endswith(b"\r\n") else "\n" if line.endswith(b"\n") else ""
            stream = io.StringIO(newline="")
            csv.writer(stream, lineterminator=ending).writerow(row)
            lines[i] = stream.getvalue().encode()
            changed.append(row[0])
    assert set(changed) == set(replacements) and len(changed) == 2
    result = b"".join(lines)
    assert len(lines) == len(original.splitlines(keepends=True))
    for old, new in zip(original.splitlines(keepends=True), result.splitlines(keepends=True)):
        if old != new:
            assert next(csv.reader([old.decode()]))[0] in replacements
    path.write_bytes(result)
    print("Updated baseline columns only: " + ", ".join(changed))


if __name__ == "__main__":
    main()
