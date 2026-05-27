import argparse
import csv
import json
import os
from pathlib import Path
import shutil
import subprocess
import tomllib
from typing import Iterable

from dsl import DSLParseError, DSLValidationError, list_operators, parse_dsl, validate_task1_ast, validate_task2_ast


PLACEHOLDER = "<placeholder>"


def _find_repo_root() -> Path:
    start = Path.cwd().resolve()
    for directory in [start, *start.parents]:
        if (directory / "crates").is_dir():
            return directory
    raise RuntimeError(
        f"could not locate repository root from {start} (expected an ancestor containing crates/)"
    )


def _find_crates(crates_dir: Path) -> list[Path]:
    if not crates_dir.is_dir():
        raise RuntimeError(f"failed to read crates directory {crates_dir}")

    crate_dirs = [
        child
        for child in crates_dir.iterdir()
        if child.is_dir() and (child / "Cargo.toml").is_file()
    ]
    return sorted(crate_dirs)


def _build_local_mirscan(repo_root: Path) -> None:
    mirscan_manifest = repo_root / "tools" / "mirscan" / "Cargo.toml"
    if not mirscan_manifest.is_file():
        return

    result = subprocess.run(
        ["cargo", "build", "--manifest-path", str(mirscan_manifest)],
        cwd=repo_root,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            "building local mirscan failed "
            f"(manifest: {mirscan_manifest}, exit: {result.returncode})"
        )


def _resolve_mirscan_rustc(repo_root: Path) -> str:
    configured = os.environ.get("MIRSCAN_RUSTC")
    if configured:
        return configured

    local_candidates = [
        repo_root / "tools" / "mirscan" / "target" / "release" / "raudit",
        repo_root / "tools" / "mirscan" / "target" / "debug" / "raudit",
    ]
    for candidate in local_candidates:
        if candidate.is_file():
            return str(candidate)
        candidate_exe = candidate.with_suffix(".exe")
        if candidate_exe.is_file():
            return str(candidate_exe)

    _build_local_mirscan(repo_root)
    for candidate in local_candidates:
        if candidate.is_file():
            return str(candidate)
        candidate_exe = candidate.with_suffix(".exe")
        if candidate_exe.is_file():
            return str(candidate_exe)

    path_raudit = shutil.which("raudit")
    if path_raudit:
        return path_raudit
    path_mirscan = shutil.which("mirscan")
    if path_mirscan:
        return path_mirscan

    raise RuntimeError(
        "could not find mirscan rustc binary; set MIRSCAN_RUSTC or build tools/mirscan "
        "(expected tools/mirscan/target/{release,debug}/raudit)"
    )


def _compile_crate(crate_dir: Path, mirscan_rustc: str, report_path: Path) -> None:
    if report_path.exists():
        report_path.unlink()

    clean_result = subprocess.run(["cargo", "clean"], cwd=crate_dir, check=False)
    if clean_result.returncode != 0:
        raise RuntimeError(f"cargo clean failed in {crate_dir} (exit: {clean_result.returncode})")

    env = os.environ.copy()
    env["RUSTC"] = mirscan_rustc
    env["ANALYSIS_OUT"] = str(report_path)
    check_result = subprocess.run(["cargo", "check"], cwd=crate_dir, env=env, check=False)
    if check_result.returncode != 0:
        raise RuntimeError(
            f"cargo check failed in {crate_dir} with RUSTC={mirscan_rustc} "
            f"and ANALYSIS_OUT={report_path} (exit: {check_result.returncode})"
        )


def _parse_package_name_and_description(cargo_toml_content: str) -> tuple[str | None, str | None]:
    manifest = tomllib.loads(cargo_toml_content)
    package = manifest.get("package")
    if not isinstance(package, dict):
        return None, None

    name = package.get("name")
    description = package.get("description")

    return (
        name if isinstance(name, str) else None,
        description if isinstance(description, str) else None,
    )


def _normalize_to_rust_relative(path_value: str) -> str:
    path = path_value.replace("\\", "/")
    marker = "/rust/"
    index = path.find(marker)
    if index >= 0:
        return path[index + 1 :]
    if path.startswith("rust/"):
        return path
    return path


def _parse_rule_path(path_with_line: str) -> tuple[str, int] | None:
    path_part, sep, line_part = path_with_line.rpartition(":")
    if not sep or not path_part:
        return None
    try:
        line = int(line_part)
    except ValueError:
        return None
    return path_part, line


