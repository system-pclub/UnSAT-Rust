from __future__ import annotations

import argparse
import csv
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any

from dsl.coverage import classify_rule


TYPE_SIZE = 'get_field(get_type_at_func(0), "size")'
TYPE_ALIGN = 'get_field(get_type_at_func(0), "align")'


def _norm(text: str) -> str:
    return " ".join(text.split())


def _field(name: str, arg: int = 0) -> str:
    return f'get_field(get_arg({arg}), "{name}")'


def _aligned(arg: int = 0) -> str:
    return (
        f'(({TYPE_ALIGN} != 0) && '
        f'((get_field(get_arg({arg}), "address") % {TYPE_ALIGN}) == 0))'
    )


def _nonnull(arg: int = 0) -> str:
    return f'!get_field(get_arg({arg}), "is_null")'


def _size_expr(text: str, name: str) -> str:
    lower = text.lower()
    if "nul terminator" in lower or "null terminator" in lower:
        return _field("c_str_len")
    if "len * size_of" in lower or "len` consecutive" in lower or "len consecutive" in lower:
        return f'(get_arg(1) * {TYPE_SIZE})'
    if "count * size_of" in lower:
        count_arg = 4 if "copy_nonoverlapping" in name else 2
        return f'(get_arg({count_arg}) * {TYPE_SIZE})'
    if "size_of::<dst>" in lower:
        return 'get_field(get_type_at_func(1), "size")'
    if "size_of_val" in lower or "align_of_val" in lower:
        return _field("dynamic_size")
    if re.search(r"\bsize\b.*(?:many )?bytes", lower):
        return "get_arg(1)"
    if "length `len`" in lower or "length of `len`" in lower:
        return "get_arg(1)"
    return TYPE_SIZE


def _range(size: str, arg: int = 0) -> str:
    return f"memory_range(get_arg({arg}), {size})"


def _valid(access: str, size: str, arg: int = 0) -> str:
    return f'is_valid_for({_range(size, arg)}, "{access}")'


def _initialized(size: str, arg: int = 0) -> str:
    return f"is_initialized_for_type({_range(size, arg)}, get_type_at_func(0))"


def _within_allocation(size: str, arg: int = 0) -> str:
    memory = _range(size, arg)
    allocation = f'get_field(get_arg({arg}), "allocation")'
    return (
        f'((get_field({allocation}, "start") <= get_field({memory}, "start")) && '
        f'(get_field({memory}, "start") <= get_field({memory}, "end")) && '
        f'(get_field({memory}, "end") <= get_field({allocation}, "end")))'
    )


def _disjoint(size: str) -> str:
    left = _range(size, 0)
    right = _range(size, 1)
    return (
        f'((get_field({left}, "end") <= get_field({right}, "start")) || '
        f'(get_field({right}, "end") <= get_field({left}, "start")))'
    )


def _state(name: str, arg: int = 0) -> str:
    return _field(name, arg)


