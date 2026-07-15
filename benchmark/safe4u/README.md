# Safe4U benchmark crates

These crate snapshots are the inputs for the Safe4U experiments in
`experiments.md`. Build artifacts from the source snapshots were not copied.

All snapshots load and pass `cargo +1.83.0 check --locked` with their default
features. Compatibility changes are intentionally limited to manifests and
dependency lockfiles:

- `i_tree-0.9.0`: use Rust edition 2021 instead of edition 2024.
- `aegis-0.9.12`: use the API-compatible `getrandom` 0.3 line.
- `revm-interpreter-10.0.3`: lock transitive dependencies to releases whose
  MSRV and manifest edition are supported by Rust/Cargo 1.83.
