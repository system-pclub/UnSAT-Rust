#!/usr/bin/env bash
# Execute inside unsat-rust:local, from /unsat-rust.
set -euo pipefail
cargo run --locked --manifest-path tools/dslgen/Cargo.toml
python -m unittest discover -s tools -p test_dsltrans.py -v
python tools/dsltrans.py --batch tools/dsltrans_samples.json "$@"
