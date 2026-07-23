import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import json
import logging
import os
import re
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
    symbolize_testcase_constants,
    target_call_arg_source_map,
    testcase_injection,
    write_certainty_chain_json,
)

logger = logging.getLogger(__name__)

RAW_PTR_DEREF_RULE = "__raw_ptr_deref__"
RAW_PTR_DEREF_CALLEE = "core::ptr::__raw_ptr_deref__"
REACHABILITY_ONLY_RULES = {"rule-192", "ruklee-unreachable-unchecked"}


def _klee_no_stats_args() -> list[str]:
    # RuKLEE verification uses KLEE stdout/stderr plus ktest artifacts, not the
    # coverage/statistics databases. Compose/snapshot paths can leave transient
    # states at unusual instruction boundaries; disabling stats keeps those
    # bookkeeping paths from masking an already-solved SAT/UNSAT result.
    return ["--output-stats=false", "--output-istats=false"]


def _utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def _atomic_write_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    tmp.replace(path)


def _safe_result_name(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9_.-]+", "-", value).strip("-") or "unknown"


def _pair_result_filename(callsite_id: str, rule_id: str) -> str:
    return f"{_safe_result_name(callsite_id)}__{_safe_result_name(rule_id)}.json"


def _pair_result_path_for_matrix(result_path: Path, callsite_id: str, rule_id: str) -> Path:
    return _pair_artifact_dir(result_path.parent, callsite_id, rule_id) / "result.json"


def _pair_result_path_for_artifacts(artifact_dir: Path, callsite_id: str, rule_id: str) -> Path:
    if artifact_dir.name == rule_id and artifact_dir.parent.name == callsite_id:
        return artifact_dir / "result.json"
    return artifact_dir / "result.json"


def _pair_artifact_dir(root: Path, callsite_id: str, rule_id: str) -> Path:
    return root / _safe_result_name(callsite_id) / _safe_result_name(rule_id)


def _legacy_matrix_crate_slug(path: Path) -> str | None:
    match = re.match(r"(.+)-full-matrix-results\.json$", path.name)
    if match:
        return match.group(1)
    match = re.match(r"(.+)-full-matrix-logs$", path.name)
    if match:
        return match.group(1)
    return None


def _matrix_result_path(repo_root: Path, args: argparse.Namespace, cargo_dir: Path) -> Path:
    result_path = _resolve_path(
        repo_root,
        args.results_json,
        f".local/verify/{cargo_dir.name}/full-matrix-results.json",
    )
    if args.results_json:
        slug = _legacy_matrix_crate_slug(result_path)
        if slug:
            return result_path.parent / slug / "full-matrix-results.json"
    return result_path


def _matrix_artifact_root(
    repo_root: Path, args: argparse.Namespace, cargo_dir: Path, result_path: Path
) -> Path:
    if args.logs_dir:
        logs_dir = _resolve_path(repo_root, args.logs_dir, "")
        slug = _legacy_matrix_crate_slug(logs_dir)
        if slug:
            return logs_dir.parent / slug
        return logs_dir
    return result_path.parent


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


def _callsite_location_key(raw_target: dict[str, object]) -> tuple[object, object, object] | None:
    callsite = raw_target.get("callsite")
    if not isinstance(callsite, dict):
        return None
    path = callsite.get("path")
    line = callsite.get("line")
    col = callsite.get("col")
    if path is None or line is None or col is None:
        return None
    return (path, line, col)


def _callee_key(raw_target: dict[str, object]) -> tuple[object, object, object] | None:
    callee = raw_target.get("callee")
    if not isinstance(callee, dict):
        return None
    name = callee.get("name")
    path = callee.get("path")
    line = callee.get("line_start")
    if name is None and path is None and line is None:
        return None
    return (name, path, line)


def _target_suffix_specificity(raw_target: dict[str, object], base_callsite_id: str) -> tuple[int, int, str]:
    """Rank same-location callsite aliases by how likely they name the real unsafe marker.

    MIR/autoinj can produce both a source-location marker (for example at an
    enclosing call expression) and a more specific marker with the unsafe API
    appended.  KLEE's compose target should use the more specific marker because
    it is the one adjacent to the unsafe API argument binding/result.
    """

    callsite_id = _callsite_id_from_target(raw_target, 0)
    suffix = callsite_id[len(base_callsite_id) + 1 :] if callsite_id.startswith(base_callsite_id + "-") else ""
    callee = raw_target.get("callee")
    callee_name = callee.get("name") if isinstance(callee, dict) else None
    callee_leaf = str(callee_name or "").rsplit("::", 1)[-1].lower()
    suffix_lower = suffix.lower()
    unsafe_name_match = 1 if callee_leaf and callee_leaf in suffix_lower else 0
    unsafe_word_match = 1 if any(word in suffix_lower for word in ("unchecked", "raw_ptr", "deref")) else 0
    return (unsafe_name_match, unsafe_word_match, callsite_id)


def _prefer_specific_callsite_alias(
    targets: list[object],
    *,
    requested_callsite_id: str,
    matched_target: dict[str, object],
    matched_callsite_id: str,
) -> tuple[dict[str, object], str]:
    # Only rewrite the common "base id requested" case.  If the caller already
    # requested a suffixed/alternate id exactly, keep that exact target.
    if requested_callsite_id != matched_callsite_id:
        return matched_target, matched_callsite_id
    if isinstance(matched_target.get("unsafe_callsite"), dict):
        return matched_target, matched_callsite_id
    location_key = _callsite_location_key(matched_target)
    callee_key = _callee_key(matched_target)
    if location_key is None or callee_key is None:
        return matched_target, matched_callsite_id

    candidates: list[dict[str, object]] = []
    for raw_target in targets:
        if not isinstance(raw_target, dict) or raw_target is matched_target:
            continue
        candidate_id = _callsite_id_from_target(raw_target, 0)
        if not candidate_id.startswith(matched_callsite_id + "-"):
            continue
        if _callsite_location_key(raw_target) != location_key:
            continue
        if _callee_key(raw_target) != callee_key:
            continue
        candidates.append(raw_target)

    if not candidates:
        return matched_target, matched_callsite_id
    best = max(candidates, key=lambda target: _target_suffix_specificity(target, matched_callsite_id))
    best_id = _callsite_id_from_target(best, 0)
    logger.info(
        "[verify] using specific unsafe marker %s for ambiguous report callsite %s",
        best_id,
        matched_callsite_id,
    )
    return best, best_id


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


def _compose_init_callsite_marker_for_target(
    *, ll_path: Path, target: dict[str, object] | None, callsite_id: str
) -> str:
    """Pick the marker used for the initial compose/reach run.

    For nested safe wrappers, autoinj emits two markers:
    - ``<id>-root`` at the source-level/report call expression.
    - ``<id>`` immediately around the actual core/std unsafe operation.

    The final rerun/DSL check must use the actual unsafe marker.  The initial
    compose run is mainly used to recover a caller/control-chain for testcase
    generation, and starting directly at the actual callee can be much more
    expensive because KLEE has to synthesize the whole callee receiver state.
    Prefer the root marker when it exists, but keep direct unsafe callsites
    unchanged.
    """

    if target is None or not isinstance(target.get("unsafe_callsite"), dict):
        return callsite_id
    root_callsite_id = f"{callsite_id}-root"
    if _llvm_ir_contains_callsite_marker(ll_path, root_callsite_id):
        return root_callsite_id
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
            return _prefer_specific_callsite_alias(
                targets,
                requested_callsite_id=requested_callsite_id,
                matched_target=raw_target,
                matched_callsite_id=callsite_id,
            )
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


def _load_rule_entry(rule_dsl_path: Path, rule_id: str) -> dict[str, Any]:
    if not rule_dsl_path.is_file():
        raise RuntimeError(f"rule DSL file does not exist: {rule_dsl_path}")
    data = _load_json(rule_dsl_path)
    rules = data.get("rules", data) if isinstance(data, dict) else {}
    if not isinstance(rules, dict):
        raise RuntimeError(f"rule DSL file must contain an object: {rule_dsl_path}")
    entry = rules.get(rule_id)
    if isinstance(entry, str):
        return {"id": rule_id, "dsl": entry}
    if isinstance(entry, dict):
        return {"id": rule_id, **entry}
    raise RuntimeError(f"could not find rule entry for {rule_id} in {rule_dsl_path}")


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
    returncode: int | None,
    text: str,
    timed_out: bool,
    callsite_id: str | None = None,
) -> str:
    lowered = text.lower()
    if timed_out:
        if "sat(constraints and not resolved constraint): sat" in lowered:
            return "violation-timeout"
        if "sat(constraints and not resolved constraint): unsat" in lowered:
            return "verified-timeout"
        if "query solved" in lowered or "[ext.dsl] at callsite" in text:
            return "reached-timeout"
        if callsite_id:
            marker = f"[ext.exec] klee_ext_callsite site='{callsite_id}' target='{callsite_id}'"
            if marker in text:
                return "reached-timeout"
        return "timeout"
    if "sat(constraints and not resolved constraint): sat" in lowered:
        return "violation"
    if "sat(constraints and not resolved constraint): unsat" in lowered:
        return "verified"
    if "query solved" in lowered:
        return "reached"
    if "[ext.dsl] at callsite" in text:
        return "reached"
    if callsite_id:
        marker = f"[ext.exec] klee_ext_callsite site='{callsite_id}' target='{callsite_id}'"
        if marker in text:
            return "reached"
    if returncode is None:
        return "unknown"
    if returncode != 0:
        return "klee-error"
    return "ok"