def _translate_rule(row: dict[str, str], family: str) -> str:
    text = _norm(row["rule"])
    lower = text.lower()
    name = row["name"].lower()
    size = _size_expr(text, name)

    if lower == "marker" or "always safe" in lower or "no additional safety" in lower:
        return "true"
    if "must never be reached" in lower or "reaching this function is undefined" in lower:
        return "false"

    if "valid nul terminator" in lower or "contains a nul terminator" in lower or "null-terminated" in lower:
        return "has_nul_terminator(get_arg(0))"
    if "entire memory range of this `cstr`" in lower or "complete memory range of this `cstr`" in lower:
        return "c_str_in_allocation(get_arg(0))"
    if ("nul" in lower or "null" in lower) and "isize::max" in lower:
        return f'mul_fits({_field("c_str_len")}, 1, "isize")'
    if "nul terminator" in lower and "valid for reads" in lower:
        return _valid("read", _field("c_str_len"))

    if "non-null" in lower and ("aligned" in lower or "alignment" in lower):
        return f"({_nonnull()} && {_aligned()})"
    if "must be non-null" in lower or "must not be null" in lower:
        return _nonnull()
    if "must be properly aligned" in lower or "must be aligned" in lower or "alignment of `t`" in lower:
        return _aligned()

    if "valid for both reads and writes" in lower or "valid for reading and writing" in lower:
        return _valid("read_write", size)
    if "valid for writes" in lower or "valid for writing" in lower:
        return _valid("write", size)
    if "valid for reads" in lower or "valid for reading" in lower:
        return _valid("read", size)
    if "valid for dropping" in lower:
        return _valid("drop", size)

    if (
        "must all be initialized" in lower
        or "must be initialized" in lower
        or "initialized state" in lower
        or "fully initialized" in lower
        or "valid value of type" in lower
        or "contains uninitialized" in lower
        or "point to initialized" in lower
        or "consecutive properly initialized" in lower
    ):
        return _initialized(size)

    if (
        "contained within a single allocated object" in lower
        or "in-bounds of the underlying allocated object" in lower
        or "span a single allocation" in lower
        or "range must be in-bounds" in lower
        or "range must be in bounds" in lower
        or "range points to the correct length" in lower
    ):
        return _within_allocation(size)

    if "must not wrap around" in lower or "must not \"wrap around\"" in lower:
        direction = "sub" if "sub" in name else "signed" if "offset" in name else "add"
        return f'offset_does_not_wrap(get_arg(0), get_arg(1), {TYPE_SIZE}, "{direction}")'

    if (
        "fit in an `isize`" in lower
        or "fit in an isize" in lower
        or "fit in `isize`" in lower
        or "overflow `isize`" in lower
        or "overflow isize" in lower
        or "no larger than `isize::max`" in lower
        or "more than `isize::max`" in lower
        or "within `isize::max`" in lower
    ):
        if "size of the entire value" in lower:
            value, factor = _field("dynamic_size"), "1"
        else:
            factor = TYPE_SIZE if any(word in lower for word in ("slice", "value", "size_of", "element")) else "1"
            value = "get_arg(1)" if any(word in lower for word in ("len", "length", "count", "offset", "size")) else _field("dynamic_size")
        return f'mul_fits({value}, {factor}, "isize")'

    if "must not overlap" in lower or "disjoint" in lower or "overlapping indices" in lower or "overlapping keys" in lower:
        return _disjoint(size)
    if "must not alias" in lower or "may not alias" in lower:
        return f'(get_field(get_arg(0), "allocation") != get_field(get_arg(1), "allocation"))'

    if "same allocation" in lower or "same allocated object" in lower:
        return f'(get_field(get_arg(0), "allocation") == get_field(get_arg(1), "allocation"))'
    if "same layout that was used to allocate" in lower or "layout` must fit" in lower or "old_layout` must fit" in lower:
        return (
            '(get_field(get_field(get_arg(0), "allocation"), "size") == get_field(get_arg(1), "size") && '
            'get_field(get_field(get_field(get_arg(0), "allocation"), "layout"), "align") == get_field(get_arg(1), "align"))'
        )
    if "same layout" in lower or "same size and alignment" in lower or "identical layouts" in lower:
        return (
            '(get_field(get_type_at_func(0), "size") == get_field(get_type_at_func(1), "size") && '
            'get_field(get_type_at_func(0), "align") == get_field(get_type_at_func(1), "align"))'
        )
    if "allocated via this allocator" in lower or "freeable with" in lower:
        return '(get_field(get_allocation(get_arg(0)), "allocator") == get_arg(1))'
    if "currently allocated" in lower or "dangling" in lower or "valid pointer" in lower or "point to valid memory" in lower:
        return _valid("read", size)

    if "greater than zero" in lower or "must be non-zero" in lower or "must not be zero" in lower:
        return "(get_arg(0) != 0)"
    if "less than or equal to `capacity" in lower or "enough capacity" in lower or "no more than" in lower and "initialized" in lower:
        return f'({_field("len")} <= {_field("capacity")})'
    if "start <=" in lower and "end" in lower or "range must be canonical" in lower:
        return f'({_field("start", 1)} <= {_field("end", 1)})'
    if "end <=" in lower or "end` must not be greater" in lower:
        return f'({_field("end", 1)} <= {_field("len")})'
    if "out-of-bounds index" in lower or "index must be in-bounds" in lower or "index must be in bounds" in lower:
        return f'(get_arg(1) < {_field("len")})'
    if "mid" in lower and ("less than or equal" in lower or "in-bounds" in lower or "in bounds" in lower):
        return f'(get_arg(1) <= {_field("len")})'
    if "range must be in-bounds" in lower or "range must be in bounds" in lower:
        return f'({_field("start", 1)} <= {_field("end", 1)} && {_field("end", 1)} <= {_field("len")})'
    if "greater than or equal to" in lower:
        return f'({_field("new_size", 1)} >= {_field("old_size", 0)})'
    if "smaller than or equal to" in lower or "less than or equal to" in lower:
        return f'({_field("new_size", 1)} <= {_field("old_size", 0)})'
    if "overflow the range" in lower or "representable in the return type" in lower:
        return f'({_field("value")} >= {_field("min_value")} && {_field("value")} <= {_field("max_value")})'
    if re.search(r"\bd\s*>\s*64", lower):
        return "(get_arg(0) <= 64)"

    if "utf-8" in lower or "wtf-8" in lower:
        field = "valid_wtf8" if "wtf-8" in lower else "valid_utf8"
        if "boundary" in lower:
            field = "utf8_boundary"
        return _state(field)
    if "ascii" in lower:
        return _state("valid_ascii")
    if "code point" in lower:
        return _state("valid_code_point")

    if "not be mutated" in lower:
        return _valid("read_no_mutation", size)
    if "not be accessed through any other pointer" in lower or "only way to access" in lower:
        return _valid("unique_borrow", size)
    if "while the returned reference is alive" in lower or "while any mutable reference" in lower:
        return _state("borrow_rules_hold")
    if "not outlive" in lower or "remain valid until" in lower or "remain open for the duration" in lower or "lifetime" in lower:
        return _state("lifetime_valid")
    if "must not be moved" in lower or "pinned" in lower or "pinning invariants" in lower:
        return _state("pin_invariants_hold")
    if "must not contain references" in lower:
        return f'!{_state("type_contains_references")}'
    if "must not contain" in lower and "pointer" in lower:
        return f'!{_state("type_contains_forbidden_pointers")}'

    if "race" in lower or "concurrently" in lower or "synchronization" in lower:
        return _state("synchronized_access")
    if "current thread" in lower or "same thread" in lower or "thread that" in lower:
        return _state("owned_by_current_thread")
    if "exclusive access" in lower or "exclusively owned" in lower:
        return _state("exclusive_access")
    if "acquire ordering" in lower or "ordering or stronger" in lower:
        return _state("acquire_observed")
    if "threads queued" in lower:
        return f'({_state("queued_threads")} > 0)'

    if "must be called only once" in lower or "only be called once" in lower or "not be called more than once" in lower or "called once per" in lower:
        return f'({_state("call_count")} == 0)'
    if "must have been called" in lower or "must be called before" in lower or "prior to this call" in lower or "previously" in lower:
        return _state("required_prior_event")
    if "must be open" in lower or "valid open" in lower or "must be an owned" in lower:
        return f'({_state("is_open")} && {_state("is_owned")})'
    if "must be locked" in lower or "read-locked" in lower or "write-locked" in lower or "lock must be held" in lower:
        return _state("required_lock_held")
    if "same mutex" in lower:
        return _state("same_mutex")
    if "unwind" in lower or "panicking" in lower:
        return _state("unwind_contract_holds")
    if "thread exit" in lower or "thread destruction" in lower:
        return _state("thread_exit_phase")
    if "state" in lower or "invariants" in lower or "contract" in lower:
        return _state("state_invariants_hold")

    # A number of protocol and trait rules also mention size/index bounds.  Keep
    # their semantic subject instead of misclassifying them as plain scalars.
    if any(word in lower for word in ("after ", "before ", "will be called", "safe to call")):
        return _state("protocol_state_valid")
    if any(word in lower for word in ("trait", "implementation", "implemented", "may not be overridden")):
        return _state("trait_contract_holds")

    if family == "trait_contract":
        if "subtype" in lower or "supertype" in lower:
            return _state("subtype_contract_holds")
        if "specializ" in lower or "implemented together" in lower:
            return _state("specialization_consistent")
        return _state("trait_contract_holds")
    if family == "vacuous":
        return "true"
    if family == "encoding":
        return _state("valid_encoding")
    if family == "concurrency":
        return _state("concurrency_contract_holds")
    if family == "lifetime_aliasing":
        return _state("aliasing_and_lifetime_contract_holds")
    if family == "protocol":
        return _state("protocol_state_valid")
    if family == "type_value":
        return _state("type_value_invariants_hold")
    if family == "scalar_bounds":
        return _state("scalar_bounds_hold")
    if family == "memory":
        return _state("memory_preconditions_hold")
    raise RuntimeError(f"no translation for {row['id']}: {row['rule']}")


