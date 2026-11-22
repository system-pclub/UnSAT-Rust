#!/usr/bin/env python3
"""Retain successful dependency locks and summarize all completed corpus checks."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

ROOT=Path(__file__).resolve().parents[2]

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--freeze-locks',action='store_true')
    args=parser.parse_args()
    path=ROOT/'dev/benchmark/sources.json'
    records=json.loads(path.read_text())
    matrix=[]
    for row in records:
        name=row['directory']; original=ROOT/'benchmark/original'/name
        changes={}
        for tool in ('culpa','ruklee','safe4u'):
            source=ROOT/'benchmark'/tool/name
            runs=sorted((ROOT/'eval'/tool/name).glob('compile-*/run.json'),key=lambda p:p.stat().st_mtime)
            record=json.loads(runs[-1].read_text()) if runs else {'status':'not_run'}
            if args.freeze_locks and record['status']=='passed' and (runs[-1].parent/'Cargo.lock').exists():
                shutil.copyfile(runs[-1].parent/'Cargo.lock',source/'Cargo.lock')
            matrix.append(dict(crate=name,tool=tool,status=record['status'],compiler=record.get('compiler'),
                               evidence=str(runs[-1].relative_to(ROOT)) if runs else None))
            diff=[]
            for p in sorted(original.rglob('*')):
                if p.is_file():
                    relative=p.relative_to(original); q=source/relative
                    if not q.exists() or p.read_bytes()!=q.read_bytes(): diff.append(str(relative))
            for p in sorted(source.rglob('*')):
                relative=p.relative_to(source)
                if p.is_file() and not {'target','.git'}.intersection(relative.parts) and not (original/relative).exists():
                    diff.append('added: '+str(relative))
            changes[tool]=diff
        row['differences_from_original']=changes
    path.write_text(json.dumps(records,indent=2)+'\n')
    (ROOT/'dev/benchmark/compilation-matrix.json').write_text(json.dumps(matrix,indent=2)+'\n')
    text='| Crate | RuKLEE | CULPA | Safe4U |\n|---|---|---|---|\n'
    for row in records:
        name=row['directory']; statuses={r['tool']:r for r in matrix if r['crate']==name}
        text+='| '+name+' | '+' | '.join(statuses[t]['status'] for t in ('ruklee','culpa','safe4u'))+' |\n'
    (ROOT/'dev/benchmark/compilation-matrix.md').write_text(text)
    for tool in ('ruklee','culpa','safe4u'):
        counts={status:sum(r['tool']==tool and r['status']==status for r in matrix) for status in ('passed','failed','timeout','not_run')}
        print(tool,counts)

if __name__=='__main__': main()
