#!/usr/bin/env python3
"""Run the pinned RustSec callsite/rule pairs through the ./x wrapper (Docker)."""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import fcntl
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time
import re

ROOT = Path(__file__).resolve().parents[2]
SUPPORT = Path(__file__).resolve().parent
BENCHMARK = ROOT / "benchmark/rustsec"
DEFAULT_OUTPUT = ROOT / "eval/rustsec20"
# Statuses where the symbolic stage found a violating state for the advisory.
DETECTED = {"violation", "violation-timeout", "low-confidence-sat"}

# crate -> {MIRScan callsite id: relevant rule ids}. Locations refer to the
# original pinned sources, not injected copies. Advisory evidence is under
# evidence/<RUSTSEC id>/advisory.md. Never fall back to scanning an entire crate.
ADVISORY_TARGETS = {
    "stack-0.3.0": {'src-vector-rs-56-13': ('rule-955',)},
    "redox_uefi_std-0.1.13": {'src-ffi-rs-20-27': ('rule-289',)},
    "rtvm-interpreter-4.0.0": {'src-interpreter-rs-323-18': ('rule-294',)},
    "ozone-0.1.0": {'src-map-rs-95-35': ('rule-192',)},
    "pprof-0.13.0": {'src-addr_validate-rs-93-28': ('rule-610', 'rule-611')},
    "grcov-0.8.19": {'src-covdir-rs-48-39': ('rule-576',)},
    "bytes-1.11.0": {'src-bytes_mut-rs-1117-13': ('rule-580',)},
    "memmap2-0.9.10": {'src-unix-rs-362-34': ('rule-431',), 'src-unix-rs-375-34': ('rule-431',), 'src-unix-rs-439-21': ('rule-431',)},
    "auto_vec-0.2.0": {'src-iter-rs-7-30': ('rule-304',), 'src-iter-rs-14-30': ('rule-304',)},
    "reorder-1.0.3": {'src-lib-rs-48-31': ('rule-587',)},
    "rdiff-0.1.2": {'src-window-rs-13-13': ('rule-961',), 'src-window-rs-17-13': ('rule-961',)},
    "anon-vec-0.1.1": {'src-vec-rs-206-20': ('rule-301',)},
    "bam-0.1.2": {'src-bgzip-mod-rs-328-13': ('rule-961',)},
    "binary_vec_io-0.1.12": {'src-lib-rs-31-26': ('rule-610',), 'src-lib-rs-51-19': ('rule-447',), 'src-lib-rs-52-34': ('rule-586',)},
    "fast-able-1.11.17": {'src-vec-rs-184-18': ('rule-572',)},
    "fast_id_map-0.1.0": {'src-fast_map-rs-22-18': ('rule-576',)},
    "iced-x86-1.10.3": {'src-decoder-mod-rs-458-42': ('rule-575',)},
    "orx-pinned-vec-3.20.0": {'src-utils-slice-rs-39-33': ('rule-301',)},
    "partial_sort-0.1.2": {'src-lib-rs-66-17-get_unchecked': ('rule-572',), 'src-lib-rs-70-9-get_unchecked': ('rule-572',)},
    "ruint-1.17.0": {'src-algorithms-div-reciprocal-rs-87-33': ('rule-572',)},
    "scanner-0.1.0": {'src-lib-rs-413-12': ('rule-289',)},
    "shaman-0.1.0": {'src-cryptoutil-rs-45-30': ('rule-576',)},
    "slab-0.4.10": {'src-lib-rs-836-28': ('__raw_ptr_deref__',)},
    "simple-slab-0.3.2": {'src-lib-rs-96-25': ('rule-321',)},
    "smallvec-0.6.9": {},
    "tanton_engine-1.0.0": {'src-search-mod-rs-128-18': ('rule-576',)},
    "transpose-0.2.2": {'src-out_of_place-rs-234-18-get_unchecked_mut': ('rule-576',), 'src-out_of_place-rs-237-9-get_unchecked_mut': ('rule-576',), 'src-out_of_place-rs-240-9-get_unchecked_mut': ('rule-576',)},
    "xmas-elf-0.9.1": {'src-hash-rs-31-23': ('rule-289',)},
    "arrow2-0.18.0": {'src-compute-sort-row-mod-rs-274-24': ('rule-572',), 'src-compute-sort-row-mod-rs-275-26': ('rule-572',), 'src-compute-sort-row-mod-rs-276-13': ('rule-572',)},
    "dahl-salso-0.6.6": {'src-clustering-rs-98-18': ('rule-572',)},
    "i_tree-0.9.0": {'src-tree-rs-65-13': ('rule-572',), 'src-tree-rs-72-13': ('rule-576',)},
    "i_triangle-0.26.0": {'src-delaunay-triangle-rs-13-14': ('rule-572',), 'src-delaunay-triangle-rs-19-14': ('rule-572',)},
    "lexical-util-0.8.5": {'src-skip-rs-449-27': ('rule-324',)},
    "rustdx-0.4.3": {'src-bytes_helper-rs-24-34': ('rule-572',), 'src-bytes_helper-rs-37-38': ('rule-572',)},
    "toodee-0.5.0": {'src-toodee-rs-693-25': ('rule-447',), 'src-toodee-rs-696-13': ('rule-955',), 'src-toodee-rs-1071-25': ('rule-955', 'rule-956'), 'src-toodee-rs-1076-21': ('rule-955', 'rule-956')},
    "tremor-script-0.11.5": {},
    "xous-0.9.50": {'src-definitions-rs-323-13': ('rule-609', 'rule-611'), 'src-definitions-rs-340-13': ('rule-585', 'rule-587')},
}

