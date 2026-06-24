#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
stage1="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage1"
stage0="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0"
stage0_sysroot="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot"
stage0_rustc="$stage0/bin/rustc"
stage1_rustc="$stage1/bin/rustc"
stage1_lib="$stage1/lib"
rustc_private_deps="$repo_root/tools/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps"

if [[ ! -x "$stage1_rustc" ]]; then
    echo "missing patched rustc: $stage1_rustc" >&2
    echo "run ./build_rust.sh first" >&2
    exit 1
fi

if [[ ! -x "$stage0_rustc" ]]; then
    echo "missing bootstrap rustc: $stage0_rustc" >&2
    echo "run ./build_rust.sh first" >&2
    exit 1
fi

if [[ ! -d "$stage0_sysroot" ]]; then
    echo "missing bootstrap sysroot: $stage0_sysroot" >&2
    echo "run ./build_rust.sh first" >&2
    exit 1
fi

if [[ ! -d "$rustc_private_deps" ]]; then
    echo "missing rustc private deps: $rustc_private_deps" >&2
    echo "run ./build_rust.sh first" >&2
    exit 1
fi

find_private_crate() {
    local crate="$1"
    local artifact
    artifact="$(
        find "$rustc_private_deps" -maxdepth 1 -type f \
            \( -name "lib${crate}-*.rlib" -o -name "lib${crate}-*.so" \) \
            | sort \
            | head -n 1
    )"
    if [[ -z "$artifact" ]]; then
        echo "missing rustc private crate artifact for $crate in $rustc_private_deps" >&2
        exit 1
    fi
    printf '%s' "$artifact"
}

rustflags="--sysroot $stage0_sysroot -L dependency=$rustc_private_deps -C link-arg=-Wl,-rpath,$stage1_lib"
for crate in \
    tracing \
    rustc_abi \
    rustc_data_structures \
    rustc_driver \
    rustc_hir \
    rustc_hir_analysis \
    rustc_interface \
    rustc_log \
    rustc_metadata \
    rustc_middle \
    rustc_session \
    rustc_span \
    rustc_hir_pretty
do
    rustflags="$rustflags --extern $crate=$(find_private_crate "$crate")"
done

# The rustc_private artifacts that form the patched stage1 compiler are built by
# the stage0 compiler, so raudit must be built with the matching stage0 ABI.
export RUSTC="$stage0_rustc"
export RUSTC_BOOTSTRAP=1
export MIRSCAN_SYSROOT="$stage1"
export LD_LIBRARY_PATH="$stage1_lib:$rustc_private_deps:${LD_LIBRARY_PATH:-}"
export RUSTFLAGS="$rustflags ${RUSTFLAGS:-}"

pushd "$repo_root/tools/mirscan"
cargo build --release "$@"
popd
