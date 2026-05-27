from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


PLACEHOLDER = "<placeholder>"


def _find_repo_root() -> Path:
    start = Path.cwd().resolve()
    for directory in [start, *start.parents]:
        if (directory / "crates").is_dir():
            return directory
    raise RuntimeError(
        f"could not locate repository root from {start} "
        "(expected an ancestor containing crates/)"
    )


def _find_meta_jsons(crates_dir: Path) -> list[Path]:
    return sorted(p for p in crates_dir.glob("*.json"))


def _task_counts_from_target(target: dict[str, Any]) -> tuple[int, int]:
    rules = target.get("rules")
    if not isinstance(rules, dict):
        return 0, 0

    total = 0
    marked = 0
    for rule_tasks in rules.values():
        if not isinstance(rule_tasks, dict):
            continue

        for task_key in ("task1", "task2", "task3"):
            value = rule_tasks.get(task_key)
            if not isinstance(value, str):
                continue

            total += 1
            if value.strip() and value.strip() != PLACEHOLDER:
                marked += 1

    return total, marked


def _summarize_meta(meta_path: Path) -> tuple[str, str, int, int, int]:
    meta = json.loads(meta_path.read_text(encoding="utf-8"))
    if not isinstance(meta, dict):
        raise RuntimeError(f"invalid crate meta JSON object: {meta_path}")

    crate_file_name = meta_path.stem
    crate_name = meta.get("crate_name") if isinstance(meta.get("crate_name"), str) else ""

    report = meta.get("report")
    targets = report.get("targets") if isinstance(report, dict) else None
    if not isinstance(targets, list):
        return crate_file_name, crate_name, 0, 0, 0

    target_count = 0
    marked_tasks = 0
    total_tasks = 0

    for raw_target in targets:
        if not isinstance(raw_target, dict):
            continue
        target_count += 1

        target_total, target_marked = _task_counts_from_target(raw_target)
        total_tasks += target_total
        marked_tasks += target_marked

    return crate_file_name, crate_name, target_count, marked_tasks, total_tasks


def run(_args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"

    meta_jsons = _find_meta_jsons(crates_dir)
    if not meta_jsons:
        print("No crate meta JSON files found in crates/.")
        return 1

    total_crates = 0
    total_targets = 0
    total_marked_tasks = 0
    total_tasks = 0

    for meta_path in meta_jsons:
        crate_file_name, crate_name, targets, marked, tasks = _summarize_meta(meta_path)
        display_name = f"{crate_file_name} ({crate_name})" if crate_name else crate_file_name
        print(f"{display_name} {targets} {marked}/{tasks}")

        total_crates += 1
        total_targets += targets
        total_marked_tasks += marked
        total_tasks += tasks

    print(
        f"TOTAL crates={total_crates} targets={total_targets} "
        f"marked={total_marked_tasks}/{total_tasks}"
    )
    return 0
