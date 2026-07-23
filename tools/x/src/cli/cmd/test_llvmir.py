import tempfile
import unittest
import os
from pathlib import Path
from unittest.mock import patch

from cli.cmd.llvmir import (
    _build_std_args,
    _collect_test_link_llvm_irs,
    _dedupe_latest_llvm_irs,
    _find_library_test_harness_llvm_ir,
    compile_test_with_emit_llvm,
)


class LlvmIrSelectionTests(unittest.TestCase):
    @staticmethod
    def write_ir(path: Path, *, main: bool = False) -> Path:
        body = "define i32 @main() { ret i32 0 }\n" if main else "; library IR\n"
        path.write_text(body, encoding="utf-8")
        return path

    def test_abort_build_std_does_not_enable_panic_unwind(self) -> None:
        args = _build_std_args(test=True, panic_abort=True)

        self.assertIn("-Zbuild-std=std,panic_abort,test", args)
        features = next(
            arg for arg in args if arg.startswith("-Zbuild-std-features=")
        )
        self.assertNotIn("panic-unwind", features)

    @patch("cli.cmd.llvmir.subprocess.run")
    def test_test_build_only_compiles_library_harness(self, run) -> None:
        run.return_value.returncode = 0
        run.return_value.stdout = ""
        run.return_value.stderr = ""

        compile_test_with_emit_llvm(Path("/tmp/example"))

        command = run.call_args.args[0]
        self.assertEqual(command[:4], ["cargo", "test", "--lib", "--no-run"])

    def test_selects_package_library_unit_test_harness(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            deps = Path(temporary)
            expected = self.write_ir(deps / "my_crate-unit.ll", main=True)
            self.write_ir(deps / "my_crate-lib.ll")
            self.write_ir(deps / "integration-test.ll", main=True)

            selected = _find_library_test_harness_llvm_ir(deps, "my_crate")

        self.assertEqual(selected.name, expected.name)

    def test_links_libraries_without_other_harness_or_unwind_runtime(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            deps = Path(temporary)
            harness = self.write_ir(deps / "my_crate-unit.ll", main=True)
            self.write_ir(deps / "integration-test.ll", main=True)
            self.write_ir(deps / "my_crate-lib.ll")
            self.write_ir(deps / "miniz_oxide-provider.ll")
            self.write_ir(deps / "panic_abort-runtime.ll")
            self.write_ir(deps / "panic_unwind-runtime.ll")

            selected = _collect_test_link_llvm_irs(
                deps,
                harness_ir=harness,
                panic_abort=True,
            )

        self.assertEqual(selected[0].name, harness.name)
        self.assertEqual(
            {path.name for path in selected[1:]},
            {
                "miniz_oxide-provider.ll",
                "my_crate-lib.ll",
                "panic_abort-runtime.ll",
            },
        )

    def test_dedupes_stale_hashed_build_std_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            deps = Path(temporary)
            old_alloc = self.write_ir(deps / "alloc-1111111111111111.ll")
            new_alloc = self.write_ir(deps / "alloc-2222222222222222.ll")
            core = self.write_ir(deps / "core-aaaaaaaaaaaaaaaa.ll")
            os.utime(old_alloc, (1, 1))
            os.utime(new_alloc, (2, 2))
            os.utime(core, (1, 1))

            selected = _dedupe_latest_llvm_irs([old_alloc, new_alloc, core])

        self.assertEqual([path.name for path in selected], [
            "alloc-2222222222222222.ll",
            "core-aaaaaaaaaaaaaaaa.ll",
        ])


if __name__ == "__main__":
    unittest.main()
