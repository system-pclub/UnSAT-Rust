from __future__ import annotations

import json
from pathlib import Path

from cli.cmd.verify import (
    _assign_matrix_rules_to_callsites,
    _load_rule_ids_by_callee,
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


def test_unreachable_false_rule_reaches_klee_as_a_boolean_literal() -> None:
    operators = json.loads(
        (REPO_ROOT / "operators.json").read_text(encoding="utf-8")
    )["operators"]
    payload = json.loads(_task1_to_ext_ast_json("false", operators))

    assert payload == {
        "simplified": {"type": "literal", "value": False},
        "original": {"type": "literal", "value": False},
    }


def test_memory_copy_counts_use_source_api_parameter_positions() -> None:
    rules = json.loads((REPO_ROOT / "ptr_rule_dsl.json").read_text())["rules"]
    # write_bytes(self, val, count) and copy_nonoverlapping(src, dst, count).
    assert "get_arg(2)" in rules["rule-950"]["dsl"]
    assert "get_arg(1)" not in rules["rule-950"]["dsl"]
    for rule in ("rule-958", "rule-959", "rule-960"):
        assert "get_arg(2)" in rules[rule]["dsl"]
        assert "get_arg(4)" not in rules[rule]["dsl"]
