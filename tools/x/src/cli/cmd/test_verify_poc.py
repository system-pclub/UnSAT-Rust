import json
import tempfile
import unittest
from pathlib import Path

from cli.cmd.verify_poc import (
    INJECTION_STATE_FILE,
    _prefix_path_obligation_items,
    _source_level_bound_relation_hint,
    build_target_context_block,
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
        self.assertNotIn("klee_ext_bind::assume!", transformed)

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
        self.assertEqual(mapping["symbols"][1]["upper_bound"], 255)
        self.assertEqual(mapping["symbols"][2]["upper_bound"], 8192)
        self.assertNotIn("122359801931345637168", transformed)
        self.assertNotIn("klee_ext_bind::assume!", transformed)
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

    def test_symbolize_testcase_constants_rewrites_symbolic_bytes_from_static(self) -> None:
        injection = testcase_injection("src-lib-rs-3-2", "rule-288")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let bytes = Bytes::from_static(&[
        0xef, 0x00,
    ]);
}}
'''
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
        )
        self.assertEqual(mapping["symbol_count"], 2)
        self.assertIn("Bytes::copy_from_slice(&[", transformed)
        self.assertNotIn("Bytes::from_static(&[\n        __unsat_rerun_sym", transformed)

    def test_prefix_obligations_trace_selected_decode_input(self) -> None:
        prefix = """src/lib.rs:1-8
```rust
fn target(receiver: &mut Receiver) {
    let selector = 0usize;
    let selected = receiver.items.get(selector).cloned().expect("item exists");
    let decoded = Thing::decode(selected.clone()).expect("valid thing");
    unsafe { decoded.ptr.offset(1) };
```
"""
        obligations = "\n".join(_prefix_path_obligation_items(prefix))
        self.assertIn("selected by `receiver.items.get(selector).expect(...)`", obligations)
        self.assertIn("valid encoded `Thing` bytes", obligations)
        self.assertIn("exact selected collection element", obligations)
        self.assertIn("Default", obligations)

    def test_symbolize_testcase_constants_uses_focus_text(self) -> None:
        injection = testcase_injection("src-lib-rs-4-1", "rule-576")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let v0 = DVertex::new(0, IntPoint::new(10, 20));
    let t0 = DTriangle::abc_bc_ac_ab(0, v0, v0, v0, 1, 1, 1);
    let mut triangles = vec![t0];
    let mut d = Delaunay {{ triangles }};
    d.build();
}}
'''
        focus = """
        Actual unsafe-call containing function prefix:
        pub fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
        Derived source-level target relation from the DSL and autoinj bound-argument map:
        construct caller state so `abc.index >= (self.triangles).len() as u64` is true.
        """
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
            focus_text=focus,
        )
        self.assertTrue(mapping["focused"])
        self.assertIn("DTriangle", mapping["focus_terms"])
        self.assertEqual(mapping["symbol_count"], 4)
        self.assertIn("DVertex::new(0, IntPoint::new(10, 20))", transformed)
        self.assertIn(
            "DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_000, v0, v0, v0, __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003)",
            transformed,
        )
        self.assertNotIn("klee_ext_bind::assume!", transformed)

    def test_symbolize_testcase_constants_focuses_multiline_literal_args(self) -> None:
        injection = testcase_injection("src-lib-rs-5-1", "rule-576")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let t0 = DTriangle::abc_bc_ac_ab(
        1,
        DVertex::new(10, IntPoint::new(0, 0)),
        DVertex::new(11, IntPoint::new(1, 0)),
        0,
    );
}}
'''
        focus = """
        pub fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
        Derived source-level target relation from the DSL and autoinj bound-argument map:
        construct caller state so `abc.index >= (self.triangles).len() as u64` is true.
        """
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
            focus_text=focus,
        )
        self.assertEqual(mapping["symbol_count"], 2)
        self.assertIn("        __unsat_rerun_sym_000,", transformed)
        self.assertIn("        __unsat_rerun_sym_001,", transformed)
        self.assertIn("DVertex::new(10, IntPoint::new(0, 0))", transformed)
        self.assertNotIn("klee_ext_bind::assume!", transformed)

    def test_symbolize_testcase_constants_skips_nested_constructor_literals(self) -> None:
        injection = testcase_injection("src-lib-rs-6-1", "rule-576")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let t0 = DTriangle::abc_bc_ac_ab(1, DVertex::new(10, IntPoint::new(0, 0)), v1, v2, 0, 0, 0);
}}
'''
        focus = """
        pub fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
        Derived source-level target relation from the DSL and autoinj bound-argument map:
        construct caller state so `abc.index >= (self.triangles).len() as u64` is true.
        """
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
            focus_text=focus,
        )
        self.assertEqual(mapping["symbol_count"], 4)
        self.assertIn("DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_000", transformed)
        self.assertIn("DVertex::new(10, IntPoint::new(0, 0))", transformed)
        self.assertIn(", __unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003)", transformed)

    def test_symbolize_testcase_constants_uses_actual_prefix_point_fields(self) -> None:
        injection = testcase_injection("src-lib-rs-6-2", "rule-576")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let a = DVertex::new(0, IntPoint::new(0, 0));
    let b = DVertex::new(1, IntPoint::new(1, 0));
    let t0 = DTriangle::abc_bc_ac_ab(1, a, b, a, 0, 0, 0);
    let mut triangles = vec![t0];
}}
'''
        focus = """
Actual unsafe-call containing function prefix. Preserve the safe preconditions needed to execute from the start of this function to the target call:
```rust
fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
    let is_pass = Self::condition(p.point, c.point, a.point, b.point);
    return if is_pass { false } else { true };
}
```
Derived source-level target relation from the DSL and autoinj bound-argument map:
construct caller state so `abc.index >= (self.triangles).len() as u64` is true.
"""
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
            focus_text=focus,
        )
        self.assertIn("IntPoint", mapping["focus_terms"])
        self.assertIn(
            "DVertex::new(__unsat_rerun_sym_000, IntPoint::new(__unsat_rerun_sym_001, __unsat_rerun_sym_002))",
            transformed,
        )
        self.assertIn(
            "DVertex::new(__unsat_rerun_sym_003, IntPoint::new(__unsat_rerun_sym_004, __unsat_rerun_sym_005))",
            transformed,
        )
        self.assertIn(
            "DTriangle::abc_bc_ac_ab(__unsat_rerun_sym_006, a, b, a, __unsat_rerun_sym_007, __unsat_rerun_sym_008, __unsat_rerun_sym_009)",
            transformed,
        )
        self.assertNotIn("klee_ext_bind::assume!", transformed)

    def test_symbolize_testcase_constants_focuses_constructed_type_field_writes(self) -> None:
        injection = testcase_injection("src-lib-rs-7-1", "rule-576")
        testcase = f'''#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let v0 = DVertex::new(0, IntPoint::new(10, 20));
    let mut t0 = DTriangle::abc(0, v0, v0, v0);
    t0.neighbors = [1, 1, 1];
    let mut other = 7usize;
    other = 8;
}}
'''
        focus = """
        pub fn swap(&mut self, abc: DTriangle, pbc: DTriangle) -> bool {
        Derived source-level target relation from the DSL and autoinj bound-argument map:
        construct caller state so `abc.index >= (self.triangles).len() as u64` is true.
        """
        transformed, mapping = symbolize_testcase_constants(
            testcase=testcase,
            injection=injection,
            focus_text=focus,
        )
        self.assertEqual(mapping["symbol_count"], 4)
        self.assertIn("DTriangle::abc(__unsat_rerun_sym_000", transformed)
        self.assertIn(
            "t0.neighbors = [__unsat_rerun_sym_001, __unsat_rerun_sym_002, __unsat_rerun_sym_003];",
            transformed,
        )
        self.assertIn("let mut other = 7usize;", transformed)
        self.assertIn("other = 8;", transformed)

    def test_target_context_includes_wrapper_argument_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            crate = Path(tmp)
            (crate / "src").mkdir()
            source = """\
pub struct Demo { pub items: Vec<Node> }
#[derive(Clone, Copy)]
pub struct Node { pub index: usize }
impl Demo {
    pub fn build(&mut self) {
        let current = self.items[0];
        let other = self.items[1];
        if self.swap(current, other) {}
    }

    fn swap(&mut self, abc: Node, pbc: Node) -> bool {
        unsafe {
            *self.items.get_unchecked_mut(abc.index) = pbc;
        }
        true
    }
}
"""
            (crate / "src/lib.rs").write_text(source, encoding="utf-8")
            lines = source.splitlines()
            root_line = lines.index("        if self.swap(current, other) {}") + 1
            unsafe_line = (
                lines.index("            *self.items.get_unchecked_mut(abc.index) = pbc;")
                + 1
            )
            context = build_target_context_block(
                crate_dir=crate,
                target={
                    "caller": {"name": "demo::Demo::build"},
                    "callee": {"name": "core::slice::<impl [T]>::get_unchecked_mut"},
                    "callsite": {
                        "path": "src/lib.rs",
                        "line": root_line,
                        "col": 17,
                    },
                    "unsafe_callsite": {
                        "path": "src/lib.rs",
                        "line": unsafe_line,
                        "col": 25,
                    },
                },
            )
            self.assertIn("Wrapper-to-actual argument bridge", context)
            self.assertIn("Direct inner-target-containing safe function option", context)
            self.assertIn("fn swap(&mut self, abc: Node, pbc: Node)", context)
            self.assertIn("actual parameter `abc` comes from root call argument `current`", context)
            self.assertIn(
                "`abc.index` at the inner target call is the `index` field of root-side `current`",
                context,
            )

    def test_target_context_includes_safe_prefix_helper_sources(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            crate = Path(tmp)
            (crate / "src").mkdir()
            source = """\
pub struct Demo { pub items: Vec<Node> }
#[derive(Clone, Copy)]
pub struct Node { pub index: usize, pub links: [usize; 3] }
impl Node {
    pub fn opposite(&self, neighbor: usize) -> usize {
        for i in 0..3 {
            if self.links[i] == neighbor {
                return i;
            }
        }
        panic!("missing");
    }
}
impl Demo {
    pub fn build(&mut self) {
        let current = self.items[0];
        let other = self.items[1];
        if self.swap(current, other) {}
    }

    fn swap(&mut self, abc: Node, pbc: Node) -> bool {
        let order = pbc.opposite(abc.index);
        if Self::condition(order) {
            return false;
        }
        unsafe {
            *self.items.get_unchecked_mut(abc.index) = pbc;
        }
        true
    }

    fn condition(order: usize) -> bool {
        order == 0
    }
}
"""
            (crate / "src/lib.rs").write_text(source, encoding="utf-8")
            lines = source.splitlines()
            root_line = lines.index("        if self.swap(current, other) {}") + 1
            unsafe_line = (
                lines.index("            *self.items.get_unchecked_mut(abc.index) = pbc;")
                + 1
            )
            context = build_target_context_block(
                crate_dir=crate,
                target={
                    "caller": {"name": "demo::Demo::build"},
                    "callee": {"name": "core::slice::<impl [T]>::get_unchecked_mut"},
                    "callsite": {
                        "path": "src/lib.rs",
                        "line": root_line,
                        "col": 17,
                    },
                    "unsafe_callsite": {
                        "path": "src/lib.rs",
                        "line": unsafe_line,
                        "col": 25,
                    },
                },
            )
            self.assertIn(
                "Relevant macro/helper definitions used by the safe prefix",
                context,
            )
            self.assertIn("pub fn opposite(&self, neighbor: usize) -> usize", context)
            self.assertIn("fn condition(order: usize) -> bool", context)
            self.assertIn(
                "construct `pbc.links` so it contains `abc.index`",
                context,
            )
            self.assertIn(
                "construct input/receiver fields so `Self::condition(order)` returns false",
                context,
            )

    def test_target_context_includes_instrumented_bound_arg_map(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source_crate = root / "source"
            injected_crate = root / "injected"
            (source_crate / "src").mkdir(parents=True)
            (injected_crate / "src").mkdir(parents=True)
            source = """\
pub struct Demo { pub items: Vec<Node> }
#[derive(Clone, Copy)]
pub struct Node { pub index: usize }
impl Demo {
    pub fn build(&mut self) {
        let current = self.items[0];
        let other = self.items[1];
        if self.swap(current, other) {}
    }

    fn swap(&mut self, abc: Node, pbc: Node) -> bool {
        unsafe {
            *self.items.get_unchecked_mut(abc.index) = pbc;
        }
        true
    }
}
"""
            injected = source.replace(
                "            *self.items.get_unchecked_mut(abc.index) = pbc;",
                """\
            let __klee_arg0 = abc.index;
            klee_ext_bind::bind_arg_u64(1, (self.items).len() as u64);
            klee_ext_bind::bind_arg_u64_value(2, &__klee_arg0);
            klee_ext_bind::callsite!("src-lib-rs-8-17");
            *self.items.get_unchecked_mut(__klee_arg0) = pbc;""",
            )
            (source_crate / "src/lib.rs").write_text(source, encoding="utf-8")
            (injected_crate / "src/lib.rs").write_text(injected, encoding="utf-8")
            lines = source.splitlines()
            root_line = lines.index("        if self.swap(current, other) {}") + 1
            unsafe_line = (
                lines.index("            *self.items.get_unchecked_mut(abc.index) = pbc;")
                + 1
            )
            context = build_target_context_block(
                crate_dir=source_crate,
                instrumented_crate_dir=injected_crate,
                target={
                    "caller": {"name": "demo::Demo::build"},
                    "callee": {"name": "core::slice::<impl [T]>::get_unchecked_mut"},
                    "callsite": {
                        "path": "src/lib.rs",
                        "line": root_line,
                        "col": 17,
                        "id": "src-lib-rs-8-17",
                    },
                    "unsafe_callsite": {
                        "path": "src/lib.rs",
                        "line": unsafe_line,
                        "col": 25,
                    },
                },
            )
            self.assertIn("Instrumented bound-argument map from autoinj", context)
            self.assertIn("bind_arg_u64(1, (self.items).len() as u64)", context)
            self.assertIn("bind_arg_u64_value(2, &__klee_arg0)", context)
            relation = _source_level_bound_relation_hint(
                target_context=context,
                klee_witness=(
                    "Witness-derived target relation: the rule is "
                    "`get_arg(2) < get_arg(1)`, so the testcase should make "
                    "`get_arg(2) >= get_arg(1)` true at the target call."
                ),
            )
            self.assertIsNotNone(relation)
            self.assertIn("abc.index >= (self.items).len() as u64", relation)

    def test_source_level_relation_uses_rule_ast_when_init_has_no_witness_values(self) -> None:
        context = """
Instrumented bound-argument map from autoinj:
274:                 let __klee_arg11 = abc.index;
275:                 klee_ext_bind::bind_arg_u64(1, (self.triangles).len() as u64);
276:                 klee_ext_bind::bind_arg_u64_value(2, &__klee_arg11);
277:                 klee_ext_bind::callsite!("src-delaunay-delaunay-rs-85-24");
"""
        witness = """Target callsite id: src-delaunay-delaunay-rs-85-24

Rule id: rule-576

Rule DSL AST. The testcase should make NOT(this rule) true at the target unsafe call:
{"simplified":{"type":"binary","op":"<","left":{"type":"simplified_var","name":"get_arg(2)"},"right":{"type":"simplified_var","name":"get_arg(1)"}},"original":{"type":"binary","op":"<","left":{"type":"call","name":"get_arg","args":[{"type":"literal","value":2}]},"right":{"type":"call","name":"get_arg","args":[{"type":"literal","value":1}]}}}
"""
        relation = _source_level_bound_relation_hint(
            target_context=context,
            klee_witness=witness,
        )
        self.assertIsNotNone(relation)
        self.assertIn("abc.index >= (self.triangles).len() as u64", relation)


if __name__ == "__main__":
    unittest.main()