def translate(catalog_path: Path, dsl_path: Path) -> tuple[int, int]:
    with catalog_path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    document: dict[str, Any] = json.loads(dsl_path.read_text(encoding="utf-8"))
    rules = document.get("rules")
    if not isinstance(rules, dict):
        raise RuntimeError(f"rules object missing from {dsl_path}")

    by_text: dict[str, set[str]] = defaultdict(set)
    for row in rows:
        entry = rules.get(row["id"])
        if isinstance(entry, dict) and isinstance(entry.get("dsl"), str):
            by_text[_norm(row["rule"])].add(entry["dsl"])

    added = 0
    covered = 0
    for row in rows:
        family = classify_rule(row["rule"])
        if family is None:
            continue
        covered += 1
        if row["id"] in rules:
            continue
        exact = by_text.get(_norm(row["rule"]), set())
        dsl = next(iter(exact)) if len(exact) == 1 else _translate_rule(row, family)
        rules[row["id"]] = {
            "path": row["path"],
            "name": row["name"],
            "rule": row["rule"],
            "dsl": dsl,
        }
        added += 1

    dsl_path.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    return added, covered


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Translate grammar-coverable catalog rules")
    parser.add_argument("--rules", type=Path, default=Path("rules.csv"))
    parser.add_argument("--dsl", type=Path, default=Path("ptr_rule_dsl.json"))
    args = parser.parse_args(argv)
    added, covered = translate(args.rules, args.dsl)
    print(json.dumps({"added": added, "covered_catalog_rules": covered}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
