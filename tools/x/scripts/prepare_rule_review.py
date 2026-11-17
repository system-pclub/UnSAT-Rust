import argparse,csv,json,importlib.util
from pathlib import Path
import compare_catalogs as compare
p=argparse.ArgumentParser();p.add_argument('--library',required=True);p.add_argument('--run-root',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
lib=a.library;root=a.run_root
units=compare.load_units(root,lib)
legacy=[r for r in csv.DictReader(Path('rules.csv').open()) if f'/library/{lib}/' in r['path']]
results=json.loads((root/lib/'step1.json').read_text())
key=lambda path,line,name:f'{path}:{line}|{name}'
packets={key(r['path'],r['line'],r['name']):{'key':key(r['path'],r['line'],r['name']),
 'location':f"{r['path']}:{r['line']}",'name':r['name'],'declaration':r['declaration'],'predictions':r['rules'],
 'legacy':[],'legacy_source_matches':[],'status':r['status']} for r in results}
for row in legacy:
 unit,match=compare.find_source(row,units)
 k=key(unit['file'],unit['line'],unit['name']) if unit else row['path']+'|'+compare.entity(row['name'])
 packet=packets.setdefault(k,{'key':k,'location':row['path'],'name':compare.entity(row['name']),'predictions':[],'legacy':[],'legacy_source_matches':[],'status':'no_source'})
 packet['legacy'].append({'id':row['id'],'rule':row['rule'],'path':row['path']})
 packet['legacy_source_matches'].append(match)
manifest=json.loads((root/lib/'step1-input.json').read_text())
base_cache=manifest['requests_directory']
cache_sources=[(base_cache,None)]
repair_sources={}
for result in results:
 if result.get('repair_request_cache'):
  repair_sources.setdefault(result['repair_request_cache'],set()).add(key(result['path'],result['line'],result['name']))
cache_sources.extend(repair_sources.items())
for cache_name,allowed in cache_sources:
 cache=Path(cache_name.removeprefix('/unsat-rust/'))
 for path in sorted(cache.glob('round-*/input.json')):
  frozen=json.loads(path.read_text())
  for request in frozen['requests'].values():
   context=json.loads(request['messages'][-1]['content']);k=key(context['path'],context['line'],context['name'])
   if k in packets and (allowed is None or k in allowed):
    packets[k]['context']=context
    packets[k]['context_provenance']={'requests_directory':cache_name,'mode':frozen['mode'],'model':request['model']}
out=sorted(packets.values(),key=lambda r:(r['location'],r['name']))
a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(out,indent=2,ensure_ascii=False)+'\n')
print(lib,'APIs',len(out),'legacy rows',sum(len(r['legacy']) for r in out),'predictions',sum(len(r['predictions']) for r in out))
