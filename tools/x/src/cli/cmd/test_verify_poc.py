import json
import tempfile
import unittest
from pathlib import Path

from cli.cmd.verify_poc import (
    INJECTION_STATE_FILE,
    ensure_cargo_feature,
    inject_testcase_at_callsite,
    symbolize_testcase_constants,
    testcase_injection,
)


class VerifyPocInjectionTests(unittest.TestCase):
    def make_crate(self, root: Path) -> None:
        (root / "src").mkdir()
        (root / "Cargo.toml").write_text(
            '[package]\nname = "demo"\nversion = "0.1.0"\nedition = "2021"\n',
            encoding="utf-8",
        )
        (root / "src/lib.rs").write_text("pub fn target() {}\n", encoding="utf-8")

    def test_each_combination_gets_an_independent_feature_and_state_entry(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            crate = Path(tmp)
            self.make_crate(crate)
            target = {"callsite": {"path": "src/lib.rs"}}
            first = testcase_injection("src-lib-rs-1-1", "rule-447")
            second = testcase_injection("src-lib-rs-1-1", "rule-448")
            self.assertNotEqual(first.feature, second.feature)
            self.assertNotEqual(first.function, second.function)

            for injection in (first, second):
                ensure_cargo_feature(crate, injection.feature)
                code = (
                    f'#[cfg(feature = "{injection.feature}")]\n'
                    "#[no_mangle]\n"
                    f'pub extern "C" fn {injection.function}() {{}}\n'
                )
                inject_testcase_at_callsite(
                    crate_dir=crate,
                    target=target,
                    testcase=code,
                    injection=injection,
                )

            manifest = (crate / "Cargo.toml").read_text(encoding="utf-8")
            source = (crate / "src/lib.rs").read_text(encoding="utf-8")
            state = json.loads(
                (crate / INJECTION_STATE_FILE).read_text(encoding="utf-8")
            )
            for injection in (first, second):
                self.assertIn(f"{injection.feature} = []", manifest)
                self.assertIn(
                    f'#[cfg(feature = "{injection.feature}")]', source
                )
                entry = state["injections"][injection.key]
                self.assertEqual(entry["feature"], injection.feature)
                self.assertEqual(entry["function"], injection.function)
                self.assertEqual(entry["source_path"], "src/lib.rs")
                self.assertGreater(entry["line"], 0)

    def test_reinjection_replaces_only_the_matching_combination(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            crate = Path(tmp)
            self.make_crate(crate)
            target = {"callsite": {"path": "src/lib.rs"}}
            injection = testcase_injection("src-lib-rs-1-1", "rule-447")
            ensure_cargo_feature(crate, injection.feature)
            for body in ("let _ = 1;", "let _ = 2;"):
                code = (
                    f'#[cfg(feature = "{injection.feature}")]\n'
                    "#[no_mangle]\n"
                    f'pub extern "C" fn {injection.function}() {{ {body} }}\n'
                )
                inject_testcase_at_callsite(
                    crate_dir=crate,
                    target=target,
                    testcase=code,
                    injection=injection,
                )
            source = (crate / "src/lib.rs").read_text(encoding="utf-8")
            self.assertNotIn("let _ = 1;", source)
            self.assertEqual(source.count("let _ = 2;"), 1)

    def test_injection_uses_unsafe_no_mangle_for_rust_2024(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            crate = Path(tmp)
            self.make_crate(crate)
            manifest = (crate / "Cargo.toml").read_text(encoding="utf-8")
            (crate / "Cargo.toml").write_text(
                manifest.replace('edition = "2021"', 'edition = "2024"'),
                encoding="utf-8",
            )
            target = {"callsite": {"path": "src/lib.rs"}}
            injection = testcase_injection("src-lib-rs-1-1", "rule-447")
            code = (
                f'#[cfg(feature = "{injection.feature}")]\n'
                "#[no_mangle]\n"
                f'pub extern "C" fn {injection.function}() {{}}\n'
            )
            inject_testcase_at_callsite(
                crate_dir=crate,
                target=target,
                testcase=code,
                injection=injection,
            )
            source = (crate / "src/lib.rs").read_text(encoding="utf-8")
            self.assertIn("#[unsafe(no_mangle)]", source)
            self.assertNotIn("\n#[no_mangle]\n", source)

    def test_symbolize_testcase_constants_lifts_scalar_literals_only(self) -> None:
        injection = testcase_injection("src-lib-rs-1-1", "rule-447")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let data = vec![0u8; 8];
    let pair = (1usize, 2usize);
    let first = pair.0;
    let flag = true;
    let ch = 'x';
    let text = "literal 9 stays concrete";
    // comment 10 stays concrete
}}
'''
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
        )
        self.assertEqual(mapping["symbol_count"], 6)
        self.assertIn('klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000', transformed)
        self.assertIn("vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001]", transformed)
        self.assertIn("let pair = (__unsat_rerun_sym_002, __unsat_rerun_sym_003);", transformed)
        self.assertIn("let first = pair.0;", transformed)
        self.assertIn("let flag = __unsat_rerun_sym_004;", transformed)
        self.assertIn("let ch = __unsat_rerun_sym_005;", transformed)
        self.assertIn('"literal 9 stays concrete"', transformed)
        self.assertIn("// comment 10 stays concrete", transformed)

    def test_symbolize_testcase_constants_does_not_bound_large_integers(self) -> None:
        injection = testcase_injection("src-lib-rs-2-1", "rule-447")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let content = vec![0x6A_21_55_79_10_90_32_F3u64; 1];
    let _index = 512usize;
}}
'''
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
        )
        self.assertEqual(mapping["symbol_count"], 3)
        self.assertIsNone(mapping["symbols"][0]["upper_bound"])
        self.assertEqual(mapping["symbols"][1]["upper_bound"], 16)
        self.assertEqual(mapping["symbols"][2]["upper_bound"], 8192)
        self.assertNotIn("122359801931345637168", transformed)
        self.assertIn(
            "let content = vec![__unsat_rerun_sym_000; __unsat_rerun_sym_001];",
            transformed,
        )

    def test_symbolize_testcase_constants_keeps_array_lengths_concrete(self) -> None:
        injection = testcase_injection("src-lib-rs-3-1", "rule-447")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let data = [7u8; 4];
    let mut values: StackVec<[u8; 2]> = StackVec::from_buf([10, 20]);
    values.length = 1;
}}
'''
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
        )
        self.assertEqual(mapping["symbol_count"], 4)
        self.assertIn("[__unsat_rerun_sym_000; 4]", transformed)
        self.assertIn("StackVec<[u8; 2]>", transformed)
        self.assertIn(
            "StackVec::from_buf([__unsat_rerun_sym_001, __unsat_rerun_sym_002])",
            transformed,
        )
        self.assertIn("values.length = __unsat_rerun_sym_003;", transformed)


if __name__ == "__main__":
    unittest.main()
