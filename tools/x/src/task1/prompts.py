"""Prompts for Task 1: translate a natural-language safety rule into the project DSL."""

from __future__ import annotations

import json


_SYSTEM = """\
You are a Rust code analysis assistant specialising in formal safety rules.

## Goal
Translate a natural-language safety rule into one expression written in the project DSL.

## DSL Syntax
- Function-call form for DSL operators: `operator_name(arg@line:col)`
- Source-bound variables: `<name>@<line>:<col>`  (e.g. `i@329:34`)
- Literals: `None`, integer literals, boolean literals
- Ordinary binary / unary operators: `==`, `!=`, `<`, `<=`, `>`, `>=`,
  `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`
- Parentheses are allowed

## Output Rules
- Return exactly ONE DSL expression as plain text.
- Do NOT wrap the answer in JSON.
- Do NOT wrap the answer in Markdown fences.
- Do NOT include explanation, prose, bullets, or multiple alternatives.
- The expression must contain at least one DSL operator call.
- Operands that reference local Rust variables must use `<name>@<line>:<col>` form.\
"""


def build_system() -> str:
    return _SYSTEM


def build_user(
    rule_text: str,
    code_context: str,
    operators: list[dict],
) -> str:
    operators_text = json.dumps(
        [
            {
                "name": op.get("name"),
                "description": op.get("description", ""),
                "input": op.get("input", []),
                "output": op.get("output", {}),
            }
            for op in operators
        ],
        indent=2,
    )

    return (
        "## Safety Rule\n"
        f"{rule_text}\n\n"
        "## Code Context\n"
        f"```rust\n{code_context}\n```\n\n"
        "## Available DSL Operators\n"
        f"```json\n{operators_text}\n```\n\n"
        "Write the DSL expression for the rule above."
    )
