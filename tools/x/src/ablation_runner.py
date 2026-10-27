from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Iterable


TOP10_CRATES: tuple[tuple[str, str], ...] = (
    ("hftbacktest", "hftbacktest-0.4.0"),
    ("jh", "jh-0.1.0"),
    ("revm", "revm-interpreter-10.0.3"),
    ("rust-crypto", "rust-crypto-0.2.36"),
    ("lurk-cli", "lurk-cli-0.3.8"),
    ("arrow2", "arrow2-0.18.0"),
    ("blade", "blade-0.2.0"),
    ("memmap", "memmap-0.7.0"),
    ("fast-float", "fast-float-0.2.0"),
    ("meshopt", "meshopt-0.3.0"),
)

CRATE_SUBDIRS: dict[str, str] = {
    # The top-level `blade` package only re-exports workspace crates; the
    # benchmark target in raudit.xlsx belongs to this library package.
    "blade": "blade-graphics",
}

CRATE_FEATURES: dict[str, tuple[str, ...]] = {
    # One of the six audited arrow2 functions is gated by this feature.
    "arrow2": ("compute_sort",),
}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def add_common_arguments(parser: argparse.ArgumentParser) -> None:
    root = repo_root()
    parser.add_argument(
        "--crate",
        action="append",
        choices=[name for name, _ in TOP10_CRATES],
        help="Run only this workbook crate (repeatable; default: all ten).",
    )
    parser.add_argument(
        "--results-root",
        type=Path,
        default=Path("ablation/results"),
        help="Root for variant outputs (default: ablation/results).",
    )
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--timeout-sec", type=int)
    parser.add_argument("--model", default="gpt-5.4-mini")
    parser.add_argument(
        "--rustc",
        type=Path,
        default=root / "tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc",
        help="Patched rustc used for LLVM IR generation.",
    )
    parser.add_argument(
        "--klee-bin",
        type=Path,
        default=root / "tools/klee/build/bin/klee",
        help="KLEE executable used by symbolic variants.",
    )


def selected_crates(names: list[str] | None) -> list[tuple[str, str]]:
    wanted = set(names or [])
    return [row for row in TOP10_CRATES if not wanted or row[0] in wanted]


def _resolve(root: Path, path: Path) -> Path:
    return path if path.is_absolute() else root / path


