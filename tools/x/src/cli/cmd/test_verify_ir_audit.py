import tempfile
import unittest
from pathlib import Path

from cli.cmd.verify import (
    _compose_has_actionable_certainty,
    _control_chain_all_certain,
    _find_target,
    _matrix_callsite_rows,
    _missing_callsite_bodies,
    _resolve_callsite_marker_for_ir,
)


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

    def test_structured_certainty_requires_every_dependency_to_be_certain(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            chain_path = Path(temporary) / "chain.json"
            chain_path.write_text(
                '{"all_certain": false, "symbols": ['
                '{"certainty": "certain_symbol"}, '
                '{"certainty": "uncertain_symbol"}]}'
            )

            verdict = _control_chain_all_certain(chain_path)

        self.assertIs(verdict, False)

    def test_legacy_certainty_data_has_no_authoritative_verdict(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            chain_path = Path(temporary) / "chain.json"
            chain_path.write_text('{"symbols": [{"certainty": "certain_symbol"}]}')

            verdict = _control_chain_all_certain(chain_path)

        self.assertIsNone(verdict)

    def test_uncertain_chain_overrides_certain_input_trace(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            chain_path = Path(temporary) / "chain.json"
            chain_path.write_text('{"all_certain": false}')
            trace = "[ext.dsl] violation query uses certain symbol: true"

            actionable = _compose_has_actionable_certainty(
                chain_path, trace, raw_ptr_deref=False
            )

        self.assertFalse(actionable)

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

    def test_find_target_prefers_specific_unsafe_marker_alias(self) -> None:
        base = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "callee": {
                "name": "core::slice::<impl [T]>::get_unchecked_mut",
                "path": "rust/library/core/src/slice/mod.rs",
                "line_start": 665,
            },
        }
        specific = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8-get_unchecked_mut",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "callee": {
                "name": "core::slice::<impl [T]>::get_unchecked_mut",
                "path": "rust/library/core/src/slice/mod.rs",
                "line_start": 665,
            },
        }

        target, callsite_id = _find_target([base, specific], "src-lib-rs-10-8")

        self.assertIs(target, specific)
        self.assertEqual(callsite_id, "src-lib-rs-10-8-get_unchecked_mut")

    def test_find_target_keeps_exact_target_when_actual_unsafe_callsite_is_known(self) -> None:
        base = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "unsafe_callsite": {
                "path": "src/lib.rs",
                "line": 20,
                "col": 12,
            },
            "callee": {
                "name": "core::slice::<impl [T]>::get_unchecked_mut",
                "path": "rust/library/core/src/slice/mod.rs",
                "line_start": 665,
            },
        }
        specific = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8-get_unchecked_mut",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "unsafe_callsite": {
                "path": "src/lib.rs",
                "line": 21,
                "col": 12,
            },
            "callee": {
                "name": "core::slice::<impl [T]>::get_unchecked_mut",
                "path": "rust/library/core/src/slice/mod.rs",
                "line_start": 665,
            },
        }

        target, callsite_id = _find_target([base, specific], "src-lib-rs-10-8")

        self.assertIs(target, base)
        self.assertEqual(callsite_id, "src-lib-rs-10-8")

    def test_matrix_does_not_fallback_to_marker_for_different_callee(self) -> None:
        base = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "callee": {
                "name": "core::ptr::const_ptr::<impl *const T>::offset_from",
                "path": "rust/library/core/src/ptr/const_ptr.rs",
                "line_start": 637,
            },
        }
        read_unaligned = {
            "caller": {"name": "crate::call"},
            "callsite": {
                "id": "src-lib-rs-10-8-read_unaligned",
                "path": "src/lib.rs",
                "line": 10,
                "col": 8,
            },
            "callee": {
                "name": "core::ptr::const_ptr::<impl *const T>::read_unaligned",
                "path": "rust/library/core/src/ptr/const_ptr.rs",
                "line_start": 1295,
            },
        }

        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text('@marker = private constant [18 x i8] c"src-lib-rs-10-8\\00"\n')
            rows = _matrix_callsite_rows(
                targets=[base, read_unaligned],
                ll_path=ll_path,
                requested_callsite=None,
            )
            resolved = _resolve_callsite_marker_for_ir(
                ll_path=ll_path,
                target=read_unaligned,
                callsite_id="src-lib-rs-10-8-read_unaligned",
                targets=[base, read_unaligned],
            )

        self.assertEqual(rows[0]["llvm_callsite_id"], "src-lib-rs-10-8")
        self.assertTrue(rows[0]["present_in_llvm_ir"])
        self.assertEqual(rows[1]["llvm_callsite_id"], "src-lib-rs-10-8-read_unaligned")
        self.assertFalse(rows[1]["present_in_llvm_ir"])
        self.assertEqual(resolved, "src-lib-rs-10-8-read_unaligned")

    def test_matrix_rejects_marker_inside_shared_helper_not_metadata_caller(self) -> None:
        target = {
            "caller": {"name": "instructions::bitwise::iszero"},
            "callsite": {
                "id": "src-instructions-bitwise-rs-41-5",
                "path": "src/instructions/bitwise.rs",
                "line": 41,
                "col": 5,
            },
            "callee": {
                "name": "core::slice::<impl [T]>::get_unchecked_mut",
                "path": "rust/library/core/src/slice/mod.rs",
                "line_start": 665,
            },
        }

        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text(
                '@site = private constant [33 x i8] '
                'c"src-instructions-bitwise-rs-41-5\\00"\n'
                "define ptr @top_unsafe(ptr %self) !dbg !2 {\n"
                "entry:\n"
                "  call void @klee_ext_callsite(ptr @site)\n"
                "  ret ptr %self\n"
                "}\n"
                "declare void @klee_ext_callsite(ptr)\n"
                '!1 = !DIFile(filename: "src/interpreter/stack.rs", directory: "/tmp/crate")\n'
                '!2 = distinct !DISubprogram(name: "top_unsafe", file: !1, '
                "line: 92, spFlags: DISPFlagDefinition)\n",
                encoding="utf-8",
            )
            rows = _matrix_callsite_rows(
                targets=[target],
                ll_path=ll_path,
                requested_callsite=None,
            )

        self.assertEqual(rows[0]["llvm_callsite_id"], "src-instructions-bitwise-rs-41-5")
        self.assertFalse(rows[0]["present_in_llvm_ir"])

    def test_matrix_rejects_outer_callsite_marker_inside_unsafe_helper(self) -> None:
        target = {
            "caller": {"name": "instructions::contract::pop_extcall_target_address"},
            "callsite": {
                "id": "src-instructions-contract-rs-205-5",
                "path": "src/instructions/contract.rs",
                "line": 205,
                "col": 5,
            },
            "unsafe_callsite": {
                "path": "src/interpreter/stack.rs",
                "line": 93,
                "col": 9,
            },
            "callee": {"name": "std::option::Option::<T>::unwrap_unchecked"},
        }

        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text(
                '@site = private constant [37 x i8] '
                'c"src-instructions-contract-rs-205-5\\00"\n'
                "define i256 @pop_unsafe(ptr %self) !dbg !2 {\n"
                "entry:\n"
                "  call void @klee_ext_callsite(ptr @site)\n"
                "  ret i256 0\n"
                "}\n"
                "declare void @klee_ext_callsite(ptr)\n"
                '!1 = !DIFile(filename: "src/interpreter/stack.rs", directory: "/tmp/crate")\n'
                '!2 = distinct !DISubprogram(name: "pop_unsafe", file: !1, '
                "line: 79, spFlags: DISPFlagDefinition)\n",
                encoding="utf-8",
            )
            rows = _matrix_callsite_rows(
                targets=[target],
                ll_path=ll_path,
                requested_callsite=None,
            )

        self.assertEqual(rows[0]["llvm_callsite_id"], "src-instructions-contract-rs-205-5")
        self.assertFalse(rows[0]["present_in_llvm_ir"])

    def test_matrix_accepts_marker_inside_metadata_caller(self) -> None:
        target = {
            "caller": {"name": "instructions::bitwise::iszero"},
            "callsite": {
                "id": "src-instructions-bitwise-rs-41-5",
                "path": "src/instructions/bitwise.rs",
                "line": 41,
                "col": 5,
            },
            "callee": {"name": "core::slice::<impl [T]>::get_unchecked_mut"},
        }

        with tempfile.TemporaryDirectory() as temporary:
            ll_path = Path(temporary) / "crate.ll"
            ll_path.write_text(
                '@site = private constant [33 x i8] '
                'c"src-instructions-bitwise-rs-41-5\\00"\n'
                "define void @iszero(ptr %interp) !dbg !2 {\n"
                "entry:\n"
                "  call void @klee_ext_callsite(ptr @site)\n"
                "  ret void\n"
                "}\n"
                "declare void @klee_ext_callsite(ptr)\n"
                '!1 = !DIFile(filename: "src/instructions/bitwise.rs", directory: "/tmp/crate")\n'
                '!2 = distinct !DISubprogram(name: "iszero", file: !1, '
                "line: 32, spFlags: DISPFlagDefinition)\n",
                encoding="utf-8",
            )
            rows = _matrix_callsite_rows(
                targets=[target],
                ll_path=ll_path,
                requested_callsite=None,
            )

        self.assertTrue(rows[0]["present_in_llvm_ir"])


if __name__ == "__main__":
    unittest.main()
