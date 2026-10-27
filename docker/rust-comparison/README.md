# Rust comparison containers

These containers reproduce seven baseline analyzers, including CULPA, for crates in
`benchmark/ruklee`.  The original USpec run supplemented only
`fast-float-0.2.0` and `falcon_capstone-0.5.2`; this copy also accepts the
additional local crates that are blank in `raudit.xlsx`'s `Comparison` tab.
`revm` in the spreadsheet maps to `revm-interpreter-10.0.3`.

Build all seven images with `bash docker/rust-comparison/build.sh`.
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
- https://github.com/shinmao/culpa

## CULPA

`Dockerfile.culpa` pins upstream commit
`4cf5c9e8e6af5f4f0e4cd97bf7428c570ac9be68` and its required
`nightly-2025-02-20` toolchain. It builds the upstream lockfile without changing
the analyzer or specifications. A small launcher only gives each compiler
invocation a separate JSON output path, avoiding upstream output overwrites.

```bash
docker build -f docker/rust-comparison/Dockerfile.culpa \
  -t uspec-rust-comparison-culpa:local docker/rust-comparison
python3 docker/rust-comparison/run.py culpa revm-interpreter-10.0.3 --attempt example
python3 docker/rust-comparison/run-culpa-benchmark.py --attempt benchmark-1 --workers 4
python3 docker/rust-comparison/summarize-culpa.py \
  eval/culpa/benchmark-1/runs --output eval/culpa/benchmark-1
```

The batch uses the current 30 directories in `benchmark/ruklee`, default package
features, locked dependencies, two CPUs and Cargo jobs per container, 12 GiB per
container, and a 600-second limit. The upstream default check configuration is
retained. `--culpa-enable-violations` separately enables the optional upstream
negative-evidence checks and should be reported as a different configuration.

Every run retains source hashes, image ID and revision, command, original logs,
per-target JSON, the resulting Cargo.lock, and process status. The generated
summary excludes `Satisfied` records and suppressed `Low`-risk findings from
alarm counts. CULPA's own printed total includes satisfied checks. Alarm counts
are not TP counts; external-evidence warnings are reported separately. Compiler
failures after JSON output are marked partial, not successful zero-alarm runs.

Two benchmark-specific integration options preserve source and default features:

```bash
# Existing ../../../patched/common_traits-0.10.0 dependency in ptr_hash.
python3 docker/rust-comparison/run.py culpa ptr_hash-0.1.1 \
  --attempt mounted-dependency --mount-patched
# Avoid upstream enumeration of jomini's feature-gated binaries.
python3 docker/rust-comparison/run.py culpa jomini-0.27.0 \
  --attempt library-only --culpa-library-only
```

The latter invokes `cargo check --lib` with upstream's `cargo-culpa` rustc wrapper
and the same target-package environment. It restricts the analysis to the library;
it does not enable the binary's optional `json` feature. Results from these
follow-up runs can be selected with the summarizer's `--prefer-runs` option while
retaining the original attempt directory. The first completed experiment and its
limitations are recorded in `eval/culpa/README.md`.

### Completed compatibility rerun

`Dockerfile.culpa-compat` applies `patches/culpa-rustc-queries.patch` to the pinned
upstream image. The patch guards layout and constant queries on unresolved
generic types and uses the selected enum variant when inspecting MIR constructor
fields. These are analyzer fixes: the five copies in `benchmark/culpa` retain
their original source and lockfiles, and all 228 YAML specifications are unchanged.
Unknown generic layouts can reduce precision; this image is a patched baseline,
not an unmodified upstream result.

```bash
docker build -f docker/rust-comparison/Dockerfile.culpa-compat \
  -t uspec-rust-comparison-culpa:compat docker/rust-comparison
python3 docker/rust-comparison/run-culpa-benchmark.py \
  --attempt compat-reproduction --workers 4 --override-root benchmark/culpa \
  --image uspec-rust-comparison-culpa:compat --env RUST_BACKTRACE=1
python3 docker/rust-comparison/summarize-culpa.py \
  eval/culpa/compat-reproduction/runs --output eval/culpa/compat-reproduction/selected
```

The batch automatically preserves the existing `ptr_hash` dependency mount and
`jomini` library-only configuration. All 30 crates completed in the retained
`compat-five` and `compat-rest` batches using the same image. Their merged raw
results, image/source hashes, and per-alarm manual decisions are in
`eval/culpa/compatibility-fix`. To validate and repackage those existing decisions:

```bash
python3 -B eval/culpa/compatibility-fix/build_review.py
```

The reviewed result is 37 TP, 41 FP, 20 invalid-binding reports and one unresolved
report. Only TP and FP enter dissertation Table 4.6. These count requirements;
the 37 TP involve 23 functions, with 15 later sinks of an already-unsafe path.
