"""Regression checks for advisory-only scheduling; no compiler/KLEE invocation."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("rustsec_batch", Path(__file__).with_name("run.py"))
batch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(batch)


class AdvisoryBatchTests(unittest.TestCase):
    def test_all_benchmark_crates_have_explicit_selections(self):
        crates = {p.name for p in batch.BENCHMARK.iterdir() if (p / "Cargo.toml").is_file()}
        self.assertTrue(crates <= batch.ADVISORY_TARGETS.keys())
        for crate in crates - batch.UNSUPPORTED.keys():
            self.assertTrue(batch.selected_pairs(crate), crate)

    def test_iced_x86_filters_only_the_advisory_check(self):
        with tempfile.TemporaryDirectory() as tmp:
            args = batch.write_selection("iced-x86-1.10.3", Path(tmp))
            self.assertEqual(Path(args[1]).read_text(), "src-decoder-mod-rs-458-42\n")
            dsl = json.loads(Path(args[3]).read_text())
            self.assertEqual(set(dsl), {"rule-575"})
            frozen = batch.read_json(batch.ROOT / "ptr_rule_dsl.json")
            self.assertEqual(dsl["rule-575"], frozen.get("rules", frozen)["rule-575"])

    def test_raw_deref_does_not_load_unrelated_dsl(self):
        with tempfile.TemporaryDirectory() as tmp:
            args = batch.write_selection("slab-0.4.10", Path(tmp))
            self.assertEqual(json.loads(Path(args[3]).read_text()), {})
            self.assertEqual(Path(args[1]).read_text(), "src-lib-rs-836-28\n")

    def test_reuse_requires_completed_selected_pairs(self):
        crate = "iced-x86-1.10.3"
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            output = root / crate / "ruklee" / "old"
            output.mkdir(parents=True)
            batch.write_json(output / "run.json", {"status": "completed", "exit_code": 0})
            matrix = {"total": 1, "completed": 1,
                      "results": [{"callsite": "unrelated", "rule": "__raw_ptr_deref__"}]}
            batch.write_json(output / "matrix.json", matrix)
            self.assertIsNone(batch.previous_run(crate, [root]))
            matrix["results"] = [{"callsite": "src-decoder-mod-rs-458-42", "rule": "rule-575"}]
            batch.write_json(output / "matrix.json", matrix)
            self.assertEqual(batch.previous_run(crate, [root]), output)
            matrix["completed"] = 0
            batch.write_json(output / "matrix.json", matrix)
            self.assertIsNone(batch.previous_run(crate, [root]))
            matrix["completed"] = 1
            batch.write_json(output / "matrix.json", matrix)
            batch.write_json(output / "run.json", {"status": "interrupted", "exit_code": 0})
            self.assertIsNone(batch.previous_run(crate, [root]))

    def test_release_only_selection_does_not_reuse_debug_run(self):
        crate = "ruint-1.16.0"
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            output = root / crate / "ruklee" / "old"
            output.mkdir(parents=True)
            record = {"status": "completed", "exit_code": 0, "command": ["./x", "verify"]}
            batch.write_json(output / "run.json", record)
            batch.write_json(output / "matrix.json", {
                "total": 1, "completed": 1,
                "results": [{"callsite": "src-algorithms-div-reciprocal-rs-87-33", "rule": "rule-572"}],
            })
            self.assertIsNone(batch.previous_run(crate, [root], no_debug_assertions=True))
            record["command"].append("--no-debug-assertions")
            batch.write_json(output / "run.json", record)
            self.assertEqual(batch.previous_run(crate, [root], no_debug_assertions=True), output)

    def test_new_crate_does_not_fall_back_to_full_scan(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "new-0.1.0"
            crate.mkdir()
            (crate / "Cargo.toml").write_text('[package]\nname = "new"\nversion = "0.1.0"\n')
            with patch.object(batch, "BENCHMARK", root), patch("sys.argv", ["run.py", "--dry-run"]):
                with contextlib.redirect_stderr(io.StringIO()) as stderr:
                    with self.assertRaises(SystemExit) as error:
                        batch.main()
                self.assertEqual(error.exception.code, 2)
                self.assertIn("no full-scan fallback", stderr.getvalue())


if __name__ == "__main__":
    unittest.main()
