#!/usr/bin/env python3
"""Run the RustSec matrix, keeping original archives and each attempt's logs."""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import time
import tomllib

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = json.loads((ROOT / "benchmark/rustsec/manifest.json").read_text())
RESULT_ROOT = ROOT / "eval/rustsec"
TOOLCHAIN = ROOT / ".local/rustsec-toolchain/nightly-2025-03-28-x86_64-unknown-linux-gnu"


def environment():
    env = dict(os.environ)
    env["PATH"] = str(ROOT / ".local/rustsec-bin") + ":" + str(TOOLCHAIN / "bin") + ":" + env["PATH"]
    env["LD_LIBRARY_PATH"] = str(TOOLCHAIN / "lib")
    env["RUSTC_BOOTSTRAP"] = "1"
    env["CARGO_BUILD_JOBS"] = "4"
    env["KLEE_RUNTIME_LIBRARY_PATH"] = str(ROOT / "tools/klee/build/runtime/lib")
    env["PYTHONUNBUFFERED"] = "1"
    # The repository already supplies credentials for its experiment tools.
    dotenv = ROOT / ".env"
    for line in dotenv.read_text().splitlines() if dotenv.exists() else []:
        if line.startswith("OPENAI_API_KEY="):
            env["OPENAI_API_KEY"] = line.split("=", 1)[1].strip().strip("\"'")
    return env


