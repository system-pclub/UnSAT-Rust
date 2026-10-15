#!/usr/bin/env bash

cd "$(dirname "$0")/.."

if [[ -f .env ]]; then
  set -a
  source .env
  set +a
fi

mkdir -p eval/ruklee-full

for crate in benchmark/ruklee/*/; do
  name="$(basename "$crate")"
  out="eval/ruklee-full/$name"
  mkdir -p "$out"

  ./x verify "$crate" \
    --results-json "$out/full-matrix-results.json" \
    --logs-dir "$out/full-matrix-logs"
done
