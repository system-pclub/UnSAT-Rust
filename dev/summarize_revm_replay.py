"""Publish evidence from the REVM concrete replay without replacing old runs."""

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CRATE = "revm-interpreter-10.0.3"
RUN_ROOT = ROOT / "eval/revm-formal-replay/release"
DESTINATION = ROOT / "eval/ruklee-full" / CRATE

WITNESSES = {
    "dup": "Via public data_mut(), replace the Vec with len=1, capacity=1; dup(1) requests new_len=2 and copies to the one-past pointer.",
    "push_slice": "Via public data_mut(), replace the Vec with len=1, capacity=1; push_slice(&[0xab]) writes a second U256 beyond its allocation.",
    "exchange": "Via public data_mut(), replace the Vec with len=1, capacity=1; exchange(usize::MAX,1) wraps n+m to zero and passes the length guard; its first swap pointer is one-past.",
    "rjump": "Public instruction_pointer points to the two-byte immediate [0x7f,0xff]; is_eof=true, sufficient gas; offset(32769) leaves this allocation.",
    "rjumpi": "As rjump, with a nonzero condition pushed through data_mut(), so the immediate contributes to the taken offset.",
    "rjumpv": "Public instruction_pointer points to [0,0x7f,0xff]; case=0, is_eof=true; the selected immediate sends the final pointer outside this allocation.",
    "program_counter": "Public instruction_pointer points to a separate local array while bytecode retains its own allocation; offset_from compares different allocations.",
    "current_opcode": "Public instruction_pointer is set to null through safe Rust; current_opcode reaches the exact raw-dereference marker with pointer=0.",
    "step": "Public instruction_pointer is null; crate-visible step reaches the exact raw-dereference marker with pointer=0.",
    "load_eof_code": "Construct an EOF with one STOP byte through Eof::default(); crate-visible load_eof_code(0,1024) computes a pointer outside the selected code section.",
    "slice_range": "SharedMemory::new() has length zero; slice_range(0..1) takes the None branch, reaching unreachable_unchecked when debug assertions are disabled.",
    "slice_mut": "SharedMemory::new() has length zero; slice_mut(0,1) takes the None branch, reaching unreachable_unchecked when debug assertions are disabled.",
    "stack_push": "Replace the escaped Vec through data_mut() with Vec::new(); capacity!=1024 reaches the unchecked assume branch with debug assertions disabled.",
    "codesize": "Replace public contract.bytecode with a safely constructed Eof; sufficient gas reaches the unchecked !is_eof() assumption.",
    "codecopy": "Use an Eof contract, sufficient gas, shared memory, and stack [len=1,code_offset=0,memory_offset=0]; the nonzero copy reaches the unchecked !is_eof() assumption.",
    "instruction_push": "Set public instruction_pointer=null; push::<1> reaches from_raw_parts(data=0,len=1), violating the non-null rule before later slice reads.",
}


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path):
    return str(Path(path).relative_to(ROOT))


def main():
    by_identity = {}
    for run in ("fixed-klee", "source-api-dsl"):
        records = json.loads((RUN_ROOT / run / "results.json").read_text())
        for record in records:
            by_identity[(record["callsite"], record["rule"])] = record

    records = []
    for (callsite, rule), record in sorted(by_identity.items()):
        entry = record["entry"].removeprefix("__revm_replay_")
        original = record["original_pair"]
        if original:
            pair = json.loads(Path(original).read_text())
        else:
            candidates = sorted((DESTINATION / callsite).glob("*/result.json"))
            pair = json.loads(candidates[0].read_text()) if candidates else {}
        callsite_meta = pair.get("callsite", {})
        source = ROOT / "benchmark/ruklee" / CRATE / callsite_meta["path"]
        injected = ROOT / "crates_inj" / CRATE / callsite_meta["path"]
        reproduced = record["result"]["full_rerun_passed"]
        record.update({
            "report_id": f"{CRATE}/{callsite}/{rule}",
            "scoring_unit": "source/DSL obligation at exact callsite",
            "report_polarity": "violation",
            "caller": callsite_meta.get("caller"),
            "source": relative(source), "source_sha256": sha256(source),
            "injected_source": relative(injected), "injected_source_sha256": sha256(injected),
            "dsl_sha256": hashlib.sha256((record["dsl"] or "__raw_ptr_deref__").encode()).hexdigest(),
            "code_verdict": "VIOLATES" if reproduced else "UNRESOLVED",
            "review_status": "DECIDED" if reproduced else "UNRESOLVED",
            "witness": WITNESSES[entry],
            "reproduction_executed": True,
            "environment": {
                "rustc": "1.87.0-dev", "target": "x86_64-unknown-linux-gnu",
                "debug_assertions": False, "overflow_checks": False,
                "panic": "abort", "klee_mode": "compose-rerun",
                "unsafe_calls_in_harness": False,
                "rule_89_instrumentation": "Marker placed only on the original unchecked macro branch; branch condition retained.",
                "execution_model": "KLEE compose-rerun; earlier out-of-range writes may be relaxed by the existing composition model. These are focused target-obligation replays, not native whole-program executions.",
            },
        })
        if entry in {"step", "load_eof_code"}:
            record["scope_note"] = "Caller is pub(crate); the harness is compiled inside the crate, matching the existing evaluation scope."
        if rule == "__raw_ptr_deref__":
            record["scope_note"] = (
                "The current raw-deref checker reports concrete marker reachability as SAT. "
                "Null-pointer invalidity is independently established from the supplied source input; "
                "the raw-deref SAT line alone is not a general memory-validity proof."
            )
        if rule == "ruklee-mut-ptr-write-bytes-valid-rustc-1_87-line1516":
            assert not reproduced, "Historical val/count mismatch unexpectedly reproduced"
            record["witness"] = (
                "Historical DSL uses get_arg(1), which is the constant fill byte 0, "
                "so it checks a zero-byte write. The current model correctly makes that "
                "condition true. Actual count=3 is get_arg(2); corrected canonical rule-950 "
                "is a separate specification and separate replay."
            )
            record["specification_status"] = "HISTORICAL_PARAMETER_BINDING_ERROR"
            record["code_verdict"] = "SATISFIES"
            record["review_status"] = "DECIDED"
        records.append(record)

    functions = []
    for entry in sorted(WITNESSES):
        selected = [r for r in records if r["entry"] == f"__revm_replay_{entry}"]
        functions.append({
            "entry": entry, "caller": selected[0]["caller"],
            "reproduced": any(r["result"]["full_rerun_passed"] for r in selected),
            "reports": [r["report_id"] for r in selected],
        })
    assert len({r["report_id"] for r in records}) == len(records)
    summary = {
        "scope": "16 REVM caller functions previously listed in eval/ruklee.md; selected obligations, not the complete 300-pair matrix",
        "functions_total": len(functions),
        "functions_reproduced": sum(f["reproduced"] for f in functions),
        "obligations_total": len(records),
        "obligations_reproduced": sum(r["result"]["full_rerun_passed"] for r in records),
        "historical_specification_mismatches": sum(r.get("specification_status") is not None for r in records),
        "functions": functions, "reports": records,
    }
    (DESTINATION / "formal-replay-results.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({k: v for k, v in summary.items() if k not in {"functions", "reports"}}, indent=2))


if __name__ == "__main__":
    main()
