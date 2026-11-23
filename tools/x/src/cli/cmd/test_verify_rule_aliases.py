from __future__ import annotations

import json
from pathlib import Path

from cli.cmd.verify import (
    _assign_matrix_rules_to_callsites,
    _klee_constraint_arg,
    _load_rule_ids_by_callee,
    _row_needs_llm_testcase,
    _rule_entry_to_ext_ast_json,
    _task1_to_ext_ast_json,
)


REPO_ROOT = Path(__file__).resolve().parents[5]


def test_rule_aliases_reuse_the_canonical_rule_id(tmp_path) -> None:
    rule_path = tmp_path / "rules.json"
    rule_path.write_text(
        json.dumps(
            {
                "rules": {
                    "rule-292": {
                        "path": "rust/library/core/src/ptr/const_ptr.rs:597",
                        "aliases": [
                            "rust/library/core/src/ptr/const_ptr.rs:637"
                        ],
                        "dsl": "true",
                    }
                }
            }
        ),
        encoding="utf-8",
    )

    by_callee = _load_rule_ids_by_callee(rule_path)

    assert by_callee[("rust/library/core/src/ptr/const_ptr.rs", 597)] == [
        "rule-292"
    ]
    assert by_callee[("rust/library/core/src/ptr/const_ptr.rs", 637)] == [
        "rule-292"
    ]


def test_unreachable_unchecked_uses_canonical_rule_89() -> None:
    callsites = [
        {
            "unsafe_callee": "std::hint::unreachable_unchecked",
            "unsafe_callee_path": "rust/library/core/src/hint.rs",
            "unsafe_callee_line_start": 9,
        }
    ]

    assigned = _assign_matrix_rules_to_callsites(
        callsites=callsites,
        rule_dsl_path=REPO_ROOT / "ptr_rule_dsl.json",
        requested_rule=None,
    )

    assert assigned[0]["rules"] == ["rule-89"]


def test_shifted_standard_library_line_falls_back_to_path_and_api_name(
    tmp_path,
) -> None:
    rule_path = tmp_path / "rules.json"
    rule_path.write_text(
        json.dumps(
            {
                "rules": {
                    "rule-a": {
                        "path": "rust/library/core/src/slice/mod.rs:625",
                        "name": "unsafe fn get_unchecked < I >",
                        "dsl": "true",
                    },
                    "rule-b": {
                        "path": "rust/library/core/src/slice/mod.rs:665",
                        "name": "unsafe fn get_unchecked_mut < I >",
                        "dsl": "true",
                    },
                }
            }
        ),
        encoding="utf-8",
    )
    callsites = [
        {
            "unsafe_callee": "core::slice::<impl [T]>::get_unchecked",
            "unsafe_callee_path": "rust/library/core/src/slice/mod.rs",
            "unsafe_callee_line_start": 655,
        }
    ]

    assigned = _assign_matrix_rules_to_callsites(
        callsites=callsites,
        rule_dsl_path=rule_path,
        requested_rule=None,
    )

    assert assigned[0]["rules"] == ["rule-a"]


def test_low_confidence_sat_is_sent_to_validation() -> None:
    assert _row_needs_llm_testcase({"status": "low-confidence-sat"})


def test_unreachable_false_rule_reaches_klee_as_a_boolean_literal() -> None:
    operators = json.loads(
        (REPO_ROOT / "operators.json").read_text(encoding="utf-8")
    )["operators"]
    payload = json.loads(_task1_to_ext_ast_json("false", operators))

    assert payload == {
        "simplified": {"type": "literal", "value": False},
        "original": {"type": "literal", "value": False},
    }


def test_smtlib_rule_keeps_formula_and_compiles_selector_dsl() -> None:
    operators = json.loads(
        (REPO_ROOT / "operators.json").read_text(encoding="utf-8")
    )["operators"]
    payload_text = _rule_entry_to_ext_ast_json(
        {
            "smtlib": (
                "(set-logic ALL)\n"
                "(declare-const count Int)\n"
                "(declare-const capacity Int)\n"
                "(assert (<= count capacity))\n"
                "(check-sat)"
            ),
            "selectors": {
                "count": "get_arg(1)",
                "capacity": 'get_field(get_arg(0), "capacity")',
            },
        },
        operators,
    )
    payload = json.loads(payload_text)

    assert payload["smtlib"].startswith("(set-logic ALL)")
    assert payload["selectors"]["count"]["name"] == "get_arg"
    assert payload["selectors"]["capacity"]["name"] == "get_field"
    assert _klee_constraint_arg(payload_text).startswith("--ext.smtlib=")


