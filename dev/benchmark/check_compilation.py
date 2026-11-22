#!/usr/bin/env python3
"""Compile isolated benchmark copies inside the selected tool's container."""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('tool', choices=['ruklee','culpa','safe4u'])
    parser.add_argument('--crates', nargs='*')
    parser.add_argument('--attempt', default='compile-1')
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--locked', action='store_true')
    parser.add_argument('--reuse-work', help='Reuse this previous attempt target cache after source repairs')
    args = parser.parse_args()
    compiler = (str(ROOT/'tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc')
                if args.tool == 'ruklee' else 'rustc')
    env = dict(os.environ, CARGO_BUILD_JOBS='2', CARGO_HOME=str(ROOT/'.local/cargo-home'))
    if args.tool == 'ruklee':
        env.update(RUSTC=compiler, RUSTUP_TOOLCHAIN='1.87.0', LD_LIBRARY_PATH=str(ROOT/'tools/rust/build/x86_64-unknown-linux-gnu/stage1/lib'))
    elif args.tool == 'safe4u':
        env['RUSTUP_TOOLCHAIN'] = '1.86.0'
    version = subprocess.check_output([compiler,'--version'], env=env, text=True).strip()
    crates = args.crates or sorted(p.name for p in (ROOT/'benchmark/original').iterdir() if (p/'Cargo.toml').exists())
    def check(name):
        source = ROOT/'benchmark'/args.tool/name
        output = ROOT/'eval'/args.tool/name/args.attempt
        output.mkdir(parents=True, exist_ok=False)
        work = ROOT/'.local/benchmark-compile'/args.tool/name/(args.reuse_work or args.attempt)
        shutil.copytree(source,work,ignore=shutil.ignore_patterns('target','.git'),
                        dirs_exist_ok=bool(args.reuse_work))
        # Preserve the depth used by benchmark snapshots' ../../../patched paths.
        manifest = work/'Cargo.toml'
        text = manifest.read_text()
        if '../../../patched/' in text:
            text = text.replace('../../../patched/',str(ROOT/'patched')+'/')
            manifest.write_text(text)
        command = ['cargo','check','--lib','--manifest-path',str(manifest)]
        if args.locked:
            command.append('--locked')
        start = time.time()
        try:
            result = subprocess.run(command,env=env,capture_output=True,text=True,timeout=900)
            (output/'stdout.log').write_text(result.stdout)
            (output/'stderr.log').write_text(result.stderr)
            status = 'passed' if result.returncode==0 else 'failed'
            code = result.returncode
        except subprocess.TimeoutExpired as exc:
            status, code = 'timeout',None
            (output/'stderr.log').write_text((exc.stderr or b'').decode() if isinstance(exc.stderr,bytes) else exc.stderr or '')
        if (work/'Cargo.lock').exists():
            shutil.copy2(work/'Cargo.lock',output/'Cargo.lock')
        record=dict(crate=name,tool=args.tool,compiler=version,command=command,status=status,returncode=code,
                    seconds=round(time.time()-start,2),source_hashes={str(p.relative_to(source)):hashlib.sha256(p.read_bytes()).hexdigest()
                    for p in sorted(source.rglob('*')) if p.is_file() and 'target' not in p.relative_to(source).parts})
        (output/'run.json').write_text(json.dumps(record,indent=2)+'\n')
        print(name,status,flush=True)
        return record
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        records=list(pool.map(check,crates))
    print(json.dumps({s:sum(r['status']==s for r in records) for s in ('passed','failed','timeout')}))

if __name__ == '__main__':
    main()
