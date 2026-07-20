import tempfile
import unittest
from pathlib import Path

from cli.cmd.verify import _missing_callsite_bodies


class VerifyIrAuditTests(unittest.TestCase):
    @staticmethod
    def target(callsite_id: str, line: int) -> dict[str, object]:
        return {
            "caller": {"name": "crate::generic::<T>::call"},
            "callsite": {
                "id": callsite_id,
                "path": "src/lib.rs",
                "line": line,
                "col": 9,
            },
            "callee": {"name": "core::slice::get_unchecked"},
        }

    def test_reports_only_callers_without_an_emitted_marker(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text(
                '@marker = private constant [17 x i8] c"present-callsite\\00"\n',
                encoding="utf-8",
            )
            targets = [
                self.target("present-callsite", 3),
                self.target("missing-callsite", 8),
            ]

            missing = _missing_callsite_bodies(targets=targets, ll_path=ll_path)

        self.assertEqual([row["callsite_id"] for row in missing], ["missing-callsite"])

    def test_debug_definition_proves_body_when_autoinj_marker_is_missing(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text(
                '!1 = !DIFile(filename: "src/lib.rs", directory: "/tmp/crate")\n'
                '!2 = distinct !DISubprogram(name: "call<u8>", file: !1, '
                "line: 4, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition)\n",
                encoding="utf-8",
            )

            missing = _missing_callsite_bodies(
                targets=[self.target("missing-marker", 8)],
                ll_path=ll_path,
            )

        self.assertEqual(missing, [])


if __name__ == "__main__":
    unittest.main()
