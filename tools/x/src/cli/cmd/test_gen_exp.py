import tempfile
import unittest
from pathlib import Path

from cli.cmd.gen_exp import _artifact_dir, _find_target, _get_rule, extract_rust_code


class GenExpTest(unittest.TestCase):
    def test_finds_callsite_by_report_id(self) -> None:
        target = {"callsite": {"id": "src-lib-rs-10-2"}}
        self.assertIs(_find_target({"targets": [target]}, "src-lib-rs-10-2"), target)

    def test_loads_nested_rule_shape(self) -> None:
        rule = {"rule": "pointer must be aligned"}
        self.assertIs(_get_rule({"rules": {"rule-1": rule}}, "rule-1"), rule)

    def test_extracts_only_rust_fence(self) -> None:
        response = "Explanation.\n```rust\nfn main() {}\n```\nMore."
        self.assertEqual(extract_rust_code(response), "fn main() {}\n")

    def test_rejects_response_without_code(self) -> None:
        with self.assertRaisesRegex(RuntimeError, "Rust code block"):
            extract_rust_code("No exploit is possible.")

    def test_artifact_path_is_scoped_and_sanitized(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            result = _artifact_dir(
                root, Path("/crates/demo"), "src/lib.rs:2:3", "rule-447"
            )
            self.assertEqual(
                result, root / "demo" / "src-lib.rs-2-3" / "rule-447"
            )


if __name__ == "__main__":
    unittest.main()
