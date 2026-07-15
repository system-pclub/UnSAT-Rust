import json
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from cli.cmd.verify import _run_llm_testcase_pipeline
from cli.cmd.verify_poc import TestcaseInjection


class VerifyContextRetryTests(unittest.TestCase):
    def run_pipeline(self, root: Path, reruns: list[dict[str, object]]) -> tuple[dict, list]:
        crate = root / "crate"
        injected = root / "injected"
        artifacts = root / "artifacts"
        compose = artifacts / "klee-out"
        for directory in (crate, injected / "src", compose):
            directory.mkdir(parents=True, exist_ok=True)
        (compose / "info").write_text("compose-verify: final result: violation\n")
        (injected / "src/lib.rs").write_text("pub fn target() {}\n")
        rule_path = root / "rules.json"
        rule_path.write_text(
            json.dumps({"rules": {"rule-1": {"rule": "index must be in bounds"}}})
        )
        args = SimpleNamespace(
            skip_llm_testcase=False,
            skip_rerun=False,
            model="fake",
            llm_testcase_retries=2,
            llm_context="slice",
            rustc=None,
            klee_bin="klee",
            timeout_sec=1,
        )
        target = {
            "caller": {"name": "crate::target"},
            "callsite": {"id": "src-lib-rs-1-1", "path": "src/lib.rs", "line": 1},
        }
        context_modes: list[str] = []

        def generate(**kwargs):
            context_modes.append(kwargs["context_mode"])
            return "generated testcase"

        def inject(**kwargs):
            injection = kwargs["injection"]
            return TestcaseInjection(
                **{
                    **injection.__dict__,
                    "source_path": "src/lib.rs",
                    "line": 2,
                }
            )

        def build(**kwargs):
            kwargs["build_log_path"].write_text("build ok\n")
            ll = kwargs["output_dir"] / "crate.ll"
            ll.parent.mkdir(parents=True, exist_ok=True)
            ll.write_text("; llvm ir\n")
            return ll

        remaining = list(reruns)

        def rerun(**kwargs):
            result = remaining.pop(0)
            kwargs["log_path"].write_text(f"rerun: {result['status']}\n")
            return result

        with (
            patch("cli.cmd.verify.generate_safe_testcase", side_effect=generate),
            patch("cli.cmd.verify.ensure_cargo_feature"),
            patch("cli.cmd.verify.inject_testcase_at_callsite", side_effect=inject),
            patch("cli.cmd.verify.ensure_linked_llvm_ir_file", side_effect=build),
            patch("cli.cmd.verify._run_klee_compose_rerun", side_effect=rerun),
        ):
            result = _run_llm_testcase_pipeline(
                args=args,
                repo_root=root,
                cargo_dir=crate,
                injected_dir=injected,
                rule_dsl_path=rule_path,
                target=target,
                callsite_id="src-lib-rs-1-1",
                rule_id="rule-1",
                ast_json="{}",
                artifact_dir=artifacts,
                compose_output=compose,
                report_json=None,
            )
        return result, context_modes

    def test_rerun_miss_retries_and_final_attempt_falls_back_to_full_context(self) -> None:
        miss = {
            "status": "callsite-not-reported",
            "returncode": 2,
            "full_rerun_passed": False,
        }
        reproduced = {
            "status": "reported-sat",
            "returncode": 0,
            "full_rerun_passed": True,
        }
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            result, context_modes = self.run_pipeline(root, [miss, reproduced])
            state = json.loads(
                (root / "artifacts/testcase-pipeline.json").read_text()
            )

        self.assertTrue(result["full_rerun_passed"])
        self.assertEqual(context_modes, ["slice", "full"])
        self.assertEqual(
            [attempt["status"] for attempt in state["llm_testcase_attempts"]],
            ["rerun-miss", "reproduced"],
        )
        self.assertIn("did not reproduce", state["llm_testcase_attempts"][0]["feedback"])

    def test_successful_slice_does_not_pay_for_full_context(self) -> None:
        reproduced = {
            "status": "reported-sat",
            "returncode": 0,
            "full_rerun_passed": True,
        }
        with tempfile.TemporaryDirectory() as temporary:
            result, context_modes = self.run_pipeline(Path(temporary), [reproduced])

        self.assertTrue(result["full_rerun_passed"])
        self.assertEqual(context_modes, ["slice"])


if __name__ == "__main__":
    unittest.main()