def run(command, output, timeout=600, cwd=ROOT):
    output.mkdir(parents=True, exist_ok=False)
    record = {"command": list(map(str, command)), "cwd": str(cwd), "timeout_seconds": timeout,
              "started_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    start = time.monotonic()
    (output / "run.json").write_text(json.dumps(record, indent=2) + "\n")
    with (output / "stdout.log").open("w") as stdout, (output / "stderr.log").open("w") as stderr:
        try:
            p = subprocess.Popen(command, cwd=cwd, env=environment(), stdout=stdout, stderr=stderr, start_new_session=True)
            try:
                record["exit_code"] = p.wait(timeout=timeout)
                record["status"] = "completed" if p.returncode == 0 else "failed"
            except subprocess.TimeoutExpired:
                import signal
                os.killpg(p.pid, signal.SIGTERM)
                try:
                    p.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(p.pid, signal.SIGKILL)
                    p.wait()
                record.update(status="timeout", exit_code=None)
        except Exception as exc:
            record.update(status="failed", error=str(exc), exit_code=None)
    record["elapsed_seconds"] = round(time.monotonic() - start, 3)
    (output / "run.json").write_text(json.dumps(record, indent=2) + "\n")
    print(output.relative_to(ROOT), record["status"], record["elapsed_seconds"], flush=True)
    return record


def workcopy(case, tool, attempt):
    parent = ROOT / ".local/rustsec-work" / tool / attempt
    dest = parent / Path(case["directory"]).name
    if dest.exists():
        return dest
    parent.mkdir(parents=True, exist_ok=True)
    archive = ROOT / "benchmark/rustsec/evidence" / case["advisory"] / f'{case["crate"]}-{case["version"]}.crate'
    with tarfile.open(archive) as tar:
        tar.extractall(parent, filter="data")
    lock = ROOT / "benchmark/rustsec/evidence" / case["advisory"] / "Cargo.lock"
    if lock.exists():
        shutil.copy2(lock, dest / "Cargo.lock")
    # scanner's optional pelite 0.2 default feature no longer compiles on the
    # experiment toolchain. The affected Match API does not depend on pelite.
    if case["crate"] == "scanner":
        manifest = dest / "Cargo.toml"
        text = manifest.read_text()
        text = text.replace('default = ["pelite"]', 'default = []')
        manifest.write_text(text)
    return dest


def baseline(job):
    case, tool, args = job
    crate = Path(case["directory"]).name
    output = RESULT_ROOT / crate / tool / args.attempt
    if output.exists():
        return
    command = ["python3", "docker/rust-comparison/run.py", tool, crate,
               "--benchmark-root", "benchmark/rustsec", "--output-root", str(RESULT_ROOT),
               "--attempt", args.attempt, "--timeout", str(args.timeout)]
    manifest = tomllib.loads((ROOT / case["directory"] / "Cargo.toml").read_text())
    deps = list(manifest.get("dependencies", {}).values())
    direct = not any(not isinstance(dep, dict) or not dep.get("optional", False) for dep in deps)
    if tool in ("rudra", "mirchecker") and direct:
        command.append("--direct")
        if case["crate"] == "scanner":
            command.append("--no-default-features")
    elif tool != "semgrep":
        command.extend(["--prepare-lock", "--lock-version", "3"])
    lock = ROOT / "benchmark/rustsec/evidence" / case["advisory"] / "Cargo.lock"
    if args.pinned and lock.exists() and tool != "semgrep":
        command.extend(["--lockfile", str(lock)])
        if tool in ("rudra", "mirchecker"):
            command.extend(["--vendor", str(ROOT / ".local/rustsec-vendor" / case["crate"])])
            if tool == "mirchecker":
                command.extend(["--lock-version", "2"])
    if tool == "mirchecker":
        entries = {"binary_vec_io": "binary_read_to_ref", "borrowck_sacrifices": "any_as_u8_slice",
                   "direct_ring_buffer": "create_ring_buffer", "maligned": "align_first",
                   "safe-transmute": "guarded_transmute_bool_vec_permissive", "byte_struct": "read_bytes_default_le",
                   "elf_rs": "section_header_raw", "smallvec": "new", "linked-hash-map": "ensure_guard_node", "lzf": "compress",
                   "fast-able": "get_uncheck", "shaman": "write_u64v_le", "scanner": "ptr",
                   "anon-vec": "get_ref", "fast_id_map": "get",
                   "orx-pinned-vec": "index_of_ptr", "tanton_engine": "get",
                   "xmas-elf": "get_bucket", "slab": "get_disjoint_mut"}
        command.extend(["--checker-entry", entries[case["crate"]]])
        if args.pinned and lock.exists():
            command.extend(["--lock-version", "1"])
    wrapper = RESULT_ROOT / "driver" / crate / tool / args.attempt
    return run(command, wrapper, timeout=args.timeout + 160)


def local(case, args):
    tool = args.mode
    crate = Path(case["directory"]).name
    output = RESULT_ROOT / crate / tool / args.attempt
    if output.exists():
        return
    work = workcopy(case, tool, args.attempt)
    if tool == "ruklee" and args.instantiate:
        bodies = {
            "binary_vec_io": 'let mut f = std::fs::OpenOptions::new().read(true).write(true).open("/dev/null").unwrap(); let mut x = 0u8; let _ = crate::binary_write_from_ref(&mut f, &x, 0); let _ = crate::binary_read_to_ref(&mut f, &mut x, 0);',
            "borrowck_sacrifices": 'let x = 1u64; let _ = crate::unsafe_casts::any_as_u8_slice(&x);',
            "direct_ring_buffer": 'let _ = crate::create_ring_buffer::<u8>(4);',
            "maligned": 'let _ = crate::align_first::<u8, crate::A16>(8);',
            "safe-transmute": 'let _ = crate::guarded_transmute_pod_vec_permissive::<u16>(Vec::new()); let _ = crate::guarded_transmute_bool_vec_permissive(Vec::new()); let _ = crate::guarded_transmute_to_bytes_pod_vec::<u16>(Vec::new());',
            "smallvec": 'let _: crate::SmallVec<[u8; 4]> = crate::SmallVec::new();',
            "lzf": 'let _ = crate::compress(b"abc");',
            "fast-able": 'let v = crate::vec::SyncVec::<u8>::new(); let _ = v.get_uncheck(0);',
            "scanner": 'let m = crate::Match { haystack: &[], at: 0, store: [0; 5] }; let _ = m.get::<u8>(0);',
            "anon-vec": 'let v = crate::AnonVec::new::<u8>(); let _ = v.get_ref::<u8>(0);',
            "fast_id_map": 'let mut m = crate::prelude::FastMap::<u8>::new(); let _ = m.get(0);',
            "orx-pinned-vec": 'let empty: &[u8] = &[]; let _ = crate::utils::slice::index_of_ptr(empty, empty.as_ptr());',
            "slab": 'let mut slab = crate::Slab::<u8>::with_capacity(1); let _ = slab.get_disjoint_mut([0usize]);',
        }
        manifest = tomllib.loads((work / "Cargo.toml").read_text())
        lib = work / manifest.get("lib", {}).get("path", "src/lib.rs")
        harness = lib.parent / "__rustsec_instantiations.rs"
        harness.write_text('#[test]\nfn instantiate_public_api() {\n    ' + bodies[case["crate"]] + '\n}\n')
        text = lib.read_text()
        declaration = '\n#[cfg(test)] mod __rustsec_instantiations;\n'
        if declaration not in text:
            lib.write_text(text + declaration)
    if tool == "sync":
        command = ["./x", "sync", "--cargo-dir", str(work), "--skip-autoinj"]
    elif tool == "safe4u":
        command = [str(ROOT / "tools/safe4u/.venv/bin/python"), str(ROOT / "tools/safe4u/cargo-safe4u"),
                   "--crate", str(work), "--out-dir", str(output / "results"), "--model", args.model,
                   "--jobs", "8", "--no-color"]
    else:
        command = ["./x", "verify", str(work), "--rustc", str(ROOT / "tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"),
                   "--klee-bin", str(ROOT / "tools/klee/build/bin/klee"),
                   "--ir-output-dir", str(ROOT / ".local/rustsec-irs" / crate),
                   "--results-json", str(output / "matrix.json"), "--logs-dir", str(output / "logs"),
                   "--artifacts-dir", str(output / "artifacts"), "--timeout-sec", str(args.timeout)]
        if not args.instantiate and not args.tests:
            command.append("--no-test")
        if not args.full:
            calls = ROOT / "benchmark/rustsec/targets" / (crate + ".txt")
            if calls.exists():
                command.extend(["--callsites-file", str(calls)])
            else:
                raise RuntimeError(f"Missing reviewed advisory target list: {calls}")
        if args.skip_llm:
            command.append("--skip-llm-testcase")
        if args.rule:
            command.extend(["--rule", args.rule])
        if args.no_debug_assertions:
            command.append("--no-debug-assertions")
        if args.stop_after_violation:
            command.append("--stop-callsite-if-violated")
    result = run(command, output, timeout=args.timeout if tool in ("sync", "safe4u") else 2400,
                 cwd=ROOT / "tools/safe4u" if tool == "safe4u" else ROOT)
    source_hashes = {str(p.relative_to(work)): hashlib.sha256(p.read_bytes()).hexdigest()
                     for p in sorted(work.rglob("*")) if p.is_file()
                     and "target" not in p.relative_to(work).parts and ".git" not in p.relative_to(work).parts}
    (output / "source-sha256.json").write_text(json.dumps(source_hashes, indent=2) + "\n")
    for harness in work.rglob("__rustsec_instantiations.rs"):
        shutil.copy2(harness, output / harness.name)
    if tool in ("sync", "ruklee"):
        for path in [work / "report.json", ROOT / "crates" / (crate + ".json")]:
            if path.exists():
                shutil.copy2(path, output / path.name)
    return result


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("mode", choices=["baselines", "sync", "safe4u", "ruklee"])
    p.add_argument("--manifest", type=Path, default=ROOT / "benchmark/rustsec/manifest.json")
    p.add_argument("--output-root", type=Path, default=RESULT_ROOT)
    p.add_argument("--attempt", default="initial")
    p.add_argument("--timeout", type=int, default=240)
    p.add_argument("--workers", type=int, default=6)
    p.add_argument("--model", default="gpt-4o-mini")
    p.add_argument("--crate", action="append")
    p.add_argument("--full", action="store_true")
    p.add_argument("--skip-llm", action="store_true")
    p.add_argument("--pinned", action="store_true")
    p.add_argument("--instantiate", action="store_true", help="Compile explicit safe API type instantiations as unit tests (assisted coverage)")
    p.add_argument("--tests", action="store_true", help="Compile the crate's existing unit tests without adding instantiations")
    p.add_argument("--stop-after-violation", action="store_true")
    p.add_argument("--rule", help="Evaluate one frozen DSL rule id")
    p.add_argument("--no-debug-assertions", action="store_true", help="Disable debug assertions for release-only paths")
    p.add_argument("--tool", action="append", choices=["semgrep", "rudra", "mirai", "mirchecker", "lockbud", "prusti"])
    args = p.parse_args()
    MANIFEST = json.loads(args.manifest.read_text())
    RESULT_ROOT = args.output_root.resolve()
    cases = [c for c in MANIFEST["cases"] if not args.crate or c["crate"] in args.crate]
    if args.mode == "baselines":
        jobs = [(c, t, args) for c in cases for t in (args.tool or ("semgrep", "rudra", "mirai", "mirchecker", "lockbud", "prusti"))]
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
            list(pool.map(baseline, jobs))
    else:
        # Both local tools use shared metadata/config files: serialize crates.
        for case in cases:
            local(case, args)
