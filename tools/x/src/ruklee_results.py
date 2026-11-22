"""Claude-shaped RuKLEE reports, retaining stage verdicts and native evidence."""
from __future__ import annotations

import json
import hashlib
import tarfile
import shutil
import re
from pathlib import Path
from collections import Counter


def pair_report(native, *, native_path, setting=None):
    site = native.get('callsite', {})
    rule = native.get('rule', {})
    stages = native.get('stages', {})
    init, llm, rerun = (stages.get(key, {}) for key in ('init', 'llm', 'rerun'))
    status = init.get('status') or native.get('result', {}).get('init_status') or 'unknown'
    confirmed = bool(rerun.get('full_rerun_passed'))
    candidate = confirmed or status == 'violation' or (status == 'violation-timeout' and init.get('has_certain_symbol') is True)
    explanation = ('The target callsite and rule violation were reproduced in the testcase rerun.' if confirmed else
                   'Initial symbolic composition found a rule violation; testcase confirmation is not established.')
    seconds = init.get('duration_sec')
    poc_seconds = llm.get('duration_sec')
    duration = seconds + poc_seconds if isinstance(seconds, (int, float)) and isinstance(poc_seconds, (int, float)) else seconds
    return {
        'schema_version': 1,
        'violations': [{'file': site.get('path'), 'function': site.get('caller'),
                        'unsafe_api': site.get('unsafe_callee'), 'requirement': rule.get('rule'),
                        'explanation': explanation}] if candidate else [],
        'crate': Path(native.get('crate', '')).name,
        'setting': setting or Path(native_path).parents[3].name.removeprefix('ruklee-'),
        'function': site.get('caller'), 'callsite': site,
        'rule_id': rule.get('id'), 'rule': rule.get('rule'),
        'dsl': rule.get('dsl') or rule.get('expression'),
        **({'baseline_dsl': rule.get('baseline_dsl'), 'baseline_rule_ids': rule.get('baseline_rule_ids'),
            'evaluator_input': {'kind': rule.get('evaluator_kind'), 'smtlib': rule.get('smtlib'),
                                'selectors': rule.get('selector_text')}} if rule.get('evaluator_kind') else {}),
        'status': status, 'backend': 'klee',
        'classification': 'reproduced_violation' if confirmed else 'unconfirmed_candidate' if candidate else 'verified_rule' if status == 'verified' else 'inconclusive',
        'full_rerun_passed': confirmed,
        'execution': {'duration_ms': round(duration * 1000, 3) if isinstance(duration, (int, float)) else None,
                      'duration_sec': duration, 'compose_duration_sec': seconds,
                      'testcase_duration_sec': poc_seconds, 'returncode': init.get('returncode'),
                      'timed_out': init.get('timed_out'), 'duration_scope': 'compose_and_testcase' if poc_seconds is not None else 'compose_only'},
        'validation_status': {'testcase': llm.get('status'), 'rerun': rerun.get('status')},
        'error': llm.get('error'),
        'evidence': {'native_result': str(native_path), 'metadata': native.get('metadata'),
                     'archive_index': str(Path(native_path).parents[2] / 'archive-index.json'),
                     'compose_log': init.get('log'), 'control_chain': init.get('certainty_chain'),
                     'testcase_prompt': llm.get('prompt'), 'testcase_response': llm.get('response'),
                     'generation_request': rule.get('generation_request'), 'generation_response': rule.get('generation_response'),
                     'rerun_log': rerun.get('log')},
    }


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + '.tmp')
    temporary.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n')
    temporary.replace(path)


