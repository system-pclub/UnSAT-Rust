"""Lowering of source-derived DSL to the JSON KLEE reads; run inside the project Docker image."""
import json
import unittest

import dslv2
import dsltrans


class LoweringChecks(unittest.TestCase):
    def model(self, api):
        return dsltrans.Model(dsltrans.ROOT / 'tools/x/generated/dsl', api)

    def test_slots_and_memory_projection(self):
        m = self.model('impl < T > core::ptr::non_null::NonNull < T > { const unsafe fn new_unchecked (ptr : * mut T) -> Self }')
        document = dslv2.lower(m, '$1.$mem.pointee.start != 0')
        self.assertEqual(document['dsl_version'], 2)
        self.assertEqual([s['kind'] for s in document['slots']], ['argument', 'type_parameter', 'return_value'])
        self.assertEqual(document['slots'][0]['arg_index'], 0)
        self.assertFalse(document['uses_result'])
        compare = document['expr']
        self.assertEqual(compare['ops'], ['!='])
        pointee = compare['left']
        self.assertEqual((pointee['k'], pointee['name']), ('attr', 'start'))
        self.assertEqual(pointee['base']['name'], 'pointee')
        self.assertEqual(pointee['base']['base'], {'k': 'mem', 'base': {'k': 'slot', 'n': 1}})

    def test_generics_are_indexed_and_result_use_is_reported(self):
        m = self.model('impl < T , const N : usize > core::array::iter::IntoIter < T , N > { const unsafe fn new_unchecked (buffer : [MaybeUninit < T > ; N] , initialized : core::ops::range::Range < usize >) -> Self }')
        document = dslv2.lower(m, '$2.end <= $4')
        generic = {s['name']: s['generic_index'] for s in document['slots'] if 'generic_index' in s}
        self.assertEqual(generic, {'T': 0, 'N': 1})
        self.assertFalse(document['uses_result'])
        used = dslv2.lower(self.model('impl < T > core::option::Option < T > { const unsafe fn unwrap_unchecked (self) -> T }'),
                           'is_variant($1, "Some") and $3 == $1.Some[0]')
        self.assertTrue(used['uses_result'])

    def test_comprehension_and_call_nodes(self):
        m = self.model('const unsafe fn from_utf8_unchecked (v : & [u8]) -> & str')
        document = dslv2.lower(m, 'all([b < 128 for b in $1])')
        call = document['expr']
        self.assertEqual((call['k'], call['name']), ('call', 'all'))
        comp = call['args'][0]
        self.assertEqual(comp['k'], 'comp')
        self.assertEqual(comp['gens'][0]['target'], 'b')

    def test_rejected_expression_is_not_lowered(self):
        m = self.model('fn f(p: *mut u32, n: usize)')
        with self.assertRaises(Exception):
            dslv2.lower(m, '$1.$mem.foobar > 0')

    def test_large_integer_survives_json_transport_exactly(self):
        value = 2 ** 240 + 17
        document = dslv2.lower(self.model('fn f(n: i64)'), f'$1 < {value}')
        restored = json.loads(json.dumps(document))
        self.assertEqual(int(restored['expr']['rights'][0]['v']), value)

    def test_verify_passes_the_lowered_document_to_klee_unchanged(self):
        from cli.cmd.verify import _rule_entry_to_ext_ast_json
        document = {'dsl_version': 2, 'slots': [], 'expr': {'k': 'bool', 'v': True}}
        self.assertEqual(json.loads(_rule_entry_to_ext_ast_json({'dsl_v2': document}, [])), document)
        with self.assertRaises(RuntimeError):
            _rule_entry_to_ext_ast_json({'dsl_v2': {'dsl_version': 3}}, [])


if __name__ == '__main__':
    unittest.main()
