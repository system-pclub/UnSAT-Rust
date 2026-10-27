#!/usr/bin/env python3
"""Run CULPA on the frozen 30-crate roster and preserve each independent attempt."""
import argparse
import concurrent.futures
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--attempt", required=True)
    parser.add_argument("--workers", type=int, default=4)
    parser.add_argument("--timeout", type=int, default=600)
    parser.add_argument("--crates", nargs="*")
    parser.add_argument("--benchmark-root", type=Path, default=ROOT / "benchmark/ruklee")
    parser.add_argument("--override-root", type=Path, help="Use copies in this directory when present")
    parser.add_argument("--image", help="Explicit CULPA image, recorded by each runner")
    parser.add_argument("--env", action="append", default=[], metavar="KEY=VALUE")
    args = parser.parse_args()
    crates = args.crates or sorted(p.name for p in args.benchmark_root.iterdir()
                                  if (p / "Cargo.toml").is_file())
    output = ROOT / "eval/culpa" / args.attempt
    output.mkdir(parents=True, exist_ok=True)
    manifest = output / "batch.json"
    if manifest.exists():
        parser.error(f"Batch already exists: {manifest}")
    manifest.write_text(json.dumps(vars(args) | {"crates": crates}, indent=2, default=str) + "\n")

    def run(crate):
        source_root = args.benchmark_root
        if args.override_root and (args.override_root / crate / "Cargo.toml").is_file():
            source_root = args.override_root
        command = [sys.executable, str(ROOT / "docker/rust-comparison/run.py"),
                   "culpa", crate, "--attempt", args.attempt,
                   "--benchmark-root", str(source_root),
                   "--output-root", str(output / "runs"), "--timeout", str(args.timeout),
                   "--jobs", "2", "--cpus", "2", "--memory", "12g"]
        if args.image:
            command.extend(["--image", args.image])
        for setting in args.env:
            command.extend(["--env", setting])
        if crate == "ptr_hash-0.1.1":
            command.append("--mount-patched")
        if crate == "jomini-0.27.0":
            command.append("--culpa-library-only")
        result = subprocess.run(command, capture_output=True, text=True)
        (output / f"{crate}.runner.log").write_text(result.stdout + result.stderr)
        return crate, result.returncode, result.stdout.strip(), result.stderr.strip()

    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        tasks = [pool.submit(run, crate) for crate in crates]
        for task in concurrent.futures.as_completed(tasks):
            crate, code, stdout, stderr = task.result()
            print(json.dumps({"crate": crate, "runner_exit": code,
                              "output": stdout, "error": stderr}), flush=True)


if __name__ == "__main__":
    main()
