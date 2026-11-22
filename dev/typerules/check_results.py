"""Check retained normal-pipeline results against reviewed source expectations.

Run in the repository image after the matrix commands in README.md. Solver
limits are reported separately; matching an expected limitation is not a proof.
"""
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).parent


def retained(case):
    path = ROOT / ".local/verify" / case / "full-matrix-results.json"
    document = json.loads(path.read_text())
    if document["completed"] != document["total"]:
        raise RuntimeError(f"Incomplete run: {case}")
    return document["results"]


def main():
    expected = {}
    for line in (HERE / "expected.txt").read_text().splitlines():
        case, caller, rule, status = line.split()
        key = (case.rstrip(":"), caller, rule)
        if key in expected:
            raise RuntimeError(f"Duplicate expectation: {key}")
        expected[key] = status
    rows, actual = [], {}
    for case in dict.fromkeys(key[0] for key in expected):
        for row in retained(case):
            key = (case, row["caller"], row["rule"])
            if key in actual:
                raise RuntimeError(f"Duplicate result: {key}")
            actual[key] = row["status"]
            rows.append((case, row))
    if actual != expected:
        mismatches = [(key, expected.get(key), actual.get(key))
                      for key in expected.keys() | actual.keys()
                      if expected.get(key) != actual.get(key)]
        raise RuntimeError(f"Unexpected standard-rule results: {mismatches}")

    observations = json.loads((HERE / "combination-rules.json").read_text())["rules"]
    combinations = retained("dsl-combinations")
    expected_keys = {(caller, rule) for caller in ("all_inputs", "checked")
                     for rule in observations}
    actual_keys = {(row["caller"], row["rule"]) for row in combinations}
    if len(combinations) != len(expected_keys) or actual_keys != expected_keys:
        raise RuntimeError("Missing or duplicated composition results")
    for row in combinations:
        caller, rule, status = row["caller"], row["rule"], row["status"]
        if rule == "obs-floor":
            if status not in {"verified", "reached-timeout"}:
                raise RuntimeError(f"Unexpected division result: {row}")
            if status == "reached-timeout":
                log = Path(row["log"])
                if "[ext.dsl] resolved constraint expr:" not in log.read_text():
                    raise RuntimeError("Division timed out before producing constraints")
        else:
            violation = rule == "obs-chain" or (
                caller == "all_inputs" and rule in {"obs-list", "obs-unicode"})
            wanted = "violation" if violation else "verified"
            if status != wanted:
                raise RuntimeError(f"Unexpected composition result: {caller}/{rule}: {status}, wanted {wanted}")
        rows.append(("dsl-combinations", row))

    lines = [f"{case}: {row['caller']:<24} {row['rule']:<9} {row['status']}"
             for case, row in rows]
    (ROOT / ".local/typerules-all.actual").write_text("\n".join(lines) + "\n")
    counts = Counter(row["status"] for _, row in rows)
    test_log = (ROOT / ".local/typerules-python-tests.log").read_text()
    test_count = re.search(r"Ran (\d+) tests", test_log)
    if not test_count or not test_log.rstrip().endswith("OK"):
        raise RuntimeError("Python validation did not pass")
    summary = {"expected_match": True, "crates": len({case for case, _ in rows}),
               "pairs": len(rows), "statuses": dict(counts),
               "validated_dsl_rules": len(json.loads((HERE / "rules.json").read_text())["rules"]) + len(observations),
               "python_tests": int(test_count[1]),
               "current_klee_sha256": hashlib.sha256((ROOT / "tools/klee/build/bin/klee").read_bytes()).hexdigest(),
               "unproved_cases": [line for line, (_, row) in zip(lines, rows)
                                  if row["status"] in {"eval-error", "reached-timeout"}],
               "contract_only": 80,
               "remaining_backend_work": ["allocation contract and originating generation tracking, including free/reuse",
                                          "bodyless or erased target operations and unchecked result models",
                                          "general DST/vtable validity and unbounded memory quantification"],
               "scope": "Existing DSL grammar and memory schema only; no ownership/borrow/pin/type-identity schema extensions."}
    (ROOT / ".local/typerules-validation-summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
