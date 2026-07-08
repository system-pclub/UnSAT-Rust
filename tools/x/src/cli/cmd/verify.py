import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import json
import logging
import os
import shutil
import subprocess
import time
from pathlib import Path
from typing import Any

from dsl import parse_dsl
from dsl.simplifier import simplify_variables

from cli.cmd.compare import (
    _build_meta_task1_index,
    _expr_to_ext_ast,
    _find_repo_root,
    _load_json,
    _load_meta_like,
    _load_operator_entries,
    _target_callsite_key,
)
from cli.cmd.llvmir import ensure_linked_llvm_ir_file
from cli.cmd.sync import ensure_crate_metadata_file, ensure_injected_crate
from cli.cmd.verify_poc import (
    ensure_cargo_feature,
    generate_safe_testcase,
    inject_testcase_at_callsite,
    testcase_injection,
    write_certainty_chain_json,
)

logger = logging.getLogger(__name__)


def _utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def _atomic_write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    tmp.replace(path)


def _clean_dir(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True, exist_ok=True)


def _pid_is_alive(pid: int) -> bool:
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


@contextmanager
def _pid_lock(lock_path: Path):
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    pid = os.getpid()
    while True:
        try:
            fd = os.open(str(lock_path), os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                f.write(f"{pid}\n")
            break
        except FileExistsError:
            try:
                raw = lock_path.read_text(encoding="utf-8").strip()
                existing_pid = int(raw.splitlines()[0])
            except Exception:
                existing_pid = -1
            if _pid_is_alive(existing_pid):
                raise RuntimeError(
                    f"another verify matrix run appears active with pid {existing_pid}; "
                    f"lock={lock_path}"
                )
            logger.warning("[verify:matrix] removing stale lock: %s", lock_path)
            lock_path.unlink(missing_ok=True)
    try:
        yield
    finally:
        try:
            if lock_path.read_text(encoding="utf-8").strip().splitlines()[0] == str(pid):
                lock_path.unlink(missing_ok=True)
        except Exception:
            pass


def _resolve_path(repo_root: Path, path_text: str | None, default: str) -> Path:
    path = Path(path_text or default)
    if not path.is_absolute():
        return (repo_root / path).resolve()
    return path.resolve()


def _callsite_id_from_target(raw_target: dict[str, object], index: int) -> str:
    callsite = raw_target.get("callsite")
    if isinstance(callsite, dict):
        callsite_id = callsite.get("id")
        if isinstance(callsite_id, str) and callsite_id:
            return callsite_id
    return str(index)


def _callsite_marker_id_from_path(raw_target: dict[str, object]) -> str | None:
    callsite = raw_target.get("callsite")
    if not isinstance(callsite, dict):
        return None
    path = callsite.get("path")
    line = callsite.get("line")
    col = callsite.get("col")
    if not isinstance(path, str) or not isinstance(line, int) or not isinstance(col, int):
        return None
    stem = path.replace("/", "-").replace(".", "-").replace("_", "_")
    return f"{stem}-{line}-{col}"


def _llvm_ir_contains_callsite_marker(ll_path: Path, callsite_id: str) -> bool:
    try:
        text = ll_path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return False
    return f'c"{callsite_id}\\00"' in text or f'c"{callsite_id}"' in text


def _resolve_callsite_marker_for_ir(
    *, ll_path: Path, target: dict[str, object] | None, callsite_id: str
) -> str:
    if _llvm_ir_contains_callsite_marker(ll_path, callsite_id):
        return callsite_id
    if target is None:
        return callsite_id
    marker_id = _callsite_marker_id_from_path(target)
    if marker_id and marker_id != callsite_id and _llvm_ir_contains_callsite_marker(ll_path, marker_id):
        logger.info(
            "[verify] using LLVM marker id %s for report callsite id %s",
            marker_id,
            callsite_id,
        )
        return marker_id
    return callsite_id


def _find_target(
    targets: list[object],
    requested_callsite_id: str,
) -> tuple[dict[str, object] | None, str]:
    for index, raw_target in enumerate(targets, start=1):
        if not isinstance(raw_target, dict):
            continue
        callsite_id = _callsite_id_from_target(raw_target, index)
        callsite_key = _target_callsite_key(raw_target, index)
        if requested_callsite_id in {callsite_id, callsite_key, str(index)}:
            return raw_target, callsite_id
    return None, requested_callsite_id


def _load_rule_task1(
    *,
    repo_root: Path,
    meta_path: Path,
    rule_dir: Path,
    callsite_id: str,
    callsite_key: str | None,
    rule_id: str,
) -> str:
    crate_human_dir = repo_root / "human" / meta_path.stem
    crate_human_file = repo_root / "human" / meta_path.name
    nested_crate_human_dir = rule_dir / meta_path.stem

    if nested_crate_human_dir.is_dir():
        rule_dir = nested_crate_human_dir.resolve()
    elif rule_dir == (repo_root / "human").resolve():
        if crate_human_dir.is_dir():
            rule_dir = crate_human_dir.resolve()
        elif crate_human_file.is_file():
            rule_dir = crate_human_file.resolve()
        else:
            raise RuntimeError(
                f"could not find human rules for crate {meta_path.stem}: "
                f"checked {crate_human_dir} and {crate_human_file}"
            )

    if not rule_dir.exists():
        if crate_human_dir.is_dir():
            rule_dir = crate_human_dir.resolve()
        elif crate_human_file.is_file():
            rule_dir = crate_human_file.resolve()
        else:
            raise RuntimeError(
                f"rule dir does not exist: {rule_dir} "
                f"(also checked {crate_human_dir} and {crate_human_file})"
            )

    rule_meta = _load_meta_like(rule_dir)
    by_callsite_rule, by_rule = _build_meta_task1_index(rule_meta)

    for key in [callsite_id, callsite_key]:
        if not key:
            continue
        task1 = by_callsite_rule.get((key, rule_id))
        if isinstance(task1, str) and task1.strip():
            return task1

    task1 = by_rule.get(rule_id)
    if isinstance(task1, str) and task1.strip():
        return task1

    searched = f"{callsite_id}/{rule_id}"
    if callsite_key and callsite_key != callsite_id:
        searched += f" or {callsite_key}/{rule_id}"
    raise RuntimeError(f"could not find non-placeholder task1 for {searched} in {rule_dir}")


def _load_rule_dsl(rule_dsl_path: Path, rule_id: str) -> str:
    if not rule_dsl_path.is_file():
        raise RuntimeError(f"rule DSL file does not exist: {rule_dsl_path}")

    data = _load_json(rule_dsl_path)
    if isinstance(data, dict) and isinstance(data.get("rules"), dict):
        data = data["rules"]

    if not isinstance(data, dict):
        raise RuntimeError(f"rule DSL file must be an object: {rule_dsl_path}")

    entry = data.get(rule_id)
    if isinstance(entry, str) and entry.strip():
        return entry
    if isinstance(entry, dict):
        dsl = entry.get("dsl")
        if isinstance(dsl, str) and dsl.strip():
            return dsl

    raise RuntimeError(f"could not find DSL for {rule_id} in {rule_dsl_path}")


def _load_rule_ids(rule_dsl_path: Path) -> list[str]:
    if not rule_dsl_path.is_file():
        raise RuntimeError(f"rule DSL file does not exist: {rule_dsl_path}")
    data = _load_json(rule_dsl_path)
    rules = data.get("rules", data) if isinstance(data, dict) else {}
    if not isinstance(rules, dict):
        raise RuntimeError(f"rule DSL file must contain an object: {rule_dsl_path}")
    return sorted(str(key) for key in rules.keys())


def _parse_rule_path(path_with_line: object) -> tuple[str, int] | None:
    if not isinstance(path_with_line, str):
        return None
    path_part, sep, line_part = path_with_line.rpartition(":")
    if not sep or not path_part:
        return None
    try:
        line = int(line_part)
    except ValueError:
        return None
    return path_part.replace("\\", "/"), line


def _load_rule_ids_by_callee(rule_dsl_path: Path) -> dict[tuple[str, int], list[str]]:
    if not rule_dsl_path.is_file():
        raise RuntimeError(f"rule DSL file does not exist: {rule_dsl_path}")
    data = _load_json(rule_dsl_path)
    rules = data.get("rules", data) if isinstance(data, dict) else {}
    if not isinstance(rules, dict):
        raise RuntimeError(f"rule DSL file must contain an object: {rule_dsl_path}")

    grouped: dict[tuple[str, int], list[str]] = {}
    for rule_id, raw_entry in rules.items():
        if not isinstance(rule_id, str):
            continue
        path_with_line: object = None
        if isinstance(raw_entry, dict):
            path_with_line = raw_entry.get("path")
        parsed = _parse_rule_path(path_with_line)
        if parsed is None:
            continue
        grouped.setdefault(parsed, []).append(rule_id)

    return {key: sorted(values) for key, values in grouped.items()}


def _task1_to_ext_ast_json(task1: str, operators: list[dict[str, object]]) -> str:
    ast = parse_dsl(task1, operators, allow_unknown_operators=True)
    simplified = simplify_variables(ast)
    return json.dumps(
        {
            "simplified": _expr_to_ext_ast(simplified.simplified),
            "original": _expr_to_ext_ast(ast),
        },
        separators=(",", ":"),
        ensure_ascii=False,
    )


def _compose_status_from_output(
    returncode: int | None, text: str, timed_out: bool
) -> str:
    if timed_out:
        return "timeout"
    if returncode is None:
        return "unknown"
    if returncode != 0:
        return "klee-error"
    lowered = text.lower()
    if "sat(constraints and not resolved constraint): unsat" in lowered:
        return "verified"
    if "sat(constraints and not resolved constraint): sat" in lowered:
        return "violation"
    if "query solved" in lowered:
        return "reached"
    if "[ext.dsl] at callsite" in text:
        return "reached"
    return "ok"


def _control_chain_has_certain_symbol(chain_path: Path) -> bool:
    if not chain_path.is_file():
        return False
    try:
        raw = json.loads(chain_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return False
    if not isinstance(raw, dict):
        return False
    symbols = raw.get("symbols")
    if not isinstance(symbols, list):
        return False
    return any(
        isinstance(symbol, dict) and symbol.get("certainty") == "certain_symbol"
        for symbol in symbols
    )


def _merge_existing_matrix_rows(result_path: Path) -> dict[tuple[int, str], dict[str, Any]]:
    if not result_path.is_file():
        return {}
    raw = _load_json(result_path)
    results = raw.get("results", []) if isinstance(raw, dict) else []
    merged: dict[tuple[int, str], dict[str, Any]] = {}
    if isinstance(results, list):
        for item in results:
            if not isinstance(item, dict):
                continue
            target_index = item.get("target_index")
            rule = item.get("rule")
            if isinstance(target_index, int) and isinstance(rule, str):
                merged[(target_index, rule)] = item
    return merged


def _run_klee_compose_verify(
    *,
    ll_path: Path,
    callsite_id: str,
    ast_json: str,
    report_json: Path | None,
    klee_bin: str,
    compose_loop_bound: int,
    output_dir: Path,
) -> tuple[int, str]:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        klee_bin,
        f"--output-dir={output_dir}",
        f"--compose-verify-chain-json={output_dir.parent / 'klee-control-chains.json'}",
        f"--ext.callsite={callsite_id}",
        f"--ext.dsl={ast_json}",
        "--compose-verify",
        f"--compose-verify-symbolic-loop-bound={compose_loop_bound}",
    ]
    if report_json is not None:
        cmd.append(f"--report-json={report_json}")
    cmd.append(str(ll_path))
    print(f"[verify] running: {' '.join(cmd)}")
    result = subprocess.run(cmd, check=False, capture_output=True, text=True)
    log_path = output_dir.parent / "klee-compose.log"
    log_path.write_text(
        "command: " + repr(cmd) + "\n"
        f"returncode: {result.returncode}\n\n"
        "[stdout]\n" + result.stdout + "\n"
        "[stderr]\n" + result.stderr,
        encoding="utf-8",
    )
    if result.stdout:
        print(result.stdout, end="")
    if result.stderr:
        print(result.stderr, end="")
    combined = result.stdout + result.stderr
    if "SAT(constraints AND NOT resolved constraint): unsat" in combined:
        return result.returncode, "verified"
    if (
        "SAT(constraints AND NOT resolved constraint): sat" in combined
        or "query solved" in combined
        or "query deferred for compose" in combined
    ):
        return result.returncode, "candidate"
    return result.returncode, "unknown"


def _run_klee_compose_rerun(
    *, ll_path: Path, callsite_id: str, ast_json: str, klee_bin: str,
    output_dir: Path, entry_function: str, log_path: Path | None = None,
) -> int:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        klee_bin,
        f"--output-dir={output_dir}",
        f"--entry-point={entry_function}",
        f"--ext.callsite={callsite_id}",
        f"--ext.dsl={ast_json}",
        "--compose-rerun",
        str(ll_path),
    ]
    logger.info("[verify:rerun] running: %s", " ".join(cmd))
    result = subprocess.run(cmd, check=False, capture_output=True, text=True)
    reached_callsite = f"[ext.dsl] at callsite '{callsite_id}'" in (
        result.stdout + result.stderr
    )
    returncode = result.returncode
    if returncode == 0 and not reached_callsite:
        returncode = 2
    if log_path is not None:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        log_path.write_text(
            "command: " + repr(cmd) + "\n"
            f"returncode: {returncode}\n"
            f"target_callsite_reached: {str(reached_callsite).lower()}\n\n"
            "[stdout]\n" + result.stdout + "\n"
            "[stderr]\n" + result.stderr,
            encoding="utf-8",
        )
    if result.stdout:
        print(result.stdout, end="")
    if result.stderr:
        print(result.stderr, end="")
    return returncode


