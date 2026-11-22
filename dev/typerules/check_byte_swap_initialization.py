"""Check byte-swap shadows against normal verification, not backend JSON."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    sources = json.loads((ROOT / 'dev/typerules/byte-swap-initialization-rules.json').read_text())['rules']
    source = sources['rule-322-initialized']
    names = tuple(f'swap{width}_{state}' for width in (16, 32, 64, 128)
                  for state in ('initialized', 'uninitialized'))
    reports = {}
    for label in ('baseline', 'fixed'):
        folder = evidence / (label + '-results')
        matrix = json.loads((folder / 'matrix.json').read_text())
        assert matrix['completed'] == matrix['total'] == len(names) + (2 if label == 'fixed' else 0), matrix
        assert matrix['ablation_level'] == 'full'
        actual = {r['caller']: r['status'] for r in matrix['results']}
        expected = {name: ('low-confidence-sat' if 'uninitialized' in name or (label == 'baseline' and not name.startswith('swap16_')) else 'verified') for name in names}
        if label == 'fixed':
            expected.update(swap32_value_nonnull='verified', swap64_value_null='low-confidence-sat')
        assert actual == expected, (label, actual, expected)
        constraints = {}
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            assert row['rule'] == ('rule-504' if '_value_' in row['caller'] else 'rule-322-initialized')
            exact = sources[row['rule']]
            log = (pair / 'klee.log').read_text()
            constraints[row['caller']] = [line for line in log.splitlines() if '[ext.dsl] resolved constraint expr:' in line]
            assert constraints[row['caller']], pair
            assert 'failed to evaluate expression' not in log, pair
            assert 'Stack dump:' not in log and 'Assertion `' not in log, pair
            result = json.loads((pair / 'result.json').read_text())
            assert result['rule']['dsl'] == exact['dsl']
            assert result['rule']['rule'] == exact['rule']
            assert result['result']['llm_status'] in (None, 'skipped')
            assert not result['stages'].get('llm') and not result['stages'].get('rerun')
        assembly = '\n'.join(p.read_text() for p in folder.glob('*/rule-322-initialized/klee-out/assembly.ll'))
        if label == 'baseline':
            assembly = (evidence / 'baseline-input.ll').read_text()
        for width in (16, 32, 64, 128):
            opcode = f'call i{width} @llvm.bswap.i{width}'
            assert opcode in assembly, (label, opcode)
        reports[label] = {'counts': dict(Counter(actual.values())), 'callers': actual, 'constraints': constraints}
    reports['rules'] = {key: {'dsl': rule['dsl'], 'original_rule': rule['rule']} for key, rule in sources.items()}
    reports.update({'rule': 'rule-322-initialized', 'dsl': source['dsl'], 'original_rule': source['rule'],
                    'llm_verification_requests': 0,
                    'limitations': ['Synthetic byte-initialization observations, not a claim that reading partial Rust values is safe.',
                                    'Fixed-vector byte swaps, nested constants, snapshot roots and allocation contracts/provenance remain unestablished.',
                                    'SAT observations are unconfirmed, not vulnerabilities.']})
    (evidence / 'byte-swap-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
