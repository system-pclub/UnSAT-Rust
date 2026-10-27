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
TOOLS = ("semgrep", "lockbud", "prusti", "rudra", "mirai", "mirchecker", "culpa")
BENCHMARK_ROOT = ROOT / "benchmark/ruklee"


def crate_choices(root=BENCHMARK_ROOT):
    return tuple(sorted(path.name for path in root.iterdir()
                        if path.is_dir() and (path / "Cargo.toml").is_file()))


def library_name(source):
    manifest = tomllib.loads((source / "Cargo.toml").read_text())
    return manifest.get("lib", {}).get(
        "name", manifest["package"]["name"].replace("-", "_"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tool", choices=TOOLS)
    parser.add_argument("crate")
    parser.add_argument("--benchmark-root", type=Path, default=BENCHMARK_ROOT)
    parser.add_argument("--output-root", type=Path, default=ROOT / "eval/rust-comparison")
    parser.add_argument("--timeout", type=int, default=600)
    parser.add_argument("--memory", default="12g", help="Docker memory limit")
    parser.add_argument("--cpus", default="4", help="Docker CPU limit")
    parser.add_argument("--jobs", type=int, default=4, help="Cargo build jobs")
    parser.add_argument("--image", help="Explicit analyzer image; its immutable ID is recorded")
    parser.add_argument("--env", action="append", default=[], metavar="KEY=VALUE", help="Additional container environment, retained in the run command")
    parser.add_argument("--culpa-enable-violations", action="store_true", help="Enable upstream's optional violation checks; default matches cargo culpa")
    parser.add_argument("--culpa-library-only", action="store_true", help="Use upstream's rustc wrapper on cargo check --lib without enumerating binaries")
    parser.add_argument("--mount-patched", action="store_true", help="Expose the repository's existing patched dependencies read-only at /patched")
    parser.add_argument("--attempt", default="original")
    parser.add_argument("--lock-version", choices=("1", "2", "3", "4"))
    parser.add_argument("--lockfile", type=Path, help="Use an explicitly pinned dependency lockfile in the work copy")
    parser.add_argument("--prepare-lock", action="store_true", help="Resolve a missing lockfile in the isolated work copy")
    parser.add_argument("--legacy-resolver", action="store_true", help="Enable the old nightly Cargo gate for the original resolver=2 manifest")
    parser.add_argument("--vendor", type=Path)
    parser.add_argument("--semgrep-config", default="default-rust.yaml")
    parser.add_argument("--direct", action="store_true", help="Analyze dependency-free fast-float directly with the same default features")
    parser.add_argument("--no-default-features", action="store_true", help="Disable default features in direct analysis")
    parser.add_argument("--checker-entry", help="MirChecker function entry name")
    args = parser.parse_args()
    source_root = args.benchmark_root.resolve()
    if args.crate not in crate_choices(source_root):
        parser.error(f"Unknown crate {args.crate} under {source_root}")
    source = source_root / args.crate
    output = args.output_root.resolve() / args.crate / args.tool / args.attempt
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
    if args.legacy_resolver:
        manifest_path = work / "Cargo.toml"
        manifest_text = manifest_path.read_text()
        if 'resolver = "2"' not in manifest_text:
            parser.error("--legacy-resolver requires an original resolver=2 manifest")
        manifest_path.write_text('cargo-features = ["resolver"]\n' + manifest_text)
        modifications.append("Enabled old nightly Cargo resolver gate; original resolver=2 and Rust source unchanged")
    if args.lockfile:
        shutil.copy2(args.lockfile.resolve(), work / "Cargo.lock")
        modifications.append("Pinned dependency lockfile: " + str(args.lockfile) + "; sha256=" + hashlib.sha256(args.lockfile.read_bytes()).hexdigest())
    if args.prepare_lock and not (work / "Cargo.lock").exists():
        resolution = subprocess.run(["cargo", "generate-lockfile"], cwd=work, capture_output=True, text=True, timeout=120)
        (output / "lock-resolution.log").write_text(resolution.stdout + resolution.stderr)
        if resolution.returncode:
            parser.error(f"Lockfile resolution failed; see {output / 'lock-resolution.log'}")
        modifications.append("Resolved missing Cargo.lock in isolated work copy; original manifest unchanged")
    if args.lock_version:
        lock = work / "Cargo.lock"
        text = lock.read_text()
        if "\nversion = 4\n" in text:
            text = text.replace("\nversion = 4\n", f"\nversion = {args.lock_version}\n", 1)
        elif "\nversion = 3\n" in text or "\nversion = 2\n" in text:
            text = text.replace("\nversion = 3\n", f"\nversion = {args.lock_version}\n", 1) if "\nversion = 3\n" in text else text.replace("\nversion = 2\n", f"\nversion = {args.lock_version}\n", 1)
        else:
            parser.error(f"Cannot locate Cargo.lock format marker in {lock}")
        if args.lock_version == "1":
            text = text.replace("\nversion = 1\n", "\n", 1)
        lock.write_text(text)
        modifications.append(f"Cargo.lock format version {args.lock_version}; dependencies unchanged")
    mounts = []
    if args.vendor:
        vendor = args.vendor.resolve()
        (work / ".cargo").mkdir(exist_ok=True)
        (work / ".cargo/config.toml").write_text('[source.crates-io]\nreplace-with = "vendored-sources"\n[source.vendored-sources]\ndirectory = "/vendor"\n[net]\noffline = true\n')
        mounts = ["-v", f"{vendor}:/vendor:ro"]
        modifications.append("Use cargo-vendored exact locked dependency sources offline")
    path_dependency_hashes = {}
    if args.mount_patched:
        patched = ROOT / "patched"
        mounts.extend(["-v", f"{patched}:/patched:ro"])
        path_dependency_hashes = {
            str(p.relative_to(patched)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(patched.rglob("*")) if p.is_file()
            and "target" not in p.relative_to(patched).parts and ".git" not in p.relative_to(patched).parts}
        modifications.append("Mounted existing repository patched dependencies read-only at /patched; source and manifests unchanged")
    image = args.image or f"uspec-rust-comparison-{args.tool}:local"
    commands = {
        "semgrep": ["semgrep", "scan", "--config", f"/rules/{args.semgrep_config}", "--json", "--metrics=off", "--disable-version-check", "--no-git-ignore", "--exclude", "target", "."],
        "lockbud": ["cargo", "lockbud", "-k", "deadlock", "-l", library_name(source), "--lib", "--locked"],
        "prusti": ["cargo", "prusti", "--lib", "--locked"],
        # Rudra selects each library target itself. Passing --lib through the
        # wrapper makes its internal `cargo check --lib` receive the flag
        # twice on crates whose metadata step succeeds.
        "rudra": ["cargo", "rudra", "--locked"],
        "mirai": ["cargo", "mirai", "--lib", "--locked"],
        "mirchecker": ["cargo", "mir-checker", "--locked"],
        "culpa": ["cargo", "culpa", "--locked"],
    }
    if args.culpa_enable_violations:
        if args.tool != "culpa":
            parser.error("--culpa-enable-violations requires culpa")
        commands["culpa"].append("--enable-violations")
    if args.culpa_library_only:
        if args.tool != "culpa":
            parser.error("--culpa-library-only requires culpa")
        commands["culpa"] = ["cargo", "check", "--lib", "--locked", "--target", "x86_64-unknown-linux-gnu"]
        modifications.append("Selected library target through upstream cargo-culpa rustc wrapper; package features unchanged")
    if args.direct:
        if args.tool not in ("rudra", "mirchecker"):
            parser.error("--direct only supports Rudra/MirChecker")
        manifest = tomllib.loads((source / "Cargo.toml").read_text())
        dependencies = list(manifest.get("dependencies", {}).values())
        for target in manifest.get("target", {}).values():
            dependencies.extend(target.get("dependencies", {}).values())
        if any(not isinstance(dep, dict) or not dep.get("optional", False) for dep in dependencies):
            parser.error("--direct requires no non-optional runtime dependencies")
        features = set() if args.no_default_features else set(manifest.get("features", {}).get("default", []))
        pending = list(features)
        while pending:
            for feature in manifest.get("features", {}).get(pending.pop(), []):
                if feature not in features:
                    features.add(feature)
                    pending.append(feature)
        if any(feature not in manifest.get("features", {}) for feature in features):
            parser.error("--direct cannot enable optional dependencies via default features")
        commands[args.tool] = ["rudra" if args.tool == "rudra" else "mir-checker",
                              "--crate-name", library_name(source),
                              "--edition=" + str(manifest["package"].get("edition", "2015")),
                              manifest.get("lib", {}).get("path", "src/lib.rs"),
                              "--crate-type", "lib", "--emit=metadata"]
        for feature in sorted(features):
            commands[args.tool].extend(["--cfg", f'feature="{feature}"'])
        feature_note = "default features disabled" if args.no_default_features else "original default features"
        modifications.append(f"Direct library analysis with {feature_note} and no runtime dependencies; bypass Cargo optional/dev-dependency resolution")
    if args.checker_entry:
        if args.tool != "mirchecker":
            parser.error("--checker-entry requires mirchecker")
        if not args.direct:
            commands[args.tool].append("--")
        commands[args.tool].extend(["--entry", args.checker_entry])
    name = f"uspec-comparison-{args.tool}-{args.crate}-{args.attempt}"
    tool_env = []
    if args.tool == "rudra" and not args.direct:
        # Upstream's old sccache cannot spawn the rustup proxy reliably here.
        # Hiding that optional cache leaves the analyzer/compiler unchanged.
        tool_env = ["-e", "PATH=/usr/local/cargo/bin:/usr/bin:/bin"]
    if args.tool == "culpa":
        tool_env = ["-e", "CULPA_FINDINGS_DIR=/artifacts/findings"]
        if args.culpa_library_only:
            manifest = tomllib.loads((source / "Cargo.toml").read_text())
            tool_env.extend(["-e", "RUSTC_WRAPPER=cargo-culpa",
                             "-e", "CULPA_TARGET_PACKAGE=" + manifest["package"]["name"],
                             "-e", "CULPA_TARGET_CRATES=" + library_name(source)])
            if args.culpa_enable_violations:
                tool_env.extend(["-e", "CULPA_ENABLE_VIOLATIONS=1"])
        mounts.extend(["-v", f"{output}:/artifacts"])
    for setting in args.env:
        if "=" not in setting or not setting.split("=", 1)[0]:
            parser.error("--env requires KEY=VALUE")
        tool_env.extend(["-e", setting])
    command = ["docker", "run", "--rm", "--name", name, "--cpus", args.cpus, "--memory", args.memory,
               "--network", "host", "-v", f"{work}:/work", "-w", "/work",
               "-e", f"CARGO_BUILD_JOBS={args.jobs}", "-e", "CARGO_TERM_COLOR=never",
               "-v", f"{ROOT / 'docker/rust-comparison/rules'}:/rules:ro",
               *mounts, *tool_env, image, *commands[args.tool]]
    info = json.loads(subprocess.check_output(["docker", "image", "inspect", image]))[0]
    record = {"tool": args.tool, "crate": args.crate, "attempt": args.attempt,
              "benchmark_root": str(source_root.relative_to(ROOT)),
              "source_sha256": hashes, "image": image, "image_id": info["Id"],
              "image_labels": info.get("Config", {}).get("Labels", {}),
              "path_dependency_sha256": path_dependency_hashes,
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
    if args.tool == "culpa":
        # Keep the actual lockfile even when Cargo metadata updated it before --locked.
        if (work / "Cargo.lock").exists():
            shutil.copy2(work / "Cargo.lock", output / "Cargo.lock")
        record["findings_files"] = sorted(str(p.relative_to(output)) for p in
                                         (output / "findings").glob("*.json"))
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
