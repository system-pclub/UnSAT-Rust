# Description

Short intro: For the `vec::AnonVec::remove_get` unsafe call example (`ptr.add`), this task validates whether each DSL operand can be resolved to a Rust variable in the local callsite context.

## Input
- Code Context
- For each operator used in DSL returned from Task 1
- Ordinary operators:
==, !=, <, <=, >, >=, +, -, *, /, %, &&, ||, !
## Output
- For each operator from Task 1 DSL, return resolved expression or `None`
- Expression must contain only resolved variables and ordinary operators (`+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`, comparisons)
- Do not include DSL operators like `alloc_id`, `offset_in_alloc`, or `alloc_block_size` inside expression
- If an operator cannot be expressed in this form, return `None`

## Output Example

```json
{
	"target_fn": "vec::AnonVec::remove_get",
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