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
HUMAN_PLACEHOLDER = "placeholder"
TASK_NAMES = ("task1", "task2", "task3")


def _task3_placeholder() -> list[dict[str, str]]:
    return [{"expression": PLACEHOLDER, "resolve": PLACEHOLDER}]


def _default_human_rule_tasks() -> dict[str, object]:
    return {
        "task1": PLACEHOLDER,
        "task2": PLACEHOLDER,
        "task3": _task3_placeholder(),
    }


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


def _build_local_autoinj(repo_root: Path) -> None:
    autoinj_manifest = repo_root / "tools" / "autoinj" / "Cargo.toml"
    if not autoinj_manifest.is_file():
        raise RuntimeError(f"could not find autoinj manifest: {autoinj_manifest}")

    result = subprocess.run(
        ["cargo", "build", "--manifest-path", str(autoinj_manifest)],
        cwd=repo_root,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            "building local autoinj failed "
            f"(manifest: {autoinj_manifest}, exit: {result.returncode})"
        )


def _resolve_autoinj_binary(repo_root: Path) -> str:
    configured = os.environ.get("AUTOINJ_BIN")
    if configured:
        return configured

    local_candidates = [
        repo_root / "tools" / "autoinj" / "target" / "release" / "autoinj",
        repo_root / "tools" / "autoinj" / "target" / "debug" / "autoinj",
    ]
    for candidate in local_candidates:
        if candidate.is_file():
            return str(candidate)
        candidate_exe = candidate.with_suffix(".exe")
        if candidate_exe.is_file():
            return str(candidate_exe)

    path_autoinj = shutil.which("autoinj")
    if path_autoinj:
        return path_autoinj

    _build_local_autoinj(repo_root)
    for candidate in local_candidates:
        if candidate.is_file():
            return str(candidate)
        candidate_exe = candidate.with_suffix(".exe")
        if candidate_exe.is_file():
            return str(candidate_exe)

    raise RuntimeError(
        "could not find autoinj binary; set AUTOINJ_BIN or build tools/autoinj "
        "(expected tools/autoinj/target/{release,debug}/autoinj)"
    )


def _looks_like_autoinj_output(dest_dir: Path) -> bool:
    manifest = dest_dir / "Cargo.toml"
    if not manifest.is_file():
        return False
    return "klee-ext-bind" in manifest.read_text(encoding="utf-8", errors="replace")


def _prepare_autoinj_destination(dest_dir: Path) -> None:
    if not dest_dir.exists():
        return
    if not dest_dir.is_dir():
        raise RuntimeError(f"autoinj destination exists and is not a directory: {dest_dir}")
    if not _looks_like_autoinj_output(dest_dir):
        raise RuntimeError(
            f"autoinj destination already exists and does not look generated: {dest_dir}"
        )
    shutil.rmtree(dest_dir)


def _run_autoinj_for_crate(
    *,
    repo_root: Path,
    crate_dir: Path,
    meta_path: Path,
    dest_root: Path,
    autoinj_bin: str,
) -> Path:
    dest_dir = dest_root / crate_dir.name
    _prepare_autoinj_destination(dest_dir)
    dest_root.mkdir(parents=True, exist_ok=True)

    result = subprocess.run(
        [autoinj_bin, str(crate_dir), str(meta_path), str(dest_dir)],
        cwd=repo_root,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"autoinj failed for crate {crate_dir.name} -> {dest_dir} "
            f"(exit: {result.returncode})"
        )
    return dest_dir


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


def _callsite_id_from_target(target: dict[str, object], fallback_index: int) -> str:
    caller = target.get("caller")
    if not isinstance(caller, dict):
        caller = target.get("target_fn")

    callsite = target.get("callsite")
    if not isinstance(caller, dict) or not isinstance(callsite, dict):
        return str(fallback_index)

    path = caller.get("path")
    line = callsite.get("line")
    col = callsite.get("col")
    if not isinstance(path, str) or not isinstance(line, int) or not isinstance(col, int):
        return str(fallback_index)

    normalized_path = path.replace("\\", "-").replace("/", "-").replace(".", "-")
    return f"{normalized_path}-{line}-{col}"


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


