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


Expression = Literal | Identifier | SourceRef | CallExpression | UnaryExpression | BinaryExpression