def run_verify_variant(
    variant: str,
    args: argparse.Namespace,
    *,
    extra_verify_args: Iterable[str] = (),
    rule_dsl: Path | None = None,
) -> int:
    root = repo_root()
    output_root = _resolve(root, args.results_root) / variant
    output_root.mkdir(parents=True, exist_ok=True)
    manifest: dict[str, Any] = {
        "variant": variant,
        "crates": [short for short, _ in selected_crates(args.crate)],
        "commands": [],
    }
    failed = False
    for short_name, crate_dirname in selected_crates(args.crate):
        crate_dir = root / "benchmark" / "ruklee" / crate_dirname
        subdir = CRATE_SUBDIRS.get(short_name)
        if subdir:
            crate_dir = crate_dir / subdir
        if not (crate_dir / "Cargo.toml").is_file():
            raise RuntimeError(f"benchmark crate is missing: {crate_dir}")
        crate_output = output_root / crate_dirname
        command = [
            str(root / "x"),
            "verify",
            str(crate_dir),
            "--results-json",
            str(crate_output / "full-matrix-results.json"),
            "--logs-dir",
            str(crate_output / "full-matrix-logs"),
            "--ir-output-dir",
            str(crate_output / "ir"),
            "--model",
            args.model,
            "--rustc",
            str(args.rustc),
            "--klee-bin",
            str(args.klee_bin),
        ]
        if rule_dsl is not None:
            command.extend(["--rule-dsl", str(rule_dsl)])
        if args.resume:
            command.append("--resume")
        if args.timeout_sec is not None:
            command.extend(["--timeout-sec", str(args.timeout_sec)])
        features = CRATE_FEATURES.get(short_name, ())
        if features:
            command.extend(["--features", ",".join(features)])
        command.extend(extra_verify_args)
        manifest["commands"].append({"crate": short_name, "argv": command})
        print("+", " ".join(command), flush=True)
        if args.dry_run:
            continue
        env = os.environ.copy()
        env["RUSTC_BOOTSTRAP"] = "1"
        cargo_bin = root / "tools/rust/build/x86_64-unknown-linux-gnu/stage0/bin"
        if cargo_bin.is_dir():
            env["PATH"] = f"{cargo_bin}{os.pathsep}{env.get('PATH', '')}"
        result = subprocess.run(command, cwd=root, env=env, check=False)
        if result.returncode != 0:
            failed = True
            print(
                f"ablation {variant}: {crate_dirname} failed with "
                f"exit code {result.returncode}",
                file=sys.stderr,
                flush=True,
            )
    (output_root / "run-manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    return 1 if failed else 0


def _load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise RuntimeError(f"expected JSON object: {path}")
    return value


def _report_for_crate(
    root: Path, short_name: str, crate_dirname: str
) -> dict[str, Any]:
    metadata_name = CRATE_SUBDIRS.get(short_name, crate_dirname)
    document = _load_json(root / "crates" / f"{metadata_name}.json")
    report = document.get("report")
    if not isinstance(report, dict):
        raise RuntimeError(f"missing report in crates/{crate_dirname}.json")
    return report


def _source_path(root: Path, path: str) -> Path:
    normalized = path.replace("\\", "/")
    if normalized.startswith("rust/"):
        return root / "tools" / normalized
    return root / normalized


def _documentation_record(root: Path, path: str, line: int) -> str:
    source_path = _source_path(root, path)
    lines = source_path.read_text(encoding="utf-8", errors="replace").splitlines()
    index = max(0, min(len(lines) - 1, line - 1))
    start = index
    while start > 0:
        text = lines[start - 1].lstrip()
        if (
            text.startswith("///")
            or text.startswith("#[")
            or text.startswith("//!")
            or not text
        ):
            start -= 1
            continue
        break
    end = index
    while end + 1 < len(lines) and end - index < 12:
        if "{" in lines[end] or lines[end].rstrip().endswith(";"):
            break
        end += 1
    return "\n".join(lines[start : end + 1]).strip()


def _operator_prompt(root: Path) -> str:
    document = _load_json(root / "operators.json")
    operators = document.get("operators", [])
    compact = []
    for operator in operators if isinstance(operators, list) else []:
        if not isinstance(operator, dict):
            continue
        compact.append(
            {
                key: operator.get(key)
                for key in ("name", "args", "return", "description")
                if operator.get(key) is not None
            }
        )
    return json.dumps(compact, indent=2, ensure_ascii=False)


def _parse_json_response(text: str) -> Any:
    stripped = text.strip()
    fenced = re.search(r"```(?:json)?\s*(.*?)```", stripped, re.S | re.I)
    if fenced:
        stripped = fenced.group(1).strip()
    value = json.loads(stripped)
    if isinstance(value, dict):
        for key in ("rules", "results", "items"):
            if isinstance(value.get(key), list):
                return value[key]
    return value


def prepare_without_rule_extraction(args: argparse.Namespace) -> Path:
    root = repo_root()
    output_root = _resolve(root, args.results_root) / "without-rule-extraction"
    output_root.mkdir(parents=True, exist_ok=True)
    output_path = output_root / "direct-document-rules.json"
    existing: dict[str, Any] = {
        "description": "Direct whole-document-to-DSL ablation rules.",
        "rules": {},
        "processed_apis": [],
        "rejected": [],
    }
    if args.resume and output_path.is_file():
        existing = _load_json(output_path)
    rules = existing.setdefault("rules", {})
    if not isinstance(rules, dict):
        raise RuntimeError(f"invalid rules object in {output_path}")
    processed_apis = existing.setdefault("processed_apis", [])
    rejected = existing.setdefault("rejected", [])
    if not isinstance(processed_apis, list) or not isinstance(rejected, list):
        raise RuntimeError(f"invalid ablation metadata in {output_path}")

    api_locations: set[tuple[str, int, str]] = set()
    for short_name, crate_dirname in selected_crates(args.crate):
        for target in _report_for_crate(root, short_name, crate_dirname).get("targets", []):
            callee = target.get("callee") if isinstance(target, dict) else None
            if not isinstance(callee, dict):
                continue
            path, line, name = callee.get("path"), callee.get("line_start"), callee.get("name")
            if name == "core::ptr::__raw_ptr_deref__":
                # This synthetic target has no API documentation to extract.
                # The backend handles it identically in all KLEE variants.
                continue
            if isinstance(path, str) and isinstance(line, int) and isinstance(name, str):
                api_locations.add((path.replace("\\", "/"), line, name))

    covered = {
        (entry.get("path"), entry.get("source_api"))
        for entry in rules.values()
        if isinstance(entry, dict)
    } | {
        (entry.get("path"), entry.get("source_api"))
        for entry in processed_apis
        if isinstance(entry, dict)
    }
    if args.dry_run:
        client = None
    else:
        from dsl.lint import lint_dsl_ast
        from dsl.parser import parse_dsl
        from llm.openai_llm import OpenAILLM

        client = OpenAILLM(model=args.model)
        operator_document = _load_json(root / "operators.json")
        operators = operator_document.get("operators", [])
        if not isinstance(operators, list):
            raise RuntimeError("operators.json must contain an operators array")
    operator_text = _operator_prompt(root)
    for path, line, name in sorted(api_locations):
        path_line = f"{path}:{line}"
        if (path_line, name) in covered:
            continue
        record = _documentation_record(root, path, line)
        prompt = f"""Translate the complete Rust unsafe-API documentation below directly into executable DSL rules.
This is an ablation: do not first split or paraphrase the Safety section into natural-language atomic rules.
Return one JSON object with a `rules` array. Each array item must contain exactly `rule` (a short faithful description) and `dsl` (one parseable Boolean DSL expression). Do not invent obligations absent from the documentation. Omit requirements that the supplied DSL cannot express.

Unsafe API: {name}
Source: {path_line}
Documentation and signature:
```rust
{record}
```

Available typed DSL operators:
```json
{operator_text}
```
"""
        request_dir = output_root / "prompts"
        request_dir.mkdir(parents=True, exist_ok=True)
        key = hashlib.sha256(f"{path_line}\0{name}".encode()).hexdigest()[:12]
        (request_dir / f"{key}.txt").write_text(prompt, encoding="utf-8")
        if args.dry_run:
            print(f"would generate direct-document rules for {name} ({path_line})")
            continue
        assert client is not None
        response = client.complete(
            "You translate Rust unsafe API documentation into a supplied typed DSL. Return JSON only.",
            prompt,
        )
        (request_dir / f"{key}.response.txt").write_text(response, encoding="utf-8")
        items = _parse_json_response(response)
        if not isinstance(items, list):
            raise RuntimeError(f"model did not return a rules array for {name}")
        for index, item in enumerate(items, start=1):
            if not isinstance(item, dict):
                continue
            rule, dsl = item.get("rule"), item.get("dsl")
            if not isinstance(rule, str) or not isinstance(dsl, str):
                continue
            try:
                ast = parse_dsl(dsl.strip(), operators)
                errors = [
                    issue for issue in lint_dsl_ast(ast, operators)
                    if issue.severity == "error"
                ]
                if errors:
                    raise ValueError(
                        "; ".join(f"{issue.code}: {issue.message}" for issue in errors)
                    )
            except Exception as exc:
                rejected.append(
                    {
                        "path": path_line,
                        "source_api": name,
                        "rule": rule.strip(),
                        "dsl": dsl.strip(),
                        "error": str(exc),
                    }
                )
                continue
            rule_id = f"woe-{key}-{index:03d}"
            rules[rule_id] = {
                "path": path_line,
                "name": name,
                "source_api": name,
                "rule": rule.strip(),
                "dsl": dsl.strip(),
            }
        processed_apis.append({"path": path_line, "source_api": name})
        output_path.write_text(
            json.dumps(existing, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
    if not args.dry_run and not rules:
        raise RuntimeError("direct-document translation produced no rules")
    return output_path


def _parse_rule_location(entry: dict[str, Any]) -> tuple[str, int] | None:
    raw_path = entry.get("path")
    if not isinstance(raw_path, str):
        return None
    path, separator, raw_line = raw_path.rpartition(":")
    if not separator or not raw_line.isdigit():
        return None
    return path.replace("\\", "/"), int(raw_line)


def _smtlib_selector_rule(
    *, response: str, operators: list[dict[str, Any]]
) -> tuple[str, dict[str, object], dict[str, str], dict[str, str]]:
    from cli.cmd.compare import _expr_to_ext_ast
    from dsl.ast import BinaryExpression, CallExpression, Literal, UnaryExpression
    from dsl.lint import lint_dsl_ast
    from dsl.parser import parse_dsl

    parsed = _parse_json_response(response)
    if not isinstance(parsed, dict):
        raise RuntimeError("model response must be one JSON object")
    smtlib = parsed.get("smtlib")
    raw_selectors = parsed.get("selectors")
    if not isinstance(smtlib, str) or "(assert" not in smtlib:
        raise RuntimeError("model response lacks an SMT-LIB assert")
    if not isinstance(raw_selectors, dict) or not raw_selectors:
        raise RuntimeError("model response lacks variable selectors")

    selectors: dict[str, object] = {}
    selector_text: dict[str, str] = {}
    sorts: dict[str, str] = {}
    output_types = {
        str(item.get("name")): str((item.get("output") or {}).get("type", ""))
        for item in operators
        if isinstance(item, dict) and isinstance(item.get("name"), str)
    }

    def selector_sort(ast: object) -> str | None:
        if isinstance(ast, CallExpression):
            output = output_types.get(ast.name, "").lower()
            if output in {"bool", "boolean"}:
                return "Bool"
            if output in {"value", "uint", "int", "usize", "isize", "scalar"}:
                return "Int"
            return None
        if isinstance(ast, BinaryExpression):
            return "Bool" if ast.operator in {"==", "!=", "<", "<=", ">", ">=", "&&", "||", "=>"} else "Int"
        if isinstance(ast, UnaryExpression):
            return "Bool" if ast.operator == "!" else "Int"
        if isinstance(ast, Literal):
            return "Bool" if isinstance(ast.value, bool) else "Int" if isinstance(ast.value, int) else None
        return None
    for name, raw_selector in raw_selectors.items():
        if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_.-]*", name):
            raise RuntimeError(f"invalid SMT variable name: {name!r}")
        if isinstance(raw_selector, str):
            text = raw_selector
            sort = "Int"
        elif isinstance(raw_selector, dict):
            text = raw_selector.get("selector")
            sort = raw_selector.get("sort", "Int")
        else:
            raise RuntimeError(f"selector {name!r} must be a string or object")
        if not isinstance(text, str) or not text.strip():
            raise RuntimeError(f"selector {name!r} lacks selector DSL")
        if sort not in {"Int", "Bool"}:
            raise RuntimeError(f"selector {name!r} has unsupported sort {sort!r}")
        ast = parse_dsl(text, operators)
        errors = [
            issue for issue in lint_dsl_ast(ast, operators)
            if issue.severity == "error"
        ]
        if errors:
            raise RuntimeError(
                f"invalid selector {name!r}: "
                + "; ".join(f"{issue.code}: {issue.message}" for issue in errors)
            )
        inferred_sort = selector_sort(ast)
        if inferred_sort is None:
            raise RuntimeError(
                f"selector {name!r} returns a non-scalar KLEE object; select a field or predicate"
            )
        if inferred_sort != sort:
            raise RuntimeError(
                f"selector {name!r} sort mismatch: declared {sort}, expression returns {inferred_sort}"
            )
        selectors[name] = _expr_to_ext_ast(ast)
        selector_text[name] = text.strip()
        sorts[name] = sort

    allowed_heads = {
        "set-logic", "declare-const", "assert", "check-sat", "exit",
        "and", "or", "not", "=>", "=", "distinct", "<", "<=", ">", ">=",
        "+", "-", "*", "div", "mod",
    }
    unsupported_heads = {
        head
        for head in re.findall(r"\(\s*([^\s()]+)", smtlib)
        if head not in allowed_heads
    }
    if unsupported_heads:
        raise RuntimeError(
            "unsupported SMT-LIB operator/command: " + ", ".join(sorted(unsupported_heads))
        )

    declarations = {
        match.group(1).strip("|"): match.group(2)
        for match in re.finditer(
            r"\(declare-const\s+(\|(?:\\.|[^|])*\||[^\s()]+)\s+(Int|Bool)\s*\)",
            smtlib,
        )
    }
    declared = set(declarations)
    missing = set(selectors) - declared
    extra = declared - set(selectors)
    if missing or extra:
        details = []
        if missing:
            details.append("selectors not declared: " + ", ".join(sorted(missing)))
        if extra:
            details.append("declared variables without selectors: " + ", ".join(sorted(extra)))
        raise RuntimeError("; ".join(details))
    sort_mismatches = [
        f"{name}: SMT-LIB={declarations[name]} selector={sorts[name]}"
        for name in sorted(declared)
        if declarations[name] != sorts[name]
    ]
    if sort_mismatches:
        raise RuntimeError("sort mismatch: " + "; ".join(sort_mismatches))
    return smtlib.strip(), selectors, selector_text, sorts


def prepare_without_rule_translation(args: argparse.Namespace) -> Path:
    root = repo_root()
    output_root = _resolve(root, args.results_root) / "without-rule-translation"
    output_root.mkdir(parents=True, exist_ok=True)
    output_path = output_root / "smtlib-selector-rules.json"
    existing: dict[str, Any] = {
        "description": (
            "Atomic natural-language rules translated directly to SMT-LIB with "
            "KLEE variable selectors, bypassing the RuKLEE rule DSL translation."
        ),
        "rules": {},
        "processed_rules": [],
        "rejected": [],
    }
    if args.resume and output_path.is_file():
        existing = _load_json(output_path)
    rules = existing.setdefault("rules", {})
    processed_rules = existing.setdefault("processed_rules", [])
    rejected = existing.setdefault("rejected", [])
    if not isinstance(rules, dict) or not isinstance(processed_rules, list) or not isinstance(rejected, list):
        raise RuntimeError(f"invalid rule-translation ablation metadata in {output_path}")

    shared = _load_json(root / "ptr_rule_dsl.json")
    shared_rules = shared.get("rules", {})
    if not isinstance(shared_rules, dict):
        raise RuntimeError("ptr_rule_dsl.json must contain a rules object")
    by_api = _rule_index(root)
    by_name = _rule_name_index(root)
    needed: dict[str, tuple[dict[str, Any], str, int, str]] = {}
    for short_name, crate_dirname in selected_crates(args.crate):
        for target in _report_for_crate(root, short_name, crate_dirname).get("targets", []):
            callee = target.get("callee") if isinstance(target, dict) else None
            if not isinstance(callee, dict):
                continue
            path, line, name = callee.get("path"), callee.get("line_start"), callee.get("name")
            if not isinstance(path, str) or not isinstance(line, int):
                continue
            for rule_id, _ in _rules_for_target(
                callee=callee, by_location=by_api, by_name=by_name
            ):
                entry = shared_rules.get(rule_id)
                if isinstance(entry, dict):
                    needed[rule_id] = (entry, path.replace("\\", "/"), line, str(name or entry.get("name") or ""))

    if args.dry_run:
        client = None
    else:
        from llm.openai_llm import OpenAILLM

        client = OpenAILLM(model=args.model)
    operator_document = _load_json(root / "operators.json")
    operators = operator_document.get("operators", [])
    if not isinstance(operators, list):
        raise RuntimeError("operators.json must contain an operators array")
    for rule_id, stored in list(rules.items()):
        if not isinstance(stored, dict):
            rules.pop(rule_id, None)
            continue
        selector_text = stored.get("selector_text")
        sorts = stored.get("sorts")
        smtlib = stored.get("smtlib")
        if not isinstance(selector_text, dict) or not isinstance(sorts, dict):
            rules.pop(rule_id, None)
            continue
        raw_selectors = {
            name: {"sort": sorts.get(name, "Int"), "selector": selector}
            for name, selector in selector_text.items()
        }
        try:
            _smtlib_selector_rule(
                response=json.dumps(
                    {"smtlib": smtlib, "selectors": raw_selectors},
                    ensure_ascii=False,
                ),
                operators=operators,
            )
        except Exception as exc:
            rules.pop(rule_id, None)
            rejected.append(
                {"rule_id": rule_id, "rule": stored.get("rule"), "error": f"cached rule invalid: {exc}"}
            )
    done = set(rules)
    operator_text = _operator_prompt(root)
    prompts_dir = output_root / "prompts"
    prompts_dir.mkdir(parents=True, exist_ok=True)

    for rule_id, (entry, api_path, api_line, api_name) in sorted(needed.items()):
        if rule_id in done:
            continue
        natural_rule = entry.get("rule")
        if not isinstance(natural_rule, str) or not natural_rule.strip():
            rejected.append({"rule_id": rule_id, "error": "missing natural-language rule"})
            processed_rules.append(rule_id)
            continue
        record = _documentation_record(root, api_path, api_line)
        prompt = f"""Translate one already-extracted Rust unsafe-API rule directly to SMT-LIB plus variable selectors.
This is a rule-translation ablation. Do not emit the RuKLEE DSL predicate and do not consult any existing translated rule.

Return one JSON object with exactly:
- `smtlib`: a complete quantifier-free SMT-LIB script with `(set-logic ALL)`, one `(declare-const NAME Int|Bool)` per variable, exactly one `(assert FORMULA)`, and `(check-sat)`. The asserted formula is the safety requirement, not its negation.
- `selectors`: an object mapping every declared SMT variable name to `{{"sort":"Int"|"Bool", "selector":"..."}}`. A selector is one executable expression made only from the operators below. KLEE resolves selectors at the unsafe callsite. Keep complex pointer/allocation predicates in Bool selectors and combine them in SMT-LIB.

Use only SMT-LIB operators `and`, `or`, `not`, `=>`, `=`, `distinct`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `div`, and `mod`. Use simple unquoted variable names. Every declared variable must have exactly one selector and vice versa.

Selector expressions use the repository DSL syntax, not SMT-LIB syntax. String/enum arguments must be quoted. Selectors cannot refer to other SMT variables. Examples of valid selectors are `get_arg(1)`, `get_field(get_type_at_func(0), "size")`, `offset_does_not_wrap(get_arg(0), get_arg(1), get_field(get_type_at_func(0), "size"), "signed")`, and `is_initialized_for_type(memory_range(get_arg(0), get_field(get_type_at_func(0), "size")), get_type_at_func(0))`. Follow the exact operator arities in the supplied operator table.

A selector declared `Bool` must call a Boolean predicate such as `is_valid_for`, `is_initialized_for_type`, `offset_does_not_wrap`, or `mul_fits`. A selector declared `Int` must resolve to a scalar `Value`, commonly through `get_arg(...)` or `get_field(...)`; object-valued operators such as `get_allocation(...)` and `get_type_at_func(...)` cannot be used directly as Int/Bool selectors. Prefer one faithful Bool selector for a complex pointer or overflow condition and simply assert that variable in SMT-LIB.
For example, declare `(declare-const offset_fits Bool)`, map it to the selector `mul_fits(...)`, and write `(assert offset_fits)`. A declared variable is a zero-arity SMT constant, so never write it as `(offset_fits)`.

Unsafe API: {api_name}
Source: {api_path}:{api_line}
Already-extracted atomic rule: {natural_rule}
Documentation and signature:
```rust
{record}
```

Available selector operators:
```json
{operator_text}
```
"""
        (prompts_dir / f"{rule_id}.txt").write_text(prompt, encoding="utf-8")
        if args.dry_run:
            print(f"would generate SMT-LIB/selectors for {rule_id} ({api_name})")
            continue
        assert client is not None
        last_error: Exception | None = None
        attempt_prompt = prompt
        for attempt in range(1, 6):
            try:
                response = client.complete(
                    "You translate an atomic Rust safety rule to SMT-LIB plus executable KLEE selectors. Return JSON only.",
                    attempt_prompt,
                )
                response_path = prompts_dir / (
                    f"{rule_id}.response.txt" if attempt == 1
                    else f"{rule_id}.response-attempt-{attempt}.txt"
                )
                response_path.write_text(response, encoding="utf-8")
                smtlib, selectors, selector_text, sorts = _smtlib_selector_rule(
                    response=response, operators=operators
                )
                rules[rule_id] = {
                    "path": entry.get("path"),
                    "aliases": entry.get("aliases", []),
                    "name": entry.get("name"),
                    "source_api": api_name,
                    "rule": natural_rule.strip(),
                    "smtlib": smtlib,
                    "selectors": selectors,
                    "selector_text": selector_text,
                    "sorts": sorts,
                }
                rejected[:] = [
                    item for item in rejected
                    if not isinstance(item, dict) or item.get("rule_id") != rule_id
                ]
                last_error = None
                break
            except Exception as exc:
                last_error = exc
                attempt_prompt = (
                    prompt
                    + "\n\nThe previous response was invalid. Repair it and return the full JSON object again.\n"
                    + f"Validator error: {exc}\n"
                    + "Use only exact operator calls from the table in selectors; do not put SMT-LIB syntax or SMT variable names inside a selector.\n"
                )
        if last_error is not None:
            rejected[:] = [
                item for item in rejected
                if not isinstance(item, dict) or item.get("rule_id") != rule_id
            ]
            rejected.append(
                {"rule_id": rule_id, "rule": natural_rule.strip(), "error": str(last_error)}
            )
        if rule_id not in processed_rules:
            processed_rules.append(rule_id)
        output_path.write_text(
            json.dumps(existing, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )

    if not args.dry_run and not rules:
        raise RuntimeError("direct SMT-LIB rule translation produced no rules")
    return output_path


def _rule_index(root: Path) -> dict[tuple[str, int], list[tuple[str, dict[str, Any]]]]:
    document = _load_json(root / "ptr_rule_dsl.json")
    raw_rules = document.get("rules", {})
    result: dict[tuple[str, int], list[tuple[str, dict[str, Any]]]] = {}
    if not isinstance(raw_rules, dict):
        return result
    for rule_id, entry in raw_rules.items():
        if not isinstance(rule_id, str) or not isinstance(entry, dict):
            continue
        raw_paths = [entry.get("path")]
        aliases = entry.get("aliases", [])
        if isinstance(aliases, list):
            raw_paths.extend(aliases)
        indexed_locations: set[tuple[str, int]] = set()
        for raw_path in raw_paths:
            if not isinstance(raw_path, str):
                continue
            path, sep, raw_line = raw_path.rpartition(":")
            if not sep or not raw_line.isdigit():
                continue
            location = (path.replace("\\", "/"), int(raw_line))
            if location in indexed_locations:
                continue
            indexed_locations.add(location)
            result.setdefault(location, []).append((rule_id, entry))
    return result


def _unsafe_function_name(value: object) -> str | None:
    if not isinstance(value, str):
        return None
    match = re.search(r"\bunsafe\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)", value)
    if match:
        return match.group(1)
    tail = value.rsplit("::", 1)[-1]
    match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", tail)
    return match.group(1) if match else None


def _rule_name_index(
    root: Path,
) -> dict[tuple[str, str], list[tuple[str, dict[str, Any]]]]:
    document = _load_json(root / "ptr_rule_dsl.json")
    raw_rules = document.get("rules", {})
    result: dict[tuple[str, str], list[tuple[str, dict[str, Any]]]] = {}
    if not isinstance(raw_rules, dict):
        return result
    for rule_id, entry in raw_rules.items():
        if not isinstance(rule_id, str) or not isinstance(entry, dict):
            continue
        location = _parse_rule_location(entry)
        function_name = _unsafe_function_name(entry.get("name"))
        if location is None or function_name is None:
            continue
        path, _ = location
        result.setdefault((path, function_name), []).append((rule_id, entry))
    return result


def _rules_for_target(
    *,
    callee: dict[str, Any],
    by_location: dict[tuple[str, int], list[tuple[str, dict[str, Any]]]],
    by_name: dict[tuple[str, str], list[tuple[str, dict[str, Any]]]],
) -> list[tuple[str, dict[str, Any]]]:
    name = callee.get("name")
    if name == "core::ptr::__raw_ptr_deref__":
        return [
            (
                "__raw_ptr_deref__",
                {
                    "name": name,
                    "rule": "The raw pointer dereference must use a valid pointer.",
                },
            )
        ]
    path, line = callee.get("path"), callee.get("line_start")
    if not isinstance(path, str) or not isinstance(line, int):
        return []
    normalized_path = path.replace("\\", "/")
    exact = by_location.get((normalized_path, line), [])
    if exact:
        return exact
    function_name = _unsafe_function_name(name)
    return by_name.get((normalized_path, function_name), []) if function_name else []


def _source_span(crate_dir: Path, item: object) -> str:
    if not isinstance(item, dict):
        return ""
    path = item.get("path")
    if not isinstance(path, str):
        return ""
    source_path = crate_dir / path
    if not source_path.is_file():
        return ""
    source = source_path.read_text(encoding="utf-8", errors="replace")
    lines = source.splitlines()
    start = max(1, int(item.get("line_start", 1) or 1))
    end = max(start, int(item.get("body_end", item.get("line_end", start)) or start))
    if start <= len(lines) and re.search(
        r"\b(?:struct|enum|union)\s+[A-Za-z_]", lines[start - 1]
    ):
        # MIRScan records a type's declaration line, not its closing brace.
        # Include fields and variants so the LLM sees the actual state shape.
        from llm.gen_example import _mask_rust_non_code, _matching_brace

        offset = sum(len(line) for line in source.splitlines(keepends=True)[:start - 1])
        masked = _mask_rust_non_code(source)
        opening = masked.find("{", offset)
        terminator = masked.find(";", offset)
        if opening >= 0 and (terminator < 0 or opening < terminator):
            closing = _matching_brace(masked, opening)
            if closing is not None:
                end = max(end, source.count("\n", 0, closing) + 1)
        elif terminator >= 0:
            end = max(end, source.count("\n", 0, terminator) + 1)
    return "\n".join(lines[start - 1 : min(end, len(lines))])


def _related_code(crate_dir: Path, report: dict[str, Any], target: dict[str, Any]) -> str:
    sections: list[tuple[str, str]] = []
    seen: set[tuple[object, object, object]] = set()

    def add(label: str, item: object) -> None:
        if not isinstance(item, dict):
            return
        key = (item.get("path"), item.get("line_start"), item.get("name"))
        if key in seen:
            return
        code = _source_span(crate_dir, item)
        if not code:
            return
        seen.add(key)
        sections.append((label, code))

    add("target safe caller", target.get("caller"))
    parent = target.get("caller_parent")
    add("enclosing type", parent)
    parent_name = parent.get("name") if isinstance(parent, dict) else None
    parent_leaf = str(parent_name or "").rsplit("::", 1)[-1]
    types = report.get("types", [])
    for type_entry in types if isinstance(types, list) else []:
        if not isinstance(type_entry, dict):
            continue
        type_info = type_entry.get("type")
        type_name = type_info.get("name") if isinstance(type_info, dict) else None
        if parent_name and type_name not in {parent_name, parent_leaf} and not str(type_name or "").endswith("::" + parent_leaf):
            continue
        add("enclosing type", type_info)
        for category in ("constructors", "mutators"):
            candidates = type_entry.get(category, [])
            for candidate in candidates if isinstance(candidates, list) else []:
                add(f"safe-context {category[:-1]}", candidate)
    return "\n\n".join(
        f"// --- {label} ---\n{code}" for label, code in sections
    )


def run_without_symbolic_execution(args: argparse.Namespace) -> int:
    root = repo_root()
    output_root = _resolve(root, args.results_root) / "without-symbolic-execution"
    output_root.mkdir(parents=True, exist_ok=True)
    by_api = _rule_index(root)
    by_name = _rule_name_index(root)
    if args.dry_run:
        client = None
    else:
        from llm.openai_llm import OpenAILLM

        client = OpenAILLM(model=args.model)
    failed = False
    for short_name, crate_dirname in selected_crates(args.crate):
        report = _report_for_crate(root, short_name, crate_dirname)
        crate_dir = root / "benchmark" / "ruklee" / crate_dirname
        subdir = CRATE_SUBDIRS.get(short_name)
        if subdir:
            crate_dir = crate_dir / subdir
        crate_output = output_root / crate_dirname
        crate_output.mkdir(parents=True, exist_ok=True)
        result_path = crate_output / "full-matrix-results.json"
        old_rows: dict[tuple[int, str], dict[str, Any]] = {}
        if args.resume and result_path.is_file():
            for row in _load_json(result_path).get("results", []):
                if isinstance(row, dict) and isinstance(row.get("target_index"), int):
                    old_rows[(row["target_index"], str(row.get("rule")))] = row
        rows = list(old_rows.values())
        prompts_dir = crate_output / "prompts"
        prompts_dir.mkdir(parents=True, exist_ok=True)
        targets = report.get("targets", [])
        for target_index, target in enumerate(targets if isinstance(targets, list) else []):
            if not isinstance(target, dict):
                continue
            callee = target.get("callee")
            callsite = target.get("callsite")
            if not isinstance(callee, dict) or not isinstance(callsite, dict):
                continue
            rules = _rules_for_target(
                callee=callee,
                by_location=by_api,
                by_name=by_name,
            )
            caller_code = _related_code(crate_dir, report, target)
            for rule_id, entry in rules:
                result_key = (target_index, rule_id)
                if result_key in old_rows:
                    continue
                prompt = f"""Decide whether the Rust code can violate the supplied unsafe-API rule.
This is the no-symbolic-execution ablation: reason from the DSL, its natural-language meaning, and related source code only. Do not assume private fields can take arbitrary values unless safe code exposes a way to set them.
Return JSON with `violated` (Boolean) and `reason` (one sentence explaining the code evidence).

Unsafe API: {callee.get('name')}
Callsite: {callsite.get('path')}:{callsite.get('line')}:{callsite.get('col')}
Natural-language rule: {entry.get('rule')}
DSL rule: {entry.get('dsl')}

Related safe caller code:
```rust
{caller_code}
```
"""
                stem = f"{target_index:04d}-{rule_id}"
                (prompts_dir / f"{stem}.txt").write_text(prompt, encoding="utf-8")
                if args.dry_run:
                    print(f"would LLM-audit {crate_dirname} target#{target_index} {rule_id}")
                    continue
                try:
                    assert client is not None
                    response = client.complete(
                        "Compare Rust benchmark code with the supplied API rule. Return the requested JSON only.",
                        prompt,
                    )
                    (prompts_dir / f"{stem}.response.txt").write_text(response, encoding="utf-8")
                    parsed = _parse_json_response(response)
                    if not isinstance(parsed, dict) or not isinstance(parsed.get("violated"), bool):
                        raise RuntimeError("response lacks Boolean `violated`")
                    violated = parsed["violated"]
                    status = "violation" if violated else "verified"
                    error = None
                except Exception as exc:
                    failed = True
                    violated = False
                    status = "error"
                    error = str(exc)
                rows.append(
                    {
                        "target_index": target_index,
                        "callsite": callsite.get("id"),
                        "path": callsite.get("path"),
                        "line": callsite.get("line"),
                        "col": callsite.get("col"),
                        "caller": (target.get("caller") or {}).get("name"),
                        "unsafe_callee": callee.get("name"),
                        "rule": rule_id,
                        "status": status,
                        "prediction": "violation" if violated else "verified",
                        "reason": parsed.get("reason") if error is None else None,
                        "error": error,
                    }
                )
                result_path.write_text(
                    json.dumps(
                        {
                            "schema_version": 1,
                            "variant": "without-symbolic-execution",
                            "crate": str(crate_dir),
                            "model": args.model,
                            "results": sorted(rows, key=lambda row: (row["target_index"], row["rule"])),
                        },
                        indent=2,
                        ensure_ascii=False,
                    )
                    + "\n",
                    encoding="utf-8",
                )
    return 1 if failed else 0
