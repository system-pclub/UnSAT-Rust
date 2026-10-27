#!/usr/bin/env python3
"""Summarize execution coverage and CULPA alarms, without assigning TP/FP labels."""
import argparse
from collections import Counter
import csv
import json
from pathlib import Path
import re


def summarize(run_path):
    record = json.loads(run_path.read_text())
    directory = run_path.parent
    files = sorted((directory / "findings").glob("*.json"))
    findings = []
    for path in files:
        for finding in json.loads(path.read_text()):
            finding["artifact"] = str(path)
            findings.append(finding)
    # Upstream's printed "Total findings" also includes Satisfied entries.
    alarms = [f for f in findings if f["kind"] != "Satisfied" and f["risk_level"] != "Low"]
    unique = {}
    for finding in alarms:
        key = json.dumps([finding["fn_def_path"], finding["callee_path"],
                          finding["sink_loc"], finding["requirement_idx"], finding["kind"]], sort_keys=True)
        unique.setdefault(key, finding)
    alarms = list(unique.values())
    external = [f for f in alarms if "External" in f.get("evidence_dependency", {})]
    internal = [f for f in alarms if f not in external]
    status = ("completed" if record["status"] == "completed" else "partial") if files else "not_analyzed"
    log = (directory / "stderr.log").read_text(errors="replace")
    log = re.sub(r"\x1b\[[0-9;]*m", "", log)
    reason = ""
    if status != "completed":
        if record["status"] == "timeout":
            reason = "timeout"
        elif "internal compiler error" in log or "thread 'rustc' panicked" in log:
            reason = "rustc internal compiler error / panic"
        elif "requires rustc" in log or "is not supported by the following packages" in log:
            reason = "dependency requires newer rustc"
        elif "failed to run custom build command" in log:
            reason = "dependency build script failed"
        else:
            errors = [line.strip() for line in log.splitlines() if line.startswith("error:")]
            reason = errors[0][:250] if errors else "no completed CULPA callback recorded"
    row = {"crate": record["crate"], "attempt": record["attempt"], "analysis_status": status,
           "exit_code": record["exit_code"], "elapsed_seconds": record["elapsed_seconds"],
           "analysis_artifacts": len(files), "raw_rule_results": len(findings),
           "internal_alarm_requirements": len(internal), "external_warning_requirements": len(external),
           "alarm_functions": len({f["fn_def_path"] for f in alarms}),
           "internal_alarm_functions": len({f["fn_def_path"] for f in internal}),
           "confidence": dict(Counter(f["confidence"] for f in alarms)),
           "failure_reason": reason, "run_record": str(run_path)}
    return row, alarms


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("runs", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--prefer-runs", type=Path, help="Select follow-up runs for matching crates; original runs remain untouched")
    args = parser.parse_args()
    rows, flat = [], []
    paths = sorted(args.runs.rglob("run.json"))
    if args.prefer_runs:
        selected = {json.loads(p.read_text())["crate"]: p for p in paths}
        for p in sorted(args.prefer_runs.rglob("run.json")):
            selected[json.loads(p.read_text())["crate"]] = p
        paths = [selected[name] for name in sorted(selected)]
    for path in paths:
        row, alarms = summarize(path)
        rows.append(row)
        flat.extend(dict(crate=row["crate"], attempt=row["attempt"], **alarm) for alarm in alarms)
    args.output.mkdir(parents=True, exist_ok=True)
    totals = dict(Counter(r["analysis_status"] for r in rows))
    totals["attempts"] = len(rows)
    for field in ("internal_alarm_requirements", "external_warning_requirements", "alarm_functions"):
        totals[field] = sum(r[field] for r in rows)
    result = {"scope": "Raw CULPA alarms; no TP/FP adjudication. Satisfied and Low-risk suppressed entries excluded.",
              "totals": totals, "rows": rows}
    (args.output / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    (args.output / "alarms.json").write_text(json.dumps(flat, indent=2) + "\n")
    if rows:
        with (args.output / "summary.csv").open("w", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
            writer.writeheader()
            writer.writerows(rows)
    lines = ["# CULPA execution results", "", result["scope"], "",
             "Internal counts include every reported confidence level. External warnings are separate.",
             "Partial results retain alarms emitted before a failed compilation; they are not complete analyses.", "",
             "| Crate | Status | Internal requirements | External warnings | Alarm functions | Seconds | Failure |",
             "|---|---|---:|---:|---:|---:|---|"]
    for r in rows:
        lines.append(f"| {r['crate']} | {r['analysis_status']} | {r['internal_alarm_requirements']} | "
                     f"{r['external_warning_requirements']} | {r['alarm_functions']} | "
                     f"{r['elapsed_seconds']:.1f} | {r['failure_reason'].replace('|', '/')} |")
    (args.output / "summary.md").write_text("\n".join(lines) + "\n")
    print(json.dumps(totals))


if __name__ == "__main__":
    main()
