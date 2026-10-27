import json

from ablation_runner import _rule_index, _rule_name_index, _rules_for_target, _source_span


def test_rule_index_includes_version_aliases(tmp_path):
    (tmp_path / "ptr_rule_dsl.json").write_text(
        json.dumps(
            {
                "rules": {
                    "rule-1": {
                        "path": "rust/library/core/src/ptr/mod.rs:10",
                        "aliases": ["rust/library/core/src/ptr/mod.rs:20"],
                        "dsl": "true",
                    }
                }
            }
        ),
        encoding="utf-8",
    )

    index = _rule_index(tmp_path)

    assert [rule_id for rule_id, _ in index[("rust/library/core/src/ptr/mod.rs", 10)]] == ["rule-1"]
    assert [rule_id for rule_id, _ in index[("rust/library/core/src/ptr/mod.rs", 20)]] == ["rule-1"]


def test_rule_index_does_not_duplicate_primary_location_alias(tmp_path):
    path = "rust/library/core/src/ptr/mod.rs:10"
    (tmp_path / "ptr_rule_dsl.json").write_text(
        json.dumps({"rules": {"rule-1": {"path": path, "aliases": [path]}}}),
        encoding="utf-8",
    )

    index = _rule_index(tmp_path)

    assert len(index[("rust/library/core/src/ptr/mod.rs", 10)]) == 1


def test_rules_for_target_falls_back_to_api_name_after_line_drift(tmp_path):
    (tmp_path / "ptr_rule_dsl.json").write_text(
        json.dumps(
            {
                "rules": {
                    "rule-1": {
                        "path": "rust/library/core/src/slice/mod.rs:625",
                        "name": "unsafe fn get_unchecked < I >",
                        "dsl": "true",
                    }
                }
            }
        ),
        encoding="utf-8",
    )

    rules = _rules_for_target(
        callee={
            "name": "core::slice::<impl [T]>::get_unchecked",
            "path": "rust/library/core/src/slice/mod.rs",
            "line_start": 655,
        },
        by_location=_rule_index(tmp_path),
        by_name=_rule_name_index(tmp_path),
    )

    assert [rule_id for rule_id, _ in rules] == ["rule-1"]


def test_rules_for_target_includes_raw_pointer_dereference():
    rules = _rules_for_target(
        callee={"name": "core::ptr::__raw_ptr_deref__"},
        by_location={},
        by_name={},
    )

    assert [rule_id for rule_id, _ in rules] == ["__raw_ptr_deref__"]


def test_source_span_includes_type_fields_when_metadata_has_only_start_line(tmp_path):
    (tmp_path / "src").mkdir()
    (tmp_path / "src/lib.rs").write_text(
        'pub struct State {\n'
        '    /// A brace in a comment: }\n'
        '    pub index: usize,\n'
        '    data: Vec<u8>,\n'
        '}\n'
        'pub fn unrelated() {}\n',
        encoding="utf-8",
    )

    code = _source_span(
        tmp_path, {"path": "src/lib.rs", "line_start": 1, "body_end": 1}
    )

    assert 'pub index: usize' in code
    assert 'data: Vec<u8>' in code
    assert code.endswith('}')
    assert 'unrelated' not in code
