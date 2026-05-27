"""Prompts for Task 2: rewrite the rule as a source-bound expression using only local
Rust variables and ordinary operators (no project DSL operator calls)."""

from __future__ import annotations


_SYSTEM = """\
You are a Rust code analysis assistant specialising in formal safety rules.

## Goal
Rewrite a natural-language safety rule as a concrete source-bound expression.
The expression must use only local Rust variables and ordinary operators —
no project-specific DSL operator calls (such as `alloc_id`, `offset_in_alloc`, etc.).

## DSL Syntax
- Source-bound variables: `<name>@<line>:<col>`  (e.g. `i@329:34`)
- Literals: integer literals, boolean literals, `None`
- Ordinary binary / unary operators: `==`, `!=`, `<`, `<=`, `>`, `>=`,
  `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`
- Parentheses are allowed

## Output Rules
- Return exactly ONE DSL expression as plain text.
- Do NOT return JSON.
- Do NOT include Markdown fences.
- Do NOT include prose, explanation, or multiple candidate answers.
- Every non-literal operand must be a concrete source-bound variable in the form
  `<name>@<line>:<col>`.
- The expression must NOT contain any DSL operator calls.\
"""


def build_system() -> str:
    return _SYSTEM


def build_user(rule_text: str, code_context: str) -> str:
    return (
        "## Safety Rule\n"
        f"{rule_text}\n\n"
        "## Code Context\n"
        f"```rust\n{code_context}\n```\n\n"
        "Write the source-bound DSL expression for the rule above."
    )
