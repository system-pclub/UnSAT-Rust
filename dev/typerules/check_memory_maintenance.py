"""Check source regressions for scalar initialization, DST layout and crashes.

Run inside the repository image after the documented normal verify commands.
Low-confidence SAT and explicit backend errors remain unconfirmed/unproved.
"""
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def rows(name):
    path = ROOT / ".local/verify" / name / "full-matrix-results.json"
    document = json.loads(path.read_text())
    if document["completed"] != document["total"]:
        raise RuntimeError(f"Incomplete matrix: {name}")
    return document["results"]


def main():
    scalar = rows("scalar-initialization")
    wanted = {}
    for caller in ("uninitialized_scalar", "initialized_scalar", "uninitialized_set",
                   "initialized_low_bytes", "invalid_indirect", "valid_indirect",
                   "conditional_scalar", "padded_aggregate"):
        for rule in ("rule-320", "rule-321", "rule-322", "rule-322-initialized"):
            status = "verified"
            if caller == "invalid_indirect":
                status = "eval-error"
            elif rule in {"rule-322", "rule-322-initialized"}:
                if caller in {"uninitialized_scalar", "uninitialized_set", "conditional_scalar"}:
                    status = "low-confidence-sat"
                elif caller == "padded_aggregate" and rule == "rule-322-initialized":
                    status = "low-confidence-sat"
            wanted[caller, rule] = status
    actual = {(r["caller"], r["rule"]): r["status"] for r in scalar
              if r["rule"] != "__raw_ptr_deref__"}
    if actual != wanted:
        raise RuntimeError(f"Scalar shadow mismatch: {[(k, wanted.get(k), actual.get(k)) for k in wanted.keys() | actual.keys() if wanted.get(k) != actual.get(k)]}")
    sentinels = [r for r in scalar if r["rule"] == "__raw_ptr_deref__"]
    if any(r["status"] != "not-in-ir" and not (
            r["caller"] == "initialized_low_bytes" and r["status"] == "verified")
           for r in sentinels):
        raise RuntimeError("Unexpected additional raw-pointer sentinel outcome")

    dst = rows("dst-layout")
    expected_dst = {(caller, rule): ("low-confidence-sat" if caller == "four_bytes" and rule == "dst-three-bytes" else "verified")
                    for caller in ("three_bytes", "four_bytes")
                    for rule in ("dst-layout", "dst-three-bytes")}
    if {(r["caller"], r["rule"]): r["status"] for r in dst} != expected_dst:
        raise RuntimeError("DST layout mismatch")

    transfers = rows("memory-transfers")
    expected_transfer = {}
    for line in (ROOT / "dev/typerules/expected.txt").read_text().splitlines():
        case, caller, rule, status = line.split()
        if case == "memory-transfers:":
            expected_transfer[caller, rule] = status
    if {(r["caller"], r["rule"]): r["status"] for r in transfers} != expected_transfer:
        raise RuntimeError("Bulk transfer regression changed")

    for row in scalar + dst + transfers:
        if row["rule"] == "__raw_ptr_deref__":
            continue
        log = ROOT / row["log"].removeprefix("/unsat-rust/")
        text = log.read_text()
        if "Assertion `" in text or "Stack dump:" in text or "#0 " in text:
            raise RuntimeError(f"Backend crash: {log}")
        if row["caller"] == "invalid_indirect":
            if "execution reached an invalid function pointer" not in text:
                raise RuntimeError("Invalid call did not report its actual error")
        elif "[ext.dsl] resolved constraint expr:" not in text:
            raise RuntimeError(f"Missing generated constraints: {log}")

    reproduction = ROOT / ".local/klee-maintenance/first-pass/adbc-rule-315"
    result = json.loads((reproduction / "result.json").read_text())
    log = (reproduction / "klee-compose.log").read_text()
    if result["stages"]["init"]["returncode"] != 0 or "field 'inner' on a non-aggregate" in log:
        raise RuntimeError("Original crate still crashes or loses the CStr field")
    if "[ext.dsl.v2] $3.inner.$mem.size" not in log:
        raise RuntimeError("Original unchanged step2 expression was not evaluated")
    if result["result"]["llm_status"] != "skipped":
        raise RuntimeError("Unexpected LLM verification stage")
    python_log = (ROOT / ".local/maintenance-python-tests.log").read_text()
    if not re.search(r"Ran 60 tests.*?\nOK\n", python_log, re.S):
        raise RuntimeError("Python validation did not pass")
    all_rows = scalar + dst + transfers
    summary = {"expected_match": True, "pairs": len(all_rows),
               "statuses": dict(Counter(r["status"] for r in all_rows)),
               "python_tests": 60, "new_source_crates": ["scalar-initialization", "dst-layout"],
               "klee_sha256": hashlib.file_digest((ROOT / "tools/klee/build/bin/klee").open("rb"), "sha256").hexdigest(),
               "original_crate_reproduction": "adbc_ffi-0.23.0 / src-types-rs-457-40 / step2-core-rule-315: exit 0, original expression evaluated; candidate remains unconfirmed",
               "remaining": ["originating allocation generations and allocator request contracts",
                             "scalar snapshot-root initialization, intrinsic summaries, exact vector-lane and nested undef propagation",
                             "general DST typed validity and vtable-backed layout", "bodyless/erased targets and unbounded quantification"]}
    output = ROOT / ".local/klee-maintenance/first-pass/validation.json"
    output.write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
