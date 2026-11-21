"""Read existing translated step2 expressions; validate/serialize without repair or LLM calls."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools/x/src'))
import dslv2
from dsltrans import Model


def apply_supplement(rules, supplement, models):
    """Add alias paths and monomorphic rules from a hand-written supplement."""
    for entry in supplement.get('aliases', []):
        for key in entry['rules']:
            library, source_id = key.split(':')
            rule = rules[f'step2-{library}-{source_id}']
            rule['aliases'] = sorted({*rule.get('aliases', []), *entry['paths']})
    for number, row in enumerate(supplement.get('rules', []), 1):
        identifier = f'step2-supplement-{number}'
        api, expression = row['api'], row['expression']
        if api not in models:
            models[api] = Model(ROOT / 'tools/x/generated/dsl/schema.json', api)
        document = dslv2.lower(models[api], expression)
        document['source'] = expression
        rules[identifier] = {**row, 'id': identifier, 'status': 'translated', 'source_rule_id': identifier,
                             'source_library': row['library'], 'dsl': expression, 'dsl_v2': document}
    return len(supplement.get('rules', []))


def prepare(source, output, supplement=None):
    output.mkdir(parents=True, exist_ok=True)
    rules, excluded, errors, inputs, models = {}, [], [], {}, {}
    for library in ('alloc', 'core', 'std'):
        path = source / library / 'step2.json'
        payload = path.read_bytes()
        rows = json.loads(payload)
        inputs[library] = {'path': str(path), 'sha256': hashlib.sha256(payload).hexdigest(),
                           'statuses': dict(Counter(row.get('status', 'missing') for row in rows))}
        for row in rows:
            if row.get('status') != 'translated':
                excluded.append({'library': library, **row})
                continue
            # Namespace eval IDs: legacy rule-89/rule-192 have special frontend
            # meanings and are unrelated to the generated catalog's numbering.
            identifier = f"step2-{library}-{row['id']}"
            if identifier in rules:
                raise RuntimeError(f'Duplicate source rule: {identifier}')
            try:
                api, expression = row['api'], row['expression']
                if api not in models:
                    models[api] = Model(ROOT / 'tools/x/generated/dsl/schema.json', api)
                document = dslv2.lower(models[api], expression)
                document['source'] = expression
                rules[identifier] = {**row, 'id': identifier, 'source_rule_id': row['id'],
                                     'source_library': library, 'dsl': expression, 'dsl_v2': document}
            except Exception as error:
                errors.append({'library': library, **row, 'lowering_error': str(error)})
    added = apply_supplement(rules, json.loads(supplement.read_text()), models) if supplement else 0
    manifest = {'llm_requests': 0, 'supplement_rules': added, 'include_raw_pointer_deref': True,
                'inputs': inputs, 'translated': len(rules),
                'excluded_non_translated': len(excluded), 'lowering_errors': errors}
    (output / 'catalog-manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n')
    (output / 'excluded.json').write_text(json.dumps(excluded, indent=2, ensure_ascii=False) + '\n')
    if errors:
        raise RuntimeError(f"{len(errors)} translated expressions could not be serialized; originals retained in catalog-manifest.json; no rules repaired or silently skipped")
    (output / 'rules.json').write_text(json.dumps(
        {'rules': rules, 'include_raw_pointer_deref': True, 'source': inputs},
        indent=2, ensure_ascii=False) + '\n')
    # sync retains every MIRScan target even with an empty legacy-rule allowlist.
    # Actual rules are matched later from the eval catalog by callee identity.
    (output / 'studied_rules').write_text('# No legacy rule IDs; eval IDs are matched from rules.json.\n')
    print(json.dumps({k: v for k, v in manifest.items() if k != 'inputs'}, ensure_ascii=False))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, default=ROOT / 'eval/rule')
    parser.add_argument('--supplement', type=Path, default=ROOT / 'tools/x/scripts/step2_supplement.json',
                        help='Hand-written aliases/monomorphic rules; pass an empty value to disable.')
    parser.add_argument('--output', type=Path, required=True)
    arguments = parser.parse_args()
    prepare(arguments.source, arguments.output, arguments.supplement if arguments.supplement.name else None)
