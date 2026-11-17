import argparse
import csv,json,re
from collections import Counter,defaultdict
from pathlib import Path

def entity(name):
    m=re.search(r'\b(?:fn|trait)\s+((?:r#)?[A-Za-z_]\w*)',name)
    if m:
        return m[1]
    bare=re.match(r'^\s*((?:r#)?[A-Za-z_]\w*)\s*(?:<[^>]*>)?\s*\(',name)
    return bare[1] if bare else name

def location(path):
    f,_,line=path.rpartition(':')
    return f.removeprefix('tools/'),int(line)

def load_units(root,library):
    doc=json.loads((root/library/'step0.json').read_text())
    return [{'file':p.removeprefix('tools/'),'line':u['line_number_start'],**u} for p,us in doc['files'].items() for u in us]

def find_source(row,units):
    filename,line=location(row['path']);name=entity(row['name'])
    exact=[u for u in units if u['file']==filename and u['line']==line and u['name']==name]
    if exact:return exact[0],'exact'
    origin=[u for u in units if u['name']==name and u.get('source_origin') and location(u['source_origin'])[0]==filename]
    if len(origin)==1:return origin[0],'public_reexport'
    nearby=[u for u in units if u['file']==filename and u['name']==name and abs(u['line']-line)<=20]
    if nearby:
        nearby.sort(key=lambda u:abs(u['line']-line))
        if len(nearby)==1 or abs(nearby[0]['line']-line)<abs(nearby[1]['line']-line):return nearby[0],'line_span_changed'
    return None,'missing'

def main():
    parser=argparse.ArgumentParser(description="Compare rule catalogs by source API, preserving re-export identities. Counts are structural, not semantic scores.")
    parser.add_argument('--old',type=Path,default=Path('rules.csv'))
    parser.add_argument('--new',type=Path,default=Path('eval/rule/rules.csv'))
    parser.add_argument('--old-source',type=Path,default=Path('eval/rule'))
    parser.add_argument('--new-source',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    old=list(csv.DictReader(args.old.open()))
    new=list(csv.DictReader(args.new.open()))
    report={'baseline':str(args.old),'initial_output':str(args.new),'scope':'std/core; counts are structural, not a semantic score','libraries':{}}
    for lib in ('std','core'):
        a=[r for r in old if f'/library/{lib}/' in r['path']]
        b=[r for r in new if f'/library/{lib}/' in r['path']]
        public=load_units(args.old_source,lib)
        complete=load_units(args.new_source,lib)
        ag,bg=defaultdict(list),defaultdict(list)
        for row in a:ag[(row['path'],entity(row['name']))].append(row)
        for row in b:bg[(row['path'],entity(row['name']))].append(row)
        packets=[];classes=Counter()
        for (path,name),rows in ag.items():
            source,match=find_source(rows[0],complete)
            pub,_=find_source(rows[0],public)
            if pub is None and source is not None and match=='public_reexport':
                pub=next((u for u in public if u['name']==source['name'] and u['file']==source['file'] and u['line']==source['line']),None)
            predictions=bg.get((path,name),[])
            if not predictions and pub:
                predictions=bg.get((f"{pub['file']}:{pub['line']}",name),[])
            category=('excluded_by_public_scope' if not pub and source else
                      'no_source_in_full_scope' if not source else
                      'no_extracted_rules' if not predictions else 'has_predictions')
            classes[category]+=len(rows)
            packets.append({'path':path,'entity':name,'category':category,'source_match':match,
                'old':rows,'new':predictions,'source':source})
        result={'old_rules':len(a),'initial_rules':len(b),'old_entities':len(ag),'initial_entities':len(bg),
                'rows_by_structural_category':dict(classes),
                'full_scope_source_match_counts':dict(Counter(find_source(r,complete)[1] for r in a)),
                'packets':sorted(packets,key=lambda p:len(p['old'])-len(p['new']),reverse=True)}
        report['libraries'][lib]=result
        print(lib,json.dumps({k:v for k,v in result.items() if k!='packets'}))
        for p in packets:
            if p['category']=='no_source_in_full_scope':
                print('MISSING SOURCE',p['path'],p['entity'],[r['id'] for r in p['old']])
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')


if __name__ == '__main__':
    main()
