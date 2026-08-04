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
                            "callsite": "src-lib-rs-10-5",
                            "rule": "rule-3",
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
                    "demo-1.0.0 src-lib-rs-10-5 rule-1 OK",
                    "demo-1.0.0 src-lib-rs-10-5 rule-2 BUG",
                    "demo-1.0.0 src-lib-rs-10-5 rule-3 BUG",
                    "demo-1.0.0 src-lib-rs-20-5 rule-1 OK",
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
                                "rule": "rule-1",
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
                    "alpha-1.0.0 src-lib-rs-1-1 rule-1 OK",
                    "beta-2.0.0 src-lib-rs-1-1 rule-1 BUG",
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
                            "rule": "rule-9",
                            "status": "violation",
                            "full_rerun_passed": True,
                        }
                    ],
                },
            )
            args = argparse.Namespace(result_dir=str(root), result_dirdir=None)

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 src-lib-rs-2-3 rule-9 BUG")

    def test_pair_result_is_supported_without_matrix_json(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "demo-1.0.0"
            _write_json(
                root / "src-lib-rs-3-4" / "rule-1" / "result.json",
                {
                    "crate": "/tmp/demo-1.0.0",
                    "callsite": {"id": "src-lib-rs-3-4"},
                    "rule": {"id": "rule-1"},
                    "result": {
                        "init_status": "violation",
                        "full_rerun_passed": True,
                    },
                },
            )
            args = argparse.Namespace(result_dir=str(root), result_dirdir=None)

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 src-lib-rs-3-4 rule-1 BUG")

    def test_caller_granularity_groups_callsites_and_rules(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "run"
            _write_json(
                root / "full-matrix-results.json",
                {
                    "crate": "/tmp/crates/demo-1.0.0",
                    "results": [
                        {
                            "callsite": "src-lib-rs-10-5",
                            "caller": "demo::parse",
                            "rule": "rule-1",
                            "status": "verified",
                        },
                        {
                            "callsite": "src-lib-rs-10-5",
                            "caller": "demo::parse",
                            "rule": "rule-2",
                            "status": "violation",
                            "full_rerun_passed": True,
                        },
                        {
                            "callsite": "src-lib-rs-20-5",
                            "caller": "demo::parse",
                            "rule": "rule-3",
                            "status": "verified",
                        },
                        {
                            "callsite": "src-lib-rs-30-5",
                            "caller": "demo::emit",
                            "rule": "rule-1",
                            "status": "verified",
                        },
                    ],
                },
            )
            args = argparse.Namespace(
                result_dir=str(root),
                result_dirdir=None,
                granularity="caller",
            )

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            self.assertEqual(
                [call.args[0] for call in output.call_args_list],
                [
                    "demo-1.0.0 demo::emit OK",
                    "demo-1.0.0 demo::parse BUG",
                ],
            )

    def test_caller_granularity_uses_matrix_callsite_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "run"
            _write_json(
                root / "full-matrix-results.json",
                {
                    "crate": "/tmp/crates/demo-1.0.0",
                    "callsites": [
                        {
                            "callsite_id": "src-lib-rs-10-5",
                            "caller": "demo::parse",
                        }
                    ],
                    "results": [
                        {
                            "callsite": "src-lib-rs-10-5",
                            "rule": "rule-1",
                            "status": "verified",
                        }
                    ],
                },
            )
            args = argparse.Namespace(
                result_dir=str(root),
                result_dirdir=None,
                granularity="caller",
            )

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 demo::parse OK")

    def test_caller_granularity_reads_pair_result_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "demo-1.0.0"
            _write_json(
                root / "src-lib-rs-3-4" / "rule-1" / "result.json",
                {
                    "crate": "/tmp/demo-1.0.0",
                    "callsite": {
                        "id": "src-lib-rs-3-4",
                        "caller": "demo::parse",
                    },
                    "rule": {"id": "rule-1"},
                    "result": {
                        "init_status": "violation",
                        "full_rerun_passed": True,
                    },
                },
            )
            args = argparse.Namespace(
                result_dir=str(root),
                result_dirdir=None,
                granularity="caller",
            )

            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            output.assert_called_once_with("demo-1.0.0 demo::parse BUG")

    def test_unsafe_api_summary_prints_per_crate_and_all_callsites(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for crate, callsites in (
                (
                    "alpha-1.0.0",
                    [
                        ("site-a", "core::ptr::read", "verified", False),
                        ("site-b", "core::ptr::write", "violation", True),
                        # Another rule for the same API must not increase its count.
                        ("site-b", "core::ptr::write", "verified", False),
                    ],
                ),
                (
                    "beta-2.0.0",
                    [
                        ("site-c", "core::ptr::read", "violation", True),
                        ("site-d", "core::slice::get_unchecked", "verified", False),
                    ],
                ),
            ):
                rows = []
                metadata = []
                for index, (callsite, api, status, rerun) in enumerate(callsites):
                    rows.append(
                        {
                            "callsite": callsite,
                            "rule": f"rule-{index}",
                            "status": status,
                            "full_rerun_passed": rerun,
                        }
                    )
                    metadata.append(
                        {
                            "callsite_id": callsite,
                            "unsafe_callee": api,
                            "unsafe_callee_path": "rust/library/core/src/api.rs",
                            "unsafe_callee_line_start": {
                                "core::ptr::read": 1,
                                "core::ptr::write": 2,
                                "core::slice::get_unchecked": 3,
                            }[api],
                        }
                    )
                _write_json(
                    root / crate / "full-matrix-results.json",
                    {
                        "crate": f"/tmp/{crate}",
                        "callsites": metadata,
                        "results": rows,
                    },
                )

            args = argparse.Namespace(
                result_dir=None,
                result_dirdir=str(root),
                granularity="rule",
                unsafe_api_summary=True,
            )
            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            self.assertEqual(
                [call.args[0] for call in output.call_args_list],
                [
                    "alpha-1.0.0:",
                    "core::ptr::read 0/1",
                    "core::ptr::write 1/1",
                    "beta-2.0.0:",
                    "core::ptr::read 1/1",
                    "core::slice::get_unchecked 0/1",
                    "all:",
                    "core::ptr::read 1/2",
                    "core::ptr::write 1/1",
                    "core::slice::get_unchecked 0/1",
                    "how many unique unsafe api: 3",
                    "how many buggy/total unsafe api: 2/3 66.67%",
                    "how many buggy/total unsafe api callsites: 2/4 50.00%",
                ],
            )

    def test_unsafe_api_summary_reads_pair_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "demo-1.0.0"
            _write_json(
                root / "site-a" / "rule-1" / "result.json",
                {
                    "crate": "/tmp/demo-1.0.0",
                    "callsite": {
                        "id": "site-a",
                        "unsafe_callee": "core::ptr::read",
                    },
                    "rule": {"id": "rule-1"},
                    "result": {
                        "init_status": "violation",
                        "full_rerun_passed": True,
                    },
                },
            )
            args = argparse.Namespace(
                result_dir=str(root),
                result_dirdir=None,
                unsafe_api_summary=True,
            )
            with patch("builtins.print") as output:
                self.assertEqual(run(args), 0)

            self.assertEqual(
                [call.args[0] for call in output.call_args_list],
                [
                    "demo-1.0.0:",
                    "core::ptr::read 1/1",
                    "all:",
                    "core::ptr::read 1/1",
                    "how many unique unsafe api: 1",
                    "how many buggy/total unsafe api: 1/1 100.00%",
                    "how many buggy/total unsafe api callsites: 1/1 100.00%",
                ],
            )


if __name__ == "__main__":
    unittest.main()
