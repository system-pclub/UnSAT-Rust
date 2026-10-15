#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
import posixpath
import re
import xml.etree.ElementTree as ET
import zipfile
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
MAIN_NS = "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}"
REL_NS = "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}"
CRATE_DIRS = {
    "hftbacktest": "hftbacktest-0.4.0",
    "jh": "jh-0.1.0",
    "revm": "revm-interpreter-10.0.3",
    "rust-crypto": "rust-crypto-0.2.36",
    "lurk-cli": "lurk-cli-0.3.8",
    "arrow2": "arrow2-0.18.0",
    "blade": "blade-0.2.0",
    "memmap": "memmap-0.7.0",
    "fast-float": "fast-float-0.2.0",
    "meshopt": "meshopt-0.3.0",
}
DIR_TO_CRATE = {value: key for key, value in CRATE_DIRS.items()}
PREDICTION_PATH_PREFIX = {
    # blade is evaluated through its nested library package, while the
    # workbook stores paths relative to the downloaded top-level crate.
    "blade": "blade-graphics",
}
DEFAULT_VARIANTS = (
    "full",
    "without-rule-extraction",
    "without-rule-translation",
    "without-mut-ref-recovery",
    "without-symbolic-execution",
    "without-validation",
)


def _cell_value(cell: ET.Element, shared: list[str]) -> str:
    if cell.attrib.get("t") == "inlineStr":
        return "".join(node.text or "" for node in cell.iter(MAIN_NS + "t"))
    value = cell.find(MAIN_NS + "v")
    raw = "" if value is None else value.text or ""
    if cell.attrib.get("t") == "s" and raw:
        return shared[int(raw)]
    return raw


def _sheet_rows(workbook_path: Path, sheet_name: str) -> list[dict[str, str]]:
    with zipfile.ZipFile(workbook_path) as archive:
        shared: list[str] = []
        if "xl/sharedStrings.xml" in archive.namelist():
            shared = [
                "".join(node.text or "" for node in item.iter(MAIN_NS + "t"))
                for item in ET.fromstring(archive.read("xl/sharedStrings.xml"))
            ]
        workbook = ET.fromstring(archive.read("xl/workbook.xml"))
        relationships = ET.fromstring(archive.read("xl/_rels/workbook.xml.rels"))
        targets = {item.attrib["Id"]: item.attrib["Target"] for item in relationships}
        sheets = workbook.find(MAIN_NS + "sheets")
        if sheets is None:
            raise RuntimeError(f"workbook has no sheets: {workbook_path}")
        sheet = next(
            item
            for item in sheets
            if item.attrib.get("name") == sheet_name
        )
        target = targets[sheet.attrib[REL_NS + "id"]]
        target = target.lstrip("/") if target.startswith("/") else posixpath.normpath("xl/" + target)
        root = ET.fromstring(archive.read(target))
        rows: list[dict[str, str]] = []
        for row in root.iter(MAIN_NS + "row"):
            values: dict[str, str] = {}
            for cell in row.findall(MAIN_NS + "c"):
                ref = cell.attrib.get("r", "")
                column = "".join(character for character in ref if character.isalpha())
                values[column] = _cell_value(cell, shared)
            rows.append(values)
        return rows


def _parse_location(value: str) -> tuple[str, set[int]]:
    path, separator, raw_lines = value.rpartition(":")
    if not separator:
        raise RuntimeError(f"ground-truth location has no line suffix: {value}")
    lines: set[int] = set()
    for part in raw_lines.split(","):
        part = part.strip()
        if "-" in part:
            start, end = (int(number) for number in part.split("-", 1))
            lines.update(range(start, end + 1))
        else:
            lines.add(int(part))
    return path.replace("\\", "/"), lines


