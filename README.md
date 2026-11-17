# UnSAT-Rust

## Directory

- `benchmark/ruklee/`: RuKLEE benchmark crate snapshots.
- `benchmark/safe4u/`: Safe4U benchmark crate snapshots.
- `benchmark/culpa/`: CULPA compatibility-rerun crate snapshots.
- `benchmark/rustsec/`: RustSec crate snapshots; see [the runbook](dev/rustsec/README.md).
- `tools/`: patched Rust, KLEE, MIRScan, Safe4U and the `x` CLI implementation.
- `docker/`: baseline analyzer images and runners.
- `dev/`: experiment scripts and benchmark metadata.
- `eval/`: experiment results and retained evidence.
- `.local/`: local work copies, caches and intermediate outputs.

Keep `benchmark/<name>/` source-only: its immediate children must be crate
directories containing `Cargo.toml`. Write logs and results under `eval/` or
`.local/`. All commands below start at the repository root and use Bash.

## Safe4U

The scanner entrypoint is [`tools/safe4u/cargo-safe4u`](tools/safe4u/cargo-safe4u).
Configure the model, API endpoint and credentials before running:

```bash
# Only create the config if it does not already exist.
test -f tools/safe4u/env.json || cp tools/safe4u/env-example.json tools/safe4u/env.json
# Edit tools/safe4u/env.json; it is ignored by Git.
```

Build the image and scan one crate. Mount results separately so they persist
outside the benchmark sources:

```bash
crate=fast_id_map-0.1.0
mkdir -p "eval/safe4u/$crate"
docker build -f docker/safe4u/Dockerfile -t safe4u .
docker run --rm \
  -v "$PWD/benchmark/safe4u:/workspace/benchmark" \
  -v "$PWD/eval/safe4u:/workspace/results" \
  -v "$PWD/tools/safe4u/env.json:/opt/safe4u/env.json:ro" \
  safe4u --crate "/workspace/benchmark/$crate" \
  --out-dir "/workspace/results/$crate"
```

To scan every crate sequentially, use this loop in place of the single-crate
build/run block:

```bash
mkdir -p eval/safe4u
for crate_dir in benchmark/safe4u/*/; do
  [[ -f "$crate_dir/Cargo.toml" ]] || continue
  crate="$(basename "$crate_dir")"
  docker build -f docker/safe4u/Dockerfile -t safe4u . || break
  docker run --rm \
    -v "$PWD/benchmark/safe4u:/workspace/benchmark" \
    -v "$PWD/eval/safe4u:/workspace/results" \
    -v "$PWD/tools/safe4u/env.json:/opt/safe4u/env.json:ro" \
    safe4u --crate "/workspace/benchmark/$crate" \
    --out-dir "/workspace/results/$crate" \
    > "eval/safe4u/$crate.log" 2>&1
  scan_status=$?
  printf '%s: exit %s\n' "$crate" "$scan_status"
done
```

Each crate's output directory contains `results.json`, retrieved context and
intermediate artifacts. `--model NAME` overrides the configured model;
`--limit N` limits candidate functions; `--reuse-results` reads cached results
without new model requests. Use separate output directories when comparing
models or configurations. See [Safe4U options](tools/safe4u/README.md) and
[container details](docker/safe4u/README.md).

## CULPA

The single-crate runner is [`docker/rust-comparison/run.py`](docker/rust-comparison/run.py);
the batch runner is [`run-culpa-benchmark.py`](docker/rust-comparison/run-culpa-benchmark.py).
Build the pinned upstream image, then run one crate:

```bash
docker build -f docker/rust-comparison/Dockerfile.culpa \
  -t uspec-rust-comparison-culpa:local docker/rust-comparison
python3 docker/rust-comparison/run.py culpa revm-interpreter-10.0.3 \
  --attempt example-1
```

This reads `benchmark/ruklee/` and writes logs, findings and provenance under
`eval/rust-comparison/<crate>/culpa/<attempt>/`. Choose a new attempt name for
each rerun; existing attempts cannot be overwritten.

To reproduce the compatibility configuration, build both images and run the
batch. The compatibility image patches analyzer compiler queries; report its
results separately from unmodified upstream CULPA.

```bash
docker build -f docker/rust-comparison/Dockerfile.culpa \
  -t uspec-rust-comparison-culpa:local docker/rust-comparison
docker build -f docker/rust-comparison/Dockerfile.culpa-compat \
  -t uspec-rust-comparison-culpa:compat docker/rust-comparison
python3 docker/rust-comparison/run-culpa-benchmark.py \
  --attempt compat-reproduction-1 --workers 4 \
  --override-root benchmark/culpa \
  --image uspec-rust-comparison-culpa:compat --env RUST_BACKTRACE=1
python3 docker/rust-comparison/summarize-culpa.py \
  eval/culpa/compat-reproduction-1/runs \
  --output eval/culpa/compat-reproduction-1/selected
```

The batch roster comes from `benchmark/ruklee/`; `--override-root` selects the
copies in `benchmark/culpa/` where present. Defaults are four concurrent runs,
two CPUs, two Cargo jobs, 12 GiB and 600 seconds per container. The runner copies
sources into `.local/rust-comparison/work/` and automatically handles the
`ptr_hash` dependency mount and `jomini` library-only setting in batch mode.
Raw results go under `eval/culpa/<attempt>/runs/`, with batch metadata and runner
logs alongside. Findings are analyzer alarms, not reviewed TP counts; failed or
partial runs must be reviewed separately. See [the comparison runbook](docker/rust-comparison/README.md)
for other analyzers and configuration details.

## RuKLEE (`./x`)

Initialize submodules once:

```bash
git submodule update --init --recursive
```

Before every `./x` invocation, refresh the root image using Docker's cache.
The wrapper currently builds it automatically only when the image is missing.
On macOS, the image and wrapper use `linux/amd64`.

```bash
platform_args=()
if [[ "$(uname -s)" == Darwin ]]; then platform_args=(--platform linux/amd64); fi
docker build "${platform_args[@]}" --build-arg "USER_ID=$(id -u)" \
  --build-arg "GROUP_ID=$(id -g)" -t unsat-rust:local .
./x build
```

After refreshing the image again, verify all callsites and matching rules in
one crate:

```bash
./x verify benchmark/ruklee/revm-interpreter-10.0.3 \
  --results-json eval/ruklee/revm-interpreter-10.0.3/full-matrix-results.json \
  --logs-dir eval/ruklee/revm-interpreter-10.0.3/logs
```

Use `--callsite ID --rule rule-N` to select a single check. The wrapper supplies
the container Rust/KLEE paths; host `$PWD` toolchain overrides are unnecessary.
`verify` syncs missing metadata automatically. See [`x` command documentation](tools/x/README.md)
for the current `./x rule` documentation-to-DSL pipeline, and the
[environment instructions](.agents/skills/ruklee-environment/SKILL.md) for build
and execution requirements.
