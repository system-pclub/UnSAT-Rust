import argparse
import json
import subprocess
from pathlib import Path
import re

from dsl import (
    BinaryExpression,
    CallExpression,
    Identifier,
    Literal,
    SourceRef,
    UnaryExpression,
    parse_dsl,
)
from dsl.simplifier import SimplifiedVariable, simplify_variables

from cli.cmd.llvmir import ensure_linked_llvm_ir_file
from cli.cmd.sync import ensure_crate_metadata_file


PLACEHOLDER = "<placeholder>"
HUMAN_PLACEHOLDER = "placeholder"
TASK_NAMES = ("task1", "task2", "task3")


def _find_repo_root() -> Path:
    start = Path.cwd().resolve()
    for directory in [start, *start.parents]:
        if (directory / "crates").is_dir():
            return directory
    raise RuntimeError(
        f"could not locate repository root from {start} (expected an ancestor containing crates/)"
    )


def _load_json_value(path: Path) -> object:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"invalid JSON in {path}: {exc}") from exc
    return data


def _load_json(path: Path) -> dict[str, object]:
    data = _load_json_value(path)
    if not isinstance(data, dict):
        raise RuntimeError(f"expected JSON object in {path}")
    return data


def _load_task_folder(path: Path) -> dict[str, object]:
    if not path.is_dir():
        raise RuntimeError(f"expected task folder: {path}")

    loaded: dict[str, object] = {}
    for callsite_dir in sorted(child for child in path.iterdir() if child.is_dir()):
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
    return loaded


def _load_meta_like(path: Path) -> dict[str, object]:
    if path.is_dir():
        return _load_task_folder(path)
    if path.is_file():
        return _load_json(path)
    raise RuntimeError(f"metadata input does not exist: {path}")


def _load_operator_entries(repo_root: Path) -> list[dict[str, object]]:
    operators_path = repo_root / "operators.json"
    if not operators_path.is_file():
        return []

    data = _load_json_value(operators_path)
    raw_entries: object
    if isinstance(data, list):
        raw_entries = data
    else:
        raw_entries = data.get("operators")
        if not isinstance(raw_entries, list):
            raise RuntimeError(f"operators.json object must contain an operators array: {operators_path}")

    entries: list[dict[str, object]] = []
    for item in raw_entries:
        if isinstance(item, dict):
            entries.append(dict(item))
    return entries


def _target_callsite_key(target: dict[str, object], fallback_index: int) -> str:
    caller = target.get("caller")
    callsite = target.get("callsite")
    if not isinstance(caller, dict) or not isinstance(callsite, dict):
        return str(fallback_index)

    path = caller.get("path")
    line = callsite.get("line")
    col = callsite.get("col")
    if isinstance(path, str) and isinstance(line, int) and isinstance(col, int):
        return f"{path}:{line}:{col}"
    return str(fallback_index)


def _build_meta_task1_index(meta_like: dict[str, object]) -> tuple[dict[tuple[str, str], str], dict[str, str]]:
    by_callsite_rule: dict[tuple[str, str], str] = {}
    by_rule: dict[str, str] = {}

    def _task1_text(value: object) -> str | None:
        if isinstance(value, str):
            normalized = value.strip()
            if normalized and normalized not in {PLACEHOLDER, HUMAN_PLACEHOLDER}:
                return value
            return None
        if isinstance(value, dict):
            task1 = value.get("task1")
            if isinstance(task1, str):
                normalized = task1.strip()
                if normalized and normalized not in {PLACEHOLDER, HUMAN_PLACEHOLDER}:
                    return task1
        return None

    report = meta_like.get("report")
    targets = None
    if isinstance(report, dict):
        targets = report.get("targets")
    elif isinstance(meta_like.get("targets"), list):
        targets = meta_like.get("targets")

    if isinstance(targets, list):
        for index, raw_target in enumerate(targets, start=1):
            if not isinstance(raw_target, dict):
                continue
            callsite_key = _target_callsite_key(raw_target, index)
            callsite_id = str(index)
            callsite = raw_target.get("callsite")
            if isinstance(callsite, dict):
                callsite_id_raw = callsite.get("id")
                if isinstance(callsite_id_raw, str) and callsite_id_raw:
                    callsite_id = callsite_id_raw
            rules = raw_target.get("rules")
            if not isinstance(rules, dict):
                continue
            for rule_id, tasks in rules.items():
                if not isinstance(rule_id, str):
                    continue
                task1 = _task1_text(tasks)
                if isinstance(task1, str):
                    by_callsite_rule[(callsite_id, rule_id)] = task1
                    if callsite_key != callsite_id:
                        by_callsite_rule[(callsite_key, rule_id)] = task1
                    by_rule.setdefault(rule_id, task1)
        return by_callsite_rule, by_rule

    for outer_key, outer_value in meta_like.items():
        if not isinstance(outer_key, str) or not isinstance(outer_value, dict):
            continue

        # Legacy shape: {"rule-1": {"task1": "..."}}
        task1 = _task1_text(outer_value)
        if isinstance(task1, str):
            by_rule[outer_key] = task1
            continue

        # New human shape: {"<callsite-id>": {"rule-1": "..."}}
        callsite_id = outer_key
        for rule_id, task_like in outer_value.items():
            if not isinstance(rule_id, str):
                continue
            task1 = _task1_text(task_like)
            if isinstance(task1, str):
                by_callsite_rule[(callsite_id, rule_id)] = task1
                by_rule.setdefault(rule_id, task1)

    return by_callsite_rule, by_rule


