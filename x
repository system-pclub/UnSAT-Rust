#!/usr/bin/env bash
set -euo pipefail

readonly IMAGE="unsat-rust:local"
readonly CONTAINER_ROOT="/unsat-rust"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

die() {
    echo "x: $*" >&2
    exit 1
}

command -v docker >/dev/null 2>&1 || die "docker is required"
docker info >/dev/null 2>&1 || die "docker is not running"

platform_args=()
if [[ "$(uname -s)" == "Darwin" ]]; then
    # The patched Rust/KLEE toolchain and its build artifacts are x86_64-only.
    platform_args=(--platform linux/amd64)
fi

if ! docker image inspect "$IMAGE" >/dev/null 2>&1; then
    echo "x: building Docker image $IMAGE" >&2
    docker build \
        "${platform_args[@]}" \
        --build-arg "USER_ID=$(id -u)" \
        --build-arg "GROUP_ID=$(id -g)" \
        --tag "$IMAGE" \
        "$repo_root"
fi

run_args=(
    --rm
    "${platform_args[@]}"
    --volume "$repo_root:$CONTAINER_ROOT"
    --workdir "$CONTAINER_ROOT"
    --env HOME=/home/user
    # Keep downloaded crates across containers so analysed crates' dependencies
    # and the mirscan build deps are fetched only once.
    --volume "$repo_root/.local/cargo-home/registry:/home/user/.cargo/registry"
    --volume "$repo_root/.local/cargo-home/git:/home/user/.cargo/git"
)
mkdir -p "$repo_root/.local/cargo-home/registry" "$repo_root/.local/cargo-home/git"
if [[ -t 0 && -t 1 ]]; then
    run_args+=(--interactive --tty)
fi
if [[ -n "${OPENAI_API_KEY:-}" ]]; then
    run_args+=(--env OPENAI_API_KEY)
fi

if [[ "${1:-}" == "build" ]]; then
    shift
    (($# == 0)) || die "usage: ./x build"
    docker run "${run_args[@]}" "$IMAGE" bash -c '
        set -euo pipefail

        echo "==> building KLEE"
        cmake -S tools/klee -B tools/klee/build \
            -DCMAKE_BUILD_TYPE=Release \
            -DENABLE_SOLVER_Z3=true \
            -DENABLE_UNIT_TESTS=OFF \
            -DENABLE_SYSTEM_TESTS=OFF \
            -DENABLE_TCMALLOC=false
        cmake --build tools/klee/build -j"$(nproc)"

        echo "==> building patched Rust"
        (cd tools/rust && ./x build --stage 1 --config /unsat-rust/bootstrap.toml)

        echo "==> building mirscan"
        repo_root=/unsat-rust
        stage1="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage1"
        stage0="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0"
        stage0_sysroot="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot"
        rustc_private_deps="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps"

        for required in "$stage1/bin/rustc" "$stage0/bin/rustc"; do
            [[ -x "$required" ]] || { echo "missing Rust build artifact: $required" >&2; exit 1; }
        done
        [[ -d "$stage0_sysroot" ]] || { echo "missing Rust sysroot: $stage0_sysroot" >&2; exit 1; }
        [[ -d "$rustc_private_deps" ]] || { echo "missing rustc private deps: $rustc_private_deps" >&2; exit 1; }

        find_private_crate() {
            local crate="$1" artifact
            artifact="$(find "$rustc_private_deps" -maxdepth 1 -type f \
                \( -name "lib${crate}-*.rlib" -o -name "lib${crate}-*.so" \) \
                | sort | head -n 1)"
            [[ -n "$artifact" ]] || { echo "missing rustc private crate: $crate" >&2; exit 1; }
            printf "%s" "$artifact"
        }

        rustflags="--sysroot $stage0_sysroot -L dependency=$rustc_private_deps -C link-arg=-Wl,-rpath,$stage1/lib"
        for crate in tracing rustc_abi rustc_data_structures rustc_driver rustc_hir \
            rustc_hir_analysis rustc_interface rustc_log rustc_metadata rustc_middle \
            rustc_session rustc_span rustc_hir_pretty; do
            rustflags="$rustflags --extern $crate=$(find_private_crate "$crate")"
        done

        export RUSTC="$stage0/bin/rustc"
        export RUSTC_BOOTSTRAP=1
        export MIRSCAN_SYSROOT="$stage1"
        export LD_LIBRARY_PATH="$stage1/lib:$rustc_private_deps:${LD_LIBRARY_PATH:-}"
        export RUSTFLAGS="$rustflags ${RUSTFLAGS:-}"
        cargo build --manifest-path tools/mirscan/Cargo.toml --release
    '
    exit
fi

# All tools/x commands execute in the same image and see the repository at the
# stable path expected by generated artifacts and verification logs.
tool_args=(
    --env PYTHONPATH="$CONTAINER_ROOT/tools/x/src"
    --env PATH="$CONTAINER_ROOT/tools/klee/build/bin:/opt/x/.venv/bin:/home/user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
)
if [[ "${1:-}" == "verify" ]]; then
    tool_args+=(
        --env MIRSCAN_RUSTC="$CONTAINER_ROOT/tools/mirscan/target/release/raudit"
        --env RUSTC="$CONTAINER_ROOT/tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
        --env LD_LIBRARY_PATH="$CONTAINER_ROOT/tools/rust/build/x86_64-unknown-linux-gnu/stage1/lib"
        --env LLVM_SYMBOLIZER_PATH=/usr/bin/llvm-symbolizer-20
        --env MIRSCAN_SYSROOT="$CONTAINER_ROOT/tools/rust/build/x86_64-unknown-linux-gnu/stage1"
    )
fi
# Preserve explicit extractor options across the Docker boundary.
for option in MIRSCAN_FOLLOW_SAFE_HELPERS MIRSCAN_MAX_CALL_DEPTH; do
    if [[ -n "${!option:-}" ]]; then
        tool_args+=(--env "$option")
    fi
done

docker run "${run_args[@]}" "${tool_args[@]}" "$IMAGE" python -m cli.cli "$@"
