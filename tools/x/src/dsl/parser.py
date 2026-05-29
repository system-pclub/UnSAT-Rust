from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence

from dsl.errors import DSLParseError, DSLValidationError


@dataclass(frozen=True)
class Literal:
    value: int | bool | None


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

_BINARY_PRECEDENCE = {
    "||": 1,
    "&&": 2,
    "==": 3,
    "!=": 3,
    "<": 4,
    "<=": 4,
    ">": 4,
    ">=": 4,
    "+": 5,
    "-": 5,
    "*": 6,
    "/": 6,
    "%": 6,
}
_UNARY_OPERATORS = {"!", "-"}
_MULTI_CHAR_OPERATORS = ("&&", "||", "==", "!=", "<=", ">=")


@dataclass(frozen=True)
class _Token:
    kind: str
    value: str
    line: int
    column: int


class _Lexer:
    def __init__(self, text: str):
        self._text = text
        self._index = 0
        self._line = 1
        self._column = 1

    def tokenize(self) -> list[_Token]:
        tokens: list[_Token] = []
        while self._index < len(self._text):
            char = self._text[self._index]
            if char in " \t\r":
                self._advance(char)
                continue
            if char == "\n":
                self._advance(char)
                continue

            token_line = self._line
            token_column = self._column

            matched_operator = next(
                (operator for operator in _MULTI_CHAR_OPERATORS if self._text.startswith(operator, self._index)),
                None,
            )
            if matched_operator is not None:
                self._advance_many(matched_operator)
                tokens.append(_Token("OP", matched_operator, token_line, token_column))
                continue

            if char in "()+-*/%,!<>@:":
                self._advance(char)
                token_kind = "OP" if char in "+-*/%!<>" else char
                tokens.append(_Token(token_kind, char, token_line, token_column))
                continue

            if char.isdigit():
                start = self._index
                while self._index < len(self._text) and self._text[self._index].isdigit():
                    self._advance(self._text[self._index])
                tokens.append(_Token("INT", self._text[start:self._index], token_line, token_column))
                continue

            if char.isalpha() or char == "_":
                start = self._index
                while self._index < len(self._text) and (
                    self._text[self._index].isalnum() or self._text[self._index] == "_"
                ):
                    self._advance(self._text[self._index])
                tokens.append(_Token("IDENT", self._text[start:self._index], token_line, token_column))
                continue

            raise DSLParseError(f"Unexpected character {char!r}", token_line, token_column)

        tokens.append(_Token("EOF", "", self._line, self._column))
        return tokens

    def _advance_many(self, text: str) -> None:
        for char in text:
            self._advance(char)

    def _advance(self, char: str) -> None:
        self._index += 1
        if char == "\n":
            self._line += 1
            self._column = 1
        else:
            self._column += 1


class _Parser:
    def __init__(self, text: str, allowed_operators: set[str], *, allow_unknown_operators: bool):
        self._tokens = _Lexer(text).tokenize()
        self._index = 0
        self._allowed_operators = allowed_operators
        self._allow_unknown_operators = allow_unknown_operators

    def parse(self) -> Expression:
        expression = self._parse_expression(1)
        if self._peek().kind != "EOF":
            token = self._peek()
            raise DSLParseError(f"Unexpected token {token.value!r}", token.line, token.column)
        return expression

    def _parse_expression(self, min_precedence: int) -> Expression:
        left = self._parse_unary()

        while True:
            token = self._peek()
            if token.kind != "OP":
                break
            precedence = _BINARY_PRECEDENCE.get(token.value)
            if precedence is None or precedence < min_precedence:
                break

            operator = self._consume("OP").value
            right = self._parse_expression(precedence + 1)
            left = BinaryExpression(operator=operator, left=left, right=right)

        return left

    def _parse_unary(self) -> Expression:
        token = self._peek()
        if token.kind == "OP" and token.value in _UNARY_OPERATORS:
            operator = self._consume("OP").value
            operand = self._parse_unary()
            return UnaryExpression(operator=operator, operand=operand)
        return self._parse_primary()

    def _parse_primary(self) -> Expression:
        token = self._peek()

        if token.kind == "INT":
            value = int(self._consume("INT").value)
            return Literal(value=value)

        if token.kind == "IDENT":
            identifier = self._consume("IDENT")
            if identifier.value == "None":
                return Literal(value=None)
            if identifier.value in {"true", "false", "True", "False"}:
                return Literal(value=identifier.value.lower() == "true")

            if self._match("("):
                args = self._parse_call_args()
                if (
                    identifier.value not in self._allowed_operators
                    and not self._allow_unknown_operators
                ):
                    raise DSLValidationError(
                        f"Unknown operator {identifier.value!r}. Allowed operators: {sorted(self._allowed_operators)}"
                    )
                return CallExpression(name=identifier.value, args=tuple(args))

            if self._match("@"):
                line = self._parse_positive_int("source line")
                self._expect(":")
                column = self._parse_positive_int("source column")
                return SourceRef(name=identifier.value, line=line, column=column)

            return Identifier(name=identifier.value)

        if self._match("("):
            expression = self._parse_expression(1)
            self._expect(")")
            return expression

        found = repr(token.value) if token.value else "end of input"
        raise DSLParseError(
            f"Expected expression, found {found}",
            token.line,
            token.column,
        )

    def _parse_call_args(self) -> list[Expression]:
        args: list[Expression] = []
        if self._match(")"):
            return args

        while True:
            args.append(self._parse_expression(1))
            if self._match(")"):
                return args
            self._expect(",")

    def _parse_positive_int(self, label: str) -> int:
        token = self._peek()
        if token.kind != "INT":
            raise DSLParseError(f"Expected {label}", token.line, token.column)
        value = int(self._consume("INT").value)
        return value

    def _peek(self) -> _Token:
        return self._tokens[self._index]

    def _consume(self, kind: str) -> _Token:
        token = self._peek()
        if token.kind != kind:
            raise DSLParseError(f"Expected {kind}", token.line, token.column)
        self._index += 1
        return token

    def _expect(self, symbol: str) -> None:
        token = self._peek()
        if token.value != symbol:
            raise DSLParseError(f"Expected {symbol!r}", token.line, token.column)
        self._index += 1

    def _match(self, symbol: str) -> bool:
        token = self._peek()
        if token.value != symbol:
            return False
        self._index += 1
        return True


