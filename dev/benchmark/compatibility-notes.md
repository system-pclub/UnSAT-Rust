# Compatibility changes added for TASK

Original published snapshots in `benchmark/original` remain byte-for-byte archive
contents. Pre-existing tool-copy changes are retained and listed in `sources.json`.

- Safe4U / azul-css 0.0.8: replace `!len.is_multiple_of(2)` with `len % 2 != 0`;
  Rust 1.86 does not yet provide the stabilized method. This preserves the branch.
- CULPA / azul-css 0.0.8: the same replacement for nightly-2025-02-20.
- Safe4U / falcon_capstone 0.5.2: `docker/safe4u/Dockerfile.benchmark` adds the
  missing system `libcapstone-dev` dependency to the existing Safe4U image.
  The crate source is unchanged by this task.
- CULPA / falcon_capstone 0.5.2, grubbnet 0.1.7, lurk-cli 0.3.8: retain compatible
  dependency locks already validated in Safe4U. Old proc-macro2 versions used the
  removed nightly `proc_macro_span_shrink` feature; the updated lock avoids it.
- CULPA / ptr_hash 0.1.1: reuse the existing `patched/common_traits-0.10.0` path
  dependency, as the RuKLEE and Safe4U copies already do. This avoids ambiguity
  between its `NonZero` trait and `core::num::NonZero`. Dependency source hashes
  are retained in `path-dependency-hashes.json`.
- CULPA / revm-interpreter 10.0.3: use the existing compatible dependency lock;
  fresh resolution selected ruint 1.21.1, which requires Rust 1.90.

Successful checks retain their generated Cargo.lock in both their attempt output
and the tool-specific snapshot. Compiler versions, commands and input source hashes
are recorded per attempt. A compile check uses default features and the library
target, and does not run the analyzer or establish a vulnerability finding.
