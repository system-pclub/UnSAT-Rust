from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Iterable


ResultKey = tuple[str, str]


def _load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"could not read verify result {path}: {error}") from error
    if not isinstance(value, dict):
        raise RuntimeError(f"verify result is not a JSON object: {path}")
    return value


def _crate_name(value: object, fallback: str) -> str:
    if isinstance(value, str) and value.strip():
        name = Path(value.rstrip("/\\")).name
        if name:
            return name
    return fallback


def _merge(results: dict[ResultKey, bool], key: ResultKey, is_bug: bool) -> None:
    # A callsite is a bug when any matched rule has a confirmed full rerun.
    results[key] = results.get(key, False) or is_bug


def _read_matrix(path: Path, results: dict[ResultKey, bool]) -> None:
    data = _load_json(path)
    crate = _crate_name(data.get("crate"), path.parent.name)
    rows = data.get("results")
    if not isinstance(rows, list):
        return

    for row in rows:
        if not isinstance(row, dict):
            continue
        callsite = row.get("callsite")
        if not isinstance(callsite, str) or not callsite:
            continue
        is_bug = row.get("status") == "violation" and bool(
            row.get("full_rerun_passed")
        )
        _merge(results, (crate, callsite), is_bug)


def _read_pair(path: Path, results: dict[ResultKey, bool]) -> None:
    data = _load_json(path)
    callsite_data = data.get("callsite")
    result_data = data.get("result")
    if not isinstance(callsite_data, dict) or not isinstance(result_data, dict):
        return

    callsite = callsite_data.get("id")
    if not isinstance(callsite, str) or not callsite:
        return
    fallback = path.parents[2].name if len(path.parents) > 2 else path.parent.name
    crate = _crate_name(data.get("crate"), fallback or "unknown")
    is_bug = result_data.get("init_status") == "violation" and bool(
        result_data.get("full_rerun_passed")
    )
    _merge(results, (crate, callsite), is_bug)


def _scan_tree(root: Path, results: dict[ResultKey, bool]) -> bool:
    if not root.exists():
        raise RuntimeError(f"result path does not exist: {root}")

    if root.is_file():
        if root.name.endswith("full-matrix-results.json"):
            _read_matrix(root, results)
            return True
        _read_pair(root, results)
        return True

    matrix_paths = sorted(root.rglob("*full-matrix-results.json"))
    if matrix_paths:
        for path in matrix_paths:
            _read_matrix(path, results)
        return True

    pair_paths = sorted(root.rglob("result.json"))
    pair_paths.extend(sorted(root.rglob("*__*.json")))
    for path in pair_paths:
        _read_pair(path, results)
    return bool(pair_paths)


def _roots(args: argparse.Namespace) -> Iterable[Path]:
    if args.result_dir:
        return [Path(args.result_dir)]

    parent = Path(args.result_dirdir or ".local/verify")
    if not parent.is_dir():
        raise RuntimeError(f"result directory does not exist: {parent}")
    return sorted(path for path in parent.iterdir() if path.is_dir())


def run(args: argparse.Namespace) -> int:
    results: dict[ResultKey, bool] = {}
    found_artifact = False
    for root in _roots(args):
        found_artifact = _scan_tree(root, results) or found_artifact

    if not found_artifact:
        raise RuntimeError("no x verify result files found")
    if not results:
        raise RuntimeError("x verify result files contain no callsite results")

    for (crate, callsite), is_bug in sorted(results.items()):
        print(f"{crate} {callsite} {'BUG' if is_bug else 'OK'}")
    return 0
