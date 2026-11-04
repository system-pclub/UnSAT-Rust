# Repository instructions

## Build and test environment

- Run all compilation, tests, linters, and other build-related validation inside the project Docker environment built from the root `Dockerfile` as `unsat-rust:local`.
- Prefer the root `./x` wrapper when it exposes the required operation. For commands not exposed by `./x`, use `docker run` with the repository mounted at `/unsat-rust` and the working directory set to `/unsat-rust`.
- On macOS, run and build the container with `--platform linux/amd64`.
- Do not compile or run tests directly on the host.

## Fixing issues

- If a DSL rule is wrong, fix the DSL.
- If KLEE is missing functionality or has a bug, fix it, recompile, and rerun.
- Fixes must be general, not specific to one issue or crate.
- `./x build` (or the underlying Docker build) must have proper caching so LLVM, rustc, and KLEE are not rebuilt from scratch every time.
- The PoC prompt already has all the information it needs. Do not add new categories of information without checking with the maintainer first. Fixing existing bugs is fine (for example, a function used in the call chain that slicing fails to include).
