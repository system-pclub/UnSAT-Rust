# Safe4U benchmark crates

The 32 requested crate snapshots are in `benchmark/safe4u`; exact published
originals are in `benchmark/original`. The extra pre-existing fast_id_map snapshot
is retained separately from the requested roster.

All 32 requested snapshots passed default-feature `cargo check --lib` with Rust
1.86.0, the compiler in the current Safe4U Dockerfile. Per-crate commands, compiler
versions, source hashes, resulting dependency locks and logs are under
`eval/safe4u/<crate>/compile-*/`. The final selected results are indexed in
`dev/benchmark/compilation-matrix.json`.

Pre-existing compatibility copies and locks are retained, including the older
`i_tree` edition adjustment, aegis/getrandom pin, and compatible revm dependency
lock. New compatibility work replaces azul-css's then-unstable
`len.is_multiple_of(2)` with the equivalent modulo expression. falcon_capstone's
system headers are supplied by `docker/safe4u/Dockerfile.benchmark`, which adds
`libcapstone-dev` to the original Safe4U image.

See `dev/benchmark/compatibility-notes.md` for the complete TASK changes and
`dev/benchmark/execution-environment.json` for the temporary Docker storage
recovery used to finish the runs. These are compilation checks, not Safe4U alarm
or vulnerability results.
