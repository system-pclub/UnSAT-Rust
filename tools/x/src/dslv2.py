"""Lower a validated source-derived DSL expression to the JSON read by KLEE.

The expression is the Starlark-style formula checked by ``dsltrans.Model``.
Nothing here evaluates it: this module only fixes the ``$N`` slot table and a
plain JSON syntax tree. KLEE resolves attributes and ``.$mem`` projections from
the actual callee's debug information, so there is one generic path for every
Rust type instead of a resolver per type.
"""
from __future__ import annotations

import ast
import json

import dslmemory

VERSION = 2
_SLOT_PREFIX = "__dsl_slot_"
_CMP = {
    ast.Eq: "==", ast.NotEq: "!=", ast.Lt: "<", ast.LtE: "<=", ast.Gt: ">",
    ast.GtE: ">=", ast.In: "in", ast.NotIn: "not in", ast.Is: "==", ast.IsNot: "!=",
}
_BIN = {ast.Add: "+", ast.Sub: "-", ast.Mult: "*", ast.FloorDiv: "//", ast.Mod: "%"}
_CALLS = {"len", "all", "any", "is_variant"}


class LoweringError(ValueError):
    pass


def slot_table(interface):
    """Describe ``$N`` without exposing compiler internals of the variables."""
    slots, argument = [], 0
    generic = 0
    for variable in interface.variables:
        slot = {
            "slot": int(variable.symbol[1:]),
            "name": variable.source_name,
            "kind": variable.kind,
            "rust_type": variable.rust_type,
        }
        if variable.kind == "argument":
            slot["arg_index"] = argument
            argument += 1
        elif variable.kind in {"type_parameter", "const_parameter"}:
            slot["generic_index"] = generic
            generic += 1
        slots.append(slot)
    return slots


def _node(node):
    if isinstance(node, ast.Constant):
        value = node.value
        if value is None:
            return {"k": "none"}
        if isinstance(value, bool):
            return {"k": "bool", "v": value}
        if isinstance(value, int):
            return {"k": "int", "v": value if -(1 << 63) <= value < (1 << 63) else str(value)}
        if isinstance(value, str):
            return {"k": "str", "v": value}
        raise LoweringError(f"unsupported literal {value!r}")
    if isinstance(node, ast.Name):
        if node.id in {"True", "False", "None"}:
            raise LoweringError("unexpected keyword name")
        if node.id.startswith(_SLOT_PREFIX):
            return {"k": "slot", "n": int(node.id[len(_SLOT_PREFIX):])}
        return {"k": "name", "id": node.id}
    if isinstance(node, ast.Attribute):
        base = _node(node.value)
        if node.attr == dslmemory.ATTRIBUTE:
            return {"k": "mem", "base": base}
        return {"k": "attr", "base": base, "name": node.attr}
    if isinstance(node, ast.Subscript):
        return {"k": "index", "base": _node(node.value), "index": _node(node.slice)}
    if isinstance(node, ast.UnaryOp):
        op = {ast.Not: "not", ast.USub: "-", ast.UAdd: "+"}.get(type(node.op))
        if op is None:
            raise LoweringError("unsupported unary operator")
        return {"k": "un", "op": op, "operand": _node(node.operand)}
    if isinstance(node, ast.BinOp):
        op = _BIN.get(type(node.op))
        if op is None:
            raise LoweringError("unsupported arithmetic operator")
        return {"k": "bin", "op": op, "left": _node(node.left), "right": _node(node.right)}
    if isinstance(node, ast.BoolOp):
        return {"k": "bool_op", "op": "and" if isinstance(node.op, ast.And) else "or",
                "values": [_node(v) for v in node.values]}
    if isinstance(node, ast.Compare):
        return {"k": "cmp", "left": _node(node.left),
                "ops": [_CMP[type(op)] for op in node.ops],
                "rights": [_node(v) for v in node.comparators]}
    if isinstance(node, ast.IfExp):
        return {"k": "if", "test": _node(node.test), "then": _node(node.body), "else": _node(node.orelse)}
    if isinstance(node, ast.List):
        return {"k": "list", "items": [_node(v) for v in node.elts]}
    if isinstance(node, ast.Tuple):
        return {"k": "list", "items": [_node(v) for v in node.elts]}
    if isinstance(node, ast.ListComp):
        generators = []
        for generator in node.generators:
            if not isinstance(generator.target, ast.Name):
                raise LoweringError("comprehension requires a simple variable")
            generators.append({"target": generator.target.id, "iter": _node(generator.iter),
                               "ifs": [_node(c) for c in generator.ifs]})
        return {"k": "comp", "elt": _node(node.elt), "gens": generators}
    if isinstance(node, ast.Call):
        if not isinstance(node.func, ast.Name) or node.func.id not in _CALLS or node.keywords:
            raise LoweringError("unsupported call")
        return {"k": "call", "name": node.func.id, "args": [_node(a) for a in node.args]}
    raise LoweringError(f"unsupported syntax {type(node).__name__}")


def _slots_read(node, found):
    if isinstance(node, dict):
        if node.get("k") == "slot":
            found.add(node["n"])
        for value in node.values():
            _slots_read(value, found)
    elif isinstance(node, list):
        for value in node:
            _slots_read(value, found)


def lower(model, expression):
    """Validate with the generated grammar/type checker, then lower to JSON."""
    model.validate(expression)
    parsed = ast.parse(model.interface.rewrite(expression.strip()), mode="eval")
    slots = slot_table(model.interface)
    expr = _node(parsed.body)
    read = set()
    _slots_read(expr, read)
    return {
        "dsl_version": VERSION,
        "source": expression.strip(),
        "slots": slots,
        "uses_result": any(s["kind"] == "return_value" and s["slot"] in read for s in slots),
        "expr": expr,
    }


def dumps(document):
    return json.dumps(document, ensure_ascii=False, separators=(",", ":"))
