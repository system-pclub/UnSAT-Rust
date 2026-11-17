import csv
import json
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import Mock, patch

from cli.cli import build_parser
from cli.cmd import extract, rule


def unit(name="api"):
    return {"name": name, "line_number_start": 5, "declaration_text": f"unsafe fn {name}()",
            "parent_struct_or_trait": None, "comment": "# Safety\nMust be valid."}


class RuleTests(unittest.TestCase):
    def test_cli_accepts_all_steps_and_multiple_directories(self):
        for step in (None, 0, 1, 2):
            argv = ["rule", "--output", "out", "library/std", "library/core", "library/alloc"]
            if step is not None:
                argv += ["--step", str(step)]
            args = build_parser().parse_args(argv)
            self.assertEqual(args.step, step)
            self.assertEqual(args.directories, ["library/std", "library/core", "library/alloc"])
        with self.assertRaises(SystemExit):
            build_parser().parse_args(["rule", "--step", "3", "--output", "out", "library/alloc"])

    def test_stage_selection_and_global_catalog_ids(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ("std", "core", "alloc"):
                (root / "library" / name).mkdir(parents=True)
            (root / "tools/x/generated/dsl").mkdir(parents=True)
            (root / "tools/x/generated/dsl/schema.json").write_text('{}')
            base = ["rule", "--output", "out", "library/std", "library/core", "library/alloc"]

            def comments(command, **kwargs):
                if "--out" in command:
                    name = Path(command[command.index("--dirs") + 1]).name
                    rule._write_json(Path(command[-1]), {"files": {f"library/{name}/src/lib.rs": [unit(name)]}})

            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule.subprocess, "run", side_effect=comments) as process:
                self.assertEqual(rule.run(build_parser().parse_args(base + ["--step", "0"])), 0)
                self.assertEqual(process.call_count, 4)  # One build, one Rust invocation per directory.
            for name in ("std", "core", "alloc"):
                self.assertTrue((root / "out" / name / "step0.json").is_file())
                self.assertFalse((root / "out" / name / "step1.json").exists())

            def extracted(destination, args):
                items = extract._source_items(json.loads((destination / "step0.json").read_text()))
                rule._write_json(destination / "step1.json", [{**item, "status": "extracted", "rules": ["Must be valid."]} for item in items])

            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_rules", side_effect=extracted), patch.object(rule, "_translate") as translate, patch.object(rule.subprocess, "run") as process:
                self.assertEqual(rule.run(build_parser().parse_args(base + ["--step", "1"])), 0)
                process.assert_not_called()
                translate.assert_not_called()
            with (root / "out/rules.csv").open() as handle:
                rows = list(csv.DictReader(handle))
            self.assertEqual([row["id"] for row in rows], ["rule-1", "rule-2", "rule-3"])
            self.assertEqual([row["path"] for row in rows], [f"library/{name}/src/lib.rs:5" for name in ("std", "core", "alloc")])

            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_rules") as rules, patch.object(rule, "_translate") as translate, patch.object(rule.subprocess, "run") as process:
                self.assertEqual(rule.run(build_parser().parse_args(base + ["--step", "2"])), 0)
                self.assertEqual(translate.call_count, 3)
                rules.assert_not_called()
                process.assert_not_called()

            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_rules"), patch.object(rule, "_translate"), patch.object(rule.subprocess, "run") as process:
                self.assertEqual(rule.run(build_parser().parse_args(base)), 0)
                process.assert_not_called()  # Existing step0.json files are reused.

    def test_global_round_barrier_survives_pending_batch(self):
        from llm.rule_jobs import BatchPending
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ("std", "core", "alloc"):
                (root / "library" / name).mkdir(parents=True)
                rule._write_json(root / "out" / name / "step0.json", {"files": {}})
                rule._write_json(root / "out" / name / "step1.json", [])
            rule._write_json(root / "tools/x/generated/dsl/schema.json", {})
            args = build_parser().parse_args(["rule", "--output", "out", "--step", "2", "--batch-no-wait", "library/std", "library/core", "library/alloc"])
            seen = []
            def translating(repo, destination, settings):
                seen.append((destination.name, settings._translation_round))
                if destination.name == "core" and settings._translation_round == 0:
                    raise BatchPending("batch pending")
                return destination.name == "std" and settings._translation_round == 0
            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_translate", side_effect=translating):
                self.assertEqual(rule.run(args), 75)
            self.assertEqual(seen, [("std", 0), ("core", 0), ("alloc", 0)])
            seen.clear()
            def completed(repo, destination, settings):
                seen.append((destination.name, settings._translation_round))
                return destination.name == "std" and settings._translation_round == 0
            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_translate", side_effect=completed):
                self.assertEqual(rule.run(args), 0)
            self.assertEqual(seen, [("std", 0), ("core", 0), ("alloc", 0), ("std", 1), ("core", 1), ("alloc", 1)])

    def test_unknown_smoke_location_is_rejected_before_paid_calls(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "core").mkdir()
            rule._write_json(root / "out/core/step0.json", {"files": {}})
            args = build_parser().parse_args(["rule", "--step", "1", "--output", "out", "--api-location", "missing.rs:1", "core"])
            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_rules") as extract:
                with self.assertRaisesRegex(RuntimeError, "unknown --api-location"):
                    rule.run(args)
                extract.assert_not_called()

    def test_missing_prerequisite_fails_before_requests(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "alloc").mkdir()
            args = build_parser().parse_args(["rule", "--step", "1", "--output", "out", "alloc"])
            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_rules") as rules:
                with self.assertRaisesRegex(RuntimeError, "run --step 0 first"):
                    rule.run(args)
                rules.assert_not_called()

    def test_duplicate_output_names_are_rejected(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "a/alloc").mkdir(parents=True)
            (root / "b/alloc").mkdir(parents=True)
            with self.assertRaisesRegex(RuntimeError, "same output name"):
                rule._inputs(root, root / "out", ["a/alloc", "b/alloc"])

    def test_reusing_output_for_a_different_source_is_rejected(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "a/alloc").mkdir(parents=True)
            (root / "b/alloc").mkdir(parents=True)
            rule._write_json(root / "out/alloc/input.json", {"directory": "a/alloc"})
            args = build_parser().parse_args(["rule", "--step", "0", "--output", "out", "b/alloc"])
            with patch.object(rule, "_repo_root", return_value=root), patch.object(rule, "_comments") as comments:
                with self.assertRaisesRegex(RuntimeError, "different input"):
                    rule.run(args)
                comments.assert_not_called()
            self.assertFalse((root / "out/inputs.json").exists())

    def test_changed_comments_require_extracting_rules_again(self):
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            destination = root / "out/alloc"
            document = {"files": {"library/alloc/src/lib.rs": [unit()]}}
            item = extract._source_items(document)[0]
            rule._write_json(destination / "step1.json", [{**item, "status": "extracted", "rules": ["Valid."]}])
            document["files"]["library/alloc/src/lib.rs"][0]["comment"] = "Changed documentation."
            rule._write_json(destination / "step0.json", document)
            with self.assertRaisesRegex(RuntimeError, "stale.*step1.json"):
                rule._catalog(root / "out", [(root / "library/alloc", destination)])
            self.assertFalse((root / "out/rules.csv").exists())

    def test_extraction_cache_reused_but_source_changes_invalidate_it(self):
        with TemporaryDirectory() as temporary:
            client = Mock()
            response = client.chat.completions.create.return_value
            response.model_dump.return_value = {}
            response.choices = [Mock(message=Mock(content='{"rules":["Must be valid."]}'))]
            item = extract._source_items({"files": {"library/alloc/src/lib.rs": [unit()]}})[0]
            first = extract._extract_one(client, "model", item, Path(temporary))
            self.assertEqual(first["status"], "extracted")
            self.assertEqual(extract._extract_one(None, "model", item, Path(temporary)), first)
            self.assertEqual(client.chat.completions.create.call_count, 1)
            extract._extract_one(client, "model", {**item, "documentation": "Changed safety requirement."}, Path(temporary))
            self.assertEqual(client.chat.completions.create.call_count, 2)

    def test_translation_includes_alloc_and_other_input_directories(self):
        from llm import translate_rules

        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            output = root / "translation"
            (output / "inputs").mkdir(parents=True)
            rows = [{"id": f"rule-{index}", "path": f"{path}:5", "name": "unsafe fn api()", "rule": "Valid."}
                    for index, path in enumerate(("rust/library/alloc/src/lib.rs", "custom/src/lib.rs"), 1)]
            rule._write_csv(output / "inputs/rules.csv", rows)
            (output / "inputs/operators.json").write_text('{"operators": []}')
            source = root / "step0.json"
            rule._write_json(source, {"files": {row["path"].rpartition(":")[0]: [unit()] for row in rows}})

            def translated(row, context, operators, args, out):
                result = {**row, "status": "translated", "dsl": "true"}
                path = out / "requests" / row["id"]
                path.mkdir(parents=True, exist_ok=True)
                translate_rules.write_json(path / "result.json", result)
                self.assertEqual(context["source_line"], 5)
                return result

            argv = ["translate_rules", "--output", str(output), "--source-units", str(source)]
            with patch("sys.argv", argv), patch.object(translate_rules, "run_one", side_effect=translated) as llm:
                translate_rules.main()
                self.assertEqual(llm.call_count, 2)
            cached = output / "requests/rule-1/result.json"
            rule._write_json(cached, {**rows[0], "status": "request_error"})
            with patch("sys.argv", argv), patch.object(translate_rules, "run_one", side_effect=translated) as llm:
                translate_rules.main()
                self.assertEqual(llm.call_count, 1)  # Retry errors while retaining successful translations.
            results = json.loads((output / "generation-results.json").read_text())
            self.assertEqual([row["id"] for row in results], ["rule-1", "rule-2"])


if __name__ == "__main__":
    unittest.main()
