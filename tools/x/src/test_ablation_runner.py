import json
import argparse
from pathlib import Path
from unittest.mock import patch

import pytest

from ablation_runner import _rule_index, _rule_name_index, _rules_for_target, _source_span, add_common_arguments, selected_crates, run_verify_variant


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


def test_roster_dry_run_uses_root_package_catalog_and_no_executions(tmp_path):
    """A roster must not inherit the workbook's blade subpackage or legacy rules."""
    source = tmp_path / 'benchmark/ruklee/blade-0.2.0'
    source.mkdir(parents=True)
    (source / 'Cargo.toml').write_text('[package]\nname="blade"\nversion="0.2.0"\n')
    roster = tmp_path / 'sources.json'
    roster.write_text(json.dumps([{'directory': 'blade-0.2.0'}]))
    parser = argparse.ArgumentParser()
    add_common_arguments(parser)
    args = parser.parse_args(['--roster', str(roster), '--results-root', 'eval',
                              '--metadata-root', 'dev/schedule', '--rule-dsl', 'dev/catalog/rules.json',
                              '--studied-rules', 'dev/catalog/studied_rules', '--dry-run',
                              '--setting-name', 'ruklee-profile-b', '--ablation-level', 'b'])
    args.rustc = tmp_path / 'tools/rust/stage1/rustc'
    args.klee_bin = tmp_path / 'tools/klee/build/bin/klee'
    with patch('ablation_runner.repo_root', return_value=tmp_path), patch('ablation_runner.subprocess.run') as execute:
        assert run_verify_variant('full', args) == 0
        execute.assert_not_called()
    manifest = json.loads((tmp_path / 'dev/schedule/ruklee-profile-b/run-manifest.json').read_text())
    argv = manifest['commands'][0]['argv']
    assert argv[2] == '/unsat-rust/benchmark/ruklee/blade-0.2.0'
    assert argv[argv.index('--results-json') + 1] == '/unsat-rust/eval/ruklee-profile-b/blade-0.2.0/full-matrix-results.json'
    assert argv[argv.index('--rule-dsl') + 1] == '/unsat-rust/dev/catalog/rules.json'
    assert argv[argv.index('--studied-rules') + 1] == '/unsat-rust/dev/catalog/studied_rules'
    assert argv[argv.index('--ablation-level') + 1] == 'b'
    assert '--features' not in argv
    assert not (tmp_path / 'eval/ruklee-profile-b/run-manifest.json').exists()


def test_roster_rejects_unknown_names_and_traversal(tmp_path):
    roster = tmp_path / 'sources.json'
    roster.write_text(json.dumps([{'directory': 'crate-1.0'}]))
    with pytest.raises(RuntimeError, match='unknown input crates'):
        selected_crates(['typo'], roster)
    roster.write_text(json.dumps([{'directory': '../outside'}]))
    with pytest.raises(RuntimeError, match='immediate crate directory'):
        selected_crates(None, roster)
    assert selected_crates(['blade']) == [('blade', 'blade-0.2.0')]
