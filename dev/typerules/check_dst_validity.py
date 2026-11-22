"""Check retained normal-pipeline DST and sequence regressions inside Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def matrix(path):
    data = json.loads(path.read_text())
    assert data["completed"] == data["total"], path
    return data["results"]


def main():
    evidence = ROOT / sys.argv[1]
    baseline = matrix(evidence / "baseline-results/full-matrix-results.json")
    assert len(baseline) == 12
    assert {r["status"] for r in baseline if r["rule"] == "dst-valid"} == {"eval-error"}
    rows = matrix(ROOT / ".local/verify/dst-validity/full-matrix-results.json")
    expected = {}
    for caller in ("valid_tail", "invalid_tail", "empty_tail", "uninitialized_prefix",
                   "nested_valid", "nested_invalid"):
        for rule in ("dst-align", "dst-size", "dst-valid", "dst-initialized"):
            status = "verified"
            if rule == "dst-initialized" or (rule == "dst-valid" and caller in {
                    "invalid_tail", "nested_invalid", "uninitialized_prefix"}):
                status = "low-confidence-sat"
            expected[caller, rule] = status
    actual = {(r["caller"], r["rule"]): r["status"] for r in rows}
    assert actual == expected, [(k, expected.get(k), actual.get(k))
                                for k in expected.keys() | actual.keys()
                                if expected.get(k) != actual.get(k)]
    for row in rows:
        log = ROOT / row["log"].removeprefix("/unsat-rust/")
        text = log.read_text()
        assert "[ext.dsl] resolved constraint expr:" in text, log
        assert "failed to evaluate expression" not in text, log
        assert "Stack dump:" not in text, log
        result = json.loads(log.with_name("result.json").read_text())
        assert result["result"]["llm_status"] in (None, "skipped"), log

    seq = matrix(ROOT / ".local/verify/memory-state/full-matrix-results.json")
    expected_seq = {}
    for line in (ROOT / "dev/typerules/expected.txt").read_text().splitlines():
        case, caller, rule, status = line.split()
        if case == "memory-state:":
            expected_seq[caller, rule] = status
    assert {(r["caller"], r["rule"]): r["status"] for r in seq} == expected_seq
    layout = matrix(ROOT / ".local/verify/dst-layout/full-matrix-results.json")
    expected_layout = {(caller, rule): ("low-confidence-sat" if caller == "four_bytes"
                                       and rule == "dst-three-bytes" else "verified")
                       for caller in ("three_bytes", "four_bytes")
                       for rule in ("dst-layout", "dst-three-bytes")}
    assert {(r["caller"], r["rule"]): r["status"] for r in layout} == expected_layout
    report = {"dst_pairs": len(rows), "dst_statuses": dict(Counter(r["status"] for r in rows)),
              "baseline_dst_valid": "eval-error (3/3)", "sequence_pairs": len(seq),
              "sequence_statuses": dict(Counter(r["status"] for r in seq)),
              "layout_pairs": len(layout),
              "layout_statuses": dict(Counter(r["status"] for r in layout)),
              "all_expected_outcomes_match": True,
              "limitations": ["SAT observations are unconfirmed, with receiver uncertainty",
                              "vtable and zero-sized tail-element metadata unsupported",
                              "symbolic sequences still require a proven finite bound",
                              "allocation generation/contracts remain incomplete"]}
    (evidence / "validation.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
