#!/usr/bin/env python3
"""Run only the pinned RustSec callsite/rule pairs; preserve prior results."""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[2]
BENCHMARK = Path(__file__).resolve().parent
DEFAULT_OUTPUT = ROOT / "eval/rustsec20"
HISTORY = [DEFAULT_OUTPUT, ROOT / "eval/rustsec", ROOT / "eval/rustsec-supplement",
           ROOT / "eval/rustsec-supplement2"]

# crate -> {MIRScan callsite id: relevant rule ids}. Locations refer to the
# original pinned sources, not injected copies. Advisory evidence is under
# evidence/<RUSTSEC id>/advisory.md. Never fall back to scanning an entire crate.
ADVISORY_TARGETS = {
    # RUSTSEC-2025-0039: AnonVec::get_ref, pointer arithmetic outside allocation.
    "anon-vec-0.1.1": {"src-vec-rs-206-20": ("rule-301",)},
    # RUSTSEC-2021-0027: Block::load, underflow makes set_len exceed capacity.
    "bam-0.1.2": {"src-bgzip-mod-rs-328-13": ("rule-961",)},
    # RUSTSEC-2025-0109: binary_write_from_ref / binary_read_to_ref.
    "binary_vec_io-0.1.12": {
        "src-lib-rs-31-26": ("rule-610",),
        "src-lib-rs-51-19": ("rule-447",),
        "src-lib-rs-52-34": ("rule-586",),
    },
    # RUSTSEC-2025-0107: any_as_u8_slice exposes uninitialized padding.
    "borrowck_sacrifices-0.1.0": {"src-unsafe_casts-rs-27-5": ("rule-611",)},
    # RUSTSEC-2021-0032: array read_bytes_default_le creates uninitialized T.
    "byte_struct-0.6.0": {"src-lib-rs-415-30": ("rule-192",)},
    # RUSTSEC-2021-0121: Chacha20 -> xor_si512_inplace, u8 -> u32 alignment.
    "crypto2-0.1.2": {"src-streamcipher-chacha20-rs-17-18": ("rule-585",)},
    # RUSTSEC-2025-0105: create_ring_buffer. The frozen DSL models only
    # set_len's capacity condition, not this advisory's initialization condition.
    "direct_ring_buffer-0.2.1": {"src-lib-rs-459-22": ("rule-961",)},
    # RUSTSEC-2022-0079: ElfGen::section_header_raw, unchecked ELF offset/size.
    "elf_rs-0.2.0": {
        "src-elf-elf-rs-88-26": ("rule-301",),
        "src-elf-elf-rs-89-13": ("rule-610",),
    },
    # RUSTSEC-2025-0063: SyncVec::get_uncheck (advisory calls it get_unchecked).
    "fast-able-1.11.17": {"src-vec-rs-184-18": ("rule-572",)},
    # RUSTSEC-2025-0034: FastMap::get.
    "fast_id_map-0.1.0": {"src-fast_map-rs-22-18": ("rule-576",)},
    # RUSTSEC-2021-0068: Decoder::new calls get_unchecked(data.len()).
    "iced-x86-1.10.3": {"src-decoder-mod-rs-458-42": ("rule-575",)},
    # RUSTSEC-2020-0026: LinkedHashMap::into_iter creates uninitialized HashMap.
    "linked-hash-map-0.5.2": {"src-lib-rs-1137-29": ("rule-192",)},
    # RUSTSEC-2022-0067: compress creates an uninitialized usize array.
    "lzf-0.3.1": {"src-compress-rs-58-48": ("rule-192",)},
    # RUSTSEC-2023-0017: align_first; no Vec::from_raw_parts DSL in frozen set.
    "maligned-0.2.1": {"src-heap-rs-24-27": ()},
    # RUSTSEC-2025-0106: index_of_ptr on an empty slice.
    "orx-pinned-vec-3.20.0": {"src-utils-slice-rs-39-33": ("rule-301",)},
    # RUSTSEC-2023-0016: partial_sort -> make_heap -> adjust_heap OOB reads.
    "partial_sort-0.1.2": {
        "src-lib-rs-113-28": ("rule-572",),
        "src-lib-rs-113-57": ("rule-572",),
        "src-lib-rs-118-31": ("rule-572",),
    },
    # RUSTSEC-2020-0038: compact::Vec::push mishandles large capacity.
    "ordnung-0.0.1": {"src-compact-rs-63-17": ("rule-958",)},
    # RUSTSEC-2025-0137: reciprocal_mg10, unchecked lookup after debug_assert.
    "ruint-1.16.0": {"src-algorithms-div-reciprocal-rs-87-33": ("rule-572",)},
    # RUSTSEC-2018-0013: transmute Vec helpers swap length and capacity.
    "safe-transmute-0.10.0": {
        "src-pod-rs-243-14": (), "src-bool-rs-99-17": (),
        "src-to_bytes-rs-280-14": (),
    },
    # RUSTSEC-2025-0033: Match::ptr (also called by Match::get).
    "scanner-0.1.0": {"src-lib-rs-413-12": ("rule-289",)},
    # RUSTSEC-2025-0113: write_u64v_le's unchecked output slice access.
    "shaman-0.1.0": {"src-cryptoutil-rs-45-30": ("rule-576",)},
    # RUSTSEC-2025-0047: get_disjoint_mut. This is a built-in check, not a DSL id.
    "slab-0.4.10": {"src-lib-rs-836-28": ("__raw_ptr_deref__",)},
    # RUSTSEC-2020-0039: Slab::remove reads at offset(len), one past the last element.
    "simple-slab-0.3.2": {
        "src-lib-rs-93-29": ("rule-431",),
        "src-lib-rs-96-25": ("rule-321",),
    },
    # RUSTSEC-2018-0018: SmallVec::new creates uninitialized generic storage.
    "smallvec-0.6.12": {"lib-rs-409-49": ("rule-192",)},
    # RUSTSEC-2025-0031: ThreadStack::get's unchecked index.
    "tanton_engine-1.0.0": {"src-search-mod-rs-128-18": ("rule-576",)},
    # RUSTSEC-2021-0028: TooDee::insert_row trusts ExactSizeIterator::len.
    "toodee-0.2.4": {
        "src-toodee-rs-685-25": ("rule-447",),
        "src-toodee-rs-687-13": ("rule-955",),
    },
    # RUSTSEC-2023-0080: transpose -> small/block/segmented unchecked writes.
    "transpose-0.2.2": {
        "src-out_of_place-rs-23-14": ("rule-576",),
        "src-out_of_place-rs-39-14": ("rule-576",),
        "src-out_of_place-rs-58-18": ("rule-576",),
    },
    # RUSTSEC-2025-0018: HashTable::get_bucket's unchecked pointer offset.
    "xmas-elf-0.9.1": {"src-hash-rs-31-23": ("rule-289",)},
}