def test_memory_copy_counts_use_source_api_parameter_positions() -> None:
    rules = json.loads((REPO_ROOT / "ptr_rule_dsl.json").read_text())["rules"]
    # write_bytes(self, val, count) and copy_nonoverlapping(src, dst, count).
    assert "get_arg(2)" in rules["rule-950"]["dsl"]
    assert "get_arg(1)" not in rules["rule-950"]["dsl"]
    for rule in ("rule-958", "rule-959", "rule-960"):
        assert "get_arg(2)" in rules[rule]["dsl"]
        assert "get_arg(4)" not in rules[rule]["dsl"]


def test_matrix_rules_select_the_callee_instance(tmp_path) -> None:
    rule_path = tmp_path / "rules.json"
    rule_path.write_text(
        json.dumps(
            {
                "rules": {
                    "usize": {
                        "path": "rust/library/core/src/slice/mod.rs:625",
                        "name": "unsafe fn get_unchecked (& self , index : usize) -> & T",
                        "instance": {"I": "usize"},
                    },
                    "range": {
                        "path": "rust/library/core/src/slice/mod.rs:625",
                        "name": "unsafe fn get_unchecked (& self , index : core::ops::range::Range < usize >) -> & [T]",
                        "instance": {"I": "core::ops::range::Range<usize>"},
                    },
                    "copy": {
                        "path": "rust/library/core/src/ptr/mod.rs:407",
                        "name": "const unsafe fn copy < T > (src : * const T , dst : * mut T , count : usize)",
                        "aliases": ["rust/library/core/src/intrinsics/mod.rs:3827"],
                    },
                },
                "include_raw_pointer_deref": False,
            }
        ),
        encoding="utf-8",
    )

    def row(name, path, line, ty=None):
        return {
            "unsafe_callee": name, "unsafe_callee_path": path, "unsafe_callee_line_start": line,
            "callee_type_args": [{"name": "I", "instantiated_ty": ty}] if ty else [],
        }

    slice_path = "rust/library/core/src/slice/mod.rs"
    rows = _assign_matrix_rules_to_callsites(
        callsites=[
            row("core::slice::<impl [T]>::get_unchecked", slice_path, 655, "usize"),
            row("core::slice::<impl [T]>::get_unchecked", slice_path, 655, "std::ops::Range<usize>"),
            row("core::slice::<impl [T]>::get_unchecked", slice_path, 655, "std::ops::RangeFrom<usize>"),
            row("std::intrinsics::copy", "rust/library/core/src/intrinsics/mod.rs", 3830),
        ],
        rule_dsl_path=rule_path,
        requested_rule=None,
    )

    assert [r["rules"] for r in rows] == [["usize"], ["range"], [], ["copy"]]


def test_unconditional_constant_violation_is_certain() -> None:
    from cli.cmd.verify import _has_unconditional_violation

    unconditional = (
        "KLEE: [ext.dsl] path constraints: count=0\n"
        "KLEE: [ext.dsl] resolved constraint is constant: width=1 value=false\n"
    )
    assert _has_unconditional_violation(unconditional)
    # A false result reached through caller-controlled path constraints is certain.
    certain_path = (
        "KLEE: [ext.dsl] path constraints: count=0\n"
        "KLEE: [ext.dsl] resolved constraint is constant: width=1 value=true\n"
        "KLEE: [ext.dsl] path constraints: count=3\n"
        "KLEE: [ext.dsl] resolved constraint is constant: width=1 value=false\n"
    )
    assert _has_unconditional_violation(certain_path)
    # ...but not when the path depends on an uncertain value (logged after the constant).
    uncertain_path = certain_path + (
        "KLEE: [ext.dsl] uncertain values used by DSL: Foo.private_field\n")
    assert not _has_unconditional_violation(uncertain_path)
    assert not _has_unconditional_violation(
        unconditional + "KLEE: [ext.dsl] DSL still depends on uncertain_symbol values\n")
