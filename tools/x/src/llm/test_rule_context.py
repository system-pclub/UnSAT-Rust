import unittest
from llm.rule_context import DocIndex, compact_docs


def unit(name, line, text, parent=None, body=None):
    return {'name':name,'line_number_start':line,'declaration_text':f'unsafe fn {name}(src: *const u8)',
            'comment':text,'parent_struct_or_trait':parent,'source_body':body}


class ContextTests(unittest.TestCase):
    def test_explicit_pointer_reference_resolves_source_contract(self):
        docs={'files':{'tools/rust/library/core/src/ptr/mod.rs':[
            unit('read',10,'# Safety\nsrc must be aligned and valid for reads.')],
            'tools/rust/library/core/src/ptr/const_ptr.rs':[
            unit('read',20,'See [`ptr::read`] for safety concerns.\n[`ptr::read`]: crate::ptr::read()', '*const T', '{ read(self) }')]}}
        index=DocIndex([docs])
        source=index.units[('rust/library/core/src/ptr/const_ptr.rs',20,'read')]
        context=index.context(source)
        self.assertEqual(len(context['referenced_docs']),1)
        self.assertIn('src must be aligned',context['referenced_docs'][0]['documentation'])
        self.assertEqual(context['forwarding_body'],'{ read(self) }')

    def test_explicit_module_reference_beats_same_named_wrapper(self):
        docs = {"files": {"rust/library/core/src/slice/raw.rs": [unit("from_raw_parts_mut", 1, "# Safety\nMust be writable.")],
                "rust/library/core/src/str/converts.rs": [unit("from_raw_parts_mut", 2,
                "Equivalent to [`slice::from_raw_parts_mut`](crate::slice::from_raw_parts_mut). See its safety docs.")]}}
        index = DocIndex([docs])
        item = index.units[("rust/library/core/src/str/converts.rs", 2, "from_raw_parts_mut")]
        self.assertEqual(index.context(item)["referenced_docs"][0]["path"], "rust/library/core/src/slice/raw.rs:1")

    def test_ambiguous_reference_is_retained_as_unresolved(self):
        index=DocIndex([{'files':{'rust/library/core/src/one.rs':[unit('f',1,'# Safety\nMust be initialized.')],
            'rust/library/core/src/two.rs':[unit('f',1,'# Safety\nMust be non-null.')],
            'rust/library/core/src/three.rs':[unit('wrapper',1,'See [`f()`] for safety requirements.')]}}])
        source=index.units[('rust/library/core/src/three.rs',1,'wrapper')]
        context=index.context(source)
        self.assertNotIn('referenced_docs',context)
        self.assertIn('f()',context['unresolved_doc_links'])

    def test_plain_code_path_in_delegated_safety_text_is_resolved(self):
        target = unit("spawn_unchecked", 10, "# Safety\nThe thread must not outlive captured references.", "Builder")
        current = unit("new", 20, "# Safety\nSee `thread::Builder::spawn_unchecked` for safety requirements.", "Thread")
        index = DocIndex([{"files": {"rust/library/std/src/thread/mod.rs": [target],
                                   "rust/library/std/src/sys/pal/itron/thread.rs": [current]}}])
        context = index.context(index.units[("rust/library/std/src/sys/pal/itron/thread.rs", 20, "new")])
        self.assertIn("captured references", context["referenced_docs"][0]["documentation"])

    def test_bare_see_under_safety_inherits_the_target_contract(self):
        source = unit("alloc", 1, "# Safety\nlayout must have non-zero size.", "GlobalAlloc")
        source["declaration_text"] = "unsafe fn alloc(layout: Layout)"
        wrapper = unit("alloc", 2, "Allocates memory.\n\n# Safety\n\nSee [`GlobalAlloc::alloc`].")
        index = DocIndex([{"files": {"rust/library/core/src/alloc/global.rs": [source],
                                    "rust/library/alloc/src/alloc.rs": [wrapper]}}])
        current = index.units[("rust/library/alloc/src/alloc.rs", 2, "alloc")]
        self.assertIn("non-zero size", index.context(current)["referenced_docs"][0]["documentation"])

    def test_method_receives_referenced_trait_definitions_not_trait_obligations(self):
        trait = unit("Allocator", 1, "### Memory fitting\nThe layout size lies between the requested and actual sizes.\n# Safety\nCloning must not invalidate blocks.")
        trait["declaration_text"] = "trait Allocator"
        method = unit("deallocate", 20, "# Safety\nlayout must [fit].\n[fit]: #memory-fitting", "Allocator")
        index = DocIndex([{"files": {"rust/library/core/src/alloc/mod.rs": [trait, method]}}])
        current = index.units[("rust/library/core/src/alloc/mod.rs", 20, "deallocate")]
        definition = index.context(current)["referenced_definitions"][0]
        self.assertIn("requested and actual", definition["text"])
        self.assertNotIn("Cloning", definition["text"])

    def test_examples_do_not_hide_later_normative_sections(self):
        text='# Examples\n```\nunsafe { made_up() }\n```\n# Safety\nMust remain valid.\n# Panics\nPanics if null.'
        result=compact_docs(text)
        self.assertNotIn('made_up',result)
        self.assertIn('Must remain valid.',result)
        self.assertIn('Panics if null.',result)

    def test_bare_delegation_and_definitions_after_examples(self):
        docs = {'files': {'rust/library/core/src/test.rs': [
            unit('target', 1, '# Safety\nThe pointer must stay valid.'),
            unit('defer', 2, '## Safety\nSame as `target` except lazy.'),
            unit('wrapper', 3, 'See [`target`] for details.\n\n# Examples\nExample.\n\n[`target`]: target'),
        ]}}
        index = DocIndex([docs])
        for name, line in [('defer', 2), ('wrapper', 3)]:
            context = index.context(index.units[('rust/library/core/src/test.rs', line, name)])
            self.assertEqual(context['referenced_docs'][0]['path'], 'rust/library/core/src/test.rs:1')

    def test_local_method_anchor_uses_the_current_owner(self):
        index = DocIndex([{'files': {'rust/library/core/src/test.rs': [
            unit('offset_from', 1, '# Safety\nMust share an allocation.', '*const T'),
            unit('offset_from', 2, '# Safety\nAnother owner.', '*mut T'),
            unit('unsigned', 3, '# Safety\nAll safety conditions of [`offset_from`](#method.offset_from) apply.', '*const T'),
        ]}}])
        context = index.context(index.units[('rust/library/core/src/test.rs', 3, 'unsigned')])
        self.assertEqual(context['referenced_docs'][0]['path'], 'rust/library/core/src/test.rs:1')

    def test_self_reference_normalizes_generic_owner(self):
        index = DocIndex([{'files': {'rust/library/core/src/test.rs': [
            unit('offset_from', 1, '# Safety\nMust share an allocation.', 'NonNull < T >'),
            unit('unsigned', 2, '# Safety\nAll safety conditions of [`offset_from`](#method.offset_from) apply.', 'NonNull < T >'),
        ]}}])
        context = index.context(index.units[('rust/library/core/src/test.rs', 2, 'unsigned')])
        self.assertEqual(context['referenced_docs'][0]['path'], 'rust/library/core/src/test.rs:1')
