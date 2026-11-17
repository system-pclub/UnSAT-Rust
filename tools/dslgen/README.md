# dslgen: source-derived library member catalog

Generate one API-independent catalog from the actual core, alloc, and std source
trees. API inputs and rule text are handled by `tools/x/src/dsltrans.py`.

Run inside `unsat-rust:local`, with the repository mounted at `/unsat-rust` and
`--platform linux/amd64`:

```sh
cargo run --locked --manifest-path tools/dslgen/Cargo.toml
```

The default Rust checkout is `tools/rust`. The source trees are:

- `library/core/src`
- `library/alloc/src`
- `library/std/src`

`--rust-src PATH` selects another Rust checkout. `--source PATH` can be repeated
to index specific source trees/files instead. `--out PATH` defaults to
`tools/x/generated/dsl`.

## Derivation

1. Discover source files and derive crate/module namespaces. Skip tests/benchmarks.
2. Use `syn` to read struct, enum, type alias, module and use declarations.
   Function and impl bodies are skipped.
3. Preserve type/const parameters, defaults, source locations and references
   between declarations. Const parameter headers retain their declared types.
4. Each named field induces `.field`; tuple fields induce `[i]`; data variants
   induce guarded `.Variant` payload projections. Unit variants have literal tags.
5. Emit version-2 `schema.json` and the human-readable `selectors.md`.
   Python reads the schema to build the API-specific grammar.

For example, the actual `Layout.size: usize` declaration supplies the selector
`core::alloc::layout::Layout -- .size --> usize`. dslgen preserves that typed
transition in `schema.json`; `dslgrammar.py` turns it into an EBNF production.

## API-specific grammar

The shared catalog is independent of APIs. dsltrans selects types mentioned by
an API and follows field, alias, payload and generic-default dependencies.
Only terms reachable from that API's numbered inputs enter its final grammar:

```text
Starlark-style expression template
+ API-relevant library field/index/variant productions
```

Inputs are numbered as arguments (including self), enclosing/method generics,
then an explicit non-unit return value. Comments explain `$1 is ...`, `$2 is ...`.
Type/lifetime parameters are opaque handles; const parameters carry values of
their declared types. For `IntoIter<T, N>::new_unchecked`, T keeps a numbered type
handle and N keeps a numbered usize value. Compiler syntax nodes are not inputs.

The memory domain supplies a separate schema in `tools/x/src/dslmemory.json`, composed
by Python with these source selectors. Raw pointer type expressions retain their
target type and mutability so storage and pointee views can be distinguished.
See `tools/x/dslmemory.md` for the domain contract.
See `tools/x/dsltrans.md` for the active input contract.

## Boundaries

Conditional duplicate declarations retain their cfg alternatives; no platform is
silently chosen. Unknown external/macro-generated types remain opaque. Macro
expansion, cfg evaluation, complete Rust name resolution, semantic type queries,
and runtime memory/history observations require additional analysis support.

## Validation

```sh
cargo test --locked --manifest-path tools/dslgen/Cargo.toml
cargo clippy --locked --manifest-path tools/dslgen/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path tools/dslgen/Cargo.toml --check
```