# Cargo features needed for the advisory code to be compiled.
# Advisories whose unsafe call sits behind a private safe helper of a public API.
_FOLLOW = {"MIRSCAN_FOLLOW_SAFE_HELPERS": "1", "MIRSCAN_MAX_CALL_DEPTH": "3"}
MIRSCAN_ENV = {'grcov-0.8.19': _FOLLOW, 'transpose-0.2.2': _FOLLOW, 'partial_sort-0.1.2': _FOLLOW}
FEATURES = {
    "lexical-util-0.8.5": 'format,parse,parse-integers',
    "arrow2-0.18.0": 'compute_sort',
}
# These advisories require release-style assertions/overflow behavior.
RELEASE_ONLY = {
    "bam-0.1.2",
    "bytes-1.11.0",
    "orx-pinned-vec-3.20.0",
    "partial_sort-0.1.2",
    "ruint-1.17.0",
    "stack-0.3.0",
    "tanton_engine-1.0.0",
    "transpose-0.2.2",
}
# Minimal safe-API calls that make generic callers appear in LLVM IR. They are
# written into the .local work copy as a #[test] only; benchmark sources stay pristine.
INSTANTIATIONS = {
    "partial_sort-0.1.2": 'let mut v = [3u8, 1, 2]; crate::partial_sort(&mut v, 2, |a, b| a < b);',
    "transpose-0.2.2": 'let i = [1u8, 2, 3, 4]; let mut o = [0u8; 4]; crate::out_of_place::transpose(&i, &mut o, 2, 2);',
    "bam-0.1.2": 'let mut b = crate::bgzip::Block::new(); let d = [31u8, 139, 8, 4, 0, 0, 0, 0, 0, 255, 6, 0, 66, 67, 2, 0, 0, 0, 0, 0, 0, 0]; let mut s: &[u8] = &d; let _ = b.load(None, &mut s);',
    "binary_vec_io-0.1.12": 'let mut f = std::fs::File::open("/dev/null").unwrap(); let x = 0u32; let _ = crate::binary_write_from_ref(&mut f, &x, 1); let mut y = 0u32; let _ = crate::binary_read_to_ref(&mut f, &mut y, 1);',
    "fast_id_map-0.1.0": 'let mut m = crate::fast_map::FastMap::<u8>::with_capacity(2); let _ = m.get(0);',
    "slab-0.4.10": 'let mut s = crate::Slab::<u8>::new(); let k = s.insert(1); let _ = s.get_disjoint_mut([k]);',
    "grcov-0.8.19": 'let mut c = std::collections::BTreeMap::new(); c.insert(0u32, 1u64); let _ = crate::defs::CDFileStats::new("a".to_string(), c, 2);',
    "pprof-0.13.0": 'let _ = crate::addr_validate::validate(8usize as *const libc::c_void);',
    "ozone-0.1.0": 'let mut m = crate::HashMap::<String, String>::new(); m.insert("a".to_string(), "b".to_string()); if let crate::map::Entry::Occupied(e) = m.entry("a".to_string()) { let _ = e.remove_entry(); }',
    "rtvm-interpreter-4.0.0": 'let other = [0u8; 1]; let mut i = crate::Interpreter::default(); i.instruction_pointer = other.as_ptr(); let _ = i.program_counter();',
    "redox_uefi_std-0.1.13": 'let v = [1u16, 2u16, 3u16]; let _ = crate::ffi::nstr(v.as_ptr());',
    "stack-0.3.0": 'use crate::Vector; let mut v = crate::ArrayVec::<[u8; 4]>::with_capacity(4); v.insert(7, 1u8);',
    "bytes-1.11.0": 'let mut a = crate::BytesMut::from(&b"hello world"[..]); let mut b = a.split_off(5); drop(a); b.reserve(usize::MAX - 6); let _ = b.spare_capacity_mut();',
    "smallvec-0.6.9": 'let mut v = crate::SmallVec::<[u8; 4]>::with_capacity(8); v.grow(2); let _ = v.into_vec();',
    "auto_vec-0.2.0": 'let mut v = crate::AutoVec::<u8>::new(); let mut c = crate::AutoChild::new(1u8); v.add(&mut c); let _ = v.iter(); let _ = v.iter_mut();',
    "reorder-1.0.3": 'struct Liar(u32); impl Iterator for Liar { type Item = u32; fn next(&mut self) -> Option<u32> { if self.0 < 2 { self.0 += 1; Some(self.0 - 1) } else { None } } } impl ExactSizeIterator for Liar { fn len(&self) -> usize { 4 } } let _ = crate::swap_index(Liar(0));',
    "rdiff-0.1.2": 'struct Liar; impl std::io::Read for Liar { fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> { Ok(b.len() + 1) } } let _ = crate::Window::new(Liar, 2);',
    "orx-pinned-vec-3.20.0": 'let empty: &[u8] = &[]; let _ = crate::utils::slice::index_of_ptr(empty, empty.as_ptr());',
    "simple-slab-0.3.2": 'let mut s = crate::Slab::<u8>::new(); s.insert(1); let _ = s.remove(0);',
    "i_tree-0.9.0": 'let mut t = crate::tree::Tree::<u8>::new(0, 2); let _ = t.node(0); let _ = t.mut_node(0);',
    "i_triangle-0.26.0": 'let t = crate::delaunay::triangle::DTriangle::new(); let _ = t.neighbor_by_order(0); let _ = t.vertex_by_order(0);',
    "lexical-util-0.8.5": 'let b = [2u8]; let _ = crate::skip::Bytes::<{ crate::format::STANDARD }>::new(&b).read::<bool>();',
    "xous-0.9.50": 'let a = core::num::NonZeroUsize::new(4096).unwrap(); let mut m = crate::MemoryRange::from_parts(a, a); let _ = m.as_slice::<bool>(); let _ = m.as_slice_mut::<bool>();',
    "rustdx-0.4.3": 'let b = [1u8, 2]; let _ = crate::bytes_helper::into_arr2(&b, 0); let _ = crate::bytes_helper::u8_from_le_bytes(&b, 0);',
    "toodee-0.5.0": 'let mut t = crate::TooDee::<u8>::init(2, 2, 0); t.insert_row(0, vec![1u8, 2]); drop(t.remove_col(0));',
}
UNSUPPORTED = {
    "smallvec-0.6.9": 'The grow defect reaches Vec::from_raw_parts ownership/allocation-layout obligations; the current catalog has no matching Vec::from_raw_parts DSL. Old rule-965/rule-966 references were undefined.',
    "tremor-script-0.11.5": 'The merge/patch defect uses core::intrinsics::transmute to create mutable aliases; the current catalog has no matching intrinsic aliasing/lifetime contract. transmutability::transmute is a different API.',
}


