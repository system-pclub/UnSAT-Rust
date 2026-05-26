# Description

Short intro: Using the same unsafe call in `vec::AnonVec::remove_get` (`ptr.add` at line 329), this task binds DSL operands to concrete Rust source variables with source positions.

## Input
- Code Context
- Natural Language Rule
- Pimitive operator: select Rust variable ( <variable name>@<line number>:<col number>)
- Ordinary operators:
==, !=, <, <=, >, >=, +, -, *, /, %, &&, ||, !

## Output
- DSL with the operator

## Output Example

Example selected primitive operands:

```text
i@329:34
len@329:20
base_offset@329:18
capacity@329:12
```

Example DSL with bound operands:

```text
i@329:34 >= 0 &&
i@329:34 <= len@329:20 &&
base_offset@329:18 + i@329:34 <= capacity@329:12
```