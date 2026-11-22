"""Whole-document translation ablation using the current source-derived DSL."""
import hashlib
import json
from pathlib import Path
import time

import dslv2
from dsltrans import Model, DSL_SEMANTICS
from llm.openai_llm import OpenAILLM


def prepare_direct_document(args):
    from ablation_runner import (repo_root, _translation_root, _baseline_path, _load_json,
                                 _selected, _report_for_crate, _rule_index, _rule_name_index,
                                 _rules_for_target, _documentation_record)
    from ruklee_results import write_json
    root = repo_root()
    source = _baseline_path(root, args)
    baseline = _load_json(source)
    fingerprint = {'baseline_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                   'model': args.model, 'protocol': 'whole-document-source-derived-dsl-v1'}
    output = _translation_root(root, args, 'without-rule-extraction')
    output.mkdir(parents=True, exist_ok=True)
    path = output / 'direct-document-rules.json'
    document = _load_json(path) if args.resume and path.exists() else {
        'rules': {}, 'include_raw_pointer_deref': False, 'input': fingerprint,
        'processed_apis': {}, 'rejected': []}
    if document.get('input') != fingerprint:
        raise RuntimeError('whole-document cache provenance differs; choose a new cache')
    by_location, by_name = _rule_index(root, source), _rule_name_index(root, source)
    groups = {}
    for short, dirname in _selected(args):
        for target in _report_for_crate(root, short, dirname, args).get('targets', []):
            callee = target.get('callee') or {}
            if callee.get('name') == 'core::ptr::__raw_ptr_deref__':
                continue
            matched = _rules_for_target(callee=callee, by_location=by_location, by_name=by_name,
                                       type_args=target.get('callee_type_args'))
            if not matched:
                continue
            first = matched[0][1]
            key = (callee['path'], callee['line_start'], first['api'])
            groups[key] = matched
    client = None if args.dry_run else OpenAILLM(model=args.model)
    for (source_path, line, api), matched in sorted(groups.items()):
        location = f'{source_path}:{line}'
        key = hashlib.sha256(f'{location}\0{api}'.encode()).hexdigest()[:16]
        if document['processed_apis'].get(key, {}).get('status') in ('translated', 'unexpressible'):
            continue
        model = Model(root / 'tools/x/generated/dsl/schema.json', api)
        record = _documentation_record(root, source_path, line)
        system = ('Translate the entire Rust unsafe API documentation Safety contract directly into one positive Boolean expression. '
                  'Do not extract, split, paraphrase, or list atomic natural-language rules first. '
                  'Return only the complete expression or exactly UNEXPRESSIBLE if any safety requirement cannot be expressed. '
                  'Do not omit requirements, invent fields, or replace requirements with True.\n' + DSL_SEMANTICS +
                  '\nNormative memory semantics:\n' + model.memory_bytes.decode() + '\nTyped grammar:\n' + model.grammar)
        prompt = f'API: {api}\nComplete documentation and signature:\n```rust\n{record}\n```'
        directory = output / 'requests' / key
        directory.mkdir(parents=True, exist_ok=True)
        write_json(directory / 'request.json', {'model': args.model, 'system': system, 'prompt': prompt,
                                                'baseline_rule_ids': [identifier for identifier, _ in matched],
                                                'documentation_sha256': hashlib.sha256(record.encode()).hexdigest()})
        if args.dry_run:
            continue
        started = time.monotonic()
        error = None
        for attempt in range(1, 4):
            try:
                raw = client.complete(system, prompt if error is None else prompt + '\nRepair the prior response. Validator: ' + error)
                (directory / f'response-{attempt}.txt').write_text(raw)
                expression = raw.strip()
                if expression == 'UNEXPRESSIBLE':
                    result = {'status': 'unexpressible', 'expression': None}
                else:
                    lowered = dslv2.lower(model, expression)
                    lowered['source'] = expression
                    identifier = 'woe-' + key
                    document['rules'][identifier] = {
                        'id': identifier, 'path': location, 'name': matched[0][1]['name'], 'api': api,
                        'rule': record, 'dsl': expression, 'dsl_v2': lowered,
                        'baseline_rule_ids': [identifier for identifier, _ in matched],
                        'baseline_dsl': [entry['dsl'] for _, entry in matched],
                        'baseline_rules': [entry['rule'] for _, entry in matched],
                        'evaluator_kind': 'whole-document-constraint',
                        'generation_request': str(directory / 'request.json'),
                        'generation_response': str(directory / f'response-{attempt}.txt'),
                        'source_api': api}
                    result = {'status': 'translated', 'expression': expression, 'rule_id': identifier}
                error = None
                break
            except Exception as exc:
                error = str(exc)
        if error is not None:
            result = {'status': 'error', 'error': error}
        result['duration_sec'] = round(time.monotonic() - started, 3)
        result['baseline_rule_ids'] = [identifier for identifier, _ in matched]
        document['processed_apis'][key] = result
        write_json(directory / 'result.json', result)
        write_json(path, document)
    if not path.exists():
        write_json(path, document)
    return path
