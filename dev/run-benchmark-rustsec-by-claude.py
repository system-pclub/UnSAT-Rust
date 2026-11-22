#!/usr/bin/env python3
"""Judge pinned RustSec advisories using one reviewed, verbatim rules.csv rule each."""
import argparse
import csv
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[1]
MODEL = "sonnet"
TIMEOUT = 600
SELECTION = ROOT / "dev/rustsec/claude-cases.json"
spec = importlib.util.spec_from_file_location("claude_runner", ROOT / "docker/rust-comparison/run-claude.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def load_cases():
    allowlist = json.loads((ROOT / "dev/rustsec/allowlist.json").read_text())["cases"]
    cases = json.loads(SELECTION.read_text())["cases"]
    allowed = {c["advisory"]: c["directory"] for c in allowlist}
    if len(cases) != len(allowed) or {c["advisory"]: c["directory"] for c in cases} != allowed:
        raise ValueError("Claude selection must cover the exact allowlist once per advisory")
    with (ROOT / "rules.csv").open(newline="") as stream:
        rows = list(csv.DictReader(stream))
    rules = {r["id"]: r for r in rows}
    if len(rules) != len(rows):
        raise ValueError("Duplicate rules.csv IDs")
    prepared = []
    for case in cases:
        case = dict(case)
        if case["rule_id"] is not None:
            # Do not prefix IDs, join rules, paraphrase, strip, or add a fallback.
            row = rules[case["rule_id"]]
            if case["rule_api"] != {key: row[key] for key in ("path", "name")}:
                raise ValueError(f"Reviewed CSV API changed: {case['advisory']}")
            case["rule"] = row["rule"]
            case["rule_source"] = rules[case["rule_id"]]
            path, line = case["source_location"].rsplit(":", 1)
            source = ROOT / case["directory"] / path
            if not source.is_file() or not 0 < int(line) <= len(source.read_text().splitlines()):
                raise ValueError(f"Invalid selected source location: {case}")
            digest = hashlib.sha256(source.read_bytes()).hexdigest()
            if digest != case["callsite"]["source_sha256"]:
                raise ValueError(f"Reviewed source changed: {source}")
            if source.read_text().splitlines()[int(line)-1].strip() != case["callsite"]["source_line"]:
                raise ValueError(f"Reviewed callsite moved: {case['advisory']}")
            case["source_sha256"] = digest
            case["unsafe_api_input"] = (f"{case['unsafe_api']} at {case['source_location']}: "
                                        f"{case['callsite']['source_line']}")
        prepared.append(case)
    return prepared


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="Show reviewed inputs without Docker or API calls")
    parser.add_argument("--snippets-manifest", type=Path,
                        help="Use only the prepared function/struct source file for each case")
    args = parser.parse_args()
    cases = load_cases()
    if args.snippets_manifest:
        snippets = json.loads(args.snippets_manifest.read_text())["cases"]
        by_id = {record["advisory"]: record for record in snippets}
        if len(snippets) != len(cases) or set(by_id) != {case["advisory"] for case in cases}:
            raise ValueError("Snippet manifest must cover all 38 advisories exactly once")
        for case in cases:
            snippet = by_id[case["advisory"]]
            path = ROOT / snippet["file"]
            if hashlib.sha256(path.read_bytes()).hexdigest() != snippet["sha256"]:
                raise ValueError(f"Snippet changed: {path}")
            if path.read_text().splitlines()[snippet["line"] - 1].strip() != case["callsite"]["source_line"]:
                raise ValueError(f"Selected API missing in snippet: {path}")
            case["snippet"] = snippet
            case["source_input"] = snippet["file"]
            case["unsafe_api_input"] = (f"{case['unsafe_api']} at input.rs:{snippet['line']}: "
                                        f"{case['callsite']['source_line']}")
    if args.dry_run:
        print(json.dumps(cases, indent=2, ensure_ascii=False))
        return 0
    runner.api_key()
    runner.build_image()
    output_base = "eval/rustsec-claude-snippets/runs" if args.snippets_manifest else "eval/rustsec-claude"
    output = ROOT / output_base / datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output.mkdir(parents=True)
    counts = {"true": 0, "false": 0, "no_matching_rule": 0, "reported_defect_not_in_snapshot": 0, "error": 0, "timeout": 0}
    records = []
    for case in cases:
        record = {"advisory": case["advisory"], "selection": case}
        if case["rule_id"] is None:
            status = case["review_status"]
        else:
            source_input = case.get("source_input", case["directory"])
            request = {"crate": source_input, "rule": case["rule"],
                       "target_function": case["target_function"], "unsafe_api": case["unsafe_api_input"],
                       "model": MODEL, "timeout": TIMEOUT}
            record["request"] = request
            try:
                result = runner.judge(ROOT / source_input, case["rule"], case["target_function"],
                                      case["unsafe_api_input"], model=MODEL, timeout=TIMEOUT)
                record.update(result)
                status = "true" if result["violation"] else "false"
            except subprocess.TimeoutExpired:
                status = "timeout"
            except (ValueError, RuntimeError, OSError, subprocess.SubprocessError) as exc:
                status = "error"
                record["error"] = str(exc)
        record["status"] = status
        counts[status] += 1
        records.append(record)
        (output / f"{case['advisory']}.json").write_text(json.dumps(record, indent=2) + "\n")
        summary = {"input_policy": "one verbatim rules.csv rule bound to the original reported faulty API callsite; no substitution",
                   "source_mode": "functions_and_structs" if args.snippets_manifest else "full_crate",
                   "snippets_manifest": str(args.snippets_manifest) if args.snippets_manifest else None,
                   "rules_csv_sha256": hashlib.sha256((ROOT / "rules.csv").read_bytes()).hexdigest(),
                   "selection_sha256": hashlib.sha256(SELECTION.read_bytes()).hexdigest(),
                   "total": len(cases), "completed": len(records), "counts": counts, "results": records}
        (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(f"{case['advisory']} {Path(case['directory']).name}: {status}", flush=True)
    print(json.dumps(counts), flush=True)
    print(f"Results: {output}")
    return int(bool(counts["error"] or counts["timeout"]))


if __name__ == "__main__":
    sys.exit(main())
