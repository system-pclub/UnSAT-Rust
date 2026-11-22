"""Check scalar subword shadows against normal verification, not backend JSON."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    source = json.loads((ROOT / 'dev/typerules/bitwise-initialization-rules.json').read_text())['rules']['rule-322-initialized']
    names = tuple(f'{op}_{state}_low' for op in ('and', 'or', 'xor', 'xor_rhs')
                  for state in ('initialized', 'uninitialized'))
    reports = {}
    for label in ('baseline', 'fixed'):
        folder = evidence / (label + '-results')
        matrix = json.loads((folder / 'matrix.json').read_text())
        assert matrix['completed'] == matrix['total'] == len(names), matrix
        assert matrix['ablation_level'] == 'full'
        actual = {r['caller']: r['status'] for r in matrix['results']}
        expected = {name: ('low-confidence-sat' if label == 'baseline' or 'uninitialized' in name else 'verified') for name in names}
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
        for opcode in ('and i32', 'or i32', 'xor i32', 'trunc i32'):
            assert opcode in assembly, (label, opcode)
        reports[label] = {'counts': dict(Counter(actual.values())), 'callers': actual}
    reports.update({'rule': 'rule-322-initialized', 'dsl': source['dsl'], 'original_rule': source['rule'],
                    'llm_verification_requests': 0,
                    'limitations': ['Synthetic byte-initialization observations, not a claim that reading partial Rust values is safe.',
                                    'Value-dependent masking, nested constants, snapshot roots and allocation contracts/provenance remain incomplete.',
                                    'SAT observations are unconfirmed, not vulnerabilities.']})
    (evidence / 'bitwise-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
