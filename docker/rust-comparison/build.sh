#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
docker build -f Dockerfile.base -t uspec-rust-comparison-base:bookworm .
for tool in semgrep lockbud prusti rudra mirai mirchecker culpa; do
    docker build -f "Dockerfile.$tool" -t "uspec-rust-comparison-$tool:local" .
done
