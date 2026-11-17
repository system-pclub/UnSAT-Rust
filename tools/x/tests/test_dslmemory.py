"""Memory-domain grammar and typing checks; execute inside the project Docker image."""
import json
from pathlib import Path
import unittest

from lark.exceptions import LarkError

import dsltrans
import dslmemory


class MemoryDomainChecks(unittest.TestCase):
    def model(self, api):
        return dsltrans.Model(dsltrans.ROOT / 'tools/x/generated/dsl', api)

    def test_pointer_slot_and_pointee_are_distinct_typed_views(self):
        m = self.model('fn read<T>(src: *const T) -> T')
        self.assertEqual(m.interface.variables[0].dsl_type['kind'], 'pointer')
        m.validate('$1.$mem.pointee.start % $1.$mem.pointee.align == 0')
        m.validate('$1.$mem.start != $1.$mem.pointee.start')
        sorts = [s['rust_type'] for s in m.sorts.values()]
        self.assertIn('MemoryStorage<*const T>', sorts)
        self.assertIn('MemoryRegion<T>', sorts)
        m.prompt_grammar.parse('$1.$mem.pointee.valid')

    def test_memory_fields_are_literal_and_checked_by_the_generated_parser(self):
        m = self.model('fn f(p: *mut u32, n: usize)')
        grammar = m.compile_grammar('$1.$mem.pointee.readable and $2.$mem.size > 0')[0]
        grammar.parse('$1.$mem.pointee.readable')
        for text in ['$1.$mem.foobar > 0', '$1.$mem.pointee.foobar > 0', '$2.$mem.pointee.readable', '$1.$mem.pointee.size.$mem.size > 0']:
            with self.subTest(text=text), self.assertRaises(LarkError):
                grammar.parse(text)
        for text in ['$1.$mem.pointee.size', '$1.$mem.pointee.valid + 1 > 0', '$1.$mem.pointee.start == "0"']:
            with self.subTest(text=text), self.assertRaises(dsltrans.ValidationError):
                m.validate(text)

    def test_source_fields_and_elements_keep_memory_projection(self):
        m = self.model('fn f(v: alloc::vec::Vec<u8>, pair: (usize, *mut u8), a: [u8; 4])')
        for formula in ['$1.len.$mem.size > 0', '$1.buf.inner.ptr.$mem.pointee.readable', '$2[1].$mem.pointee.readable', '$3[0].$mem.initialized', 'all([b.$mem.initialized for b in $3])']:
            with self.subTest(formula=formula):
                m.validate(formula)
        with self.assertRaises(dsltrans.ValidationError):
            m.validate('($1.len + 1).$mem.size > 0')
        with self.assertRaises(dsltrans.ValidationError):
            m.validate('1.$mem.size > 0')

    def test_meta_inputs_and_modeled_scalars_have_no_storage_projection(self):
        m = self.model("fn f<'a, T>(p: *const T, n: usize)")
        self.assertEqual([v.source_name for v in m.interface.variables], ['p', 'n', "'a", 'T'])
        for formula in ['$3.$mem.size > 0', '$4.$mem.size > 0', '$1.$mem.pointee.start.$mem.size > 0', '$1.$mem.$mem.size > 0']:
            with self.subTest(formula=formula), self.assertRaises(dsltrans.ValidationError):
                m.validate(formula)

    def test_reserved_projection_is_rewritten_without_changing_strings(self):
        m = self.model('fn f(p: *const u8)')
        m.validate('"$mem" == "$mem" and $1 . $mem . pointee . readable')
        for formula in ['$mem.start > 0', '$1.__dsl_mem.pointee.readable', '$1.$memory.start > 0', '__dsl_mem == 0']:
            with self.subTest(formula=formula), self.assertRaises(dsltrans.ValidationError):
                m.validate(formula)

    def test_counted_memory_uses_access_extent_and_type_stride(self):
        m = self.model('fn copy<T>(src: *const T, dst: *mut T, count: usize)')
        m.validate('$3 * $1.$mem.pointee.size <= $1.$mem.pointee.readable_size')
        m.validate('$3 * $2.$mem.pointee.size <= $2.$mem.pointee.writable_size')
        m.validate('$1.$mem.pointee.start + $3 * $1.$mem.pointee.size <= $2.$mem.pointee.start or $2.$mem.pointee.start + $3 * $2.$mem.pointee.size <= $1.$mem.pointee.start')
        m.validate('$1.$mem.pointee.valid')
        m.validate('$1.$mem.pointee.initialized')

    def test_origin_allocation_is_typed_and_retains_contract_minimum(self):
        m = self.model('fn f(p: core::ptr::NonNull<u8>, q: *mut u8)')
        m.validate('$1.$mem.pointee.allocation.exists and $2.$mem.pointee.allocation.exists and $1.$mem.pointee.allocation.id == $2.$mem.pointee.allocation.id')
        m.validate('$1.$mem.pointee.allocation.minimum_size <= $1.$mem.pointee.allocation.size')
        m.validate('$1.$mem.pointee.allocation.live')
        with self.assertRaises(dsltrans.ValidationError):
            m.validate('$1.$mem.pointee.allocation.id > 0')

    def test_current_memory_does_not_invent_temporal_or_borrow_properties(self):
        m = self.model('fn f(p: *mut u8)')
        for formula in ['$1.$mem.pointee.never_mutated', '$1.$mem.pointee.after.valid', '$1.$mem.pointee.unique_borrow']:
            with self.subTest(formula=formula), self.assertRaises(dsltrans.ValidationError):
                m.validate(formula)

    def test_modeled_schema_is_separate_from_rust_source_definitions(self):
        m = self.model('fn f(p: *const u8)')
        self.assertEqual(json.loads(m.memory_bytes)['projection'], '.$mem')
        self.assertTrue(all(n.startswith(('core::', 'alloc::', 'std::')) for n in m.schema['definitions']))
        self.assertIn('memory_region', m.memory_grammar)
        self.assertEqual(dslmemory.SCHEMA['fields']['valid']['type'], 'bool')


if __name__ == '__main__':
    unittest.main()