def _analyze_rerun_output(
    *, returncode: int | None, text: str, timed_out: bool, callsite_id: str
) -> dict[str, Any]:
    lowered = text.lower()
    reported_callsite = f"[ext.dsl] at callsite '{callsite_id}'" in text
    observed_callsites = [
        {"site": site, "target": target}
        for site, target in re.findall(
            r"\[ext\.exec\]\s+klee_ext_callsite\s+site='([^']+)'\s+target='([^']*)'",
            text,
        )
    ]
    saw_rust_panic = "terminating rust panic path" in lowered or "panic_" in lowered
    external_calls = sorted(
        {
            name
            for pattern in (
                r"external call with symbolic argument:\s*([A-Za-z_][A-Za-z0-9_:]*)",
                r"failed external call:\s*([A-Za-z_][A-Za-z0-9_:]*)",
                r"calling external:\s*([A-Za-z_][A-Za-z0-9_:]*)",
            )
            for name in re.findall(pattern, text)
        }
    )
    dsl_sat = "sat(constraints and not resolved constraint): sat" in lowered
    has_certain_symbol = (
        "[ext.dsl] resolved constraint uses certain symbol: true" in text
        or "[ext.dsl] violation query uses certain symbol: true" in text
        or "[ext.raw-ptr-deref] pointer uses certain symbol: true" in text
        or "pointer uses certain symbol: true" in text
    )
    # Reproduction is decided at the target callsite. Once KLEE has reached the
    # exact callsite and the target rule query is SAT, later exploration errors
    # (for example an unrelated extern destructor) or an eventual timeout must
    # not turn an already-reproduced counterexample back into a miss.
    full_rerun_passed = reported_callsite and dsl_sat
    if full_rerun_passed and timed_out:
        status = "reported-sat-timeout"
    elif full_rerun_passed and returncode not in (0, None):
        status = "reported-sat-klee-error"
    elif full_rerun_passed:
        status = "reported-sat"
    elif timed_out:
        status = "timeout"
    elif returncode is None:
        status = "unknown"
    elif returncode != 0:
        status = "klee-error"
    elif not reported_callsite:
        status = "callsite-not-reported"
    elif not dsl_sat:
        status = "callsite-reported-non-sat"
    else:
        status = "unknown"
    return {
        "status": status,
        "returncode": returncode,
        "timed_out": timed_out,
        "reported_callsite": reported_callsite,
        "dsl_sat": dsl_sat,
        "has_certain_symbol": has_certain_symbol,
        "full_rerun_passed": full_rerun_passed,
        "external_calls": external_calls,
        "observed_callsites": observed_callsites[-12:],
        "saw_rust_panic": saw_rust_panic,
    }


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
    timeout_sec: int | None,
) -> tuple[int, str]:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        klee_bin,
        *_klee_no_stats_args(),
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
    started = time.time()
    timed_out = False
    try:
        result = subprocess.run(
            cmd,
            check=False,
            capture_output=True,
            text=True,
            timeout=timeout_sec,
            env={**os.environ, "RUST_BACKTRACE": os.environ.get("RUST_BACKTRACE", "0")},
        )
        returncode = result.returncode
        stdout = result.stdout or ""
        stderr = result.stderr or ""
    except subprocess.TimeoutExpired as exc:
        timed_out = True
        returncode = 124
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
    log_path = output_dir.parent / "klee-compose.log"
    log_path.write_text(
        "command: " + repr(cmd) + "\n"
        f"returncode: {returncode}\n"
        f"timeout: {str(timed_out).lower()}\n"
        f"duration_sec: {time.time() - started:.3f}\n\n"
        "[stdout]\n" + stdout + "\n"
        "[stderr]\n" + stderr,
        encoding="utf-8",
    )
    if stdout:
        print(stdout, end="")
    if stderr:
        print(stderr, end="")
    combined = stdout + stderr
    if timed_out:
        status = _compose_status_from_output(
            returncode, combined, timed_out=True, callsite_id=callsite_id
        )
        if status in {"violation-timeout", "reached-timeout"}:
            return returncode, "candidate"
        if status == "verified-timeout":
            return returncode, "verified"
        return returncode, "timeout"
    if (
        "SAT(constraints AND NOT resolved constraint): sat" in combined
        or "query solved" in combined
        or "query deferred for compose" in combined
    ):
        return returncode, "candidate"
    if "SAT(constraints AND NOT resolved constraint): unsat" in combined:
        return returncode, "verified"
    return returncode, "unknown"


def _run_klee_compose_rerun(
    *, ll_path: Path, callsite_id: str, ast_json: str, klee_bin: str,
    output_dir: Path, entry_function: str, log_path: Path | None = None,
    timeout_sec: int | None = None, report_json: Path | None = None,
    raw_ptr_deref: bool = False, rerun_sym: bool = False,
) -> dict[str, Any]:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.parent.mkdir(parents=True, exist_ok=True)
    compose_flag = "--compose-rerun-sym" if rerun_sym else "--compose-rerun"
    mode = "rerun-sym" if rerun_sym else "rerun"
    cmd = [
        klee_bin,
        *_klee_no_stats_args(),
        f"--output-dir={output_dir}",
        f"--entry-point={entry_function}",
        f"--ext.callsite={callsite_id}",
        compose_flag,
    ]
    if raw_ptr_deref:
        cmd.append("--ext.raw-ptr-deref")
    else:
        cmd.append(f"--ext.dsl={ast_json}")
    if report_json is not None:
        cmd.append(f"--report-json={report_json}")
    cmd.append(str(ll_path))
    logger.info("[verify:%s] running: %s", mode, " ".join(cmd))
    started = time.time()
    timed_out = False
    try:
        result = subprocess.run(
            cmd,
            check=False,
            capture_output=True,
            text=True,
            timeout=timeout_sec,
            env={**os.environ, "RUST_BACKTRACE": os.environ.get("RUST_BACKTRACE", "0")},
        )
        stdout = result.stdout or ""
        stderr = result.stderr or ""
        returncode = result.returncode
    except subprocess.TimeoutExpired as exc:
        timed_out = True
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
        returncode = 124
    combined = stdout + stderr
    analysis = _analyze_rerun_output(
        returncode=returncode,
        text=combined,
        timed_out=timed_out,
        callsite_id=callsite_id,
    )
    if log_path is not None:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        log_path.write_text(
            "command: " + repr(cmd) + "\n"
            f"mode: {mode}\n"
            f"returncode: {returncode}\n"
            f"timeout: {str(timed_out).lower()}\n"
            f"duration_sec: {time.time() - started:.3f}\n"
            f"target_callsite_reported: {str(analysis['reported_callsite']).lower()}\n"
            f"dsl_sat: {str(analysis['dsl_sat']).lower()}\n"
            f"has_certain_symbol: {str(analysis['has_certain_symbol']).lower()}\n"
            f"full_rerun_passed: {str(analysis['full_rerun_passed']).lower()}\n\n"
            "[stdout]\n" + stdout + "\n"
            "[stderr]\n" + stderr,
            encoding="utf-8",
        )
    if stdout:
        print(stdout, end="")
    if stderr:
        print(stderr, end="")
    analysis["duration_sec"] = round(time.time() - started, 3)
    analysis["mode"] = mode
    return analysis


def _matrix_callsite_rows(
    *,
    targets: list[object],
    ll_path: Path,
    requested_callsite: str | None,
    requested_callsites: set[str] | None = None,
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
        if requested_callsites and not ({callsite_id, callsite_key, str(idx)} & requested_callsites):
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


def _load_callsites_file(repo_root: Path, path_arg: str | None) -> set[str] | None:
    if not path_arg:
        return None
    path = _resolve_path(repo_root, path_arg, path_arg)
    callsites = {
        line.strip()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    }
    if not callsites:
        raise RuntimeError(f"--callsites-file is empty: {path}")
    return callsites


def _missing_callsite_bodies(
    *, targets: list[object], ll_path: Path
) -> list[dict[str, Any]]:
    """Return MIRScan callsites whose injected caller has no emitted body.

    Prefer autoinj's unique string marker inside the caller.  For callsites
    autoinj cannot rewrite (for example some macro-expanded/raw-deref sites),
    fall back to LLVM definition debug metadata for the same source function.
    """
    rows = _matrix_callsite_rows(
        targets=targets,
        ll_path=ll_path,
        requested_callsite=None,
        requested_callsites=None,
    )
    unresolved = [row for row in rows if not row["present_in_llvm_ir"]]
    if not unresolved:
        return []

    definitions = _llvm_ir_defined_source_functions(ll_path)
    return [
        row
        for row in unresolved
        if not _row_has_defined_source_function(row, definitions)
    ]


_LLVM_DI_FILE_RE = re.compile(
    r'^!(\d+) = !DIFile\(filename: "([^"]*)", directory: "([^"]*)"'
)
_LLVM_DI_SUBPROGRAM_RE = re.compile(
    r'^!\d+ = (?:distinct )?!DISubprogram\(name: "([^"]*)".*?'
    r"file: !(\d+), line: \d+.*?spFlags: ([^)]*)"
)


def _llvm_ir_defined_source_functions(ll_path: Path) -> set[tuple[str, str]]:
    """Index defined Rust functions by normalized source path and debug name."""
    files: dict[str, tuple[str, str]] = {}
    subprograms: list[tuple[str, str]] = []
    try:
        with ll_path.open("r", encoding="utf-8", errors="ignore") as llvm_ir:
            for line in llvm_ir:
                file_match = _LLVM_DI_FILE_RE.match(line)
                if file_match:
                    files[file_match.group(1)] = (
                        file_match.group(2),
                        file_match.group(3),
                    )
                    continue
                if "DISubprogram(" not in line or "DISPFlagDefinition" not in line:
                    continue
                subprogram_match = _LLVM_DI_SUBPROGRAM_RE.match(line)
                if subprogram_match and "DISPFlagDefinition" in subprogram_match.group(3):
                    subprograms.append(
                        (subprogram_match.group(1), subprogram_match.group(2))
                    )
    except OSError:
        return set()

    definitions: set[tuple[str, str]] = set()
    for function_name, file_id in subprograms:
        source = files.get(file_id)
        if source is None:
            continue
        filename, directory = source
        source_path = f"{directory.rstrip('/')}/{filename}" if directory else filename
        definitions.add((source_path.replace("\\", "/"), function_name))
    return definitions


def _caller_leaf_name(caller_name: object) -> str:
    if not isinstance(caller_name, str):
        return ""
    if ">::" in caller_name:
        leaf = caller_name.rsplit(">::", 1)[-1]
    else:
        leaf = caller_name.rsplit("::", 1)[-1]
    return leaf.split("<", 1)[0]


def _row_has_defined_source_function(
    row: dict[str, Any], definitions: set[tuple[str, str]]
) -> bool:
    source_path = row.get("path")
    leaf_name = _caller_leaf_name(row.get("caller"))
    if not isinstance(source_path, str) or not source_path or not leaf_name:
        return False
    normalized_path = source_path.replace("\\", "/")
    for defined_path, defined_name in definitions:
        if not (
            defined_path == normalized_path
            or defined_path.endswith("/" + normalized_path)
        ):
            continue
        if defined_name == leaf_name or defined_name.startswith(leaf_name + "<"):
            return True
    return False


def _validate_callsite_bodies(*, meta_path: Path, ll_path: Path) -> int:
    current_meta = _load_json(meta_path)
    report = current_meta.get("report")
    if not isinstance(report, dict):
        raise RuntimeError(f"missing report object in {meta_path}")
    targets = report.get("targets")
    if not isinstance(targets, list):
        raise RuntimeError(f"missing targets in {meta_path}")

    missing = _missing_callsite_bodies(targets=targets, ll_path=ll_path)
    if missing:
        details = "\n".join(
            "  - "
            f"{row['callsite_id']}: caller={row.get('caller') or '<unknown>'} "
            f"at {row.get('path') or '<unknown>'}:"
            f"{row.get('line') or 0}:{row.get('col') or 0}"
            for row in missing
        )
        raise RuntimeError(
            f"{len(missing)} of {len(targets)} MIRScan callsites have no caller "
            "body in linked LLVM IR (a generic caller may need a minimal unit-test "
            f"instantiation):\n{details}"
        )
    return len(targets)


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
        if callsite.get("unsafe_callee") == RAW_PTR_DEREF_CALLEE:
            callsite["rules"] = [RAW_PTR_DEREF_RULE]
            continue
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
    return row.get("status") in {"violation", "violation-timeout"}


def _matrix_sort_key(row: dict[str, Any]) -> tuple[Any, Any]:
    return (row.get("target_index", 0), row.get("rule", ""))


def _is_confirmed_callsite_violation(row: dict[str, Any]) -> bool:
    return row.get("status") == "violation"


def _is_reached_callsite_status(status: object) -> bool:
    return status in {
        "violation",
        "verified",
        "reached",
        "reached-timeout",
        "low-confidence-sat",
    }


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


def _callsite_summary_from_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "target_index": row.get("target_index"),
        "id": row.get("callsite"),
        "llvm_id": row.get("llvm_callsite"),
        "path": row.get("path"),
        "line": row.get("line"),
        "col": row.get("col"),
        "caller": row.get("caller"),
        "unsafe_callee": row.get("unsafe_callee"),
        "unsafe_callee_path": row.get("unsafe_callee_path"),
        "unsafe_callee_line_start": row.get("unsafe_callee_line_start"),
    }


