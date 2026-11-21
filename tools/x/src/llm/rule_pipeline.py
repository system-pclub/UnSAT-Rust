"""The two LLM stages of x rule, with durable inputs and bounded repair rounds."""
from __future__ import annotations

import csv
import json
from pathlib import Path

from cli.cmd.extract import SYSTEM_PROMPT, _parse_rules, _source_items, _write_json
from llm import rule_jobs as jobs


def extract(output, args):
    document = json.loads((output / 'step0.json').read_text())
    model_name, mode = args.model, args.step1_mode
    if getattr(args, 'resume_step1_input', False):
        if args.api_location or args.limit is not None:
            raise RuntimeError('--resume-step1-input cannot change the saved API selection')
        saved = json.loads((output / 'step1-input.json').read_text())
        if saved.get('source_sha256') and saved['source_sha256'] != jobs.fingerprint(document):
            raise RuntimeError('source changed since saved step 1 inputs')
        items = saved['items']
        current = _source_items(document)
        if any(item not in current for item in items):
            raise RuntimeError('source changed since saved step 1 inputs')
        keyed = {'api-' + jobs.fingerprint(item)[:24]: item for item in items}
        directory = Path(saved['requests_directory'])
        path = directory / 'round-00/input.json'
        frozen = json.loads(path.read_text()) if keyed else {'requests': {}, 'mode': saved['mode']}
        requests = frozen['requests']
        if set(requests) != set(keyed) or frozen['mode'] != saved['mode']:
            raise RuntimeError('saved step 1 request manifest does not match its item inventory')
        model_name, mode = saved['model'], saved['mode']
        print(f'[rule] {output.name}: resuming saved {mode} inputs ({model_name})', flush=True)
    else:
        items = _source_items(document)
        if args.limit is not None:
            items = items[:args.limit]
        from llm.rule_context import index_for
        index = index_for(output)
        if args.api_location:
            wanted = set(args.api_location)
            items = [item for item in items if f"{item['path']}:{item['line']}" in wanted]
        requests = {}
        keyed = {}
        for item in items:
            key = 'api-' + jobs.fingerprint(item)[:24]
            keyed[key] = item
            requests[key] = {'model': args.model, 'reasoning_effort': args.reasoning_effort, 'messages': [
                {'role': 'system', 'content': SYSTEM_PROMPT},
                {'role': 'user', 'content': json.dumps(index.context(item), ensure_ascii=False)}]}
        identity = jobs.fingerprint(requests)[:24]
        directory = output / 'requests' / identity
        _write_json(output / 'step1-input.json', {'mode': args.step1_mode, 'model': args.model,
                    'items': items, 'source_sha256': jobs.fingerprint(document), 'requests_directory': str(directory)})
    repairs = {}
    repair_path = output / 'step1-repairs.json'
    if getattr(args, 'resume_step1_input', False) and repair_path.exists():
        ledger = json.loads(repair_path.read_text())
        if ledger['source_sha256'] != jobs.fingerprint(document):
            raise RuntimeError('source changed since recorded step 1 repairs')
        for identifier, entry in ledger['repairs'].items():
            result = entry['result']
            item = {field: result[field] for field in ('path', 'line', 'name', 'declaration', 'parent', 'documentation')}
            repair_key = 'api-' + jobs.fingerprint(item)[:24]
            if identifier != jobs.fingerprint(item) or repair_key not in keyed or result['status'] != 'extracted':
                raise RuntimeError('step 1 repair inventory does not match saved inputs')
            repairs[repair_key] = result
    results = {}
    attempt = 0
    while requests:
        round_dir = directory / f"round-{attempt:02}"
        # Replay retained raw responses before selecting failed requests. Each
        # invocation submits at most one fresh extraction round.
        existing = (round_dir / "input.json").exists()
        records = jobs.execute(round_dir, requests, '/v1/chat/completions', mode, args)
        failed = {}
        for key in requests:
            if key in repairs:
                results[key] = repairs[key]
                continue
            item = keyed[key]
            result = {**item, 'model': model_name, 'mode': mode, 'rules': []}
            try:
                text = jobs.response_text(jobs.body(records[key]), '/v1/chat/completions')
                result.update(status='extracted', rules=_parse_rules(text))
            except Exception as error:
                result.update(status='error', error=jobs.safe_error(error))
                failed[key] = requests[key]
            results[key] = result
            _write_json(round_dir / key / 'result.json', result)
        requests = failed
        attempt += 1
        if not existing:
            break
    ordered = [results[key] for key in keyed]
    _write_json(output / 'step1.json', ordered)
    if any(r['status'] != 'extracted' for r in ordered):
        raise RuntimeError(f'extraction failed in {output}; rerun to retry failed items only')


def api_for(row, document):
    # Source context includes enclosing impl/trait and generic declarations.
    filename, _, line = row['path'].rpartition(':')
    units = [u for path, entries in document['files'].items()
             if path.removeprefix('./').removeprefix('tools/') == filename
             for u in entries if int(u['line_number_start']) == int(line)]
    unit = next((u for u in units if u['declaration_text'] == row['name']), None)
    if unit is None:
        raise ValueError(f"missing source unit for {row['id']}")
    declaration = unit['declaration_text'].strip()
    enclosing = unit.get('enclosing_declaration') or ''
    if enclosing and enclosing.strip() != declaration:
        # Only the header is needed; the source body is never an input variable.
        declaration = enclosing.strip() + ' { ' + declaration + ' }'
    return declaration, filename


