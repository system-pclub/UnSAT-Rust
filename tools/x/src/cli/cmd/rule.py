from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from openai import OpenAI

from cli.cmd.extract import _extract_one, _repo_root, _source_items, _write_json
from cli.cmd.print_step0 import print_step0
from llm.openai_llm import _load_openai_api_key_from_dotenv


def _inputs(root: Path, output: Path, directories: list[str]) -> list[tuple[Path, Path]]:
    inputs = []
    names: set[str] = set()
    for directory in directories:
        source = (root / directory).resolve()
        if not source.is_dir():
            raise RuntimeError(f"input directory does not exist: {directory}")
        if not source.is_relative_to(root):
            raise RuntimeError(f"input directory must be inside the mounted repository: {directory}")
        if source.name in names:
            raise RuntimeError(f"input directories have the same output name: {source.name}")
        destination = output / source.name
        if output.is_relative_to(source) or source.is_relative_to(destination):
            raise RuntimeError(f"input and output directories overlap: {directory}")
        names.add(source.name)
        inputs.append((source, destination))
    return inputs


def _comments(root: Path, inputs: list[tuple[Path, Path]], *, refresh: bool) -> None:
    pending = [(source, output) for source, output in inputs
               if refresh or not (output / "step0.json").is_file()]
    if not pending:
        return
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", "tools/extract-comment/Cargo.toml"],
        cwd=root, check=True,
    )
    for source, output in pending:
        output.mkdir(parents=True, exist_ok=True)
        temporary = output / "step0.tmp.json"
        subprocess.run(
            [str(root / "tools/extract-comment/target/release/extract-comment"),
             "--dirs", str(source.relative_to(root)), "--out", str(temporary)],
            cwd=root, check=True,
        )
        temporary.replace(output / "step0.json")
        print(f"[rule] {source.name}: wrote {output / 'step0.json'}", flush=True)
        print_step0(output / "step0.json")


def _rules(output: Path, args: argparse.Namespace) -> None:
    items = _source_items(json.loads((output / "step0.json").read_text(encoding="utf-8")))
    if args.limit is not None:
        items = items[:args.limit]
    # A completed per-item result can be reused without creating an API client.
    # _extract_one checks the source, model and prompt before accepting its cache.
    key = os.environ.get("OPENAI_API_KEY") or _load_openai_api_key_from_dotenv()
    client = OpenAI(api_key=key, timeout=180, max_retries=1) if key else None
    results = []
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [pool.submit(_extract_one, client, args.model, item, output / "requests")
                   for item in items]
        for index, future in enumerate(as_completed(futures), 1):
            result = future.result()
            results.append(result)
            print(f"[rule] {output.name}: {index}/{len(items)} {result['name']} {result['status']}", flush=True)
    results.sort(key=lambda item: (item["path"], item["line"], item["name"]))
    _write_json(output / "step1.json", results)
    errors = [result for result in results if result["status"] != "extracted"]
    if errors:
        raise RuntimeError(f"{len(errors)} extraction requests failed in {output}; rerun to resume")


def _catalog(output: Path, inputs: list[tuple[Path, Path]]) -> None:
    rows = []
    catalogs = []
    for _, destination in inputs:
        results = json.loads((destination / "step1.json").read_text(encoding="utf-8"))
        if any(result["status"] != "extracted" for result in results):
            raise RuntimeError(f"incomplete {destination / 'step1.json'}; rerun --step 1")
        current_items = {
            (item["path"], item["line"], item["name"]): item
            for item in _source_items(json.loads((destination / "step0.json").read_text(encoding="utf-8")))
        }
        for result in results:
            current = current_items.get((result["path"], result["line"], result["name"]))
            if current is None or any(result.get(key) != value for key, value in current.items()):
                raise RuntimeError(f"stale {destination / 'step1.json'}; rerun --step 1")
        local_rows = []
        for result in results:
            for rule in result["rules"]:
                row = {"id": f"rule-{len(rows) + 1}",
                       "path": f"{result['path']}:{result['line']}",
                       "name": result["declaration"], "rule": rule}
                rows.append(row)
                local_rows.append(row)
        catalogs.append((destination / "rules.csv", local_rows))
    for path, local_rows in catalogs:
        _write_csv(path, local_rows)
    _write_csv(output / "rules.csv", rows)
    print(f"[rule] wrote {len(rows)} rules to {output / 'rules.csv'}", flush=True)


def _write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(".csv.tmp")
    with temporary.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=("id", "path", "name", "rule"))
        writer.writeheader()
        writer.writerows(rows)
    temporary.replace(path)


def _translate(root: Path, output: Path, args: argparse.Namespace) -> None:
    # Keep independent translation caches when catalog IDs, source or settings change.
    from llm import translate_rules
    from dsl.parser import _GRAMMAR

    fingerprint = hashlib.sha256()
    for content in ((output / "rules.csv").read_bytes(), (output / "step0.json").read_bytes(),
                    (root / "operators.json").read_bytes(), Path(translate_rules.__file__).read_bytes(),
                    _GRAMMAR.encode(), args.model.encode(), args.reasoning_effort.encode()):
        fingerprint.update(len(content).to_bytes(8, "big"))
        fingerprint.update(content)
    translations = output / "translations" / fingerprint.hexdigest()[:16]
    inputs = translations / "inputs"
    inputs.mkdir(parents=True, exist_ok=True)
    shutil.copy2(output / "rules.csv", inputs / "rules.csv")
    shutil.copy2(root / "operators.json", inputs / "operators.json")
    subprocess.run(
        [sys.executable, "-m", "llm.translate_rules", "--output", str(translations),
         "--source-units", str(output / "step0.json"), "--model", args.model,
         "--workers", str(args.workers), "--reasoning-effort", args.reasoning_effort],
        cwd=root, check=True,
    )
    results = json.loads((translations / "generation-results.json").read_text(encoding="utf-8"))
    _write_json(output / "step2.json", results)
    if any(result["status"] != "translated" for result in results):
        raise RuntimeError(f"incomplete translations in {output / 'step2.json'}; rerun --step 2")


def run(args: argparse.Namespace) -> int:
    if args.workers < 1 or (args.limit is not None and args.limit < 0):
        raise RuntimeError("--workers must be positive and --limit must be nonnegative")
    root = _repo_root()
    output = (root / args.output).resolve()
    inputs = _inputs(root, output, args.directories)
    # Check all prerequisites before starting paid requests or changing catalogs.
    required = {1: ("step0.json",), 2: ("step0.json", "step1.json")}.get(args.step, ())
    for _, destination in inputs:
        for filename in required:
            if not (destination / filename).is_file():
                raise RuntimeError(f"missing {destination / filename}; run --step {int(filename[4])} first")
    for source, destination in inputs:
        marker = destination / "input.json"
        identity = {"directory": source.relative_to(root).as_posix()}
        if marker.is_file() and json.loads(marker.read_text(encoding="utf-8")) != identity:
            raise RuntimeError(f"{destination} belongs to a different input directory; use a new output directory")
    for source, destination in inputs:
        _write_json(destination / "input.json", {"directory": source.relative_to(root).as_posix()})
    if args.step in (None, 0):
        _comments(root, inputs, refresh=args.step == 0)
    if args.step in (None, 1):
        for _, destination in inputs:
            _rules(destination, args)
    if args.step in (None, 1, 2):
        _catalog(output, inputs)
    if args.step in (None, 2):
        for _, destination in inputs:
            _translate(root, destination, args)
    return 0
