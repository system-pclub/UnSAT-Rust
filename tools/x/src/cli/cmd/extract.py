from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
from typing import Any

from openai import OpenAI

from llm.openai_llm import _load_openai_api_key_from_dotenv


SYSTEM_PROMPT = """You extract atomic caller obligations from Rust unsafe API documentation.
Return JSON only: {"rules": ["requirement one", "requirement two"]}.
Use only requirements stated by the supplied Safety documentation. Split conjunctions
into independently checkable atomic rules, preserve every condition and exception, and
keep each rule self-contained. Do not translate to code or a formal DSL. Do not add
implementation advice, descriptive guarantees, or requirements from general memory.
If the documentation contains no caller safety obligation, return an empty rules array.
"""


def _repo_root() -> Path:
    for candidate in (Path.cwd().resolve(), *Path.cwd().resolve().parents):
        if (candidate / "tools" / "extract-comment" / "Cargo.toml").is_file():
            return candidate
    raise RuntimeError("could not locate the UnSAT-Rust repository root")


def _write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    temporary.replace(path)


def _parse_rules(text: str) -> list[str]:
    stripped = text.strip()
    fenced = re.search(r"```(?:json)?\s*(.*?)```", stripped, re.I | re.S)
    if fenced:
        stripped = fenced.group(1).strip()
    value = json.loads(stripped)
    items = value.get("rules") if isinstance(value, dict) else value
    if not isinstance(items, list):
        raise ValueError("response must contain a rules array")
    rules: list[str] = []
    for item in items:
        rule = item.get("rule") if isinstance(item, dict) else item
        if not isinstance(rule, str) or not rule.strip():
            raise ValueError("every extracted rule must be a non-empty string")
        rules.append(" ".join(rule.split()))
    return rules


def _source_items(document: dict[str, Any]) -> list[dict[str, Any]]:
    result = []
    files = document.get("files")
    if not isinstance(files, dict):
        raise RuntimeError("comment extraction JSON has no files object")
    for raw_path, units in files.items():
        if not isinstance(raw_path, str) or not isinstance(units, list):
            continue
        path = raw_path.replace("\\", "/")
        path = path.removeprefix("./").removeprefix("tools/")
        for unit in units:
            if not isinstance(unit, dict):
                continue
            result.append(
                {
                    "path": path,
                    "line": int(unit["line_number_start"]),
                    "name": str(unit["name"]),
                    "declaration": str(unit["declaration_text"]),
                    "parent": unit.get("parent_struct_or_trait"),
                    "documentation": str(unit["comment"]),
                }
            )
    return sorted(result, key=lambda item: (item["path"], item["line"], item["name"]))


def _extract_one(client: OpenAI | None, model: str, item: dict[str, Any], request_dir: Path) -> dict[str, Any]:
    key = f"{item['path']}:{item['line']}:{item['name']}"
    readable = re.sub(r"[^A-Za-z0-9_.-]+", "-", item["name"]).strip("-")[:48]
    safe_key = f"{hashlib.sha256(key.encode()).hexdigest()[:16]}-{readable}"
    directory = request_dir / safe_key
    directory.mkdir(parents=True, exist_ok=True)
    result_path = directory / "result.json"
    if result_path.is_file():
        previous = json.loads(result_path.read_text(encoding="utf-8"))
        if (previous.get("status") == "extracted" and previous.get("model") == model
                and all(previous.get(field) == value for field, value in item.items())
                and previous.get("prompt_sha256") == hashlib.sha256(SYSTEM_PROMPT.encode()).hexdigest()):
            return previous

    user = (
        f"Source: {item['path']}:{item['line']}\n"
        f"Item: {item['name']}\nParent: {item['parent']}\n"
        f"Declaration: {item['declaration']}\n\nDocumentation:\n{item['documentation']}"
    )
    request = {
        "model": model,
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user},
        ],
    }
    _write_json(directory / "request.json", request)
    try:
        if client is None:
            raise RuntimeError("OPENAI_API_KEY is required for LLM rule extraction")
        response = client.chat.completions.create(**request)
        raw = response.model_dump(mode="json")
        _write_json(directory / "response.json", raw)
        text = response.choices[0].message.content or ""
        (directory / "response.txt").write_text(text, encoding="utf-8")
        result = {**item, "rules": _parse_rules(text), "status": "extracted", "model": model,
                  "prompt_sha256": hashlib.sha256(SYSTEM_PROMPT.encode()).hexdigest()}
    except Exception as error:
        result = {
            **item,
            "rules": [],
            "status": "error",
            "model": model,
            "error": f"{type(error).__name__}: {error}",
        }
    _write_json(result_path, result)
    return result


