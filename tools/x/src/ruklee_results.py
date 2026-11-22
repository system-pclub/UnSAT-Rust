"""Claude-shaped RuKLEE reports, retaining stage verdicts and native evidence."""
from __future__ import annotations

import json
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
        'status': status, 'backend': 'klee',
        'classification': 'reproduced_violation' if confirmed else 'unconfirmed_candidate' if candidate else 'verified_rule' if status == 'verified' else 'inconclusive',
        'full_rerun_passed': confirmed,
        'execution': {'duration_ms': round(duration * 1000, 3) if isinstance(duration, (int, float)) else None,
                      'duration_sec': duration, 'compose_duration_sec': seconds,
                      'testcase_duration_sec': poc_seconds, 'returncode': init.get('returncode'),
                      'timed_out': init.get('timed_out'), 'duration_scope': 'compose_and_testcase' if poc_seconds is not None else 'compose_only'},
        'evidence': {'native_result': str(native_path), 'metadata': native.get('metadata'),
                     'compose_log': init.get('log'), 'control_chain': init.get('certainty_chain'),
                     'testcase_prompt': llm.get('prompt'), 'testcase_response': llm.get('response'),
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
    status = 'error' if returncode else 'no-translated-rules' if not rows and sites else 'no-callsites' if not rows else 'completed'
    summary = {'schema_version': 1, 'crate': crate, 'setting': setting, 'status': status,
               'violations': [], 'pair_count': len(rows), 'uncovered_callsites': len(uncovered),
               'counts': counts, 'execution': {'returncode': returncode, 'duration_sec': duration_sec,
                                              'duration_ms': round(duration_sec * 1000, 3)},
               'evidence': {'matrix': str(matrix_path) if matrix else None, 'catalog': str(catalog)}, 'error': error}
    write_json(output / 'summary.json', summary)
    return summary
