from dataclasses import dataclass

from dsl.ast import (
    BinaryExpression,
    CallExpression,
    Expression,
    Identifier,
    Literal,
    SourceRef,
    UnaryExpression,
)
from dsl.errors import DSLValidationError


@dataclass(frozen=True)
class SimplifiedVariable:
    name: str


SimplifiedExpression = SimplifiedVariable | Literal | UnaryExpression | BinaryExpression


@dataclass(frozen=True)
class SimplifiedDSL:
    variables: dict[str, CallExpression]
    simplified: SimplifiedExpression


def simplify_variables(ast: Expression) -> SimplifiedDSL:
    variables: dict[str, CallExpression] = {}
    simplified = _simplify_expression(ast, variables)
    return SimplifiedDSL(variables=variables, simplified=simplified)


def _simplify_expression(
    ast: Expression,
    variables: dict[str, CallExpression],
) -> SimplifiedExpression:
    if isinstance(ast, CallExpression):
        variable_name = _call_to_variable_name(ast)
        variables[variable_name] = ast
        return SimplifiedVariable(variable_name)

    if isinstance(ast, Literal):
        return ast

    if isinstance(ast, UnaryExpression):
        return UnaryExpression(
            operator=ast.operator,
            operand=_simplify_expression(ast.operand, variables),
        )

    if isinstance(ast, BinaryExpression):
        return BinaryExpression(
            operator=ast.operator,
            left=_simplify_expression(ast.left, variables),
            right=_simplify_expression(ast.right, variables),
        )

    raise DSLValidationError(f"Cannot simplify non-call expression: {ast!r}")


def _call_to_variable_name(ast: CallExpression) -> str:
    parts = _call_to_variable_parts(ast)
    parts.reverse()
    return ".".join(parts)


def _call_to_variable_parts(ast: CallExpression) -> list[str]:
    if ast.name == "get_var":
        return [_literal_string_arg(ast, 0)]

    if ast.name == "get_result":
        _expect_arg_count(ast, 0)
        return ["result"]

    if ast.name == "get_receiver":
        _expect_arg_count(ast, 0)
        return ["receiver"]

    if ast.name == "get_alloc":
        _expect_arg_count(ast, 1)
        return ["alloc", *_call_arg_to_variable_parts(ast, 0)]

    if ast.name == "get_field":
        _expect_arg_count(ast, 2)
        return [_literal_string_arg(ast, 1), *_call_arg_to_variable_parts(ast, 0)]

    if ast.name == "load":
        _expect_arg_count(ast, 1)
        return ["load", *_call_arg_to_variable_parts(ast, 0)]

    # Keep unknown operators verifiable by giving them a stable symbolic key.
    args = ",".join(_expr_to_variable_atom(arg) for arg in ast.args)
    return [f"{ast.name}({args})"]


def _expr_to_variable_atom(expr: Expression) -> str:
    if isinstance(expr, CallExpression):
        return _call_to_variable_name(expr)
    if isinstance(expr, Literal):
        return repr(expr.value)
    if isinstance(expr, Identifier):
        return expr.name
    if isinstance(expr, SourceRef):
        return f"{expr.name}@{expr.line}:{expr.column}"
    if isinstance(expr, UnaryExpression):
        return f"{expr.operator}{_expr_to_variable_atom(expr.operand)}"
    if isinstance(expr, BinaryExpression):
        left = _expr_to_variable_atom(expr.left)
        right = _expr_to_variable_atom(expr.right)
        return f"({left}{expr.operator}{right})"
    raise DSLValidationError(f"Cannot stringify unsupported expression: {expr!r}")


def _call_arg_to_variable_parts(ast: CallExpression, arg_index: int) -> list[str]:
    arg = ast.args[arg_index]
    if not isinstance(arg, CallExpression):
        raise DSLValidationError(
            f"Expected call expression argument {arg_index} for {ast.name!r}, got {arg!r}"
        )
    return _call_to_variable_parts(arg)


def _literal_string_arg(ast: CallExpression, arg_index: int) -> str:
    _expect_arg_count(ast, arg_index + 1)
    arg = ast.args[arg_index]
    if not isinstance(arg, Literal) or not isinstance(arg.value, str):
        raise DSLValidationError(
            f"Expected string literal argument {arg_index} for {ast.name!r}, got {arg!r}"
        )
    return arg.value


def _expect_arg_count(ast: CallExpression, expected: int) -> None:
    if len(ast.args) != expected:
        raise DSLValidationError(
            f"Expected {expected} argument(s) for {ast.name!r}, got {len(ast.args)}"
        )
