from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from dsl import (  # noqa: E402
    BinaryExpression,
    CallExpression,
    DSLParseError,
    DSLValidationError,
    SourceRef,
    list_operators,
    parse_dsl,
    validate_task1_ast,
    validate_task2_ast,
)


OPERATORS_PATH = Path(__file__).resolve().parents[4] / "operators.json"


def test_parse_task1_dsl_and_list_operators() -> None:
    ast = parse_dsl(
        "alloc_id(ptr@329:30) != None && offset_in_alloc(ptr@329:30) + i@329:34 <= alloc_block_size(alloc_id(ptr@329:30))",
        OPERATORS_PATH,
    )

    assert isinstance(ast, BinaryExpression)
    assert list_operators(ast) == ["alloc_block_size", "alloc_id", "offset_in_alloc"]
    validate_task1_ast(ast, ["alloc_id", "offset_in_alloc"])


def test_parse_task2_dsl_without_configured_operators() -> None:
    ast = parse_dsl(
        "i@329:34 >= 0 && base_offset@329:18 + i@329:34 <= capacity@329:12",
        OPERATORS_PATH,
    )

    assert list_operators(ast) == []
    validate_task2_ast(ast)


def test_parse_source_ref_leaf() -> None:
    ast = parse_dsl("ptr@329:30", OPERATORS_PATH)

    assert ast == SourceRef(name="ptr", line=329, column=30)


def test_reject_unknown_operator() -> None:
    with pytest.raises(DSLValidationError, match="Unknown operator"):
        parse_dsl("unknown_op(ptr@1:2)", OPERATORS_PATH)


def test_allow_unknown_operator_for_discovery() -> None:
    ast = parse_dsl("fresh_op(ptr@1:2)", OPERATORS_PATH, allow_unknown_operators=True)

    assert list_operators(ast) == ["fresh_op"]


def test_task2_validation_rejects_operator_usage() -> None:
    ast = parse_dsl("alloc_id(ptr@329:30) != None", OPERATORS_PATH)

    with pytest.raises(DSLValidationError, match="must not contain configured operators"):
        validate_task2_ast(ast)


def test_task1_validation_requires_operators() -> None:
    ast = parse_dsl("i@329:34 >= 0", OPERATORS_PATH)

    with pytest.raises(DSLValidationError, match="must contain at least one configured operator"):
        validate_task1_ast(ast, [])


def test_parse_reports_position_for_syntax_error() -> None:
    with pytest.raises(DSLParseError, match=r"line 1, column 15"):
        parse_dsl("alloc_id(ptr@1)", OPERATORS_PATH)


def test_nested_calls_are_preserved() -> None:
    ast = parse_dsl("alloc_block_size(alloc_id(ptr@329:30))", OPERATORS_PATH)

    assert isinstance(ast, CallExpression)
    assert isinstance(ast.args[0], CallExpression)