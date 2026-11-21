"""Index existing reports; never judge SAT as a confirmed vulnerability or invoke an LLM."""
import argparse
import json
from pathlib import Path

FINDINGS = {'violation', 'violation-timeout', 'low-confidence-sat'}
ROOT = Path(__file__).resolve().parents[3]


def host_path(value):
    if not isinstance(value, str):
        return None
    path = Path(value)
    return str(ROOT / path.relative_to('/unsat-rust')) if path.is_relative_to('/unsat-rust') else value


def index_reports(run, job):
    matrix = job / 'matrix.json'
    if not matrix.is_file():
        return
    data = json.loads(matrix.read_text())
    source = (job / 'source.path').read_text().strip()
    catalog_path = run / 'catalog/rules.json'
    catalog = json.loads(catalog_path.read_text()).get('rules', {}) if catalog_path.is_file() else {}
    findings, errors = [], []
    for row in data.get('results', []):
        if row['status'] == 'verified':
            continue
        pair_result = host_path(row.get('pair_result_json'))
        log = host_path(row.get('log'))
        metadata = dict(catalog.get(row.get('rule'), {}))
        if pair_result and Path(pair_result).is_file():
            metadata.update(json.loads(Path(pair_result).read_text()).get('rule', {}))
        # Keep exact source wording/whitespace; do not regenerate either text.
        dsl = next((metadata[key] for key in ('expression', 'dsl')
                    if isinstance(metadata.get(key), str)), None)
        record = {'source': source, 'job': str(job), 'status': row['status'],
                  'caller': row.get('caller'), 'callsite': row.get('callsite'), 'rule': row.get('rule'),
                  'dsl': dsl, 'original_rule': metadata.get('rule'),
                  'confirmation': 'not_validated',
                  'pair_result': pair_result, 'log': log,
                  'log_gzip': log + '.gz' if log else None,
                  'klee_archive': str(Path(pair_result).parent / 'klee-out.tar.gz') if pair_result else None,
                  'control_chain': host_path(row.get('certainty_chain')),
                  'replay_archive': str(job / 'replay-inputs.tar.gz')}
        if row['status'] in FINDINGS:
            findings.append(record)
        elif row['status'] != 'verified':
            errors.append(record)
    for name, rows in (('findings', findings), ('analysis-errors', errors)):
        (job / f'{name}.json').write_text(json.dumps(rows, indent=2, ensure_ascii=False) + '\n')
        with (run / f'{name}.jsonl').open('a') as handle:
            for row in rows:
                handle.write(json.dumps(row, ensure_ascii=False) + '\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=Path)
    parser.add_argument('--job', required=True, type=Path)
    args = parser.parse_args()
    index_reports(args.run, args.job)
