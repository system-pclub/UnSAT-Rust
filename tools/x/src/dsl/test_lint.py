from __future__ import annotations

from dsl import parse_dsl
from dsl.lint import lint_dsl_ast, lint_rule_semantics


OPERATORS = [
    {"name": "get_arg", "input": [{"name": "index"}]},
    {"name": "get_field", "input": [{"name": "base"}, {"name": "field"}]},
    {"name": "memory_range", "input": [{"name": "ptr"}, {"name": "size"}]},
    {"name": "is_valid_for", "input": [{"name": "range"}, {"name": "access"}]},
    {"name": "is_initialized", "input": [{"name": "range"}]},
    {
        "name": "mul_fits",
        "input": [
            {"name": "a"},
            {"name": "b"},
            {"name": "target"},
        ],
    },
]


def _codes(source: str) -> set[str]:
    return {issue.code for issue in lint_dsl_ast(parse_dsl(source, OPERATORS), OPERATORS)}


def test_rejects_placeholder_access_kind() -> None:
    assert "placeholder-access-kind" in _codes(
        'is_valid_for(memory_range(get_arg(0), 1), "rule-1")'
    )


def test_warns_for_untranslated_access_kind() -> None:
    assert "unsupported-access-kind" in _codes(
        'is_valid_for(memory_range(get_arg(0), 1), "unique_borrow")'
    )


def test_warns_for_untyped_initialization() -> None:
    assert "untyped-initialization" in _codes(
        "is_initialized(memory_range(get_arg(0), 1))"
    )


def test_checks_mul_fits_literals() -> None:
    assert not _codes('mul_fits(get_arg(0), 4, "isize")')
    assert "mul-fits-target" in _codes('mul_fits(get_arg(0), 4, "u16")')


def test_rule_semantics_rejects_extra_overflow_condition() -> None:
    ast = parse_dsl(
        'mul_fits(get_arg(0), 4, "usize") && '
        'is_valid_for(memory_range(get_arg(1), get_arg(0) * 4), "write")',
        OPERATORS,
    )
    issues = lint_rule_semantics("dst must be valid for writes", ast)
    assert "extraneous-mul-fits" in {issue.code for issue in issues}


def test_rule_semantics_requires_mul_fits_for_isize_rule() -> None:
    ast = parse_dsl("get_arg(0) != 0", OPERATORS)
    issues = lint_rule_semantics(
        "count * size_of::<T>() must fit in an `isize`", ast
    )
    assert "missing-mul-fits" in {issue.code for issue in issues}


def test_rule_semantics_rejects_null_exemption_on_alignment_rule() -> None:
    ast = parse_dsl(
        'get_field(get_arg(0), "is_null") || get_arg(0) != 0', OPERATORS
    )
    issues = lint_rule_semantics("The pointer must be aligned", ast)
    assert "extraneous-null-exemption" in {issue.code for issue in issues}
