"""Check saved rule inventories and compilation evidence without API requests."""
import argparse
from collections import Counter
import csv
import json
from pathlib import Path


def read(path):
    return json.loads(path.read_text())


def catalog(path):
    with path.open() as handle:
        return list(csv.DictReader(handle))


def check(root):
    directories = read(root / 'inputs.json')['directories']
    merged, report, pending = [], {}, False
    for directory in directories:
        output = root / Path(directory).name
        assert read(output / 'input.json')['directory'] == directory, 'source identity changed'
        source = read(output / 'step0.json')
        extracted = read(output / 'step1.json')
        expected = {(path.removeprefix('tools/'), unit['line_number_start'], unit['name'])
                    for path, units in source['files'].items() for unit in units}
        actual = [(row['path'], row['line'], row['name']) for row in extracted]
        assert len(actual) == len(set(actual)) and set(actual) == expected, 'incomplete source inventory'
        assert all(row['status'] == 'extracted' for row in extracted), 'extraction errors'
        rows = catalog(output / 'rules.csv')
        assert len(rows) == sum(len(row['rules']) for row in extracted), 'clause inventory mismatch'
        merged.extend(rows)
        result = {'source_apis': len(extracted), 'clauses': len(rows)}
        manifest = read(output / 'step2-input.json')
        assert manifest['rules'] == rows, 'translation selection differs from full catalog'
        cache = Path(manifest['translations_directory'])
        if not cache.exists():
            # Saved container paths are portable to the repository working tree.
            cache = Path(str(cache).removeprefix('/unsat-rust/'))
        if not (output / 'step2.json').exists():
            result['state'] = 'pending'
            pending = True
        else:
            translated = read(output / 'step2.json')
            assert [r['id'] for r in translated] == [r['id'] for r in rows], 'translation inventory mismatch'
            counts = Counter(row['status'] for row in translated)
            for original, row in zip(rows, translated):
                assert all(row[k] == value for k, value in original.items()), 'translation source mismatch'
                assert row.get('attempt', 0) <= manifest['retries'], 'repair budget exceeded'
                if row['status'] == 'translated':
                    validation = row['validation']
                    assert validation['generated_grammar'] == validation['syntax_and_schema'] == 'passed'
                    assert (cache / row['id'] / 'grammar.lark').is_file(), 'missing accepted grammar'
                    assert row.get('expression'), 'missing accepted expression'
            result['statuses'] = dict(counts)
            result['attempts'] = dict(Counter(str(row.get('attempt', 0)) for row in translated))
            allowed = {'translated'} if manifest.get('output_contract') == 'expression_only' else {'translated', 'unexpressible'}
            result['state'] = 'complete' if set(counts) <= allowed else 'errors'
        report[output.name] = result
    assert catalog(root / 'rules.csv') == merged, 'global catalog differs from library catalogs'
    assert [row['id'] for row in merged] == [f'rule-{i}' for i in range(1, len(merged)+1)], 'unstable eval IDs'
    return {'libraries': report, 'total_clauses': len(merged), 'pending': pending,
            'semantic_fidelity': 'not_verified',
            'note': 'Checks source inventory and retained compilation evidence, not DSL semantic equivalence.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run-root', type=Path, default=Path('eval/rule'))
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    result = check(args.run_root)
    text = json.dumps(result, indent=2) + '\n'
    if args.output:
        args.output.write_text(text)
    print(text, end='')
    if result['pending']:
        raise SystemExit(75)
    if any(lib['state'] != 'complete' for lib in result['libraries'].values()):
        raise SystemExit(1)


if __name__ == '__main__':
    main()
