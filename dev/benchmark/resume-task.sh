#!/usr/bin/env bash
# Complete pending TASK validation after host storage / Docker writeback recovers.
set -euo pipefail
cd "$(dirname "$0")/../.."
attempt="compile-$(date -u +%Y%m%dT%H%M%SZ)"
uid="$(id -u)"
gid="$(id -g)"
root_image() {
    docker build --build-arg "USER_ID=$uid" --build-arg "GROUP_ID=$gid" -t unsat-rust:local .
}
run_check() {
    local tool="$1" image="$2"
    shift 2
    root_image
    docker run --rm --user "$uid:$gid" --entrypoint python3 \
      -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" -w /unsat-rust \
      "$image" dev/benchmark/check_compilation.py "$tool" --attempt "$attempt" "$@"
}
root_image
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" -w /unsat-rust \
  unsat-rust:local bash -c 'python3 dev/benchmark/check_layout.py && python3 -m unittest dev.benchmark.test_claude'
run_check ruklee unsat-rust:local --reuse-work compile-1 --crates revm-interpreter-10.0.3 roqoqo-quest-0.14.4
docker build -f docker/safe4u/Dockerfile.benchmark -t safe4u:benchmark .
run_check safe4u safe4u:benchmark --crates azul-css-0.0.8 falcon_capstone-0.5.2
docker build -f docker/rust-comparison/Dockerfile.base -t uspec-rust-comparison-base:bookworm docker/rust-comparison
docker build -f docker/rust-comparison/Dockerfile.culpa -t uspec-rust-comparison-culpa:local docker/rust-comparison
run_check culpa uspec-rust-comparison-culpa:local
python3 dev/benchmark/finalize.py --freeze-locks
for crate in talc-5.0.3 redis-module-2.0.8 azul-css-0.0.8; do
    root_image
    ./x claude "benchmark/original/$crate" --mode crate --libraries tools/rust/library --timeout 1800
done
python3 dev/benchmark/finalize.py
# Never present unresolved checks or model calls as completed.
python3 - <<'PY'
import json
from pathlib import Path
rows=json.loads(Path('dev/benchmark/compilation-matrix.json').read_text())
failed=[r for r in rows if r['status']!='passed']
if failed:
    raise SystemExit('Remaining compilation checks: '+json.dumps(failed,indent=2))
for name in ('talc-5.0.3','redis-module-2.0.8','azul-css-0.0.8'):
    results=list((Path('eval/claude-crate')/name).glob('*/result.json'))
    if not results:
        raise SystemExit('Missing Claude result: '+name)
print('All 96 compilation checks and three whole-crate Claude calls completed.')
PY
df -h .
du -sh eval/ .local/ crates_inj/
