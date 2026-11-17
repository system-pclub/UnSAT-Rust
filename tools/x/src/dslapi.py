"""Fixed, numbered API inputs, independent of the natural-language requirement.

This reads API signature/context strings. Member schemas come from dslgen's
library declaration catalog. Type/lifetime parameters are opaque handles until
an explicit analysis-domain schema supplies their semantic properties.
"""
from __future__ import annotations

from dataclasses import asdict, dataclass
from pathlib import Path
import re

from dsltypes import display


def split_top(text, separator=","):
    result, start, stack = [], 0, []
    pairs = {"<": ">", "(": ")", "[": "]", "{": "}"}
    for i, char in enumerate(text):
        if char in pairs:
            stack.append(pairs[char])
        elif stack and char == stack[-1] and not (char == ">" and i and text[i-1] == "-"):
            stack.pop()
        elif char == separator and not stack:
            result.append(text[start:i].strip())
            start = i + 1
    result.append(text[start:].strip())
    return [x for x in result if x]


def group(text, start):
    opening = text[start]
    closing = {"<": ">", "(": ")", "[": "]", "{": "}"}[opening]
    depth = 0
    for i in range(start, len(text)):
        char = text[i]
        if char == opening:
            depth += 1
        elif char == closing and not (char == ">" and i and text[i-1] == "-"):
            depth -= 1
            if depth == 0:
                return text[start+1:i], i+1
    raise ValueError("unclosed API signature delimiter")


def split_body(text):
    """Separate an impl/trait body without splitting const-generic expressions."""
    stack = []
    pairs = {"<": ">", "(": ")", "[": "]", "{": "}"}
    for i, char in enumerate(text):
        if char == "{" and not stack:
            return text[:i], text[i+1:]
        if char in pairs:
            stack.append(pairs[char])
        elif stack and char == stack[-1] and not (char == ">" and i and text[i-1] == "-"):
            stack.pop()
    return text, ""


@dataclass(frozen=True)
class ApiVariable:
    symbol: str
    source_name: str
    kind: str
    rust_type: str
    dsl_type: dict

    @property
    def internal(self):
        return "__dsl_slot_" + self.symbol[1:]

    def record(self):
        return asdict(self)


