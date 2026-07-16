import tempfile
import unittest
from pathlib import Path

from llm.gen_example import build_witness_guided_rust_context


class WitnessGuidedRustContextTests(unittest.TestCase):
    def make_crate(self, root: Path) -> dict[str, object]:
        (root / "src").mkdir()
        (root / "tests").mkdir()
        (root / "Cargo.toml").write_text(
            '[package]\nname = "demo"\nversion = "0.1.0"\nedition = "2021"\n',
            encoding="utf-8",
        )
        (root / "src/lib.rs").write_text(
            """mod other;

use core::marker::PhantomData;

pub struct State<T> {
    pub index: usize,
    marker: PhantomData<T>,
    choice: other::Choice,
}

impl<T> State<T> {
    pub fn mutate(&mut self, index: usize) {
        self.index = index;
    }

    pub fn irrelevant(&self) -> usize {
        let text = "a brace in a string: }";
        // A comment containing { must not confuse brace matching.
        self.index + text.len()
    }

    pub fn target(&self) -> usize {
        self.index
    }
}

// UNSAT-GENERATED-TESTCASE-BEGIN:old::rule
fn old_generated_poc() { panic!("do not retain me") }
// UNSAT-GENERATED-TESTCASE-END:old::rule
""",
            encoding="utf-8",
        )
        (root / "src/other.rs").write_text(
            "pub enum Choice { A, B }\n\npub fn helper() -> usize { 99 }\n",
            encoding="utf-8",
        )
        (root / "tests/external.rs").write_text(
            "fn expensive_test_body() { panic!(\"not library context\") }\n",
            encoding="utf-8",
        )
        return {
            "caller": {"name": "crate::State::<T>::target"},
            "callsite": {"path": "src/lib.rs", "line": 23},
        }

    def test_slice_retains_shapes_and_semantic_bodies_only(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            target = self.make_crate(root)
            chains = [
                {
                    "steps": [
                        {"kind": "mut_ref_escape", "function": "crate::State::mutate"}
                    ]
                }
            ]

            context = build_witness_guided_rust_context(
                root, target=target, control_chains=chains
            )

            self.assertIn("pub struct State<T>", context.text)
            self.assertIn("pub enum Choice", context.text)
            self.assertIn("self.index = index;", context.text)
            self.assertIn("pub fn target(&self)", context.text)
            self.assertIn("self.index\n", context.text)
            self.assertIn("pub fn irrelevant(&self) -> usize", context.text)
            self.assertNotIn("a brace in a string", context.text)
            self.assertIn("pub fn helper() -> usize", context.text)
            self.assertNotIn("99", context.text)
            self.assertNotIn("expensive_test_body", context.text)
            self.assertNotIn("old_generated_poc", context.text)
            # The fixed XML/policy wrapper can outweigh savings in a tiny
            # synthetic crate; real crates are covered by measured regressions.
            self.assertGreater(context.stats["baseline_chars"], 0)
            self.assertEqual(context.stats["mode"], "slice")
            retained = context.stats["retained_function_bodies"]["src/lib.rs"]
            self.assertEqual(retained, ["mutate", "target"])

    def test_full_mode_preserves_historical_all_rust_files_context(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            target = self.make_crate(root)
            context = build_witness_guided_rust_context(
                root, target=target, control_chains=[], mode="full"
            )

            self.assertIn("expensive_test_body", context.text)
            self.assertIn("a brace in a string", context.text)
            self.assertEqual(context.stats["char_reduction_ratio"], 0.0)
            self.assertEqual(context.stats["mode"], "full")

    def test_rejects_unknown_context_mode(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            target = self.make_crate(root)
            with self.assertRaisesRegex(ValueError, "unsupported Rust context mode"):
                build_witness_guided_rust_context(
                    root, target=target, control_chains=[], mode="unknown"
                )


if __name__ == "__main__":
    unittest.main()
