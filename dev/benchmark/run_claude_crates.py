#!/usr/bin/env python3
"""Resume whole-crate Claude judgments for the fixed TASK roster; keep all attempts."""
import argparse
import concurrent.futures
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import threading

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('claude_runner', ROOT/'docker/rust-comparison/run-claude.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
MODEL = 'sonnet'
TIMEOUT = 1800


def matching_result(name):
    candidates = sorted((ROOT/'eval/claude-crate'/name).glob('*/result.json'), key=lambda p:p.stat().st_mtime, reverse=True)
    for path in candidates:
        try:
            result=json.loads(path.read_text())
            request=json.loads((path.parent/'request.json').read_text())
            runner.validate_crate_result(result)
            if request.get('request')=={'mode':'crate','model':MODEL} and request.get('prompt')==runner.crate_prompt():
                return path
        except (OSError, ValueError):
            continue
    return None


def write_json(path, value):
    tmp=path.with_suffix(path.suffix+'.tmp')
    tmp.write_text(json.dumps(value,indent=2)+'\n')
    tmp.replace(path)


def summarize(names):
    rows=[]
    for name in names:
        path=matching_result(name)
        if path is None:
            rows.append({'crate':name,'status':'missing'})
            continue
        result=json.loads(path.read_text()); request=json.loads((path.parent/'request.json').read_text())
        execution=result.get('execution',{})
        row=dict(crate=name,status='completed',violations=len(result['violations']),
                 classification='unreviewed model findings',model=list((execution.get('modelUsage') or {}).keys()),
                 seconds=round((execution.get('duration_ms') or 0)/1000,3),cost_usd=execution.get('total_cost_usd'),
                 result=str(path.relative_to(ROOT)),request=str((path.parent/'request.json').relative_to(ROOT)),
                 image_id=request.get('image_id'))
        rows.append(row)
        write_json(path.parent.parent/'summary.json',row)
    write_json(ROOT/'dev/benchmark/claude-results.json', rows)
    text='# Claude whole-crate results\n\nFindings are unreviewed model reports, not confirmed bugs.\n\n| Crate | Status | Candidate violations | Model | Seconds | Result |\n|---|---|---:|---|---:|---|\n'
    for row in rows:
        link=f"[result.json](../../{row['result']})" if row.get('result') else ''
        text+=f"| {row['crate']} | {row['status']} | {row.get('violations','')} | {', '.join(row.get('model',[]))} | {row.get('seconds','')} | {link} |\n"
    text+='\nRequests retain the exact prompt, input source hashes and image identity. Raw stdout/stderr, structured findings and execution usage are retained in each attempt. The three original runs are reused. New runs use the same prompt, sonnet alias, 1800-second timeout, Read/Glob/Grep tools, JSON schema, original crate snapshots and core/std/alloc sources.\n'
    (ROOT/'dev/benchmark/claude-results.md').write_text(text)
    return rows


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workers',type=int,default=3)
    parser.add_argument('--summarize-only',action='store_true')
    args=parser.parse_args()
    names=[row['directory'] for row in json.loads((ROOT/'dev/benchmark/sources.json').read_text())]
    if len(names)!=32 or len(set(names))!=32:
        raise ValueError('Expected the exact 32-crate roster')
    if args.summarize_only:
        rows=summarize(names)
        print(json.dumps({'completed':sum(r['status']=='completed' for r in rows),'total':len(rows),'violations':sum(r.get('violations',0) for r in rows)}))
        return
    libraries=ROOT/'tools/rust/library'
    hashes={str(p.relative_to(libraries)):hashlib.sha256(p.read_bytes()).hexdigest() for name in ('core','std','alloc') for p in sorted((libraries/name).rglob('*')) if p.is_file()}
    if hashes!=json.loads((ROOT/'dev/benchmark/claude-library-hashes.json').read_text()):
        raise ValueError('Rust library sources differ from the three initial runs')
    runner.api_key()
    done=[name for name in names if matching_result(name)]
    pending=[name for name in names if name not in done]
    batch_id=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    batch=ROOT/'dev/benchmark/claude-crate-batches'/batch_id
    batch.mkdir(parents=True)
    logs=ROOT/'.local/claude-crate-batches'/batch_id
    logs.mkdir(parents=True)
    manifest=dict(batch=batch_id,total=len(names),reused=done,pending=pending,model=MODEL,timeout=TIMEOUT,
                  workers=args.workers,prompt=runner.crate_prompt(),schema=runner.CRATE_SCHEMA,
                  libraries_hashes='dev/benchmark/claude-library-hashes.json',results=[])
    write_json(batch/'batch.json',manifest)
    # Reproduce the original host networking without touching host Docker settings.
    bin_dir=logs/'bin'; bin_dir.mkdir()
    shim=bin_dir/'docker'
    shim.write_text('#!/usr/bin/env bash\nset -euo pipefail\nop="${1:-}"\nif [[ "$op" == run ]]; then shift; exec /usr/bin/docker run --network host "$@"; fi\nif [[ "$op" == build && "$*" == *Dockerfile.claude* ]]; then shift; exec /usr/bin/docker build --network host "$@"; fi\nexec /usr/bin/docker "$@"\n')
    shim.chmod(0o755)
    os.environ['PATH']=str(bin_dir)+os.pathsep+os.environ['PATH']
    with (logs/'claude-image.log').open('w') as log:
        # The ordinary runner builds the image from the same Dockerfile and libraries.
        import contextlib
        with contextlib.redirect_stderr(log):
            runner.build_image(libraries)
    pinned='uspec-claude-crate-batch:'+batch_id.lower()
    subprocess.run(['docker','image','tag',runner.IMAGE,pinned],check=True)
    runner.IMAGE=pinned
    image=json.loads(subprocess.check_output(['docker','image','inspect',pinned],text=True))[0]
    manifest['image_id']=image['Id']
    manifest['image_tag']=pinned
    write_json(batch/'image.json',{k:image.get(k) for k in ('Id','RepoTags','Descriptor','RootFS')})
    write_json(batch/'batch.json',manifest)
    build_lock=threading.Lock(); save_lock=threading.Lock()
    def work(name):
        output=ROOT/'eval/claude-crate'/name/batch_id
        record={'crate':name,'output':str(output.relative_to(ROOT))}
        try:
            # Build-first is required before each container; serialize image-tag mutations.
            with build_lock, (logs/(name+'.root-image.log')).open('w') as log:
                subprocess.run(['docker','build','--build-arg',f'USER_ID={os.getuid()}',
                                '--build-arg',f'GROUP_ID={os.getgid()}','-t','unsat-rust:local',str(ROOT)],
                               check=True,stdout=log,stderr=subprocess.STDOUT,timeout=300)
            value=runner.judge_crate(ROOT/'benchmark/original'/name,output,model=MODEL,timeout=TIMEOUT)
            record.update(status='completed',violations=len(value['violations']),model=list((value.get('execution',{}).get('modelUsage') or {}).keys()))
        except Exception as exc:
            record.update(status='timeout' if isinstance(exc,subprocess.TimeoutExpired) else 'error',error=str(exc))
        with save_lock:
            manifest['results'].append(record)
            write_json(batch/'batch.json',manifest)
            print(json.dumps(record),flush=True)
        # Required post-run disk checks remain outside the source snapshots.
        with (logs/(name+'.disk.log')).open('w') as log:
            subprocess.run(['df','-h',str(ROOT)],stdout=log,stderr=subprocess.STDOUT)
            subprocess.run(['du','-sh',str(ROOT/'eval'),str(ROOT/'.local'),str(ROOT/'crates_inj')],stdout=log,stderr=subprocess.STDOUT)
        return record
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        results=list(pool.map(work,pending))
    rows=summarize(names)
    print(json.dumps({'completed':sum(r['status']=='completed' for r in rows),'total':len(rows),'violations':sum(r.get('violations',0) for r in rows),'batch':str(batch.relative_to(ROOT))}),flush=True)
    if any(r['status']!='completed' for r in results):
        raise SystemExit(1)


if __name__=='__main__':
    main()