def _load_studied_rule_ids(studied_rules_path: Path) -> set[str]:
    if not studied_rules_path.is_file():
        raise RuntimeError(f"studied rules file does not exist: {studied_rules_path}")

    studied_rule_ids: set[str] = set()
    for raw_line in studied_rules_path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        studied_rule_ids.add(line)
    return studied_rule_ids


def _load_rules_by_path(repo_root: Path) -> dict[str, list[dict[str, object]]]:
    rules_path = repo_root / "rules.csv"
    if not rules_path.is_file():
        return {}

    grouped: dict[str, list[dict[str, object]]] = {}
    with rules_path.open("r", encoding="utf-8", newline="") as infile:
        reader = csv.DictReader(infile)
        for idx, row in enumerate(reader, start=1):
            path_field = (row.get("path") or "").strip()
            parsed = _parse_rule_path(path_field)
            if parsed is None:
                continue
            rule_path, rule_line = parsed
            normalized_rule_path = _normalize_to_rust_relative(rule_path)
            rule_id = (row.get("id") or "").strip() or f"rule-{idx}"

            record = {
                "id": rule_id,
                "line": rule_line,
                "name": (row.get("name") or "").strip(),
                "rule": (row.get("rule") or "").strip(),
            }
            grouped.setdefault(normalized_rule_path, []).append(record)

    return grouped


def _load_operator_entries(repo_root: Path) -> list[dict[str, object]]:
    operators_path = repo_root / "operators.json"
    if not operators_path.is_file():
        return []

    data = json.loads(operators_path.read_text(encoding="utf-8"))
    if not isinstance(data, list):
        raise RuntimeError(f"operators.json must be a JSON array: {operators_path}")

    entries: list[dict[str, object]] = []
    for item in data:
        if not isinstance(item, dict):
            raise RuntimeError(f"operators.json entries must be JSON objects: {operators_path}")
        entries.append(dict(item))
    return entries


