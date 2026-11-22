# TASK benchmark preparation

Current completion limits and the host blocker are recorded in [STATUS.md](STATUS.md).
Resume pending work with `bash dev/benchmark/resume-task.sh` after storage recovers.

The roster in `sources.json` contains the requested 29 existing crates and three
additions: talc 5.0.3, redis-module 2.0.8 and azul-css 0.0.8. The existing revm row
maps to revm-interpreter 10.0.3, matching prior experiments. We preserve this
version rather than substituting the current revm release.

- `benchmark/original`: exact crates.io archives, with archive SHA-256 and local
  archive path recorded in `sources.json`.
- `benchmark/culpa`, `benchmark/ruklee`, `benchmark/safe4u`: tool-specific sources.
  Existing local changes are preserved. Newly required copies start from originals.
- `crates.tsv` and `crates.md`: requested three-crate statistics and counting policy.
- `metadata`: saved GitHub and crates.io API responses used by that table.
- `table-evidence.json`: exact reviewed faulty locations and MIRScan callsites.

23 published versions were available as local archives. Nine older exact versions
were restored from static.crates.io because local source copies had modifications
and corresponding local archives were absent. The downloaded versions are recorded
in `sources.json`; no newer crate version is substituted.

`benchmark/rustsec` remains in place: `dev/rustsec/allowlist.json`, selected
callsites, replay tooling and historical Claude experiments still refer to its
37 snapshots. Its removal was conditional in TASK and would break those inputs.
The extra pre-existing fast_id_map snapshots are also retained.

After building the root image as required by the environment skill, validate the
layout and runner contracts inside Docker:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" \
  -w /unsat-rust unsat-rust:local python3 dev/benchmark/check_layout.py
# Build the root image again before the next container run.
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" \
  -w /unsat-rust unsat-rust:local python3 -m unittest dev.benchmark.test_claude
```

`check_compilation.py <tool>` runs **inside** the selected tool's Docker image,
compiles isolated copies using default features and `cargo check --lib`, and saves
commands, actual compiler versions, source hashes, lockfiles and logs below
`eval/<tool>/<crate>/compile-<attempt>/`. It does not run the analyzers or classify
warnings as bugs. RuKLEE uses the patched stage1 compiler; CULPA uses its pinned
nightly; Safe4U uses Rust 1.86.0 from its current Dockerfile.

Claude whole-crate experiments run with:

```bash
./x claude benchmark/original/talc-5.0.3 --mode crate \
  --libraries tools/rust/library --timeout 1800
```

Use the same command for redis-module and azul-css. Results are written to fresh
`eval/claude-crate/<crate-version>/<timestamp>/` directories. The prompt supplies
only the crate and the Rust library requirements, with no reference bugs, chosen
functions or selected rules. Findings need independent review.
