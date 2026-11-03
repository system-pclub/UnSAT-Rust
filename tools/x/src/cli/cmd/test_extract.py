import csv
import json
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest

from cli.cmd.extract import _parse_rules, _source_items, _write_rules_csv


class ExtractTest(unittest.TestCase):
    def test_parse_rules_accepts_json_fence_and_normalizes_whitespace(self) -> None:
        self.assertEqual(
            _parse_rules('```json\n{"rules":[" first   rule ", {"rule":"second rule"}]}\n```'),
            ["first rule", "second rule"],
        )

    def test_source_items_normalize_paths_and_sort(self) -> None:
        document = {
            "files": {
                "tools/rust/library/std/src/z.rs": [
                    {
                        "line_number_start": 9,
                        "name": "z",
                        "declaration_text": "unsafe fn z()",
                        "parent_struct_or_trait": None,
                        "comment": "# Safety",
                    }
                ],
                "./tools/rust/library/core/src/a.rs": [
                    {
                        "line_number_start": 2,
                        "name": "a",
                        "declaration_text": "unsafe fn a()",
                        "parent_struct_or_trait": "A",
                        "comment": "# Safety\nBe valid.",
                    }
                ],
            }
        }
        items = _source_items(document)
        self.assertEqual(items[0]["path"], "rust/library/core/src/a.rs")
        self.assertEqual(items[1]["path"], "rust/library/std/src/z.rs")

    def test_write_rules_csv_assigns_stable_ids(self) -> None:
        results = [
            {
                "path": "rust/library/core/src/a.rs",
                "line": 2,
                "name": "a",
                "declaration": "unsafe fn a()",
                "rules": ["one", "two"],
            }
        ]
        with TemporaryDirectory() as directory:
            path = Path(directory) / "rules.csv"
            self.assertEqual(_write_rules_csv(path, results), 2)
            with path.open(encoding="utf-8") as handle:
                rows = list(csv.DictReader(handle))
        self.assertEqual([row["id"] for row in rows], ["rule-1", "rule-2"])
        self.assertEqual(rows[0]["path"], "rust/library/core/src/a.rs:2")


if __name__ == "__main__":
    unittest.main()
