# Task 2

## Goal
Rewrite the rule as a source-bound expression using only local Rust variables and ordinary operators.

Task 2 is intentionally more concrete than Task 1. It should not use project-specific DSL operator calls such as `alloc_id(...)` or `offset_in_alloc(...)`.

## Input
- Code context for one unsafe call target.
- Natural-language rule text.
- Primitive operand form: `<variable name>@<line>:<col>`.
- Ordinary operators:
	`==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

## Desired Output Format
Return exactly one DSL expression as plain text.

Rules:
- Do not return JSON.
- Do not include Markdown fences.
- Do not include prose, explanation, or multiple candidate answers.
- Every non-literal operand should be a concrete source-bound variable in the form `<name>@<line>:<col>`.
- The output must be parseable by the DSL parser under [tools/x/src/dsl/parser.py](/workspaces/UnSAT-Rust/tools/x/src/dsl/parser.py).
- The expression must not contain any configured DSL operator calls from [operators.json](/workspaces/UnSAT-Rust/operators.json).

## Validation By Sync
When sync encounters a non-placeholder `task2` DSL:
- It parses the DSL.
- It validates that the AST contains no DSL operator calls.
- If parsing or validation fails, sync prints an invalid-DLS message instead of stopping the whole sync.

## Example
Example selected primitive operands:

```text
i@329:34
len@329:20
base_offset@329:18
capacity@329:12
```

Possible output:

```text
i@329:34 >= 0 &&
i@329:34 <= len@329:20 &&
base_offset@329:18 + i@329:34 <= capacity@329:12
```