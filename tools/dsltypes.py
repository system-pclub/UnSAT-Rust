"""Generic type-catalog operations shared by DSL prompt slicing and checking."""

from __future__ import annotations

import copy
import json
import re


def named(name, args=()):
    return {"kind": "named", "name": name, "args": list(args)}


def substitute(value, bindings):
    if isinstance(value, dict):
        if value.get("kind") == "parameter":
            return copy.deepcopy(bindings.get(value["name"], value))
        return {key: substitute(item, bindings) for key, item in value.items()}
    if isinstance(value, list):
        return [substitute(item, bindings) for item in value]
    return value


def display(value):
    kind = value["kind"]
    if kind == "named":
        return value["name"] + ("<" + ", ".join(map(display, value["args"])) + ">" if value["args"] else "")
    if kind == "parameter":
        return value["name"]
    if kind == "tuple":
        return "(" + ", ".join(map(display, value["elements"])) + ")"
    if kind == "sequence":
        return "[" + display(value["element"]) + ("; " + value["length"] if value.get("length") is not None else "") + "]"
    return value.get("rust", value.get("expression", "opaque"))


class TypeCatalog:
    def __init__(self, schema):
        if schema.get("version") != 2:
            raise ValueError("regenerate dslgen output: an API-independent version-2 catalog is required")
        self.schema = schema
        self.definitions = schema["definitions"]
        self.selectors = schema["selectors"]
        self.aliases = schema["aliases"]
        self.short_names = {}
        for name in self.definitions:
            parent = name.rsplit("::", 1)[0]
            if self.definitions.get(parent, {}).get("shape", {}).get("kind") == "enum":
                continue  # A synthetic variant payload is not an independent Rust type.
            self.short_names.setdefault(name.rsplit("::", 1)[-1], []).append(name)
        self.sorts = {}
        self._ids = {}
        self._types = {}
        self._expanded = set()

    def canonical(self, name):
        visited = set()
        while name not in self.definitions and name in self.aliases:
            if name in visited:
                raise ValueError(f"cyclic type alias {name}")
            visited.add(name)
            name = self.aliases[name]
        if name in self.schema.get("alternatives", {}):
            raise ValueError(f"type {name} has unresolved cfg alternatives")
        if name not in self.definitions and "::" not in name:
            candidates = self.short_names.get(name, [])
            if len(candidates) == 1:
                return candidates[0]
        return name

    def parse(self, text):
        # Type syntax only: no API declaration, body, or parameter-list parsing.
        tokens = re.findall(r"::|'[A-Za-z_]\w*|[A-Za-z_]\w*|\d+|[<>,;&*()\[\]!?+{}-]", text)
        if "".join(tokens) != re.sub(r"\s+", "", text):
            raise ValueError(f"unsupported Rust type syntax: {text}")
        position = 0

        def take(wanted=None):
            nonlocal position
            if position >= len(tokens) or (wanted is not None and tokens[position] != wanted):
                raise ValueError(f"invalid Rust type: {text}")
            value = tokens[position]
            position += 1
            return value

        def parse_type():
            if position >= len(tokens):
                raise ValueError("missing type")
            if tokens[position] == "&":
                take()
                if position < len(tokens) and tokens[position].startswith("'"):
                    take()
                if position < len(tokens) and tokens[position] == "mut":
                    take()
                return parse_type()
            if tokens[position] == "*":
                begin = position
                take()
                if position < len(tokens) and tokens[position] in {"const", "mut"}:
                    take()
                parse_type()
                return {"kind": "opaque", "rust": " ".join(tokens[begin:position])}
            if tokens[position] == "[":
                take("[")
                element = parse_type()
                length = None
                if tokens[position] == ";":
                    take(";")
                    start = position
                    while position < len(tokens) and tokens[position] != "]":
                        take()
                    length = " ".join(tokens[start:position])
                take("]")
                return {"kind": "sequence", "element": element, "length": length}
            if tokens[position] == "(":
                take("(")
                elements = []
                while tokens[position] != ")":
                    elements.append(parse_type())
                    if tokens[position] != ",":
                        break
                    take(",")
                take(")")
                return {"kind": "tuple", "elements": elements}
            first = take()
            if not re.fullmatch(r"[A-Za-z_]\w*", first):
                raise ValueError(f"invalid type name: {first}")
            parts = [first]
            while position < len(tokens) and tokens[position] == "::":
                take("::")
                parts.append(take())
            args = []
            if position < len(tokens) and tokens[position] == "<":
                take("<")
                while tokens[position] != ">":
                    if tokens[position].startswith("'"):
                        take()
                    elif tokens[position].isdigit():
                        args.append({"kind": "const", "expression": take()})
                    else:
                        args.append(parse_type())
                    if tokens[position] != ",":
                        break
                    take(",")
                take(">")
            name = self.canonical("::".join(parts))
            if name not in self.definitions and "::" not in name and (name == "_" or len(name) == 1 and name.isupper()):
                return {"kind": "parameter", "name": name}
            return named(name, args)

        try:
            result = parse_type()
            if position != len(tokens):
                raise ValueError(f"trailing tokens in Rust type: {text}")
            return result
        except IndexError as error:
            raise ValueError(f"incomplete Rust type: {text}") from error

    def normalize(self, value):
        value = copy.deepcopy(value)
        if value["kind"] != "named":
            return value
        value["name"] = self.canonical(value["name"])
        definition = self.definitions.get(value["name"])
        if definition:
            params = definition["parameters"]
            if len(value["args"]) > len(params):
                raise ValueError(f"too many generic arguments for {value['name']}")
            bindings = {}
            for i, param in enumerate(params):
                if len(value["args"]) == i:
                    default = param["default"]
                    value["args"].append(substitute(default, bindings) if default is not None else {"kind": "parameter", "name": param["name"]})
                bindings[param["name"]] = value["args"][i]
        return value

    def language_types(self, text):
        """Read array/slice/tuple type shapes in context, without binding parameters.

        These are Rust language constructors, so they need no struct declaration.
        Function parameter lists fail the type parser and are simply not shapes.
        """
        stack, found = [], {}
        for i, char in enumerate(text):
            if char in "([":
                stack.append((char, i))
            elif char in ")]" and stack:
                opening, start = stack.pop()
                if (opening, char) not in {("(", ")"), ("[", "]")}:
                    continue
                candidate = text[start:i+1]
                try:
                    value = self.parse(candidate)
                except ValueError:
                    continue
                if value["kind"] == "sequence" or value["kind"] == "tuple" and ("," in candidate or candidate == "()"):
                    found[json.dumps(value, sort_keys=True)] = value
        return [found[k] for k in sorted(found)]

    def intern(self, value):
        value = self.normalize(value)
        key = json.dumps(value, sort_keys=True)
        if key in self._ids:
            return self._ids[key]
        if len(self.sorts) >= 10000:
            raise ValueError("expression type expansion exceeded 10000 sorts")
        sort_id = len(self.sorts)
        self._ids[key] = sort_id
        self._types[sort_id] = value
        self.sorts[sort_id] = {"id": sort_id, "rust_type": display(value), "shape": "opaque", "edges": [], "opaque_reason": None}
        return sort_id

    def expand(self, sort_id):
        if sort_id in self._expanded:
            return
        self._expanded.add(sort_id)
        value = self._types[sort_id]
        sort = self.sorts[sort_id]
        kind = value["kind"]
        edges = []
        if kind == "named" and value["name"] in self.definitions:
            definition = self.definitions[value["name"]]
            sort["shape"] = definition["shape"]["kind"]
            if sort["shape"] == "enum":
                sort["variants"] = [v["name"] for v in definition["shape"]["variants"]]
                sort["unit_variants"] = [v["name"] for v in definition["shape"]["variants"] if not v["fields"]]
            bindings = {p["name"]: arg for p, arg in zip(definition["parameters"], value["args"])}
            for member in self.selectors.get(value["name"], []):
                edges.append({"selector": member["syntax"], "target": self.intern(substitute(member["result"], bindings)), "guard": member["guard"], "source": member["source"]})
        elif kind == "sequence":
            sort["shape"] = "sequence"
            edges.append({"selector": "[index]", "target": self.intern(value["element"]), "guard": None})
        elif kind == "tuple":
            sort["shape"] = "tuple"
            edges.extend({"selector": f"[{i}]", "target": self.intern(t), "guard": None} for i, t in enumerate(value["elements"]))
        elif kind == "named" and re.fullmatch(r"bool|char|str|[ui](?:8|16|32|64|128)|usize|isize|f(?:16|32|64|128)", value["name"]):
            sort["shape"] = "scalar"
        elif kind == "named" and value["name"] == "alloc::boxed::Box" and value["args"]:
            # Usually handled by the real Box definition above; no guessed members.
            sort["shape"] = "opaque"
        else:
            sort["opaque_reason"] = "No member definition in the supplied source catalog."
        sort["edges"] = edges

    def references(self, value):
        if isinstance(value, dict):
            if value.get("kind") == "named":
                try:
                    yield self.canonical(value["name"])
                except ValueError:
                    yield value["name"]
            for child in value.values():
                yield from self.references(child)
        elif isinstance(value, list):
            for child in value:
                yield from self.references(child)

    def slice(self, api, rule):
        # API/rule text retrieves type definitions, not argument bindings or roots.
        words = set(re.findall(r"[A-Za-z_]\w*", api + "\n" + rule))
        selected = set()
        seeds = set()
        qualified = {}
        for path in re.findall(r"(?:[A-Za-z_]\w*::)+[A-Za-z_]\w*", api + "\n" + rule):
            try:
                target = self.canonical(path)
            except ValueError:
                continue
            if target in self.definitions:
                qualified.setdefault(path.rsplit("::", 1)[-1], set()).add(target)
        for word in words:
            seeds.update(qualified.get(word, self.short_names.get(word, [])))
        queue = list(seeds | selected)
        while queue:
            name = queue.pop()
            if name not in self.definitions:
                continue
            if name not in selected:
                selected.add(name)
            definition = self.definitions[name]
            dependencies = set(self.references(definition))
            dependencies.update(self.references(self.selectors.get(name, [])))
            for dep in dependencies:
                if dep in self.definitions and dep not in selected:
                    selected.add(dep)
                    queue.append(dep)
        return {
            "version": 2,
            "definitions": {n: self.definitions[n] for n in sorted(selected)},
            "selectors": {n: self.selectors.get(n, []) for n in sorted(selected)},
            "aliases": {n: t for n, t in self.aliases.items() if t in selected},
            "selection": {"stdlib_seed_types": sorted(seeds), "selected_type_count": len(selected), "total_type_count": len(self.definitions)},
        }