def _stage_paths_from_pipeline(pipeline: dict[str, Any]) -> dict[str, Any]:
    attempts = pipeline.get("llm_testcase_attempts")
    if not isinstance(attempts, list):
        attempts = []
    status = pipeline.get("status")
    if status == "compose-verified-skip-testcase":
        status = "skipped"
    elif attempts:
        last = attempts[-1]
        status = last.get("status") if isinstance(last, dict) else None
    if status is None and pipeline.get("source_path"):
        status = "build-ok"
    return {
        "status": status,
        "artifact_dir": pipeline.get("artifact_dir"),
        "source_path": pipeline.get("source_path"),
        "source_line": pipeline.get("source_line"),
        "prompt": pipeline.get("testcase_prompt"),
        "response": pipeline.get("testcase_response"),
        "context_stats": pipeline.get("testcase_context_stats"),
        "context_policy": pipeline.get("llm_context"),
        "build_log": pipeline.get("build_log"),
        "attempts": attempts,
        "rerun_sym": pipeline.get("rerun_sym"),
    }


def _rerun_paths_for_result(artifact_dir: Path, rerun_result: dict[str, Any]) -> dict[str, str]:
    if rerun_result.get("mode") == "rerun-sym":
        return {
            "log": str(artifact_dir / "klee-rerun-sym.log"),
            "klee_output_dir": str(artifact_dir / "klee-rerun-sym"),
        }
    return {
        "log": str(artifact_dir / "klee-rerun.log"),
        "klee_output_dir": str(artifact_dir / "klee-rerun"),
    }


def _write_pair_result(
    *,
    path: Path,
    crate_dir: Path,
    injected_dir: Path,
    meta_path: Path | None,
    rule_dsl_path: Path,
    callsite: dict[str, Any],
    rule_id: str,
    init: dict[str, Any] | None,
    llm: dict[str, Any] | None = None,
    rerun: dict[str, Any] | None = None,
) -> None:
    rule = _load_rule_entry(rule_dsl_path, rule_id) if rule_id != RAW_PTR_DEREF_RULE else {
        "id": RAW_PTR_DEREF_RULE,
        "path": None,
        "name": RAW_PTR_DEREF_CALLEE,
        "rule": "Raw pointer dereference is treated as a bug when the pointer expression uses a certain symbol.",
        "dsl": None,
    }
    rerun = rerun or {}
    result = {
        "init_status": (init or {}).get("status"),
        "llm_status": (llm or {}).get("status"),
        "rerun_status": rerun.get("status"),
        "rerun_returncode": rerun.get("returncode"),
        "rerun_reported_callsite": bool(rerun.get("reported_callsite")),
        "rerun_dsl_sat": bool(rerun.get("dsl_sat")),
        "rerun_has_certain_symbol": bool(rerun.get("has_certain_symbol")),
        "full_rerun_passed": bool(rerun.get("full_rerun_passed")),
    }
    _atomic_write_json(
        path,
        {
            "schema_version": 2,
            "updated_at": _utc_now(),
            "crate": str(crate_dir),
            "injected_crate": str(injected_dir),
            "metadata": str(meta_path) if meta_path is not None else None,
            "callsite": callsite,
            "rule": rule,
            "result": result,
            "stages": {
                "init": init or {},
                "llm": llm or {},
                "rerun": rerun,
            },
        },
    )


def _tail_text(text: str, *, max_chars: int = 12000) -> str:
    if len(text) <= max_chars:
        return text
    return text[-max_chars:]


def _merge_semantic_retry_feedback(
    existing: str | None,
    new_feedback: str | None,
) -> str | None:
    if not existing:
        return new_feedback
    if not new_feedback:
        return existing
    if existing in new_feedback:
        return new_feedback

    sticky_lines: list[str] = []
    for raw_line in existing.splitlines():
        line = raw_line.strip()
        lower = line.lower()
        if (
            "hard retry constraint" in lower
            or "must not be `0`" in lower
            or "zero offset/count" in lower
            or "target counterexample query was unsat" in lower
        ):
            if line and line not in new_feedback and line not in sticky_lines:
                sticky_lines.append(line)
    if not sticky_lines:
        return new_feedback
    merged = (
        new_feedback
        + "\n\n<still-active semantic constraints from earlier KLEE reruns>\n"
        + "\n".join(sticky_lines)
        + "\n</still-active semantic constraints from earlier KLEE reruns>"
    )
    return _tail_text(merged)


def _read_failure_feedback(path: Path | None, fallback: BaseException) -> str:
    parts = [str(fallback)]
    if path is not None and path.is_file():
        try:
            parts.append(path.read_text(encoding="utf-8", errors="replace"))
        except OSError:
            pass
    return _tail_text("\n\n".join(part for part in parts if part))


def _is_testcase_format_feedback(feedback: str | None) -> bool:
    if not feedback:
        return False
    return any(
        needle in feedback
        for needle in (
            "testcase generator returned no Rust code block",
            "generated testcase contains `unsafe`",
            "generated testcase contains `#[cfg(test)]`",
            "generated testcase must define",
            "generated testcase must be gated by feature",
        )
    )


def _should_use_semantic_retry(rerun_result: dict[str, Any] | None) -> bool:
    if not rerun_result or rerun_result.get("full_rerun_passed"):
        return False
    status = rerun_result.get("status")
    return status == "callsite-reported-non-sat" or (
        bool(rerun_result.get("reported_callsite"))
        and not bool(rerun_result.get("dsl_sat"))
    )


def _read_rerun_failure_feedback(
    path: Path | None,
    *,
    guidance: str,
) -> str:
    return guidance


def _testcase_retry_guidance(
    *, rerun_result: dict[str, Any], callsite_id: str, target: dict[str, object]
) -> str:
    caller = target.get("caller") if isinstance(target, dict) else None
    caller = caller if isinstance(caller, dict) else {}
    caller_name = caller.get("name")
    caller_is_unsafe = caller.get("is_unsafe")
    caller_hint = ""
    if isinstance(caller_name, str) and caller_name:
        if isinstance(target.get("unsafe_callsite"), dict):
            caller_hint = (
                f"The metadata caller `{caller_name}` is the root wrapper for "
                "this target. The target context also shows a separate actual "
                "unsafe-call containing function; if it is safe and callable "
                "from the injected module, prefer constructing its receiver/"
                "parameters and calling that actual-containing function directly "
                "instead of routing through the root wrapper."
            )
        elif caller_is_unsafe is False:
            caller_hint = (
                f"The metadata caller `{caller_name}` is a safe function that "
                "contains the target unsafe callsite. Prefer calling this exact "
                "caller directly instead of routing through a higher-level "
                "wrapper."
            )
        else:
            caller_hint = (
                f"The metadata caller is `{caller_name}`. Build the smallest "
                "safe path that reaches this caller and avoid unrelated wrappers "
                "that can validate inputs first."
            )

    status = rerun_result.get("status")
    external_calls = rerun_result.get("external_calls")
    external_hint = ""
    if isinstance(external_calls, list):
        names = [
            name
            for name in external_calls
            if isinstance(name, str) and name
        ]
        if names:
            external_hint = (
                "KLEE stopped before the target at external function(s): "
                + ", ".join(f"`{name}`" for name in names[:8])
                + ". Avoid constructors/wrappers/destructors that call these "
                "symbols; build the target caller receiver/arguments from "
                "fields visible in the injection module when possible."
            )
    observed_hint = ""
    observed_callsites = rerun_result.get("observed_callsites")
    if isinstance(observed_callsites, list):
        sites: list[str] = []
        for item in observed_callsites[-8:]:
            if not isinstance(item, dict):
                continue
            site = item.get("site")
            if isinstance(site, str) and site:
                sites.append(site)
        if sites:
            observed_hint = (
                "KLEE reached these callsite markers before missing the target "
                "(last markers first-order): "
                + " -> ".join(f"`{site}`" for site in sites)
                + ". Treat a marker with a different id as prefix progress only; "
                "the testcase must still reach the exact target marker "
                f"`{callsite_id}` and reproduce the target counterexample there."
            )
    panic_hint = ""
    if rerun_result.get("saw_rust_panic"):
        panic_hint = (
            "KLEE observed a Rust panic before reproducing the target. This "
            "usually means a safe prefix lookup/index/branch precondition was "
            "not satisfied; adjust constructor fields so every prefix helper "
            "returns normally while keeping the target relation violating. "
            "Re-check the actual target function prefix shown in the context: "
            "any `.get(...).expect(...)`, `Type::decode(...).expect(...)`, "
            "`let Some(...) = ... else`, or `if !guard { panic!(...) }` must "
            "succeed before the target. Avoid using a default/empty receiver "
            "state as the whole testcase when the prefix needs a non-empty "
            "collection, valid encoded element, or coherent branch guard. If "
            "the prefix reads/writes through raw pointer fields, do not leave "
            "those prefix pointer fields as `null()`/`null_mut()`; provide tiny "
            "local backing arrays/Vecs or other safe pointer-producing values "
            "so the prefix can execute normally before the target callsite. If "
            "a target argument/index is also used by earlier prefix raw-pointer "
            "operations, make the prefix backing large enough for that value "
            "but keep the actual target receiver/buffer relation independent "
            "and violating."
        )
    if status == "callsite-not-reported":
        reason = (
            f"KLEE did not report the exact target callsite `{callsite_id}`. "
            "The testcase likely panicked, returned `None`/`Err`, returned "
            "early, or exercised a different unsafe call before reaching the "
            "target."
        )
    elif status == "callsite-reported-non-sat":
        reason = (
            f"KLEE reached `{callsite_id}`, but the target counterexample was "
            "not reproduced. The previous concrete values satisfied the target "
            "callsite's safety requirement. If the target caller takes separate "
            "pointer/slice/buffer and length/count/capacity/index inputs, do not "
            "tie those numeric inputs to the local allocation size by default; "
            "try a small boundary relationship that reaches the same target "
            "callsite but makes the safety requirement false."
        )
    elif status == "timeout":
        reason = (
            "KLEE timed out during rerun. Make the testcase smaller and more "
            "direct, with tiny concrete allocations and no loops unless needed "
            "to reach the target."
        )
    else:
        reason = (
            "The testcase compiled but did not reproduce the target violation "
            f"(rerun status: {status})."
        )

    pieces = [
        "The testcase compiled but did not reproduce the target violation.",
        reason,
        observed_hint,
        panic_hint,
        external_hint,
        caller_hint,
        (
            "Do not use KLEE helpers or symbolic inputs. Do not introduce local "
            "bindings that shadow crate constants/statics from the target module; "
            "call the crate code under test."
        ),
    ]
    return "\n".join(piece for piece in pieces if piece)


