"""Merge selected, source-matched extraction repairs without discarding Batch provenance."""
import argparse
import hashlib
import json
from pathlib import Path

FIELDS = ('path', 'line', 'name', 'declaration', 'parent', 'documentation')


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, ensure_ascii=False).encode()).hexdigest()


def write(path, value):
    temporary = path.with_suffix(path.suffix + '.tmp')
    temporary.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')
    temporary.replace(path)


def key(row):
    return digest({field: row[field] for field in FIELDS})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--dataset', type=Path, required=True)
    parser.add_argument('--repairs', type=Path, required=True)
    parser.add_argument('--libraries', nargs='+', default=['std', 'core', 'alloc'])
    args = parser.parse_args()
    # Validate every selected library before mutating any aggregate.
    updates = []
    for library in args.libraries:
        destination, source = args.dataset / library, args.repairs / library
        incoming = json.loads((source / 'step1.json').read_text())
        if not incoming:
            continue
        results = json.loads((destination / 'step1.json').read_text())
        positions = {key(row): i for i, row in enumerate(results)}
        origin = json.loads((source / 'step1-input.json').read_text())
        source_hash = digest(json.loads((destination / 'step0.json').read_text()))
        if origin.get('source_sha256') and origin['source_sha256'] != source_hash:
            raise ValueError(f'{library}: repair source snapshot differs')
        path = destination / 'step1-repairs.json'
        ledger = json.loads(path.read_text()) if path.exists() else {'source_sha256': source_hash, 'repairs': {}}
        if ledger['source_sha256'] != source_hash:
            raise ValueError(f'{library}: prior repairs refer to different sources')
        for row in incoming:
            identifier = key(row)
            if identifier not in positions or row.get('status') != 'extracted':
                raise ValueError(f'{library}: stale, unknown, or unsuccessful repair: {row.get("name")}')
            result = {**row, 'repair_request_cache': origin['requests_directory']}
            results[positions[identifier]] = result
            ledger['repairs'][identifier] = {'result': result, 'mode': row['mode'],
                'model': row['model'], 'requests_directory': origin['requests_directory']}
        updates.append((destination, results, ledger, len(incoming)))
    for destination, results, ledger, count in updates:
        backup = destination / 'step1-base.json'
        if not backup.exists():
            write(backup, json.loads((destination / 'step1.json').read_text()))
        write(destination / 'step1-repairs.json', ledger)
        write(destination / 'step1.json', results)
        print(f'{destination.name}: merged {count} repairs; retained the base Batch results')


if __name__ == '__main__':
    main()