def load_ground_truth(workbook_path: Path) -> list[dict[str, Any]]:
    rows = _sheet_rows(workbook_path, "top10 github bugs")
    ground_truth: list[dict[str, Any]] = []
    for index, row in enumerate(rows[1:], start=2):
        crate, bug_id = row.get("A", "").strip(), row.get("B", "").strip()
        if not crate or not bug_id:
            continue
        if crate not in CRATE_DIRS:
            raise RuntimeError(f"unexpected crate in top10 sheet row {index}: {crate}")
        path, lines = _parse_location(bug_id)
        ground_truth.append(
            {"row": index, "crate": crate, "bug_id": bug_id, "path": path, "lines": lines}
        )
    unique_crates = {item["crate"] for item in ground_truth}
    if len(unique_crates) != 10:
        raise RuntimeError(f"expected ten crates in top10 sheet, found {len(unique_crates)}")
    return ground_truth


def _load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    return value if isinstance(value, dict) else {}


def _is_positive(variant: str, row: dict[str, Any]) -> bool:
    status = row.get("status")
    if variant == "without-validation":
        return status in {
            "violation",
            "violation-timeout",
            "low-confidence-sat",
        }
    if variant == "without-symbolic-execution":
        return row.get("prediction") == "violation" or status == "violation"
    return status in {"violation", "violation-timeout", "low-confidence-sat"} and bool(
        row.get("full_rerun_passed")
    )


def _crate_from_result(path: Path, document: dict[str, Any]) -> str | None:
    raw_crate = document.get("crate")
    dirname = Path(raw_crate).name if isinstance(raw_crate, str) else ""
    if dirname in DIR_TO_CRATE:
        return DIR_TO_CRATE[dirname]
    for parent in path.parents:
        if parent.name in DIR_TO_CRATE:
            return DIR_TO_CRATE[parent.name]
    return None


def result_crates(result_root: Path) -> set[str]:
    crates: set[str] = set()
    for path in sorted(result_root.rglob("full-matrix-results.json")):
        crate = _crate_from_result(path, _load_json(path))
        if crate is not None:
            crates.add(crate)
    return crates


def predicted_callsites(
    variant: str,
    result_root: Path,
    include_crates: set[str] | None = None,
) -> tuple[set[tuple[str, str, int]], int]:
    predictions: set[tuple[str, str, int]] = set()
    errors = 0
    for path in sorted(result_root.rglob("full-matrix-results.json")):
        document = _load_json(path)
        crate = _crate_from_result(path, document)
        if crate is None:
            continue
        if include_crates is not None and crate not in include_crates:
            continue
        for row in document.get("results", []):
            if not isinstance(row, dict):
                continue
            if row.get("status") in {"error", "timeout", "unknown"}:
                errors += 1
            if not _is_positive(variant, row):
                continue
            source_path, line = row.get("path"), row.get("line")
            if isinstance(source_path, str) and isinstance(line, int):
                source_path = source_path.replace("\\", "/")
                prefix = PREDICTION_PATH_PREFIX.get(crate)
                if prefix and not source_path.startswith(prefix + "/"):
                    source_path = f"{prefix}/{source_path}"
                predictions.add((crate, source_path, line))
    return predictions, errors


