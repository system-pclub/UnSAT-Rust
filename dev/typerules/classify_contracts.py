"""Report the source inventory's trait obligations separately from callsite proofs.

This report makes no DSL translation or verification claim. Exact source/rule
matches remain attached so implementation contracts cannot count as UNSAT.
"""
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    inventory = ROOT / "Rust 1.87.0 type rules.md"
    section = inventory.read_text().split("## F. Trait 实现契约\n", 1)[1].split("\n## G.", 1)[0]
    with (ROOT / "rules.csv").open(newline="") as handle:
        rules = list(csv.DictReader(handle))
    entries = []
    for line in section.splitlines():
        if not line.startswith("| `library/"):
            continue
        location, api, obligation = (cell.strip() for cell in line.strip("|").split("|", 2))
        location, api = location.strip("`"), api.strip("`")
        matches = [rule["id"] for rule in rules
                   if rule["path"] == "rust/" + location and rule["rule"] == obligation]
        entries.append({"path": location, "api": api, "rule": obligation,
                        "matching_rule_ids": matches, "status": "contract-only",
                        "reason": "Trait implementation obligation; a single callsite state cannot establish this contract."})
    if len(entries) != 80:
        raise RuntimeError(f"Trait inventory changed: expected 80 obligations, found {len(entries)}")
    output = {"source": str(inventory.relative_to(ROOT)), "count": len(entries),
              "verification_claim": False, "rules": entries}
    (Path(__file__).parent / "contract-only.json").write_text(
        json.dumps(output, ensure_ascii=False, indent=2) + "\n")
    print(f"Reported {len(entries)} trait obligations as contract-only (no SAT/UNSAT claim)")


if __name__ == "__main__":
    main()
