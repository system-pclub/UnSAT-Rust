"""Concrete EBNF and executable parser generated from source type transitions.

Field and index terminals are produced only from graph edges. The same production
tree renders both EBNF and Lark, so parsing tests exercise the emitted language.
"""

from __future__ import annotations

from collections import Counter, defaultdict, deque
import json
import re

from dsltypes import TypeCatalog, display, named
import dslmemory


def literal(text):
    return ("literal", text)


def reference(name):
    return ("reference", name)


def sequence(*items):
    return ("sequence", items)


def alternative(*items):
    return ("alternative", items)


def optional(item):
    return ("optional", item)


def repeated(item):
    return ("repeated", item)


def render(expr, lark=False):
    kind, value = expr
    if kind == "literal":
        return json.dumps(value)
    if kind == "reference":
        return value
    if kind == "sequence":
        return (" " if lark else ", ").join(render(x, lark) for x in value)
    if kind == "alternative":
        return "(" + " | ".join(render(x, lark) for x in value) + ")"
    if kind == "optional":
        return "(" + render(value, True) + ")?" if lark else "[ " + render(value) + " ]"
    if kind == "repeated":
        return "(" + render(value, True) + ")*" if lark else "{ " + render(value) + " }"
    raise ValueError(kind)


def snake(text):
    text = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", text)
    text = re.sub(r"[^a-zA-Z0-9]+", "_", text).strip("_").lower()
    return text or "opaque"