def _expr_to_ext_ast(expr: object) -> dict[str, object]:
    if isinstance(expr, SimplifiedVariable):
        return {"type": "simplified_var", "name": expr.name}
    if isinstance(expr, Literal):
        return {"type": "literal", "value": expr.value}
    if isinstance(expr, SourceRef):
        return {"type": "source_ref", "selector": f"{expr.name}@{expr.line}:{expr.column}"}
    if isinstance(expr, Identifier):
        return {"type": "identifier", "name": expr.name}
    if isinstance(expr, CallExpression):
        return {
            "type": "call",
            "name": expr.name,
            "args": [_expr_to_ext_ast(arg) for arg in expr.args],
        }
    if isinstance(expr, UnaryExpression):
        return {
            "type": "unary",
            "op": expr.operator,
            "operand": _expr_to_ext_ast(expr.operand),
        }
    if isinstance(expr, BinaryExpression):
        return {
            "type": "binary",
            "op": expr.operator,
            "left": _expr_to_ext_ast(expr.left),
            "right": _expr_to_ext_ast(expr.right),
        }
    raise RuntimeError(f"unsupported DSL node: {type(expr).__name__}")


def _smt_symbol(name: str) -> str:
    escaped = name.replace("\\", "\\\\").replace("|", "\\|")
    return f"|{escaped}|"


def _simplified_expr_to_smt2(expr: object) -> str:
    if isinstance(expr, SimplifiedVariable):
        return _smt_symbol(expr.name)
    if isinstance(expr, Literal):
        if isinstance(expr.value, bool):
            return "true" if expr.value else "false"
        if isinstance(expr.value, int):
            return str(expr.value)
        if expr.value is None:
            return "0"
        raise RuntimeError(f"cannot dump string literal to simplified SMT-LIB: {expr.value!r}")
    if isinstance(expr, UnaryExpression):
        operand = _simplified_expr_to_smt2(expr.operand)
        if expr.operator == "!":
            return f"(not {operand})"
        if expr.operator == "-":
            return f"(- {operand})"
        raise RuntimeError(f"unsupported unary operator in simplified SMT-LIB: {expr.operator!r}")
    if isinstance(expr, BinaryExpression):
        left = _simplified_expr_to_smt2(expr.left)
        right = _simplified_expr_to_smt2(expr.right)
        op = expr.operator
        if op == "&&":
            return f"(and {left} {right})"
        if op == "||":
            return f"(or {left} {right})"
        if op == "==":
            return f"(= {left} {right})"
        if op == "!=":
            return f"(not (= {left} {right}))"
        if op == "/":
            return f"(div {left} {right})"
        if op == "%":
            return f"(mod {left} {right})"
        if op in {"<", "<=", ">", ">=", "+", "-", "*"}:
            return f"({op} {left} {right})"
        raise RuntimeError(f"unsupported binary operator in simplified SMT-LIB: {op!r}")
    raise RuntimeError(f"unsupported simplified DSL node: {type(expr).__name__}")


