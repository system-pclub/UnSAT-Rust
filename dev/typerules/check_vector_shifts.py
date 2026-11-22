"""Check normal-pipeline vector shift shadows against the retained baseline."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    rule = json.loads((ROOT / "dev/typerules/vector-shifts-rules.json").read_text())["rules"]["rule-322-initialized"]
    names = ("logical_initialized_half", "logical_uninitialized_half",
             "left_initialized_fill", "left_uninitialized_half",
             "arithmetic_initialized_fill", "arithmetic_uninitialized_sign")
    report = {}
    for label in ("baseline", "fixed"):
        folder = evidence / (label + "-results")
        matrix = json.loads((folder / "matrix.json").read_text())
        callers = names + (("arithmetic_initialized_second_lane",
                            "arithmetic_uninitialized_second_lane") if label == "fixed" else ())
        assert matrix["total"] == matrix["completed"] == len(callers), matrix
        assert matrix["ablation_level"] == "full"
        actual = {row["caller"]: row["status"] for row in matrix["results"]}
        expected = {name: "low-confidence-sat" if label == "baseline" or "uninitialized" in name else "verified"
                    for name in callers}
        assert actual == expected, (label, actual, expected)
        for row in matrix["results"]:
            assert row["rule"] == "rule-322-initialized"
            pair = folder / row["callsite"] / row["rule"]
            result = json.loads((pair / "result.json").read_text())
            assert result["rule"]["dsl"] == rule["dsl"]
            assert result["rule"]["rule"] == rule["rule"]
            assert result["result"]["llm_status"] in (None, "skipped")
            assert not result["stages"].get("llm") and not result["stages"].get("rerun")
            log = (pair / "klee.log").read_text()
            assert "[ext.dsl] resolved constraint expr:" in log, pair
            assert "Stack dump:" not in log and "Assertion `" not in log, pair
        assembly = "\n".join(p.read_text() for p in folder.glob("*/rule-322-initialized/klee-out/assembly.ll"))
        for opcode in ("lshr <4 x i32>", "shl <4 x i32>", "ashr <4 x i32>"):
            assert opcode in assembly, (label, opcode)
        report[label] = {"counts": dict(Counter(actual.values())), "callers": actual}
    report.update({"rule": "rule-322-initialized", "dsl": rule["dsl"], "original_rule": rule["rule"],
                   "llm_verification_requests": 0,
                   "limitations": ["Synthetic byte-initialization observation, not ptr::read's API contract.",
                                   "Symbolic/uninitialized shift amounts and other vector element widths lack source coverage here.",
                                   "Allocation origin/contracts, nested constant shadows and scalar snapshot roots remain incomplete."]})
    (evidence / "vector-shifts-validation.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