class ConcreteGrammar:
    def __init__(self, catalog: TypeCatalog, variables=None, family=False, max_depth=None, entry_types=(), quantifier_slots=False, variable_comments=""):
        self.quantifier_slots = quantifier_slots
        self.variable_comments = variable_comments
        self.catalog = catalog
        self.family = family
        self.rules = {}
        self.comments = {}
        self.rules_section = {}
        self._parser = None
        self._graph(variables, max_depth, entry_types)
        self._syntax()
        self._prune()

    def add(self, name, expr, section="template"):
        self.rules[name] = expr
        self.rules_section[name] = section

    def _prune(self):
        def references(expr):
            kind, value = expr
            if kind == "reference":
                yield value
            elif kind in {"sequence", "alternative"}:
                for item in value:
                    yield from references(item)
            elif kind in {"optional", "repeated"}:
                yield from references(value)
        reachable, pending = set(), ["formula"]
        while pending:
            name = pending.pop()
            if name in reachable:
                continue
            reachable.add(name)
            if name in self.rules:
                pending.extend(references(self.rules[name]))
            elif name not in {"identifier", "integer", "string"}:
                raise ValueError(f"undefined generated nonterminal: {name}")
        self.rules = {n: expr for n, expr in self.rules.items() if n in reachable}

    def intern(self, value):
        if self.family:
            def erase(ty):
                ty = dict(ty)
                if ty["kind"] == "named":
                    name = self.catalog.canonical(ty["name"])
                    parameters = self.catalog.definitions.get(name, {}).get("parameters", [])
                    return named(name, [{"kind": "parameter", "name": p["name"]} for p in parameters])
                if ty["kind"] == "memory" and "of" in ty:
                    ty["of"] = erase(ty["of"])
                if ty["kind"] in {"sequence", "pointer"}:
                    ty["element"] = erase(ty["element"])
                if ty["kind"] == "tuple":
                    ty["elements"] = [erase(x) for x in ty["elements"]]
                return ty
            value = erase(value)
        return self.catalog.intern(value)

    def _graph(self, variables, max_depth, entry_types):
        pending = deque()
        entries = defaultdict(list)
        if self.family and variables is None:
            for name in sorted(self.catalog.definitions):
                parent = name.rsplit("::", 1)[0]
                if self.catalog.definitions.get(parent, {}).get("shape", {}).get("kind") == "enum":
                    continue
                node = self.intern(named(name))
                entries[node].append(reference("identifier"))
                pending.append((node, 0))
            for name in ["usize", "u8", "bool", "str"]:
                node = self.intern(named(name))
                entries[node].append(reference("identifier"))
                pending.append((node, 0))
            for ty in entry_types:
                node = self.intern(ty)
                entries[node].append(reference("identifier"))
                pending.append((node, 0))
        else:
            for name, node in sorted((variables or {}).items()):
                node = self.intern(self.catalog._types[node]) if self.family else node
                entries[node].append(literal(name))
                pending.append((node, 0))
        edges, depths = [], {}
        while pending:
            node, depth = pending.popleft()
            if node in depths and depths[node] <= depth:
                continue
            depths[node] = depth
            # Family mode erases generic arguments before constructing children,
            # so recursive generic declarations remain a finite graph.
            if self.family:
                self._expand_family(node)
            else:
                self.catalog.expand(node)
            if max_depth is not None and depth >= max_depth:
                continue
            sort = self.catalog.sorts[node]
            for edge in sort["edges"]:
                if edge.get("guard") in sort.get("unit_variants", []):
                    continue
                edge_key = (node, edge["selector"], edge["target"])
                if edge_key not in edges:
                    edges.append(edge_key)
                pending.append((edge["target"], depth + bool(edge["selector"])))
                if self.quantifier_slots and edge["selector"] == "[index]":
                    entries[edge["target"]].append(reference("identifier"))

        # A type alias denotes the same expression, not a runtime field access.
        parent = {n: n for n in depths}
        for source, _, target in edges:
            parent.setdefault(source, source)
            parent.setdefault(target, target)

        def find(node):
            while parent[node] != node:
                parent[node] = parent[parent[node]]
                node = parent[node]
            return node

        for source, selector, target in edges:
            if selector == "":
                parent[find(target)] = find(source)
        grouped = defaultdict(list)
        for node in parent:
            grouped[find(node)].append(node)
        productive = {find(n) for n in entries}
        active = True
        while active:
            before = len(productive)
            for source, selector, target in edges:
                if find(source) in productive:
                    productive.add(find(target))
            active = len(productive) != before
        groups = sorted(productive, key=lambda g: min(self.catalog.sorts[n]["rust_type"] for n in grouped[g]))
        bases = {}
        for group in groups:
            nodes = grouped[group]
            declared = [n for n in nodes if self.catalog._types[n]["kind"] == "named" and self.catalog._types[n]["name"] in self.catalog.definitions]
            chosen = min(declared or nodes, key=lambda n: self.catalog.sorts[n]["rust_type"])
            ty = self.catalog._types[chosen]
            if ty["kind"] == "memory":
                base = "memory_" + ty["view"]
            elif ty["kind"] == "memory_scalar":
                base = "memory_" + ty["name"]
            elif ty["kind"] == "pointer":
                base = "pointer"
            elif ty["kind"] == "named":
                base = snake(ty["name"].rsplit("::", 1)[-1])
            elif ty["kind"] == "sequence":
                base = "sequence"
            elif ty["kind"] == "tuple":
                base = "tuple"
            else:
                base = snake(ty.get("name", "opaque"))
            bases[group] = base
        counts, used = Counter(bases.values()), Counter()
        names = {}
        for group in groups:
            base = bases[group]
            used[base] += 1
            names[group] = base + (f"_{used[base]}" if counts[base] > 1 else "") + "_term"
        incoming = defaultdict(list)
        for node, alternatives in entries.items():
            incoming[find(node)].extend(alternatives)
        for source, selector, target in edges:
            source, target = find(source), find(target)
            if not selector or source not in productive:
                continue
            if selector.startswith("."):
                suffix = (literal("."), literal(selector[1:]))
            elif selector == "[index]":
                suffix = (literal("["), reference("index"), literal("]"))
            elif re.fullmatch(r"\[\d+\]", selector):
                suffix = (literal("["), literal(selector[1:-1]), literal("]"))
            else:
                raise ValueError(f"unsupported generated selector: {selector}")
            incoming[target].append(sequence(reference(names[source]), *suffix))

        self.access_names = []
        self.integer_names, self.collection_names, enum_tests = [], [], []
        for group in groups:
            name = names[group]
            self.access_names.append(name)
            members = grouped[group]
            types = sorted({self.catalog.sorts[n]["rust_type"] for n in members})
            self.comments[name] = "; ".join(types)
            section = "memory" if any(self.catalog._types[n]["kind"] in {"memory", "memory_scalar"} for n in members) else "library"
            choices = incoming[group] + [sequence(literal("("), reference(name), literal(")"))]
            self.add(name, alternative(*dict.fromkeys(choices)), section)
            for node in members:
                sort = self.catalog.sorts[node]
                ty = self.catalog._types[node]
                if sort["shape"] == "scalar" and (sort.get("scalar_kind") == "integer" or re.fullmatch(r"[ui](?:8|16|32|64|128)|usize|isize", sort["rust_type"])):
                    self.integer_names.append(name)
                if ty["kind"] == "sequence" or sort["rust_type"] in {"str", "String"}:
                    self.collection_names.append(name)
                variants = sort.get("variants", [])
                if variants:
                    tag_name = name.removesuffix("_term") + "_variant"
                    test_name = name.removesuffix("_term") + "_test"
                    tags = [literal(quote + variant + quote) for variant in variants for quote in ['"', "'"]]
                    self.add(tag_name, alternative(*tags), section)
                    self.add(test_name, sequence(literal("is_variant"), literal("("), reference(name), literal(","), reference(tag_name), literal(")")), section)
                    enum_tests.append(reference(test_name))
                    break
        if self.access_names:
            self.add("typed_term", alternative(*(reference(n) for n in self.access_names)))
        if enum_tests:
            self.add("variant_test", alternative(*enum_tests))

    def _expand_family(self, node):
        if node in self.catalog._expanded:
            return
        self.catalog._expanded.add(node)
        value = self.catalog._types[node]
        sort = self.catalog.sorts[node]
        if dslmemory.expand(self.catalog, value, sort, self.intern):
            return
        edges = []
        if value["kind"] == "named" and value["name"] in self.catalog.definitions:
            definition = self.catalog.definitions[value["name"]]
            sort["shape"] = definition["shape"]["kind"]
            if sort["shape"] == "enum":
                sort["variants"] = [v["name"] for v in definition["shape"]["variants"]]
                sort["unit_variants"] = [v["name"] for v in definition["shape"]["variants"] if not v["fields"]]
            edges = [{"selector": m["syntax"], "target": self.intern(m["result"]), "guard": m["guard"]} for m in self.catalog.selectors.get(value["name"], [])]
        elif value["kind"] == "sequence":
            sort["shape"] = "sequence"
            edges = [{"selector": "[index]", "target": self.intern(value["element"])}]
        elif value["kind"] == "tuple":
            sort["shape"] = "tuple"
            edges = [{"selector": f"[{i}]", "target": self.intern(t)} for i, t in enumerate(value["elements"])]
        elif value["kind"] == "named" and re.fullmatch(r"bool|char|str|[ui](?:8|16|32|64|128)|usize|isize", value["name"]):
            sort["shape"] = "scalar"
        sort["edges"] = edges
        dslmemory.attach(self.catalog, value, sort, self.intern)

    def _syntax(self):
        # Ordinary EBNF, with no open-ended attribute or method production.
        r, t, s, a, o, many = reference, literal, sequence, alternative, optional, repeated
        self.add("formula", r("expression"))
        self.add("expression", s(r("disjunction"), o(s(t("if"), r("disjunction"), t("else"), r("expression")))))
        self.add("disjunction", s(r("conjunction"), many(s(t("or"), r("conjunction")))))
        self.add("conjunction", s(r("negation"), many(s(t("and"), r("negation")))))
        self.add("negation", a(s(t("not"), r("negation")), r("comparison")))
        self.add("comparison", s(r("arithmetic"), many(s(r("compare"), r("arithmetic")))))
        self.add("compare", a(*(t(op) for op in ["==", "!=", "<", "<=", ">", ">=", "in"]), s(t("not"), t("in"))))
        self.add("arithmetic", s(r("product"), many(s(a(t("+"), t("-")), r("product")))))
        self.add("product", s(r("unary"), many(s(a(t("*"), t("//"), t("%")), r("unary")))))
        self.add("unary", s(o(a(t("+"), t("-"))), r("primary")))
        primary = [r("integer"), r("string"), t("True"), t("False"), t("None"), s(t("("), r("expression"), t(")")), r("collection"), r("builtin")]
        if self.access_names:
            primary.insert(0, r("typed_term"))
        self.add("primary", a(*primary))
        self.add("collection", a(s(t("["), o(s(r("expression"), many(s(t(","), r("expression"))))), t("]")), s(t("["), r("expression"), r("comprehension"), t("]"))))
        self.add("comprehension", s(t("for"), r("identifier"), t("in"), r("disjunction"), many(s(t("if"), r("disjunction"))), o(r("comprehension"))))
        builtins = [s(a(t("all"), t("any")), t("("), r("expression"), t(")")), s(t("len"), t("("), r("expression"), t(")"))]
        if "variant_test" in self.rules:
            builtins.append(r("variant_test"))
        self.add("builtin", a(*builtins))
        self.add("index", s(r("index_product"), many(s(a(t("+"), t("-")), r("index_product")))))
        self.add("index_product", s(r("index_unary"), many(s(a(t("*"), t("//"), t("%")), r("index_unary")))))
        self.add("index_unary", s(o(a(t("+"), t("-"))), r("index_primary")))
        index_primary = [r("integer"), s(t("("), r("index"), t(")")), s(t("len"), t("("), r("expression"), t(")"))]
        index_primary.extend(r(n) for n in dict.fromkeys(self.integer_names))
        self.add("index_primary", a(*index_primary))

    def ebnf(self, section=None):
        header = "(* Generated concrete EBNF. Every field, tuple position and enum tag is a literal. *)\n"
        header += self.variable_comments
        if self.quantifier_slots:
            header += "(* Bare identifier entries are comprehension locals only; scope and element types are checked separately. *)\n"
        lines = [header]
        for name, expression in self.rules.items():
            if section is not None and self.rules_section[name] != section:
                continue
            if name in self.comments:
                lines.append("(* " + self.comments[name].replace("*)", "* )") + " *)")
            lines.append(name + " = " + render(expression) + " ;")
        if section in {None, "template"}:
            lines.extend([
                "identifier = ? ASCII identifier [A-Za-z_][A-Za-z0-9_]* ? ;",
                "integer = ? nonnegative decimal integer literal ? ;",
                "string = ? single- or double-quoted escaped string literal ? ;",
            ])
        return "\n".join(lines) + "\n"

    def lark(self):
        lines = [name + ": " + render(expr, True) for name, expr in self.rules.items()]
        lines.extend([
            "identifier: IDENTIFIER", "integer: INTEGER", "string: STRING",
            r"IDENTIFIER: /[A-Za-z_][A-Za-z0-9_]*/", r"INTEGER: /[0-9]+/",
            r'''STRING: /"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'/''',
            "%import common.WS", "%ignore WS",
        ])
        return "\n".join(lines) + "\n"

    def parse(self, text):
        from lark import Lark
        if self._parser is None:
            self._parser = Lark(self.lark(), start="formula", parser="earley", lexer="dynamic", ambiguity="resolve")
        return self._parser.parse(text)
