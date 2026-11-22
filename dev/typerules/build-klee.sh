#!/usr/bin/env bash
# Incremental KLEE build in the unsat-rust image. The existing CMake cache was
# configured under the host checkout path, so mount the repository there too.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
docker run --rm --user "$(id -u):$(id -g)" -v "$repo:$repo" -w "$repo" unsat-rust:local \
    cmake --build tools/klee/build -j"$(nproc)"
