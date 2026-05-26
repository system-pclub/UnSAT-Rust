import argparse
import csv
import json
import os
from pathlib import Path
import shutil
import subprocess
import tomllib


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

            record = {
                "id": f"rule-{idx}",
                "line": rule_line,
                "name": (row.get("name") or "").strip(),
                "rule": (row.get("rule") or "").strip(),
            }
            grouped.setdefault(normalized_rule_path, []).append(record)

    return grouped


def _match_rules_for_target(
    target: dict[str, object],
    rules_by_path: dict[str, list[dict[str, object]]],
) -> dict[str, dict[str, str]]:
    unsafe_call = target.get("unsafe_call")
    if not isinstance(unsafe_call, dict):
        return {}

    unsafe_path = unsafe_call.get("path")
    line_start = unsafe_call.get("line_start")
    line_end = unsafe_call.get("line_end")
    if not isinstance(unsafe_path, str):
        return {}

    matched_rules: dict[str, dict[str, str]] = {}
    candidates = rules_by_path.get(unsafe_path, [])

    for candidate in candidates:
        candidate_line = candidate.get("line")
        rule_id = candidate.get("id")
        if not isinstance(candidate_line, int) or not isinstance(rule_id, str):
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
    unsafe_name = unsafe_call.get("name")
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

        normalized_rule_name = " ".join(rule_name.lower().split())
        if fn_name in normalized_rule_name:
            matched_rules[rule_id] = {
                "task1": "<placeholder>",
                "task2": "<placeholder>",
                "task3": "<placeholder>",
            }

    return matched_rules


def _transform_report(
    report: dict[str, object],
    rules_by_path: dict[str, list[dict[str, object]]],
) -> dict[str, object]:
    report_targets = report.get("targets")
    if not isinstance(report_targets, list):
        raise RuntimeError("mirscan report is missing required targets array")

    targets_input = report_targets

    targets: list[dict[str, object]] = []
    for raw_target in targets_input:
        if not isinstance(raw_target, dict):
            continue

        target = dict(raw_target)
        target["rules"] = _match_rules_for_target(target, rules_by_path)
        targets.append(target)

    return {"targets": targets}


def run(args: argparse.Namespace) -> int:
    _ = args
    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"
    rules_by_path = _load_rules_by_path(repo_root)

    mirscan_rustc = _resolve_mirscan_rustc(repo_root)
    crate_dirs = _find_crates(crates_dir)

    for crate_dir in crate_dirs:
        crate_name = crate_dir.name or "unknown"
        crate_dir_relative = crate_dir.relative_to(repo_root).as_posix()

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

        transformed_report = _transform_report(report, rules_by_path)
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

        out_path = crates_dir / f"{crate_name}.json"
        out_path.write_text(json.dumps(out, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    return 0
