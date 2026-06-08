"""eval command: run task1/2/3 LLM prompts against each crate meta JSON in crates/
and save results to eval/<meta_filename>.json.

Result shape:
  { "<rule_id>": { "task1": "...", "task2": "...", "task3": "..." }, ... }
"""

from __future__ import annotations

import argparse
import csv
import json
import sys
from pathlib import Path
from typing import Any


PLACEHOLDER = "<placeholder>"


# ---------------------------------------------------------------------------
# Helpers shared with sync (duplicated here to keep eval self-contained)
# ---------------------------------------------------------------------------


def _find_repo_root() -> Path:
    start = Path.cwd().resolve()
    for directory in [start, *start.parents]:
        if (directory / "crates").is_dir():
            return directory
    raise RuntimeError(
        f"could not locate repository root from {start} "
        "(expected an ancestor containing crates/)"
    )


def _parse_rule_path(path_with_line: str) -> tuple[str, int] | None:
    path_part, sep, line_part = path_with_line.rpartition(":")
    if not sep or not path_part:
        return None
    try:
        line = int(line_part)
    except ValueError:
        return None
    return path_part, line


def _load_rules_by_id(repo_root: Path) -> dict[str, dict[str, str]]:
    """Return mapping rule_id -> {name, rule, path} from rules.csv."""
    rules_path = repo_root / "rules.csv"
    if not rules_path.is_file():
        return {}

    result: dict[str, dict[str, str]] = {}
    with rules_path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        for idx, row in enumerate(reader, start=1):
            path_field = (row.get("path") or "").strip()
            rule_id = (row.get("id") or "").strip() or f"rule-{idx}"
            parsed = _parse_rule_path(path_field)
            result[rule_id] = {
                "id": rule_id,
                "name": (row.get("name") or "").strip(),
                "rule": (row.get("rule") or "").strip(),
                "path": parsed[0] if parsed else path_field,
            }
    return result


def _load_operators(repo_root: Path) -> list[dict]:
    operators_path = repo_root / "operators.json"
    if not operators_path.is_file():
        return []
    data = json.loads(operators_path.read_text(encoding="utf-8"))
    if isinstance(data, list):
        return [item for item in data if isinstance(item, dict)]
    if isinstance(data, dict):
        operators = data.get("operators")
        if isinstance(operators, list):
            return [item for item in operators if isinstance(item, dict)]
    return []


def _read_source_lines(
    repo_root: Path, crate_dir: str, relative_path: str, line_start: int, body_end: int
) -> str:
    """Read the source lines for a target function body."""
    # relative_path is relative to the crate directory
    full_path = repo_root / crate_dir / relative_path
    if not full_path.is_file():
        return f"(source not found: {full_path})"
    lines = full_path.read_text(encoding="utf-8").splitlines()
    start = max(0, line_start - 1)
    end = min(len(lines), body_end)
    return "\n".join(lines[start:end])


def _build_code_context(
    repo_root: Path,
    meta: dict[str, Any],
    target: dict[str, Any],
) -> str:
    crate_dir: str = meta.get("crate_dir", "")
    crate_name: str = meta.get("crate_name", "")
    caller: dict = target.get("caller") or target.get("target_fn", {})
    caller_parent: dict = target.get("caller_parent") or target.get("target_fn_parent", {})
    callee: dict = target.get("callee") or target.get("unsafe_call", {})
    callsite: dict = target.get("callsite", {})

    fn_name: str = caller.get("name", "")
    fn_path: str = caller.get("path", "")
    fn_line_start: int = caller.get("line_start", 1)
    fn_body_end: int = caller.get("body_end", fn_line_start)

    callsite_line: int = callsite.get("line", 0)
    callsite_col: int = callsite.get("col", 0)

    source_snippet = _read_source_lines(
        repo_root, crate_dir, fn_path, fn_line_start, fn_body_end
    )

    lines = [
        f"Crate: {crate_name}",
    ]
    if caller_parent:
        lines.append(f"Parent: {caller_parent.get('name', '')}")
    lines.append(f"Function: {fn_name}")
    lines.append(f"File: {fn_path}")
    if callee:
        lines.append(f"Unsafe call: {callee.get('name', '')}")
    lines.append(f"Callsite: {fn_path}:{callsite_line}:{callsite_col}")
    lines.append("")
    lines.append(source_snippet)

    return "\n".join(lines)


