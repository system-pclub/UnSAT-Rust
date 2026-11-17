#!/usr/bin/env bash
# Execute inside unsat-rust:local, from /unsat-rust.
set -euo pipefail
cargo run --locked --manifest-path tools/dslgen/Cargo.toml -- --out tools/x/generated/dsl
PYTHONPATH=tools/x/src python -m unittest discover -s tools/x/tests -p 'test_dsl*.py' -v
python tools/x/src/dsltrans.py --batch tools/x/dsltrans_samples.json "$@"
