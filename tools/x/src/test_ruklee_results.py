import tarfile

from ruklee_results import pair_report, finish_crate, archive_crate, restore_pair_outputs


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


def test_retention_keeps_all_tests_queries_and_source_target_directories(tmp_path):
    output = tmp_path / 'eval/ruklee-full/demo'
    klee = output / 'site/rule/klee-out'
    klee.mkdir(parents=True)
    for name in ('test000001.ktest', 'test000002.ktest', 'query.smt2', 'execution.err'):
        (klee / name).write_text(name)
    ir = output / 'ir'
    ir.mkdir()
    (ir / 'input.ll').write_text('exact linked IR')
    injected = tmp_path / 'crates_inj/demo'
    (injected / 'target').mkdir(parents=True)
    (injected / 'target/cache').write_text('rebuildable')
    (injected / 'src/target').mkdir(parents=True)
    (injected / 'src/target/contract.rs').write_text('source, not cache')
    (injected / 'Cargo.toml').write_text('[package]')
    archive_crate(tmp_path, output, 'demo')
    assert not klee.exists()
    assert not ir.exists()
    with tarfile.open(output / 'replay-inputs.tar.gz') as archive:
        names = archive.getnames()
    assert 'crates_inj/demo/src/target/contract.rs' in names
    assert 'crates_inj/demo/target/cache' not in names
    restore_pair_outputs(output)
    assert {p.name for p in klee.iterdir()} == {'test000001.ktest', 'test000002.ktest', 'query.smt2', 'execution.err'}
    assert (klee / 'test000002.ktest').read_text() == 'test000002.ktest'
