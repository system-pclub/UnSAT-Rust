"""Check normal-pipeline intrinsic shadow regressions inside Docker."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    fixed = ROOT / '.local/verify/intrinsic-initialization'
    names = ('initialized_count', 'uninitialized_count', 'initialized_fabs',
             'uninitialized_fabs', 'initialized_rotate', 'uninitialized_rotate',
             'rotated_initialized_half', 'rotated_uninitialized_half')
    reports = {}
    for label, folder in [('baseline', evidence / 'baseline-results'), ('fixed', fixed)]:
        matrix = json.loads((folder / 'full-matrix-results.json').read_text())
        assert matrix['completed'] == matrix['total'] == len(names), matrix
        actual = {r['caller']: r['status'] for r in matrix['results']}
        expected = {name: ('low-confidence-sat' if (label == 'fixed' and 'uninitialized' in name or
                            name == 'uninitialized_count') else 'verified') for name in names}
        assert actual == expected, (label, actual, expected)
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
    assembly = '\n'.join(p.read_text() for p in fixed.glob('*/rule-322-initialized/klee-out/assembly.ll'))
    for intrinsic in ('llvm.fabs.f32', 'llvm.fshl.i32', 'llvm.fshr.i32'):
        assert intrinsic in assembly, intrinsic
    assert 'ctpop.step' in assembly  # Existing cleaner lowers population count.
    source = json.loads((ROOT / 'dev/typerules/intrinsic-initialization-rules.json').read_text())['rules']['rule-322-initialized']
    reports.update({'rule': 'rule-322-initialized', 'dsl': source['dsl'],
                    'original_rule': source['rule'], 'llm_verification_requests': 0,
                    'limitations': ['Symbolic shift amounts and uninitialized shift operands are implemented but not established by this fixture.',
                                    'Other intrinsic/opaque summaries, nested undef constants and snapshot-root initialization remain incomplete.']})
    (evidence / 'intrinsic-validation.json').write_text(json.dumps(reports, indent=2) + '\n')
    print(json.dumps(reports, indent=2))


if __name__ == '__main__':
    main()
