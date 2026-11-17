"""Regression checks for advisory-only scheduling; no compiler/KLEE invocation."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch
from types import SimpleNamespace
import shutil

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
        with tempfile.TemporaryDirectory(dir=batch.ROOT / ".local") as tmp:
            args = batch.write_selection("iced-x86-1.10.3", Path(tmp))
            self.assertEqual(Path(args[1]).read_text(), "src-decoder-mod-rs-458-42\n")
            dsl = json.loads(Path(args[3]).read_text())
            self.assertEqual(set(dsl), {"rule-575"})
            frozen = batch.read_json(batch.ROOT / "ptr_rule_dsl.json")
            self.assertEqual(dsl["rule-575"], frozen.get("rules", frozen)["rule-575"])

    def test_raw_deref_does_not_load_unrelated_dsl(self):
        with tempfile.TemporaryDirectory(dir=batch.ROOT / ".local") as tmp:
            args = batch.write_selection("slab-0.4.10", Path(tmp))
            self.assertEqual(json.loads(Path(args[3]).read_text()), {})
            self.assertEqual(Path(args[1]).read_text(), "src-lib-rs-836-28\n")

    def test_success_requires_exact_matrix_and_selected_detection(self):
        variants = [
            ("src-decoder-mod-rs-458-42", "violation", "completed"),
            ("unrelated", "violation", "failed"),
            ("src-decoder-mod-rs-458-42", "verified", "missed"),
        ]
        for callsite, status, expected in variants:
            with self.subTest(status=expected), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                source = root / "inputs/iced-x86-1.10.3"
                source.mkdir(parents=True)
                (source / "Cargo.toml").write_text('[package]\nname="iced-x86"\nversion="1.10.3"\n')
                (source / "src").mkdir()
                (source / "src/lib.rs").write_text("")
                shutil.copy2(batch.ROOT / "ptr_rule_dsl.json", root / "ptr_rule_dsl.json")
                args = SimpleNamespace(output_root=root / "results", tests=False,
                                       skip_llm=True, no_debug_assertions=False,
                                       timeout=60, crate_timeout=300)

                def launch(command, **kwargs):
                    matrix_path = root / command[command.index("--results-json") + 1]
                    batch.write_json(matrix_path, {
                        "total": 1, "completed": 1,
                        "results": [{"callsite": callsite, "rule": "rule-575", "status": status}],
                    })
                    process = Mock()
                    process.wait.return_value = 0
                    return process

                with patch.object(batch, "ROOT", root), patch.object(batch.subprocess, "Popen", launch):
                    result = batch.run_crate(source, args, {}, "test")
                record = batch.read_json(args.output_root / source.name / "ruklee/test/run.json")
                self.assertEqual(record["status"], expected)
                self.assertEqual(result, expected == "completed")

    def test_toodee_selections_cover_both_advisories(self):
        targets = batch.ADVISORY_TARGETS["toodee-0.5.0"]
        self.assertIn("src-toodee-rs-696-13", targets)  # insert_row
        self.assertIn("src-toodee-rs-1076-21", targets)  # DrainCol's DropGuard
        self.assertNotIn("toodee-0.5.0", batch.UNSUPPORTED)

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
