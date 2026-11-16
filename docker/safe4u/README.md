# Safe4U container

Safe4U source is tracked directly in `tools/safe4u`, including local changes.
Build the image from the repository root:

```bash
docker build -f docker/safe4u/Dockerfile -t safe4u .
```

Safe4U reads its model configuration from `/opt/safe4u/env.json`. Mount that
file and the Rust project, then pass the container path to `--crate`:

```bash
docker run --rm -it \
  -v "$PWD/path/to/crate:/workspace/crate" \
  -v "$PWD/tools/safe4u/env.json:/opt/safe4u/env.json:ro" \
  safe4u --crate /workspace/crate
```

To retain scan results on the host, set `--out-dir` to a path below the mounted
crate, such as `/workspace/crate/safe4u-results`.

The configuration format is documented in
`tools/safe4u/env-example.json`.

The image uses Rust 1.86.0, matching the context retriever toolchain, and builds
the retriever with its committed Cargo.lock in the dev profile used by the CLI.
Build-context exclusions keep local virtual environments, build outputs, scan
results, and env.json out of the image. The working directory is `/opt/safe4u`
so the bundled parser libraries resolve correctly; use absolute container paths
for `--crate` and `--out-dir`.

Source provenance is recorded in `tools/safe4u/UPSTREAM.md`.