def read_json(path):
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return {}


def selected_pairs(crate):
    return {(target, rule) for target, rules in ADVISORY_TARGETS[crate].items()
            for rule in rules}


def matrix_pairs(matrix):
    return {(row.get("callsite"), row.get("rule"))
            for row in matrix.get("results", []) if isinstance(row, dict)}


def environment():
    """Environment for ./x: it runs everything inside the unsat-rust:local image."""
    env = dict(os.environ)
    env["PYTHONUNBUFFERED"] = "1"
    for dotenv in (ROOT / ".local" / ".env", ROOT / ".env"):
        lines = dotenv.read_text().splitlines() if dotenv.exists() else []
        for line in lines:
            if line.startswith("OPENAI_API_KEY=") and "OPENAI_API_KEY" not in env:
                env["OPENAI_API_KEY"] = line.split("=", 1)[1].strip().strip("\"'")
    return env


def package_name(source):
    match = re.search(r'(?ms)^\[package\].*?^name\s*=\s*"([^"]+)"', (source / "Cargo.toml").read_text())
    return match.group(1) if match else None


def lib_path(work):
    manifest = (work / "Cargo.toml").read_text()
    match = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', manifest)
    return work / (match.group(1) if match else "src/lib.rs")


def rel(path):
    """Path as seen inside the container, where the repository is the cwd."""
    return str(Path(path).resolve().relative_to(ROOT))