def _load_operator_entries(repo_root: Path) -> tuple[dict[str, object] | None, list[dict[str, object]]]:
    operators_path = repo_root / "operators.json"
    if not operators_path.is_file():
        return None, []

    data = json.loads(operators_path.read_text(encoding="utf-8"))
    document: dict[str, object] | None = None
    raw_entries: object
    if isinstance(data, list):
        raw_entries = data
    elif isinstance(data, dict):
        raw_entries = data.get("operators")
        if not isinstance(raw_entries, list):
            raise RuntimeError(f"operators.json object must contain an operators array: {operators_path}")
        document = dict(data)
    else:
        raise RuntimeError(f"operators.json must be a JSON array or object: {operators_path}")

    entries: list[dict[str, object]] = []
    for item in raw_entries:
        if not isinstance(item, dict):
            raise RuntimeError(f"operators.json entries must be JSON objects: {operators_path}")
        entries.append(dict(item))
    return document, entries


def _save_operator_entries(
    repo_root: Path,
    operators: list[dict[str, object]],
    document: dict[str, object] | None,
) -> None:
    operators_path = repo_root / "operators.json"
    if document is None:
        payload: object = operators
    else:
        payload = dict(document)
        payload["operators"] = operators
    operators_path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


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

            preserved_tasks: dict[str, str] = {}
            for task_name in ("task1", "task2", "task3"):
                task_value = rule_tasks.get(task_name)
                if isinstance(task_value, str):
                    preserved_tasks[task_name] = task_value

            preserved_rules[rule_id] = preserved_tasks

        saved[_target_identity(raw_target)] = preserved_rules

    return saved


def _validate_task_dsl(
    crate_name: str,
    target: dict[str, object],
    rule_id: str,
    task_name: str,
    dsl_text: str,
    operators: list[dict[str, object]],
    operator_document: dict[str, object] | None,
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
                _save_operator_entries(repo_root, operators, operator_document)
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
    operator_document: dict[str, object] | None,
    repo_root: Path,
) -> dict[str, dict[str, str]]:
    merged: dict[str, dict[str, str]] = {}

    for rule_id in matched_rules.keys():
        existing = preserved_rules.get(rule_id, {})
        merged_tasks: dict[str, str] = {}

        for task_name in ("task1", "task2", "task3"):
            task_text = existing.get(task_name)
            if not isinstance(task_text, str):
                continue
            if not task_text.strip() or task_text == PLACEHOLDER:
                continue
            merged_tasks[task_name] = task_text

        for task_name in ("task1", "task2"):
            dsl_text = merged_tasks.get(task_name)
            if isinstance(dsl_text, str):
                _validate_task_dsl(
                    crate_name,
                    target,
                    rule_id,
                    task_name,
                    dsl_text,
                    operators,
                    operator_document,
                    repo_root,
                )

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
    if not isinstance(unsafe_path, str) or not isinstance(line_start, int):
        return {}

    matched_rules: dict[str, dict[str, str]] = {}
    candidates = rules_by_path.get(unsafe_path, [])

    def _matches_line_and_allowlist(candidate: dict[str, object], *, enforce_allowlist: bool) -> bool:
        candidate_line = candidate.get("line")
        rule_id = candidate.get("id")
        if not isinstance(candidate_line, int) or not isinstance(rule_id, str):
            return False
        if enforce_allowlist and rule_id not in allowed_rule_ids:
            return False
        return candidate_line == line_start

    for candidate in candidates:
        rule_id = candidate.get("id")
        if isinstance(rule_id, str) and _matches_line_and_allowlist(candidate, enforce_allowlist=True):
            matched_rules[rule_id] = {}

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
    operator_document: dict[str, object] | None,
    repo_root: Path,
) -> tuple[dict[str, object], dict[str, dict[str, dict[str, object]]]]:
    report_targets = report.get("targets")
    if not isinstance(report_targets, list):
        raise RuntimeError("mirscan report is missing required targets array")

    targets_input = report_targets

    targets: list[dict[str, object]] = []
    human_placeholders: dict[str, dict[str, dict[str, object]]] = {}
    for target_index, raw_target in enumerate(targets_input, start=1):
        if not isinstance(raw_target, dict):
            continue

        target = _normalize_target_schema(raw_target)
        callsite_id = _callsite_id_from_target(target, target_index)
        callsite = target.get("callsite")
        if isinstance(callsite, dict):
            callsite_with_id = dict(callsite)
            callsite_with_id["id"] = callsite_id
            target["callsite"] = callsite_with_id
        matched_rules = _match_rules_for_target(target, rules_by_path, allowed_rule_ids)
        preserved_rules = preserved_rule_tasks.get(_target_identity(target), {})
        merged_rules = _merge_rule_tasks(
            crate_name,
            target,
            matched_rules,
            preserved_rules,
            operators,
            operator_document,
            repo_root,
        )

        if merged_rules:
            human_placeholders[callsite_id] = {
                rule_id: _default_human_rule_tasks()
                for rule_id in merged_rules.keys()
            }

        # Keep crate metadata report free of rule/task payloads; those live in human/<crate>.json.
        target.pop("rules", None)
        targets.append(target)
    transformed: dict[str, object] = {"targets": targets}
    types = report.get("types")
    if isinstance(types, list):
        transformed["types"] = types
    return transformed, human_placeholders


