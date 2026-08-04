from __future__ import annotations

import json
from pathlib import Path

from dsl import DSLParseError, parse_dsl
from dsl.lint import lint_dsl_ast, lint_rule_semantics


def run(args: object) -> int:
    rule_path = Path(getattr(args, "rule_dsl")).resolve()
    operators_path = Path(getattr(args, "operators")).resolve()
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
    for rule_id, entry in rules.items():
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
