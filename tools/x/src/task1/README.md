# Task 1

## Goal
Translate a natural-language safety rule into the project DSL.

This task may use project-specific DSL operators from [operators.json](/workspaces/UnSAT-Rust/operators.json) together with ordinary logical and arithmetic operators. The output should capture the rule semantically, even if some operands are still abstract or only partially bound to source variables.

## Input
- Code context for one unsafe call target.
- Natural-language rule text.
- Allowed DSL operators from [operators.json](/workspaces/UnSAT-Rust/operators.json).
- Ordinary operators:
	`==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

## Desired Output Format
Return exactly one DSL expression as plain text.

Rules:
- Do not wrap the answer in JSON.
- Do not wrap the answer in Markdown fences.
- Do not include explanation, prose, bullets, or multiple alternatives.
- The output must be parseable by the DSL parser under [tools/x/src/dsl/parser.py](/workspaces/UnSAT-Rust/tools/x/src/dsl/parser.py).
- DSL operator calls must use function-call form such as `alloc_id(ptr@329:30)`.
- Source-bound operands must use `<name>@<line>:<col>` such as `i@329:34`.
- `None`, integer literals, boolean literals, parentheses, unary operators, and binary operators are allowed where syntactically valid.

## Validation By Sync
When sync encounters a non-placeholder `task1` DSL:
- It parses the DSL.
- It validates that the AST contains at least one DSL operator call.
- It lists all operator calls used in the AST.
- If a previously unknown operator appears, sync merges it into [operators.json](/workspaces/UnSAT-Rust/operators.json).
- If parsing or validation fails, sync prints an invalid-DLS message instead of stopping the whole sync.

## Current Documented DSL Operators

### `alloc_id(p)`
Returns the allocation/provenance object of pointer or reference `p`, or `None`.

### `offset_in_alloc(p)`
Returns the byte offset of pointer or reference `p` from the beginning of its allocation.

### `alloc_live(a, site)`
Returns whether allocation `a` is live at program point `site`.

### `alloc_allocator(a)`
Returns the allocator identity that created allocation `a`.

### `alloc_layout(a)`
Returns the original layout of allocation `a`.

### `alloc_block_size(a)`
Returns the current usable size in bytes of allocation `a`.

### `size_of(x)`
Returns the size component of a layout-like or type-like value.

### `align_of(x)`
Returns the alignment component of a layout-like or type-like value.

### `is_power_of_two(x)`
Returns whether integer `x` is a power of two.

## Example
Natural-language rule:

`ptr.add(i)` is valid only when `i` stays within the initialized range and does not overflow allocation bounds.

Possible output:

```text
alloc_id(ptr@329:30) != None &&
offset_in_alloc(ptr@329:30) + i@329:34 >= 0 &&
offset_in_alloc(ptr@329:30) + i@329:34 <= alloc_block_size(alloc_id(ptr@329:30))
```