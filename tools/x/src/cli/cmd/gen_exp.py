import argparse
import json
import re
from pathlib import Path
from typing import Any

from llm.gen_example import build_final_prompt, read_rust_files_as_context
from llm.openai_llm import OpenAILLM


SYSTEM_PROMPT = (
    "You are a Rust soundness researcher. Return the runnable example in a "
    "```rust fenced code block. Keep any explanation outside that block."
)


def _load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise RuntimeError(f"File not found: {path}") from None
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"Invalid JSON in {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise RuntimeError(f"Expected a JSON object in {path}")
    return value


def _find_target(report: dict[str, Any], callsite_id: str) -> dict[str, Any]:
    targets = report.get("targets")
    if not isinstance(targets, list):
        raise RuntimeError("report.json does not contain a targets array")

    for index, target in enumerate(targets):
        if not isinstance(target, dict):
            continue
        callsite = target.get("callsite")
        ids = {str(index)}
        if isinstance(callsite, dict):
            value = callsite.get("id")
            if isinstance(value, str):
                ids.add(value)
        if callsite_id in ids:
            return target
    raise RuntimeError(f"Callsite not found in report.json: {callsite_id}")


def _get_rule(rule_document: dict[str, Any], rule_id: str) -> dict[str, Any]:
    rules = rule_document.get("rules", rule_document)
    if not isinstance(rules, dict) or not isinstance(rules.get(rule_id), dict):
        raise RuntimeError(f"Rule not found in rule metadata: {rule_id}")
    return rules[rule_id]


def _describe_item(label: str, item: Any) -> str:
    if not isinstance(item, dict):
        return f"{label}: unavailable"
    name = item.get("name", "unknown")
    path = item.get("path", "unknown")
    start = item.get("line_start", "?")
    end = item.get("body_end", item.get("line_end", "?"))
    return f"{label}: {name}\nLocation: {path}:{start}-{end}"


def extract_rust_code(completion: str) -> str:
    rust_blocks = re.findall(
        r"```(?:rust|rs)\s*\n(.*?)```", completion, flags=re.IGNORECASE | re.DOTALL
    )
    if rust_blocks:
        return rust_blocks[0].strip() + "\n"

    generic_blocks = re.findall(r"```\s*\n(.*?)```", completion, flags=re.DOTALL)
    if generic_blocks:
        return generic_blocks[0].strip() + "\n"

    stripped = completion.strip()
    if stripped and ("fn main" in stripped or stripped.startswith("#![")):
        return stripped + "\n"
    raise RuntimeError("The model response did not contain a Rust code block")


def _default_output(crate_dir: Path, callsite_id: str, rule_id: str) -> Path:
    safe_name = re.sub(r"[^A-Za-z0-9_-]+", "-", f"{callsite_id}-{rule_id}")
    return crate_dir / "examples" / f"{safe_name}.rs"


def _artifact_dir(
    root: Path, crate_dir: Path, callsite_id: str, rule_id: str
) -> Path:
    safe_parts = [
        re.sub(r"[^A-Za-z0-9_.-]+", "-", value)
        for value in (crate_dir.name, callsite_id, rule_id)
    ]
    return root.joinpath(*safe_parts)


def run(args: argparse.Namespace) -> int:
    crate_dir = Path(args.crate_dir).resolve()
    if not (crate_dir / "Cargo.toml").is_file():
        raise RuntimeError(f"Not a Rust crate: {crate_dir}")

    report_path = (
        Path(args.report_json).resolve()
        if args.report_json
        else crate_dir / "report.json"
    )
    target = _find_target(_load_object(report_path), args.callsite)
    rule = _get_rule(_load_object(Path(args.rule_dsl).resolve()), args.rule)

    callsite = target.get("callsite")
    callsite_text = ""
    if isinstance(callsite, dict):
        callsite_text = (
            f"\nCallsite: {callsite.get('path', 'unknown')}:"
            f"{callsite.get('line', '?')}:{callsite.get('col', '?')}"
        )
    struct_method = _describe_item("Safe caller", target.get("caller")) + callsite_text
    unsafe_api = _describe_item("Unsafe callee", target.get("callee"))
    safety_requirement = str(rule.get("rule", "")).strip()
    if not safety_requirement:
        raise RuntimeError(f"Rule {args.rule} has no safety requirement text")

    prompt = build_final_prompt(
        rust_context=read_rust_files_as_context(crate_dir),
        struct_method=struct_method,
        unsafe_api=unsafe_api,
        safety_requirement=safety_requirement,
    )

    artifacts = _artifact_dir(
        Path(args.artifacts_dir).resolve(), crate_dir, args.callsite, args.rule
    )
    artifacts.mkdir(parents=True, exist_ok=True)
    (artifacts / "system-prompt.txt").write_text(
        SYSTEM_PROMPT + "\n", encoding="utf-8"
    )
    (artifacts / "user-prompt.txt").write_text(prompt + "\n", encoding="utf-8")
    (artifacts / "metadata.json").write_text(
        json.dumps(
            {
                "crate_dir": str(crate_dir),
                "report_json": str(report_path),
                "callsite": args.callsite,
                "rule": args.rule,
                "model": args.model,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )

    completion = OpenAILLM(model=args.model).complete(SYSTEM_PROMPT, prompt)
    (artifacts / "raw-response.txt").write_text(completion + "\n", encoding="utf-8")
    output = (
        Path(args.output).resolve()
        if args.output
        else _default_output(crate_dir, args.callsite, args.rule)
    )
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(extract_rust_code(completion), encoding="utf-8")
    print(f"Generated example: {output}")
    print(f"Saved prompt artifacts: {artifacts}")
    return 0