def score(
    variant: str,
    result_root: Path,
    truth: list[dict[str, Any]],
    include_crates: set[str] | None = None,
) -> dict[str, Any]:
    predictions, errors = predicted_callsites(variant, result_root, include_crates)
    matched_rows: set[int] = set()
    false_positives: list[dict[str, Any]] = []
    for crate, path, line in sorted(predictions):
        matches = [
            item for item in truth
            if item["crate"] == crate and item["path"] == path and line in item["lines"]
        ]
        if matches:
            matched_rows.update(item["row"] for item in matches)
        else:
            false_positives.append({"crate": crate, "path": path, "line": line})
    missed = [
        {key: item[key] for key in ("row", "crate", "bug_id")}
        for item in truth
        if item["row"] not in matched_rows
    ]
    return {
        "label": variant,
        "result_root": str(result_root),
        "TP": len(matched_rows),
        "FP": len(false_positives),
        "FN": len(missed),
        "ground_truth_total": len(truth),
        "predicted_callsites": len(predictions),
        "error_rows": errors,
        "false_positives": false_positives,
        "missed_ground_truth": missed,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Score RuKLEE ablations against raudit.xlsx/top10 github bugs.")
    parser.add_argument("--workbook", type=Path, default=ROOT / "raudit.xlsx")
    parser.add_argument("--results-root", type=Path, default=ROOT / "ablation" / "results")
    parser.add_argument(
        "--variant",
        action="append",
        metavar="NAME=PATH",
        help="Override/add a result directory; repeatable.",
    )
    parser.add_argument("--output", type=Path, default=ROOT / "ablation" / "results" / "summary.json")
    parser.add_argument(
        "--allow-partial",
        action="store_true",
        help=(
            "Score only crates shared by every available variant. Intended for "
            "smoke tests; final top-10 results require complete coverage."
        ),
    )
    args = parser.parse_args()
    variants = {name: args.results_root / name for name in DEFAULT_VARIANTS}
    for value in args.variant or []:
        name, separator, path = value.partition("=")
        if not separator:
            parser.error("--variant must use NAME=PATH")
        variants[name] = Path(path)
    truth = load_ground_truth(args.workbook)
    available = {
        name: path
        for name, path in variants.items()
        if path.is_dir() and any(path.rglob("full-matrix-results.json"))
    }
    if not available:
        raise RuntimeError("no ablation result matrices found")
    coverage = {name: result_crates(path) for name, path in available.items()}
    expected_crates = set(CRATE_DIRS)
    missing_variants = sorted(set(DEFAULT_VARIANTS) - set(available))
    missing_by_variant = {
        name: sorted(expected_crates - coverage.get(name, set()))
        for name in DEFAULT_VARIANTS
        if expected_crates - coverage.get(name, set())
    }
    complete = not missing_variants and not missing_by_variant
    if not complete and not args.allow_partial:
        details = []
        if missing_variants:
            details.append("missing variants: " + ", ".join(missing_variants))
        details.extend(
            f"{name} missing crates: {', '.join(crates)}"
            for name, crates in missing_by_variant.items()
        )
        raise RuntimeError(
            "top-10 ablation results are incomplete ("
            + "; ".join(details)
            + "). Finish the runs or pass --allow-partial for a smoke-test summary."
        )
    included_crates = (
        expected_crates
        if complete
        else set.intersection(*(coverage[name] for name in available))
    )
    if not included_crates:
        raise RuntimeError("available variants have no commonly covered crates")
    scoped_truth = [item for item in truth if item["crate"] in included_crates]
    results = [
        score(name, path, scoped_truth, included_crates)
        for name, path in available.items()
    ]
    payload = {
        "workbook": str(args.workbook),
        "sheet": "top10 github bugs",
        "top10_crates": list(CRATE_DIRS),
        "ground_truth_total": len(scoped_truth),
        "top10_ground_truth_total": len(truth),
        "coverage": {
            "status": "complete" if complete else "partial-smoke",
            "included_crates": sorted(included_crates),
            "missing_variants": missing_variants,
            "missing_by_variant": missing_by_variant,
        },
        "matching": {
            "mode": "crate_path_line",
            "status": "preliminary",
            "warning": (
                "A location match cannot distinguish multiple obligations at the same "
                "callsite. Paper-grade TP/FP/FN requires rule-level adjudication."
            ),
        },
        "variants": results,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    for item in results:
        print(f"{item['label']}: TP={item['TP']} FP={item['FP']} FN={item['FN']} errors={item['error_rows']}")
    if not complete:
        print(
            "warning: partial smoke-test summary; common crates: "
            + ", ".join(sorted(included_crates))
        )
    print("warning: location-level scores are preliminary; adjudicate same-callsite obligations manually")
    print(f"wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