class ApiInterface:
    def __init__(self, catalog, api):
        self.variables = []
        self.catalog = catalog
        self.generics = []
        self._generic_names = set()
        self.owner = None
        self._parse(api.strip())

    def type_ref(self, text):
        try:
            return self.catalog.parse(text)
        except ValueError:
            # These valid Rust forms have no modeled operations yet. Preserve
            # their source type as opaque instead of dropping the API's inputs.
            if text.strip() == "!" or re.search(r"\b(?:dyn|impl|fn)\b|<[^>]+\bas\b", text):
                return {"kind": "opaque", "rust": text}
            raise

    def add(self, name, kind, rust_type, ty=None):
        if ty is None:
            ty = self.type_ref(rust_type)
        if kind in {"argument", "return_value"}:
            owner = self.type_ref(self.owner) if self.owner else {"kind": "parameter", "name": "Self"}
            if owner["kind"] == "named" and owner["name"] not in self.catalog.definitions and not re.fullmatch(r"bool|char|str|[ui](?:8|16|32|64|128)|usize|isize|f(?:16|32|64|128)", owner["name"]):
                owner = {"kind": "parameter", "name": "Self"}

            def resolve_self(value):
                if isinstance(value, dict):
                    if value.get("kind") in {"named", "parameter"} and value.get("name") == "Self":
                        return owner
                    return {key: resolve_self(item) for key, item in value.items()}
                if isinstance(value, list):
                    return [resolve_self(item) for item in value]
                return value

            ty = resolve_self(ty)
        self.variables.append(ApiVariable(f"${len(self.variables)+1}", name, kind, rust_type, ty))

    def generic(self, text, kind=None, const_type=None):
        text = text.strip()
        if not text:
            return
        if text.startswith("const "):
            name, _, const_type = text[6:].partition(":")
            name = name.strip()
            kind = "const_parameter"
            const_type = const_type.split("=", 1)[0].strip()
        elif text.startswith("'"):
            name = text.split(":", 1)[0].strip()
            kind = "lifetime_parameter"
            if name in {"'static", "'_"}:
                return
        else:
            name = re.split(r"[:=]", text, maxsplit=1)[0].strip()
            kind = kind or "type_parameter"
        if name in self._generic_names:
            return
        if not re.fullmatch(r"'?[A-Za-z_]\w*", name):
            # Concrete substitutions are recorded with their declaration name by
            # the owner-argument path, rather than inventing variable names.
            raise ValueError(f"unsupported API generic parameter: {text}")
        self._generic_names.add(name)
        self.generics.append((name, kind, const_type))

    def _parse(self, api):
        if not api:
            return
        rest = api
        impl = re.match(r"(?:unsafe\s+)?impl\s*", rest)
        if impl:
            pos = impl.end()
            if rest[pos:pos+1] == "<":
                params, pos = group(rest, pos)
                for param in split_top(params):
                    self.generic(param)
            owner = split_body(rest[pos:])[0].strip()
            # Trait impls use the implementing type after `for`.
            owner = re.split(r"\s+for\s+", owner)[-1]
            owner = re.split(r"\s+where\s+", owner)[0].strip()
            self.owner = owner
            rest = split_body(rest)[1]
        trait = re.match(r"(?:unsafe\s+)?trait\s+([\w:]+)", rest)
        if trait:
            self.generic("Self")
            pos = trait.end()
            while pos < len(rest) and rest[pos].isspace():
                pos += 1
            if rest[pos:pos+1] == "<":
                params, _ = group(rest, pos)
                for param in split_top(params):
                    self.generic(param)
            if not re.search(r"\bfn\s", rest):
                self._add_generics()
                return
            self.owner = "Self"
            rest = split_body(rest)[1]

        # Locate the function parameter list outside angle/array groups. For
        # declaration fragments in tools/tests, accept a typed parameter list.
        fn = re.search(r"\bfn\s+", rest)
        head = rest[fn.end():] if fn else rest
        pos, nesting = None, []
        for i, char in enumerate(head):
            if char in "<[":
                nesting.append({"<": ">", "[": "]"}[char])
            elif nesting and char == nesting[-1]:
                nesting.pop()
            elif char == "(" and not nesting:
                pos = i
                break
        if pos is None:
            if not re.search(r"(?<!:):(?!:)", head):
                raise ValueError("API must include its parameter signature (or a trait declaration)")
            args, prefix, tail = head, "", ""
        else:
            prefix = head[:pos].strip()
            args, end = group(head, pos)
            tail = head[end:].strip()
            # Get all explicit path generic groups. Owner headers classify const
            # arguments; method generic declarations carry their own kinds.
            # Normalize turbofish punctuation for declaration-name lookup.
            prefix = re.sub(r"\s*::\s*", "::", prefix).replace("::<", "<")
            owner_end, angle = None, 0
            for i, char in enumerate(prefix):
                if char == "<":
                    angle += 1
                elif char == ">":
                    angle -= 1
                elif not angle and prefix[i:i+2] == "::":
                    owner_end = i
            if self.owner is None and owner_end is not None:
                self.owner = prefix[:owner_end]
            groups, cursor = [], 0
            while cursor < len(prefix):
                if prefix[cursor] != "<":
                    cursor += 1
                    continue
                contents, end = group(prefix, cursor)
                groups.append((prefix[:cursor].strip().removesuffix("::"), contents, end))
                cursor = end
            for before, contents, end in groups:
                is_owner = bool(prefix[end:].lstrip().startswith("::"))
                definition = self.catalog.definitions.get(self.catalog.canonical(before), {}) if is_owner else {}
                headers = definition.get("parameters", [])
                header_index = 0
                for param in split_top(contents):
                    if param.startswith("'"):
                        self.generic(param)
                        continue
                    header = headers[header_index] if header_index < len(headers) else {}
                    header_index += 1
                    if header and not re.fullmatch(r"[A-Z]\w*", param) and not param.startswith("'"):
                        # A concrete instantiation still exposes the template
                        # parameter under its declaration name.
                        self.generic(header["name"], "const_parameter" if header.get("kind") == "const" else "type_parameter", display(header["const_type"]) if header.get("const_type") else None)
                    else:
                        self.generic(param, "const_parameter" if header.get("kind") == "const" else None, display(header["const_type"]) if header.get("const_type") else None)
            # A shorthand owner path without <...> still has source-declared
            # generics. Explicit impl contexts already declare their binders.
            if not impl and self.owner and "<" not in self.owner:
                definition = self.catalog.definitions.get(self.catalog.canonical(self.owner), {})
                owner_parameters = []
                for parameter in definition.get("parameters", []):
                    if parameter["name"] not in self._generic_names:
                        kind = "const_parameter" if parameter.get("kind") == "const" else "type_parameter"
                        const_type = display(parameter["const_type"]) if parameter.get("const_type") else None
                        owner_parameters.append((parameter["name"], kind, const_type))
                self.generics = owner_parameters + self.generics

        unresolved_self = False
        for arg in split_top(args):
            receiver = re.fullmatch(r"(?:&\s*(?:'\w+\s*)?(?:mut\s+)?)?(?:mut\s+)?self", arg)
            if receiver:
                owner = self.type_ref(self.owner) if self.owner else {"kind": "parameter", "name": "Self"}
                if owner["kind"] == "named" and owner["name"] not in self.catalog.definitions and not re.fullmatch(r"bool|char|str|[ui](?:8|16|32|64|128)|usize|isize|f(?:16|32|64|128)", owner["name"]):
                    owner = {"kind": "parameter", "name": "Self"}
                unresolved_self |= owner == {"kind": "parameter", "name": "Self"}
                self.add("self", "argument", self.owner or "Self", owner)
                continue
            match = re.fullmatch(r"(?:mut\s+)?([A-Za-z_]\w*|\([^:]+\))\s*:(?!:)\s*(.+)", arg, re.S)
            if not match:
                raise ValueError(f"API argument needs a simple source name and type: {arg}")
            name, rust = match.groups()
            self.add(name, "argument", rust)
        if unresolved_self and "Self" not in self._generic_names:
            self.generics.insert(0, ("Self", "type_parameter", None))
        self._add_generics()
        if tail.startswith("->"):
            result = tail[2:].split("}", 1)[0].strip().rstrip(";").strip()
            result = re.split(r"\s+where\s+", result)[0].strip()
            if result and result != "()":
                self.add("result", "return_value", result)

    def _add_generics(self):
        for name, kind, const_type in self.generics:
            if kind == "const_parameter":
                if not const_type:
                    raise ValueError(f"const generic {name} needs a declared source type")
                self.add(name, kind, const_type)
            elif kind == "lifetime_parameter":
                self.add(name, kind, "lifetime metadata", {"kind": "opaque", "rust": "lifetime parameter " + name})
            else:
                self.add(name, kind, "type metadata", {"kind": "opaque", "rust": "type parameter " + name})

    def records(self):
        return [v.record() for v in self.variables]

    def comments(self):
        return "\n".join(f"(* {v.symbol} is {v.source_name}: {v.rust_type} [{v.kind}]. *)" for v in self.variables) + "\n"

    def rewrite(self, expression):
        """Rewrite slot tokens for ast only, preserving string literal contents."""
        allowed = {v.symbol: v.internal for v in self.variables}
        output, i = [], 0
        while i < len(expression):
            char = expression[i]
            if char in "\"'":
                quote, start = char, i
                i += 1
                while i < len(expression):
                    if expression[i] == "\\":
                        i += 2
                    elif expression[i] == quote:
                        i += 1
                        break
                    else:
                        i += 1
                output.append(expression[start:i])
            elif char == "$":
                if expression.startswith("$mem", i) and (i+4 == len(expression) or not (expression[i+4].isalnum() or expression[i+4] == "_")) and expression[:i].rstrip().endswith("."):
                    output.append("__dsl_mem")
                    i += 4
                    continue
                match = re.match(r"\$\d+", expression[i:])
                symbol = match[0] if match else "$"
                if symbol not in allowed:
                    raise ValueError(f"unknown API variable {symbol}")
                output.append(allowed[symbol])
                i += len(symbol)
            elif char.isalpha() or char == "_":
                match = re.match(r"\w+", expression[i:])
                token = match[0]
                if token.startswith("__dsl_slot_") or token == "__dsl_mem":
                    raise ValueError("reserved internal variable name")
                output.append(token)
                i += len(token)
            else:
                output.append(char)
                i += 1
        return "".join(output)


