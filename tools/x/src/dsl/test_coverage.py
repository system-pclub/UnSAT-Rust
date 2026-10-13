import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from dsl import (  # noqa: E402
    BinaryExpression,
    CollectionExpression,
    QuantifiedExpression,
    TemporalExpression,
    list_operators,
    parse_dsl,
)
from dsl.coverage import FAMILY_WITNESSES, audit_rules  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parents[4]
OPERATORS_PATH = REPO_ROOT / "operators.json"


def test_quantified_byte_constraint_uses_existing_operators_only() -> None:
    ast = parse_dsl(
        'forall b in get_field(get_arg(0), "bytes"): b >= 0 && b <= 127',
        OPERATORS_PATH,
    )

    assert isinstance(ast, QuantifiedExpression)
    assert list_operators(ast) == ["get_arg", "get_field"]


def test_temporal_protocol_constraint_uses_existing_operators_only() -> None:
    ast = parse_dsl(
        'always (get_field(get_arg(0), "alive") => '
        'eventually get_field(get_arg(0), "dropped"))',
        OPERATORS_PATH,
    )

    assert isinstance(ast, TemporalExpression)
    assert list_operators(ast) == ["get_arg", "get_field"]


def test_membership_and_collection_literals() -> None:
    ast = parse_dsl("get_arg(0) in [0, 1, 2]", OPERATORS_PATH)

    assert isinstance(ast, BinaryExpression)
    assert ast.operator == "in"
    assert isinstance(ast.right, CollectionExpression)


def test_operator_budget_does_not_grow() -> None:
    document = json.loads(OPERATORS_PATH.read_text(encoding="utf-8"))

    assert len(document["operators"]) <= 17


def test_each_counted_rule_family_has_a_parseable_witness() -> None:
    for source in FAMILY_WITNESSES.values():
        parse_dsl(source, OPERATORS_PATH)


def test_std_core_grammar_coverage_exceeds_eighty_percent() -> None:
    result = audit_rules(REPO_ROOT / "rules.csv")

    assert result.total == 948
    assert result.ratio >= 0.80
    assert result.by_library["core"][1] == 698
    assert result.by_library["std"][1] == 250
    for supported, total in result.by_library.values():
        assert supported / total >= 0.80
