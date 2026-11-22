---
name: ruklee-environment
description: Build or run RuKLEE, execute repository tests and validation, manage the Docker environment and caches, or clean experiment disk usage. Use for changes to x and build/test entrypoints as well.
---

# RuKLEE build and test environment

Start from the repository root and the root `./x` entrypoint. Before **every container run**, build the image from the root `Dockerfile` as `unsat-rust:local`, using Docker's normal cache. The existence of an image is not a reason to skip the build. All compilation, tests, linters and other build validation execute inside that image; never compile or test on the host.

For operations exposed by `./x`, use that wrapper. `./x build` builds the toolchain inside Docker; other subcommands run their tools inside Docker. Preserve incremental KLEE, LLVM, rustc and MIRScan build directories; do not clean or rebuild them from scratch merely to refresh the image.

If `x` still skips image construction when the tag exists, explicitly run the image build before invoking `./x`. For an operation not exposed by `x`, build first and then use `docker run` with the repository mounted at `/unsat-rust` and that working directory. Match the image user/group to the repository owner. On macOS add `--platform linux/amd64` to both build and run.

```bash
platform_args=()
if [[ "$(uname -s)" == Darwin ]]; then platform_args=(--platform linux/amd64); fi
docker build "${platform_args[@]}" --build-arg "USER_ID=$(id -u)" \
  --build-arg "GROUP_ID=$(id -g)" -t unsat-rust:local .
./x <subcommand> <arguments>
```

For a command not exposed by `x`, replace the last line with:

```bash
docker run --rm "${platform_args[@]}" -e PYTHONDONTWRITEBYTECODE=1 \
  -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local <command>
```

## Review changes to the workflow

The maintainer requires review for changes to `x` or the build/test environment. Prepare the concrete diff and relevant checks for review **before adopting the changed workflow**. Keep unaffected work moving with the current entrypoint and the explicit build-first sequence above. Ordinary runs using the established workflow need no new approval; honor any review already given in the session.

Keep Docker cache layers effective when changing the Dockerfile. Image refreshes must not trigger full LLVM, rustc or KLEE recompilation. Confirm the actual compiler version used by an experiment, rather than inferring it from the image's bootstrap toolchain.

## Disk usage after runs

All new evaluation outputs use `eval/<tool>/<crate-name>/`, with optional attempt
or timestamp directories below the crate. For example, whole-crate Claude runs
belong in `eval/claude-crate/talc-5.0.3/<timestamp>/`; compilation checks belong in
`eval/<tool>/<crate-name>/compile-<attempt>/`. Put shared scheduling metadata in
`dev/` or `.local/`, not between the tool and crate directory. Historical outputs
may retain their old layout; never silently relocate evidence referenced by a run.

`./x claude <crate> --mode crate --libraries tools/rust/library` builds the Claude
image with copies of `core`, `std`, and `alloc`, and asks for violations of unsafe
fn requirements across the crate. It loads `CLAUDE_API_KEY` from `.env` on the
host. Preserve requests, source hashes, image IDs, raw output and structured
results; model findings are not independently confirmed bugs.

After each run check `df -h` and `du -sh eval/ .local/ crates_inj/`. Promptly remove obsolete `klee-out-*`, per-run `logs/`, stale `eval/*/batch-*`, and `.local/rustsec-work` copies. Preserve results still needed, especially `matrix.json`, `run.json`, selected evidence and current toolchain caches. Determine ownership and purpose before deleting existing outputs; do not prune Docker images, shared caches or unrelated experiments indiscriminately.
