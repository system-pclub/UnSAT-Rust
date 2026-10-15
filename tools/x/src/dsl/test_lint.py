from __future__ import annotations

from dsl import parse_dsl
from dsl.lint import lint_dsl_ast, lint_rule_semantics


OPERATORS = [
    {"name": "get_arg", "input": [{"name": "index"}]},
    {"name": "get_type_at_func", "input": [{"name": "index"}]},
    {"name": "get_field", "input": [{"name": "base"}, {"name": "field"}]},
    {"name": "memory_range", "input": [{"name": "ptr"}, {"name": "size"}]},
    {"name": "is_valid_for", "input": [{"name": "range"}, {"name": "access"}]},
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


def test_accepts_every_klee_access_kind() -> None:
    access_kinds = {
        "read",
        "write",
        "read_write",
        "volatile_read_no_race",
        "volatile_write_no_race",
        "shared_borrow",
        "unique_borrow",
        "read_no_mutation",
        "no_use_after_drop",
        "drop",
        "exclusive_during_drop",
    }
    for access_kind in access_kinds:
        assert "unsupported-access-kind" not in _codes(
            f'is_valid_for(memory_range(get_arg(0), 1), "{access_kind}")'
        )


def test_warns_for_unknown_access_kind() -> None:
    assert "unsupported-access-kind" in _codes(
        'is_valid_for(memory_range(get_arg(0), 1), "future_access")'
    )


def test_checks_mul_fits_literals() -> None:
    assert not _codes('mul_fits(get_arg(0), 4, "isize")')
    assert "mul-fits-target" in _codes('mul_fits(get_arg(0), 4, "u16")')


def test_get_field_requires_a_literal_field_name() -> None:
    assert "field-name-literal" in _codes("get_field(get_arg(0), get_arg(1))")


def test_allows_rust_typed_fields_on_get_arg() -> None:
    assert not _codes('get_field(get_arg(1), "start")')


def test_rejects_type_fields_that_klee_does_not_expose() -> None:
    assert "unsupported-type-field" in _codes(
        'get_field(get_type_at_func(0), "trait_contract_holds")'
    )
    assert not _codes('get_field(get_type_at_func(0), "size")')


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


def test_rule_semantics_accepts_explicit_memory_range_bounds() -> None:
    ast = parse_dsl(
        'get_field(get_arg(0), "start") <= '
        'get_field(memory_range(get_arg(1), 1), "start") && '
        'get_field(memory_range(get_arg(1), 1), "end") <= '
        'get_field(get_arg(0), "end")',
        OPERATORS,
    )
    issues = lint_rule_semantics(
        "The memory range must be contained within a single allocated object", ast
    )
    assert "missing-within" not in {issue.code for issue in issues}


def test_rule_semantics_rejects_null_exemption_on_alignment_rule() -> None:
    ast = parse_dsl(
        'get_field(get_arg(0), "is_null") || get_arg(0) != 0', OPERATORS
    )
    issues = lint_rule_semantics("The pointer must be aligned", ast)
    assert "extraneous-null-exemption" in {issue.code for issue in issues}
