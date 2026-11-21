import contextlib
import io
import subprocess
import contextlib
import io
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from cli.cmd.verify import _compose_status_from_output, _run_klee_compose_verify


class ComposeStatusChecks(unittest.TestCase):
    def test_execution_model_failure_survives_final_query_boundary(self):
        prefix = ("[ext.dsl] failed to evaluate expression: execution model: "
                  "concretized symbolic size\n[compose-verify] final-query-begin\n")
        output = prefix + "[ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n"
        self.assertEqual(_compose_status_from_output(0, output, False), "eval-error")
        self.assertEqual(_compose_status_from_output(None, output, True), "eval-error")
        output = prefix + "[ext.dsl] SAT(constraints AND NOT resolved constraint): sat\n"
        self.assertEqual(_compose_status_from_output(0, output, False), "violation")

    def test_final_composed_state_supersedes_optimistic_probe(self):
        output = (
            "[ext.dsl] SAT(constraints AND NOT resolved constraint): sat\n"
            "[compose-verify] final-query-begin\n"
            "[ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n")
        self.assertEqual(_compose_status_from_output(0, output, False), 'verified')
        self.assertEqual(_compose_status_from_output(None, output, True), 'reached-timeout')

    def test_incomplete_final_query_cannot_reuse_probe_counterexample(self):
        output = (
            "[ext.dsl] SAT(constraints AND NOT resolved constraint): sat\n"
            "[compose-verify] final-query-begin\n")
        self.assertEqual(_compose_status_from_output(0, output, False), 'no-final-result')
        self.assertEqual(_compose_status_from_output(None, output, True), 'reached-timeout')

    def test_evaluation_error_is_not_a_proof_from_another_path(self):
        output = (
            "KLEE: [ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n"
            "KLEE: WARNING: [ext.dsl] failed to evaluate expression: missing observation\n"
        )
        self.assertEqual(_compose_status_from_output(0, output, False), "eval-error")

    def test_counterexample_survives_another_paths_evaluation_error(self):
        output = (
            "KLEE: WARNING: [ext.dsl] failed to evaluate expression: missing observation\n"
            "KLEE: [ext.dsl] SAT(constraints AND NOT resolved constraint): sat\n"
        )
        self.assertEqual(_compose_status_from_output(0, output, False), "violation")

    def test_crash_after_marker_is_an_error(self):
        output = "[ext.exec] klee_ext_callsite site='target' target='target'\n"
        self.assertEqual(_compose_status_from_output(-6, output, False, "target"), "klee-error")

    def test_solver_failure_prevents_verification(self):
        output = (
            "[ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n"
            "[ext.dsl] SAT check failed: solver returned an error\n"
        )
        self.assertEqual(_compose_status_from_output(0, output, False), "solver-error")

    def test_unsat_before_timeout_is_not_a_complete_proof(self):
        output = "[ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n"
        self.assertEqual(_compose_status_from_output(None, output, True), "reached-timeout")
        output += "[ext.dsl] failed to evaluate expression: missing observation\n"
        self.assertEqual(_compose_status_from_output(None, output, True), "eval-error")

    def test_single_pair_pipeline_preserves_evaluation_errors(self):
        result = subprocess.CompletedProcess([], 0,
            stdout="[ext.dsl] SAT(constraints AND NOT resolved constraint): unsat\n",
            stderr="[ext.dsl] failed to evaluate expression: missing observation\n")
        with tempfile.TemporaryDirectory() as temporary, patch(
                'cli.cmd.verify.subprocess.run', return_value=result), contextlib.redirect_stdout(io.StringIO()):
            code, status = _run_klee_compose_verify(
                ll_path=Path(temporary) / 'input.ll', callsite_id='target',
                ast_json='{}', report_json=None, klee_bin='klee',
                compose_loop_bound=1, output_dir=Path(temporary) / 'klee-out',
                timeout_sec=60)
        self.assertEqual((code, status), (0, 'eval-error'))


if __name__ == "__main__":
    unittest.main()
