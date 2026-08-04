from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Iterable, Literal


Granularity = Literal["rule", "caller"]
ResultKey = tuple[str, ...]
Results = dict[ResultKey, bool]
# crate -> unsafe callee -> callsite -> whether any rule confirmed a bug
UnsafeApis = dict[str, dict[str, dict[str, bool]]]


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


def _merge(results: Results, key: ResultKey, is_bug: bool) -> None:
    # A group is a bug when any result in it has a confirmed full rerun.
    results[key] = results.get(key, False) or is_bug


def _merge_unsafe_api(
    unsafe_apis: UnsafeApis,
    crate: str,
    unsafe_callee: str | None,
    callsite: str,
    is_bug: bool,
) -> None:
    if not unsafe_callee:
        return
    crate_apis = unsafe_apis.setdefault(crate, {})
    api_callsites = crate_apis.setdefault(unsafe_callee, {})
    # An unsafe API callsite reports a bug if any rule has a confirmed bug.
    api_callsites[callsite] = api_callsites.get(callsite, False) or is_bug


def _matrix_caller(
    data: dict[str, Any], row: dict[str, Any], callsite: str
) -> str | None:
    caller = row.get("caller")
    if isinstance(caller, str) and caller:
        return caller

    callsites = data.get("callsites")
    if not isinstance(callsites, list):
        return None
    for item in callsites:
        if not isinstance(item, dict):
            continue
        item_id = item.get("callsite_id", item.get("id"))
        item_caller = item.get("caller")
        if (
            item_id == callsite
            and isinstance(item_caller, str)
            and item_caller
        ):
            return item_caller
    return None


def _matrix_unsafe_callee(
    data: dict[str, Any], row: dict[str, Any], callsite: str
) -> str | None:
    unsafe_callee = row.get("unsafe_callee")
    if isinstance(unsafe_callee, str) and unsafe_callee:
        return unsafe_callee

    callsites = data.get("callsites")
    if not isinstance(callsites, list):
        return None
    for item in callsites:
        if not isinstance(item, dict):
            continue
        if item.get("callsite_id", item.get("id")) != callsite:
            continue
        unsafe_callee = item.get("unsafe_callee")
        if isinstance(unsafe_callee, str) and unsafe_callee:
            return unsafe_callee
    return None


def _key(
    *,
    crate: str,
    callsite: str,
    rule: str,
    caller: str | None,
    granularity: Granularity,
    path: Path,
) -> ResultKey:
    if granularity == "rule":
        return (crate, callsite, rule)
    if caller is None:
        raise RuntimeError(
            f"verify result has no caller for callsite {callsite}: {path}"
        )
    return (crate, caller)


def _read_matrix(
    path: Path,
    results: Results,
    unsafe_apis: UnsafeApis,
    granularity: Granularity,
) -> None:
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
        rule = row.get("rule")
        if not isinstance(rule, str) or not rule:
            continue
        is_bug = row.get("status") == "violation" and bool(
            row.get("full_rerun_passed")
        )
        _merge_unsafe_api(
            unsafe_apis,
            crate,
            _matrix_unsafe_callee(data, row, callsite),
            callsite,
            is_bug,
        )
        _merge(
            results,
            _key(
                crate=crate,
                callsite=callsite,
                rule=rule,
                caller=_matrix_caller(data, row, callsite),
                granularity=granularity,
                path=path,
            ),
            is_bug,
        )


def _read_pair(
    path: Path,
    results: Results,
    unsafe_apis: UnsafeApis,
    granularity: Granularity,
) -> None:
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
    unsafe_callee = callsite_data.get("unsafe_callee")
    _merge_unsafe_api(
        unsafe_apis,
        crate,
        unsafe_callee if isinstance(unsafe_callee, str) else None,
        callsite,
        is_bug,
    )
    rule_data = data.get("rule")
    rule = rule_data.get("id") if isinstance(rule_data, dict) else None
    if not isinstance(rule, str) or not rule:
        return
    caller = callsite_data.get("caller")
    _merge(
        results,
        _key(
            crate=crate,
            callsite=callsite,
            rule=rule,
            caller=caller if isinstance(caller, str) and caller else None,
            granularity=granularity,
            path=path,
        ),
        is_bug,
    )


