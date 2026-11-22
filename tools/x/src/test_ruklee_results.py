from ruklee_results import pair_report, finish_crate


def native(status, certain=False, confirmed=False):
    return {'crate': '/unsat-rust/benchmark/ruklee/demo-1.0',
            'callsite': {'caller': 'State::read', 'path': 'src/lib.rs', 'unsafe_callee': 'ptr::read'},
            'rule': {'id': 'step2-core-rule-1', 'rule': 'Pointer must be readable.', 'dsl': '$1.$mem.pointee.readable'},
            'stages': {'init': {'status': status, 'has_certain_symbol': certain, 'duration_sec': 1.25},
                       'rerun': {'full_rerun_passed': confirmed}}}


def test_uncertain_or_error_is_not_a_safe_rule_or_confirmed_violation():
    for status in ('low-confidence-sat', 'timeout', 'eval-error', 'not-in-ir'):
        result = pair_report(native(status), native_path='/eval/ruklee-full/demo/site/rule/native-result.json')
        assert result['violations'] == []
        assert result['status'] == status
        assert result['classification'] == 'inconclusive'
        assert result['dsl'] == '$1.$mem.pointee.readable'
        assert result['execution']['duration_ms'] == 1250
        assert 'stages' not in result


def test_candidate_and_confirmation_remain_separate():
    candidate = pair_report(native('violation', certain=True), native_path='/eval/ruklee-full/demo/site/rule/native-result.json')
    assert candidate['violations'][0]['requirement'] == 'Pointer must be readable.'
    assert candidate['classification'] == 'unconfirmed_candidate'
    assert candidate['full_rerun_passed'] is False
    confirmed = pair_report(native('violation', certain=True, confirmed=True), native_path='/eval/ruklee-full/demo/site/rule/native-result.json')
    assert confirmed['classification'] == 'reproduced_violation'


def test_no_results_with_failed_compile_is_error(tmp_path):
    result = finish_crate(tmp_path, setting='full', crate='demo', catalog=tmp_path/'rules.json',
                          returncode=1, duration_sec=3.0)
    assert result['status'] == 'error'
    assert result['violations'] == []
