from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping, Sequence

from dsl.ast import (
    BinaryExpression,
    CallExpression,
    CollectionExpression,
    Expression,
    Literal,
    QuantifiedExpression,
    TemporalExpression,
    UnaryExpression,
)


@dataclass(frozen=True)
class DSLLintIssue:
    severity: str
    code: str
    message: str


SUPPORTED_ACCESS_KINDS = {
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
_ISIZE_FIT_PHRASES = (
    "fit in an `isize`",
    "fit in an isize",
    "fit in `isize`",
    "overflow `isize`",
    "overflow isize",
    "no larger than `isize::max`",
    "more than `isize::max`",
    "within `isize::max`",
    "located within `isize::max`",
)


def _call_names(node: Expression) -> set[str]:
    names: set[str] = set()
    if isinstance(node, CallExpression):
        names.add(node.name)
        for arg in node.args:
            names.update(_call_names(arg))
    elif isinstance(node, BinaryExpression):
        names.update(_call_names(node.left))
        names.update(_call_names(node.right))
    elif isinstance(node, UnaryExpression):
        names.update(_call_names(node.operand))
    elif isinstance(node, CollectionExpression):
        for item in node.items:
            names.update(_call_names(item))
    elif isinstance(node, QuantifiedExpression):
        names.update(_call_names(node.domain))
        names.update(_call_names(node.predicate))
    elif isinstance(node, TemporalExpression):
        names.update(_call_names(node.operand))
    return names


def _access_kinds(node: Expression) -> set[str]:
    kinds: set[str] = set()
    if isinstance(node, CallExpression):
        if node.name == "is_valid_for" and len(node.args) == 2:
            access = node.args[1]
            if isinstance(access, Literal) and isinstance(access.value, str):
                kinds.add(access.value)
        for arg in node.args:
            kinds.update(_access_kinds(arg))
    elif isinstance(node, BinaryExpression):
        kinds.update(_access_kinds(node.left))
        kinds.update(_access_kinds(node.right))
    elif isinstance(node, UnaryExpression):
        kinds.update(_access_kinds(node.operand))
    elif isinstance(node, CollectionExpression):
        for item in node.items:
            kinds.update(_access_kinds(item))
    elif isinstance(node, QuantifiedExpression):
        kinds.update(_access_kinds(node.domain))
        kinds.update(_access_kinds(node.predicate))
    elif isinstance(node, TemporalExpression):
        kinds.update(_access_kinds(node.operand))
    return kinds


def _field_names(node: Expression) -> set[str]:
    fields: set[str] = set()
    if isinstance(node, CallExpression):
        if node.name == "get_field" and len(node.args) == 2:
            field = node.args[1]
            if isinstance(field, Literal) and isinstance(field.value, str):
                fields.add(field.value)
        for arg in node.args:
            fields.update(_field_names(arg))
    elif isinstance(node, BinaryExpression):
        fields.update(_field_names(node.left))
        fields.update(_field_names(node.right))
    elif isinstance(node, UnaryExpression):
        fields.update(_field_names(node.operand))
    elif isinstance(node, CollectionExpression):
        for item in node.items:
            fields.update(_field_names(item))
    elif isinstance(node, QuantifiedExpression):
        fields.update(_field_names(node.domain))
        fields.update(_field_names(node.predicate))
    elif isinstance(node, TemporalExpression):
        fields.update(_field_names(node.operand))
    return fields


def _unsupported_type_fields(node: Expression) -> set[str]:
    unsupported: set[str] = set()
    if isinstance(node, CallExpression):
        if node.name == "get_field" and len(node.args) == 2:
            base, field = node.args
            if (
                isinstance(base, CallExpression)
                and base.name == "get_type_at_func"
                and isinstance(field, Literal)
                and isinstance(field.value, str)
                and field.value
                not in {"size", "align", "is_zst", "allows_uninitialized"}
            ):
                unsupported.add(field.value)
        for arg in node.args:
            unsupported.update(_unsupported_type_fields(arg))
    elif isinstance(node, BinaryExpression):
        unsupported.update(_unsupported_type_fields(node.left))
        unsupported.update(_unsupported_type_fields(node.right))
    elif isinstance(node, UnaryExpression):
        unsupported.update(_unsupported_type_fields(node.operand))
    elif isinstance(node, CollectionExpression):
        for item in node.items:
            unsupported.update(_unsupported_type_fields(item))
    elif isinstance(node, QuantifiedExpression):
        unsupported.update(_unsupported_type_fields(node.domain))
        unsupported.update(_unsupported_type_fields(node.predicate))
    elif isinstance(node, TemporalExpression):
        unsupported.update(_unsupported_type_fields(node.operand))
    return unsupported


def lint_rule_semantics(rule: str, ast: Expression) -> list[DSLLintIssue]:
    """Check high-confidence one-rule/one-condition correspondences."""
    text = " ".join(rule.lower().split())
    calls = _call_names(ast)
    access_kinds = _access_kinds(ast)
    fields = _field_names(ast)
    issues: list[DSLLintIssue] = []
    expects_isize_fit = any(phrase in text for phrase in _ISIZE_FIT_PHRASES)
    contextual_summary = "all of the following" in text

    if "mul_fits" in calls and not (expects_isize_fit or contextual_summary):
        issues.append(
            DSLLintIssue(
                "error",
                "extraneous-mul-fits",
                "mul_fits adds an overflow condition that is not stated by this rule",
            )
        )
    if expects_isize_fit and "mul_fits" not in calls:
        issues.append(
            DSLLintIssue(
                "error",
                "missing-mul-fits",
                "an isize fit/overflow rule must be expressed with mul_fits",
            )
        )
    if (
        expects_isize_fit
        and "wrap around" not in text
        and isinstance(ast, BinaryExpression)
        and ast.operator == "&&"
    ):
        issues.append(
            DSLLintIssue(
                "error",
                "extra-fit-condition",
                "a pure integer-fit rule must not be conjoined with another condition",
            )
        )

    allocation_bounds = any(
        phrase in text
        for phrase in (
            "contained within a single allocated object",
            "in-bounds of the underlying allocated object",
            "span a single allocation",
        )
    )
    explicit_range_bounds = (
        {"start", "end"}.issubset(fields)
        and bool({"memory_range", "memory_between"} & calls)
    )
    if (
        allocation_bounds
        and "within" not in calls
        and not explicit_range_bounds
        and "cstr" not in text
    ):
        issues.append(
            DSLLintIssue(
                "error",
                "missing-within",
                "an allocation-containment rule must compare MemoryRange start/end bounds",
            )
        )
    if (
        ("must be aligned" in text or "must be properly aligned" in text)
        and "null" not in text
        and "is_null" in fields
    ):
        issues.append(
            DSLLintIssue(
                "error",
                "extraneous-null-exemption",
                "this alignment rule does not state a null exemption",
            )
        )

    if "must not wrap around" in text or "must not \"wrap around\"" in text:
        if "offset_does_not_wrap" not in calls:
            issues.append(
                DSLLintIssue(
                    "error",
                    "missing-address-wrap-check",
                    "an address-space wrap rule must use offset_does_not_wrap",
                )
            )

    if "valid for reads and writes" in text:
        if "read_write" not in access_kinds:
            issues.append(
                DSLLintIssue(
                    "error", "wrong-access-kind", "this rule requires read_write"
                )
            )
    elif "valid for reads" in text:
        if "read" not in access_kinds:
            issues.append(
                DSLLintIssue("error", "wrong-access-kind", "this rule requires read")
            )
    elif "valid for writes" in text:
        if not ({"write", "read_write"} & access_kinds):
            issues.append(
                DSLLintIssue("error", "wrong-access-kind", "this rule requires write")
            )

    if "complete memory range of this `cstr`" in text:
        # Kept for wording variants in downstream rule sets.
        if "c_str_in_allocation" not in calls:
            issues.append(
                DSLLintIssue(
                    "error", "wrong-cstr-predicate", "use c_str_in_allocation"
                )
            )
    if "entire memory range of this `cstr`" in text and "c_str_in_allocation" not in calls:
        issues.append(
            DSLLintIssue(
                "error", "wrong-cstr-predicate", "use c_str_in_allocation"
            )
        )
    if "valid nul terminator" in text and "has_nul_terminator" not in calls:
        issues.append(
            DSLLintIssue(
                "error", "wrong-cstr-predicate", "use has_nul_terminator"
            )
        )

    return issues


def lint_dsl_ast(
    ast: Expression, operators: Sequence[Mapping[str, object]]
) -> list[DSLLintIssue]:
    signatures: dict[str, int] = {}
    for entry in operators:
        name = entry.get("name")
        inputs = entry.get("input")
        if isinstance(name, str) and isinstance(inputs, list):
            signatures[name] = len(inputs)

    issues: list[DSLLintIssue] = []

    for field in sorted(_unsupported_type_fields(ast)):
        issues.append(
            DSLLintIssue(
                "error",
                "unsupported-type-field",
                f"KLEE type metadata does not expose field {field!r}",
            )
        )

    def add(severity: str, code: str, message: str) -> None:
        issues.append(DSLLintIssue(severity, code, message))

    def visit(
        node: Expression,
        *,
        parent_call: str | None = None,
    ) -> None:
        if isinstance(node, CallExpression):
            expected = signatures.get(node.name)
            if expected is None:
                add("error", "unknown-operator", f"unknown operator {node.name!r}")
            elif len(node.args) != expected:
                add(
                    "error",
                    "operator-arity",
                    f"{node.name} expects {expected} arguments, got {len(node.args)}",
                )

            if node.name == "get_field" and len(node.args) == 2:
                base, field = node.args
                if not isinstance(field, Literal) or not isinstance(field.value, str):
                    add(
                        "error",
                        "field-name-literal",
                        "get_field field name must be a string literal",
                    )
                elif isinstance(base, CallExpression) and base.name == "get_arg":
                    # get_arg is intentionally typed as Value in operators.json.
                    # Its concrete Rust type is available only at the callsite,
                    # so fields such as slice.len and SliceIndex.start/end cannot
                    # be checked by this source-independent linter yet.
                    pass

            if node.name == "is_valid_for" and len(node.args) == 2:
                access = node.args[1]
                if not isinstance(access, Literal) or not isinstance(access.value, str):
                    add(
                        "error",
                        "access-kind-literal",
                        "is_valid_for access kind must be a string literal",
                    )
                elif access.value.startswith("rule-"):
                    add(
                        "error",
                        "placeholder-access-kind",
                        f"placeholder access kind {access.value!r} has no semantics",
                    )
                elif access.value not in SUPPORTED_ACCESS_KINDS:
                    add(
                        "warning",
                        "unsupported-access-kind",
                        f"is_valid_for access kind {access.value!r} is not translated by KLEE",
                    )

            if node.name == "mul_fits" and len(node.args) == 3:
                target = node.args[2]
                if (
                    not isinstance(target, Literal)
                    or target.value
                    not in {"isize", "usize", "i32", "u32", "i64", "u64"}
                ):
                    add(
                        "error",
                        "mul-fits-target",
                        "mul_fits target must be one of isize, usize, i32, u32, i64, or u64",
                    )

            if node.name == "as_signed" and len(node.args) == 2:
                target = node.args[1]
                if (
                    not isinstance(target, Literal)
                    or target.value not in {"isize", "i32", "i64"}
                ):
                    add(
                        "error",
                        "as-signed-target",
                        "as_signed type must be one of isize, i32, or i64",
                    )

            if node.name == "offset_does_not_wrap" and len(node.args) == 4:
                direction = node.args[3]
                if (
                    not isinstance(direction, Literal)
                    or direction.value not in {"signed", "add", "sub"}
                ):
                    add(
                        "error",
                        "offset-direction",
                        "offset_does_not_wrap direction must be signed, add, or sub",
                    )

            for arg in node.args:
                visit(arg, parent_call=node.name)
            return

        if isinstance(node, BinaryExpression):
            visit(node.left, parent_call=parent_call)
            visit(node.right, parent_call=parent_call)
            return

        if isinstance(node, UnaryExpression):
            visit(node.operand, parent_call=parent_call)
            return

        if isinstance(node, CollectionExpression):
            for item in node.items:
                visit(item, parent_call=parent_call)
            return

        if isinstance(node, QuantifiedExpression):
            visit(node.domain, parent_call=parent_call)
            visit(node.predicate, parent_call=parent_call)
            return

        if isinstance(node, TemporalExpression):
            visit(node.operand, parent_call=parent_call)

    visit(ast)
    return issues
