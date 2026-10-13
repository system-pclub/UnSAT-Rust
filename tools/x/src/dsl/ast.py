from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Literal:
    value: int | bool | str | None


@dataclass(frozen=True)
class Identifier:
    name: str


@dataclass(frozen=True)
class SourceRef:
    name: str
    line: int
    column: int


@dataclass(frozen=True)
class CallExpression:
    name: str
    args: tuple[Expression, ...]


@dataclass(frozen=True)
class UnaryExpression:
    operator: str
    operand: Expression


@dataclass(frozen=True)
class BinaryExpression:
    operator: str
    left: Expression
    right: Expression


@dataclass(frozen=True)
class CollectionExpression:
    kind: str
    items: tuple[Expression, ...]


@dataclass(frozen=True)
class QuantifiedExpression:
    quantifier: str
    variable: str
    domain: Expression
    predicate: Expression


@dataclass(frozen=True)
class TemporalExpression:
    operator: str
    operand: Expression


Expression = (
    Literal
    | Identifier
    | SourceRef
    | CallExpression
    | UnaryExpression
    | BinaryExpression
    | CollectionExpression
    | QuantifiedExpression
    | TemporalExpression
)
