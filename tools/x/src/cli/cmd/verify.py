import argparse
import json
import subprocess
from pathlib import Path

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
from cli.cmd.sync import ensure_crate_metadata_file


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


def _run_klee_compose_verify(
    *,
    ll_path: Path,
    callsite_id: str,
    ast_json: str,
    klee_bin: str,
) -> int:
    cmd = [
        klee_bin,
        f"--ext.callsite={callsite_id}",
        f"--ext.dsl={ast_json}",
        "--compose-verify",
        str(ll_path),
    ]
    print(f"[verify] running: {' '.join(cmd)}")
    result = subprocess.run(cmd, check=False)
    return result.returncode


def run(args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()

    cargo_dir = Path(args.cargo_dir)
    if not cargo_dir.is_absolute():
        cargo_dir = (repo_root / cargo_dir).resolve()
    else:
        cargo_dir = cargo_dir.resolve()

    if not cargo_dir.is_dir() or not (cargo_dir / "Cargo.toml").is_file():
        raise RuntimeError(f"invalid crate directory: {cargo_dir}")

    studied_rules = _resolve_path(repo_root, args.studied_rules, "studied_rules")
    meta_path = ensure_crate_metadata_file(
        repo_root,
        cargo_dir,
        studied_rules=studied_rules,
        force=False,
    )

    ir_output_dir = _resolve_path(repo_root, args.ir_output_dir, ".local/irs")
    ll_path = ensure_linked_llvm_ir_file(
        cargo_dir=cargo_dir,
        output_dir=ir_output_dir,
        rustc=args.rustc,
        test=args.test,
        build_std=True,
        force=False,
    )

    current_meta = _load_json(meta_path)
    report = current_meta.get("report")
    if not isinstance(report, dict):
        raise RuntimeError(f"missing report object in {meta_path}")
    targets = report.get("targets")
    if not isinstance(targets, list):
        raise RuntimeError(f"missing targets in {meta_path}")

    target, resolved_callsite_id = _find_target(targets, args.callsite)
    callsite_key = _target_callsite_key(target, 0) if target is not None else None
    task1 = _load_rule_task1(
        repo_root=repo_root,
        meta_path=meta_path,
        rule_dir=_resolve_path(repo_root, args.rule_dir, "human"),
        callsite_id=resolved_callsite_id,
        callsite_key=callsite_key,
        rule_id=args.rule,
    )

    operators = _load_operator_entries(repo_root)
    ast_json = _task1_to_ext_ast_json(task1, operators)

    print(f"[verify] crate={cargo_dir}")
    print(f"[verify] llvm-ir={ll_path}")
    print(f"[verify] callsite={resolved_callsite_id} rule={args.rule}")
    return _run_klee_compose_verify(
        ll_path=ll_path,
        callsite_id=resolved_callsite_id,
        ast_json=ast_json,
        klee_bin=args.klee_bin,
    )
