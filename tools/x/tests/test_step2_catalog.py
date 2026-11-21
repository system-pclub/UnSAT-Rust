import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from cli.cmd.verify import _assign_matrix_rules_to_callsites, RAW_PTR_DEREF_CALLEE, RAW_PTR_DEREF_RULE

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('prepare_step2', ROOT / 'tools/x/scripts/prepare_step2_catalog.py')
catalog = importlib.util.module_from_spec(spec)
spec.loader.exec_module(catalog)


class CatalogChecks(unittest.TestCase):
    def test_original_expression_and_status_filter_without_llm(self):
        with tempfile.TemporaryDirectory() as temporary:
            source = Path(temporary) / 'source'
            for library in ('alloc', 'core', 'std'):
                (source / library).mkdir(parents=True)
                rows = [] if library != 'core' else [
                    {'id': 'rule-89', 'status': 'translated', 'api': 'unsafe fn f(n: usize)',
                     'path': 'rust/library/core/src/ptr/mod.rs:1', 'name': 'unsafe fn f',
                     'rule': 'positive n', 'expression': '  $1 > 0  '},
                    {'id': 'rule-90', 'status': 'validation_error', 'expression': 'unchanged failure'}]
                (source / library / 'step2.json').write_text(json.dumps(rows))
            output = Path(temporary) / 'output'
            with patch('llm.openai_llm.OpenAILLM.__init__', side_effect=AssertionError('LLM forbidden')):
                catalog.prepare(source, output)
            document = json.loads((output / 'rules.json').read_text())
            self.assertIs(document['include_raw_pointer_deref'], True)
            manifest = json.loads((output / 'catalog-manifest.json').read_text())
            self.assertIs(manifest['include_raw_pointer_deref'], True)
            self.assertEqual(manifest['llm_requests'], 0)
            self.assertEqual(manifest['lowering_errors'], [])
            self.assertEqual(list(document['rules']), ['step2-core-rule-89'])
            rule = document['rules']['step2-core-rule-89']
            for expression in (rule['expression'], rule['dsl'], rule['dsl_v2']['source']):
                self.assertEqual(expression, '  $1 > 0  ')
            self.assertEqual(json.loads((output / 'excluded.json').read_text())[0]['expression'], 'unchanged failure')
            rows = _assign_matrix_rules_to_callsites(
                callsites=[{'unsafe_callee': RAW_PTR_DEREF_CALLEE}],
                rule_dsl_path=output / 'rules.json', requested_rule=None)
            self.assertEqual(rows[0]['rules'], [RAW_PTR_DEREF_RULE])
