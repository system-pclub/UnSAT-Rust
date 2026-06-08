

from typing import Any, Dict

from verirule.errors import RuleValidationError
from verirule.schema import parse_variables, validate_rule


def translate_buggy_violation(rule: Dict[str, Any]) -> str:
    """
    Validate the rule, then translate buggy_violation into a simple DSL string.
    """

    validate_rule(rule)

    variables = parse_variables(rule)
    var_to_selector_text = {
        var_name: selector_to_text(var_def.selector)
        for var_name, var_def in variables.items()
    }

    return expr_to_dsl(rule["buggy_violation"], var_to_selector_text)


def selector_to_text(selector: Dict[str, Any]) -> str:
    apply_name = selector["apply"]
    args = []

    for arg in selector["args"]:
        if "argument" in arg:
            args.append(arg["argument"])
        elif "result" in arg:
            args.append(arg["result"])
        else:
            raise RuleValidationError(f"Invalid selector arg: {arg}")

    return f"{apply_name}({', '.join(args)})"


def expr_to_dsl(expr: Dict[str, Any], var_to_selector_text: Dict[str, str]) -> str:
    if "var" in expr:
        name = expr["var"]
        return var_to_selector_text.get(name, name)

    if "int" in expr:
        return str(expr["int"])

    if "bool" in expr:
        return "true" if expr["bool"] else "false"

    if "string" in expr:
        return repr(expr["string"])

    op = expr["op"]
    args = expr["args"]

    if op in {"=", "!=", ">", ">=", "<", "<=", "+", "-", "*", "/", "=>"}:
        left = expr_to_dsl(args[0], var_to_selector_text)
        right = expr_to_dsl(args[1], var_to_selector_text)

        readable_op = "==" if op == "=" else op
        readable_op = "implies" if op == "=>" else readable_op

        return f"({left} {readable_op} {right})"

    if op == "not":
        inner = expr_to_dsl(args[0], var_to_selector_text)
        return f"(NOT {inner})"

    if op == "and":
        parts = [expr_to_dsl(arg, var_to_selector_text) for arg in args]
        return "(\n  " + "\n  AND ".join(parts) + "\n)"

    if op == "or":
        parts = [expr_to_dsl(arg, var_to_selector_text) for arg in args]
        return "(\n  " + "\n  OR ".join(parts) + "\n)"

    raise RuleValidationError(f"Unsupported op: {op}")