def _save_operator_entries(repo_root: Path, operators: list[dict[str, object]]) -> None:
    operators_path = repo_root / "operators.json"
    operators_path.write_text(json.dumps(operators, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def _operator_names(operators: list[dict[str, object]]) -> set[str]:
    names: set[str] = set()
    for entry in operators:
        name = entry.get("name")
        if isinstance(name, str) and name:
            names.add(name)
    return names


def _merge_new_operator_entries(
    operators: list[dict[str, object]],
    new_operator_names: Iterable[str],
) -> list[str]:
    existing = _operator_names(operators)
    added: list[str] = []
    for name in sorted(set(new_operator_names)):
        if name in existing:
            continue
        operators.append(
            {
                "name": name,
                "input": [],
                "output": {"type": "Unknown"},
                "description": "Discovered from task1 DSL during sync.",
            }
        )
        existing.add(name)
        added.append(name)
    return added


def _target_identity(target: dict[str, object]) -> tuple[str, str, str, int, int]:
    caller = target.get("caller")
    if not isinstance(caller, dict):
        caller = target.get("target_fn")

    callee = target.get("callee")
    if not isinstance(callee, dict):
        callee = target.get("unsafe_call")

    callsite = target.get("callsite")

    target_name = caller.get("name") if isinstance(caller, dict) else ""
    unsafe_name = callee.get("name") if isinstance(callee, dict) else ""
    unsafe_path = callee.get("path") if isinstance(callee, dict) else ""
    callsite_line = callsite.get("line") if isinstance(callsite, dict) else 0
    callsite_col = callsite.get("col") if isinstance(callsite, dict) else 0

    return (
        target_name if isinstance(target_name, str) else "",
        unsafe_name if isinstance(unsafe_name, str) else "",
        unsafe_path if isinstance(unsafe_path, str) else "",
        callsite_line if isinstance(callsite_line, int) else 0,
        callsite_col if isinstance(callsite_col, int) else 0,
    )


def _load_existing_rule_tasks(meta_path: Path) -> dict[tuple[str, str, str, int, int], dict[str, dict[str, str]]]:
    if not meta_path.is_file():
        return {}

    try:
        data = json.loads(meta_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {}

    if not isinstance(data, dict):
        return {}
    report = data.get("report")
    if not isinstance(report, dict):
        return {}
    targets = report.get("targets")
    if not isinstance(targets, list):
        return {}

    saved: dict[tuple[str, str, str, int, int], dict[str, dict[str, str]]] = {}
    for raw_target in targets:
        if not isinstance(raw_target, dict):
            continue
        rules = raw_target.get("rules")
        if not isinstance(rules, dict):
            continue

        preserved_rules: dict[str, dict[str, str]] = {}
        for rule_id, rule_tasks in rules.items():
            if not isinstance(rule_id, str) or not isinstance(rule_tasks, dict):
                continue

            preserved_rules[rule_id] = {
                "task1": rule_tasks.get("task1") if isinstance(rule_tasks.get("task1"), str) else PLACEHOLDER,
                "task2": rule_tasks.get("task2") if isinstance(rule_tasks.get("task2"), str) else PLACEHOLDER,
                "task3": rule_tasks.get("task3") if isinstance(rule_tasks.get("task3"), str) else PLACEHOLDER,
            }

        saved[_target_identity(raw_target)] = preserved_rules

    return saved


def _validate_task_dsl(
    crate_name: str,
    target: dict[str, object],
    rule_id: str,
    task_name: str,
    dsl_text: str,
    operators: list[dict[str, object]],
    repo_root: Path,
) -> None:
    caller = target.get("caller")
    if not isinstance(caller, dict):
        caller = target.get("target_fn")
    target_name = caller.get("name") if isinstance(caller, dict) else "<unknown target>"

    try:
        ast = parse_dsl(dsl_text, operators, allow_unknown_operators=True)
        if task_name == "task1":
            used_operators = list_operators(ast)
            added = _merge_new_operator_entries(operators, used_operators)
            if added:
                _save_operator_entries(repo_root, operators)
            validate_task1_ast(ast, used_operators)
        elif task_name == "task2":
            validate_task2_ast(ast)
    except (DSLParseError, DSLValidationError) as exc:
        print(
            f"invalid {task_name} DSL for crate={crate_name} target={target_name} rule={rule_id}: {exc}"
        )


def _merge_rule_tasks(
    crate_name: str,
    target: dict[str, object],
    matched_rules: dict[str, dict[str, str]],
    preserved_rules: dict[str, dict[str, str]],
    operators: list[dict[str, object]],
    repo_root: Path,
) -> dict[str, dict[str, str]]:
    merged: dict[str, dict[str, str]] = {}

    for rule_id, default_tasks in matched_rules.items():
        existing = preserved_rules.get(rule_id, {})
        merged_tasks = {
            "task1": existing.get("task1", default_tasks["task1"]),
            "task2": existing.get("task2", default_tasks["task2"]),
            "task3": existing.get("task3", default_tasks["task3"]),
        }

        for task_name in ("task1", "task2"):
            dsl_text = merged_tasks.get(task_name)
            if isinstance(dsl_text, str) and dsl_text != PLACEHOLDER:
                _validate_task_dsl(crate_name, target, rule_id, task_name, dsl_text, operators, repo_root)

        merged[rule_id] = merged_tasks

    return merged


def _match_rules_for_target(
    target: dict[str, object],
    rules_by_path: dict[str, list[dict[str, object]]],
    allowed_rule_ids: set[str],
) -> dict[str, dict[str, str]]:
    callee = target.get("callee")
    if not isinstance(callee, dict):
        callee = target.get("unsafe_call")
    if not isinstance(callee, dict):
        return {}

    unsafe_path = callee.get("path")
    line_start = callee.get("line_start")
    line_end = callee.get("line_end")
    if not isinstance(unsafe_path, str):
        return {}

    matched_rules: dict[str, dict[str, str]] = {}
    candidates = rules_by_path.get(unsafe_path, [])

    for candidate in candidates:
        candidate_line = candidate.get("line")
        rule_id = candidate.get("id")
        if not isinstance(candidate_line, int) or not isinstance(rule_id, str):
            continue
        if rule_id not in allowed_rule_ids:
            continue

        is_match = False
        if isinstance(line_start, int) and isinstance(line_end, int):
            is_match = line_start <= candidate_line <= line_end
        elif isinstance(line_start, int):
            is_match = candidate_line == line_start

        if is_match:
            matched_rules[rule_id] = {
                "task1": "<placeholder>",
                "task2": "<placeholder>",
                "task3": "<placeholder>",
            }

    if matched_rules:
        return matched_rules

    # Fallback for rust source version drift: if line numbers differ,
    # match rules in the same file by unsafe function name token.
    unsafe_name = callee.get("name")
    if not isinstance(unsafe_name, str):
        return matched_rules

    fn_name = unsafe_name.rsplit("::", maxsplit=1)[-1].strip().lower()
    if not fn_name:
        return matched_rules

    for candidate in candidates:
        rule_id = candidate.get("id")
        rule_name = candidate.get("name")
        if not isinstance(rule_id, str) or not isinstance(rule_name, str):
            continue
        if rule_id not in allowed_rule_ids:
            continue

        normalized_rule_name = " ".join(rule_name.lower().split())
        if fn_name in normalized_rule_name:
            matched_rules[rule_id] = {
                "task1": "<placeholder>",
                "task2": "<placeholder>",
                "task3": "<placeholder>",
            }

    return matched_rules


def _normalize_target_schema(raw_target: dict[str, object]) -> dict[str, object]:
    target = dict(raw_target)

    if "caller" not in target and isinstance(target.get("target_fn"), dict):
        target["caller"] = target["target_fn"]
    if "callee" not in target and isinstance(target.get("unsafe_call"), dict):
        target["callee"] = target["unsafe_call"]
    if "caller_parent" not in target and isinstance(target.get("target_fn_parent"), dict):
        target["caller_parent"] = target["target_fn_parent"]

    target.pop("target_fn", None)
    target.pop("unsafe_call", None)
    target.pop("target_fn_parent", None)
    return target


def _transform_report(
    crate_name: str,
    report: dict[str, object],
    rules_by_path: dict[str, list[dict[str, object]]],
    allowed_rule_ids: set[str],
    preserved_rule_tasks: dict[tuple[str, str, str, int, int], dict[str, dict[str, str]]],
    operators: list[dict[str, object]],
    repo_root: Path,
) -> dict[str, object]:
    report_targets = report.get("targets")
    if not isinstance(report_targets, list):
        raise RuntimeError("mirscan report is missing required targets array")

    targets_input = report_targets

    targets: list[dict[str, object]] = []
    for raw_target in targets_input:
        if not isinstance(raw_target, dict):
            continue

        target = _normalize_target_schema(raw_target)
        matched_rules = _match_rules_for_target(target, rules_by_path, allowed_rule_ids)
        preserved_rules = preserved_rule_tasks.get(_target_identity(target), {})
        target["rules"] = _merge_rule_tasks(
            crate_name,
            target,
            matched_rules,
            preserved_rules,
            operators,
            repo_root,
        )
        targets.append(target)

    return {"targets": targets}


def run(args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"
    studied_rules_path = Path(args.studied_rules)
    if not studied_rules_path.is_absolute():
        studied_rules_path = (repo_root / studied_rules_path).resolve()
    allowed_rule_ids = _load_studied_rule_ids(studied_rules_path)

    rules_by_path = _load_rules_by_path(repo_root)
    operators = _load_operator_entries(repo_root)

    mirscan_rustc = _resolve_mirscan_rustc(repo_root)
    crate_dirs = _find_crates(crates_dir)

    for crate_dir in crate_dirs:
        crate_name = crate_dir.name or "unknown"
        crate_dir_relative = crate_dir.relative_to(repo_root).as_posix()
        out_path = crates_dir / f"{crate_name}.json"
        preserved_rule_tasks = _load_existing_rule_tasks(out_path)

        print(f"syncing crate {crate_name} with rustc={mirscan_rustc}")

        report_path = crate_dir / "report.json"
        _compile_crate(crate_dir, mirscan_rustc, report_path)

        try:
            report = json.loads(report_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            raise RuntimeError(
                f"report.json is not valid JSON for crate {crate_name} at {report_path}"
            ) from exc

        if not isinstance(report, dict):
            raise RuntimeError(f"report.json must be a JSON object for crate {crate_name} at {report_path}")

        transformed_report = _transform_report(
            crate_name,
            report,
            rules_by_path,
            allowed_rule_ids,
            preserved_rule_tasks,
            operators,
            repo_root,
        )
        report_path.write_text(
            json.dumps(transformed_report, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )

        cargo_toml_path = crate_dir / "Cargo.toml"
        cargo_toml_content = cargo_toml_path.read_text(encoding="utf-8")
        parsed_name, parsed_description = _parse_package_name_and_description(cargo_toml_content)

        out = {
            "crate_dir": crate_dir_relative,
            "crate_name": parsed_name,
            "description": parsed_description,
            "report": transformed_report,
        }

        out_path.write_text(json.dumps(out, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    return 0
