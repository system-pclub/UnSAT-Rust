"""Pin the Rust toolchain for cargo runs on analysed crates."""
from __future__ import annotations

import functools
import os
import subprocess
import tempfile


@functools.lru_cache(maxsize=1)
def _default_toolchain() -> str | None:
    # Ask from a directory without a rust-toolchain file: the answer is the
    # repository's default toolchain, not whatever a crate pins.
    try:
        with tempfile.TemporaryDirectory() as neutral:
            result = subprocess.run(
                ["rustup", "show", "active-toolchain"],
                cwd=neutral, capture_output=True, text=True, check=False,
            )
    except OSError:
        return None
    words = result.stdout.split()
    return words[0] if result.returncode == 0 and words else None


def pin_default_toolchain(env: dict[str, str]) -> dict[str, str]:
    """Make cargo ignore a crate's rust-toolchain file, which can select
    a stable cargo (no `-Z` flags, different target layout) or download a toolchain.

    An explicit RUSTUP_TOOLCHAIN in *env* is kept.
    """
    if "RUSTUP_TOOLCHAIN" not in env:
        toolchain = _default_toolchain()
        if toolchain:
            env["RUSTUP_TOOLCHAIN"] = toolchain
    return env
