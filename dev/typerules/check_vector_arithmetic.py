"""Inspect normal-pipeline fixed-vector arithmetic evidence inside Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def main():
    evidence = ROOT / sys.argv[1]
    rules = json.loads((ROOT / 'dev/typerules/vector-arithmetic-rules.json').read_text())['rules']
    fixed = {
        'add_initialized_lane': 'verified',
        'add_uninitialized_lane': 'low-confidence-sat',
        'sub_initialized_lane': 'verified',
        'mul_initialized_lane': 'verified',
        'mul_uninitialized_rhs': 'low-confidence-sat',
        'add_carry': 'low-confidence-sat',
        'sub_borrow': 'low-confidence-sat',
        'mul_cross_lane': 'low-confidence-sat',
    }
    report = {'rules': {k: {'dsl': r['dsl'], 'original_rule': r['rule']} for k, r in rules.items()},
              'llm_requests': 0}
    for label in ('baseline', 'fixed'):
        folder = evidence / f'{label}-results'
        matrix = json.loads((folder / 'matrix.json').read_text())
        actual = {r['caller']: r['status'] for r in matrix['results']}
        assert matrix['completed'] == matrix['total'] == len(fixed), matrix['counts']
        if label == 'fixed':
            assert actual == fixed, (actual, fixed)
        else:
            for caller in ('add_initialized_lane', 'sub_initialized_lane', 'mul_initialized_lane'):
                assert actual[caller] == 'low-confidence-sat', actual
            for caller in ('add_carry', 'sub_borrow', 'mul_cross_lane'):
                assert actual[caller] == 'verified', actual
        assembly = ''
        for row in matrix['results']:
            # Paths in the matrix are container paths under the checkout.
            path = Path(row['pair_result_json'])
            path = ROOT / path.relative_to('/unsat-rust')
            data = json.loads(path.read_text())
            rule = rules[row['rule']]
            assert data['rule']['dsl'] == rule['dsl']
            assert data['rule']['rule'] == rule['rule']
            assert data['stages']['init']['ablation_level'] == 'full'
            assert not data['stages'].get('llm') and not data['stages'].get('rerun')
            assert row['returncode'] == 0
            log = (path.parent / 'klee.log').read_text()
            assert '[ext.dsl] resolved constraint expr:' in log
            assert 'failed to evaluate expression' not in log
            assembly += (path.parent / 'klee-out/assembly.ll').read_text()
        for opcode in ('add', 'sub', 'mul'):
            assert f'{opcode} <4 x i32>' in assembly, opcode
        report[label] = {'callers': actual, 'counts': dict(Counter(actual.values()))}
    report['limitations'] = [
        'SAT observations are unconfirmed; negative initialization fixtures are verifier inputs only.',
        'Coverage establishes wrapping i32 SIMD Add/Sub/Mul lanes, not vector division, floating arithmetic or overflow flags.',
        'Allocation origin/request contracts, nested undef constants and scalar snapshot-root initialization remain incomplete.',
    ]
    (evidence / 'vector-arithmetic-validation.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))

if __name__ == '__main__':
    main()