def _collect_simplified_variables(expr: object) -> set[str]:
    if isinstance(expr, SimplifiedVariable):
        return {expr.name}
    if isinstance(expr, UnaryExpression):
        return _collect_simplified_variables(expr.operand)
    if isinstance(expr, BinaryExpression):
        return _collect_simplified_variables(expr.left) | _collect_simplified_variables(expr.right)
    return set()


def _write_simplified_smt2(expr: object, output_path: Path) -> None:
    names = sorted(_collect_simplified_variables(expr))
    lines = ["(set-logic ALL)"]
    lines.extend(f"(declare-const {_smt_symbol(name)} Int)" for name in names)
    lines.append(f"(assert {_simplified_expr_to_smt2(expr)})")
    lines.append("(check-sat)")
    lines.append("(exit)")
    output_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _run_klee_for_constraint(
    *,
    ll_path: Path,
    callsite_id: str,
    ast_json: str,
    output_path: Path,
    klee_bin: str,
) -> None:
    if output_path.exists():
        output_path.unlink()

    cmd = [
        klee_bin,
        f"--ext.callsite={callsite_id}",
        f"--ext.dsl={ast_json}",
        f"--dump-constraints-to-file={output_path}",
        str(ll_path),
    ]
    result = subprocess.run(cmd, check=False, capture_output=True, text=True)
    if result.returncode == 0 and output_path.is_file():
        return

    klee_logs = [
        (Path.cwd() / "klee-last" / "messages.txt").resolve(),
        (Path.cwd() / "klee-last" / "warnings.txt").resolve(),
    ]
    existing_logs = [str(path) for path in klee_logs if path.is_file()]
    log_hint = ", ".join(existing_logs) if existing_logs else str((Path.cwd() / "klee-last").resolve())

    raise RuntimeError(
        "klee failed:\n"
        f"cmd: {' '.join(cmd)}\n"
        f"exit: {result.returncode}\n"
        f"dump_exists: {output_path.is_file()}\n"
        f"dump_path: {output_path}\n"
        f"klee_log: {log_hint}"
    )


def _split_top_level_sexprs(text: str) -> list[str]:
    exprs: list[str] = []
    depth = 0
    start = -1
    in_string = False
    escape = False
    in_comment = False

    for i, ch in enumerate(text):
        if in_comment:
            if ch == "\n":
                in_comment = False
            continue

        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
            continue

        if ch == ";":
            in_comment = True
            continue
        if ch == '"':
            in_string = True
            continue

        if ch == "(":
            if depth == 0:
                start = i
            depth += 1
            continue

        if ch == ")":
            depth -= 1
            if depth == 0 and start >= 0:
                exprs.append(text[start : i + 1].strip())
                start = -1

    return exprs


def _extract_command_name(expr: str) -> str | None:
    match = re.match(r"^\(\s*([A-Za-z0-9_\-.]+)", expr)
    if not match:
        return None
    return match.group(1)


def _extract_assert_body(assert_expr: str) -> str:
    inner = assert_expr.strip()
    if not inner.startswith("(assert"):
        raise RuntimeError(f"expected assert expression, got: {assert_expr[:40]}")
    body = inner[len("(assert") :].strip()
    if not body.endswith(")"):
        raise RuntimeError(f"malformed assert expression: {assert_expr[:40]}")
    return body[:-1].strip()


def _read_smt_parts(path: Path) -> tuple[list[str], list[str]]:
    text = path.read_text(encoding="utf-8")
    commands = _split_top_level_sexprs(text)
    decls: list[str] = []
    asserts: list[str] = []

    for cmd in commands:
        name = _extract_command_name(cmd)
        if name is None:
            continue
        if name == "assert":
            asserts.append(_extract_assert_body(cmd))
            continue
        if name in {"declare-fun", "declare-const", "define-fun", "declare-sort", "define-sort"}:
            decls.append(cmd)

    if not asserts:
        asserts = ["true"]
    return decls, asserts


def _equivalence_script(left_path: Path, right_path: Path) -> str:
    left_decls, left_asserts = _read_smt_parts(left_path)
    right_decls, right_asserts = _read_smt_parts(right_path)

    merged_decls: list[str] = []
    seen: set[str] = set()
    for decl in [*left_decls, *right_decls]:
        if decl in seen:
            continue
        seen.add(decl)
        merged_decls.append(decl)

    left_formula = "(and " + " ".join(left_asserts) + ")"
    right_formula = "(and " + " ".join(right_asserts) + ")"

    lines = ["(set-logic ALL)"]
    lines.extend(merged_decls)
    lines.append(f"(assert (xor {left_formula} {right_formula}))")
    lines.append("(check-sat)")
    lines.append("(exit)")
    return "\n".join(lines) + "\n"