# These advisories require release-style assertions/overflow behavior.
RELEASE_ONLY = {"bam-0.1.2", "orx-pinned-vec-3.20.0", "partial_sort-0.1.2", "ruint-1.16.0",
                "tanton_engine-1.0.0", "transpose-0.2.2"}
# Minimal safe-API calls that make generic callers appear in LLVM IR. They are
# written into the .local work copy as a #[test] only; benchmark sources stay pristine.
INSTANTIATIONS = {
    "ordnung-0.0.1": "let mut v = crate::compact::Vec::<u8>::new(); v.push(1); let _ = v.pop();",
    "orx-pinned-vec-3.20.0": "let empty: &[u8] = &[]; let _ = crate::utils::slice::index_of_ptr(empty, empty.as_ptr());",
    "simple-slab-0.3.2": "let mut s = crate::Slab::<u8>::new(); s.insert(1); let _ = s.remove(0);",
    "toodee-0.2.4": "let mut t = crate::TooDee::<u8>::init(2, 1, 0u8); t.insert_row(0, vec![1u8, 2u8]);",
}
UNSUPPORTED = {
    "maligned-0.2.1": "frozen DSL has no Vec::from_raw_parts allocation-layout rule",
    "safe-transmute-0.10.0": "frozen DSL has no Vec::from_raw_parts length/capacity rule",
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


def previous_run(crate, roots, *, no_debug_assertions=False):
    """Reuse only finished invocations covering every selected callsite/rule.

    Older targeted/instantiated runs can qualify; not-in-ir is still a finished
    attempt, not evidence of coverage or an advisory hit.
    """
    for root in dict.fromkeys(roots):
        for path in sorted((root / crate / "ruklee").glob("*/run.json"), reverse=True):
            run = read_json(path)
            matrix = read_json(path.parent / "matrix.json")
            if no_debug_assertions and "--no-debug-assertions" not in run.get("command", []):
                continue
            if (run.get("status") == "completed" and run.get("exit_code") == 0
                    and isinstance(matrix.get("total"), int)
                    and matrix.get("completed") == matrix["total"]
                    and selected_pairs(crate) <= matrix_pairs(matrix)):
                return path.parent
    return None


def environment():
    # Reuse the existing experiment's compiler, Cargo shim and credentials.
    spec = importlib.util.spec_from_file_location("rustsec_runner", ROOT / "dev/run-rustsec.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.environment()


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
    return ["--callsites-file", str(calls_path), "--rule-dsl", str(dsl_path)]


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
    shutil.copytree(source, work, ignore=shutil.ignore_patterns("target", ".git"))
    # Reuse the reviewed dependency resolutions instead of resolving old broad
    # version requirements to incompatible modern dependencies.
    for provenance in (BENCHMARK / "evidence").glob("*/provenance.json"):
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
        manifest = tomllib.loads((work / "Cargo.toml").read_text())
        lib = work / manifest.get("lib", {}).get("path", "src/lib.rs")
        (lib.parent / "__rustsec_instantiations.rs").write_text(
            "#[test]\nfn instantiate_public_api() {\n    " + INSTANTIATIONS[source.name] + "\n}\n")
        declaration = "\n#[cfg(test)] mod __rustsec_instantiations;\n"
        lib.write_text(lib.read_text() + declaration)
    command = [str(ROOT / "x"), "verify", str(work),
               "--rustc", str(args.rustc), "--klee-bin", str(args.klee_bin),
               "--ir-output-dir", str(ROOT / ".local/rustsec-irs" / source.name),
               "--results-json", str(output / "matrix.json"),
               "--logs-dir", str(output / "logs"),
               "--artifacts-dir", str(output / "artifacts"),
               "--timeout-sec", str(args.timeout),
               "--test" if args.tests or source.name in INSTANTIATIONS else "--no-test",
               *selection_args]
    if args.skip_llm:
        command.append("--skip-llm-testcase")
    if args.no_debug_assertions or source.name in RELEASE_ONLY:
        command.append("--no-debug-assertions")
    record = {"command": command, "cwd": str(ROOT), "source": str(source),
              "selection": ADVISORY_TARGETS[source.name],
              "started_at": datetime.now(timezone.utc).isoformat(),
              "timeout_seconds": args.crate_timeout, "status": "running"}
    write_json(output / "run.json", record)
    start = time.monotonic()
    process = None
    try:
        with (output / "stdout.log").open("w") as stdout, (output / "stderr.log").open("w") as stderr:
            process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=stdout,
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
    parser.add_argument("--rerun", action="store_true", help="Ignore prior completed runs; write a new attempt")
    parser.add_argument("--dry-run", action="store_true", help="List decisions and exact target/rule pairs without running tools")
    parser.add_argument("--output-root", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--timeout", type=int, default=60, help="Per KLEE invocation timeout in seconds (default: 60)")
    parser.add_argument("--crate-timeout", type=int, default=300,
                        help="Whole crate timeout in seconds (default: 300); 0 means unlimited")
    parser.add_argument("--tests", action="store_true", help="Compile existing library tests to instantiate more generic functions")
    parser.add_argument("--skip-llm", action="store_true", help="Only run the symbolic stage; skip generated PoCs/replays")
    parser.add_argument("--no-debug-assertions", action="store_true", help="Use release-style assertion behavior")
    parser.add_argument("--advisory-only", action="store_true", help="Compatibility flag: advisory-only is now always enabled")
    parser.add_argument("--rustc", type=Path, default=ROOT / "tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc")
    parser.add_argument("--klee-bin", type=Path, default=ROOT / "tools/klee/build/bin/klee")
    args = parser.parse_args()
    if args.timeout <= 0 or args.crate_timeout < 0:
        parser.error("--timeout must be positive and --crate-timeout nonnegative")
    args.output_root = args.output_root.resolve()
    args.rustc = args.rustc.resolve()
    args.klee_bin = args.klee_bin.resolve()
    crates = []
    matched = set()
    for source in sorted(BENCHMARK.iterdir()):
        if not source.is_dir() or not (source / "Cargo.toml").is_file():
            continue
        package = tomllib.loads((source / "Cargo.toml").read_text()).get("package", {})
        names = {source.name, package.get("name")}
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
    if not args.dry_run:
        for binary in (args.rustc, args.klee_bin):
            if not os.access(binary, os.X_OK):
                parser.error(f"Missing executable: {binary}")
    # The experiment tools share metadata and injected directories. Serialize
    # both crates and simultaneous invocations of this batch entrypoint.
    lock_path = ROOT / ".local/rustsec-batch.lock"
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    with lock_path.open("a") as lock:
        if not args.dry_run:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError:
                parser.error("Another benchmark/rustsec/run.py is running")
        attempt = datetime.now(timezone.utc).strftime("batch-%Y%m%dT%H%M%S-%f")
        env = None
        skipped = failed = ran = unsupported = 0
        for index, source in enumerate(crates, 1):
            if args.dry_run:
                for target, rules in ADVISORY_TARGETS[source.name].items():
                    print(f"  {source.name}: {target} -> {', '.join(rules) or '(no DSL)'}")
            if source.name in UNSUPPORTED:
                print(f"[{index}/{len(crates)}] UNSUPPORTED {source.name}: {UNSUPPORTED[source.name]}", flush=True)
                unsupported += 1
                continue
            release = args.no_debug_assertions or source.name in RELEASE_ONLY
            old = None if args.rerun else previous_run(
                source.name, [args.output_root, *HISTORY], no_debug_assertions=release)
            if old is not None:
                print(f"[{index}/{len(crates)}] SKIP {source.name}: {old}", flush=True)
                skipped += 1
                continue
            print(f"[{index}/{len(crates)}] RUN  {source.name} "
                  f"targets={len(ADVISORY_TARGETS[source.name])} pairs={len(selected_pairs(source.name))}"
                  f"{' no-debug-assertions' if release else ''}", flush=True)
            if not args.dry_run:
                if env is None:
                    env = environment()
                if not run_crate(source, args, env, attempt):
                    failed += 1
            ran += 1
        print(f"{'Plan' if args.dry_run else 'Done'}: run={ran}, skipped={skipped}, failed={failed}, unsupported={unsupported}")
        return int(failed > 0)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\nInterrupted; logs preserved. The next run retries unfinished crates.", file=sys.stderr)
        sys.exit(130)
