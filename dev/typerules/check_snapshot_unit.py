"""Check normal-pipeline zero-sized snapshot layout regressions in Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    reports = {}
    source = json.loads((ROOT / 'dev/typerules/snapshot-unit-rules.json').read_text())['rules']['rule-504']
    for label in ('baseline', 'fixed'):
        folder = evidence / (label + '-results')
        matrix = json.loads((folder / 'matrix.json').read_text())
        assert matrix['completed'] == matrix['total'] == 6, matrix
        assert matrix['ablation_level'] == 'full'
        actual = {r['caller']: r['status'] for r in matrix['results']}
        expected = dict.fromkeys(('unit_tail', 'unit_array', 'unit_arc', 'null_unit_tail', 'unsized_arc'), 'eval-error')
        expected['sized_arc'] = 'verified'
        if label == 'fixed':
            expected.update(dict.fromkeys(('unit_tail', 'unit_array', 'unit_arc'), 'verified'))
            expected['null_unit_tail'] = 'low-confidence-sat'
        assert actual == expected, (label, actual, expected)
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            result = json.loads((pair / 'result.json').read_text())
            assert result['rule']['dsl'] == source['dsl'] == '$1.$mem.pointee.start != 0'
            assert result['rule']['rule'] == source['rule'] == '`ptr` must be non-null.'
            assert result['result']['llm_status'] in (None, 'skipped')
            assert not result['stages'].get('llm') and not result['stages'].get('rerun')
            log = (pair / 'klee.log').read_text()
            assert 'Stack dump:' not in log and 'Assertion `' not in log
            if row['status'] == 'eval-error':
                diagnostic = ('snapshot array elements exceed static storage'
                              if row['caller'] == 'unit_array' else
                              'snapshot field layout exceeds static storage')
                assert diagnostic in log, pair
            else:
                assert '[ext.dsl] resolved constraint expr:' in log
        reports[label] = {'counts': dict(Counter(actual.values())), 'callers': actual}
    assembly = next((evidence / 'fixed-results').glob('*/rule-504/klee-out/assembly.ll')).read_text()
    assert '!DIBasicType(name: "()", encoding: DW_ATE_unsigned)' in assembly
    reports.update({'rule': 'rule-504', 'dsl': source['dsl'], 'original_rule': source['rule'],
                    'llm_verification_requests': 0,
                    'limitations': ['Unsized Arc<str> snapshot layout remains an explicit error.',
                                    'Allocation contracts, generation/provenance and nested constant initialization remain incomplete.',
                                    'SAT is a candidate, not a confirmed vulnerability.']})
    (evidence / 'snapshot-unit-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
