"""Source-backed documentation references for atomic safety extraction.

References are context, never an automatic license to inherit another API's
requirements. Ambiguous targets stay unresolved rather than guessing a contract.
"""
from __future__ import annotations

from collections import defaultdict
import re
from pathlib import Path
import json


def module(path):
    parts = path.replace('\\', '/').split('/')
    try:
        i = parts.index('library')
        tail = parts[i+3:]
        tail[-1] = Path(tail[-1]).stem
        if tail[-1] in {'mod', 'lib'}:
            tail.pop()
        return '::'.join([parts[i+1], *tail])
    except (ValueError, IndexError):
        return ''


def compact_docs(text):
    # Examples are not normative obligations. Preserve all non-example sections,
    # including safety prose without a heading and named definitions.
    lines, out, fenced, skip_level = text.splitlines(), [], False, None
    for line in lines:
        if line.strip().startswith('```'):
            fenced = not fenced
            continue
        if fenced:
            continue
        heading = re.match(r'\s*(#+)\s+(.+)', line)
        if heading:
            level, title = len(heading[1]), heading[2].strip().lower()
            if title.startswith('example'):
                skip_level = level
                continue
            if skip_level is not None and level <= skip_level:
                skip_level = None
        if skip_level is None:
            out.append(line.strip())
    return '\n'.join(out).strip()


