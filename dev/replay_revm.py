"""Replay concrete REVM inputs against their original callsite/DSL pairs.

Run from the repository root with PYTHONPATH=tools/x/src. The input harness
is dev/revm_formal_replay.rs, compiled by the injected crate's
unsat-revm-formal-replay feature. Original full-matrix evidence is retained.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
import hashlib
import json
import os
import shutil
from pathlib import Path
import subprocess
import time

from cli.cmd.compare import _load_operator_entries
from cli.cmd.verify import (
    _analyze_rerun_output,
    _load_rule_dsl,
    _task1_to_ext_ast_json,
)
from cli.cmd.llvmir import ensure_linked_llvm_ir_file
from cli.cmd.verify_poc import ensure_cargo_feature


CASES = [
    ("dup", "src-interpreter-stack-rs-242-17", "ruklee-intrinsics-copy-nonoverlapping-dst-valid"),
    ("dup", "src-interpreter-stack-rs-243-17", "ruklee-vec-set-len-capacity-rustc-1_87-line1832"),
    ("push_slice", "src-interpreter-stack-rs-337-17-write", "rule-389"),
    ("push_slice", "src-interpreter-stack-rs-346-17", "rule-447"),
    ("push_slice", "src-interpreter-stack-rs-346-17-write_bytes", "ruklee-mut-ptr-write-bytes-valid-rustc-1_87-line1516"),
    ("push_slice", "src-interpreter-stack-rs-346-17-write_bytes", "rule-950"),
    ("dup", "src-interpreter-stack-rs-242-17", "rule-959"),
    ("exchange", "src-interpreter-stack-rs-283-13", "rule-309"),
    ("rjump", "src-instructions-control-rs-14-48", "rule-289"),
    ("rjumpi", "src-instructions-control-rs-28-48", "rule-289"),
    ("rjumpv", "src-instructions-control-rs-53-48", "rule-289"),
    ("program_counter", "src-interpreter-rs-343-18", "rule-294-rustc-1_87-line637"),
    ("current_opcode", "src-interpreter-rs-311-18", "__raw_ptr_deref__"),
    ("step", "src-interpreter-rs-355-31", "__raw_ptr_deref__"),
    ("load_eof_code", "src-interpreter-rs-135-45", "rule-301-rustc-1_87-line880"),
    ("slice_range", "src-interpreter-shared_memory-rs-159-21", "rule-89"),
    ("slice_mut", "src-interpreter-shared_memory-rs-174-21", "rule-89"),
    ("stack_push", "src-interpreter-stack-rs-204-9", "rule-89"),
    ("codesize", "src-instructions-system-rs-35-5", "rule-89"),
    ("codecopy", "src-instructions-system-rs-51-5", "rule-89"),
    ("instruction_push", "src-instructions-stack-rs-32-30", "rule-608"),
]


def prepare_injected_crate(root):
    """Install the version-pinned fixture, retaining original macro guards."""
    crate = root / "crates_inj/revm-interpreter-10.0.3"
    ensure_cargo_feature(crate, "unsat-revm-formal-replay")
    lib = crate / "src/lib.rs"
    text = lib.read_text()
    if 'mod formal_replay;' not in text:
        lib.write_text(text + '\n#[cfg(feature = "unsat-revm-formal-replay")]\n#[path = "../../../dev/revm_formal_replay.rs"]\nmod formal_replay;\n')
    macros = crate / "src/macros.rs"
    text = macros.read_text()
    if "macro_rules! debug_unreachable_at" not in text:
        macros.write_text(text + '''
macro_rules! debug_unreachable_at {
    ($site:literal, $($t:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($t)*);
        } else {
            klee_ext_bind::callsite!($site);
            unsafe { core::hint::unreachable_unchecked() };
        }
    };
}
macro_rules! assume_at {
    ($site:literal, $e:expr $(,)?) => {
        if !$e {
            debug_unreachable_at!($site, stringify!($e));
        }
    };
}
''')
    replacements = {
        "src/interpreter/stack.rs": [
            ('assume!(self.data.capacity() == STACK_LIMIT);', 'assume_at!("src-interpreter-stack-rs-204-9", self.data.capacity() == STACK_LIMIT);'),
        ],
        "src/interpreter/shared_memory.rs": [
            ('debug_unreachable!("slice OOB: {start}..{end}; len: {}", self.len())', 'debug_unreachable_at!("src-interpreter-shared_memory-rs-159-21", "slice OOB: {start}..{end}; len: {}", self.len())'),
            ('debug_unreachable!("slice OOB: {offset}..{end}")', 'debug_unreachable_at!("src-interpreter-shared_memory-rs-174-21", "slice OOB: {offset}..{end}")'),
        ],
    }
    for file, pairs in replacements.items():
        path = crate / file
        text = path.read_text()
        for before, after in pairs:
            if after not in text:
                if text.count(before) != 1:
                    raise RuntimeError(f"Fixture no longer matches {path}: {before}")
                text = text.replace(before, after)
        path.write_text(text)
    system = crate / "src/instructions/system.rs"
    text = system.read_text()
    for site in ("src-instructions-system-rs-35-5", "src-instructions-system-rs-51-5"):
        after = f'assume_at!("{site}", !interpreter.contract.bytecode.is_eof());'
        if after not in text:
            import re
            text, count = re.subn(r'assume!\(\s*!\s*interpreter\.contract\.bytecode\.is_eof\(\)\s*\);', after, text, count=1)
            if count != 1:
                raise RuntimeError(f"Fixture no longer matches {system}: {site}")
    system.write_text(text)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("eval/revm-formal-replay"))
    parser.add_argument("--only", nargs="*")
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--timeout", type=int, default=120)
    parser.add_argument("--run-label", default=datetime.now(timezone.utc).strftime("run-%Y%m%dT%H%M%S"))
    parser.add_argument("--klee-bin", type=Path, default=Path("tools/klee/build/bin/klee"))
    parser.add_argument("--build", action="store_true")
    args = parser.parse_args()
    root = Path.cwd()
    output = args.output.resolve()
    ir = output / "ir/revm_interpreter.ll"
    if args.build:
        prepare_injected_crate(root)
        ensure_linked_llvm_ir_file(
            cargo_dir=root / "crates_inj/revm-interpreter-10.0.3",
            output_dir=output / "ir",
            rustc=str(root / "tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"),
            test=False, build_std=True, panic_abort=True, debug_assertions=False,
            force=True, features=["unsat-revm-formal-replay"],
            build_log_path=output / "build.log",
        )
    report = root / "crates_inj/revm-interpreter-10.0.3/report.json"
    operators = _load_operator_entries(root)
    harness = root / "dev/revm_formal_replay.rs"
    source_hash = hashlib.sha256(harness.read_bytes()).hexdigest()
    run_output = output / args.run_label
    run_output.mkdir(parents=True, exist_ok=False)
    shutil.copy2(harness, run_output / "harness.rs")

    def run(case):
        entry, callsite, rule = case
        directory = run_output / callsite / rule
        directory.mkdir(parents=True, exist_ok=False)
        raw = rule == "__raw_ptr_deref__"
        original = root / "eval/ruklee-full/revm-interpreter-10.0.3" / callsite / rule / "result.json"
        original_pair = json.loads(original.read_text()) if original.is_file() else None
        dsl = None if raw else (
            original_pair["rule"]["dsl"] if original_pair else
            _load_rule_dsl(root / "ptr_rule_dsl.json", rule)
        )
        ast = "" if raw else _task1_to_ext_ast_json(dsl, operators)
        cmd = [
            str(args.klee_bin.resolve()),
            "--output-stats=false", "--output-istats=false",
            f"--output-dir={directory / 'klee-out'}",
            f"--entry-point=__revm_replay_{entry}",
            f"--ext.callsite={callsite}", "--compose-rerun",
            "--ext.raw-ptr-deref" if raw else f"--ext.dsl={ast}",
            f"--report-json={report}", str(ir),
        ]
        started = time.monotonic()
        timed_out = False
        try:
            process = subprocess.run(
                cmd, capture_output=True, text=True, timeout=args.timeout,
                env={**os.environ, "KLEE_RUNTIME_LIBRARY_PATH": str(root / "tools/klee/build/runtime/lib")},
            )
            code, stdout, stderr = process.returncode, process.stdout, process.stderr
        except subprocess.TimeoutExpired as exc:
            timed_out = True
            code = 124
            stdout = (exc.stdout or b"").decode("utf-8", "replace")
            stderr = (exc.stderr or b"").decode("utf-8", "replace")
        text = stdout + stderr
        result = _analyze_rerun_output(
            returncode=code, text=text, timed_out=timed_out, callsite_id=callsite
        )
        result["duration_sec"] = round(time.monotonic() - started, 3)
        record = {
            "entry": f"__revm_replay_{entry}", "callsite": callsite,
            "rule": rule, "dsl": dsl, "dsl_ast": ast,
            "command": cmd, "result": result,
            "harness": str(harness), "harness_sha256": source_hash,
            "harness_snapshot": str(run_output / "harness.rs"),
            "original_pair": str(original) if original_pair else None,
            "executed_at": datetime.now(timezone.utc).isoformat(),
            "log": str(directory / "klee.log"),
        }
        (directory / "klee.log").write_text(
            "command: " + repr(cmd) + "\n\n" + text, encoding="utf-8"
        )
        (directory / "result.json").write_text(json.dumps(record, indent=2) + "\n")
        print(entry, rule, result["status"], flush=True)
        return record

    cases = [c for c in CASES if not args.only or c[0] in args.only]
    records = []
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [pool.submit(run, case) for case in cases]
        for future in as_completed(futures):
            records.append(future.result())
            (run_output / "results.json").write_text(json.dumps(records, indent=2) + "\n")


if __name__ == "__main__":
    main()
