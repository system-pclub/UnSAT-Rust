"""Modeled current-state memory domain, composed with source-derived type edges."""
from __future__ import annotations

import json
from pathlib import Path

from dsltypes import substitute

PATH = Path(__file__).with_suffix('.json')
SCHEMA_BYTES = PATH.read_bytes()
SCHEMA = json.loads(SCHEMA_BYTES)
ATTRIBUTE = '__dsl_mem'


def view(kind, typ=None):
    result = {'kind': 'memory', 'view': kind}
    if typ is not None:
        result['of'] = typ
    return result


def scalar(name):
    return {'kind': 'memory_scalar', 'name': name}


def runtime_type(typ):
    if typ['kind'] in {'memory', 'memory_scalar', 'const'}:
        return False
    if typ['kind'] == 'opaque' and typ.get('rust', '').startswith(('type parameter ', 'lifetime parameter ')):
        return False
    return True


def underlying(catalog, typ):
    visited = set()
    while typ['kind'] == 'named':
        typ = catalog.normalize(typ)
        key = json.dumps(typ, sort_keys=True)
        if key in visited or len(visited) >= 64:
            raise ValueError('cyclic/unbounded alias in memory view')
        visited.add(key)
        definition = catalog.definitions.get(typ['name'])
        if not definition or definition['shape']['kind'] != 'alias':
            break
        bindings = {p['name']: a for p, a in zip(definition['parameters'], typ['args'])}
        typ = substitute(definition['shape']['target'], bindings)
    return typ


def pointee(catalog, typ):
    typ = underlying(catalog, typ)
    if typ['kind'] == 'pointer':
        return typ['element']
    if typ['kind'] == 'named' and typ['name'] in SCHEMA['pointer_wrappers'] and typ['args']:
        return typ['args'][0]
    return None


def expand(catalog, typ, sort, intern):
    """Return True for modeled nodes; source nodes are handled by the catalog."""
    if typ['kind'] == 'memory_scalar':
        sort.update(shape='scalar', scalar_kind=typ['name'], edges=[])
        return True
    if typ['kind'] != 'memory':
        return False
    fields = SCHEMA['allocation_fields'] if typ['view'] == 'allocation' else SCHEMA['fields']
    edges = []
    for name, spec in fields.items():
        result = view('allocation') if spec['type'] == 'allocation' else scalar(spec['type'])
        edges.append({'selector': '.' + name, 'target': intern(result), 'guard': None, 'domain': 'memory'})
    if typ['view'] == 'storage':
        target = pointee(catalog, typ['of'])
        if target is not None:
            edges.append({'selector': '.pointee', 'target': intern(view('region', target)), 'guard': None, 'domain': 'memory'})
    sort.update(shape='record', edges=edges, domain='memory')
    return True


def attach(catalog, typ, sort, intern):
    if runtime_type(typ):
        target = view('storage', underlying(catalog, typ))
        sort['edges'].append({'selector': '.$mem', 'target': intern(target), 'guard': None, 'domain': 'memory'})
