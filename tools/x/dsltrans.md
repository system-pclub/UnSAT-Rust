# API-specific safety-rule DSL

`tools/x/src/dsltrans.py` reads the shared core/alloc/std declaration catalog and accepts
`--api` and `--rule` strings. It uses `gpt-6.1-sol` by default.

## Grammar assembly

1. Read dslgen's `schema.json` library declaration catalog and the separately
   modeled `dslmemory.json` domain. dslgen emits no general EBNF.
2. Select API-relevant library types and their field/alias/generic dependencies.
3. Parse the API signature into fixed `$1`, `$2`, ... inputs. Number value
   parameters (receiver included), enclosing then method generics, and an explicit
   non-unit return value last. The header maps every token to its source meaning.
4. Add modeled storage/pointee/allocation projections and generate literal
   field/index/variant productions reachable from these inputs.
   Join them to the fixed Starlark-style expression template and formula start.
5. Ask the model to translate the atomic requirement using the fixed inputs.
6. Infer comprehension-local types and compile an exact executable Lark grammar.
   Check fields, scopes, operand types, enum guards and the Boolean result.

The prompt's finite declaration-family graph approximates recursive generics;
exact substitutions are checked against the source catalog after translation.
Private source fields are visible. Ordinary names are comprehension locals only.
Type and lifetime generic slots are opaque parameter handles. Const generic slots
are values of their declared types. Type handles expose no compiler syntax tree,
evaluated layout, trait solver, memory state, or execution history.

The supported subset has dot/subscript access, mathematical integer arithmetic,
comparisons, Boolean logic, conditionals, lists/comprehensions, `all`, `any`, `len`
and `is_variant`. Array/slice length counts elements; Rust str length counts UTF-8
bytes. Enum payload access requires a matching variant guard.

## Example

```text
API: core::array::IntoIter::<T,N>::new_unchecked(
    buffer: [core::mem::MaybeUninit<T>; N], initialized: core::ops::Range<usize>)

$1 is buffer: [core::mem::MaybeUninit<T>; N] [argument]
$2 is initialized: core::ops::Range<usize> [argument]
$3 is T: type metadata [type_parameter; opaque]
$4 is N: usize [const_parameter]

English: The range must be in-bounds for the buffer, with initialized.end <= N.
DSL: $2.end <= $4
```

For `fn f(v: Vec<u8>, layout: Layout)`, literal productions accept `$1.len` and
`$2.size` and reject `$1.size`, `$2.len`, and `$2.foo`. Fields are constrained by
source declarations rather than an open-ended `.identifier` postfix.

## Run inside Docker

Use `unsat-rust:local`, `--platform linux/amd64`, the repository mounted at
`/unsat-rust`, and that directory as the working directory:

```sh
cargo run --locked --manifest-path tools/dslgen/Cargo.toml
PYTHONPATH=tools/x/src python -m unittest discover -s tools/x/tests -p 'test_dsl*.py' -v
python tools/x/src/dsltrans.py \
  --api 'core::alloc::GlobalAlloc::alloc(&self, layout: core::alloc::Layout) -> *mut u8' \
  --rule '`layout` must have non-zero size.' \
  --out eval/dsltrans/library-only-single
```

The model returns the raw expression:

```text
$2.size != 0
```

The prompt requests a complete expression directly, with no expressibility classification or refusal marker.

There is no JSON response schema and no model-generated explanation/status
wrapper. The generated parser and type checker validate the raw expression.
`False` is a valid Boolean formula; semantic fidelity must still be reviewed. Local
`result.json` and the SDK's `response.json` retain execution metadata; the model's
unchanged plain-text output is saved in `response.txt`.

Use `--batch tools/x/dsltrans_samples.json` for the ten-case sample. `--dry-run`
saves inputs without a model call. `--resume` requires identical request hashes.
Credentials come from `OPENAI_API_KEY`, `.local/.env`, or `.env` and are not saved.

Artifacts include `api_variables.json`, the selected `schema.json`,
`prompt_grammar.ebnf`, `grammar.ebnf`, `template.ebnf`, `library.ebnf`,
`memory.ebnf`, `memory-schema.json`, and model
request/response JSON. Accepted formulas also save an executable `grammar.lark`.

## Memory domain

`x.$mem` describes x's own storage; `p.$mem.pointee` describes a pointer's
referenced region. The domain has literal fields for size, start, required
alignment, read/write extents, initialization, typed validity and originating
allocation. Details and semantic boundaries are in `tools/x/dslmemory.md`.
Use `--batch tools/x/dslmem_samples.json` for ten real pointer/memory requirements.
Memory observations are symbolic specification inputs; runtime KLEE binding is
outside this implementation. Current-state fields do not express future history.

## Evidence and limits

All replies are checked as DSL expressions. Refusal/status labels fail validation.
Unresolved generic values and external aliases are not silently numeric. Sufficient safe-call cases are
not turned into mandatory preconditions. Parsing never executes expressions;
passing grammar/type checks does not prove semantic equivalence or backend support.

The earlier 962-rule compiler-HIR audit remains in
`eval/dsltrans/full-audit-20261106/review.md`. Its pre-removal tools and catalog
are archived under `tool-snapshot/` there. They document historical evaluation
rather than the active generation/translation pipeline.