def detection(matrix, crate):
    """Callsite -> whether any selected rule reported a violating state."""
    found = {target: False for target in ADVISORY_TARGETS[crate]}
    for row in matrix.get("results", []):
        if row.get("callsite") in found and row.get("status") in DETECTED:
            found[row["callsite"]] = True
    return found


def write_json(path, value):
    temporary = path.with_suffix(".tmp")
    temporary.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
    temporary.replace(path)


def write_selection(crate, output):
    """Use the existing verifier's filters, preserving the frozen DSL entries."""
    targets = ADVISORY_TARGETS[crate]
    rule_ids = sorted({rule for rules in targets.values() for rule in rules})
    document = read_json(ROOT / "ptr_rule_dsl.json")
    rules = document.get("rules", document)
    dsl_ids = [rule for rule in rule_ids if rule != "__raw_ptr_deref__"]
    missing = set(dsl_ids) - rules.keys()
    if missing:
        raise ValueError(f"{crate}: missing DSL rules: {sorted(missing)}")
    calls_path = output / "callsites.txt"
    calls_path.write_text("\n".join(targets) + "\n")
    dsl_path = output / "rule-dsl.json"
    write_json(dsl_path, {rule: rules[rule] for rule in dsl_ids})
    write_json(output / "selection.json", targets)
    return ["--callsites-file", rel(calls_path), "--rule-dsl", rel(dsl_path)]


def stop_process(process):
    if process.poll() is not None:
        return
    os.killpg(process.pid, signal.SIGTERM)
    try:
        process.wait(timeout=10)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()


