import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from dsl import BinaryExpression, CallExpression, DSLValidationError, UnaryExpression, parse_dsl  # noqa: E402
from dsl.simplifier import SimplifiedVariable, simplify_variables  # noqa: E402


OPERATORS_PATH = Path(__file__).resolve().parents[4] / "operators.json"


def test_simplify_get_var_alloc_field_to_variables() -> None:
    ast = parse_dsl(
        'get_field(get_alloc(get_var("p")), "layout") == get_var("layout")',
        OPERATORS_PATH,
    )

    result = simplify_variables(ast)

    assert result.variables == {
        "p.alloc.layout": ast.left,
        "layout": ast.right,
    }
    assert result.simplified == BinaryExpression(
        operator="==",
        left=SimplifiedVariable("p.alloc.layout"),
        right=SimplifiedVariable("layout"),
    )


def test_simplify_keeps_only_top_level_calls_as_variables() -> None:
    ast = parse_dsl(
        'get_field(get_alloc(get_var("p")), "layout") != get_field(get_var("q"), "len")',
        OPERATORS_PATH,
    )

    result = simplify_variables(ast)

    assert set(result.variables) == {"p.alloc.layout", "q.len"}
    assert isinstance(result.variables["p.alloc.layout"], CallExpression)
    assert result.simplified == BinaryExpression(
        operator="!=",
        left=SimplifiedVariable("p.alloc.layout"),
        right=SimplifiedVariable("q.len"),
    )


def test_simplify_preserves_unary_and_binary_shape() -> None:
    ast = parse_dsl('!(get_var("ready") == get_var("done"))', OPERATORS_PATH)

    result = simplify_variables(ast)

    assert result.variables == {
        "ready": ast.operand.left,
        "done": ast.operand.right,
    }
    assert result.simplified == UnaryExpression(
        operator="!",
        operand=BinaryExpression(
            operator="==",
            left=SimplifiedVariable("ready"),
            right=SimplifiedVariable("done"),
        ),
    )


def test_simplify_unknown_call_stringification_is_stable() -> None:
    ast = parse_dsl("offset_in_alloc(get_var(\"p\"))", OPERATORS_PATH)
    result = simplify_variables(ast)

    assert set(result.variables) == {"offset_in_alloc(p)"}
    assert result.simplified == SimplifiedVariable("offset_in_alloc(p)")


def test_simplify_get_result_and_get_receiver() -> None:
    ast = parse_dsl("get_alloc(get_result()) == get_alloc(get_receiver())", OPERATORS_PATH)

    result = simplify_variables(ast)

    assert set(result.variables) == {"result.alloc", "receiver.alloc"}
    assert result.simplified == BinaryExpression(
        operator="==",
        left=SimplifiedVariable("result.alloc"),
        right=SimplifiedVariable("receiver.alloc"),
    )


def test_simplify_rejects_non_call_leaf() -> None:
    ast = parse_dsl("count@1:2 == get_var(\"count\")", OPERATORS_PATH)

    with pytest.raises(DSLValidationError, match="Cannot simplify non-call expression"):
        simplify_variables(ast)