def translate(root, output, args):
    import dsltrans
    import dslapi
    from dsltypes import TypeCatalog

    schema_path = root / args.dslgen
    if schema_path.is_dir():
        schema_path = schema_path / 'schema.json'
    schema = json.loads(schema_path.read_text())
    catalog = TypeCatalog(schema)
    with (output / 'rules.csv').open() as handle:
        rows = list(csv.DictReader(handle))
    if args.rule_limit is not None:
        rows = rows[:args.rule_limit]
    document = json.loads((output / 'step0.json').read_text())
    implementation = {name: jobs.fingerprint((root / 'tools/x/src' / name).read_text())
                      for name in ('dsltrans.py', 'dslapi.py', 'dsltypes.py', 'dslgrammar.py', 'dslmemory.py', 'dslmemory.json', 'llm/rule_pipeline.py')}
    identity = jobs.fingerprint({'rows': rows, 'source': document, 'schema': schema,
                'implementation': implementation, 'model': args.model, 'effort': args.reasoning_effort,
                'tokens': args.max_output_tokens})[:24]
    directory = output / 'translations' / identity
    directory.mkdir(parents=True, exist_ok=True)
    _write_json(output / 'step2-input.json', {'mode': args.step2_mode, 'model': args.model,
                'retries': args.retries, 'rules': rows, 'translations_directory': str(directory)})
    requests, cases = {}, {}
    # Prepare every input before any paid call. Preparation errors remain visible
    # per rule and never masquerade as accepted translations.
    results = {}
    for row in rows:
        key = row['id']
        try:
            api, source = api_for(row, document)
            interface = dslapi.ApiInterface(catalog, api)
            api = dslapi.qualify_api({'api': api, 'api_variables': interface.records(), 'source': source}, catalog)
            model = dsltrans.Model(schema_path, api, row['rule'])
            requests[key] = dsltrans.build_request(model, api, row['rule'], args)
            cases[key] = {**row, 'api': api}
            location = directory / key
            _write_json(location / 'api_variables.json', model.interface.records())
            _write_json(location / 'schema.json', model.schema)
            (location / 'prompt_grammar.ebnf').write_text(model.grammar)
            (location / 'memory-schema.json').write_bytes(model.memory_bytes)
        except Exception as error:
            results[key] = {**row, 'status': 'preparation_error', 'error': jobs.safe_error(error)}
    # Replaying persisted rounds makes interruption between validation and the next
    # submission safe, and raising --retries extends rather than restarts the run.
    round_limit = min(args.retries, getattr(args, "_translation_round", args.retries))
    for attempt in range(round_limit + 1):
        if not requests:
            break
        round_dir = directory / f'round-{attempt:02}'
        records = jobs.execute(round_dir, requests, '/v1/responses', args.step2_mode, args)
        failed = {}
        for key, request in requests.items():
            result = {**cases[key], 'attempt': attempt, 'mode': args.step2_mode,
                      'validation': {'semantic_fidelity': 'not_verified'}}
            text = ''
            try:
                raw = jobs.body(records[key])
                text = jobs.response_text(raw, '/v1/responses')
                result.update(dsltrans.parse_model_output(text))
                if result['status'] == 'translated':
                    model = dsltrans.Model(schema_path, cases[key]['api'], cases[key]['rule'])
                    result['validation'] = model.validate(result['expression'])
                    (directory / key / 'grammar.lark').write_text(model.bound_grammar.lark())
                result.update(response_id=raw.get('id'), usage=raw.get('usage'))
            except Exception as error:
                result.update(status='validation_error', error=jobs.safe_error(error))
                # Retain full prior input and diagnostic in each repair prompt.
                failed[key] = {**request, 'input': request['input'] + [
                    {'role': 'assistant', 'content': text or '(no completed response)'},
                    {'role': 'user', 'content': 'The previous output failed validation: ' + result['error'] +
                     '\nReturn a corrected complete expression, or UNEXPRESSIBLE if the whole requirement cannot be expressed.'}]}
            _write_json(round_dir / key / 'result.json', result)
            _write_json(directory / key / 'result.json', result)
            (round_dir / key / 'response.txt').write_text(text)
            results[key] = result
            print(f"[rule] {output.name} {key} attempt {attempt}: {result['status']}", flush=True)
        _write_json(round_dir / 'retry-queue.json', list(failed))
        _write_json(output / 'step2.json', [results[row['id']] for row in rows])
        requests = failed
    ordered = [results[row['id']] for row in rows]
    _write_json(output / 'step2.json', ordered)
    _write_json(directory / 'generation-results.json', ordered)
    if any(r['status'] == 'preparation_error' for r in ordered):
        raise RuntimeError(f"preparation errors in {output / 'step2.json'}; inspect source context/schema")
    if requests and round_limit < args.retries:
        return True
    if any(r['status'] not in {'translated', 'unexpressible'} for r in ordered):
        raise RuntimeError(f"incomplete translations in {output / 'step2.json'}; inspect errors or increase --retries")
    return False
