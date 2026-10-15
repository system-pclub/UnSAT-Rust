"""End-to-end check of autoinj's unreachable model at LLVM O0 and O3.

The emitted input uses the real klee-ext-bind shim. No modified KLEE binary
or manual macro expansion is needed. Run with PYTHONPATH=tools/x/src.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import subprocess

from cli.cmd.verify import _analyze_rerun_output, _matrix_callsite_rows
from cli.cmd.llvmir import ensure_linked_llvm_ir_file


ROOT = Path(__file__).resolve().parents[1]
RUSTC = ROOT / "tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"


def command(cmd, log, **kwargs):
    process = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, **kwargs)
    log.write_text(repr(cmd) + "\n" + process.stdout + process.stderr)
    if process.returncode:
        raise RuntimeError(f"Command failed ({process.returncode}): {log}")
    return process


def check_real_crates(output, selected_crate=None, resume=False):
    plans = [
        ("revm-interpreter-10.0.3", [
            ("src-interpreter-shared_memory-rs-159-21", "__revm_replay_slice_range"),
            ("src-interpreter-shared_memory-rs-174-21", "__revm_replay_slice_mut"),
            ("src-interpreter-stack-rs-204-9", "__revm_replay_stack_push"),
            ("src-instructions-system-rs-35-5", "__revm_replay_codesize"),
            ("src-instructions-system-rs-51-5", "__revm_replay_codecopy"),
        ]),
        ("arrow2-0.18.0", [("src-offset-rs-409-30", "__arrow_first_default")]),
    ]
    records = []
    for crate_name, cases in plans:
        if selected_crate and selected_crate != crate_name:
            continue
        directory = output / "real" / crate_name
        directory.mkdir(parents=True, exist_ok=resume)
        ids = {site for site, _ in cases}
        metadata = json.loads((ROOT / "crates" / f"{crate_name}.json").read_text())
        metadata["report"]["targets"] = [t for t in metadata["report"]["targets"] if t["callsite"]["id"] in ids]
        assert len(metadata["report"]["targets"]) == len(cases)
        meta_path = directory / "metadata.json"
        meta_path.write_text(json.dumps(metadata, indent=2))
        report_path = directory / "report.json"
        report_path.write_text(json.dumps(metadata["report"], indent=2))
        injected = directory / "injected"
        if not (resume and injected.exists()):
            command([ROOT / "tools/autoinj/target/release/autoinj", ROOT / "benchmark/ruklee" / crate_name, meta_path, injected], directory / "injection.log", cwd=ROOT)
        if crate_name.startswith("revm-") and not resume:
            shutil.copy2(ROOT / "dev/revm_formal_replay.rs", injected / "src/formal_replay.rs")
            lib = injected / "src/lib.rs"
            lib.write_text(lib.read_text() + "\nmod formal_replay;\n")
        elif not resume:
            lib = injected / "src/lib.rs"
            lib.write_text(lib.read_text() + '''
#[no_mangle]
#[allow(missing_docs)]
pub extern "C" fn __arrow_first_default() -> i32 {
    let offsets = crate::offset::OffsetsBuffer::<i32>::new();
    *offsets.first()
}
''')
        ir = ensure_linked_llvm_ir_file(
            cargo_dir=injected, output_dir=directory / "ir", rustc=str(RUSTC),
            test=False, build_std=True, panic_abort=True, debug_assertions=False,
            force=True, build_log_path=directory / "build.log",
        )
        rows = _matrix_callsite_rows(targets=metadata["report"]["targets"], ll_path=ir, requested_callsite=None, requested_callsites=None)
        (directory / "callsite-presence.json").write_text(json.dumps(rows, indent=2))
        assert all(r["present_in_llvm_ir"] for r in rows), rows
        for site, entry in cases:
            run_dir = directory / site
            run_dir.mkdir()
            cmd = [ROOT / "tools/klee/build/bin/klee", "--output-stats=false", "--output-istats=false", f"--output-dir={run_dir / 'klee-out'}", "--compose-rerun", f"--entry-point={entry}", f"--ext.callsite={site}", '--ext.dsl={"type":"literal","value":false}', f"--report-json={report_path}", ir]
            process = command(cmd, run_dir / "klee.log", timeout=120, env={**os.environ, "KLEE_RUNTIME_LIBRARY_PATH": str(ROOT / "tools/klee/build/runtime/lib")})
            result = _analyze_rerun_output(returncode=process.returncode, text=process.stdout + process.stderr, timed_out=False, callsite_id=site)
            expected = crate_name.startswith("revm-")
            assert result["full_rerun_passed"] == expected, (crate_name, site, result)
            records.append({"crate": crate_name, "callsite": site, "entry": entry, "present_in_llvm_ir": True, "expected_reachable": expected, "result": result, "log": str(run_dir / "klee.log")})
            (run_dir / "result.json").write_text(json.dumps(records[-1], indent=2))
            print(crate_name, site, "present in IR; replay", result["status"], flush=True)
    records = [json.loads(path.read_text()) for path in sorted((output / "real").glob("*/*/result.json"))]
    (output / "real-results.json").write_text(json.dumps(records, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--real", action="store_true", help="Also reinject original REVM and Arrow2 sources and check six real callsites.")
    parser.add_argument("--resume-real", action="store_true")
    parser.add_argument("--real-crate", choices=("revm-interpreter-10.0.3", "arrow2-0.18.0"))
    args = parser.parse_args()
    output = args.output or ROOT / ".local/unreachable-instrumentation" / datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S")
    output = output.resolve()
    if args.resume_real:
        check_real_crates(output, args.real_crate, resume=True)
        return
    output.mkdir(parents=True, exist_ok=False)
    original = ROOT / "tools/autoinj/tests/unreachable"
    source = (original / "src/lib.rs").read_text()
    targets = []
    for caller, needle in [("direct", "false => unsafe"), ("macro_guard", "assume!(flag)"), ("evaluate_once", "assume!({ count")]:
        number, line = next((n, line) for n, line in enumerate(source.splitlines(), 1) if needle in line)
        column = line.index("core::hint::unreachable_unchecked" if caller == "direct" else "assume!") + 1
        targets.append({
            "caller": {"name": caller, "path": "src/lib.rs", "require_template": False},
            "callee": {"name": "core::hint::unreachable_unchecked"},
            "callsite": {"path": "src/lib.rs", "line": number, "col": column},
        })
    metadata = output / "metadata.json"
    metadata.write_text(json.dumps({"report": {"targets": targets}}, indent=2))
    injected = output / "injected"
    command([ROOT / "tools/autoinj/target/release/autoinj", original, metadata, injected], output / "injection.log", cwd=ROOT)
    ids = {t["caller"]["name"]: f'src-lib-rs-{t["callsite"]["line"]}-{t["callsite"]["col"]}' for t in targets}
    records = []
    tasks = []
    for opt, debug in [(0, False), (3, False), (3, True)]:
        build = output / f"o{opt}-debug-{str(debug).lower()}"
        build.mkdir()
        flags = [f"-Copt-level={opt}", f"-Cdebug-assertions={'on' if debug else 'off'}", "-Cpanic=abort", "-Zinline-mir=no"]
        command([RUSTC, "--crate-name", "klee_ext_bind", "--crate-type", "lib", "--edition=2021", "--emit=link,llvm-ir", *flags, "--out-dir", build, ROOT / "tools/klee-ext-bind/src/lib.rs"], build / "runtime-build.log")
        fixture = build / "fixture.ll"
        command([RUSTC, "--crate-name", "fixture", "--crate-type", "lib", "--edition=2021", "--emit=llvm-ir", *flags, "--extern", f'klee_ext_bind={build / "libklee_ext_bind.rlib"}', "-o", fixture, injected / "src/lib.rs"], build / "fixture-build.log")
        linked = build / "linked.ll"
        command(["llvm-link-20", "-S", fixture, build / "klee_ext_bind.ll", "-o", linked], build / "link.log")
        ir = linked.read_text()
        assert ids["direct"] in ir, "Direct branch marker was erased"
        assert debug or ids["macro_guard"] in ir, "Release macro marker was erased"
        for entry, selected, expected in [
            ("direct_bad", "direct", True), ("direct_good", "direct", False),
            ("macro_bad", "macro_guard", not debug), ("macro_good", "macro_guard", False),
            ("prior_unreachable", "macro_guard", False),
        ]:
            tasks.append((build, linked, entry, ids[selected], expected))

    def check(task):
        build, linked, entry, site, expected = task
        directory = build / entry
        directory.mkdir()
        cmd = [ROOT / "tools/klee/build/bin/klee", "--output-stats=false", "--output-istats=false", f"--output-dir={directory / 'klee-out'}", "--compose-rerun", f"--entry-point={entry}", f"--ext.callsite={site}", '--ext.dsl={"type":"literal","value":false}', linked]
        process = command(cmd, directory / "klee.log", timeout=60, env={**os.environ, "KLEE_RUNTIME_LIBRARY_PATH": str(ROOT / "tools/klee/build/runtime/lib")})
        result = _analyze_rerun_output(returncode=process.returncode, text=process.stdout + process.stderr, timed_out=False, callsite_id=site)
        assert result["full_rerun_passed"] == expected, (entry, build, expected, result)
        record = {"configuration": build.name, "entry": entry, "callsite": site, "expected_reachable": expected, "result": result, "log": str(directory / "klee.log")}
        (directory / "result.json").write_text(json.dumps(record, indent=2))
        print(build.name, entry, "PASS", flush=True)
        return record

    with ThreadPoolExecutor(max_workers=2) as pool:
        records = list(pool.map(check, tasks))
    (output / "results.json").write_text(json.dumps(records, indent=2))
    print(f"{len(records)} checks passed: {output}")
    if args.real:
        check_real_crates(output, args.real_crate)


if __name__ == "__main__":
    main()
