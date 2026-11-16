# Modeled memory projections

The active grammar is composed of the expression template, API-relevant Rust
selectors, and the modeled memory selectors declared in `dslmemory.json`.
`dslmemory.py` lowers this domain into the same typed transition graph as source
fields. The grammar and checker consume that graph; there is no arbitrary
`.identifier` access rule.

## Storage and pointees

For a Rust value/place `x`, `x.$mem` describes its own storage. Source fields and
array/tuple elements retain this projection:

```text
$1.len.$mem.size
$2[0].$mem.initialized
```

For a raw pointer `p: *const T` / `*mut T`, distinguish:

```text
p.$mem.size                 # pointer-slot representation size
p.$mem.start                # address of the pointer slot
p.$mem.pointee.size         # sizeof(T), or the full dynamic size for DST T
p.$mem.pointee.start        # data address carried by p, including zero
p.$mem.pointee.align        # alignment required by T
```

These examples illustrate projections; comments are not part of the returned
model expression. `p.$mem.pointee` is a logical observation, not an executed
Rust dereference. It can describe null, dangling, or misaligned pointers.
`NonNull<T>` and `Unique<T>` also expose this pointer view through explicitly
listed semantic wrapper identities; other unknown wrappers do not acquire it by
name guessing. References are transparent in the existing Rust value view.

Type/lifetime parameter handles, modeled metadata scalars, literals, and computed
arithmetic expressions do not expose storage. A source const-generic value keeps
a value sort; its storage observation must report absence/unknown if no physical
backing place is represented. The schema does not invent an address for an erased
constant or unmaterialized value.

## Field meanings

Storage and region views expose:

| Field | Meaning |
| --- | --- |
| `exists` | Backing storage is represented |
| `start`, `size`, `end` | Beginning byte address, associated value size, and mathematical start + size |
| `align` | Required type alignment; the address may fail to satisfy it |
| `readable`, `writable` | The entire view's byte range has the corresponding current-state access validity |
| `readable_size`, `writable_size` | Maximum contiguous accessible byte extent from start within one live provenance-compatible allocation |
| `initialized` | Every byte is initialized, including padding |
| `valid` | A language-valid initialized value of the associated type; uninitialized padding is allowed |
| `allocation` | The originating allocation-generation record |

Access validity does not imply alignment or initialized contents. Typed validity
does not imply library safety invariants, alignment, access permission or
allocation liveness. Zero-byte read/write access is vacuous; alignment remains
an independent obligation even for zero-sized values.

Allocation records expose `exists`, `id`, `start`, `size`, `end`, `align`,
`minimum_size`, `live`, and `allocator`. The ID includes allocation generation;
reusing an address does not imply the same allocation. Records survive release,
with `live = false`. `minimum_size` retains the producing request/contract and is
not inferred from capacity. ID/bounds reasoning must establish record existence.
Allocator contract identity is distinct from the address of an allocator object.

## Examples

For a pointer `$1`, proper target alignment is:

```text
$1.$mem.pointee.start % $1.$mem.pointee.align == 0
```

A properly initialized value of its target type is:

```text
$1.$mem.pointee.valid
```

For `copy_nonoverlapping<T>(src, dst, count)`, `$1` is src, `$2` is dst, `$3` is
count, and `$4` is a type handle. The copied extent is count times one target's
size. For example:

```text
$3 * $2.$mem.pointee.size <= $2.$mem.pointee.writable_size
```

Using `pointee.end` alone would cover one T, not the operation's counted range.
Using allocation size would instead concern the containing allocation.

## Graph construction

Let `Storage<T>`, `Region<T>`, `Allocation`, and metadata scalar sorts be modeled
nodes. Add the following typed edges:

```text
T                         -- .$mem --> Storage<T>
Storage<Pointer<T>>        -- .pointee --> Region<T>
Storage<T> / Region<T>     -- .size --> MemoryInteger
Storage<T> / Region<T>     -- .valid --> MemoryBool
Storage<T> / Region<T>     -- .allocation --> Allocation
Allocation                -- .id --> MemoryString
```

All other declared fields are lowered in the same way. Metadata scalars use
separate sorts from Rust place-valued scalars, preventing repeated introspection
such as `p.$mem.size.$mem`.

The existing graph-to-production rule applies unchanged: an edge from receiver
sort A to result sort B labeled s contributes `N_B ::= N_A tokens(s)`. The
reserved `.$mem` projection is rewritten to a private AST attribute only during
static checking. User-written private attribute spellings are rejected; string
contents remain untouched.

## Running the pointer trial

Inside `unsat-rust:local` with `--platform linux/amd64`:

```sh
cargo run --locked --manifest-path tools/dslgen/Cargo.toml
python -m unittest discover -s tools -p 'test_dsl*.py' -v
python tools/dsltrans.py --batch tools/dslmem_samples.json \
  --out eval/dsltrans/memory-domain-20261107 --workers 2
```

Each case saves the exact `memory-schema.json`, `memory.ebnf`, source/template
components, full prompt grammar, executable validation grammar, model request,
plain-text response and static validation result.

These are specification observations. This change implements domain grammar,
typing, prompt semantics and translation experiments; it does not bind these
fields to KLEE execution state. Current-state views cannot express future
non-mutation, lifetime intervals, races, post-call states or arbitrary library
behavior. Such requirements should still return `UNEXPRESSIBLE`.