def _resolve_z3_binary(repo_root: Path) -> str:
    candidates = [
        "z3",
        str(repo_root / "tools" / "x" / ".venv" / "bin" / "z3"),
    ]
    for candidate in candidates:
        try:
            result = subprocess.run([candidate, "-version"], check=False, capture_output=True, text=True)
        except OSError:
            continue
        if result.returncode == 0:
            return candidate
    raise RuntimeError("could not find z3 binary (tried PATH and tools/x/.venv/bin/z3)")


def _check_smt_equivalence(left_path: Path, right_path: Path, z3_bin: str) -> tuple[bool | None, str]:
    script = _equivalence_script(left_path, right_path)
    result = subprocess.run(
        [z3_bin, "-in"],
        input=script,
        text=True,
        check=False,
        capture_output=True,
    )
    if result.returncode != 0:
        raise RuntimeError(f"z3 failed comparing {left_path.name} and {right_path.name}: {result.stderr}")

    output = [line.strip() for line in result.stdout.splitlines() if line.strip()]
    status = output[-1] if output else ""
    if status == "unsat":
        return True, status
    if status == "sat":
        return False, status
    return None, status or "unknown"




def run(args: argparse.Namespace) -> int:
    repo_root = _find_repo_root()

    cargo_dir = Path(args.cargo_dir)
    if not cargo_dir.is_absolute():
        cargo_dir = (repo_root / cargo_dir).resolve()
    else:
        cargo_dir = cargo_dir.resolve()

    if not cargo_dir.is_dir() or not (cargo_dir / "Cargo.toml").is_file():
        raise RuntimeError(f"invalid crate directory: {cargo_dir}")

    other_path = Path(args.other)
    if not other_path.is_absolute():
        other_path = (repo_root / other_path).resolve()
    else:
        other_path = other_path.resolve()
    if not other_path.is_file() and not other_path.is_dir():
        raise RuntimeError(f"other metadata input does not exist: {other_path}")

    studied_rules = Path(args.studied_rules)
    if not studied_rules.is_absolute():
        studied_rules = (repo_root / studied_rules).resolve()

    meta_path = ensure_crate_metadata_file(
        repo_root,
        cargo_dir,
        studied_rules=studied_rules,
        force=False,
    )

    ir_output_dir = Path(args.ir_output_dir)
    if not ir_output_dir.is_absolute():
        ir_output_dir = (repo_root / ir_output_dir).resolve()
    else:
        ir_output_dir = ir_output_dir.resolve()
        

    ll_path = ensure_linked_llvm_ir_file(
        cargo_dir=cargo_dir,
        output_dir=ir_output_dir,
        rustc=args.rustc,
        test=args.test,
        build_std=True,
        force=False,
    )

    current_meta = _load_json(meta_path)
    human_dir = (repo_root / "human" / meta_path.stem).resolve()
    human_path = (repo_root / "human" / meta_path.name).resolve()
    if human_dir.is_dir():
        current_human = _load_task_folder(human_dir)
    elif human_path.is_file():
        current_human = _load_json(human_path)
    else:
        # Backward compatibility for older synced metadata that still stores rules in report.targets.
        current_human = current_meta

    other_meta = _load_meta_like(other_path)
    operators = _load_operator_entries(repo_root)

    report = current_meta.get("report")
    if not isinstance(report, dict):
        raise RuntimeError(f"missing report object in {meta_path}")
    targets = report.get("targets")
    if not isinstance(targets, list):
        raise RuntimeError(f"missing targets in {meta_path}")

    current_by_callsite_rule, _ = _build_meta_task1_index(current_human)
    other_by_callsite_rule, other_by_rule = _build_meta_task1_index(other_meta)
    work_dir = Path(args.work_dir)
    if not work_dir.is_absolute():
        work_dir = (repo_root / work_dir).resolve()
    else:
        work_dir = work_dir.resolve()
    work_dir.mkdir(parents=True, exist_ok=True)

    klee_bin = args.klee_bin
    z3_bin = _resolve_z3_binary(repo_root)
    results: list[dict[str, object]] = []

    for index, raw_target in enumerate(targets, start=1):
        if not isinstance(raw_target, dict):
            continue

        callsite_key = _target_callsite_key(raw_target, index)
        callsite_id = str(index)
        callsite = raw_target.get("callsite")
        if isinstance(callsite, dict):
            callsite_id_raw = callsite.get("id")
            if isinstance(callsite_id_raw, str) and callsite_id_raw:
                callsite_id = callsite_id_raw

        left_rule_values = {
            rule_id: task1
            for (candidate_callsite_id, rule_id), task1 in current_by_callsite_rule.items()
            if candidate_callsite_id == callsite_id
        }
        if not left_rule_values:
            continue

        for rule_id, left_dsl in sorted(left_rule_values.items()):
            if not isinstance(rule_id, str) or not isinstance(left_dsl, str) or not left_dsl:
                continue

            right_dsl = other_by_callsite_rule.get((callsite_id, rule_id))
            if not isinstance(right_dsl, str) or not right_dsl:
                right_dsl = other_by_callsite_rule.get((callsite_key, rule_id), other_by_rule.get(rule_id))
            if not isinstance(right_dsl, str) or not right_dsl:
                print(f"[skip] target={callsite_key} rule={rule_id}: missing eval task1")
                continue

            left_ast = parse_dsl(left_dsl, operators, allow_unknown_operators=True)
            right_ast = parse_dsl(right_dsl, operators, allow_unknown_operators=True)
            left_simplified = simplify_variables(left_ast)
            right_simplified = simplify_variables(right_ast)

            left_ast_json = json.dumps(
                {
                    "simplified": _expr_to_ext_ast(left_simplified.simplified),
                    "original": _expr_to_ext_ast(left_ast),
                },
                separators=(",", ":"),
                ensure_ascii=False,
            )
            right_ast_json = json.dumps(
                {
                    "simplified": _expr_to_ext_ast(right_simplified.simplified),
                    "original": _expr_to_ext_ast(right_ast),
                },
                separators=(",", ":"),
                ensure_ascii=False,
            )

            left_smt = work_dir / f"{callsite_id}-{rule_id}-task1.smt2"
            right_smt = work_dir / f"{callsite_id}-{rule_id}-task1.eval.smt2"

            try:
                if left_simplified.variables.keys() == right_simplified.variables.keys():
                    _write_simplified_smt2(left_simplified.simplified, left_smt)
                    _write_simplified_smt2(right_simplified.simplified, right_smt)
                else:
                    _run_klee_for_constraint(
                        ll_path=ll_path,
                        callsite_id=callsite_id,
                        ast_json=left_ast_json,
                        output_path=left_smt,
                        klee_bin=klee_bin,
                    )
                    _run_klee_for_constraint(
                        ll_path=ll_path,
                        callsite_id=callsite_id,
                        ast_json=right_ast_json,
                        output_path=right_smt,
                        klee_bin=klee_bin,
                    )

                equivalent, z3_status = _check_smt_equivalence(left_smt, right_smt, z3_bin)
            except Exception as exc:
                print(f"[skip] target={callsite_key} rule={rule_id}: {exc}")
                continue

            results.append(
                {
                    "target": callsite_key,
                    "callsite_id": callsite_id,
                    "rule_id": rule_id,
                    "equivalent": equivalent,
                    "z3": z3_status,
                    "left_smt2": str(left_smt),
                    "right_smt2": str(right_smt),
                }
            )

            status_text = "equivalent" if equivalent is True else "different" if equivalent is False else "unknown"
            print(f"[compare] target={callsite_key} rule={rule_id} => {status_text} (z3={z3_status})")

    summary = {
        "total": len(results),
        "equivalent": sum(1 for item in results if item["equivalent"] is True),
        "different": sum(1 for item in results if item["equivalent"] is False),
        "unknown": sum(1 for item in results if item["equivalent"] is None),
    }
    print(
        f"summary: total={summary['total']} equivalent={summary['equivalent']} "
        f"different={summary['different']} unknown={summary['unknown']}"
    )

    output_path = Path(args.output)
    if not output_path.is_absolute():
        output_path = (repo_root / output_path).resolve()
    else:
        output_path = output_path.resolve()
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps({"summary": summary, "results": results}, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    print(f"wrote compare report to {output_path}")
    return 0