def qualify_api(context, catalog):
    api = re.sub(r"\s*::\s*", "::", context["api"])
    formal = {v["source_name"] for v in context["api_variables"]}
    formal.add("Self")
    formal.update(re.findall(r"\b(?:fn|trait)\s+([A-Za-z_]\w*)", api))
    path = Path(context["source"])
    parts = list(path.parts)
    try:
        at = parts.index("library")
        crate = parts[at+1]
        tail = parts[at+3:]
        tail[-1] = Path(tail[-1]).stem
        if tail[-1] in {"lib", "mod"}:
            tail.pop()
        module = "::".join([crate] + tail)
    except (ValueError, IndexError):
        return api
    aliases = {}
    names = set(re.findall(r"(?<![\w:'])[A-Z][A-Za-z_0-9]*(?![\w:])", api)) - formal
    for name in names:
        scope = module
        while scope:
            candidate = scope + "::" + name
            try:
                resolved = catalog.canonical(candidate)
            except ValueError:
                break
            if resolved in catalog.definitions:
                aliases[name] = resolved
                break
            scope = scope.rsplit("::", 1)[0] if "::" in scope else ""
        if name not in aliases:
            try:
                canonical = catalog.canonical(name)
            except ValueError:
                canonical = name
            if canonical in catalog.definitions and name not in {"Option", "Result", "Vec", "String", "Box"}:
                # A globally unique short name can belong to another platform.
                # Keep the local unresolved alias opaque instead of binding to it.
                aliases[name] = module + "::" + name
        # Rust's implicit prelude, after explicit local imports/declarations.
        if name not in aliases and name in {"Option", "Result", "Vec", "String", "Box"}:
            aliases[name] = {"Option": "core::option::Option", "Result": "core::result::Result", "Vec": "alloc::vec::Vec", "String": "alloc::string::String", "Box": "alloc::boxed::Box"}[name]
    return re.sub(r"(?<![\w:'])[A-Z][A-Za-z_0-9]*(?![\w:])", lambda m: aliases.get(m[0], m[0]), api)

