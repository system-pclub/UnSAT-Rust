"""Check baseline/fixed normal-pipeline vector-bitwise shadow evidence."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    expected = {
        'xor_initialized_lane': 'verified',
        'xor_uninitialized_lane': 'low-confidence-sat',
        'and_initialized_lane': 'verified',
        'or_uninitialized_lane': 'low-confidence-sat',
        'or_initialized_vector': 'verified',
        'or_initialized_lane': 'verified',
        'and_uninitialized_rhs': 'low-confidence-sat',
    }
    source = json.loads((ROOT / 'dev/typerules/vector-bitwise-rules.json').read_text())['rules']['rule-322-initialized']
    report = {'rule': 'rule-322-initialized', 'dsl': source['dsl'],
              'original_rule': source['rule'], 'llm_verification_requests': 0}
    for label in ('baseline', 'fixed'):
        folder = evidence / f'{label}-results'
        matrix = json.loads((folder / 'matrix.json').read_text())
        count = 5 if label == 'baseline' else len(expected)
        assert matrix['completed'] == matrix['total'] == count, matrix
        actual = {r['caller']: r['status'] for r in matrix['results']}
        wanted = expected if label == 'fixed' else {
            name: ('verified' if name == 'or_initialized_vector' else 'low-confidence-sat')
            for name in list(expected)[:5]}
        assert actual == wanted, (label, actual, wanted)
        assembly = ''
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            log = (pair / 'klee.log').read_text()
            assert '[ext.dsl] resolved constraint expr:' in log, pair
            assert 'failed to evaluate expression' not in log, pair
            result = json.loads((pair / 'result.json').read_text())
            assert result['rule']['dsl'] == source['dsl']
            assert result['rule']['rule'] == source['rule']
            assert result['result']['llm_status'] in (None, 'skipped')
            assert result['stages']['init']['ablation_level'] == 'full'
            assert not result['stages'].get('llm') and not result['stages'].get('rerun')
            assembly += (pair / 'klee-out/assembly.ll').read_text()
        for opcode in ('and', 'or', 'xor'):
            assert f'{opcode} <2 x i64>' in assembly, opcode
        report[label] = {'counts': dict(Counter(actual.values())), 'callers': actual}
    report['limitations'] = [
        'SAT examples are synthetic unconfirmed observations, never confirmed vulnerabilities.',
        'Vector arithmetic, comparisons, casts and vector-condition selects remain unaudited.',
        'Nested undef/poison constants and scalar snapshot-root initialization remain incomplete.',
    ]
    (evidence / 'vector-bitwise-validation.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
