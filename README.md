## Directory
- ./crates: A list of real world rust crates
- ./meta:   A list of metadata for each crate in ./crates
- ./rust:   std and core library from Rust compiler
- ./tools:  Helper tools for using this dataset

## Instructions

### Task 1

Goal: translate a natural-language safety rule into the project DSL.

Input:
- Code context for one unsafe call target
- Natural-language rule text
- Allowed DSL operators from [operators.json](/workspaces/UnSAT-Rust/operators.json)
- Ordinary operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

Desired output format:
- Return exactly one DSL expression as plain text
- Do not return JSON
- Do not include Markdown fences
- Do not include explanation or multiple alternatives
- DSL operator calls must use function-call form such as `alloc_id(ptr@329:30)`
- Source-bound operands must use `<name>@<line>:<col>` such as `i@329:34`
- The output must be parseable by [tools/x/src/dsl/parser.py](/workspaces/UnSAT-Rust/tools/x/src/dsl/parser.py)

Sync behavior for non-placeholder `task1`:
- Sync parses the DSL
- Sync validates that the AST contains at least one DSL operator call
- Sync lists operator calls used in the AST
- If a new operator is found, sync merges it into [operators.json](/workspaces/UnSAT-Rust/operators.json)
- If parsing or validation fails, sync prints an invalid-DLS message and continues

Documented Task 1 operators:
- `alloc_id(p)`: returns the allocation/provenance object of pointer or reference `p`, or `None`
- `offset_in_alloc(p)`: returns the byte offset of pointer or reference `p` from the start of its allocation
- `alloc_live(a, site)`: returns whether allocation `a` is live at program point `site`
- `alloc_allocator(a)`: returns the allocator identity that created allocation `a`
- `alloc_layout(a)`: returns the original layout of allocation `a`
- `alloc_block_size(a)`: returns the current usable size in bytes of allocation `a`
- `size_of(x)`: returns the size component of a layout-like or type-like value
- `align_of(x)`: returns the alignment component of a layout-like or type-like value
- `is_power_of_two(x)`: returns whether integer `x` is a power of two

Example output:

```text
alloc_id(ptr@329:30) != None &&
offset_in_alloc(ptr@329:30) + i@329:34 >= 0 &&
offset_in_alloc(ptr@329:30) + i@329:34 <= alloc_block_size(alloc_id(ptr@329:30))
```

### Task 2

Goal: rewrite the rule as a source-bound expression using only local Rust variables and ordinary operators.

Input:
- Code context for one unsafe call target
- Natural-language rule text
- Primitive operand form `<variable name>@<line>:<col>`
- Ordinary operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

Desired output format:
- Return exactly one DSL expression as plain text
- Do not return JSON
- Do not include Markdown fences
- Do not include explanation or multiple candidate answers
- Every non-literal operand should be a source-bound variable in `<name>@<line>:<col>` form
- The output must be parseable by [tools/x/src/dsl/parser.py](/workspaces/UnSAT-Rust/tools/x/src/dsl/parser.py)
- The expression must not contain any configured DSL operator calls from [operators.json](/workspaces/UnSAT-Rust/operators.json)

Sync behavior for non-placeholder `task2`:
- Sync parses the DSL
- Sync validates that the AST contains no DSL operator calls
- If parsing or validation fails, sync prints an invalid-DLS message and continues

Example output:

```text
i@329:34 >= 0 &&
i@329:34 <= len@329:20 &&
base_offset@329:18 + i@329:34 <= capacity@329:12
```

### Task 3

Goal: resolve each Task 1 operator or operand into a concrete source-level expression when possible.

Input:
- Code context for one unsafe call target
- The Task 1 DSL, especially the operators used in that DSL
- Ordinary operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `/`, `%`, `&&`, `||`, `!`

Desired output format:
- Return exactly one JSON object
- Required top-level fields:
	- `caller`: string
	- `callsite`: string in `<path>:<line>:<col>` form
	- `resolutions`: array
- Each `resolutions` entry must contain:
	- `operator`: string
	- `expression`: string or `null`
- `expression` must use only source-bound variables, literals, and ordinary operators
- Do not include project DSL operators like `alloc_id`, `offset_in_alloc`, or `alloc_block_size` inside `expression`
- If an operator cannot be resolved locally, set `expression` to `null`

Example output:

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

Current sync relationship:
- Sync currently validates Task 1 and Task 2 DSL fields stored in crate metadata
- Task 3 is not currently DSL-validated by sync