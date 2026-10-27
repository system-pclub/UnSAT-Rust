#!/usr/bin/env python3
"""Run a fixed, local KLEE replay for selected RustSec coverage cases."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import sys


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "x" / "src"))

from cli.cmd.compare import _load_operator_entries  # noqa: E402
from cli.cmd.llvmir import ensure_linked_llvm_ir_file  # noqa: E402
from cli.cmd.verify import (  # noqa: E402
    _load_rule_entry,
    _rule_entry_to_ext_ast_json,
    _run_klee_compose_rerun,
)
from cli.cmd.verify_poc import (  # noqa: E402
    ensure_cargo_feature,
    inject_testcase_at_callsite,
    testcase_injection,
)


CASES = {
    "orx-pinned-vec-3.20.0": {
        "callsite": "src-utils-slice-rs-39-33",
        "rule": "rule-301",
        "body": """let empty: &[u8] = &[];
    let _ = index_of_ptr(empty, empty.as_ptr());""",
        "debug_assertions": False,
    },
    "tanton_engine-1.0.0": {
        "callsite": "src-search-mod-rs-128-18",
        "rule": "rule-576",
        "body": """let mut stack = ThreadStack::new();
    let _ = stack.get(usize::MAX);""",
        "debug_assertions": False,
    },
    "xmas-elf-0.9.1": {
        "callsite": "src-hash-rs-31-23",
        "rule": "rule-289",
        "body": """let table = HashTable {
        bucket_count: 4,
        chain_count: 0,
        first_bucket: 0,
    };
    let _ = table.get_bucket(3);""",
        "debug_assertions": False,
    },
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("crate", choices=sorted(CASES))
    parser.add_argument("--core-matrix", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    toolchain = ROOT / ".local" / "rustsec-toolchain" / "nightly-2025-03-28-x86_64-unknown-linux-gnu"
    os.environ["PATH"] = ":".join(
        [str(ROOT / ".local" / "rustsec-bin"), str(toolchain / "bin"), os.environ.get("PATH", "")]
    )
    os.environ["LD_LIBRARY_PATH"] = str(toolchain / "lib")
    os.environ["RUSTC_BOOTSTRAP"] = "1"
    os.environ["CARGO_BUILD_JOBS"] = "4"
    os.environ["KLEE_RUNTIME_LIBRARY_PATH"] = str(ROOT / "tools" / "klee" / "build" / "runtime" / "lib")

    case = CASES[args.crate]
    injected = ROOT / "crates_inj" / args.crate
    report_path = injected / "report.json"
    report = json.loads(report_path.read_text(encoding="utf-8"))
    target = next(
        target
        for target in report["targets"]
        if target.get("callsite", {}).get("id") == case["callsite"]
    )

    injection = testcase_injection(case["callsite"], case["rule"])
    testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    {case["body"]}
}}'''
    ensure_cargo_feature(injected, injection.feature)
    injection = inject_testcase_at_callsite(
        crate_dir=injected,
        target=target,
        testcase=testcase,
        injection=injection,
    )

    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    testcase_path = output / "fixed-testcase.rs"
    testcase_path.write_text(testcase + "\n", encoding="utf-8")
    ir_dir = output / "rerun-ir"
    build_log = output / "testcase-build.log"
    ll_path = ensure_linked_llvm_ir_file(
        cargo_dir=injected,
        output_dir=ir_dir,
        rustc=str(ROOT / "tools" / "rust" / "build" / "x86_64-unknown-linux-gnu" / "stage1" / "bin" / "rustc"),
        test=False,
        build_std=True,
        panic_abort=True,
        overflow_checks=False,
        debug_assertions=case["debug_assertions"],
        force=True,
        features=[injection.feature],
        build_log_path=build_log,
    )
    rule_entry = _load_rule_entry(ROOT / "ptr_rule_dsl.json", case["rule"])
    ast_json = _rule_entry_to_ext_ast_json(rule_entry, _load_operator_entries(ROOT))
    rerun = _run_klee_compose_rerun(
        ll_path=ll_path,
        callsite_id=case["callsite"],
        ast_json=ast_json,
        klee_bin=str(ROOT / "tools" / "klee" / "build" / "bin" / "klee"),
        output_dir=output / "klee-rerun",
        entry_function=injection.function,
        log_path=output / "klee-rerun.log",
        timeout_sec=240,
        report_json=report_path,
    )

    matrix = json.loads(args.core_matrix.read_text(encoding="utf-8"))
    row = next(
        row
        for row in matrix["results"]
        if row.get("callsite") == case["callsite"] and row.get("rule") == case["rule"]
    )
    row.update(
        {
            "testcase_status": "ok" if rerun["full_rerun_passed"] else "rerun-error",
            "testcase_origin": "fixed-local",
            "testcase_path": str(testcase_path),
            "rerun_reported_callsite": bool(rerun["reported_callsite"]),
            "rerun_dsl_sat": bool(rerun["dsl_sat"]),
            "rerun_has_certain_symbol": bool(rerun["has_certain_symbol"]),
            "full_rerun_passed": bool(rerun["full_rerun_passed"]),
            "rerun_log": str(output / "klee-rerun.log"),
        }
    )
    matrix["fixed_local_replay"] = {
        "callsite": case["callsite"],
        "rule": case["rule"],
        "testcase": str(testcase_path),
        "result": rerun,
    }
    (output / "matrix.json").write_text(
        json.dumps(matrix, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    shutil.copyfile(report_path, output / "report.json")
    print(json.dumps(rerun, sort_keys=True))
    return 0 if rerun["full_rerun_passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
