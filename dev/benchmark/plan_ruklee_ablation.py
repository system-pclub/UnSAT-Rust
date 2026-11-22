"""Plan (by default), or explicitly execute a serial, resumable roster experiment."""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import concurrent.futures
import threading


ROOT = Path(__file__).resolve().parents[2]
PIPELINE = ('full', 'without-rule-extraction', 'without-rule-translation',
            'without-mut-ref-recovery', 'without-symbolic-execution', 'without-validation')
PROFILES = ('full', 'profile-a', 'profile-b', 'profile-c')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--suite', choices=('pipeline', 'profiles', 'all'), required=True)
    parser.add_argument('--setting', action='append', help='Optional subset of the chosen suite.')
    parser.add_argument('--roster', type=Path, default=ROOT / 'dev/benchmark/sources.json')
    parser.add_argument('--catalog', type=Path, required=True)
    parser.add_argument('--metadata-root', type=Path, required=True)
    parser.add_argument('--results-root', type=Path, default=ROOT / 'eval')
    parser.add_argument('--model', default='gpt-5.4-mini')
    parser.add_argument('--timeout-sec', type=int, default=180)
    parser.add_argument('--package-scope', choices=('root', 'workbook'), default='root')
    parser.add_argument('--resume', action='store_true')
    parser.add_argument('--workers', type=int, default=2)
    parser.add_argument('--execute', action='store_true', help='Without this option, only write the reviewable plan.')
    args = parser.parse_args()
    for key in ('roster', 'catalog', 'metadata_root', 'results_root'):
        path = getattr(args, key)
        setattr(args, key, (ROOT / path).resolve())
    if args.metadata_root == args.results_root or args.results_root in args.metadata_root.parents:
        parser.error('metadata-root must be outside results-root')
    settings = PIPELINE if args.suite == 'pipeline' else PROFILES if args.suite == 'profiles' else (*PIPELINE, *PROFILES[1:])
    if args.setting:
        if set(args.setting) - set(settings):
            parser.error('setting is not in the selected suite')
        settings = tuple(s for s in settings if s in args.setting)
    roster = json.loads(args.roster.read_text())
    directories = [row['directory'] for row in roster]
    if len(directories) != len(set(directories)):
        parser.error('duplicate roster entries')
    commands = []
    for setting in settings:
        entry = 'full' if setting.startswith('profile-') else setting
        for directory in directories:
            if not isinstance(directory, str) or Path(directory).name != directory or directory in ('.', '..'):
                parser.error('roster directory must be an immediate child name')
            argv = [sys.executable, str(ROOT / 'ablation' / (entry.replace('-', '_') + '.py')),
                    '--crate', directory, '--roster', str(args.roster),
                    '--results-root', str(args.results_root), '--setting-name', 'ruklee-' + setting,
                    '--metadata-root', str(args.metadata_root / 'runners' / directory),
                    '--rule-dsl', str(args.catalog / 'rules.json'),
                    '--studied-rules', str(args.catalog / 'studied_rules'),
                    '--package-scope', args.package_scope, '--model', args.model,
                    '--result-format', 'claude',
                    '--timeout-sec', str(args.timeout_sec)]
            if setting.startswith('profile-'):
                argv.extend(['--ablation-level', setting[-1]])
            if args.resume:
                argv.append('--resume')
            commands.append({'setting': setting, 'crate': directory, 'argv': argv})
    identity = {'roster_sha256': digest(args.roster), 'rules_sha256': digest(args.catalog / 'rules.json'),
                'studied_sha256': digest(args.catalog / 'studied_rules'), 'settings': list(settings),
                'package_scope': args.package_scope, 'model': args.model, 'timeout_sec': args.timeout_sec,
                'results_root': str(args.results_root)}
    args.metadata_root.mkdir(parents=True, exist_ok=True)
    plan_path = args.metadata_root / 'plan.json'
    if plan_path.exists():
        previous = json.loads(plan_path.read_text())
        if previous['identity'] != identity:
            parser.error('existing plan has different inputs/settings; use a new metadata-root')
    plan_path.write_text(json.dumps({'identity': identity, 'commands': commands}, indent=2) + '\n')
    print(f'{len(settings)} settings x {len(directories)} crates: {plan_path}', flush=True)
    if not args.execute:
        return 0
    # Share the established serial-run lock: injection and metadata paths depend
    # on crate basename and cannot be isolated with --ir-output-dir alone.
    lock_path = ROOT / '.local/run-all-crates/runner.lock'
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    with lock_path.open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        crate_locks = {directory: threading.Lock() for directory in directories}
        def execute(command):
            job = args.metadata_root / 'jobs' / command['setting'] / command['crate']
            job.mkdir(parents=True, exist_ok=True)
            result_path = job / 'run.json'
            if args.resume and result_path.exists() and json.loads(result_path.read_text()).get('returncode') == 0:
                return 0
            with crate_locks[command['crate']], (job / 'runner.log').open('a') as log:
                completed = subprocess.run(command['argv'], cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=False)
            result_path.write_text(json.dumps({**command, 'returncode': completed.returncode}, indent=2) + '\n')
            print(f"{command['setting']} {command['crate']}: exit {completed.returncode}", flush=True)
            return completed.returncode
        order = ['full', 'without-mut-ref-recovery', 'without-validation', 'without-symbolic-execution', 'without-rule-extraction', 'without-rule-translation']
        ordered = sorted(commands, key=lambda row: order.index(row['setting']) if row['setting'] in order else len(order))
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
            codes = list(pool.map(execute, ordered))
    return int(any(codes))


if __name__ == '__main__':
    raise SystemExit(main())
