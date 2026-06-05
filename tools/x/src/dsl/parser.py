import json
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence

from lark import Lark, Transformer, Token
from lark.exceptions import UnexpectedCharacters, UnexpectedToken

from dsl.ast import BinaryExpression, CallExpression, Expression, Identifier, Literal, SourceRef, UnaryExpression
from dsl.errors import DSLParseError, DSLValidationError

_GRAMMAR = r"""
?start: expr

?expr: or_expr

?or_expr: and_expr
    | or_expr "||" and_expr   -> or_expr_bin

?and_expr: equality_expr
     | and_expr "&&" equality_expr   -> and_expr_bin

?equality_expr: comparison_expr
          | equality_expr "==" comparison_expr   -> eq_expr_bin
          | equality_expr "!=" comparison_expr   -> ne_expr_bin

?comparison_expr: add_expr
        | comparison_expr "<" add_expr   -> lt_expr_bin
        | comparison_expr "<=" add_expr   -> le_expr_bin
        | comparison_expr ">" add_expr   -> gt_expr_bin
        | comparison_expr ">=" add_expr   -> ge_expr_bin

?add_expr: mul_expr
     | add_expr "+" mul_expr   -> add_expr_bin
     | add_expr "-" mul_expr   -> sub_expr_bin

?mul_expr: unary_expr
     | mul_expr "*" unary_expr   -> mul_expr_bin
     | mul_expr "/" unary_expr   -> div_expr_bin
     | mul_expr "%" unary_expr   -> mod_expr_bin

?unary_expr: "!" unary_expr   -> not_unary
       | "-" unary_expr   -> neg_unary
           | primary

?primary: call_expr
        | source_ref
        | literal
        | IDENT   -> identifier
        | "(" expr ")"

call_expr: IDENT "(" [call_args] ")"
call_args: expr ("," expr)*
source_ref: IDENT "@" INT ":" INT

?literal: INT      -> int_lit
        | BOOL     -> bool_lit
        | NONE     -> none_lit
        | STRING   -> string_lit

BOOL: "true" | "false" | "True" | "False"
NONE: "None"
IDENT: /[A-Za-z_][A-Za-z0-9_]*/

%import common.INT
%import common.ESCAPED_STRING -> STRING
%import common.WS
%ignore WS
"""


_LARK_PARSER = Lark(_GRAMMAR, parser="lalr", lexer="contextual", maybe_placeholders=False)


class _TreeToAst(Transformer[Token, Expression]):
    def __init__(self, allowed_operators: set[str], *, allow_unknown_operators: bool):
        super().__init__()
        self._allowed_operators = allowed_operators
        self._allow_unknown_operators = allow_unknown_operators

    def int_lit(self, children: list[Token]) -> Literal:
        return Literal(value=int(children[0]))

    def bool_lit(self, children: list[Token]) -> Literal:
        return Literal(value=str(children[0]).lower() == "true")

    def none_lit(self, children: list[Token]) -> Literal:
        _ = children
        return Literal(value=None)

    def string_lit(self, children: list[Token]) -> Literal:
        return Literal(value=json.loads(str(children[0])))

    def identifier(self, children: list[Token]) -> Identifier:
        return Identifier(name=str(children[0]))

    def source_ref(self, children: list[Token]) -> SourceRef:
        return SourceRef(name=str(children[0]), line=int(children[1]), column=int(children[2]))

    def call_args(self, children: list[Expression]) -> list[Expression]:
        return list(children)

    def call_expr(self, children: list[Token | list[Expression]]) -> CallExpression:
        name = str(children[0])
        args: list[Expression] = []
        if len(children) > 1 and isinstance(children[1], list):
            args = children[1]

        if name not in self._allowed_operators and not self._allow_unknown_operators:
            raise DSLValidationError(
                f"Unknown operator {name!r}. Allowed operators: {sorted(self._allowed_operators)}"
            )

        return CallExpression(name=name, args=tuple(args))

    def not_unary(self, children: list[Expression]) -> UnaryExpression:
        return UnaryExpression(operator="!", operand=children[0])

    def neg_unary(self, children: list[Expression]) -> UnaryExpression:
        return UnaryExpression(operator="-", operand=children[0])

    def _binary(self, operator: str, children: list[Expression]) -> BinaryExpression:
        return BinaryExpression(operator=operator, left=children[0], right=children[1])

    def or_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("||", children)

    def and_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("&&", children)

    def eq_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("==", children)

    def ne_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("!=", children)

    def lt_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("<", children)

    def le_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("<=", children)

    def gt_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary(">", children)

    def ge_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary(">=", children)

    def add_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("+", children)

    def sub_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("-", children)

    def mul_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("*", children)

    def div_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("/", children)

    def mod_expr_bin(self, children: list[Expression]) -> BinaryExpression:
        return self._binary("%", children)


def _to_parse_error(exc: UnexpectedCharacters | UnexpectedToken) -> DSLParseError:
    if isinstance(exc, UnexpectedCharacters):
        return DSLParseError(f"Unexpected character {exc.char!r}", exc.line, exc.column)

    token_value = str(exc.token.value) if exc.token.value is not None else ""
    found = repr(token_value) if token_value else "end of input"
    return DSLParseError(f"Expected expression, found {found}", exc.line, exc.column)


def parse_dsl(
    source: str,
    operator_config: str | Path | Sequence[str] | Sequence[Mapping[str, Any]],
    *,
    allow_unknown_operators: bool = False,
) -> Expression:
    allowed_operators = _resolve_operator_names(operator_config)
    try:
        parse_tree = _LARK_PARSER.parse(source)
    except (UnexpectedCharacters, UnexpectedToken) as exc:
        raise _to_parse_error(exc) from exc

    transformer = _TreeToAst(allowed_operators, allow_unknown_operators=allow_unknown_operators)
    return transformer.transform(parse_tree)


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