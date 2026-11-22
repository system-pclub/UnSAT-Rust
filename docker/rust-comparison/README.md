# Rust comparison containers

These containers reproduce seven baseline analyzers, including CULPA. Pristine
packages live in `benchmark/original`; compiler-compatible copies live in
`benchmark/culpa`, `benchmark/ruklee`, and `benchmark/safe4u`.  The original USpec run supplemented only
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
`eval/<tool>/<crate>/<attempt>/`. Existing attempts cannot be
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
  eval/culpa --attempt benchmark-1 --output .local/culpa-batches/benchmark-1/summary
```

The batch uses the 32 requested directories in `benchmark/culpa`, default package
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
  eval/culpa --attempt compat-reproduction --output .local/culpa-batches/compat-reproduction/selected
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

### Claude CLI

`Dockerfile.claude` installs Claude Code without the Rust toolchain. Set
`CLAUDE_API_KEY` in the repository `.env` (or environment), then run:

```bash
./x claude benchmark/rustsec/ruint-1.17.0 \
  --rule 'The index must be within the slice bounds.' \
  --target-function 'algorithms::div::reciprocal::reciprocal_mg10' \
  --unsafe-api 'core::slice::get_unchecked'
python3 dev/run-benchmark-rustsec-by-claude.py
```

The wrapper builds the Claude image with Docker cache before running, mounts only
that crate at `/work:ro`, and forwards the key as `ANTHROPIC_API_KEY` through the
container environment. `--api-token`, `--model` (default `sonnet`) and `--timeout`
(default 600 seconds) are also supported. Successful stdout is exactly
`{"violation": true}` or `{"violation": false}`. CLI failures and malformed
responses return nonzero; timeouts remove the container.

The benchmark uses the 38 allowlist records with one manually reviewed rule/API
selection per advisory in `dev/rustsec/claude-cases.json`. The `rule` string is
exactly that row's original `rule` field from `rules.csv`, without an ID prefix,
paraphrase, combined rules, or fallback. Selection reasons and source locations
stay in the result metadata and are not sent to Claude. Cases with no applicable
CSV rule are recorded as `no_matching_rule` without calling Claude. An original
reported defect already fixed in the pinned snapshot is recorded separately as
`reported_defect_not_in_snapshot`. Each matched input pins the original faulty
API's file, line, source expression and source hash; the API string sent to Claude
includes the exact location/expression. The CSV API name/path are also checked
against the reviewed selection. A rule from a different API or another defect in
the same function is not an eligible substitute. Run with
`--dry-run` to inspect all inputs without API calls.

It sends no advisory descriptions or known witnesses. Both toodee advisories run
separately. Results and requests are saved to a fresh
`eval/rustsec-claude/<timestamp>/` with true/false/no_matching_rule/error/timeout
counts in `summary.json`. The model and timeout are constants in the script.
Runs before this input policy was corrected used a generic fallback and must not
be used as results for the verbatim-CSV experiment.
These counts are Claude judgments, not confirmed vulnerability detections.

Claude print mode and structured output follow the
[official CLI documentation](https://code.claude.com/docs/en/headless).

### Functions and structs only

`dev/rustsec/prepare-claude-snippets.py` uses Tree-sitter to extract the selected
function bodies and required struct declarations from the pinned sources, keeping
impl/trait headers and cross-function defect context. Install `tree-sitter==0.25.2`
and `tree-sitter-rust==0.24.0` in a Python environment inside the repository Docker
image, then run the preparer there. Generated files and a source-span/hash manifest
are written under `eval/rustsec-claude-snippets/inputs/<timestamp>/`.
There are 38 `.rs` files, one per advisory (the two toodee advisories are separate).
These are analysis excerpts, not compilable standalone crates.

Run the comparison on the host using the printed manifest path:

```bash
python3 dev/run-benchmark-rustsec-by-claude.py \
  --snippets-manifest eval/rustsec-claude-snippets/inputs/<timestamp>/manifest.json
```

Only the selected file is mounted as `/work/input.rs`; the original crate, Cargo
manifest and experiment metadata are not mounted. The rule and verdict prompt
remain the same, with the source instruction pointing to the file. Rule exclusions
remain explicit. Results go under `eval/rustsec-claude-snippets/runs/`.
The standalone `./x claude` entry also accepts an `.rs` file as its first argument.

### Whole-crate Claude mode

```bash
./x claude benchmark/original/talc-5.0.3 --mode crate \
  --libraries tools/rust/library --timeout 1800
```

The image contains copies of `library/core`, `library/std`, and `library/alloc`
under `/libraries`. `--libraries` selects the local Rust library directory at
build time. The prompt asks to list the crate's violations of unsafe fn
requirements. It supplies no rule, target function, advisory or known bug list.
The crate is mounted read-only at `/work`; Claude can use Read, Glob and Grep.
The existing callsite mode and its boolean `violation` output remain available.

Results use `eval/claude-crate/<crate-version>/<timestamp>/` and contain the exact
prompt, source hashes, image ID, raw stdout/stderr and structured `violations`.
Each finding contains file, function, unsafe API, requirement and explanation.
`--output` selects a fresh explicit attempt directory; existing attempts cannot
be overwritten. Errors and timeouts are recorded separately from empty findings.
Model findings require independent review before being counted as bugs.

New comparison runs use `eval/<tool>/<crate>/<attempt>/`. CULPA batch scheduling
metadata now lives in `.local/culpa-batches/<attempt>/`; the summarizer can read
`eval/culpa` recursively. Historical experiment paths above remain unchanged.
