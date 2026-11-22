"""Check normal-pipeline ZST-tail outcomes, constraints and skipped LLM stage."""
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
    assert len(baseline) == 20
    assert {row["status"] for row in baseline} == {"eval-error"}
    rows = matrix(ROOT / ".local/verify/dst-zero-tail/full-matrix-results.json")
    expected = {}
    for caller in ("valid_tail", "empty_tail", "invalid_prefix", "uninitialized_prefix"):
        for rule in ("zst-valid", "zst-initialized", "zst-size", "zst-align", "zst-wrong-size"):
            sat = (rule in {"zst-initialized", "zst-wrong-size"} or
                   rule == "zst-valid" and caller in {"invalid_prefix", "uninitialized_prefix"})
            expected[caller, rule] = "low-confidence-sat" if sat else "verified"
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
        pair = json.loads(log.with_name("result.json").read_text())
        assert pair["result"]["llm_status"] in (None, "skipped"), log
    previous = ROOT / ".local/klee-maintenance/pass-20261119T0840"
    regressions = {}
    for case, old in (("dst-validity", previous / "dst-validity-replay.tar.gz"),
                      ("dst-layout", previous / "dst-layout-replay.tar.gz")):
        import tarfile
        with tarfile.open(old) as archive:
            data = json.load(archive.extractfile(f".local/verify/{case}/full-matrix-results.json"))
        current = matrix(ROOT / f".local/verify/{case}/full-matrix-results.json")
        key = lambda r: (r["caller"], r["rule"], r["status"])
        assert sorted(map(key, current)) == sorted(map(key, data["results"])), case
        regressions[case] = dict(Counter(r["status"] for r in current))
    report = {"baseline": dict(Counter(r["status"] for r in baseline)),
              "zst_tail": dict(Counter(r["status"] for r in rows)),
              "regressions": regressions, "all_expected_outcomes_match": True,
              "limitations": ["SAT observations remain unconfirmed",
                              "only complete empty-structure ZST elements are newly supported",
                              "other zero-size/absent metadata remains unsupported",
                              "vtable DSTs, allocation provenance/contracts remain incomplete"]}
    allocation = matrix(ROOT / ".local/verify/symbolic-allocation/full-matrix-results.json")
    expected_allocation = {
        (caller, rule): ("eval-error" if caller == "many_sizes" else
                         "low-confidence-sat" if caller == "two_sizes_uninitialized"
                         and rule == "rule-322-initialized" else "verified")
        for caller in ("many_sizes", "two_sizes", "two_sizes_uninitialized")
        for rule in ("allocation-align", "rule-322-initialized")}
    assert {(r["caller"], r["rule"]): r["status"] for r in allocation} == expected_allocation
    allocation_baseline = matrix(evidence / "alloc-baseline-results/full-matrix-results.json")
    assert any(r["caller"] == "many_sizes" and r["status"] == "klee-error"
               for r in allocation_baseline)
    for row in allocation:
        log = ROOT / row["log"].removeprefix("/unsat-rust/")
        text = log.read_text()
        assert "Stack dump:" not in text and "#0 " not in text, log
        if row["caller"] == "many_sizes":
            assert "execution model: concretized symbolic size" in text, log
        else:
            assert "[ext.dsl] resolved constraint expr:" in text, log
        pair = json.loads(log.with_name("result.json").read_text())
        assert pair["result"]["llm_status"] in (None, "skipped"), log
    report["allocation"] = dict(Counter(r["status"] for r in allocation))
    (evidence / "validation.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