def run_crate(source, args, env, attempt):
    output = args.output_root / source.name / "ruklee" / attempt
    # Unique attempts keep old logs, generated witnesses and source copies intact.
    output.mkdir(parents=True, exist_ok=False)
    selection_args = write_selection(source.name, output)
    work = ROOT / ".local/rustsec-work/ruklee" / attempt / source.name
    work.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, work, ignore=shutil.ignore_patterns("target", ".git"))
    shutil.copy2(SUPPORT / "rust-toolchain.toml", work / "rust-toolchain.toml")
    # Reuse the reviewed dependency resolutions instead of resolving old broad
    # version requirements to incompatible modern dependencies. A Cargo.lock
    # shipped in the benchmark directory is already part of the copy.
    for provenance in (SUPPORT / "evidence").glob("*/provenance.json"):
        case = read_json(provenance)
        if Path(case.get("directory", "")).name == source.name:
            lockfile = provenance.parent / "Cargo.lock"
            if lockfile.exists():
                shutil.copy2(lockfile, work / "Cargo.lock")
            break
    if source.name == "scanner-0.1.0":
        manifest = work / "Cargo.toml"
        manifest.write_text(manifest.read_text().replace('default = ["pelite"]', 'default = []'))
    if source.name in INSTANTIATIONS:
        lib = lib_path(work)
        (lib.parent / "__rustsec_instantiations.rs").write_text(
            "#[test]\nfn instantiate_public_api() {\n    " + INSTANTIATIONS[source.name] + "\n}\n")
        declaration = "\n#[cfg(test)] mod __rustsec_instantiations;\n"
        lib.write_text(lib.read_text() + declaration)
    command = [str(ROOT / "x"), "verify", rel(work),
               "--ir-output-dir", rel(ROOT / ".local/rustsec-irs" / source.name),
               "--results-json", rel(output / "matrix.json"),
               "--logs-dir", rel(output / "logs"),
               "--artifacts-dir", rel(output / "artifacts"),
               "--timeout-sec", str(args.timeout),
               "--test" if args.tests or source.name in INSTANTIATIONS else "--no-test",
               *selection_args]
    if args.skip_llm:
        command.append("--skip-llm-testcase")
    if args.no_debug_assertions or source.name in RELEASE_ONLY:
        command.append("--no-debug-assertions")
    if source.name in FEATURES:
        command += ["--features", FEATURES[source.name]]
    record = {"runner": 2, "command": command, "cwd": str(ROOT), "source": str(source),
              "selection": ADVISORY_TARGETS[source.name],
              "started_at": datetime.now(timezone.utc).isoformat(),
              "timeout_seconds": args.crate_timeout, "status": "running"}
    write_json(output / "run.json", record)
    start = time.monotonic()
    process = None
    try:
        with (output / "stdout.log").open("w") as stdout, (output / "stderr.log").open("w") as stderr:
            process = subprocess.Popen(command, cwd=ROOT, env={**env, **MIRSCAN_ENV.get(source.name, {})}, stdout=stdout,
                                       stderr=stderr, start_new_session=True)
            record["exit_code"] = process.wait(timeout=args.crate_timeout or None)
            record["status"] = "completed" if record["exit_code"] == 0 else "failed"
            if record["status"] == "completed":
                matrix = read_json(output / "matrix.json")
                expected = selected_pairs(source.name)
                actual = matrix_pairs(matrix)
                if (actual != expected or matrix.get("total") != len(expected)
                        or matrix.get("completed") != len(expected)):
                    record.update(status="failed", error="advisory selection was not fully executed",
                                  missing_pairs=sorted(expected - actual),
                                  unexpected_pairs=sorted(actual - expected))
                record["detected"] = detection(matrix, source.name)
                if not all(record["detected"].values()) and record["status"] == "completed":
                    record.update(status="missed", error="advisory callsite not reported SAT")
    except subprocess.TimeoutExpired:
        stop_process(process)
        record.update(status="timeout", exit_code=process.returncode)
    except KeyboardInterrupt:
        if process is not None:
            stop_process(process)
        record.update(status="interrupted", exit_code=None)
        raise
    except OSError as exc:
        record.update(status="failed", exit_code=None, error=str(exc))
    finally:
        record["elapsed_seconds"] = round(time.monotonic() - start, 3)
        write_json(output / "run.json", record)
    print(f"  {record['status']} ({record['elapsed_seconds']}s): {output.relative_to(ROOT) if output.is_relative_to(ROOT) else output}", flush=True)
    return record["status"] == "completed"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crate", action="append", help="Only this package name or directory; repeatable")
    parser.add_argument("--dry-run", action="store_true", help="List decisions and exact target/rule pairs without running tools")
    parser.add_argument("--output-root", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--timeout", type=int, default=60, help="Per KLEE invocation timeout in seconds (default: 60)")
    parser.add_argument("--crate-timeout", type=int, default=300,
                        help="Whole crate timeout in seconds (default: 300); 0 means unlimited")
    parser.add_argument("--tests", action="store_true", help="Compile existing library tests to instantiate more generic functions")
    parser.add_argument("--skip-llm", action="store_true", help="Only run the symbolic stage; skip generated PoCs/replays")
    parser.add_argument("--no-debug-assertions", action="store_true", help="Use release-style assertion behavior")
    parser.add_argument("--advisory-only", action="store_true", help="Compatibility flag: advisory-only is now always enabled")
    args = parser.parse_args()
    if args.timeout <= 0 or args.crate_timeout < 0:
        parser.error("--timeout must be positive and --crate-timeout nonnegative")
    args.output_root = args.output_root.resolve()
    crates = []
    matched = set()
    for source in sorted(BENCHMARK.iterdir()):
        if not source.is_dir() or not (source / "Cargo.toml").is_file():
            continue
        names = {source.name, package_name(source)}
        if args.crate and not names.intersection(args.crate):
            continue
        matched.update(names.intersection(args.crate or []))
        crates.append(source)
    if set(args.crate or []) - matched:
        parser.error(f"Unknown crate(s): {sorted(set(args.crate) - matched)}")
    missing = {source.name for source in crates} - ADVISORY_TARGETS.keys()
    if missing:
        parser.error(f"Missing hard-coded RustSec selections (no full-scan fallback): {sorted(missing)}")
    dsl = read_json(ROOT / "ptr_rule_dsl.json")
    known_rules = set(dsl.get("rules", dsl)) | {"__raw_ptr_deref__"}
    for source in crates:
        pairs = selected_pairs(source.name)
        if source.name not in UNSUPPORTED and not pairs:
            parser.error(f"Empty RustSec selection: {source.name}")
        unknown = {rule for _, rule in pairs} - known_rules
        if unknown:
            parser.error(f"Unknown DSL rules for {source.name}: {sorted(unknown)}")
    # The experiment tools share metadata and injected directories. Serialize
    # both crates and simultaneous invocations of this batch entrypoint.
    lock_path = ROOT / ".local/rustsec-batch.lock"
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    with lock_path.open("a") as lock:
        if not args.dry_run:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError:
                parser.error("Another dev/rustsec/run.py is running")
        attempt = datetime.now(timezone.utc).strftime("batch-%Y%m%dT%H%M%S-%f")
        env = None
        failed = ran = unsupported = 0
        for index, source in enumerate(crates, 1):
            if args.dry_run:
                for target, rules in ADVISORY_TARGETS[source.name].items():
                    print(f"  {source.name}: {target} -> {', '.join(rules) or '(no DSL)'}")
            if source.name in UNSUPPORTED:
                print(f"[{index}/{len(crates)}] UNSUPPORTED {source.name}: {UNSUPPORTED[source.name]}", flush=True)
                unsupported += 1
                continue
            release = args.no_debug_assertions or source.name in RELEASE_ONLY
            print(f"[{index}/{len(crates)}] RUN  {source.name} "
                  f"targets={len(ADVISORY_TARGETS[source.name])} pairs={len(selected_pairs(source.name))}"
                  f"{' no-debug-assertions' if release else ''}", flush=True)
            if not args.dry_run:
                if env is None:
                    env = environment()
                if not run_crate(source, args, env, attempt):
                    failed += 1
            ran += 1
        print(f"{'Plan' if args.dry_run else 'Done'}: run={ran}, failed={failed}, unsupported={unsupported}")
        return int(failed > 0)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\nInterrupted; logs preserved.", file=sys.stderr)
        sys.exit(130)
