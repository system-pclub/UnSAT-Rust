#!/usr/bin/env python3
"""Build the requested three-row table from retained upstream and local evidence."""
import csv
from datetime import datetime, timezone
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEST = ROOT/'dev/benchmark'
NAMES = ('talc-5.0.3','redis-module-2.0.8','azul-css-0.0.8')
HEADER = ['Link','crates.io','Date','GitHub Stars','Recent Downloads','Total Downloads','LoC','Unsafe STD Call Sites','# of bugs','Version or Commit Hash']

def main():
    rows=[]
    evidence=[]
    for name in NAMES:
        package, version = name.rsplit('-',1)
        meta=json.loads((DEST/'metadata'/f'{package}.json').read_text())['response']['crate']
        github=json.loads((DEST/'metadata'/f'{package}-github.json').read_text())['response']
        crate=ROOT/'benchmark/original'/name
        lines=sum(bool(line.strip()) for p in (crate/'src').rglob('*.rs') for line in p.read_text().splitlines())
        report=json.loads((ROOT/'crates'/f'{name}.json').read_text())['report']
        sites={ (t['callsite']['path'],t['callsite']['line'],t['callsite']['col'])
               for t in report['targets'] if t['callee'].get('is_unsafe')
               and t['callee']['name'].startswith(('core::','std::','alloc::')) }
        bugs=set()
        for source in ('fast-findings-verdicts.tsv','fast-findings-ast-verdicts.tsv'):
            for row in csv.reader((ROOT/source).open(),delimiter='\t'):
                if len(row)>2 and name+'/' in row[0] and 'TP' in row[1:3]:
                    bugs.add(row[0].split(name+'/',1)[1])
        reviewed=json.loads((ROOT/'dev/findings/verdicts.json').read_text())
        for key,value in reviewed.items():
            if key.startswith(name+'::') and value['verdict']=='真阳性':
                # Merge calls on one source line with the text-review location convention.
                site=key.split('::')[1]
                for t in report['targets']:
                    cs=t['callsite']
                    if cs['id']==site:
                        bugs.add(f"{cs['path']}:{cs['line']}")
        # The review tables count faulty source lines; multiple unsafe calls on that line count once.
        bug_lines={b.rsplit(':',1)[0] if b.count(':')>1 else b for b in bugs}
        rows.append([github['html_url'],f'https://crates.io/crates/{package}',meta['updated_at'][:10],github['stargazers_count'],meta['recent_downloads'],meta['downloads'],lines,len(sites),len(bug_lines),version])
        evidence.append(dict(crate=name,reviewed_bug_locations=sorted(bug_lines),unsafe_std_sites=sorted(sites)))
    with (DEST/'crates.tsv').open('w') as f:
        writer=csv.writer(f,delimiter='\t'); writer.writerow(HEADER); writer.writerows(rows)
    text='# 补充 crate 统计\n\n采集时间：'+datetime.now(timezone.utc).isoformat()+'。\n\n'
    text+='| '+' | '.join(HEADER)+' |\n| '+' | '.join(['---']*len(HEADER))+' |\n'
    for row in rows: text+='| '+' | '.join(map(str,row))+' |\n'
    text+='''
Date 使用 crates.io 的 `updated_at`，表示该包最近更新时间；版本列固定为本次本地实验的版本，二者可能不同。GitHub Stars 来自对应仓库 API；Recent Downloads 为 crates.io `recent_downloads`（最近 90 天），Total Downloads 为该包所有版本累计下载量。原始 API 响应保存在 `metadata/`。

LoC 为原始归档 `src/**/*.rs` 中非空行数，包含注释，不含测试和示例目录。Unsafe STD Call Sites 是现有 MIRScan 报告中调用 `core/std/alloc` unsafe 函数的唯一 `(文件, 行, 列)` 数量；受当时编译特性和实例化覆盖范围限制，并非全特性静态词法统计。

`# of bugs` 为已有人工复核判为 TP 的源码行数，同一行多个 unsafe 调用合并；不是独立根因数，也不使用本次 Claude 的未复核输出。来源为根目录两个 `fast-findings*verdicts.tsv` 和 `dev/findings/verdicts.json`，具体位置见 `table-evidence.json`。
'''
    (DEST/'crates.md').write_text(text)
    (DEST/'table-evidence.json').write_text(json.dumps(evidence,indent=2)+'\n')
    print(text)

if __name__=='__main__': main()
