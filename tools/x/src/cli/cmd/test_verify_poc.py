import json
import tempfile
import unittest
from pathlib import Path

from cli.cmd.verify_poc import (
    INJECTION_STATE_FILE,
    ensure_cargo_feature,
    inject_testcase_at_callsite,
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


if __name__ == "__main__":
    unittest.main()
