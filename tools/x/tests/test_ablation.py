import contextlib
import io
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from cli.cmd.verify import _merge_existing_matrix_rows, _run_klee_compose_verify
from cli.cli import build_parser


class AblationChecks(unittest.TestCase):
    def test_cli_defaults_and_profiles(self):
        parser = build_parser()
        self.assertEqual(parser.parse_args(['verify', 'fixture']).ablation_level, 'full')
        for level in ('A', 'B', 'C', 'full'):
            args = parser.parse_args(['verify', 'fixture', '--ablation-level', level])
            self.assertEqual(args.ablation_level, level.lower())
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            parser.parse_args(['verify', 'fixture', '--ablation-level', 'unsupported'])
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            parser.parse_args(['verify', 'fixture', '--ablation-level', 'c-symbolic'])

    def test_selected_profile_reaches_single_pair_klee_command(self):
        result = subprocess.CompletedProcess([], 0, stdout="", stderr="")
        with tempfile.TemporaryDirectory() as temporary, patch(
                'cli.cmd.verify.subprocess.run', return_value=result) as run, contextlib.redirect_stdout(io.StringIO()):
            _run_klee_compose_verify(
                ll_path=Path(temporary) / 'input.ll', callsite_id='target',
                ast_json='{}', report_json=None, klee_bin='klee',
                compose_loop_bound=1, output_dir=Path(temporary) / 'klee-out',
                timeout_sec=60, ablation_level='b')
        self.assertIn('--ruklee-ablation=b', run.call_args.args[0])

    def test_resume_cannot_mix_profiles_or_legacy_override(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'matrix.json'
            path.write_text(json.dumps({'ablation_level': 'b', 'ablation_profile_version': 2, 'results': []}))
            self.assertEqual(_merge_existing_matrix_rows(path, ablation_level='b'), {})
            with self.assertRaisesRegex(RuntimeError, 'different ablation'):
                _merge_existing_matrix_rows(path, ablation_level='c')
            with self.assertRaisesRegex(RuntimeError, 'different ablation'):
                _merge_existing_matrix_rows(path, ablation_level='b', disable_mut_ref_recovery=True)

    def test_old_results_are_full_only(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'matrix.json'
            path.write_text(json.dumps({'results': []}))
            self.assertEqual(_merge_existing_matrix_rows(path), {})
            with self.assertRaises(RuntimeError):
                _merge_existing_matrix_rows(path, ablation_level='a')
            path.write_text(json.dumps({'ablation_level': 'full', 'results': []}))
            with self.assertRaisesRegex(RuntimeError, 'older ablation definition'):
                _merge_existing_matrix_rows(path)
