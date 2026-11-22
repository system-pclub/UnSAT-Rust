import argparse
import json
from pathlib import Path

from ablation_translations import prepare_direct_document


def test_whole_document_uses_real_typed_dsl_and_preserves_baseline(tmp_path, monkeypatch):
    """The ablated constraint is generated from docs, separately from the frozen atomic DSL."""
    import ablation_runner
    import ablation_translations
    baseline = tmp_path / 'rules.json'
    baseline.write_text(json.dumps({'include_raw_pointer_deref': False, 'rules': {
        'step2-alloc-rule-960': {'path': 'rust/library/alloc/src/alloc.rs:170',
                               'name': 'unsafe fn alloc_zeroed (layout : Layout) -> * mut u8',
                               'api': 'unsafe fn alloc_zeroed (layout : core::alloc::layout::Layout) -> * mut u8',
                               'rule': 'The caller must ensure that layout has non-zero size.',
                               'dsl': '$1.size != 0'}}}))
    roster = tmp_path / 'roster.json'
    roster.write_text(json.dumps([{'directory': 'demo-1.0'}]))
    report = {'targets': [{'callee': {'name': 'std::alloc::alloc_zeroed',
                                      'path': 'rust/library/alloc/src/alloc.rs', 'line_start': 170}}]}
    monkeypatch.setattr(ablation_runner, '_report_for_crate', lambda *args: report)
    calls = []
    class Client:
        def __init__(self, **kwargs):
            pass
        def complete(self, system, prompt):
            calls.append((system, prompt))
            return '$1.size != 0'
    monkeypatch.setattr(ablation_translations, 'OpenAILLM', Client)
    args = argparse.Namespace(rule_dsl=baseline, roster=roster, crate=None, metadata_root=tmp_path/'metadata',
                              translation_cache_root=tmp_path/'cache', results_root=tmp_path/'results',
                              setting_name=None, model='offline-test', dry_run=False, resume=True)
    result_path = prepare_direct_document(args)
    result = json.loads(result_path.read_text())
    generated = next(iter(result['rules'].values()))
    assert generated['dsl'] == '$1.size != 0'
    assert generated['dsl_v2']['source'] == '$1.size != 0'
    assert generated['baseline_dsl'] == ['$1.size != 0']
    assert generated['baseline_rule_ids'] == ['step2-alloc-rule-960']
    assert generated['evaluator_kind'] == 'whole-document-constraint'
    assert result['include_raw_pointer_deref'] is False
    assert len(calls) == 1
    assert 'Complete documentation' in calls[0][1]
    prepare_direct_document(args)
    assert len(calls) == 1  # successful generation survives a resumed crate