def _resolve_studied_rules_path(repo_root: Path, studied_rules: str | Path) -> Path:
    studied_rules_path = Path(studied_rules)
    if not studied_rules_path.is_absolute():
        studied_rules_path = (repo_root / studied_rules_path).resolve()
    return studied_rules_path


def _normalize_loaded_human_report(loaded: object) -> dict[str, dict[str, object]]:
    if not isinstance(loaded, dict):
        return {}

    existing: dict[str, dict[str, object]] = {}
    for callsite_id, raw_rules in loaded.items():
        if not isinstance(callsite_id, str) or not isinstance(raw_rules, dict):
            continue
        existing_rules: dict[str, object] = {}
        for rule_id, task in raw_rules.items():
            if isinstance(rule_id, str):
                existing_rules[rule_id] = task
        existing[callsite_id] = existing_rules
    return existing


def _load_existing_human_report(report_path: Path) -> dict[str, dict[str, object]]:
    if not report_path.is_file():
        return {}

    try:
        loaded = json.loads(report_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {}

    return _normalize_loaded_human_report(loaded)


def _load_existing_human_folder(report_dir: Path) -> dict[str, dict[str, object]]:
    if not report_dir.is_dir():
        return {}

    loaded: dict[str, object] = {}
    for callsite_dir in sorted(child for child in report_dir.iterdir() if child.is_dir()):
        rules: dict[str, object] = {}
        for rule_dir in sorted(child for child in callsite_dir.iterdir() if child.is_dir()):
            tasks: dict[str, object] = {}
            for task_name in TASK_NAMES:
                task_path = rule_dir / task_name
                if not task_path.is_file():
                    continue
                text = task_path.read_text(encoding="utf-8").rstrip("\n")
                if task_name == "task3":
                    stripped = text.strip()
                    if stripped.startswith("[") or stripped.startswith("{"):
                        try:
                            tasks[task_name] = json.loads(stripped)
                            continue
                        except json.JSONDecodeError:
                            pass
                tasks[task_name] = text
            if tasks:
                rules[rule_dir.name] = tasks
        if rules:
            loaded[callsite_dir.name] = rules
    return _normalize_loaded_human_report(loaded)


def _normalize_human_rule_tasks(task_value: object) -> dict[str, object]:
    normalized = _default_human_rule_tasks()

    # Backward compatibility with old schema: rule -> "placeholder" or rule -> "task1 text".
    if isinstance(task_value, str):
        value = task_value.strip()
        if value:
            normalized["task1"] = task_value
        return normalized

    if not isinstance(task_value, dict):
        return normalized

    task1 = task_value.get("task1")
    if isinstance(task1, str) and task1.strip():
        normalized["task1"] = task1

    task2 = task_value.get("task2")
    if isinstance(task2, str) and task2.strip():
        normalized["task2"] = task2

    task3 = task_value.get("task3")
    if isinstance(task3, list):
        normalized["task3"] = task3
    elif isinstance(task3, str) and task3.strip() and task3.strip() != PLACEHOLDER:
        # Preserve unusual legacy task3 text instead of dropping user-authored content.
        normalized["task3"] = task3

    return normalized


def _is_non_placeholder_task(value: object) -> bool:
    if isinstance(value, str):
        normalized = value.strip()
        return normalized not in {"", PLACEHOLDER, HUMAN_PLACEHOLDER}

    if isinstance(value, dict):
        for candidate in value.values():
            if _is_non_placeholder_task(candidate):
                return True
        return False

    if isinstance(value, list):
        for candidate in value:
            if _is_non_placeholder_task(candidate):
                return True
        return False

    return False


def _merge_human_report(
    existing: dict[str, dict[str, object]],
    incoming: dict[str, dict[str, dict[str, object]]],
    *,
    strict: bool,
) -> dict[str, dict[str, object]]:
    merged: dict[str, dict[str, object]] = {}

    for callsite_id, incoming_rules in incoming.items():
        existing_rules = existing.get(callsite_id, {})
        merged_rules: dict[str, object] = {}

        for rule_id, placeholder_tasks in incoming_rules.items():
            existing_task = existing_rules.get(rule_id)
            if _is_non_placeholder_task(existing_task):
                merged_rules[rule_id] = _normalize_human_rule_tasks(existing_task)
            elif existing_task is not None:
                merged_rules[rule_id] = _normalize_human_rule_tasks(existing_task)
            else:
                merged_rules[rule_id] = _normalize_human_rule_tasks(placeholder_tasks)

        if not strict:
            for rule_id, existing_task in existing_rules.items():
                if rule_id not in merged_rules and _is_non_placeholder_task(existing_task):
                    merged_rules[rule_id] = _normalize_human_rule_tasks(existing_task)

        if merged_rules:
            merged[callsite_id] = merged_rules

    if not strict:
        for callsite_id, existing_rules in existing.items():
            if callsite_id in merged:
                continue
            preserved_rules = {
                rule_id: _normalize_human_rule_tasks(task)
                for rule_id, task in existing_rules.items()
                if _is_non_placeholder_task(task)
            }
            if preserved_rules:
                merged[callsite_id] = preserved_rules

    return merged


def _write_task_file(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(value, (dict, list)):
        text = json.dumps(value, indent=2, ensure_ascii=False)
    else:
        text = str(value)
    path.write_text(text + "\n", encoding="utf-8")


def _prune_human_folder(report_dir: Path, merged: dict[str, dict[str, object]]) -> None:
    if not report_dir.is_dir():
        return

    for callsite_dir in [child for child in report_dir.iterdir() if child.is_dir()]:
        expected_rules = merged.get(callsite_dir.name, {})
        for rule_dir in [child for child in callsite_dir.iterdir() if child.is_dir()]:
            expected_task = expected_rules.get(rule_dir.name)
            expected_task_names = set(TASK_NAMES) if isinstance(expected_task, dict) else set()
            for task_file in [child for child in rule_dir.iterdir() if child.is_file()]:
                if task_file.name not in expected_task_names:
                    task_file.unlink()
            if not expected_task_names and not any(rule_dir.iterdir()):
                rule_dir.rmdir()
        if not expected_rules and not any(callsite_dir.iterdir()):
            callsite_dir.rmdir()


def _write_human_folder(
    report_dir: Path,
    merged: dict[str, dict[str, object]],
    *,
    strict: bool,
) -> None:
    report_dir.mkdir(parents=True, exist_ok=True)
    if strict:
        _prune_human_folder(report_dir, merged)

    for callsite_id, rules in merged.items():
        for rule_id, task in rules.items():
            normalized = _normalize_human_rule_tasks(task)
            for task_name in TASK_NAMES:
                _write_task_file(report_dir / callsite_id / rule_id / task_name, normalized[task_name])


def _sync_human_report(
    *,
    repo_root: Path,
    report_name: str,
    incoming: dict[str, dict[str, dict[str, object]]],
    strict: bool,
) -> Path:
    human_dir = repo_root / "human"
    human_dir.mkdir(parents=True, exist_ok=True)
    human_report_path = human_dir / report_name
    human_report_dir = human_dir / Path(report_name).stem

    existing = _load_existing_human_report(human_report_path)
    existing.update(_load_existing_human_folder(human_report_dir))
    merged = _merge_human_report(existing, incoming, strict=strict)

    _write_human_folder(human_report_dir, merged, strict=strict)
    return human_report_dir


def _sync_single_crate(
    *,
    repo_root: Path,
    crate_dir: Path,
    allowed_rule_ids: set[str],
    rules_by_path: dict[str, list[dict[str, object]]],
    operators: list[dict[str, object]],
    operator_document: dict[str, object] | None,
    mirscan_rustc: str,
    strict: bool,
    autoinj_output_dir: Path | None,
    autoinj_bin: str | None,
) -> Path:
    crate_name = crate_dir.name or "unknown"
    crates_dir = repo_root / "crates"
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

    transformed_report, human_placeholders = _transform_report(
        crate_name,
        report,
        rules_by_path,
        allowed_rule_ids,
        preserved_rule_tasks,
        operators,
        operator_document,
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
    _sync_human_report(
        repo_root=repo_root,
        report_name=out_path.name,
        incoming=human_placeholders,
        strict=strict,
    )
    if autoinj_output_dir is not None and autoinj_bin is not None:
        injected_dir = _run_autoinj_for_crate(
            repo_root=repo_root,
            crate_dir=crate_dir,
            meta_path=out_path,
            dest_root=autoinj_output_dir,
            autoinj_bin=autoinj_bin,
        )
        print(f"wrote injected crate to {injected_dir}")
    return out_path


def ensure_crate_metadata_file(
    repo_root: Path,
    cargo_dir: str | Path,
    *,
    studied_rules: str | Path = "studied_rules",
    force: bool = False,
    strict: bool = False,
) -> Path:
    cargo_dir_path = Path(cargo_dir)
    if not cargo_dir_path.is_absolute():
        cargo_dir_path = (repo_root / cargo_dir_path).resolve()
    else:
        cargo_dir_path = cargo_dir_path.resolve()

    if not cargo_dir_path.is_dir():
        raise RuntimeError(f"provided cargo dir is not a directory: {cargo_dir_path}")
    if not (cargo_dir_path / "Cargo.toml").is_file():
        raise RuntimeError(f"provided cargo dir does not contain Cargo.toml: {cargo_dir_path}")

    out_path = (repo_root / "crates" / f"{cargo_dir_path.name}.json").resolve()
    if out_path.is_file() and not force:
        try:
            loaded = json.loads(out_path.read_text(encoding="utf-8"))
            if isinstance(loaded, dict):
                return out_path
        except json.JSONDecodeError:
            # Existing metadata is malformed; regenerate it.
            pass

    studied_rules_path = _resolve_studied_rules_path(repo_root, studied_rules)
    allowed_rule_ids = _load_studied_rule_ids(studied_rules_path)
    rules_by_path = _load_rules_by_path(repo_root)
    operator_document, operators = _load_operator_entries(repo_root)
    mirscan_rustc = _resolve_mirscan_rustc(repo_root)

    return _sync_single_crate(
        repo_root=repo_root,
        crate_dir=cargo_dir_path,
        allowed_rule_ids=allowed_rule_ids,
        rules_by_path=rules_by_path,
        operators=operators,
        operator_document=operator_document,
        mirscan_rustc=mirscan_rustc,
        strict=strict,
        autoinj_output_dir=None,
        autoinj_bin=None,
    )


def run(args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"
    cargo_dir = getattr(args, "cargo_dir", None)

    studied_rules_path = _resolve_studied_rules_path(repo_root, args.studied_rules)
    allowed_rule_ids = _load_studied_rule_ids(studied_rules_path)
    rules_by_path = _load_rules_by_path(repo_root)
    operator_document, operators = _load_operator_entries(repo_root)
    mirscan_rustc = _resolve_mirscan_rustc(repo_root)
    strict = bool(getattr(args, "strict", False))
    skip_autoinj = bool(getattr(args, "skip_autoinj", False))
    autoinj_output_dir = Path(getattr(args, "autoinj_output_dir", "crates_inj"))
    if not autoinj_output_dir.is_absolute():
        autoinj_output_dir = (repo_root / autoinj_output_dir).resolve()
    else:
        autoinj_output_dir = autoinj_output_dir.resolve()
    autoinj_bin = None if skip_autoinj else _resolve_autoinj_binary(repo_root)

    if cargo_dir:
        cargo_dir_path = Path(cargo_dir)
        if not cargo_dir_path.is_absolute():
            cargo_dir_path = (repo_root / cargo_dir_path).resolve()
        else:
            cargo_dir_path = cargo_dir_path.resolve()
        if not cargo_dir_path.is_dir():
            raise RuntimeError(f"provided cargo dir is not a directory: {cargo_dir}")
        if not (cargo_dir_path / "Cargo.toml").is_file():
            raise RuntimeError(f"provided cargo dir does not contain Cargo.toml: {cargo_dir}")
        crate_dirs = [cargo_dir_path]
    else:
        crate_dirs = _find_crates(crates_dir)

    for crate_dir in crate_dirs:
        _sync_single_crate(
            repo_root=repo_root,
            crate_dir=crate_dir,
            allowed_rule_ids=allowed_rule_ids,
            rules_by_path=rules_by_path,
            operators=operators,
            operator_document=operator_document,
            mirscan_rustc=mirscan_rustc,
            strict=strict,
            autoinj_output_dir=None if skip_autoinj else autoinj_output_dir,
            autoinj_bin=autoinj_bin,
        )

    return 0
