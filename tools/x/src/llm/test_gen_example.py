import tempfile
import unittest
from pathlib import Path

from llm.gen_example import (
    build_testcase_prompt,
    build_witness_guided_rust_context,
    crate_api_slice_function_names,
    render_crate_api_slice,
)


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

    def test_testcase_prompt_prioritizes_direct_actual_safe_function(self) -> None:
        prompt = build_testcase_prompt(
            rust_context="<rust context/>",
            call_chain="[]",
            callsite="{}",
            safety_requirement="demo rule",
            function_name="__unsat_poc_demo",
            feature_name="unsat-poc-demo",
            target_context=(
                "Direct actual-containing safe function option. "
                "The actual unsafe call is inside safe function `fn swap(...)`."
            ),
        )
        self.assertIn("Direct actual-containing safe function", prompt)
        self.assertIn("use that direct function", prompt)
        self.assertIn("Do not fall back to the root wrapper merely", prompt)

    def test_testcase_prompt_mentions_no_std_only_when_requested(self) -> None:
        kwargs = dict(
            rust_context="<rust context/>",
            call_chain="[]",
            callsite="{}",
            safety_requirement="demo rule",
            function_name="__unsat_poc_demo",
            feature_name="unsat-poc-demo",
        )
        self.assertIn("`#![no_std]`", build_testcase_prompt(**kwargs, no_std=True))
        self.assertNotIn("`#![no_std]`", build_testcase_prompt(**kwargs))

    def test_testcase_prompt_is_compact_and_keeps_core_requirements(self) -> None:
        prompt = build_testcase_prompt(
            rust_context="<rust context/>",
            call_chain="a -> b",
            callsite="{}",
            safety_requirement="demo rule",
            function_name="__unsat_poc_demo",
            feature_name="unsat-poc-demo",
        )
        self.assertLess(len(prompt), 9000)
        self.assertEqual(prompt.count('#[cfg(feature = "unsat-poc-demo")]'), 1)
        for required in ("#[no_mangle]", "cfg(test)", "one Rust fenced code block"):
            self.assertIn(required, prompt)

    def test_api_slice_function_names_cover_target_and_chain(self) -> None:
        names = crate_api_slice_function_names(
            {"caller": {"name": "c::T::target"}},
            [{"steps": [{"function": "c::T::new"}, {"function": "c::T::target"}]}],
        )
        self.assertEqual(names, ["c::T::new", "c::T::target"])

    def test_render_api_slice_keeps_bodies_for_chain_and_signatures_for_apis(self) -> None:
        context = render_crate_api_slice(
            {
                "types": [
                    {"name": "c::T", "path": "src/lib.rs", "line": 1, "definition": "struct T;"}
                ],
                "functions": [
                    {"name": "c::T::target", "role": "requested", "path": "src/lib.rs",
                     "line": 3, "signature": "fn target()", "body": "fn target() { 1 }"},
                    {"name": "c::T::api", "role": "related_type_api", "path": "src/lib.rs",
                     "line": 5, "signature": "pub fn api()"},
                ],
            }
        )
        self.assertIn("struct T;", context.text)
        self.assertIn("fn target() { 1 }", context.text)
        self.assertIn("pub fn api()", context.text)
        self.assertEqual(context.stats["retained_type_count"], 1)


if __name__ == "__main__":
    unittest.main()
