"""Check concrete-width and bounded-symbolic scalar initialization observations."""
import json
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    folder = evidence / "results"
    matrix = json.loads((folder / "matrix.json").read_text())
    rule = json.loads((ROOT / "dev/typerules/shift-initialization-rules.json").read_text())["rules"]["rule-322-initialized"]
    names = ("wide_initialized_half", "wide_uninitialized_half",
             "narrow_initialized_fill", "narrow_uninitialized_bits",
             "symbolic_initialized_half", "symbolic_uninitialized_half")
    expected = {name: "low-confidence-sat" if "uninitialized" in name else "verified" for name in names}
    # The constrained symbolic amount retains a certain-symbol path; this is
    # still a synthetic observation violation, without native replay or an LLM.
    expected["symbolic_uninitialized_half"] = "violation"
    actual = {row["caller"]: row["status"] for row in matrix["results"]}
    assert matrix["total"] == matrix["completed"] == 6, matrix
    assert matrix["ablation_level"] == "full"
    assert actual == expected, (actual, expected)
    constraints = {}
    for row in matrix["results"]:
        assert row["rule"] == "rule-322-initialized"
        pair = folder / row["callsite"] / row["rule"]
        result = json.loads((pair / "result.json").read_text())
        assert result["rule"]["dsl"] == rule["dsl"]
        assert result["rule"]["rule"] == rule["rule"]
        assert result["result"]["llm_status"] in (None, "skipped")
        assert not result["stages"].get("llm") and not result["stages"].get("rerun")
        log = (pair / "klee.log").read_text()
        constraints[row["caller"]] = [line for line in log.splitlines() if "[ext.dsl] resolved constraint expr:" in line]
        assert constraints[row["caller"]], pair
        assert "Stack dump:" not in log and "Assertion `" not in log, pair
    assembly = next(folder.glob("*/rule-322-initialized/klee-out/assembly.ll")).read_text()
    for instruction in (r"lshr i64 .*?, 32", r"lshr i8 .*?, 7", r"lshr i32 %[^,]+, %"):
        assert re.search(instruction, assembly), instruction
    report = {"outcomes": actual, "counts": dict(Counter(actual.values())),
              "rule": "rule-322-initialized", "dsl": rule["dsl"],
              "original_rule": rule["rule"], "constraints": constraints,
              "llm_verification_requests": 0,
              "limitations": "Scalar logical right shifts only; allocation provenance/contracts, missing amounts and general initialization remain incomplete."}
    (evidence / "shift-widths-validation.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
