import importlib.resources
import json
from typing import Any, Dict, List
from dataclasses import dataclass

import jsonschema

from verirule.errors import RuleValidationError


def load_rule_schema() -> Dict[str, Any]:
    ref = importlib.resources.files("verirule").joinpath("rule.schema.json")
    with importlib.resources.as_file(ref) as path:
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)


RULE_SCHEMA = load_rule_schema()

@dataclass(frozen=True)
class AbstractDef:
    name: str
    parameter_sorts: List[str]
    return_sort: str


@dataclass(frozen=True)
class VariableDef:
    name: str
    sort: str
    selector: Dict[str, Any]


def validate_rule(rule: Dict[str, Any]) -> None:
    """
    Validate the rule using:
    1. JSON Schema structural validation
    2. Semantic validation for references and selector typing
    """

    jsonschema.validate(instance=rule, schema=RULE_SCHEMA)

    function = rule["function"]

    argument_names = [arg["name"] for arg in function["arguments"]]
    argument_sorts = {
        arg["name"]: arg.get("sort")
        for arg in function["arguments"]
    }

    result = function.get("result")
    result_name = result["name"] if result else None
    result_sort = result.get("sort") if result else None

    abstracts = parse_abstracts(rule)
    variables = parse_variables(rule)

    check_unique(argument_names, "function argument")
    check_unique(list(abstracts.keys()), "abstract")
    check_unique(list(variables.keys()), "variable")

    validate_selectors(
        variables=variables,
        abstracts=abstracts,
        argument_names=set(argument_names),
        argument_sorts=argument_sorts,
        result_name=result_name,
        result_sort=result_sort,
    )

    validate_expr_references(rule["buggy_violation"], variables)


def parse_abstracts(rule: Dict[str, Any]) -> Dict[str, AbstractDef]:
    result: Dict[str, AbstractDef] = {}

    for abs_def in rule["abstracts"]:
        name = abs_def["name"]
        if name in result:
            raise RuleValidationError(f"Duplicate abstract: {name}")

        result[name] = AbstractDef(
            name=name,
            parameter_sorts=[
                param["sort"]
                for param in abs_def["parameters"]
            ],
            return_sort=abs_def["returns"]["sort"],
        )

    return result


def parse_variables(rule: Dict[str, Any]) -> Dict[str, VariableDef]:
    result: Dict[str, VariableDef] = {}

    for var_def in rule["variables"]:
        name = var_def["name"]
        if name in result:
            raise RuleValidationError(f"Duplicate variable: {name}")

        result[name] = VariableDef(
            name=name,
            sort=var_def["sort"],
            selector=var_def["selector"],
        )

    return result


def check_unique(values: List[str], what: str) -> None:
    seen = set()
    for value in values:
        if value in seen:
            raise RuleValidationError(f"Duplicate {what}: {value}")
        seen.add(value)


def validate_selectors(
    *,
    variables: Dict[str, VariableDef],
    abstracts: Dict[str, AbstractDef],
    argument_names: set[str],
    argument_sorts: Dict[str, str | None],
    result_name: str | None,
    result_sort: str | None,
) -> None:
    for variable in variables.values():
        selector = variable.selector
        apply_name = selector["apply"]

        if apply_name not in abstracts:
            raise RuleValidationError(
                f"Variable {variable.name} uses unknown abstract: {apply_name}"
            )

        abstract = abstracts[apply_name]
        selector_args = selector["args"]

        if len(selector_args) != len(abstract.parameter_sorts):
            raise RuleValidationError(
                f"Variable {variable.name} selector {apply_name} expects "
                f"{len(abstract.parameter_sorts)} args, got {len(selector_args)}"
            )

        if variable.sort != abstract.return_sort:
            raise RuleValidationError(
                f"Variable {variable.name} sort mismatch: variable sort is "
                f"{variable.sort}, but abstract {apply_name} returns {abstract.return_sort}"
            )

        for index, selector_arg in enumerate(selector_args):
            expected_sort = abstract.parameter_sorts[index]

            if "argument" in selector_arg:
                arg_name = selector_arg["argument"]

                if arg_name not in argument_names:
                    raise RuleValidationError(
                        f"Variable {variable.name} selector references unknown "
                        f"function argument: {arg_name}"
                    )

                actual_sort = argument_sorts.get(arg_name)
                if actual_sort is not None and actual_sort != expected_sort:
                    raise RuleValidationError(
                        f"Variable {variable.name} selector arg {arg_name} sort mismatch: "
                        f"expected {expected_sort}, got {actual_sort}"
                    )

            elif "result" in selector_arg:
                selected_result = selector_arg["result"]

                if result_name is None:
                    raise RuleValidationError(
                        f"Variable {variable.name} selector references result, "
                        f"but function has no result"
                    )

                if selected_result != result_name:
                    raise RuleValidationError(
                        f"Variable {variable.name} selector references unknown result: "
                        f"{selected_result}"
                    )

                if result_sort is not None and result_sort != expected_sort:
                    raise RuleValidationError(
                        f"Variable {variable.name} selector result sort mismatch: "
                        f"expected {expected_sort}, got {result_sort}"
                    )

            else:
                raise RuleValidationError(
                    f"Variable {variable.name} selector arg must be argument or result"
                )


def validate_expr_references(expr: Dict[str, Any], variables: Dict[str, VariableDef]) -> None:
    if "var" in expr:
        name = expr["var"]
        if name not in variables:
            raise RuleValidationError(f"Unknown variable in expression: {name}")
        return

    if "int" in expr or "bool" in expr or "string" in expr:
        return

    if "op" in expr:
        op = expr["op"]
        args = expr["args"]

        validate_op_arity(op, args)

        for arg in args:
            validate_expr_references(arg, variables)
        return

    raise RuleValidationError(f"Invalid expression: {expr}")


def validate_op_arity(op: str, args: List[Dict[str, Any]]) -> None:
    binary_ops = {
        "=",
        "!=",
        ">",
        ">=",
        "<",
        "<=",
        "+",
        "-",
        "*",
        "/",
        "=>",
    }

    if op in binary_ops and len(args) != 2:
        raise RuleValidationError(f"Operator {op} expects 2 args, got {len(args)}")

    if op == "not" and len(args) != 1:
        raise RuleValidationError(f"Operator not expects 1 arg, got {len(args)}")

    if op in {"and", "or"} and len(args) < 1:
        raise RuleValidationError(f"Operator {op} expects at least 1 arg")