def _observed_target_arg_feedback(
    log_path: Path,
    ast_json: str,
    *,
    target: dict[str, Any] | None = None,
    crate_dir: Path | None = None,
) -> str:
    try:
        text = log_path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return ""
    observations: list[dict[str, Any]] = []
    current: dict[str, Any] | None = None
    for raw in text.splitlines():
        line = raw.rstrip()
        if "[ext.call] values:" in line:
            current = {"args": {}, "dsl": []}
            continue
        if current is None:
            continue
        stripped_line = line.strip()
        if stripped_line.startswith("__klee_call_arg"):
            match = re.match(
                r"__klee_call_arg(\d+)\s*=\s*(.*?)(?:\s+\(explicit\))?$",
                stripped_line,
            )
            if match:
                current["args"][f"get_arg({match.group(1)})"] = match.group(2).strip()
                continue
        if "[ext.dsl]" in line and " -> " in line:
            stripped = line.split("[ext.dsl]", 1)[-1].strip()
            if stripped.startswith("(") or stripped.startswith("get_arg("):
                current["dsl"].append(stripped)
            continue
        if "SAT(constraints AND NOT resolved constraint):" in line:
            current["sat"] = line.rsplit(":", 1)[-1].strip()
            observations.append(current)
            current = None
            continue
        if line.startswith("KLEE: [ext.exec]") or line.startswith("KLEE: [ext.callsite]"):
            # A new event before a SAT line means this was not the final target
            # query block we want to summarize.
            if current.get("args"):
                observations.append(current)
            current = None
    if current and current.get("args"):
        observations.append(current)
    if not observations:
        return ""

    try:
        ast = json.loads(ast_json)
    except json.JSONDecodeError:
        ast = {}
    simplified = ast.get("simplified") if isinstance(ast, dict) else None
    op = simplified.get("op") if isinstance(simplified, dict) else None
    left = _describe_ext_expr(simplified.get("left")) if isinstance(simplified, dict) else None
    right = _describe_ext_expr(simplified.get("right")) if isinstance(simplified, dict) else None
    negated = {
        "<": ">=",
        "<=": ">",
        ">": "<=",
        ">=": "<",
        "==": "!=",
        "!=": "==",
    }.get(op)

    arg_source_map: dict[str, str] = {}
    if target is not None and crate_dir is not None:
        try:
            arg_source_map = target_call_arg_source_map(
                crate_dir=crate_dir,
                target=target,
            )
        except Exception:
            arg_source_map = {}

    lines = [
        "Observed KLEE rerun target-argument feedback from the failed testcase:"
    ]
    for index, obs in enumerate(observations[-3:], start=1):
        args = obs.get("args") if isinstance(obs.get("args"), dict) else {}
        pieces = [
            f"{name} = {value}"
            for name, value in sorted(args.items(), key=lambda item: item[0])
        ]
        sat = obs.get("sat")
        suffix = ""
        if isinstance(sat, str) and sat:
            suffix = f"; target counterexample query was {sat}"
        lines.append(f"- Observation {index}: " + "; ".join(pieces) + suffix + ".")
        for name, source_expr in sorted(arg_source_map.items()):
            value = args.get(name)
            if not isinstance(value, str):
                continue
            lines.append(
                f"  At the selected target expression, `{name}` is source "
                f"expression `{source_expr}`; this failed observation had "
                f"`{source_expr} = {value}`."
            )
            if value == "0":
                lines.append(
                    f"  HARD RETRY CONSTRAINT: `{source_expr}` must not be `0` "
                    "in the next generated testcase at this exact target call. "
                    "If this source expression is a target-caller parameter, "
                    "the target-caller invocation must pass a non-zero concrete "
                    "value for that parameter; do not call it with literal `0` "
                    "or with a variable initialized to `0`."
                )
                lines.append(
                    f"  In the next testcase, make source expression "
                    f"`{source_expr}` non-zero at the same target callsite. "
                    "If it is a caller argument or visible receiver field, set "
                    "that caller input/field directly rather than reaching the "
                    "target through a wrapper that fixes it back to zero. For "
                    "pointer arithmetic targets such as `add`/`offset`, zero "
                    "offset/count often makes the safety relation trivially "
                    "true; try a concrete small non-zero value such as `1`."
                )
        if left and right and left in args and right in args and op and negated:
            lines.append(
                f"  The rule is `{left} {op} {right}`. This failed observation "
                f"had `{left} = {args[left]}` and `{right} = {args[right]}`; "
                f"the next testcase must reach the same target but make "
                f"`{left} {negated} {right}` true at the actual unsafe call."
            )
    lines.append(
        "If an earlier guarded lookup/index is required to reach the target, "
        "keep that earlier value in-bounds. Do not reuse that same value for "
        "the actual target unsafe-call argument unless the source code requires "
        "them to be equal; construct independent visible fields/arguments when "
        "the target caller state allows it. If the target context shows an "
        "actual unsafe-call source line, change the concrete source value of "
        "that exact receiver/argument expression rather than only changing an "
        "earlier guarded lookup that merely reaches the target. Do not preserve "
        "implicit invariants like `index field == Vec slot` or `stored length == "
        "allocation length` when the target caller does not enforce them before "
        "the actual unsafe call; safe construction of an inconsistent but "
        "type-correct receiver state is allowed. When the same source value is "
        "used by prefix pointer operations and the target pointer operation, "
        "the prefix pointer backing may need to be larger than the target "
        "pointer backing so execution reaches the target and the selected "
        "target relation is still false. If multiple visible raw-pointer "
        "receiver fields are used with the same index/count before and at the "
        "target, do not make all of those fields share the same allocation "
        "length by default: keep prefix receivers large enough and make the "
        "target receiver shorter or otherwise independent when that is what "
        "violates the target relation."
    )
    return "\n".join(lines)


def _build_klee_witness_text(
    *,
    callsite_id: str,
    rule_id: str,
    ast_json: str,
    compose_output: Path,
) -> str:
    parts: list[str] = [
        f"Target callsite id: {callsite_id}",
        f"Rule id: {rule_id}",
    ]
    log_path = compose_output.parent / "klee-compose.log"
    witness_lines = _extract_klee_init_witness_lines(log_path, callsite_id=callsite_id)
    if witness_lines:
        obligations = _derive_witness_obligations(ast_json, witness_lines)
        parts.append(
            "KLEE init witness at the target call. These are the call argument "
            "expressions/values from a path where KLEE found a target "
            "counterexample satisfiable. Shape the concrete testcase so the "
            "same target call arguments reproduce the counterexample:\n"
            + ("\n".join(obligations) + "\n" if obligations else "")
            + "\n".join(witness_lines)
        )
    return "\n\n".join(parts)


def _describe_ext_expr(expr: object) -> str | None:
    if not isinstance(expr, dict):
        return None
    kind = expr.get("type")
    if kind == "call" and expr.get("name") == "get_arg":
        args = expr.get("args")
        if (
            isinstance(args, list)
            and len(args) == 1
            and isinstance(args[0], dict)
            and args[0].get("type") == "literal"
        ):
            return f"get_arg({args[0].get('value')})"
    if kind == "simplified_var":
        name = expr.get("name")
        return name if isinstance(name, str) else None
    if kind == "literal":
        return str(expr.get("value"))
    return None


def _extract_witness_arg_map(witness_lines: list[str]) -> dict[str, str]:
    arg_map: dict[str, str] = {}
    for line in witness_lines:
        match = re.search(
            r"get_arg\((\d+)\)\s*->\s*simplified\(__klee_call_arg\d+=(.*)\)",
            line,
        )
        if match:
            arg_map[f"get_arg({match.group(1)})"] = match.group(2).strip()
            continue
        match = re.search(r"__klee_call_arg(\d+)\s*=\s*(.*?)(?:\s+\(explicit\))?$", line)
        if match:
            arg_map.setdefault(f"get_arg({match.group(1)})", match.group(2).strip())
    return arg_map


def _friendly_witness_expr(expr: str) -> str:
    # KLEE symbols often contain a Rust-mangled function prefix followed by a
    # readable snapshot path such as `.arg0.triangles.len`. Surface that suffix
    # for the testcase generator; the raw expression remains available below.
    match = re.search(r"\.(arg\d+(?:\.[A-Za-z_][A-Za-z0-9_]*)+)", expr)
    if not match:
        return expr
    path = match.group(1)
    bits = path.split(".")
    if len(bits) >= 3 and bits[-1] in {"len", "cap", "capacity", "virtual_base"}:
        owner = ".".join(bits[:-1])
        suffix = bits[-1]
        if suffix == "len":
            return f"receiver/caller state `{owner}` length"
        if suffix in {"cap", "capacity"}:
            return f"receiver/caller state `{owner}` capacity"
        return f"receiver/caller state `{owner}` pointer/base"
    return f"receiver/caller state `{path}`"


