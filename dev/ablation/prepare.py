"""Validate the existing ptr::read access rule for the ablation source fixture."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/x/src'))
from dsltrans import Model
import dslv2

source = json.loads((ROOT / 'dev/typerules/rules.json').read_text())['rules']['rule-321']
rule = dict(source)
rule['dsl_v2'] = dslv2.lower(Model(ROOT / 'tools/x/generated/dsl/schema.json', rule['api']), rule['dsl'])
here = Path(__file__).parent
(here / 'rules.json').write_text(json.dumps({'rules': {'rule-321': rule}}, indent=2) + '\n')
(here / 'studied_rules').write_text('rule-321\n')
print('Validated ptr::read readable DSL for the source ablation fixture')
