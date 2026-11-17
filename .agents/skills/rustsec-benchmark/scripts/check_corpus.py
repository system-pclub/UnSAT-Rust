#!/usr/bin/env python3
"""Check the pinned corpus and batch selections without compiling or running KLEE."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[4]
BENCHMARK = ROOT / "benchmark/rustsec"
SUPPORT = ROOT / "dev/rustsec"


def read_json(path):
    return json.loads(path.read_text())


def case_keys(cases):
    return {(case["crate"], case["version"], case["advisory"]) for case in cases}


def check(check_selections=False, rustc=None):
    # Benchmark inputs contain only crate directories, for every benchmark.
    layout_errors = []
    for benchmark in (ROOT / "benchmark").iterdir():
        if not benchmark.is_dir():
            continue
        for entry in benchmark.iterdir():
            if not entry.is_dir() or not (entry / "Cargo.toml").is_file():
                layout_errors.append(f"non-crate benchmark entry: {entry.relative_to(ROOT)}")
    allowlist = read_json(SUPPORT / "allowlist.json")
    cases = allowlist["cases"]
    keys = case_keys(cases)
    errors = layout_errors
    if len(keys) != len(cases):
        errors.append("duplicate advisory/crate/version records in allowlist")
    allowed = {f'{case["crate"]}-{case["version"]}' for case in cases}
    actual = {path.name for path in BENCHMARK.iterdir() if (path / "Cargo.toml").is_file()}
    if actual != allowed:
        errors.append(f"crate directories: missing={sorted(allowed - actual)}, extra={sorted(actual - allowed)}")
    for case in cases:
        expected = f'benchmark/rustsec/{case["crate"]}-{case["version"]}'
        if case["directory"] != expected:
            errors.append(f"wrong directory for {case['crate']}: {case['directory']}")
    for name in actual & allowed:
        package = tomllib.loads((BENCHMARK / name / "Cargo.toml").read_text())["package"]
        if f'{package["name"]}-{package["version"]}' != name:
            errors.append(f"Cargo package identity differs from directory: {name}")
    for filename in ("manifest.json", "cases.json"):
        entries = read_json(SUPPORT / filename)["cases"]
        if len(entries) != len(cases) or case_keys(entries) != keys:
            errors.append(f"{filename} does not match allowlist")
    for path in (SUPPORT / "evidence").glob("*/provenance.json"):
        entry = read_json(path)
        if case_keys([entry]) - keys or entry["advisory"] != path.parent.name:
            errors.append(f"stale provenance: {path.relative_to(ROOT)}")
    advisory_ids = {case["advisory"] for case in cases}
    extra_evidence = {path.name for path in (SUPPORT / "evidence").iterdir() if path.is_dir()} - advisory_ids
    if extra_evidence:
        errors.append(f"stale evidence directories: {sorted(extra_evidence)}")
    extra_targets = {path.stem for path in (SUPPORT / "targets").glob("*.txt")} - allowed
    if extra_targets:
        errors.append(f"stale target files: {sorted(extra_targets)}")
    pin = tomllib.loads((SUPPORT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    if pin != "1.87.0" or allowlist["rust_version"] != "1.87.0":
        errors.append("Rust version must be pinned to 1.87.0")
    if rustc:
        version = subprocess.check_output([str(rustc), "--version"], text=True).strip()
        release = version.split()[1]
        if release != "1.87.0" and not release.startswith("1.87.0-"):
            errors.append(f"wrong compiler release: {version}")
        print(version)
    if check_selections:
        spec = importlib.util.spec_from_file_location("rustsec_batch", SUPPORT / "run.py")
        batch = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(batch)
        if set(batch.ADVISORY_TARGETS) != allowed:
            errors.append("batch selections do not match allowlist")
        dsl = read_json(ROOT / "ptr_rule_dsl.json")
        known = set(dsl.get("rules", dsl)) | {"__raw_ptr_deref__"}
        for name in sorted(allowed):
            pairs = batch.selected_pairs(name) if name in batch.ADVISORY_TARGETS else set()
            if name not in batch.UNSUPPORTED and not pairs:
                errors.append(f"empty supported selection: {name}")
            unknown = {rule for _, rule in pairs} - known
            if unknown:
                errors.append(f"unknown DSL rules for {name}: {sorted(unknown)}")
        for mapping in (batch.UNSUPPORTED, batch.FEATURES, batch.MIRSCAN_ENV,
                        batch.INSTANTIATIONS, batch.RELEASE_ONLY):
            if set(mapping) - allowed:
                errors.append(f"retired batch settings: {sorted(set(mapping) - allowed)}")
        for path in (SUPPORT / "targets").glob("*.txt"):
            expected = set(batch.ADVISORY_TARGETS.get(path.stem, {}))
            if set(path.read_text().splitlines()) != expected:
                errors.append(f"stale target contents: {path.name}")
        evidence_path = SUPPORT / "selection-evidence.json"
        if evidence_path.exists():
            for record in read_json(evidence_path)["records"]:
                name = record["crate"]
                current = batch.ADVISORY_TARGETS.get(name, {})
                if set(record["targets"]) != set(current):
                    errors.append(f"stale callsite evidence: {name}")
                if record["features"] != batch.FEATURES.get(name, ""):
                    errors.append(f"stale feature evidence: {name}")
                for callsite, target in record["targets"].items():
                    source = BENCHMARK / name / target["caller"]["path"]
                    if not source.is_file() or hashlib.sha256(source.read_bytes()).hexdigest() != target["source_sha256"]:
                        errors.append(f"source differs from callsite evidence: {name}/{callsite}")
                    if tuple(target["rules"]) != current.get(callsite):
                        errors.append(f"stale rule evidence: {name}/{callsite}")
        for name, reason in sorted(batch.UNSUPPORTED.items()):
            print(f"UNSUPPORTED {name}: {reason}")
    for error in errors:
        print(f"ERROR: {error}")
    print(f"{'FAIL' if errors else 'PASS'}: {len(actual)}/{len(allowed)} crate versions; {len(cases)} advisory records")
    return int(bool(errors))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check-selections", action="store_true")
    parser.add_argument("--rustc", type=Path, help="Check the actual experiment compiler's release")
    args = parser.parse_args()
    return check(args.check_selections, args.rustc)


if __name__ == "__main__":
    raise SystemExit(main())