def _write_rules_csv(path: Path, results: list[dict[str, Any]]) -> int:
    rows = []
    for result in results:
        for rule in result["rules"]:
            rows.append(
                {
                    "id": f"rule-{len(rows) + 1}",
                    "path": f"{result['path']}:{result['line']}",
                    "name": result["declaration"],
                    "rule": rule,
                }
            )
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=("id", "path", "name", "rule"))
        writer.writeheader()
        writer.writerows(rows)
    return len(rows)


def run(args: argparse.Namespace) -> int:
    root = _repo_root()
    output = (root / args.output_dir).resolve()
    output.mkdir(parents=True, exist_ok=True)
    source_units = output / "source-units.json"
    rules_csv = output / "rules.csv"

    if not args.skip_comment_extract:
        print("[extract] building and running extract-comment", flush=True)
        subprocess.run(
            ["cargo", "build", "--release", "--manifest-path", "tools/extract-comment/Cargo.toml"],
            cwd=root,
            check=True,
        )
        subprocess.run(
            [
                str(root / "tools/extract-comment/target/release/extract-comment"),
                "--dirs", "tools/rust/library/core",
                "--dirs", "tools/rust/library/std",
                "--dirs", "tools/rust/library/alloc",
                "--out", str(source_units),
            ],
            cwd=root,
            check=True,
        )
    if not source_units.is_file():
        raise RuntimeError(f"missing {source_units}; remove --skip-comment-extract")
    if args.comments_only:
        print(f"[extract] source units: {source_units}")
        return 0

    if not args.skip_rule_extract:
        items = _source_items(json.loads(source_units.read_text(encoding="utf-8")))
        if args.limit is not None:
            items = items[: args.limit]
        key = os.environ.get("OPENAI_API_KEY") or _load_openai_api_key_from_dotenv()
        if not key:
            raise RuntimeError("OPENAI_API_KEY is required for LLM rule extraction")
        client = OpenAI(api_key=key, timeout=180, max_retries=1)
        print(f"[extract] extracting rules from {len(items)} documented APIs", flush=True)
        results: list[dict[str, Any]] = []
        with ThreadPoolExecutor(max_workers=args.workers) as pool:
            futures = {
                pool.submit(_extract_one, client, args.model, item, output / "requests"): item
                for item in items
            }
            for index, future in enumerate(as_completed(futures), 1):
                result = future.result()
                results.append(result)
                print(f"[extract] {index}/{len(items)} {result['name']} {result['status']}", flush=True)
        results.sort(key=lambda item: (item["path"], item["line"], item["name"]))
        _write_json(output / "extraction-results.json", results)
        errors = [result for result in results if result["status"] != "extracted"]
        count = _write_rules_csv(rules_csv, results)
        print(f"[extract] wrote {count} rules to {rules_csv}", flush=True)
        if errors:
            raise RuntimeError(f"{len(errors)} LLM extraction requests failed; rerun to resume")
    if not rules_csv.is_file():
        raise RuntimeError(f"missing {rules_csv}; remove --skip-rule-extract")
    if args.skip_translate:
        return 0

    translations = output / "translations"
    inputs = translations / "inputs"
    inputs.mkdir(parents=True, exist_ok=True)
    shutil.copy2(rules_csv, inputs / "rules.csv")
    shutil.copy2(root / "operators.json", inputs / "operators.json")
    command = [
        sys.executable,
        "-m", "llm.translate_rules",
        "--output", str(translations),
        "--source-units", str(source_units),
        "--model", args.model,
        "--workers", str(args.workers),
        "--reasoning-effort", args.reasoning_effort,
    ]
    print("[extract] translating extracted rules to DSL", flush=True)
    subprocess.run(command, cwd=root, check=True)
    print(f"[extract] translations: {translations / 'generation-results.json'}")
    return 0
