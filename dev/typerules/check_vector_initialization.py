"""Check normal Rust-pipeline SIMD initialization evidence inside Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    fixed = ROOT / '.local/verify/vector-initialization'
    baseline = evidence / 'baseline-results'
    expected = {name: ('low-confidence-sat' if name in {
        'uninitialized_lane', 'unrepaired_lane', 'shuffled_uninitialized_lane'
    } else 'verified') for name in (
        'initialized_lane', 'uninitialized_lane', 'initialized_vector',
        'repaired_lane', 'unrepaired_lane', 'shuffled_initialized_lane',
        'shuffled_uninitialized_lane')}
    reports = {}
    for label, folder in [('baseline', baseline), ('fixed', fixed)]:
        matrix = json.loads((folder / 'full-matrix-results.json').read_text())
        assert matrix['completed'] == matrix['total'] == 7, matrix
        actual = {r['caller']: r['status'] for r in matrix['results']}
        wanted = expected if label == 'fixed' else {
            name: ('verified' if name == 'initialized_vector' else 'low-confidence-sat')
            for name in expected}
        assert actual == wanted, (label, actual, wanted)
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            log = (pair / 'klee.log').read_text()
            assert '[ext.dsl] resolved constraint expr:' in log, pair
            assert 'Stack dump:' not in log and 'Assertion `' not in log, pair
            result = json.loads((pair / 'result.json').read_text())
            assert result['result']['llm_status'] in (None, 'skipped'), pair
            assert not result['stages'].get('llm') and not result['stages'].get('rerun'), pair
            assert result['rule']['dsl'] == '$1.$mem.pointee.initialized'
        reports[label] = {'counts': dict(Counter(actual.values())), 'callers': actual}
    # Retain evidence that these are actual LLVM lane operations, not scalar
    # source rewrites which bypass the responsible execution layer.
    assembly = '\n'.join(p.read_text() for p in fixed.glob('*/rule-322-initialized/klee-out/assembly.ll'))
    for opcode in ('extractelement', 'insertelement', 'shufflevector'):
        assert opcode in assembly, opcode
    source = json.loads((ROOT / 'dev/typerules/vector-initialization-rules.json').read_text())['rules']['rule-322-initialized']
    reports.update({'rule': 'rule-322-initialized', 'dsl': source['dsl'], 'original_rule': source['rule'],
                    'llm_verification_requests': 0,
                    'limitations': ['Poison-mask branches and symbolic indices are not exercised by this source fixture.',
                                    'Vector arithmetic, nested undef constants and snapshot-root shadows remain incomplete.']})
    (evidence / 'vector-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
