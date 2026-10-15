import json

from ablation_runner import _rule_index


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
