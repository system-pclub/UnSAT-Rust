#!/usr/bin/env python3
"""Run a baseline on an isolated copy; preserve raw output and execution identity."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import time
import tomllib

ROOT = Path(__file__).resolve().parents[2]
TOOLS = ("semgrep", "lockbud", "prusti", "rudra", "mirai", "mirchecker")
BENCHMARK_ROOT = ROOT / "benchmark/ruklee"


def crate_choices():
    return tuple(sorted(path.name for path in BENCHMARK_ROOT.iterdir()
                        if path.is_dir() and (path / "Cargo.toml").is_file()))


def library_name(source):
    manifest = tomllib.loads((source / "Cargo.toml").read_text())
    return manifest.get("lib", {}).get(
        "name", manifest["package"]["name"].replace("-", "_"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tool", choices=TOOLS)
    parser.add_argument("crate", choices=crate_choices())
    parser.add_argument("--timeout", type=int, default=600)
    parser.add_argument("--memory", default="12g", help="Docker memory limit")
    parser.add_argument("--attempt", default="original")
    parser.add_argument("--lock-version", choices=("3", "4"))
    parser.add_argument("--vendor", type=Path)
    parser.add_argument("--semgrep-config", default="default-rust.yaml")
    parser.add_argument("--direct", action="store_true", help="Analyze dependency-free fast-float directly with the same default features")
    parser.add_argument("--checker-entry", help="MirChecker function entry name")
    args = parser.parse_args()
    source = BENCHMARK_ROOT / args.crate
    output = ROOT / "eval/rust-comparison" / args.crate / args.tool / args.attempt
    if output.exists():
        parser.error(f"Output already exists: {output}; use a new --attempt")
    output.mkdir(parents=True)
    work = ROOT / ".local/rust-comparison/work" / args.crate / args.tool / args.attempt
    shutil.copytree(source, work, ignore=shutil.ignore_patterns("target", ".git"))
    hashes = {str(p.relative_to(source)): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in sorted(source.rglob("*"))
              if p.is_file() and "target" not in p.relative_to(source).parts
              and ".git" not in p.relative_to(source).parts}
    modifications = []
    if args.lock_version:
        lock = work / "Cargo.lock"
        text = lock.read_text()
        if "\nversion = 4\n" in text:
            text = text.replace("\nversion = 4\n", f"\nversion = {args.lock_version}\n", 1)
        elif "\nversion = 3\n" in text:
            text = text.replace("\nversion = 3\n", f"\nversion = {args.lock_version}\n", 1)
        else:
            parser.error(f"Cannot locate Cargo.lock format marker in {lock}")
        lock.write_text(text)
        modifications.append(f"Cargo.lock format version {args.lock_version}; dependencies unchanged")
    mounts = []
    if args.vendor:
        vendor = args.vendor.resolve()
        (work / ".cargo").mkdir(exist_ok=True)
        (work / ".cargo/config.toml").write_text('[source.crates-io]\nreplace-with = "vendored-sources"\n[source.vendored-sources]\ndirectory = "/vendor"\n[net]\noffline = true\n')
        mounts = ["-v", f"{vendor}:/vendor:ro"]
        modifications.append("Use cargo-vendored exact locked dependency sources offline")
    image = f"uspec-rust-comparison-{args.tool}:local"
    commands = {
        "semgrep": ["semgrep", "scan", "--config", f"/rules/{args.semgrep_config}", "--json", "--metrics=off", "--disable-version-check", "--no-git-ignore", "--exclude", "target", "."],
        "lockbud": ["cargo", "lockbud", "-k", "deadlock", "-l", library_name(source), "--lib", "--locked"],
        "prusti": ["cargo", "prusti", "--lib", "--locked"],
        # Rudra selects each library target itself. Passing --lib through the
        # wrapper makes its internal `cargo check --lib` receive the flag
        # twice on crates whose metadata step succeeds.
        "rudra": ["cargo", "rudra", "--locked"],
        "mirai": ["cargo", "mirai", "--lib", "--locked"],
        "mirchecker": ["cargo", "mir-checker", "--lib", "--locked"],
    }
    if args.direct:
        if args.crate != "fast-float-0.2.0" or args.tool not in ("rudra", "mirchecker"):
            parser.error("--direct only supports Rudra/MirChecker on dependency-free fast-float")
        commands[args.tool] = ["rudra" if args.tool == "rudra" else "mir-checker",
                              "--crate-name", "fast_float", "--edition=2018", "src/lib.rs",
                              "--crate-type", "lib", "--cfg", 'feature="std"', "--emit=metadata"]
        modifications.append("Direct library analysis with default std feature; no runtime dependencies; bypass Cargo dev-dependency resolution")
    if args.checker_entry:
        if args.tool != "mirchecker":
            parser.error("--checker-entry requires mirchecker")
        if not args.direct:
            commands[args.tool].append("--")
        commands[args.tool].extend(["--entry", args.checker_entry])
    name = f"uspec-comparison-{args.tool}-{args.crate}-{args.attempt}"
    command = ["docker", "run", "--rm", "--name", name, "--cpus", "4", "--memory", args.memory,
               "--network", "host", "-v", f"{work}:/work", "-w", "/work",
               "-e", "CARGO_BUILD_JOBS=4", "-e", "CARGO_TERM_COLOR=never",
               "-v", f"{ROOT / 'docker/rust-comparison/rules'}:/rules:ro",
               *mounts, image, *commands[args.tool]]
    info = json.loads(subprocess.check_output(["docker", "image", "inspect", image]))[0]
    record = {"tool": args.tool, "crate": args.crate, "attempt": args.attempt,
              "benchmark_root": str(BENCHMARK_ROOT.relative_to(ROOT)),
              "source_sha256": hashes, "image": image, "image_id": info["Id"],
              "command": command, "modifications": modifications,
              "timeout_seconds": args.timeout, "started_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    if args.tool == "semgrep":
        config = ROOT / "docker/rust-comparison/rules" / args.semgrep_config
        record["rules_sha256"] = hashlib.sha256(config.read_bytes()).hexdigest()
    start = time.monotonic()
    with (output / "stdout.log").open("wb") as stdout, (output / "stderr.log").open("wb") as stderr:
        try:
            result = subprocess.run(command, stdout=stdout, stderr=stderr, timeout=args.timeout)
            record["exit_code"] = result.returncode
            record["status"] = "completed" if result.returncode == 0 else "failed"
        except subprocess.TimeoutExpired:
            subprocess.run(["docker", "rm", "-f", name], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            record.update(exit_code=None, status="timeout")
    record["elapsed_seconds"] = round(time.monotonic() - start, 3)
    if record["exit_code"] == 137:
        events = subprocess.run(
            ["docker", "events", "--since", record["started_at"], "--until", str(int(time.time())),
             "--filter", "container=" + name, "--format", "{{json .}}"],
            capture_output=True, text=True, timeout=15)
        (output / "docker-events.jsonl").write_text(events.stdout)
        record["oom_killed"] = any(json.loads(line).get("Action") == "oom"
                                   for line in events.stdout.splitlines())
    (output / "run.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({k: record[k] for k in ("tool", "crate", "attempt", "status", "exit_code", "elapsed_seconds")}))


if __name__ == "__main__":
    main()