def _derive_witness_obligations(ast_json: str, witness_lines: list[str]) -> list[str]:
    try:
        ast = json.loads(ast_json)
    except json.JSONDecodeError:
        return []
    simplified = ast.get("simplified") if isinstance(ast, dict) else None
    if not isinstance(simplified, dict) or simplified.get("type") != "binary":
        return []
    op = simplified.get("op")
    negated = {
        "<": ">=",
        "<=": ">",
        ">": "<=",
        ">=": "<",
        "==": "!=",
        "!=": "==",
    }.get(op)
    if not isinstance(op, str) or negated is None:
        return []
    left = _describe_ext_expr(simplified.get("left"))
    right = _describe_ext_expr(simplified.get("right"))
    if left is None or right is None:
        return []
    arg_map = _extract_witness_arg_map(witness_lines)
    lines = [
        f"Witness-derived target relation: construct the target caller state "
        f"so `{left} {negated} {right}` is true at the target call."
    ]
    left_value = arg_map.get(left)
    right_value = arg_map.get(right)
    if left_value is not None or right_value is not None:
        pieces = []
        if left_value is not None:
            pieces.append(f"{left} = {left_value}")
        if right_value is not None:
            pieces.append(f"{right} = {right_value}")
        lines.append("Witness argument mapping: " + "; ".join(pieces) + ".")
    if left_value is not None and right_value is not None:
        if re.fullmatch(r"-?\d+", left_value) and not re.fullmatch(r"-?\d+", right_value):
            if negated in {">=", ">"}:
                direct = f"`{right}` must be {'<=' if negated == '>=' else '<'} {left_value}"
            elif negated in {"<=", "<"}:
                direct = f"`{right}` must be {'>=' if negated == '<=' else '>'} {left_value}"
            elif negated == "==":
                direct = f"`{right}` must equal {left_value}"
            else:
                direct = f"`{right}` must differ from {left_value}"
            lines.append(
                f"Because `{left}` is concrete `{left_value}`, the concrete "
                f"target state should satisfy: {direct}. The witness maps "
                f"`{right}` to `{_friendly_witness_expr(right_value)}` "
                f"(raw: `{right_value}`); if that expression is a "
                "length/capacity/field, set that field at the required boundary."
            )
            if ".len" in right_value and negated in {">=", ">"}:
                lines.append(
                    "This is a length boundary obligation: use a zero-length "
                    "container/slice for that mapped field if it type-checks; "
                    "do not use `vec![...]`, arrays with elements, or constructors "
                    "that add elements to that same field."
                )
        elif re.fullmatch(r"-?\d+", right_value) and not re.fullmatch(r"-?\d+", left_value):
            if negated in {">=", ">"}:
                direct = f"`{left}` must be {'>=' if negated == '>=' else '>'} {right_value}"
            elif negated in {"<=", "<"}:
                direct = f"`{left}` must be {'<=' if negated == '<=' else '<'} {right_value}"
            elif negated == "==":
                direct = f"`{left}` must equal {right_value}"
            else:
                direct = f"`{left}` must differ from {right_value}"
            lines.append(
                f"Because `{right}` is concrete `{right_value}`, the concrete "
                f"target state should satisfy: {direct}. The witness maps "
                f"`{left}` to `{_friendly_witness_expr(left_value)}` "
                f"(raw: `{left_value}`); if that expression is a "
                "length/capacity/field, set that field at the required boundary."
            )
            if ".len" in left_value and negated in {"<=", "<"}:
                lines.append(
                    "This is a length boundary obligation: use a zero-length "
                    "container/slice for that mapped field if it type-checks; "
                    "do not use `vec![...]`, arrays with elements, or constructors "
                    "that add elements to that same field."
                )
    return lines


def _extract_klee_init_witness_lines(
    log_path: Path,
    *,
    callsite_id: str,
    max_lines: int = 80,
) -> list[str]:
    if not log_path.is_file():
        return []
    try:
        text = log_path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return []
    lines = text.splitlines()
    candidates: list[list[str]] = []
    current: list[str] | None = None
    saw_target = False
    for raw in lines:
        line = raw.rstrip()
        target_marker = (
            f"[ext.exec] klee_ext_callsite site='{callsite_id}' target='{callsite_id}'"
        )
        if target_marker in line or f"[ext.dsl] at callsite '{callsite_id}'" in line:
            saw_target = True
        if saw_target and "[ext.call] values:" in line:
            current = [line]
            continue
        if current is None:
            continue
        if line.startswith("  __klee_call_arg") or "[ext.dsl]" in line:
            current.append(line)
            if "SAT(constraints AND NOT resolved constraint): sat" in line:
                candidates.append(current)
                current = None
                saw_target = False
            elif "SAT(constraints AND NOT resolved constraint): unsat" in line:
                current = None
                saw_target = False
            elif len(current) >= max_lines:
                candidates.append(current)
                current = None
                saw_target = False
            continue
        if not line.strip():
            continue
        if current:
            # Keep scanning across KLEE's blank-separated DSL trace, but stop
            # when the next unrelated subsystem starts.
            if not line.startswith("KLEE:"):
                continue
            if "[ext." not in line:
                continue
    if not candidates:
        return []
    selected = candidates[-1][:max_lines]
    return selected


