"""Durable, bounded request rounds shared by rule extraction and translation."""
from __future__ import annotations

from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import time

from openai import OpenAI
from cli.cmd.extract import _write_json
from llm.openai_llm import _load_openai_api_key_from_dotenv
import os


class BatchPending(RuntimeError):
    pass


def fingerprint(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, ensure_ascii=False).encode()).hexdigest()


def client():
    key = os.environ.get('OPENAI_API_KEY') or _load_openai_api_key_from_dotenv()
    if not key:
        raise RuntimeError('OPENAI_API_KEY is required; set it in .env or the environment')
    return OpenAI(api_key=key, timeout=180, max_retries=1)


def safe_error(error):
    message = str(error)
    key = os.environ.get('OPENAI_API_KEY') or _load_openai_api_key_from_dotenv()
    return message.replace(key, '[REDACTED]') if key else message


def response_text(body, endpoint):
    if endpoint == '/v1/chat/completions':
        choices = body.get('choices', [])
        if not choices or choices[0].get('finish_reason') != 'stop':
            raise ValueError('extraction did not complete normally')
        return choices[0]['message'].get('content') or ''
    if body.get('status') != 'completed':
        raise ValueError(f"response status: {body.get('status')}")
    return ''.join(part['text'] for item in body.get('output', [])
                   if item.get('type') == 'message' for part in item.get('content', [])
                   if part.get('type') == 'output_text')


def _batch(directory, requests, endpoint, args):
    """Resume by saved job ID; output lines are joined by custom_id, never order."""
    directory.mkdir(parents=True, exist_ok=True)
    state_path = directory / 'job.json'
    identity = fingerprint({'endpoint': endpoint, 'requests': requests})
    state = json.loads(state_path.read_text()) if state_path.exists() else {'request_sha256': identity}
    if state['request_sha256'] != identity:
        raise RuntimeError(f'changed batch inputs in {directory}')
    done = directory / 'results.json'
    if done.exists():
        return json.loads(done.read_text())
    api = client()
    input_path = directory / 'input.jsonl'
    if not input_path.exists():
        input_path.write_text(''.join(json.dumps({'custom_id': k, 'method': 'POST', 'url': endpoint, 'body': v}, ensure_ascii=False)+'\n' for k,v in requests.items()))
    if not state.get('input_file_id'):
        with input_path.open('rb') as handle:
            state['input_file_id'] = api.files.create(file=handle, purpose='batch').id
        _write_json(state_path, state)
    if not state.get('job_id'):
        # A crash after remote creation but before local persistence is recovered
        # by metadata, avoiding a second paid batch on resume.
        matches = [b for b in api.batches.list(limit=100)
                   if (b.metadata or {}).get('rule_request_sha256') == identity
                   and b.input_file_id == state['input_file_id']]
        job = matches[0] if matches else api.batches.create(
            input_file_id=state['input_file_id'], endpoint=endpoint, completion_window='24h',
            metadata={'rule_request_sha256': identity})
        state['job_id'] = job.id
        _write_json(state_path, state)
    while True:
        job = api.batches.retrieve(state['job_id'])
        state.update(status=job.status, batch=job.model_dump(mode='json'))
        _write_json(state_path, state)
        print(f"[rule batch] {job.id}: {job.status}", flush=True)
        if job.status in {'completed', 'failed', 'expired', 'cancelled'}:
            break
        if args.batch_no_wait:
            raise BatchPending(f"{job.id} pending; rerun the same command to resume")
        time.sleep(args.batch_poll_seconds)
    records = {}
    for field in ('output_file_id', 'error_file_id'):
        file_id = getattr(job, field, None)
        if not file_id:
            continue
        text = api.files.content(file_id).text
        (directory / f'{field}.jsonl').write_text(text)
        for line in text.splitlines():
            if line.strip():
                record = json.loads(line)
                key = record['custom_id']
                if key not in requests or key in records:
                    raise RuntimeError(f'unknown or duplicate batch result: {key}')
                records[key] = record
    for key in requests:
        records.setdefault(key, {'error': {'message': f'batch {job.status}: missing result', 'details': state['batch'].get('errors')}})
    _write_json(done, records)
    return records


def _shards(requests):
    shard, size = {}, 0
    for key, value in requests.items():
        n = len(json.dumps(value, ensure_ascii=False).encode()) + 512
        if n > 150_000_000:
            raise ValueError(f'request {key} exceeds batch size limit')
        if shard and (size+n > 150_000_000 or len(shard) >= 40_000):
            yield shard
            shard, size = {}, 0
        shard[key], size = value, size+n
    if shard:
        yield shard


def execute(directory: Path, requests, endpoint, mode, args):
    """Persist the entire round before requests; return all raw replies before validation."""
    directory.mkdir(parents=True, exist_ok=True)
    manifest = {'mode': mode, 'endpoint': endpoint, 'requests': requests}
    path = directory / 'input.json'
    if path.exists() and json.loads(path.read_text()) != manifest:
        raise RuntimeError(f'inputs changed in {directory}; use a new output directory')
    _write_json(path, manifest)
    if mode == 'batch':
        results, pending = {}, []
        for index, shard in enumerate(_shards(requests)):
            try:
                results.update(_batch(directory / f'batch-{index:04}', shard, endpoint, args))
            except BatchPending as error:
                pending.append(str(error))
        if pending:
            raise BatchPending('; '.join(pending))
        return results
    def one(pair):
        key, request = pair
        path = directory / key / 'response.json'
        if path.exists():
            return key, json.loads(path.read_text())
        try:
            api = client()
            response = (api.chat.completions.create(**request) if endpoint == '/v1/chat/completions'
                        else api.responses.create(**request))
            record = {'response': {'status_code': 200, 'body': response.model_dump(mode='json')}}
        except Exception as error:
            record = {'error': {'message': safe_error(error)}}
        _write_json(path, record)
        return key, record
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        return dict(pool.map(one, requests.items()))


def body(record):
    response = record.get('response') or {}
    if record.get('error') or response.get('status_code') != 200:
        raise RuntimeError(json.dumps(record.get('error') or response, ensure_ascii=False))
    return response['body']