class DocIndex:
    def __init__(self, documents):
        self.units, self.by_name, self.module_docs = {}, defaultdict(list), {}
        for document in documents:
            for path, text in document.get('module_documentation', {}).items():
                self.module_docs[path.removeprefix('./').removeprefix('tools/')] = text
            for raw_path, units in document.get('files', {}).items():
                path = raw_path.replace('\\', '/').removeprefix('./').removeprefix('tools/')
                for unit in units:
                    key = (path, int(unit['line_number_start']), unit['name'])
                    item = {'path': path, 'line': key[1], 'name': unit['name'],
                            'declaration': unit['declaration_text'],
                            'parent': unit.get('parent_struct_or_trait'),
                            'documentation': unit['comment'],
                            'enclosing_declaration': unit.get('enclosing_declaration'),
                            'source_body': unit.get('source_body')}
                    if key not in self.units:
                        self.units[key] = item
                        self.by_name[item['name']].append(item)

    def resolve(self, link, current):
        link = link.strip('` ').rstrip('()')
        if link.startswith(('http:', 'https:', '#')) or not link:
            return None
        link = re.sub(r'<[^<>]*>', '', link).replace('!', '')
        tokens = [t for t in link.split('::') if t]
        name = tokens[-1]
        candidates = self.by_name.get(name, [])
        origin = module(current['path'])
        if tokens[0] == 'crate':
            tokens[0] = origin.split('::')[0]
        elif tokens[0] in {'self', 'Self'}:
            owner = re.sub(r'<.*', '', current.get('parent') or '').strip()
            tokens = ([owner] if tokens[0] == 'Self' else origin.split('::')) + tokens[1:]
        ranked = []
        for item in candidates:
            scope = module(item['path'])
            parent = re.sub(r'<.*', '', item.get('parent') or '').strip()
            full = '::'.join([scope, *([parent] if parent else []), name])
            if len(tokens) > 1 and tokens[-2][:1].islower() and parent and tokens[-2] != 'pointer':
                continue
            if tokens[0] in {'core', 'alloc', 'std'} and scope.split('::')[0] != tokens[0]:
                continue
            # Re-exported types can hide their implementation module, but an
            # explicit owner must still match the source owner.
            if len(tokens) > 1 and tokens[-2][:1].isupper() and tokens[-2] != parent:
                continue
            score = 100 if full == '::'.join(tokens) else 0
            if full == '::'.join(tokens) or full.endswith('::' + '::'.join(tokens)):
                score += 30
            if len(tokens) > 1 and scope.startswith('::'.join(tokens[:-1]) + '::'):
                score += 60
            if len(tokens) > 1 and tokens[0] not in {'core', 'alloc', 'std'} and ('::' + '::'.join(tokens[:-1]) + '::') in ('::' + scope + '::'):
                score += 60
            score += 10 * (item['path'] == current['path'])
            score += 5 * bool(parent and parent == re.sub(r'<.*', '', current.get('parent') or '').strip())
            score += len(set(scope.split('::')) & set(tokens[:-1]))
            ranked.append((score, item))
        if not ranked:
            return None
        best = max(score for score, _ in ranked)
        winners = [item for score, item in ranked if score == best]
        return winners[0] if len(winners) == 1 else None

    def context(self, item):
        current = self.units.get((item['path'], item['line'], item['name']), item)
        root_key = (item['path'], item['line'], item['name'])
        seen = {root_key}
        references, unresolved = [], []
        queue = [(current, 0)]
        while queue:
            source, depth = queue.pop(0)
            if depth >= 3:
                continue
            doc = compact_docs(source['documentation'])
            definitions = {label.strip('` ').removesuffix('()'): target for label, target in
                           re.findall(r'(?m)^\s*\[([^\]]+)\]:\s*(\S+)', source['documentation'])}
            links = []
            markdown = list(re.finditer(r'\[([^\]]+)\](?:\(([^)]+)\)|\[([^\]]*)\])?', doc))
            mentions = [(match, *match.groups()) for match in markdown]
            for match in re.finditer(r'`((?:[A-Za-z_]\w*::)*[A-Za-z_]\w*(?:\(\))?)`', doc):
                if not any(parent.start() <= match.start() and match.end() <= parent.end() for parent in markdown):
                    mentions.append((match, match[1], None, None))
            for match, label, inline, reference in sorted(mentions, key=lambda entry: entry[0].start()):
                if re.match(r':', doc[match.end():]):
                    continue
                boundary = doc.rfind('\n\n', 0, match.start())
                start = 0 if boundary < 0 else boundary + 2
                end = doc.find('\n\n', match.end())
                paragraph = doc[max(0, start):len(doc) if end < 0 else end]
                # Only explicit delegation/equivalence supplies inherited
                # contracts. Mere method mentions in trait definitions do not.
                headings = list(re.finditer(r'(?m)^#+\s+(.+)$', doc[:match.start()]))
                in_safety = bool(headings and headings[-1][1].strip().lower() == 'safety')
                delegates = re.search(r"(?is)(?:see|refer to).*?(?:safety|requirements?|documentation|information|details)|(?:safety conditions|requirements).*?apply|\b(?:same|equivalent|like|deprecated|instead)\b", paragraph)
                if not delegates and not re.match(r'(?i)^See\s+', paragraph.strip()) and not (in_safety and re.search(r'(?i)\b(?:see|refer to)\b', paragraph)):
                    continue
                key = (reference or label).strip('` ').removesuffix('()')
                target = (inline or definitions.get(key) or label).strip('` ')
                if target.startswith('#method.'):
                    target = 'Self::' + target.removeprefix('#method.')
                if re.fullmatch(r'(?:[A-Za-z_]\w*::)*[A-Za-z_]\w*(?:\(\))?', target):
                    links.append(target)
            for target in dict.fromkeys(links):
                unit = self.resolve(target, source)
                if unit is None:
                    unresolved.append(target)
                    continue
                key = (unit['path'], unit['line'], unit['name'])
                if key in seen:
                    continue
                seen.add(key)
                references.append({'reference': target, 'path': f"{unit['path']}:{unit['line']}",
                    'declaration': unit['declaration'], 'parent': unit['parent'],
                    'documentation': compact_docs(unit['documentation'])})
                queue.append((unit, depth+1))
        result = {**item, 'documentation': compact_docs(item['documentation'])}
        owner = re.sub(r'<.*', '', current.get('parent') or '').strip().split('::')[-1]
        anchors = set(re.findall(r'#([A-Za-z][A-Za-z0-9_-]*)', current['documentation']))
        definitions = []
        parents = [unit for unit in self.by_name.get(owner, [])
                   if unit['path'] == current['path'] and unit['declaration'].startswith('trait ')]
        for parent in parents:
            text = compact_docs(parent['documentation'])
            headings = list(re.finditer(r'(?m)^(#{2,})\s+(.+)$', text))
            for i, heading in enumerate(headings):
                slug = re.sub(r'[^a-z0-9 -]', '', heading[2].lower()).replace(' ', '-')
                if slug in anchors:
                    end = next((h.start() for h in headings[i+1:] if len(h[1]) <= len(heading[1])), len(text))
                    # A level-one heading (especially Safety) also ends a definition.
                    top = re.search(r'(?m)^#\s+', text[heading.end():end])
                    if top:
                        end = heading.end() + top.start()
                    definitions.append({'source': f"{parent['path']}:{parent['line']}#{slug}",
                                        'text': text[heading.end():end].strip()})
        # Local rustdoc anchors can refer to module-level layout contracts,
        # e.g. Box::from_raw -> its own module's Memory layout section.
        text = compact_docs(self.module_docs.get(current['path'], ''))
        headings = list(re.finditer(r'(?m)^(#+)\s+(.+)$', text))
        for i, heading in enumerate(headings):
            slug = re.sub(r'[^a-z0-9 -]', '', heading[2].lower()).replace(' ', '-')
            if slug in anchors:
                end = next((h.start() for h in headings[i+1:] if len(h[1]) <= len(heading[1])), len(text))
                definitions.append({'source': current['path'] + '#' + slug,
                                    'text': text[heading.end():end].strip()})
        if definitions:
            result['referenced_definitions'] = definitions
        if current.get('enclosing_declaration'):
            result['enclosing_declaration'] = current['enclosing_declaration']
        if references:
            result['referenced_docs'] = references
            # A short forwarding body binds e.g. callee src/dst to wrapper self.
            body = current.get('source_body') or ''
            if len(body) <= 1600:
                result['forwarding_body'] = body
        if unresolved:
            result['unresolved_doc_links'] = list(dict.fromkeys(unresolved))
        return result


def index_for(output):
    paths = sorted(output.parent.glob('*/step0.json'))
    return DocIndex([json.loads(path.read_text()) for path in paths])
