# Rust comparison containers

These containers reproduce the six baseline analyzers for crates in
`benchmark/ruklee`.  The original USpec run supplemented only
`fast-float-0.2.0` and `falcon_capstone-0.5.2`; this copy also accepts the
additional local crates that are blank in `raudit.xlsx`'s `Comparison` tab.
`revm` in the spreadsheet maps to `revm-interpreter-10.0.3`.

Build all six images with `bash docker/rust-comparison/build.sh`.
Dockerfiles pin upstream images by digest or source commits and Rust nightlies.
Prusti uses its pinned upstream binary release. LockBud's old published Docker
image is also retained as `Dockerfile.lockbud-legacy`; the main Dockerfile builds
the current upstream source to support the benchmark's newer dependencies.

The runner discovers crates from `benchmark/ruklee` and copies sources without
`target` into `.local/rust-comparison/work`.
It never edits the benchmark source. Each unique `--attempt` preserves the exact
command, source hashes, image ID, return status, and stdout/stderr under
`eval/rust-comparison/<crate>/<tool>/<attempt>/`. Existing attempts cannot be
overwritten. Nonzero exit codes, timeouts, unsupported features, and missing
entry points must be reviewed separately from analyzer diagnostics; none imply
a successful zero-alarm result.

```bash
python3 docker/rust-comparison/run.py semgrep fast-float-0.2.0 --attempt rerun
python3 docker/rust-comparison/run.py lockbud falcon_capstone-0.5.2 --attempt rerun
python3 docker/rust-comparison/run.py prusti fast-float-0.2.0 --attempt rerun --lock-version 3
python3 docker/rust-comparison/run.py rudra fast-float-0.2.0 --attempt rerun --direct
python3 docker/rust-comparison/run.py mirai falcon_capstone-0.5.2 --attempt rerun
python3 docker/rust-comparison/run.py mirchecker fast-float-0.2.0 --attempt rerun --direct --checker-entry parse --memory 32g
```

`--lock-version 3` changes only the lockfile's format marker in the work copy;
it does not change dependency versions. The dependency-free fast-float library
can also be passed directly to Rudra with its default `std` feature, avoiding
Cargo's unnecessary resolution of newer development dependencies. This is a
library analysis, not a test/benchmark run.

Semgrep uses the Rust and generic rules selected from the upstream `p/default`
configuration fetched on 2026-09-17, checked into `rules/default-rust.yaml`.
`rules/rust.yaml` preserves the broader `p/rust` audit pack. It can be selected
with `--semgrep-config rust.yaml`; its informational request to audit every
`unsafe` block must not be presented as a claim of memory unsafety. Both rule
packs were run during setup and their raw results are retained. The original
18 rows have no associated tool versions/rules/commands in this repository;
exact historical configuration parity cannot be established.

Upstream installation references:

- https://github.com/semgrep/semgrep
- https://github.com/BurtonQin/lockbud
- https://github.com/viperproject/prusti-dev/releases/tag/v-2024-03-26-1504
- https://github.com/sslab-gatech/Rudra
- https://github.com/endorlabs/MIRAI/blob/main/documentation/InstallationGuide.md
- https://github.com/lizhuohua/rust-mir-checker