def finish_crate(output, *, setting, crate, catalog, returncode, duration_sec, error=None, metadata=None):
    """Preserve errors and untranslated callsites instead of treating emptiness as safe."""
    matrix_path = output / 'full-matrix-results.json'
    matrix = json.loads(matrix_path.read_text()) if matrix_path.exists() else {}
    rows = matrix.get('results', [])
    sites = matrix.get('callsites', [])
    if not sites and metadata and Path(metadata).exists():
        report = json.loads(Path(metadata).read_text()).get('report', {})
        sites = [{'callsite_id': (target.get('callsite') or {}).get('id'),
                  'path': (target.get('callsite') or {}).get('path'),
                  'line': (target.get('callsite') or {}).get('line'),
                  'caller': (target.get('caller') or {}).get('name'), 'rules': []}
                 for target in report.get('targets', [])]
    uncovered = []
    for site in sites:
        if site.get('rules'):
            continue
        identifier = site.get('callsite_id') or f"line-{site.get('line', 'unknown')}"
        record = {'schema_version': 1, 'violations': [], 'crate': crate, 'setting': setting,
                  'function': site.get('caller'), 'callsite': site,
                  'rule_id': None, 'rule': None, 'dsl': None, 'status': 'no-translated-rule',
                  'classification': 'uncovered', 'backend': 'klee' if setting != 'without-symbolic-execution' else 'llm',
                  'execution': {'duration_ms': None, 'duration_sec': None},
                  'evidence': {'catalog': str(catalog), 'matrix': str(matrix_path)}}
        write_json(output / identifier / '__uncovered__' / 'result.json', record)
        uncovered.append(record)
    counts = dict(Counter(row.get('status', 'unknown') for row in rows))
    violations = []
    for row in rows:
        identifier = re.sub(r'[^A-Za-z0-9_.-]+', '-', str(row.get('callsite', 'unknown'))).strip('-')
        pair_path = output / identifier / str(row.get('rule')) / 'result.json'
        if pair_path.exists():
            pair = json.loads(pair_path.read_text())
            violations.extend(pair.get('violations', []))
    status = 'error' if returncode else 'no-translated-rules' if not rows and sites else 'no-callsites' if not rows else 'completed'
    if not matrix:
        matrix = {'schema_version': 1, 'crate': crate, 'setting': setting, 'status': status,
                  'results': [], 'callsites': sites, 'counts': {}, 'completed': 0, 'total': 0,
                  'error': error or (f'verification exited {returncode}; inspect scheduling runner.log' if returncode else None)}
        write_json(matrix_path, matrix)
    summary = {'schema_version': 1, 'crate': crate, 'setting': setting, 'status': status,
               'violations': violations, 'pair_count': len(rows), 'uncovered_callsites': len(uncovered),
               'counts': counts, 'execution': {'returncode': returncode, 'duration_sec': duration_sec,
                                              'duration_ms': round(duration_sec * 1000, 3)},
               'evidence': {'matrix': str(matrix_path), 'catalog': str(catalog)},
               'error': error or (f'verification exited {returncode}; inspect scheduling runner.log' if returncode else None)}
    write_json(output / 'summary.json', summary)
    return summary


def archive_crate(root, output, crate_slug):
    """Retain lossless replay evidence, then prune only this completed run's IR/KLEE copies."""
    index = []
    for directory in list(output.glob('*/*/klee-*')):
        if not directory.is_dir() or directory.is_symlink():
            continue
        if directory.name not in ('klee-out', 'klee-rerun', 'klee-rerun-sym') and not directory.name.startswith('klee-rerun-attempt-'):
            continue
        archive = directory.with_name(directory.name + '.tar.gz')
        temporary = archive.with_suffix('.gz.tmp')
        with tarfile.open(temporary, 'w:gz', compresslevel=1) as handle:
            handle.add(directory, arcname=directory.name)
        with tarfile.open(temporary, 'r:gz') as handle:
            handle.getmembers()
        temporary.replace(archive)
        with archive.open('rb') as handle:
            checksum = hashlib.file_digest(handle, 'sha256').hexdigest()
        archive.with_suffix(archive.suffix + '.sha256').write_text(checksum + '\n')
        index.append({'original': str(directory), 'archive': str(archive), 'sha256': checksum})
        shutil.rmtree(directory)
    replay = output / 'replay-inputs.tar.gz'
    if (output / 'ir').exists():
        temporary = replay.with_suffix('.gz.tmp')
        injected = root / 'crates_inj' / crate_slug
        excluded = str(injected.relative_to(root) / 'target')
        def retain(info):
            return None if info.name == excluded or info.name.startswith(excluded + '/') else info
        with tarfile.open(temporary, 'w:gz', compresslevel=1) as handle:
            for path in (output / 'ir', injected, root / 'crates' / f'{crate_slug}.json'):
                if path.exists():
                    handle.add(path, arcname=str(path.relative_to(root)), filter=retain)
        with tarfile.open(temporary, 'r:gz') as handle:
            handle.getmembers()
        temporary.replace(replay)
        with replay.open('rb') as handle:
            checksum = hashlib.file_digest(handle, 'sha256').hexdigest()
        replay.with_suffix('.gz.sha256').write_text(checksum + '\n')
        index.append({'original': str(output / 'ir'), 'archive': str(replay), 'sha256': checksum})
        shutil.rmtree(output / 'ir')
    write_json(output / 'archive-index.json', {'archives': index})


def restore_pair_outputs(output):
    # These are our own archives, restricted to a single named KLEE directory.
    for archive in output.glob('*/*/klee-*.tar.gz'):
        directory = archive.with_name(archive.name.removesuffix('.tar.gz'))
        if directory.exists():
            continue
        with tarfile.open(archive) as handle:
            handle.extractall(archive.parent, filter='data')