def _matrix_callsite_rows(
    *, targets: list[object], ll_path: Path, requested_callsite: str | None
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    try:
        llvm_text = ll_path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        llvm_text = ""

    for idx, raw_target in enumerate(targets, start=1):
        if not isinstance(raw_target, dict):
            continue
        callsite_id = _callsite_id_from_target(raw_target, idx)
        callsite_key = _target_callsite_key(raw_target, idx)
        if requested_callsite and requested_callsite not in {callsite_id, callsite_key, str(idx)}:
            continue

        callsite = raw_target.get("callsite") if isinstance(raw_target.get("callsite"), dict) else {}
        caller = raw_target.get("caller") if isinstance(raw_target.get("caller"), dict) else {}
        callee = raw_target.get("callee")
        if not isinstance(callee, dict):
            callee = (
                raw_target.get("unsafe_callee")
                if isinstance(raw_target.get("unsafe_callee"), dict)
                else {}
            )
        llvm_callsite_id = callsite_id
        if not (f'c"{llvm_callsite_id}\\00"' in llvm_text or f'c"{llvm_callsite_id}"' in llvm_text):
            marker_id = _callsite_marker_id_from_path(raw_target)
            if marker_id and (
                f'c"{marker_id}\\00"' in llvm_text or f'c"{marker_id}"' in llvm_text
            ):
                llvm_callsite_id = marker_id

        rows.append(
            {
                "target_index": idx,
                "target": raw_target,
                "callsite_id": callsite_id,
                "llvm_callsite_id": llvm_callsite_id,
                "present_in_llvm_ir": (
                    f'c"{llvm_callsite_id}\\00"' in llvm_text
                    or f'c"{llvm_callsite_id}"' in llvm_text
                ),
                "path": callsite.get("path"),
                "line": callsite.get("line"),
                "col": callsite.get("col"),
                "caller": caller.get("name"),
                "unsafe_callee": callee.get("name"),
                "unsafe_callee_path": callee.get("path"),
                "unsafe_callee_line_start": callee.get("line_start"),
            }
        )
    return rows


def _assign_matrix_rules_to_callsites(
    *,
    callsites: list[dict[str, Any]],
    rule_dsl_path: Path,
    requested_rule: str | None,
) -> list[dict[str, Any]]:
    if requested_rule:
        _load_rule_dsl(rule_dsl_path, requested_rule)
        for callsite in callsites:
            callsite["rules"] = [requested_rule]
        return callsites

    rules_by_callee = _load_rule_ids_by_callee(rule_dsl_path)
    for callsite in callsites:
        callee_path = callsite.get("unsafe_callee_path")
        callee_line = callsite.get("unsafe_callee_line_start")
        rules: list[str] = []
        if isinstance(callee_path, str) and isinstance(callee_line, int):
            rules = rules_by_callee.get((callee_path.replace("\\", "/"), callee_line), [])
        callsite["rules"] = rules
    return callsites


def _row_needs_llm_testcase(row: dict[str, Any]) -> bool:
    # This is the important semantic split for the full pipeline:
    # compose found SAT for constraints AND NOT(safety rule), so a fully safe
    # concrete PoC may exist and is worth asking the LLM to construct.
    return row.get("status") == "violation"


def _matrix_summary(results: list[dict[str, Any]]) -> dict[str, Any]:
    needs = [row for row in results if _row_needs_llm_testcase(row)]
    direct_unsat = [row for row in results if row.get("status") == "verified"]
    return {
        "buggy_dsl_sat_needs_llm_testcase": len(needs),
        "direct_unsat_no_bug": len(direct_unsat),
        "other_or_unclassified": len(results) - len(needs) - len(direct_unsat),
        "needs_llm_testcase": [
            {
                "target_index": row.get("target_index"),
                "callsite": row.get("callsite"),
                "llvm_callsite": row.get("llvm_callsite"),
                "rule": row.get("rule"),
                "log": row.get("log"),
            }
            for row in needs
        ],
    }


def _run_llm_testcase_pipeline(
    *,
    args: argparse.Namespace,
    repo_root: Path,
    cargo_dir: Path,
    injected_dir: Path,
    rule_dsl_path: Path,
    target: dict[str, object],
    callsite_id: str,
    rule_id: str,
    ast_json: str,
    artifact_dir: Path,
    compose_output: Path,
) -> int:
    chain = write_certainty_chain_json(
        info_path=compose_output / "info",
        output_path=artifact_dir / "certainty-chain.json",
        callsite=callsite_id,
        rule=rule_id,
    )
    if args.skip_llm_testcase:
        logger.info("[verify] --skip-llm-testcase set; stopping before LLM testcase generation")
        return 0

    rule_doc = _load_json(rule_dsl_path)
    rules = rule_doc.get("rules", rule_doc) if isinstance(rule_doc, dict) else {}
    rule = rules.get(rule_id) if isinstance(rules, dict) else None
    if not isinstance(rule, dict):
        raise RuntimeError(f"testcase generation requires metadata for {rule_id}")

    injection = testcase_injection(callsite_id, rule_id)
    logger.info(
        "[verify:testcase] key=%s feature=%s function=%s",
        injection.key,
        injection.feature,
        injection.function,
    )
    testcase = generate_safe_testcase(
        crate_dir=injected_dir,
        target=target,
        rule=rule,
        chain=chain,
        model=args.model,
        artifacts_dir=artifact_dir,
        injection=injection,
    )
    ensure_cargo_feature(injected_dir, injection.feature)
    injection = inject_testcase_at_callsite(
        crate_dir=injected_dir,
        target=target,
        testcase=testcase,
        injection=injection,
    )
    logger.info(
        "[verify:testcase] source=%s:%d state=%s",
        injection.source_path,
        injection.line,
        injected_dir / ".unsat-test-injections.json",
    )

    pipeline_state = {
        "schema_version": 1,
        "callsite": callsite_id,
        "rule": rule_id,
        "feature": injection.feature,
        "entry_function": injection.function,
        "source_path": injection.source_path,
        "source_line": injection.line,
        "klee_compose_dir": str(compose_output),
        "klee_compose_log": str(artifact_dir / "klee-compose.log"),
        "skip_rerun": bool(args.skip_rerun),
    }
    (artifact_dir / "testcase-pipeline.json").write_text(
        json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
    )

    if args.skip_rerun:
        logger.info("[verify:testcase] --skip-rerun set; stopping after testcase injection")
        return 0

    rerun_ir_dir = artifact_dir / "rerun-ir"
    _clean_dir(rerun_ir_dir)
    rerun_ll = ensure_linked_llvm_ir_file(
        cargo_dir=injected_dir,
        output_dir=rerun_ir_dir,
        rustc=args.rustc,
        test=False,
        build_std=True,
        panic_abort=True,
        force=True,
        features=[injection.feature],
        build_log_path=artifact_dir / "testcase-build.log",
    )
    pipeline_state.update(
        {
            "llvm_ir": str(rerun_ll),
            "build_log": str(artifact_dir / "testcase-build.log"),
            "klee_rerun_dir": str(artifact_dir / "klee-rerun"),
            "klee_rerun_log": str(artifact_dir / "klee-rerun.log"),
        }
    )
    (artifact_dir / "testcase-pipeline.json").write_text(
        json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
    )
    rerun_rc = _run_klee_compose_rerun(
        ll_path=rerun_ll,
        callsite_id=callsite_id,
        ast_json=ast_json,
        klee_bin=args.klee_bin,
        output_dir=artifact_dir / "klee-rerun",
        entry_function=injection.function,
        log_path=artifact_dir / "klee-rerun.log",
    )
    pipeline_state["rerun_returncode"] = rerun_rc
    (artifact_dir / "testcase-pipeline.json").write_text(
        json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
    )
    return rerun_rc


def _run_verify_matrix(
    *,
    args: argparse.Namespace,
    repo_root: Path,
    cargo_dir: Path,
    injected_dir: Path,
    meta_path: Path,
    ll_path: Path,
    targets: list[object],
    rule_dsl_path: Path,
    report_json: Path | None,
) -> int:
    callsites = _matrix_callsite_rows(
        targets=targets,
        ll_path=ll_path,
        requested_callsite=args.callsite,
    )
    if not callsites:
        raise RuntimeError(f"no callsites matched {args.callsite!r}")
    callsites = _assign_matrix_rules_to_callsites(
        callsites=callsites,
        rule_dsl_path=rule_dsl_path,
        requested_rule=args.rule,
    )
    total = sum(len(row.get("rules", [])) for row in callsites)
    if total == 0:
        suffix = f" for callsite {args.callsite!r}" if args.callsite else ""
        raise RuntimeError(
            "no rules matched callsite callee path+line"
            f"{suffix} in {rule_dsl_path}"
        )

    result_path = _resolve_path(
        repo_root,
        args.results_json,
        f".local/verify/{cargo_dir.name}/full-matrix-results.json",
    )
    lock_cm = _pid_lock(result_path.with_suffix(result_path.suffix + ".lock"))
    lock_cm.__enter__()
    try:
        return _run_verify_matrix_locked(
            args=args,
            repo_root=repo_root,
            cargo_dir=cargo_dir,
            injected_dir=injected_dir,
            meta_path=meta_path,
            ll_path=ll_path,
            targets=targets,
            rule_dsl_path=rule_dsl_path,
            report_json=report_json,
            result_path=result_path,
            callsites=callsites,
        )
    finally:
        lock_cm.__exit__(None, None, None)


def _run_verify_matrix_locked(
    *,
    args: argparse.Namespace,
    repo_root: Path,
    cargo_dir: Path,
    injected_dir: Path,
    meta_path: Path,
    ll_path: Path,
    targets: list[object],
    rule_dsl_path: Path,
    report_json: Path | None,
    result_path: Path,
    callsites: list[dict[str, Any]],
) -> int:
    logs_dir = _resolve_path(
        repo_root,
        args.logs_dir,
        f".local/verify/{cargo_dir.name}/full-matrix-logs",
    )
    logs_dir.mkdir(parents=True, exist_ok=True)

    planned_keys = {
        (int(row["target_index"]), str(rule))
        for row in callsites
        for rule in row.get("rules", [])
        if isinstance(row.get("target_index"), int) and isinstance(rule, str)
    }
    existing = _merge_existing_matrix_rows(result_path) if args.resume else {}
    existing = {key: value for key, value in existing.items() if key in planned_keys}
    results: list[dict[str, Any]] = list(existing.values())
    counts: dict[str, int] = {}
    for item in results:
        status = item.get("status")
        if isinstance(status, str):
            counts[status] = counts.get(status, 0) + 1

    total = len(planned_keys)
    all_rules = sorted({rule for _, rule in planned_keys})
    rules_by_callsite = {
        str(row["callsite_id"]): list(row.get("rules", []))
        for row in callsites
        if isinstance(row.get("callsite_id"), str)
    }
    state: dict[str, Any] = {
        "schema_version": 1,
        "started_at": _utc_now(),
        "updated_at": _utc_now(),
        "crate": str(cargo_dir),
        "injected_crate": str(injected_dir),
        "metadata": str(meta_path),
        "llvm_ir": str(ll_path),
        "rule_dsl": str(rule_dsl_path),
        "report_json": str(report_json) if report_json is not None else None,
        "klee_bin": args.klee_bin,
        "compose_loop_bound": args.compose_loop_bound,
        "timeout_sec": args.timeout_sec,
        "test": args.test,
        "total": total,
        "completed": len(results),
        "counts": dict(sorted(counts.items())),
        "callsites": [
            {key: value for key, value in row.items() if key != "target"}
            for row in callsites
        ],
        "rules": all_rules,
        "rules_by_callsite": rules_by_callsite,
        "results": sorted(
            results, key=lambda x: (x.get("target_index", 0), x.get("rule", ""))
        ),
    }
    _atomic_write_json(result_path, state)
    print(f"[verify:matrix] callsites={len(callsites)} matched-rules={len(all_rules)} total={total}")
    print(f"[verify:matrix] result-json={result_path}")
    print(f"[verify:matrix] logs-dir={logs_dir}")

    operators = _load_operator_entries(repo_root)
    ast_cache: dict[str, str] = {}
    completed = len(results)
    target_by_index = {
        int(row["target_index"]): row["target"]
        for row in callsites
        if isinstance(row.get("target_index"), int) and isinstance(row.get("target"), dict)
    }

    for callsite in callsites:
        target_index = int(callsite["target_index"])
        callsite_id = str(callsite["callsite_id"])
        llvm_callsite_id = str(callsite.get("llvm_callsite_id") or callsite_id)
        for rule in callsite.get("rules", []):
            if (target_index, rule) in existing:
                continue
            if not callsite.get("present_in_llvm_ir"):
                row = {
                    "target_index": target_index,
                    "callsite": callsite_id,
                    "llvm_callsite": llvm_callsite_id,
                    "path": callsite.get("path"),
                    "line": callsite.get("line"),
                    "col": callsite.get("col"),
                    "rule": rule,
                    "status": "not-in-ir",
                    "returncode": None,
                    "timed_out": False,
                    "duration_sec": 0.0,
                    "log": None,
                    "klee_output_dir": None,
                }
                results.append(row)
                completed += 1
                counts["not-in-ir"] = counts.get("not-in-ir", 0) + 1
                state.update(
                    {
                        "updated_at": _utc_now(),
                        "completed": completed,
                        "counts": dict(sorted(counts.items())),
                        "results": sorted(
                            results,
                            key=lambda x: (x.get("target_index", 0), x.get("rule", "")),
                        ),
                    }
                )
                _atomic_write_json(result_path, state)
                continue

            if rule not in ast_cache:
                ast_cache[rule] = _task1_to_ext_ast_json(
                    _load_rule_dsl(rule_dsl_path, rule), operators
                )
            output_dir = logs_dir / f"{target_index:03d}-{callsite_id}" / rule / "klee-out"
            log_path = logs_dir / f"{target_index:03d}-{callsite_id}" / rule / "klee.log"
            if output_dir.exists():
                shutil.rmtree(output_dir)
            output_dir.parent.mkdir(parents=True, exist_ok=True)
            cmd = [
                args.klee_bin,
                f"--output-dir={output_dir}",
                f"--compose-verify-chain-json={output_dir.parent / 'klee-control-chains.json'}",
                f"--ext.callsite={llvm_callsite_id}",
                f"--ext.dsl={ast_cache[rule]}",
                "--compose-verify",
                f"--compose-verify-symbolic-loop-bound={args.compose_loop_bound}",
            ]
            if report_json is not None:
                cmd.append(f"--report-json={report_json}")
            cmd.append(str(ll_path))

            print(
                f"[verify:matrix] {completed + 1}/{total} "
                f"target#{target_index} {callsite_id} {rule}",
                flush=True,
            )
            started = time.time()
            timed_out = False
            try:
                proc = subprocess.run(
                    cmd,
                    check=False,
                    capture_output=True,
                    text=True,
                    timeout=args.timeout_sec,
                    env={
                        **os.environ,
                        "RUST_BACKTRACE": os.environ.get("RUST_BACKTRACE", "0"),
                    },
                )
                returncode: int | None = proc.returncode
                combined = (proc.stdout or "") + (proc.stderr or "")
                log_text = (
                    "command: " + repr(cmd) + "\n"
                    f"returncode: {returncode}\n"
                    "timeout: false\n"
                    f"duration_sec: {time.time() - started:.3f}\n\n"
                    "[stdout]\n" + (proc.stdout or "") + "\n"
                    "[stderr]\n" + (proc.stderr or "")
                )
            except subprocess.TimeoutExpired as exc:
                timed_out = True
                returncode = None
                stdout = (
                    exc.stdout.decode("utf-8", "replace")
                    if isinstance(exc.stdout, bytes)
                    else (exc.stdout or "")
                )
                stderr = (
                    exc.stderr.decode("utf-8", "replace")
                    if isinstance(exc.stderr, bytes)
                    else (exc.stderr or "")
                )
                combined = stdout + stderr
                log_text = (
                    "command: " + repr(cmd) + "\n"
                    "returncode: null\n"
                    "timeout: true\n"
                    f"duration_sec: {time.time() - started:.3f}\n\n"
                    "[stdout]\n" + stdout + "\n"
                    "[stderr]\n" + stderr
                )

            log_path.write_text(log_text, encoding="utf-8")
            status = _compose_status_from_output(returncode, combined, timed_out)
            chain_path = output_dir.parent / "klee-control-chains.json"
            has_certain_symbol = _control_chain_has_certain_symbol(chain_path)
            if status == "violation" and not has_certain_symbol:
                status = "low-confidence-sat"
            row = {
                "target_index": target_index,
                "callsite": callsite_id,
                "llvm_callsite": llvm_callsite_id,
                "path": callsite.get("path"),
                "line": callsite.get("line"),
                "col": callsite.get("col"),
                "rule": rule,
                "status": status,
                "returncode": returncode,
                "timed_out": timed_out,
                "duration_sec": round(time.time() - started, 3),
                "log": str(log_path),
                "klee_output_dir": str(output_dir),
                "certainty_chain": str(chain_path),
                "has_certain_symbol": has_certain_symbol,
            }
            results.append(row)
            completed += 1
            counts[status] = counts.get(status, 0) + 1
            state.update(
                {
                    "updated_at": _utc_now(),
                    "completed": completed,
                    "counts": dict(sorted(counts.items())),
                    "results": sorted(
                        results,
                        key=lambda x: (x.get("target_index", 0), x.get("rule", "")),
                    ),
                }
            )
            _atomic_write_json(result_path, state)

    state["finished_at"] = _utc_now()
    state["updated_at"] = state["finished_at"]
    state["summary"] = _matrix_summary(results)
    _atomic_write_json(result_path, state)
    summary = state["summary"]
    print(
        "[verify:matrix] klee done: "
        f"buggy_dsl_sat_needs_llm_testcase={summary['buggy_dsl_sat_needs_llm_testcase']} "
        f"direct_unsat_no_bug={summary['direct_unsat_no_bug']} "
        f"other_or_unclassified={summary['other_or_unclassified']}"
    )

    if args.skip_llm_testcase:
        print(f"[verify:matrix] --skip-llm-testcase set; done: {result_path}")
        return 0

    llm_failures = 0
    for row in results:
        if not _row_needs_llm_testcase(row):
            continue
        target_index = row.get("target_index")
        target = target_by_index.get(target_index) if isinstance(target_index, int) else None
        if not isinstance(target, dict):
            row["llm_testcase_status"] = "error"
            row["llm_testcase_error"] = f"missing target metadata for target_index={target_index}"
            llm_failures += 1
            continue
        rule_id = row.get("rule")
        callsite_id = row.get("llvm_callsite") or row.get("callsite")
        klee_output_dir = row.get("klee_output_dir")
        if not isinstance(rule_id, str) or not isinstance(callsite_id, str) or not isinstance(klee_output_dir, str):
            row["llm_testcase_status"] = "error"
            row["llm_testcase_error"] = "missing rule/callsite/klee output metadata"
            llm_failures += 1
            continue
        if rule_id not in ast_cache:
            ast_cache[rule_id] = _task1_to_ext_ast_json(
                _load_rule_dsl(rule_dsl_path, rule_id), operators
            )
        artifact_dir = Path(klee_output_dir).parent
        print(
            f"[verify:llm] target#{target_index} {callsite_id} {rule_id} "
            f"artifact-dir={artifact_dir}",
            flush=True,
        )
        try:
            rc = _run_llm_testcase_pipeline(
                args=args,
                repo_root=repo_root,
                cargo_dir=cargo_dir,
                injected_dir=injected_dir,
                rule_dsl_path=rule_dsl_path,
                target=target,
                callsite_id=callsite_id,
                rule_id=rule_id,
                ast_json=ast_cache[rule_id],
                artifact_dir=artifact_dir,
                compose_output=artifact_dir / "klee-out",
            )
            row["llm_testcase_status"] = "ok" if rc == 0 else "rerun-error"
            row["llm_testcase_returncode"] = rc
            if rc != 0:
                llm_failures += 1
        except Exception as exc:
            row["llm_testcase_status"] = "error"
            row["llm_testcase_error"] = str(exc)
            llm_failures += 1
        state.update(
            {
                "updated_at": _utc_now(),
                "results": sorted(
                    results,
                    key=lambda x: (x.get("target_index", 0), x.get("rule", "")),
                ),
            }
        )
        _atomic_write_json(result_path, state)

    state["llm_testcase_finished_at"] = _utc_now()
    state["llm_testcase_failures"] = llm_failures
    state["updated_at"] = state["llm_testcase_finished_at"]
    _atomic_write_json(result_path, state)
    print(f"[verify:matrix] done: {result_path}")
    if llm_failures:
        return 1
    return 0


def run(args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()
    if args.compose_loop_bound < 0:
        raise RuntimeError("--compose-loop-bound must be non-negative")
    if args.timeout_sec <= 0:
        raise RuntimeError("--timeout-sec must be positive")

    cargo_dir = Path(args.cargo_dir)
    if not cargo_dir.is_absolute():
        cargo_dir = (repo_root / cargo_dir).resolve()
    else:
        cargo_dir = cargo_dir.resolve()

    if not cargo_dir.is_dir() or not (cargo_dir / "Cargo.toml").is_file():
        raise RuntimeError(f"invalid crate directory: {cargo_dir}")
    if cargo_dir.parent == (repo_root / "crates_inj").resolve():
        raise RuntimeError(
            "verify input must be the original crate (for example crates/<name>), "
            "not crates_inj/<name>; autoinj recreates crates_inj"
        )

    studied_rules = _resolve_path(repo_root, args.studied_rules, "studied_rules")
    meta_path = ensure_crate_metadata_file(
        repo_root,
        cargo_dir,
        studied_rules=studied_rules,
        force=True,
    )
    injected_dir = ensure_injected_crate(repo_root, cargo_dir, meta_path)

    ir_output_dir = _resolve_path(repo_root, args.ir_output_dir, ".local/irs")
    _clean_dir(ir_output_dir)
    ll_path = ensure_linked_llvm_ir_file(
        cargo_dir=injected_dir,
        output_dir=ir_output_dir,
        rustc=args.rustc,
        test=args.test,
        build_std=True,
        panic_abort=True,
        force=True,
    )

    if args.skip_klee:
        print(f"[verify] crate={cargo_dir}")
        print(f"[verify] injected-crate={injected_dir}")
        print(f"[verify] llvm-ir={ll_path}")
        print("[verify] mirscan, autoinj, and LLVM IR generation succeeded; skipping KLEE")
        return 0

    current_meta = _load_json(meta_path)
    report = current_meta.get("report")
    if not isinstance(report, dict):
        raise RuntimeError(f"missing report object in {meta_path}")
    targets = report.get("targets")
    if not isinstance(targets, list):
        raise RuntimeError(f"missing targets in {meta_path}")
    rule_dsl_path = _resolve_path(repo_root, args.rule_dsl, "ptr_rule_dsl.json")
    report_json = _resolve_path(repo_root, args.report_json, str(injected_dir / "report.json"))
    if not report_json.is_file():
        fallback_report_json = _resolve_path(repo_root, None, str(cargo_dir / "report.json"))
        report_json = fallback_report_json if fallback_report_json.is_file() else None

    if not args.callsite or not args.rule:
        return _run_verify_matrix(
            args=args,
            repo_root=repo_root,
            cargo_dir=cargo_dir,
            injected_dir=injected_dir,
            meta_path=meta_path,
            ll_path=ll_path,
            targets=targets,
            rule_dsl_path=rule_dsl_path,
            report_json=report_json,
        )

    target, resolved_callsite_id = _find_target(targets, args.callsite)
    if target is None:
        raise RuntimeError(f"could not find callsite {args.callsite!r} in {meta_path}")
    report_callsite_id = resolved_callsite_id
    resolved_callsite_id = _resolve_callsite_marker_for_ir(
        ll_path=ll_path, target=target, callsite_id=resolved_callsite_id
    )
    callsite_key = _target_callsite_key(target, 0) if target is not None else None
    task1 = _load_rule_dsl(rule_dsl_path, args.rule)

    operators = _load_operator_entries(repo_root)
    ast_json = _task1_to_ext_ast_json(task1, operators)

    print(f"[verify] crate={cargo_dir}")
    print(f"[verify] injected-crate={injected_dir}")
    print(f"[verify] llvm-ir={ll_path}")
    if report_json is not None:
        print(f"[verify] report-json={report_json}")
    if report_callsite_id != resolved_callsite_id:
        print(f"[verify] report-callsite={report_callsite_id}")
    print(f"[verify] callsite={resolved_callsite_id} rule={args.rule}")
    print(f"[verify] rule-dsl={rule_dsl_path}")
    artifact_dir = _resolve_path(
        repo_root, args.artifacts_dir,
        f".local/verify/{cargo_dir.name}/{resolved_callsite_id}/{args.rule}",
    )
    compose_output = artifact_dir / "klee-compose"
    rc, compose_status = _run_klee_compose_verify(
        ll_path=ll_path,
        callsite_id=resolved_callsite_id,
        ast_json=ast_json,
        report_json=report_json,
        klee_bin=args.klee_bin,
        compose_loop_bound=args.compose_loop_bound,
        output_dir=compose_output,
    )
    if rc != 0:
        return rc

    if compose_status == "verified":
        pipeline_state = {
            "schema_version": 1,
            "callsite": resolved_callsite_id,
            "rule": args.rule,
            "status": "compose-verified-skip-testcase",
            "reason": "compose-verify proved constraints AND NOT(rule) unsat",
            "klee_compose_log": str(artifact_dir / "klee-compose.log"),
        }
        (artifact_dir / "testcase-pipeline.json").write_text(
            json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
        )
        logger.info(
            "[verify] compose result is unsat for NOT(rule); skipping testcase generation/rerun"
        )
        return 0

    return _run_llm_testcase_pipeline(
        args=args,
        repo_root=repo_root,
        cargo_dir=cargo_dir,
        injected_dir=injected_dir,
        rule_dsl_path=rule_dsl_path,
        target=target,
        callsite_id=resolved_callsite_id,
        rule_id=args.rule,
        ast_json=ast_json,
        artifact_dir=artifact_dir,
        compose_output=compose_output,
    )
