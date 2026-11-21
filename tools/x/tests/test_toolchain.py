import os
import subprocess
import unittest
from unittest.mock import patch

from cli import toolchain


class PinDefaultToolchainChecks(unittest.TestCase):
    def setUp(self):
        toolchain._default_toolchain.cache_clear()

    def test_default_toolchain_overrides_the_crates_rust_toolchain_file(self):
        result = subprocess.CompletedProcess(
            [], 0, stdout="nightly-2025-03-28-x86_64-unknown-linux-gnu (default)\n", stderr="")
        with patch("cli.toolchain.subprocess.run", return_value=result) as run:
            env = toolchain.pin_default_toolchain({})
        self.assertEqual(env["RUSTUP_TOOLCHAIN"], "nightly-2025-03-28-x86_64-unknown-linux-gnu")
        # Asked outside any crate, so a rust-toolchain file cannot change the answer.
        self.assertNotEqual(run.call_args.kwargs["cwd"], os.getcwd())

    def test_explicit_choice_is_kept(self):
        with patch("cli.toolchain.subprocess.run") as run:
            env = toolchain.pin_default_toolchain({"RUSTUP_TOOLCHAIN": "1.87.0"})
        self.assertEqual(env["RUSTUP_TOOLCHAIN"], "1.87.0")
        run.assert_not_called()

    def test_missing_rustup_leaves_environment_unchanged(self):
        with patch("cli.toolchain.subprocess.run", side_effect=FileNotFoundError):
            self.assertEqual(toolchain.pin_default_toolchain({}), {})


if __name__ == "__main__":
    unittest.main()
