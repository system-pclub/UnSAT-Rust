from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path
import subprocess


from cli.cmd.extract import _repo_root, _source_items, _write_json
from cli.cmd.print_step0 import print_step0


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


def _comments(root: Path, inputs: list[tuple[Path, Path]], *, refresh: bool, include_private: bool = False) -> None:
    pending = [(source, output) for source, output in inputs
               if refresh or not (output / "step0.json").is_file()
               or (json.loads((output / "step0-input.json").read_text()).get("include_private", False)
                   if (output / "step0-input.json").exists() else False) != include_private]
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
             "--dirs", str(source.relative_to(root)),
             *(["--include-private"] if include_private else []), "--out", str(temporary)],
            cwd=root, check=True,
        )
        temporary.replace(output / "step0.json")
        _write_json(output / "step0-input.json", {"include_private": include_private})
        print(f"[rule] {source.name}: wrote {output / 'step0.json'}", flush=True)
        print_step0(output / "step0.json")


def _rules(output: Path, args: argparse.Namespace) -> None:
    from llm.rule_pipeline import extract
    extract(output, args)


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


def _translate(root: Path, output: Path, args: argparse.Namespace) -> bool:
    from llm.rule_pipeline import translate
    return translate(root, output, args)


def run(args: argparse.Namespace) -> int:
    if args.workers < 1 or (args.limit is not None and args.limit < 0):
        raise RuntimeError("--workers must be positive and --limit must be nonnegative")
    if args.retries < 0 or args.batch_poll_seconds <= 0 or args.max_output_tokens < 1:
        raise RuntimeError("retries must be nonnegative; polling interval and output tokens must be positive")
    if args.rule_limit is not None and args.rule_limit < 0:
        raise RuntimeError("--rule-limit must be nonnegative")
    root = _repo_root()
    output = (root / args.output).resolve()
    inputs = _inputs(root, output, args.directories)
    # Check all prerequisites before starting paid requests or changing catalogs.
    required = {1: ("step0.json",), 2: ("step0.json", "step1.json")}.get(args.step, ())
    for _, destination in inputs:
        for filename in required:
            if not (destination / filename).is_file():
                raise RuntimeError(f"missing {destination / filename}; run --step {int(filename[4])} first")
    manifest_path = output / "inputs.json"
    manifest = {"directories": [source.relative_to(root).as_posix() for source, _ in inputs]}
    if manifest_path.exists() and json.loads(manifest_path.read_text()) != manifest:
        raise RuntimeError("input directories/order changed; use a new output directory")
    for source, destination in inputs:
        marker = destination / "input.json"
        identity = {"directory": source.relative_to(root).as_posix()}
        if marker.is_file() and json.loads(marker.read_text(encoding="utf-8")) != identity:
            raise RuntimeError(f"{destination} belongs to a different input directory; use a new output directory")
    _write_json(manifest_path, manifest)
    for source, destination in inputs:
        _write_json(destination / "input.json", {"directory": source.relative_to(root).as_posix()})
    if args.step in (None, 0):
        _comments(root, inputs, refresh=args.step == 0, include_private=args.include_private)
    if args.step in (None, 1) and args.api_location:
        available = {f"{item['path']}:{item['line']}" for _, destination in inputs
                     for item in _source_items(json.loads((destination / "step0.json").read_text()))}
        missing = set(args.api_location) - available
        if missing:
            raise RuntimeError("unknown --api-location: " + ", ".join(sorted(missing)))
    if args.step in (None, 1):
        from llm.rule_jobs import BatchPending
        pending = []
        for _, destination in inputs:
            try:
                _rules(destination, args)
            except BatchPending as error:
                pending.append(str(error))
        if pending:
            print("[rule] " + "; ".join(pending), flush=True)
            return 75
    if args.step in (None, 1, 2):
        _catalog(output, inputs)
    if args.step in (None, 2):
        schema = root / args.dslgen
        if not (schema if schema.is_file() else schema / "schema.json").is_file():
            subprocess.run(["cargo", "run", "--locked", "--manifest-path", "tools/dslgen/Cargo.toml",
                            "--", "--out", str(schema)], cwd=root, check=True)
        from llm.rule_jobs import BatchPending
        # Global round barrier: all libraries finish this round before any
        # library submits repairs, including after a --batch-no-wait resume.
        for round_index in range(args.retries + 1):
            args._translation_round = round_index
            pending, errors, retry = [], [], False
            for _, destination in inputs:
                try:
                    retry = (_translate(root, destination, args) is True) or retry
                except BatchPending as error:
                    pending.append(str(error))
                except RuntimeError as error:
                    errors.append(str(error))
            if errors:
                raise RuntimeError("; ".join(errors))
            if pending:
                print("[rule] " + "; ".join(pending), flush=True)
                return 75
            if not retry:
                break
    return 0