def _run_llm_testcase_pipeline(
    *,
    args: argparse.Namespace,
    repo_root: Path,
    cargo_dir: Path,
    injected_dir: Path,
    rule_dsl_path: Path,
    target: dict[str, object],
    callsite_id: str,
    chain_callsite_id: str | None = None,
    rule_id: str,
    ast_json: str,
    artifact_dir: Path,
    compose_output: Path,
    report_json: Path | None,
) -> dict[str, Any]:
    chain = write_certainty_chain_json(
        info_path=compose_output / "info",
        output_path=artifact_dir / "certainty-chain.json",
        callsite=chain_callsite_id or callsite_id,
        rule=rule_id,
    )
    if args.skip_llm_testcase:
        logger.info("[verify] --skip-llm-testcase set; stopping before LLM testcase generation")
        pipeline_state = {
            "schema_version": 1,
            "callsite": callsite_id,
            "chain_callsite": chain_callsite_id or callsite_id,
            "rule": rule_id,
        }
        pipeline_state["status"] = "skipped"
        pipeline_state["reason"] = "--skip-llm-testcase set"
        (artifact_dir / "testcase-pipeline.json").write_text(
            json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
        )
        return {"status": "skipped", "returncode": 0, "full_rerun_passed": False}

    if rule_id == RAW_PTR_DEREF_RULE:
        rule = {
            "path": None,
            "name": RAW_PTR_DEREF_CALLEE,
            "rule": (
                "Raw pointer dereference is considered buggy when the dereferenced "
                "pointer expression is derived from a certain external symbol."
            ),
            "dsl": None,
        }
    else:
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
    pipeline_state = {
        "schema_version": 1,
        "callsite": callsite_id,
        "chain_callsite": chain_callsite_id or callsite_id,
        "rule": rule_id,
        "feature": injection.feature,
        "entry_function": injection.function,
        "artifact_dir": str(artifact_dir),
        "klee_compose_dir": str(compose_output),
        "klee_compose_log": str(artifact_dir / "klee-compose.log"),
        "skip_rerun": bool(args.skip_rerun),
        "skip_rerun_sym": bool(getattr(args, "skip_rerun_sym", False)),
        "llm_context": str(getattr(args, "llm_context", "slice")),
        "llm_testcase_attempts": [],
    }
    klee_witness = _build_klee_witness_text(
        callsite_id=callsite_id,
        rule_id=rule_id,
        ast_json=ast_json,
        compose_output=compose_output,
    )
    (artifact_dir / "klee-witness.txt").write_text(
        klee_witness + "\n", encoding="utf-8"
    )
    pipeline_state["klee_witness"] = str(artifact_dir / "klee-witness.txt")

    max_attempts = max(1, int(getattr(args, "llm_testcase_retries", 3)))
    configured_max_attempts = max_attempts
    semantic_retry_used = False
    format_retry_used = False
    requested_context_mode = str(getattr(args, "llm_context", "slice"))
    retry_feedback: str | None = None
    rerun_ll: Path | None = None
    build_log_path: Path | None = None
    last_error: str | None = None
    rerun_result: dict[str, Any] | None = None
    last_testcase: str | None = None
    last_built_testcase: str | None = None
    semantic_retry_feedback: str | None = None
    attempt = 1
    while attempt <= max_attempts:
        # A failed sliced attempt gets one conservative full-context attempt at
        # the end. Successful cases therefore pay only for the semantic slice.
        context_mode = (
            "full"
            if requested_context_mode == "slice"
            and retry_feedback is not None
            and attempt == max_attempts
            else requested_context_mode
        )
        attempt_state: dict[str, Any] = {
            "attempt": attempt,
            "context_mode": context_mode,
        }
        build_log_path = artifact_dir / f"testcase-build-attempt-{attempt}.log"
        build_started = False
        try:
            testcase = generate_safe_testcase(
                crate_dir=injected_dir,
                source_crate_dir=cargo_dir,
                target=target,
                rule=rule,
                chain=chain,
                report_path=report_json,
                model=args.model,
                artifacts_dir=artifact_dir,
                injection=injection,
                klee_witness=klee_witness,
                retry_feedback=retry_feedback,
                attempt=attempt,
                context_mode=context_mode,
            )
            last_testcase = testcase
            ensure_cargo_feature(injected_dir, injection.feature)
            injection = inject_testcase_at_callsite(
                crate_dir=injected_dir,
                target=target,
                testcase=testcase,
                injection=injection,
            )
            attempt_state.update(
                {
                    "source_path": str(injection.source_path),
                    "source_line": injection.line,
                }
            )
            logger.info(
                "[verify:testcase] attempt %d/%d source=%s:%d state=%s",
                attempt,
                max_attempts,
                injection.source_path,
                injection.line,
                injected_dir / ".unsat-test-injections.json",
            )

            if args.skip_rerun:
                attempt_state["status"] = "injected"
                pipeline_state["source_path"] = str(injection.source_path)
                pipeline_state["source_line"] = injection.line
                pipeline_state["llm_testcase_attempts"].append(attempt_state)
                (artifact_dir / "testcase-pipeline.json").write_text(
                    json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
                )
                logger.info("[verify:testcase] --skip-rerun set; stopping after testcase injection")
                return {
                    "status": "injected",
                    "returncode": 0,
                    "reported_callsite": False,
                    "dsl_sat": False,
                    "has_certain_symbol": False,
                    "full_rerun_passed": False,
                }

            rerun_ir_dir = artifact_dir / "rerun-ir"
            _clean_dir(rerun_ir_dir)
            build_started = True
            rerun_ll = ensure_linked_llvm_ir_file(
                cargo_dir=injected_dir,
                output_dir=rerun_ir_dir,
                rustc=args.rustc,
                test=False,
                build_std=True,
                panic_abort=True,
                force=True,
                features=[injection.feature],
                build_log_path=build_log_path,
            )
            shutil.copyfile(build_log_path, artifact_dir / "testcase-build.log")
            last_built_testcase = testcase
            attempt_state.update(
                {
                    "status": "build-ok",
                    "build_log": str(build_log_path),
                    "llvm_ir": str(rerun_ll),
                }
            )

            rerun_log_path = artifact_dir / f"klee-rerun-attempt-{attempt}.log"
            rerun_result = _run_klee_compose_rerun(
                ll_path=rerun_ll,
                callsite_id=callsite_id,
                ast_json=ast_json,
                klee_bin=args.klee_bin,
                output_dir=artifact_dir / "klee-rerun",
                entry_function=injection.function,
                log_path=rerun_log_path,
                timeout_sec=args.timeout_sec,
                report_json=report_json,
                raw_ptr_deref=(rule_id == RAW_PTR_DEREF_RULE),
            )
            shutil.copyfile(rerun_log_path, artifact_dir / "klee-rerun.log")
            attempt_state["rerun"] = rerun_result
            attempt_state["rerun_log"] = str(rerun_log_path)
            attempt_state["status"] = (
                "reproduced"
                if rerun_result.get("full_rerun_passed")
                else "rerun-miss"
            )
            pipeline_state["llm_testcase_attempts"].append(attempt_state)
            if rerun_result.get("full_rerun_passed"):
                break

            guidance = _testcase_retry_guidance(
                rerun_result=rerun_result,
                callsite_id=callsite_id,
                target=target,
            )
            observed_feedback = _observed_target_arg_feedback(
                rerun_log_path,
                ast_json,
                target=target,
                crate_dir=cargo_dir,
            )
            if observed_feedback:
                guidance = guidance + "\n\n" + observed_feedback
            retry_feedback = _read_rerun_failure_feedback(
                rerun_log_path,
                guidance=guidance,
            )
            semantic_retry_feedback = _merge_semantic_retry_feedback(
                semantic_retry_feedback,
                retry_feedback,
            )
            last_error = retry_feedback
            attempt_state["feedback"] = retry_feedback
            (artifact_dir / "testcase-pipeline.json").write_text(
                json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
            )
            if (
                attempt >= max_attempts
                and not semantic_retry_used
                and _should_use_semantic_retry(rerun_result)
            ):
                semantic_retry_used = True
                max_attempts += 1
                pipeline_state["llm_semantic_retry_used"] = True
                pipeline_state["llm_configured_testcase_retries"] = configured_max_attempts
                logger.warning(
                    "[verify:testcase] attempt %d/%d reached the callsite but "
                    "did not reproduce the target counterexample; adding one "
                    "semantic retry with KLEE feedback",
                    attempt,
                    configured_max_attempts,
                )
            if attempt < max_attempts:
                logger.warning(
                    "[verify:testcase] attempt %d/%d compiled but did not reproduce; "
                    "retrying with KLEE feedback",
                    attempt,
                    max_attempts,
                )
        except Exception as exc:
            retry_feedback = _read_failure_feedback(
                build_log_path if build_started else None,
                exc,
            )
            if semantic_retry_feedback:
                retry_feedback = (
                    retry_feedback
                    + "\n\n<still required semantic retry feedback from the "
                    "last KLEE rerun>\n"
                    + semantic_retry_feedback
                    + "\n</still required semantic retry feedback from the "
                    "last KLEE rerun>"
                )
            last_error = retry_feedback
            attempt_state.update(
                {
                    "status": "error",
                    "error": str(exc),
                    "feedback": retry_feedback,
                }
            )
            if build_started:
                attempt_state["build_log"] = str(build_log_path)
            pipeline_state["llm_testcase_attempts"].append(attempt_state)
            (artifact_dir / "testcase-pipeline.json").write_text(
                json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
            )
            if attempt >= max_attempts:
                if (
                    not format_retry_used
                    and _is_testcase_format_feedback(retry_feedback)
                ):
                    format_retry_used = True
                    max_attempts += 1
                    pipeline_state["llm_format_retry_used"] = True
                    pipeline_state["llm_configured_testcase_retries"] = configured_max_attempts
                    logger.warning(
                        "[verify:testcase] attempt %d/%d produced malformed "
                        "testcase output; adding one formatting retry with "
                        "validator feedback",
                        attempt,
                        configured_max_attempts,
                    )
                    attempt += 1
                    continue
                if (
                    rerun_ll is not None
                    and rerun_result is not None
                    and last_built_testcase is not None
                    and not getattr(args, "skip_rerun_sym", False)
                ):
                    logger.warning(
                        "[verify:testcase] attempt %d/%d failed after a prior "
                        "testcase compiled; proceeding to rerun-sym with the "
                        "last compiled testcase",
                        attempt,
                        max_attempts,
                    )
                    break
                raise RuntimeError(
                    f"testcase generation/build failed after {max_attempts} attempts:\n"
                    f"{last_error or exc}"
                ) from exc
            logger.warning(
                "[verify:testcase] attempt %d/%d failed; retrying with compiler feedback",
                attempt,
                max_attempts,
            )
        attempt += 1

    if rerun_ll is None:
        raise RuntimeError("testcase build did not produce LLVM IR")

    if (
        rerun_result is not None
        and not rerun_result.get("full_rerun_passed")
        and not getattr(args, "skip_rerun_sym", False)
    ):
        rerun_sym_state: dict[str, Any] = {
            "status": "started",
            "mode": "rerun-sym",
        }
        try:
            testcase_for_sym = last_built_testcase or last_testcase
            if testcase_for_sym is None:
                raise RuntimeError("no generated testcase is available for rerun-sym")
            rerun_sym_focus_text = ""
            prompt_path = artifact_dir / "testcase-prompt.txt"
            if prompt_path.is_file():
                rerun_sym_focus_text = prompt_path.read_text(
                    encoding="utf-8", errors="replace"
                )
            if retry_feedback:
                rerun_sym_focus_text += "\n\n" + retry_feedback
            sym_testcase, sym_map = symbolize_testcase_constants(
                testcase=testcase_for_sym,
                injection=injection,
                focus_text=rerun_sym_focus_text,
            )
            mapping_path = artifact_dir / "rerun-sym-constants.json"
            mapping_path.write_text(
                json.dumps(sym_map, indent=2, ensure_ascii=False) + "\n",
                encoding="utf-8",
            )
            sym_testcase_path = artifact_dir / "testcase-rerun-sym.rs"
            sym_testcase_path.write_text(sym_testcase + "\n", encoding="utf-8")
            for item in sym_map.get("symbols", []):
                if isinstance(item, dict):
                    logger.info(
                        "[verify:rerun-sym] constant %s at body %s:%s -> certain symbol %s",
                        item.get("literal"),
                        item.get("body_line"),
                        item.get("body_col"),
                        item.get("name"),
                    )
            injection = inject_testcase_at_callsite(
                crate_dir=injected_dir,
                target=target,
                testcase=sym_testcase,
                injection=injection,
            )
            rerun_sym_state.update(
                {
                    "status": "symbolized",
                    "source_path": str(injection.source_path),
                    "source_line": injection.line,
                    "testcase": str(sym_testcase_path),
                    "constants": str(mapping_path),
                    "symbol_count": sym_map.get("symbol_count"),
                }
            )
            rerun_sym_ir_dir = artifact_dir / "rerun-sym-ir"
            rerun_sym_build_log = artifact_dir / "testcase-build-rerun-sym.log"
            _clean_dir(rerun_sym_ir_dir)
            rerun_sym_ll = ensure_linked_llvm_ir_file(
                cargo_dir=injected_dir,
                output_dir=rerun_sym_ir_dir,
                rustc=args.rustc,
                test=False,
                build_std=True,
                panic_abort=True,
                force=True,
                features=[injection.feature],
                build_log_path=rerun_sym_build_log,
            )
            rerun_sym_log_path = artifact_dir / "klee-rerun-sym.log"
            rerun_sym_result = _run_klee_compose_rerun(
                ll_path=rerun_sym_ll,
                callsite_id=callsite_id,
                ast_json=ast_json,
                klee_bin=args.klee_bin,
                output_dir=artifact_dir / "klee-rerun-sym",
                entry_function=injection.function,
                log_path=rerun_sym_log_path,
                timeout_sec=args.timeout_sec,
                report_json=report_json,
                raw_ptr_deref=(rule_id == RAW_PTR_DEREF_RULE),
                rerun_sym=True,
            )
            rerun_sym_state.update(
                {
                    "status": (
                        "reproduced"
                        if rerun_sym_result.get("full_rerun_passed")
                        else "rerun-miss"
                    ),
                    "build_log": str(rerun_sym_build_log),
                    "llvm_ir": str(rerun_sym_ll),
                    "klee_rerun_sym_dir": str(artifact_dir / "klee-rerun-sym"),
                    "klee_rerun_sym_log": str(rerun_sym_log_path),
                    "rerun": rerun_sym_result,
                }
            )
            pipeline_state["rerun_sym"] = rerun_sym_state
            if rerun_sym_result.get("full_rerun_passed"):
                rerun_result = rerun_sym_result
        except Exception as exc:
            rerun_sym_state.update(
                {
                    "status": "error",
                    "error": str(exc),
                }
            )
            pipeline_state["rerun_sym"] = rerun_sym_state
            logger.warning("[verify:rerun-sym] failed: %s", exc)
        (artifact_dir / "testcase-pipeline.json").write_text(
            json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
        )

    pipeline_state.update(
        {
            "source_path": str(injection.source_path),
            "source_line": injection.line,
            "llvm_ir": str(rerun_ll),
            "build_log": str(artifact_dir / "testcase-build.log"),
            "testcase_prompt": str(artifact_dir / "testcase-prompt.txt"),
            "testcase_response": str(artifact_dir / "testcase-response.txt"),
            "testcase_context_stats": str(
                artifact_dir / "testcase-context-stats.json"
            ),
            "klee_rerun_dir": str(artifact_dir / "klee-rerun"),
            "klee_rerun_log": str(artifact_dir / "klee-rerun.log"),
        }
    )
    (artifact_dir / "testcase-pipeline.json").write_text(
        json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
    )
    if rerun_result is None:
        raise RuntimeError(
            "testcase build succeeded but compose-rerun did not produce a result"
        )
    pipeline_state["rerun_returncode"] = rerun_result.get("returncode")
    pipeline_state["rerun"] = rerun_result
    (artifact_dir / "testcase-pipeline.json").write_text(
        json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
    )
    return rerun_result


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
        requested_callsites=_load_callsites_file(repo_root, getattr(args, "callsites_file", None)),
    )
    if not callsites:
        raise RuntimeError(
            f"no callsites matched --callsite={args.callsite!r} "
            f"--callsites-file={getattr(args, 'callsites_file', None)!r}"
        )
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

    result_path = _matrix_result_path(repo_root, args, cargo_dir)
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
    artifact_root = _matrix_artifact_root(repo_root, args, cargo_dir, result_path)
    artifact_root.mkdir(parents=True, exist_ok=True)

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
        "stop_callsite_if_violated": bool(getattr(args, "stop_callsite_if_violated", False)),
        "stop_callsite_if_reached": bool(getattr(args, "stop_callsite_if_reached", False)),
        "stop_callsite_after_timeout": bool(getattr(args, "stop_callsite_after_timeout", False)),
        "total": total,
        "completed": len(results),
        "counts": dict(sorted(counts.items())),
        "callsites": [
            {key: value for key, value in row.items() if key != "target"}
            for row in callsites
        ],
        "rules": all_rules,
        "rules_by_callsite": rules_by_callsite,
        "results": sorted(results, key=_matrix_sort_key),
    }
    _atomic_write_json(result_path, state)
    print(f"[verify:matrix] callsites={len(callsites)} matched-rules={len(all_rules)} total={total}")
    print(f"[verify:matrix] result-json={result_path}")
    print(f"[verify:matrix] artifact-root={artifact_root}")

    operators = _load_operator_entries(repo_root)
    ast_cache: dict[str, str] = {}
    completed = len(results)
    target_by_index = {
        int(row["target_index"]): row["target"]
        for row in callsites
        if isinstance(row.get("target_index"), int) and isinstance(row.get("target"), dict)
    }
    stopped_callsites: set[str] = set()
    if (
        getattr(args, "stop_callsite_if_violated", False)
        or getattr(args, "stop_callsite_if_reached", False)
        or getattr(args, "stop_callsite_after_timeout", False)
    ):
        for item in existing.values():
            status = item.get("status")
            should_stop = (
                getattr(args, "stop_callsite_if_violated", False)
                and _is_confirmed_callsite_violation(item)
            ) or (
                getattr(args, "stop_callsite_if_reached", False)
                and _is_reached_callsite_status(status)
            ) or (
                getattr(args, "stop_callsite_after_timeout", False)
                and status == "timeout"
            )
            if should_stop:
                callsite_name = item.get("callsite")
                if isinstance(callsite_name, str):
                    stopped_callsites.add(callsite_name)

    for callsite in callsites:
        target_index = int(callsite["target_index"])
        callsite_id = str(callsite["callsite_id"])
        llvm_callsite_id = str(callsite.get("llvm_callsite_id") or callsite_id)
        for rule in callsite.get("rules", []):
            if (target_index, rule) in existing:
                continue
            if callsite_id in stopped_callsites:
                skip_reason = "earlier rule for this callsite triggered matrix early-stop"
                row = {
                    "target_index": target_index,
                    "callsite": callsite_id,
                    "llvm_callsite": llvm_callsite_id,
                    "path": callsite.get("path"),
                    "line": callsite.get("line"),
                    "col": callsite.get("col"),
                    "caller": callsite.get("caller"),
                    "unsafe_callee": callsite.get("unsafe_callee"),
                    "unsafe_callee_path": callsite.get("unsafe_callee_path"),
                    "unsafe_callee_line_start": callsite.get("unsafe_callee_line_start"),
                    "rule": rule,
                    "status": "skipped-callsite-violated",
                    "returncode": None,
                    "timed_out": False,
                    "duration_sec": 0.0,
                    "log": None,
                    "klee_output_dir": None,
                    "skip_reason": skip_reason,
                }
                pair_path = _pair_result_path_for_matrix(result_path, callsite_id, str(rule))
                _write_pair_result(
                    path=pair_path,
                    crate_dir=cargo_dir,
                    injected_dir=injected_dir,
                    meta_path=meta_path,
                    rule_dsl_path=rule_dsl_path,
                    callsite=_callsite_summary_from_row(row),
                    rule_id=str(rule),
                    init={
                        "status": "skipped-callsite-violated",
                        "returncode": None,
                        "timed_out": False,
                        "duration_sec": 0.0,
                        "log": None,
                        "klee_output_dir": None,
                        "skip_reason": skip_reason,
                    },
                    llm={"status": "skipped", "reason": skip_reason},
                    rerun={"status": "skipped", "full_rerun_passed": False},
                )
                row["pair_result_json"] = str(pair_path)
                results.append(row)
                completed += 1
                counts["skipped-callsite-violated"] = counts.get("skipped-callsite-violated", 0) + 1
                state.update(
                    {
                        "updated_at": _utc_now(),
                        "completed": completed,
                        "counts": dict(sorted(counts.items())),
                        "results": sorted(results, key=_matrix_sort_key),
                    }
                )
                _atomic_write_json(result_path, state)
                print(
                    f"[verify:matrix] {completed}/{total} target#{target_index} "
                    f"{callsite_id} {rule} skipped: callsite early-stop",
                    flush=True,
                )
                continue
            if not callsite.get("present_in_llvm_ir"):
                row = {
                    "target_index": target_index,
                    "callsite": callsite_id,
                    "llvm_callsite": llvm_callsite_id,
                    "path": callsite.get("path"),
                    "line": callsite.get("line"),
                    "col": callsite.get("col"),
                    "caller": callsite.get("caller"),
                    "unsafe_callee": callsite.get("unsafe_callee"),
                    "unsafe_callee_path": callsite.get("unsafe_callee_path"),
                    "unsafe_callee_line_start": callsite.get("unsafe_callee_line_start"),
                    "rule": rule,
                    "status": "not-in-ir",
                    "returncode": None,
                    "timed_out": False,
                    "duration_sec": 0.0,
                    "log": None,
                    "klee_output_dir": None,
                }
                pair_path = _pair_result_path_for_matrix(result_path, callsite_id, str(rule))
                _write_pair_result(
                    path=pair_path,
                    crate_dir=cargo_dir,
                    injected_dir=injected_dir,
                    meta_path=meta_path,
                    rule_dsl_path=rule_dsl_path,
                    callsite=_callsite_summary_from_row(row),
                    rule_id=str(rule),
                    init={
                        "status": "not-in-ir",
                        "returncode": None,
                        "timed_out": False,
                        "duration_sec": 0.0,
                        "log": None,
                        "klee_output_dir": None,
                    },
                )
                row["pair_result_json"] = str(pair_path)
                results.append(row)
                completed += 1
                counts["not-in-ir"] = counts.get("not-in-ir", 0) + 1
                state.update(
                    {
                        "updated_at": _utc_now(),
                        "completed": completed,
                        "counts": dict(sorted(counts.items())),
                        "results": sorted(results, key=_matrix_sort_key),
                    }
                )
                _atomic_write_json(result_path, state)
                continue

            raw_ptr_deref = rule == RAW_PTR_DEREF_RULE
            if not raw_ptr_deref and rule not in ast_cache:
                ast_cache[rule] = _task1_to_ext_ast_json(
                    _load_rule_dsl(rule_dsl_path, rule), operators
                )
            artifact_dir = _pair_artifact_dir(artifact_root, callsite_id, str(rule))
            output_dir = artifact_dir / "klee-out"
            log_path = artifact_dir / "klee.log"
            if output_dir.exists():
                shutil.rmtree(output_dir)
            output_dir.parent.mkdir(parents=True, exist_ok=True)
            cmd = [
                args.klee_bin,
                *_klee_no_stats_args(),
                f"--output-dir={output_dir}",
                f"--compose-verify-chain-json={output_dir.parent / 'klee-control-chains.json'}",
                f"--ext.callsite={llvm_callsite_id}",
                "--compose-verify",
                f"--compose-verify-symbolic-loop-bound={args.compose_loop_bound}",
            ]
            if raw_ptr_deref:
                cmd.append("--ext.raw-ptr-deref")
            else:
                cmd.append(f"--ext.dsl={ast_cache[rule]}")
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
            status = _compose_status_from_output(
                returncode, combined, timed_out, llvm_callsite_id
            )
            chain_path = output_dir.parent / "klee-control-chains.json"
            has_certain_symbol = _control_chain_has_certain_symbol(chain_path)
            if "[ext.dsl] resolved constraint uses certain symbol: true" in combined:
                has_certain_symbol = True
            if "[ext.dsl] violation query uses certain symbol: true" in combined:
                has_certain_symbol = True
            if raw_ptr_deref and "pointer uses certain symbol: true" in combined:
                has_certain_symbol = True
            if status == "violation" and not has_certain_symbol and rule not in REACHABILITY_ONLY_RULES:
                status = "low-confidence-sat"
            row = {
                "target_index": target_index,
                "callsite": callsite_id,
                "llvm_callsite": llvm_callsite_id,
                "path": callsite.get("path"),
                "line": callsite.get("line"),
                "col": callsite.get("col"),
                "caller": callsite.get("caller"),
                "unsafe_callee": callsite.get("unsafe_callee"),
                "unsafe_callee_path": callsite.get("unsafe_callee_path"),
                "unsafe_callee_line_start": callsite.get("unsafe_callee_line_start"),
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
            pair_path = _pair_result_path_for_matrix(result_path, callsite_id, str(rule))
            _write_pair_result(
                path=pair_path,
                crate_dir=cargo_dir,
                injected_dir=injected_dir,
                meta_path=meta_path,
                rule_dsl_path=rule_dsl_path,
                callsite=_callsite_summary_from_row(row),
                rule_id=str(rule),
                init={
                    "status": status,
                    "returncode": returncode,
                    "timed_out": timed_out,
                    "duration_sec": row["duration_sec"],
                    "log": str(log_path),
                    "klee_output_dir": str(output_dir),
                    "certainty_chain": str(chain_path),
                    "has_certain_symbol": has_certain_symbol,
                },
            )
            row["pair_result_json"] = str(pair_path)
            results.append(row)
            completed += 1
            counts[status] = counts.get(status, 0) + 1
            if (
                (getattr(args, "stop_callsite_if_violated", False) and status == "violation")
                or (
                    getattr(args, "stop_callsite_if_reached", False)
                    and _is_reached_callsite_status(status)
                )
                or (getattr(args, "stop_callsite_after_timeout", False) and status == "timeout")
            ):
                stopped_callsites.add(callsite_id)
            state.update(
                {
                    "updated_at": _utc_now(),
                    "completed": completed,
                    "counts": dict(sorted(counts.items())),
                    "results": sorted(results, key=_matrix_sort_key),
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
        raw_ptr_deref = rule_id == RAW_PTR_DEREF_RULE
        if not raw_ptr_deref and rule_id not in ast_cache:
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
            rerun_result = _run_llm_testcase_pipeline(
                args=args,
                repo_root=repo_root,
                cargo_dir=cargo_dir,
                injected_dir=injected_dir,
                rule_dsl_path=rule_dsl_path,
                target=target,
                callsite_id=callsite_id,
                rule_id=rule_id,
                ast_json=ast_cache.get(rule_id, ""),
                artifact_dir=artifact_dir,
                compose_output=artifact_dir / "klee-out",
                report_json=report_json,
            )
            pipeline_path = artifact_dir / "testcase-pipeline.json"
            pipeline_state = _load_json(pipeline_path) if pipeline_path.is_file() else {}
            pipeline_skipped = rerun_result.get("status") in {"skipped", "injected"}
            row["llm_testcase_status"] = (
                "ok" if rerun_result.get("full_rerun_passed")
                else str(rerun_result.get("status")) if pipeline_skipped
                else "rerun-error"
            )
            row["llm_testcase_returncode"] = rerun_result.get("returncode")
            row["rerun_reported_callsite"] = bool(rerun_result.get("reported_callsite"))
            row["rerun_dsl_sat"] = bool(rerun_result.get("dsl_sat"))
            row["rerun_has_certain_symbol"] = bool(rerun_result.get("has_certain_symbol"))
            row["full_rerun_passed"] = bool(rerun_result.get("full_rerun_passed"))
            pair_path = _pair_result_path_for_matrix(
                result_path, str(row.get("callsite") or callsite_id), rule_id
            )
            row["pair_result_json"] = str(pair_path)
            _write_pair_result(
                path=pair_path,
                crate_dir=cargo_dir,
                injected_dir=injected_dir,
                meta_path=meta_path,
                rule_dsl_path=rule_dsl_path,
                callsite=_callsite_summary_from_row(row),
                rule_id=rule_id,
                init={
                    "status": row.get("status"),
                    "returncode": row.get("returncode"),
                    "timed_out": row.get("timed_out"),
                    "duration_sec": row.get("duration_sec"),
                    "log": row.get("log"),
                    "klee_output_dir": row.get("klee_output_dir"),
                    "certainty_chain": row.get("certainty_chain"),
                    "has_certain_symbol": row.get("has_certain_symbol"),
                },
                llm=_stage_paths_from_pipeline(pipeline_state if isinstance(pipeline_state, dict) else {}),
                rerun={
                    **rerun_result,
                    **_rerun_paths_for_result(artifact_dir, rerun_result),
                },
            )
            if not rerun_result.get("full_rerun_passed") and not pipeline_skipped:
                llm_failures += 1
        except Exception as exc:
            row["llm_testcase_status"] = "error"
            row["llm_testcase_error"] = str(exc)
            pair_path = _pair_result_path_for_matrix(
                result_path, str(row.get("callsite") or callsite_id), str(row.get("rule") or "")
            )
            row["pair_result_json"] = str(pair_path)
            _write_pair_result(
                path=pair_path,
                crate_dir=cargo_dir,
                injected_dir=injected_dir,
                meta_path=meta_path,
                rule_dsl_path=rule_dsl_path,
                callsite=_callsite_summary_from_row(row),
                rule_id=rule_id if isinstance(rule_id, str) else str(row.get("rule") or ""),
                init={
                    "status": row.get("status"),
                    "returncode": row.get("returncode"),
                    "timed_out": row.get("timed_out"),
                    "duration_sec": row.get("duration_sec"),
                    "log": row.get("log"),
                    "klee_output_dir": row.get("klee_output_dir"),
                    "certainty_chain": row.get("certainty_chain"),
                    "has_certain_symbol": row.get("has_certain_symbol"),
                },
                llm={"status": "error", "error": str(exc), "artifact_dir": str(artifact_dir)},
                rerun={"status": "not-run", "full_rerun_passed": False},
            )
            llm_failures += 1
        state.update(
            {
                "updated_at": _utc_now(),
                "results": sorted(results, key=_matrix_sort_key),
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
    if args.llm_testcase_retries <= 0:
        raise RuntimeError("--llm-testcase-retries must be positive")

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
        callsite_count = _validate_callsite_bodies(meta_path=meta_path, ll_path=ll_path)
        print(f"[verify] crate={cargo_dir}")
        print(f"[verify] injected-crate={injected_dir}")
        print(f"[verify] llvm-ir={ll_path}")
        print(f"[verify] llvm-callsite-bodies={callsite_count}/{callsite_count}")
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
    compose_callsite_id = _compose_init_callsite_marker_for_target(
        ll_path=ll_path,
        target=target,
        callsite_id=resolved_callsite_id,
    )
    compose_reach_only = compose_callsite_id != resolved_callsite_id
    if compose_reach_only:
        print(
            f"[verify] compose-init-callsite={compose_callsite_id} "
            f"(reach/chain for actual unsafe callsite {resolved_callsite_id})"
        )
    compose_output = artifact_dir / "klee-compose"
    rc, compose_status = _run_klee_compose_verify(
        ll_path=ll_path,
        callsite_id=compose_callsite_id,
        ast_json=ast_json,
        report_json=report_json,
        klee_bin=args.klee_bin,
        compose_loop_bound=args.compose_loop_bound,
        output_dir=compose_output,
        timeout_sec=args.timeout_sec,
    )
    if compose_reach_only and compose_status in {"candidate", "verified", "unknown"}:
        # A root marker proves reachability/context for testcase generation, not
        # absence of an actual unsafe-call violation.  Keep going to LLM/rerun,
        # which uses resolved_callsite_id at the actual unsafe marker.
        compose_status = "candidate"
    if rc != 0 and compose_status not in {"candidate", "verified"}:
        return rc

    if compose_status == "verified" and not compose_reach_only:
        pipeline_state = {
            "schema_version": 1,
            "callsite": resolved_callsite_id,
            "compose_callsite": compose_callsite_id,
            "rule": args.rule,
            "status": "compose-verified-skip-testcase",
            "reason": "compose-verify proved constraints AND NOT(rule) unsat",
            "klee_compose_log": str(artifact_dir / "klee-compose.log"),
        }
        (artifact_dir / "testcase-pipeline.json").write_text(
            json.dumps(pipeline_state, indent=2) + "\n", encoding="utf-8"
        )
        _write_pair_result(
            path=_pair_result_path_for_artifacts(artifact_dir, resolved_callsite_id, args.rule),
            crate_dir=cargo_dir,
            injected_dir=injected_dir,
            meta_path=meta_path,
            rule_dsl_path=rule_dsl_path,
            callsite={
                "id": resolved_callsite_id,
                "report_id": report_callsite_id,
                **_callsite_summary_from_row(
                    {
                        "target_index": None,
                        "callsite": resolved_callsite_id,
                        "llvm_callsite": resolved_callsite_id,
                        "path": (target.get("callsite") or {}).get("path") if isinstance(target.get("callsite"), dict) else None,
                        "line": (target.get("callsite") or {}).get("line") if isinstance(target.get("callsite"), dict) else None,
                        "col": (target.get("callsite") or {}).get("col") if isinstance(target.get("callsite"), dict) else None,
                    }
                ),
            },
            rule_id=args.rule,
            init={
                "status": "verified",
                "returncode": rc,
                "compose_callsite": compose_callsite_id,
                "log": str(artifact_dir / "klee-compose.log"),
                "klee_output_dir": str(compose_output),
            },
            llm={"status": "skipped"},
            rerun={"status": "skipped", "full_rerun_passed": False},
        )
        logger.info(
            "[verify] compose result is unsat for NOT(rule); skipping testcase generation/rerun"
        )
        return 0

    rerun_result = _run_llm_testcase_pipeline(
        args=args,
        repo_root=repo_root,
        cargo_dir=cargo_dir,
        injected_dir=injected_dir,
        rule_dsl_path=rule_dsl_path,
        target=target,
        callsite_id=resolved_callsite_id,
        chain_callsite_id=compose_callsite_id,
        rule_id=args.rule,
        ast_json=ast_json,
        artifact_dir=artifact_dir,
        compose_output=compose_output,
        report_json=report_json,
    )
    pipeline_path = artifact_dir / "testcase-pipeline.json"
    pipeline_state = _load_json(pipeline_path) if pipeline_path.is_file() else {}
    callsite_obj = target.get("callsite") if isinstance(target.get("callsite"), dict) else {}
    caller_obj = target.get("caller") if isinstance(target.get("caller"), dict) else {}
    callee_obj = target.get("callee") if isinstance(target.get("callee"), dict) else {}
    if not callee_obj and isinstance(target.get("unsafe_callee"), dict):
        callee_obj = target.get("unsafe_callee")  # type: ignore[assignment]
    _write_pair_result(
        path=_pair_result_path_for_artifacts(artifact_dir, resolved_callsite_id, args.rule),
        crate_dir=cargo_dir,
        injected_dir=injected_dir,
        meta_path=meta_path,
        rule_dsl_path=rule_dsl_path,
        callsite={
            "id": resolved_callsite_id,
            "report_id": report_callsite_id,
            "llvm_id": resolved_callsite_id,
            "path": callsite_obj.get("path"),
            "line": callsite_obj.get("line"),
            "col": callsite_obj.get("col"),
            "caller": caller_obj.get("name"),
            "unsafe_callee": callee_obj.get("name"),
            "unsafe_callee_path": callee_obj.get("path"),
            "unsafe_callee_line_start": callee_obj.get("line_start"),
        },
        rule_id=args.rule,
        init={
            "status": compose_status,
            "returncode": rc,
            "compose_callsite": compose_callsite_id,
            "log": str(artifact_dir / "klee-compose.log"),
            "klee_output_dir": str(compose_output),
        },
        llm=_stage_paths_from_pipeline(pipeline_state if isinstance(pipeline_state, dict) else {}),
        rerun={
            **rerun_result,
            **_rerun_paths_for_result(artifact_dir, rerun_result),
        },
    )
    if rerun_result.get("full_rerun_passed") or rerun_result.get("status") in {"skipped", "injected"}:
        return 0
    returncode = rerun_result.get("returncode")
    if isinstance(returncode, int) and returncode != 0:
        return returncode
    return 2
