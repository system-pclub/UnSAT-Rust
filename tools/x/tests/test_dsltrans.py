"""Run in Docker: python -m unittest discover -s tools/x/tests -p test_dsltrans.py -v."""
import argparse
import ast
import csv
import json
from pathlib import Path
import tempfile
import unittest
from types import SimpleNamespace
from unittest.mock import patch

import dsltrans


class CatalogTranslationChecks(unittest.TestCase):
    def model(self, api="", rule=""):
        return dsltrans.Model(dsltrans.ROOT / "tools/x/generated/dsl", api, rule)

    def test_common_catalog_contains_only_library_definitions(self):
        m = self.model()
        self.assertEqual(m.schema["version"], 2)
        self.assertNotIn("roots", m.schema)
        full = json.loads(m.catalog_bytes)["definitions"]
        for name in ["core::alloc::layout::Layout", "alloc::vec::Vec", "alloc::string::String", "std::fs::File"]:
            self.assertIn(name, full)
        self.assertTrue(all(n.startswith(("core::", "alloc::", "std::")) for n in full))
        self.assertFalse(any("/compiler/" in f for f in json.loads(m.catalog_bytes)["files"]))

    def test_layout_members_have_exact_source_evidence(self):
        m = self.model("fn alloc(layout: core::alloc::Layout)")
        size = next(e for e in m.schema["selectors"]["core::alloc::layout::Layout"] if e["syntax"] == ".size")
        self.assertTrue(size["source"]["file"].endswith("library/core/src/alloc/layout.rs"))
        m.validate("$1.size > 0")
        node = ast.parse(m.interface.rewrite("$1.align[0]"), mode="eval").body
        m.infer(node, m.environment(), set())
        for expr in ["$1.minimum_size > 0", "$1.size", "$1.size < '0'", "$1.size() > 0", "layout.size > 0", "$2.size > 0"]:
            with self.subTest(expr=expr), self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)

    def test_stdlib_slice_preserves_dependencies_and_is_api_specific(self):
        api = "fn alloc(layout: core::alloc::Layout)"
        m = self.model(api)
        self.assertIn("core::alloc::layout::Layout", m.schema["definitions"])
        self.assertNotIn("std::fs::File", m.schema["definitions"])
        self.assertTrue(all(n.startswith(("core::", "alloc::", "std::")) for n in m.schema["definitions"]))
        self.assertEqual(m.grammar, self.model(api, "unrelated Vec and File words").grammar)
        self.assertNotIn("rustc_hir", m.grammar)
        array = self.model("core::array::IntoIter::<T,N>::new_unchecked(buffer: [T; N], initialized: core::ops::Range<usize>)")
        self.assertIn("core::array::iter::IntoIter", array.schema["selection"]["stdlib_seed_types"])
        self.assertNotIn("std::collections::hash::map::IntoIter", array.schema["selection"]["stdlib_seed_types"])

    def test_range_substitution_and_const_generic_value(self):
        m = self.model("core::array::IntoIter::<T,N>::new_unchecked(buffer: [T; N], initialized: core::ops::Range<usize>)")
        self.assertEqual([(v.symbol, v.source_name, v.kind) for v in m.interface.variables], [("$1", "buffer", "argument"), ("$2", "initialized", "argument"), ("$3", "T", "type_parameter"), ("$4", "N", "const_parameter")])
        m.validate("$2.start <= $2.end")
        m.validate("$2.end <= $4")
        m.validate("$2.end <= len($1)")
        with self.assertRaises(dsltrans.ValidationError):
            m.validate("$2.end <= $3")  # T metadata is not a numeric const.

    def test_arrays_are_language_types_and_comprehension_locals_are_scoped(self):
        m = self.model("impl<const N: usize> [u8; N] { fn check(&self) -> bool }")
        expr = "all([0 <= byte and byte <= 127 for byte in $1])"
        m.prompt_grammar.parse(expr)
        m.validate(expr)
        self.assertEqual([v.source_name for v in m.interface.variables], ["self", "N", "result"])
        for expr in ["$1.elements[0] < 128", "all([byte.initialized for byte in $1])", "all([byte for byte in $1])", "all(byte < 128 for byte in $1)", "byte < 128", "all([$1 < 128 for $1 in $1])"]:
            with self.subTest(expr=expr), self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)

    def test_api_array_tuple_and_return_entries(self):
        m = self.model("fn f(buffer: &[u8; 37], pair: (u8, usize)) -> usize")
        self.assertTrue(any("[u8; 37]" in text for text in m.prompt_grammar.comments.values()))
        m.validate("$1[0] <= $2[1] and $3 >= 0")
        self.assertEqual(m.interface.variables[-1].kind, "return_value")

    def test_library_enum_payloads_still_require_guards(self):
        m = self.model("fn f(value: core::option::Option<(usize, u8)>)")
        m.validate('not is_variant($1, "Some") or $1.Some[0][0] >= 0')
        for expr in ['$1.Some[0][0] >= 0', 'is_variant($1, "Some") and $1.Some[0][1] > "0"', '$1.kind > 0']:
            with self.subTest(expr=expr), self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)

    def test_generic_parameters_are_opaque_without_compiler_metadata(self):
        m = self.model("fn f<T>(value: T)")
        self.assertEqual(m.interface.variables[1].source_name, "T")
        self.assertEqual(m.interface.variables[1].dsl_type, {"kind": "opaque", "rust": "type parameter T"})
        self.assertNotIn("rustc_hir", m.grammar)
        for expr in ['$2.kind == "Array"', '$2.size > 0', '$1 > 0']:
            with self.subTest(expr=expr), self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)

    def test_missing_runtime_properties_are_not_invented(self):
        m = self.model("fn f(ptr: core::ptr::NonNull<u8>)")
        for expr in ["$1.allocation.live", "$1.minimum_size > 0"]:
            with self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)

    def test_no_execution_named_api_variables_or_internal_name_bypass(self):
        m = self.model("fn f(layout: core::alloc::Layout)")
        for expr in ["__import__('os').system('true') == 0", "(lambda: True)()", "get_field($1, 'size') > 0", "layout.size > 0", "__dsl_slot_1.size > 0", "$0.size > 0", "$01.size > 0"]:
            with self.subTest(expr=expr), self.assertRaises(dsltrans.ValidationError):
                m.validate(expr)
        m.validate('"$1" == "$1" and $1.size >= 0')

    def test_sample_requirements_are_real_and_share_one_catalog(self):
        cases = json.loads((dsltrans.ROOT / "tools/x/dsltrans_samples.json").read_text())
        with (dsltrans.ROOT / "rules.csv").open(newline="") as f:
            catalog = {r["id"]: r["rule"] for r in csv.DictReader(f)}
        self.assertEqual(len(cases), 10)
        for case in cases:
            self.assertEqual(case["rule"], catalog[case["id"]])
            self.assertNotIn("dslgen", case)
            self.assertNotIn("expected_status", case)
            self.model(case["api"], case["rule"])

    def test_dry_run_and_model_cannot_declare_bindings(self):
        case = json.loads((dsltrans.ROOT / "tools/x/dsltrans_samples.json").read_text())[0]
        args = argparse.Namespace(model="gpt-6.1-sol", reasoning_effort="medium", max_output_tokens=6000, dslgen=str(dsltrans.ROOT / "tools/x/generated/dsl"), dry_run=True, resume=False)
        with tempfile.TemporaryDirectory() as directory:
            result = dsltrans.run_case(case, args, Path(directory), None)
            request = json.loads((Path(directory) / "request.json").read_text())
            self.assertEqual(result["status"], "prepared")
            self.assertEqual(request["model"], "gpt-6.1-sol")
            self.assertNotIn("text", request)
            self.assertIn("plain text", request["input"][0]["content"])
            self.assertNotIn("response schema", request["input"][0]["content"])
            grammar = (Path(directory) / "grammar.ebnf").read_text()
            self.assertIn("$2 is layout", grammar)
            self.assertNotIn("member<", grammar)
            self.assertFalse((Path(directory) / "hir.ebnf").exists())
            self.assertNotIn('\".\", identifier', grammar)
            self.assertEqual(json.loads((Path(directory) / "api_variables.json").read_text()), result["api_variables"])

    def test_schema_json_is_the_only_required_dslgen_input(self):
        m = self.model("fn f(layout: core::alloc::Layout)")
        with tempfile.TemporaryDirectory() as directory:
            schema = Path(directory) / "schema.json"
            schema.write_bytes(m.schema_bytes)
            isolated = dsltrans.Model(schema, "fn f(layout: core::alloc::Layout)")
            isolated.validate("$1.size > 0")
            self.assertFalse((Path(directory) / "grammar.ebnf").exists())

    def test_plain_response_protocol_and_failure_marker(self):
        self.assertEqual(dsltrans.parse_model_output(" $2.end <= $4\n"), {"status": "translated", "expression": "$2.end <= $4"})
        self.assertEqual(dsltrans.parse_model_output("UNEXPRESSIBLE\n"), {"status": "unexpressible", "expression": None})
        self.assertEqual(dsltrans.parse_model_output("False")["status"], "translated")
        for text in ["", " \n"]:
            with self.assertRaises(dsltrans.ValidationError):
                dsltrans.parse_model_output(text)

    def test_model_responses_are_checked_as_raw_dsl(self):
        case = json.loads((dsltrans.ROOT / "tools/x/dsltrans_samples.json").read_text())[0]
        args = argparse.Namespace(model="gpt-6.1-sol", reasoning_effort="medium", max_output_tokens=6000, dslgen=str(dsltrans.ROOT / "tools/x/generated/dsl"), dry_run=False, resume=False, timeout=1)
        cases = [("$2.size > 0\n", "translated"), ("UNEXPRESSIBLE", "unexpressible"), ('{"expression":"$2.size > 0"}', "validation_error"), ("```text\n$2.size > 0\n```", "validation_error"), ("UNEXPRESSIBLE\n$2.size > 0", "validation_error")]
        for raw, expected in cases:
            with self.subTest(raw=raw), tempfile.TemporaryDirectory() as directory, patch("openai.OpenAI") as client:
                response = SimpleNamespace(output_text=raw, model="gpt-6.1-sol", id="test-response", status="completed", usage=None, model_dump=lambda mode: {"output_text": raw, "status": "completed"})
                client.return_value.__enter__.return_value.responses.create.return_value = response
                result = dsltrans.run_case(case, args, Path(directory), "test-key")
                self.assertEqual(result["status"], expected)
                self.assertEqual((Path(directory) / "response.txt").read_text(), raw)
                if expected == "translated":
                    self.assertEqual(result["expression"], "$2.size > 0")
                    self.assertEqual(result["validation"]["generated_grammar"], "passed")
                elif expected == "unexpressible":
                    self.assertIsNone(result["expression"])
                    self.assertIn("UNEXPRESSIBLE", dsltrans.markdown([result]))

    def test_generated_parser_rejects_wrong_receiver_fields_directly(self):
        from lark.exceptions import LarkError
        m = self.model("fn f(v: alloc::vec::Vec<u8>, layout: core::alloc::Layout)")
        for grammar in [m.prompt_grammar, m.compile_grammar("$1.len >= $2.size")[0]]:
            grammar.parse("$1.len >= $2.size")
            grammar.parse("($1).len + $2.size > 0")
            for expr in ["$1.size > 0", "$2.len > 0", "$2.foo > 0", "$1.foo > 0", "layout.size > 0", "$3.size > 0", "($2.size).align == 0"]:
                with self.subTest(expr=expr), self.assertRaises(LarkError):
                    grammar.parse(expr)

    def test_generated_parser_follows_nested_member_result_types(self):
        from lark.exceptions import LarkError
        m = self.model("fn f(v: alloc::vec::Vec<u8>)")
        grammar = m.compile_grammar("$1.buf.inner == $1.buf.inner")[0]
        grammar.parse("$1.buf.inner == $1.buf.inner")
        with self.assertRaises(LarkError):
            grammar.parse("$1.buf.size == 0")

    def test_unit_enum_tags_are_literals_without_fake_payload_types(self):
        from lark.exceptions import LarkError
        m = self.model("fn f(kind: core::cmp::Ordering)")
        grammar = m.compile_grammar('is_variant($1, "Less")')[0]
        grammar.parse('is_variant($1, "Less")')
        grammar.parse("is_variant($1, 'Equal')")
        for expr in ['is_variant($1, "Unknown")', '$1.Less == $1.Less']:
            with self.subTest(expr=expr), self.assertRaises(LarkError):
                grammar.parse(expr)
        self.assertNotIn("core::cmp::Ordering::Less", m.schema["definitions"])
        m.validate('is_variant($1, "Less")')

    def test_numbering_stable_and_lifetimes_are_metadata(self):
        m = self.model("impl<T, const N: usize> alloc::vec::Vec<T> { fn f<'a, U>(&'a self, other: U) -> &'a T }")
        self.assertEqual([v.source_name for v in m.interface.variables], ["self", "other", "T", "N", "'a", "U", "result"])
        self.assertEqual(m.interface.variables[4].dsl_type, {"kind": "opaque", "rust": "lifetime parameter 'a"})
        trait = self.model("unsafe trait core::cmp::BytewiseEq<Rhs = Self>: PartialEq<Rhs> + Sized")
        self.assertEqual([v.source_name for v in trait.interface.variables], ["Self", "Rhs"])
        with self.assertRaises(dsltrans.ValidationError):
            trait.validate("$1.size == $2.size")

    def test_source_trait_method_context_and_spaced_generics(self):
        m = self.model("trait Allocator < T > { unsafe fn deallocate (& self, layout: core :: alloc :: Layout) }")
        self.assertEqual([v.source_name for v in m.interface.variables], ["self", "layout", "Self", "T"])
        m.validate("$2.size > 0")
        trait = self.model("unsafe trait BytewiseEq < Rhs = Self > : Sized")
        self.assertEqual([v.source_name for v in trait.interface.variables], ["Self", "Rhs"])

    def test_const_generic_default_braces_are_not_a_trait_method_body(self):
        m = self.model("trait TransmuteFrom <Src, const ASSUME: core::mem::transmutability::Assume = { Assume::NOTHING }> where Src: ?Sized")
        self.assertEqual([v.source_name for v in m.interface.variables], ["Self", "Src", "ASSUME"])
        self.assertEqual(m.interface.variables[-1].kind, "const_parameter")

    def test_opaque_function_types_and_tuple_parameter_patterns_keep_slots(self):
        m = self.model("fn f((id, extra): (usize, u8), f: impl FnOnce() -> T) -> !")
        self.assertEqual([v.source_name for v in m.interface.variables], ["(id, extra)", "f", "result"])
        self.assertEqual(m.interface.variables[1].dsl_type["kind"], "opaque")
        m.validate("$1[0] > $1[1]")
        with self.assertRaises(dsltrans.ValidationError):
            m.validate("$2.result > 0")

    def test_audit_qualifies_types_using_the_source_module(self):
        from dslapi import qualify_api as qualify
        from dsltypes import TypeCatalog
        api = "impl<T, const N: usize> IntoIter<T, N> { fn new_unchecked(buffer: [T; N], initialized: Range<usize>) -> Self }"
        m = self.model(api)
        full = TypeCatalog(json.loads(m.catalog_bytes))
        context = {"api": api, "api_variables": m.interface.records(), "source": "tools/rust/library/core/src/array/iter.rs"}
        resolved = qualify(context, full)
        self.assertIn("core::ops::range::Range", resolved)
        self.assertIn("core::array::iter::IntoIter", resolved)
        checked = self.model(resolved)
        checked.validate("$2.start <= $2.end and $2.end <= $4")

    def test_primitive_self_receiver_preserves_str_and_byte_length(self):
        m = self.model("impl str { fn split_at_mut_unchecked(&mut self, mid: usize) -> (&mut str, &mut str) }")
        self.assertEqual([v.source_name for v in m.interface.variables], ["self", "mid", "result"])
        m.validate("$2 <= len($1)")

    def test_audit_does_not_resolve_a_local_alias_to_another_platform(self):
        from dslapi import qualify_api as qualify
        from dsltypes import TypeCatalog
        api = "fn f(fd: RawFd)"
        m = self.model(api)
        full = TypeCatalog(json.loads(m.catalog_bytes))
        context = {"api": api, "api_variables": m.interface.records(), "source": "tools/rust/library/std/src/os/fd/owned.rs"}
        resolved = qualify(context, full)
        self.assertNotIn("solid", resolved)
        self.assertIn("std::os::fd::owned::RawFd", resolved)
        with self.assertRaises(dsltrans.ValidationError):
            self.model(resolved).validate("$1 != -1")

    def test_source_owner_headers_and_const_kinds(self):
        m = self.model("core::array::IntoIter::new_unchecked<U>(buffer: [T; N])")
        self.assertEqual([v.source_name for v in m.interface.variables], ["buffer", "T", "N", "U"])
        self.assertEqual(m.interface.variables[2].kind, "const_parameter")
        m.validate("len($1) <= $3")
        flag = self.model("fn f<const FLAG: bool>(b: bool)")
        flag.validate("$1 == $2")
        with self.assertRaises(dsltrans.ValidationError):
            flag.validate("$2 + 1 > 0")

    def test_self_return_uses_the_inherent_owner_type(self):
        m = self.model("impl<T> alloc::vec::Vec<T> { fn f(&self) -> Self }")
        self.assertEqual([v.source_name for v in m.interface.variables], ["self", "T", "result"])
        m.validate("$3.len == $1.len")
        with self.assertRaises(ValueError):
            self.model("core::alloc::Allocator::grow")

    def test_all_final_nonterminals_reachable_and_deterministic(self):
        m = self.model("fn f(layout: core::alloc::Layout)")
        a = m.compile_grammar("$1.size > 0")[0]
        b = m.compile_grammar("$1.size > 0")[0]
        self.assertEqual(a.ebnf(), b.ebnf())
        self.assertNotIn("index =", a.ebnf())
        self.assertIn('\".\", \"size\"', a.ebnf())
        a.parse("$1.size != 0")


if __name__ == "__main__":
    unittest.main()
