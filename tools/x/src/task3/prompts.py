"""Prompts for Task 3: resolve each Task 1 operator or operand into a concrete
source-level expression when possible, producing a structured JSON report."""

from __future__ import annotations


_SYSTEM = """\
You are a Rust code analysis assistant specialising in formal safety rules.

## Goal
Given the Task 1 DSL expression for a safety rule, produce a structured resolution
report that records, for each DSL operator or operand in that expression, whether it
can be rewritten using only local Rust variables and ordinary operators.

## Output Format
Return exactly one JSON object with the following top-level fields:
- `caller`  — string: the fully-qualified name of the caller function
- `callsite`   — string: `<path>:<line>:<col>`
- `resolutions` — array of objects, each with:
    - `operator`   — string: the operator or operand name from the Task 1 DSL
    - `expression` — string (a plain DSL expression using only source-bound variables,
                     literals, and ordinary operators) or `null` if the operator
                     cannot be resolved locally

## Rules
- If an operator/operand can be resolved locally, `expression` must not contain
  any project DSL operator calls (e.g. `alloc_id`, `offset_in_alloc`).
- If the operator cannot be resolved locally, set `expression` to `null`.
- Do not include explanation outside the JSON object.
- Do not wrap the JSON in Markdown fences.\
"""


def build_system() -> str:
    return _SYSTEM


def build_user(code_context: str, task1_dsl: str) -> str:
    return (
        "## Code Context\n"
        f"```rust\n{code_context}\n```\n\n"
        "## Task 1 DSL Expression\n"
        f"```\n{task1_dsl}\n```\n\n"
        "Produce the resolution report JSON object."
    )
