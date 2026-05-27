# Task 3

## Goal
Resolve each Task 1 operator or operand into a concrete source-level expression when possible.

Task 3 is not a single DSL expression. It is a structured resolution report that records, for each relevant operator or operand, whether it can be rewritten using local Rust variables and ordinary operators only.

## Input
- Code context for one unsafe call target.
- The Task 1 DSL, especially the operators used in that DSL.
- Ordinary operators:
	`==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

## Desired Output Format
Return exactly one JSON object.

Required top-level fields:
- `caller`: string
- `callsite`: string in `<path>:<line>:<col>` form
- `resolutions`: array

Each `resolutions` entry must be an object with:
- `operator`: string
- `expression`: string or `null`

Rules:
- If an operator or operand can be resolved locally, `expression` must be a plain DSL expression that uses only source-bound variables, literals, and ordinary operators.
- Do not include project DSL operators like `alloc_id`, `offset_in_alloc`, or `alloc_block_size` inside `expression`.
- If the operator cannot be resolved locally, set `expression` to `null`.
- Do not include explanation outside the JSON object.

## Example

```json
{
	"caller": "vec::AnonVec::remove_get",
	"callsite": "src/vec.rs:329:30",
	"resolutions": [
		{
			"operator": "alloc_id",
			"expression": null
		},
		{
			"operator": "ptr",
			"expression": "ptr@329:30"
		},
		{
			"operator": "offset_in_alloc",
			"expression": null
		},
		{
			"operator": "i",
			"expression": "i@329:34"
		},
		{
			"operator": "alloc_block_size",
			"expression": null
		}
	]
}
```

## Current Sync Relationship
Sync currently validates Task 1 and Task 2 DSL fields stored in crate metadata. Task 3 is still a structured JSON output task and is not currently DSL-validated by sync.