def parse_dsl(
    source: str,
    operator_config: str | Path | Sequence[str] | Sequence[Mapping[str, Any]],
    *,
    allow_unknown_operators: bool = False,
) -> Expression:
    allowed_operators = _resolve_operator_names(operator_config)
    parser = _Parser(source, allowed_operators, allow_unknown_operators=allow_unknown_operators)
    return parser.parse()


def validate_task1_ast(ast: Expression, required_operators: Iterable[str]) -> None:
    used_operators = set(list_operators(ast))
    if not used_operators:
        raise DSLValidationError("Task1 AST must contain at least one configured operator")

    missing = [name for name in required_operators if name not in used_operators]
    if missing:
        raise DSLValidationError(f"Task1 AST is missing required operators: {missing}")


def validate_task2_ast(ast: Expression) -> None:
    used_operators = list_operators(ast)
    if used_operators:
        raise DSLValidationError(f"Task2 AST must not contain configured operators: {used_operators}")


def list_operators(ast: Expression) -> list[str]:
    names = sorted(_collect_operator_names(ast))
    return names


def _collect_operator_names(ast: Expression) -> set[str]:
    if isinstance(ast, CallExpression):
        names = {ast.name}
        for arg in ast.args:
            names.update(_collect_operator_names(arg))
        return names
    if isinstance(ast, UnaryExpression):
        return _collect_operator_names(ast.operand)
    if isinstance(ast, BinaryExpression):
        names = _collect_operator_names(ast.left)
        names.update(_collect_operator_names(ast.right))
        return names
    return set()


def _resolve_operator_names(
    operator_config: str | Path | Mapping[str, Any] | Sequence[str] | Sequence[Mapping[str, Any]],
) -> set[str]:
    if isinstance(operator_config, Path):
        data = json.loads(operator_config.read_text(encoding="utf-8"))
        return _extract_operator_names(data)

    if isinstance(operator_config, str):
        candidate_path = Path(operator_config)
        if candidate_path.is_file():
            data = json.loads(candidate_path.read_text(encoding="utf-8"))
            return _extract_operator_names(data)
        stripped = operator_config.strip()
        if stripped.startswith("[") or stripped.startswith("{"):
            return _extract_operator_names(json.loads(stripped))
        raise DSLValidationError("Operator config string must be a file path or JSON payload")

    return _extract_operator_names(operator_config)


def _extract_operator_names(data: Any) -> set[str]:
    if isinstance(data, Mapping):
        operators = data.get("operators")
        if operators is None:
            raise DSLValidationError("Operator config object must contain an 'operators' array")
        data = operators

    if not isinstance(data, Sequence) or isinstance(data, (str, bytes, bytearray)):
        raise DSLValidationError("Operator config must be a JSON array of operator entries")

    names: set[str] = set()
    for item in data:
        if isinstance(item, str):
            name = item
        elif isinstance(item, Mapping):
            name = item.get("name")
        else:
            raise DSLValidationError("Operator entries must be strings or objects with a name field")

        if not isinstance(name, str) or not name:
            raise DSLValidationError(f"Invalid operator entry: {item!r}")
        names.add(name)

    return names