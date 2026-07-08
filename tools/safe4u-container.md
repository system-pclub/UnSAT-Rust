# Safe4U container

Initialize the submodule and build the image from the repository root:

```bash
git submodule update --init tools/safe4u
docker build -f tools/safe4u.Dockerfile -t safe4u .
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