def _scan_tree(
    root: Path,
    results: Results,
    unsafe_apis: UnsafeApis,
    granularity: Granularity,
) -> bool:
    if not root.exists():
        raise RuntimeError(f"result path does not exist: {root}")

    if root.is_file():
        if root.name.endswith("full-matrix-results.json"):
            _read_matrix(root, results, unsafe_apis, granularity)
            return True
        _read_pair(root, results, unsafe_apis, granularity)
        return True

    matrix_paths = sorted(root.rglob("*full-matrix-results.json"))
    if matrix_paths:
        for path in matrix_paths:
            _read_matrix(path, results, unsafe_apis, granularity)
        return True

    pair_paths = sorted(root.rglob("result.json"))
    pair_paths.extend(sorted(root.rglob("*__*.json")))
    for path in pair_paths:
        _read_pair(path, results, unsafe_apis, granularity)
    return bool(pair_paths)


def _roots(args: argparse.Namespace) -> Iterable[Path]:
    if args.result_dir:
        return [Path(args.result_dir)]

    parent = Path(args.result_dirdir or ".local/verify")
    if not parent.is_dir():
        raise RuntimeError(f"result directory does not exist: {parent}")
    return sorted(path for path in parent.iterdir() if path.is_dir())


def run(args: argparse.Namespace) -> int:
    granularity: Granularity = getattr(args, "granularity", "rule")
    results: Results = {}
    unsafe_apis: UnsafeApis = {}
    found_artifact = False
    for root in _roots(args):
        found_artifact = (
            _scan_tree(root, results, unsafe_apis, granularity) or found_artifact
        )

    if not found_artifact:
        raise RuntimeError("no x verify result files found")
    if not results:
        raise RuntimeError("x verify result files contain no callsite results")

    if getattr(args, "unsafe_api_summary", False):
        if not unsafe_apis:
            raise RuntimeError("x verify result files contain no unsafe API metadata")
        all_apis: dict[str, dict[tuple[str, str], bool]] = {}
        for crate, crate_apis in sorted(unsafe_apis.items()):
            print(f"{crate}:")
            for unsafe_callee, callsites in sorted(crate_apis.items()):
                print(f"{unsafe_callee} {sum(callsites.values())}/{len(callsites)}")
                all_callsites = all_apis.setdefault(unsafe_callee, {})
                for callsite, is_bug in callsites.items():
                    all_callsites[(crate, callsite)] = is_bug
        print("all:")
        for unsafe_callee, callsites in sorted(all_apis.items()):
            print(f"{unsafe_callee} {sum(callsites.values())}/{len(callsites)}")
        unique_api_count = len(all_apis)
        buggy_api_count = sum(
            1 for callsites in all_apis.values() if any(callsites.values())
        )
        buggy_api_percentage = (
            buggy_api_count / unique_api_count * 100 if unique_api_count else 0.0
        )
        print(f"how many unique unsafe api: {unique_api_count}")
        print(
            "how many buggy/total unsafe api: "
            f"{buggy_api_count}/{unique_api_count} {buggy_api_percentage:.2f}%"
        )
        buggy_callsite_count = sum(
            sum(callsites.values()) for callsites in all_apis.values()
        )
        total_callsite_count = sum(len(callsites) for callsites in all_apis.values())
        buggy_callsite_percentage = (
            buggy_callsite_count / total_callsite_count * 100
            if total_callsite_count
            else 0.0
        )
        print(
            "how many buggy/total unsafe api callsites: "
            f"{buggy_callsite_count}/{total_callsite_count} "
            f"{buggy_callsite_percentage:.2f}%"
        )
        return 0

    for key, is_bug in sorted(results.items()):
        print(f"{' '.join(key)} {'BUG' if is_bug else 'OK'}")
    return 0
