from __future__ import annotations

import csv
import json
from pathlib import Path
import re

from dsl import DSLParseError, parse_dsl
from dsl.lint import lint_dsl_ast, lint_rule_semantics


def run(args: object) -> int:
    rule_path = Path(getattr(args, "rule_dsl")).resolve()
    operators_path = Path(getattr(args, "operators")).resolve()
    rules_csv_path = Path(getattr(args, "rules_csv", "rules.csv")).resolve()
    rules_doc = json.loads(rule_path.read_text(encoding="utf-8"))
    operators_doc = json.loads(operators_path.read_text(encoding="utf-8"))
    rules = rules_doc.get("rules", rules_doc)
    operators = operators_doc.get("operators", operators_doc)
    if not isinstance(rules, dict):
        raise RuntimeError(f"rule DSL must contain an object: {rule_path}")
    if not isinstance(operators, list):
        raise RuntimeError(f"operators must contain an array: {operators_path}")

    errors = 0
    warnings = 0
    catalog: dict[str, dict[str, str]] = {}
    if not rules_csv_path.is_file():
        raise RuntimeError(f"canonical rule catalog does not exist: {rules_csv_path}")
    with rules_csv_path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            rule_id = (row.get("id") or "").strip()
            if not re.fullmatch(r"rule-\d+", rule_id):
                print(f"error noncanonical-catalog-id {rule_id}: expected rule-<number>")
                errors += 1
                continue
            if rule_id in catalog:
                print(f"error duplicate-catalog-id {rule_id}: repeated in {rules_csv_path}")
                errors += 1
                continue
            catalog[rule_id] = row

    for rule_id, entry in rules.items():
        if not re.fullmatch(r"rule-\d+", str(rule_id)):
            print(f"error noncanonical-rule-id {rule_id}: expected rule-<number>")
            errors += 1
        elif rule_id not in catalog:
            print(f"error unknown-rule-id {rule_id}: not present in {rules_csv_path}")
            errors += 1
        elif isinstance(entry, dict):
            for field in ("path", "name", "rule"):
                if entry.get(field) != catalog[rule_id].get(field):
                    print(
                        f"error catalog-metadata-mismatch {rule_id}: "
                        f"{field} differs from {rules_csv_path}"
                    )
                    errors += 1
            aliases = entry.get("aliases", [])
            if not isinstance(aliases, list) or not all(
                isinstance(alias, str) and re.fullmatch(r".+:\d+", alias)
                for alias in aliases
            ):
                print(
                    f"error invalid-rule-alias {rule_id}: aliases must be path:line strings"
                )
                errors += 1
        dsl = entry.get("dsl") if isinstance(entry, dict) else entry
        if not isinstance(dsl, str) or not dsl.strip():
            print(f"error missing-dsl {rule_id}: missing non-empty DSL")
            errors += 1
            continue
        try:
            ast = parse_dsl(dsl, operators)
        except DSLParseError as exc:
            print(f"error parse-error {rule_id}: {exc}")
            errors += 1
            continue
        rule_text = entry.get("rule") if isinstance(entry, dict) else None
        issues = lint_dsl_ast(ast, operators)
        if isinstance(rule_text, str):
            issues.extend(lint_rule_semantics(rule_text, ast))
        for issue in issues:
            print(f"{issue.severity} {issue.code} {rule_id}: {issue.message}")
            if issue.severity == "error":
                errors += 1
            else:
                warnings += 1

    print(f"linted {len(rules)} rules: {errors} error(s), {warnings} warning(s)")
    return 1 if errors else 0
