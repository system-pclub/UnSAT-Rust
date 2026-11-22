"""Check retained normal-pipeline SIMD select evidence inside Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    expected = {
        "selected_initialized": "verified",
        "unselected_uninitialized": "verified",
        "selected_uninitialized": "low-confidence-sat",
        "initialized_mask_lane": "verified",
        "uninitialized_mask": "low-confidence-sat",
        "lane_value": "verified",
        "lane_value_checked": "verified",
        "lane_value_other_arm": "verified",
        "lane_value_violation": "low-confidence-sat",
    }
    rules = json.loads((ROOT / "dev/typerules/vector-select-rules.json").read_text())["rules"]
    report = {"rules": {k: {"dsl": r["dsl"], "original_rule": r["rule"]}
                        for k, r in rules.items()}, "llm_requests": 0}
    for label in ("baseline", "fixed"):
        folder = evidence / f"{label}-results"
        matrix = json.loads((folder / "matrix.json").read_text())
        wanted = expected if label == "fixed" else {k: "klee-error" for k in list(expected)[:6]}
        actual = {r["caller"]: r["status"] for r in matrix["results"]}
        assert matrix["completed"] == matrix["total"] == len(wanted)
        assert actual == wanted, (label, actual, wanted)
        assembly = ""
        for row in matrix["results"]:
            pair = folder / row["callsite"] / row["rule"]
            data = json.loads((pair / "result.json").read_text())
            rule = rules[row["rule"]]
            assert data["rule"]["dsl"] == rule["dsl"]
            assert data["rule"]["rule"] == rule["rule"]
            assert data["stages"]["init"]["ablation_level"] == "full"
            assert not data["stages"].get("llm") and not data["stages"].get("rerun")
            log = (pair / "klee.log").read_text()
            if label == "baseline":
                assert row["returncode"] == -6
                assert 'c->getWidth()==Bool' in log and "SelectExpr::create" in log
            else:
                assert row["returncode"] == 0
                assert "[ext.dsl] resolved constraint expr:" in log
                assert "failed to evaluate expression" not in log
            assembly += (pair / "klee-out/assembly.ll").read_text()
        for instruction in ("icmp slt <4 x i32>", "sext <4 x i1>",
                            "lshr <4 x i32>", "trunc <4 x i32>", "select <4 x i1>"):
            assert instruction in assembly, instruction
        report[label] = {"counts": dict(Counter(actual.values())), "callers": actual}
    report["limitations"] = [
        "Synthetic SAT observations are unconfirmed, never confirmed vulnerabilities.",
        "Blend exercises signed-less-than, sext, lshr, trunc and vector-condition select.",
        "Other ICmp predicates, shifts, zext and scalar-condition vector select lack end-to-end coverage here.",
        "General vector arithmetic, nested undef constants and scalar snapshot roots remain incomplete.",
    ]
    (evidence / "vector-select-validation.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
