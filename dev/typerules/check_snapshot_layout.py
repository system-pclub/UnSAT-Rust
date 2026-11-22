"""Check retained baseline and normal-pipeline snapshot layout results in Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    reports = {}
    for label, folder in [("baseline", evidence / "layout-baseline-results"),
                          ("fixed", ROOT / ".local/verify/snapshot-layout")]:
        matrix = json.loads((folder / "full-matrix-results.json").read_text())
        assert matrix["completed"] == matrix["total"] == 4, matrix
        actual = {r["caller"]: r["status"] for r in matrix["results"]}
        expected = {"sized_arc": "verified", "null_pointer": "low-confidence-sat",
                    "unsized_arc": "klee-error" if label == "baseline" else "eval-error",
                    "nested_unsized_arc": "verified"}
        assert actual == expected, (label, actual, expected)
        for row in matrix["results"]:
            pair = folder / row["callsite"] / row["rule"]
            result = json.loads((pair / "result.json").read_text())
            assert result["rule"]["dsl"] == "$1.$mem.pointee.start != 0"
            assert result["rule"]["rule"] == "`ptr` must be non-null."
            assert not result["stages"].get("llm") and not result["stages"].get("rerun")
            log = (pair / "klee.log").read_text()
            if row["caller"] == "unsized_arc":
                if label == "baseline":
                    assert row["returncode"] == -11
                    assert "ObjectState::setKnownSymbolic" in log
                else:
                    assert row["returncode"] == 0
                    assert "snapshot field layout exceeds static storage" in log
                    assert "failed to evaluate expression: snapshot model:" in log
                    assert "ObjectState::setKnownSymbolic" not in log
            else:
                assert "[ext.dsl] resolved constraint expr:" in log
        reports[label] = {"counts": dict(Counter(actual.values())), "callers": actual}
    # The constant fixture is an audit, not evidence that nested poison is fixed.
    folder = ROOT / ".local/verify/constant-initialization"
    matrix = json.loads((folder / "full-matrix-results.json").read_text())
    actual = {r["caller"]: r["status"] for r in matrix["results"]}
    assert actual == {"initialized_field": "verified", "initialized_pair": "verified",
                      "uninitialized_field": "low-confidence-sat",
                      "uninitialized_pair": "low-confidence-sat"}, actual
    reports["constant_audit"] = actual
    reports.update({"rule": "rule-504", "source_rule": "core:rule-843",
                    "dsl": "$1.$mem.pointee.start != 0",
                    "original_rule": "`ptr` must be non-null.",
                    "llm_verification_requests": 0,
                    "limitations": [
                        "Unsupported dynamic snapshot storage is rejected, not modeled.",
                        "Option<Arc<str>> does not exercise inner Arc shaping in the current niche model.",
                        "Extern-return materialization is outside the snapshot-entry preflight.",
                        "Nested undef/poison constant initialization remains incomplete."]})
    (evidence / "snapshot-validation.json").write_text(json.dumps(reports, indent=2) + "\n")
    print(json.dumps(reports, indent=2))


if __name__ == "__main__":
    main()
