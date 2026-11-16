"""Print the API count and one source API per line from step0.json."""

import argparse
import json
from pathlib import Path


def print_step0(path: Path) -> None:
    if path.is_dir():
        path = path / "step0.json"
    document = json.loads(path.read_text(encoding="utf-8"))
    apis = [(source, unit) for source, units in document["files"].items() for unit in units]
    apis.sort(key=lambda item: (item[0], item[1]["line_number_start"], item[1]["name"]))
    print(f"[step0] {path}: APIs={len(apis)}", flush=True)
    for source, unit in apis:
        declaration = " ".join(unit["declaration_text"].split())
        parent = " ".join((unit.get("parent_struct_or_trait") or "").split())
        owner = f"[{parent}] " if parent else ""
        print(f"{source}:{unit['line_number_start']}\t{owner}{declaration}", flush=True)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path, help="step0.json files or directories containing them.")
    args = parser.parse_args()
    for path in args.paths:
        print_step0(path)


if __name__ == "__main__":
    main()
