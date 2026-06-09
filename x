#!/usr/bin/env bash
set -euo pipefail

python_bin="./tools/x/.venv/bin/python"
if [[ ! -x "$python_bin" ]]; then
  python_bin="python3"
fi

PYTHONPATH=./tools/x/src "$python_bin" -m cli.cli "$@"
