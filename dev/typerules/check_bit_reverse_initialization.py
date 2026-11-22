"""Check source-derived bitreverse verdicts, payloads, and initialization."""
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    evidence = ROOT / sys.argv[1]
    rules = json.loads((ROOT / 'dev/typerules/bit-reverse-initialization-rules.json').read_text())['rules']
    names = [f'reverse{width}_{state}' for width in (16, 32, 64, 128)
             for state in ('initialized', 'uninitialized')]
    report = {}
    for label in ('baseline', 'fixed'):
        folder = evidence / (label + '-results')
        matrix = json.loads((folder / 'matrix.json').read_text())
        expected = {name: 'klee-error' for name in names}
        if label == 'fixed':
            expected = {name: ('low-confidence-sat' if 'uninitialized' in name else 'verified')
                        for name in names + ['reverse8_initialized', 'reverse8_uninitialized']}
            expected.update(reverse32_value_nonnull='verified', reverse64_value_null='low-confidence-sat')
        assert matrix['completed'] == matrix['total'] == len(expected), matrix
        assert matrix['ablation_level'] == 'full'
        actual = {row['caller']: row['status'] for row in matrix['results']}
        assert actual == expected, (label, actual, expected)
        constraints = {}
        for row in matrix['results']:
            pair = folder / row['callsite'] / row['rule']
            rule_id = 'rule-504' if '_value_' in row['caller'] else 'rule-322-initialized'
            assert row['rule'] == rule_id
            result = json.loads((pair / 'result.json').read_text())
            assert result['rule']['dsl'] == rules[rule_id]['dsl']
            assert result['rule']['rule'] == rules[rule_id]['rule']
            assert result['result']['llm_status'] in (None, 'skipped')
            assert not result['stages'].get('llm') and not result['stages'].get('rerun')
            log = (pair / 'klee.log').read_text()
            if label == 'baseline':
                assert 'unimplemented intrinsic: llvm.bitreverse.' in log, pair
            else:
                constraints[row['caller']] = [line for line in log.splitlines()
                    if '[ext.dsl] resolved constraint expr:' in line]
                assert constraints[row['caller']], pair
                for error in ('failed to evaluate expression', 'Stack dump:', 'Assertion `', 'unimplemented intrinsic:'):
                    assert error not in log, (pair, error)
        if label == 'fixed':
            assembly = '\n'.join(path.read_text() for path in folder.glob('*/rule-322-initialized/klee-out/assembly.ll'))
            for width in (8, 16, 32, 64, 128):
                assert f'call i{width} @llvm.bitreverse.i{width}' in assembly, width
        report[label] = {'counts': dict(Counter(actual.values())), 'callers': actual, 'constraints': constraints}
    report['rules'] = {key: {'dsl': rule['dsl'], 'original_rule': rule['rule']} for key, rule in rules.items()}
    report['llm_verification_requests'] = 0
    report['limitations'] = [
        'Synthetic initialization observations; partial Rust loads are verifier inputs only.',
        'Fixed-vector bit reversal has no source coverage; nested constants, snapshot roots and allocation contracts remain incomplete.',
        'SAT reports are unconfirmed candidates, never confirmed vulnerabilities.']
    (evidence / 'bit-reverse-validation.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