def _find_meta_jsons(crates_dir: Path) -> list[Path]:
    return sorted(p for p in crates_dir.glob("*.json"))


# ---------------------------------------------------------------------------
# Core evaluation logic
# ---------------------------------------------------------------------------


def _eval_target_rule(
    llm: Any,
    code_context: str,
    rule_text: str,
    operators: list[dict],
) -> dict[str, str]:
    from task1 import prompts as t1_prompts
    from task2 import prompts as t2_prompts
    from task3 import prompts as t3_prompts

    task1_result = llm.complete(
        system=t1_prompts.build_system(),
        user=t1_prompts.build_user(rule_text, code_context, operators),
    ).strip()

    task2_result = llm.complete(
        system=t2_prompts.build_system(),
        user=t2_prompts.build_user(rule_text, code_context),
    ).strip()

    task3_result = llm.complete(
        system=t3_prompts.build_system(),
        user=t3_prompts.build_user(code_context, task1_result),
    ).strip()

    return {
        "task1": task1_result,
        "task2": task2_result,
        "task3": task3_result,
    }


def _eval_meta(
    llm: Any,
    meta_path: Path,
    repo_root: Path,
    rules_by_id: dict[str, dict],
    operators: list[dict],
    verbose: bool,
) -> dict[str, dict[str, str]]:
    meta: dict[str, Any] = json.loads(meta_path.read_text(encoding="utf-8"))
    report = meta.get("report", {})
    targets: list[dict] = report.get("targets", [])

    results: dict[str, dict[str, str]] = {}

    for target in targets:
        rules: dict[str, dict] = target.get("rules", {})
        if not rules:
            continue

        code_context = _build_code_context(repo_root, meta, target)

        for rule_id, _tasks in rules.items():
            rule_record = rules_by_id.get(rule_id)
            if rule_record is None:
                if verbose:
                    print(f"  [skip] {rule_id}: not found in rules.csv", flush=True)
                continue

            rule_text: str = rule_record.get("rule", "")
            if not rule_text:
                if verbose:
                    print(f"  [skip] {rule_id}: empty rule text", flush=True)
                continue

            if verbose:
                caller_name = (target.get("caller") or target.get("target_fn", {})).get("name", "?")
                print(f"  {rule_id} / {caller_name} ...", flush=True)

            try:
                result = _eval_target_rule(llm, code_context, rule_text, operators)
            except Exception as exc:
                print(f"  [error] {rule_id}: {exc}", file=sys.stderr, flush=True)
                result = {
                    "task1": f"<error: {exc}>",
                    "task2": f"<error: {exc}>",
                    "task3": f"<error: {exc}>",
                }

            # If a rule appears in multiple targets, last result wins.
            results[rule_id] = result

    return results


# ---------------------------------------------------------------------------
# Command entry-point
# ---------------------------------------------------------------------------


def run(args: argparse.Namespace) -> int:
    from llm.openai_llm import OpenAILLM

    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"

    eval_dir = repo_root / "eval"
    eval_dir.mkdir(parents=True, exist_ok=True)

    rules_by_id = _load_rules_by_id(repo_root)
    operators = _load_operators(repo_root)

    model: str = args.model
    verbose: bool = args.verbose

    llm = OpenAILLM(model=model)

    meta_jsons = _find_meta_jsons(crates_dir)
    if not meta_jsons:
        print("No crate meta JSON files found in crates/.", file=sys.stderr)
        return 1

    for meta_path in meta_jsons:
        print(f"Evaluating {meta_path.name} ...", flush=True)
        results = _eval_meta(llm, meta_path, repo_root, rules_by_id, operators, verbose)

        out_path = eval_dir / meta_path.name
        out_path.write_text(
            json.dumps(results, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        print(f"  => {out_path.relative_to(repo_root)} ({len(results)} rule(s))", flush=True)

    return 0
