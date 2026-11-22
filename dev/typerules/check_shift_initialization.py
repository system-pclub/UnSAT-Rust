"""Check source-level scalar shift shadows and unchanged DSL provenance."""
import json
import sys
from pathlib import Path
from collections import Counter

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    source = json.loads((ROOT / 'dev/typerules/shift-initialization-rules.json').read_text())['rules']['rule-322-initialized']
    names = ('logical_initialized_half', 'logical_uninitialized_half',
             'left_initialized_zero', 'left_uninitialized_half',
             'arithmetic_initialized_sign', 'arithmetic_uninitialized_sign')
    reports = {}
    for label in ('baseline', 'fixed'):
        folder = evidence / (label + '-results')
        matrix = json.loads((folder / 'matrix.json').read_text())
        expected_names = names + (('arithmetic_initialized_fill', 'logical_initialized_fill') if label == 'fixed' else ())
        assert matrix['completed'] == matrix['total'] == len(expected_names), matrix
        assert matrix['ablation_level'] == 'full'
        actual = {r['caller']: r['status'] for r in matrix['results']}
        expected = {name: ('low-confidence-sat' if label == 'baseline' or 'uninitialized' in name else 'verified') for name in expected_names}
        assert actual == expected, (label, actual, expected)
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            assert row['rule'] == 'rule-322-initialized'
            log = (pair / 'klee.log').read_text()
            assert '[ext.dsl] resolved constraint expr:' in log, pair
            assert 'Stack dump:' not in log and 'Assertion `' not in log, pair
            result = json.loads((pair / 'result.json').read_text())
            assert result['rule']['dsl'] == source['dsl']
            assert result['rule']['rule'] == source['rule']
            assert result['result']['llm_status'] in (None, 'skipped')
            assert not result['stages'].get('llm') and not result['stages'].get('rerun')
        assembly = '\n'.join(p.read_text() for p in folder.glob('*/rule-322-initialized/klee-out/assembly.ll'))
        for opcode in ('lshr i32', 'shl i32', 'ashr i32'):
            assert opcode in assembly, (label, opcode)
        reports[label] = {'counts': dict(Counter(actual.values())), 'callers': actual}
    reports.update({'rule': 'rule-322-initialized', 'dsl': source['dsl'], 'original_rule': source['rule'],
                    'llm_verification_requests': 0,
                    'limitations': ['Symbolic/uninitialized shift amounts and vector shifts are not established by these source examples.',
                                    'Allocation provenance/contracts, nested constant shadows and scalar snapshot roots remain incomplete.']})
    (evidence / 'shift-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
