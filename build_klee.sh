#!/usr/bin/env bash
set -euo pipefail

pushd klee
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_SOLVER_Z3=true \
    -DENABLE_UNIT_TESTS=OFF \
    -DENABLE_SYSTEM_TESTS=OFF \
    -DENABLE_TCMALLOC=false 
cmake --build build -j"$(nproc)"
popd