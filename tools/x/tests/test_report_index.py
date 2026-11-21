import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('report_index', ROOT / 'tools/x/scripts/index_crate_reports.py')
index = importlib.util.module_from_spec(spec)
spec.loader.exec_module(index)


class ReportIndexChecks(unittest.TestCase):
    def check_record(self, metadata, pair=None):
        with tempfile.TemporaryDirectory() as temporary:
            run = Path(temporary)
            job = run / 'jobs/000001'
            job.mkdir(parents=True)
            (run / 'catalog').mkdir()
            (run / 'catalog/rules.json').write_text(json.dumps({'rules': {'example': metadata}}))
            (job / 'source.path').write_text('/example/source')
            row = {'status': 'low-confidence-sat', 'rule': 'example'}
            if pair is not None:
                path = job / 'result.json'
                path.write_text(json.dumps({'rule': pair}))
                row['pair_result_json'] = str(path)
            (job / 'matrix.json').write_text(json.dumps({'results': [row]}))
            index.index_reports(run, job)
            record = json.loads((run / 'findings.jsonl').read_text())
            self.assertEqual(record, json.loads((job / 'findings.json').read_text())[0])
            return record

    def test_pair_preserves_original_expression_and_sentence(self):
        record = self.check_record({'dsl': 'stale'}, {
            'expression': '  $1 > 0  ', 'dsl': 'different',
            'rule': 'The count must be positive.\nKeep this sentence unchanged.'})
        self.assertEqual(record['dsl'], '  $1 > 0  ')
        self.assertEqual(record['original_rule'], 'The count must be positive.\nKeep this sentence unchanged.')
        self.assertEqual(record['rule'], 'example')

    def test_missing_pair_uses_original_catalog(self):
        record = self.check_record({'expression': '$1 != 0', 'rule': 'The pointer must be non-null.'})
        self.assertEqual(record['dsl'], '$1 != 0')
        self.assertEqual(record['original_rule'], 'The pointer must be non-null.')

    def test_builtin_check_does_not_invent_dsl(self):
        record = self.check_record({}, {'dsl': None, 'rule': 'Built-in raw pointer check.'})
        self.assertIsNone(record['dsl'])
        self.assertEqual(record['original_rule'], 'Built-in raw pointer check.')
