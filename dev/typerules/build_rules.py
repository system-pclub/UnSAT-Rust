#!/usr/bin/env python3
"""Build dev/typerules/rules.json from generated source-derived DSL.

Each case names a legacy root rule (the catalog used to select callsites) and a
generated ``eval/rule`` rule (the new DSL). The output follows the shape of
``ptr_rule_dsl.json`` so ``./x verify --rule-dsl`` accepts it, plus a lowered
``dsl_v2`` document for KLEE. The generated catalog is read-only evidence.

Run inside the unsat-rust image::

    python dev/typerules/build_rules.py
"""
from __future__ import annotations

import csv
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/x/src"))

import dslv2  # noqa: E402
from dsltrans import Model  # noqa: E402

SCHEMA = ROOT / "tools/x/generated/dsl/schema.json"
HERE = Path(__file__).resolve().parent


def load_root_rules():
    with (ROOT / "rules.csv").open(newline="", encoding="utf-8") as handle:
        return {row["id"]: row for row in csv.DictReader(handle)}


def load_generated(library, rule_id):
    for row in json.loads((ROOT / "eval/rule" / library / "step2.json").read_text()):
        if row["id"] == rule_id:
            return row
    raise SystemExit(f"generated rule not found: {library}:{rule_id}")


def main():
    cases = json.loads((HERE / "cases.json").read_text())
    root_rules = load_root_rules()
    rules = {}
    for case in cases:
        # `path_rule` lets several rules share one legacy callee entry.
        root = root_rules[case.get("path_rule", case["root_rule"])]
        library, generated_id = case["generated"].split(":")
        generated = load_generated(library, generated_id)
        expression = case.get("dsl") or generated["expression"]
        api = case.get("api") or generated["api"]
        model = Model(SCHEMA, api, generated["rule"])
        document = dslv2.lower(model, expression)
        rules[case["root_rule"]] = {
            "path": root["path"],
            "name": root["name"],
            "rule": case.get("rule", root["rule"]),
            "dsl": expression.strip(),
            "dsl_v2": document,
            "generated_rule": case["generated"],
            "api": api,
            "case": case["case"],
        }
        if case.get("note"):
            rules[case["root_rule"]]["note"] = case["note"]
        print(f"{case['root_rule']}: {expression.strip()}")
    output = {
        "description": "Type-rule experiments: root rule ids select callsites; dsl_v2 is lowered source-derived DSL for KLEE.",
        "rules": rules,
    }
    (HERE / "rules.json").write_text(json.dumps(output, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
