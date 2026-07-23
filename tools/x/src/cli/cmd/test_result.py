from __future__ import annotations

import argparse
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from cli.cmd.result import run


def _write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value), encoding="utf-8")


class ResultCommandTests(unittest.TestCase):
    def test_dir_aggregates_rule_rows_by_callsite(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "run"
            _write_json(
                root / "full-matrix-results.json",
                {
                    "crate": "/tmp/crates/demo-1.0.0",
                    "results": [
                        {
                            "callsite": "src-lib-rs-10-5",
                            "rule": "rule-1",
                            "status": "verified",
                        },
                        {
                            "callsite": "src-lib-rs-10-5",
                            "rule": "rule-2",
                            "status": "violation",
                            "full_rerun_passed": True,
                        },
                        {
                            "callsite": "src-lib-rs-20-5",
                            "rule": "rule-1",
                            "status": "verified",
                        },
                    ],
                },
            )
            args = argparse.Namespace(result_dir=str(root), result_dirdir=None)

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            self.assertEqual(
                [call.args[0] for call in output.call_args_list],
                [
                    "demo-1.0.0 src-lib-rs-10-5 BUG",
                    "demo-1.0.0 src-lib-rs-20-5 OK",
                ],
            )

    def test_dirdir_reads_each_crate_directory(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for crate, status, rerun in (
                ("alpha-1.0.0", "ok", False),
                ("beta-2.0.0", "violation", True),
            ):
                _write_json(
                    root / crate / "full-matrix-results.json",
                    {
                        "crate": f"/tmp/{crate}",
                        "results": [
                            {
                                "callsite": "src-lib-rs-1-1",
                                "status": status,
                                "full_rerun_passed": rerun,
                            }
                        ],
                    },
                )
            args = argparse.Namespace(result_dir=None, result_dirdir=str(root))

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            self.assertEqual(
                [call.args[0] for call in output.call_args_list],
                [
                    "alpha-1.0.0 src-lib-rs-1-1 OK",
                    "beta-2.0.0 src-lib-rs-1-1 BUG",
                ],
            )

    def test_legacy_prefixed_matrix_filename_is_supported(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "old-run"
            _write_json(
                root / "demo-full-matrix-results.json",
                {
                    "crate": "/tmp/demo-1.0.0",
                    "results": [
                        {
                            "callsite": "src-lib-rs-2-3",
                            "status": "violation",
                            "full_rerun_passed": True,
                        }
                    ],
                },
            )
            args = argparse.Namespace(result_dir=str(root), result_dirdir=None)

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 src-lib-rs-2-3 BUG")

    def test_pair_result_is_supported_without_matrix_json(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "demo-1.0.0"
            _write_json(
                root / "src-lib-rs-3-4" / "rule-1" / "result.json",
                {
                    "crate": "/tmp/demo-1.0.0",
                    "callsite": {"id": "src-lib-rs-3-4"},
                    "result": {
                        "init_status": "violation",
                        "full_rerun_passed": True,
                    },
                },
            )
            args = argparse.Namespace(result_dir=str(root), result_dirdir=None)

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 src-lib-rs-3-4 BUG")


if __name__ == "__main__":
    unittest.main()
