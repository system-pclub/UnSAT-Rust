"""Check actual source-fixture outcomes and selected recovery paths in Docker."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROFILES = ('a', 'b', 'c', 'full')
PROBES = ('PublicState::public_probe', 'Constructed::constructor_probe',
          'Escaped::escape_probe', 'Mutated::mutator_probe')


def main():
    report = {}
    for index, profile in enumerate(PROFILES):
        matrix = json.loads((ROOT / '.local/ablation' / profile / 'matrix.json').read_text())
        assert matrix['ablation_level'] == profile
        assert matrix['ablation_profile_version'] == 2
        assert matrix['total'] == matrix['completed'] == 4
        results = {row['caller']: row for row in matrix['results']}
        assert set(results) == set(PROBES)
        report[profile] = {}
        for probe_index, probe in enumerate(PROBES):
            row = results[probe]
            enabled = (probe_index == 0 or probe_index == 1 and index >= 1
                       or probe_index == 2 and index >= 2
                       or probe_index == 3 and profile == 'full')
            expected = 'violation' if enabled else 'low-confidence-sat'
            assert row['status'] == expected, (profile, probe, row['status'], expected)
            assert row['ablation_level'] == profile
            assert not row['timed_out'] and row['returncode'] == 0
            chain = json.loads(Path(row['certainty_chain']).read_text())
            assert chain['ablation_level'] == profile
            assert chain['ablation_profile_version'] == 2
            assert chain['mut_ref_recovery'] == (index >= 2)
            assert chain['all_certain'] == enabled
            log = Path(row['log']).read_text()
            assert f'[ablation] level={profile} ' in log
            assert f'symbolic-functions={"on" if profile == "full" else "off"}' in log
            info = (Path(row['klee_output_dir']) / 'info').read_text()
            assert 'function body was not found' not in info
            if profile == 'a':
                assert 'selected constructor' not in info and 'selected mutator ' not in info
            if profile != 'full':
                assert "selected mutator 'Mutated::set_index'" not in info
            if probe_index == 1 and enabled:
                assert "selected constructor 'Constructed::new'" in info
            if probe_index == 2 and enabled:
                assert any(step['kind'] == 'mut_ref_escape'
                           for symbol in chain['symbols'] for step in symbol['steps'])
            if probe_index == 3 and enabled:
                assert "selected mutator 'Mutated::set_index'" in info
            report[profile][probe] = row['status']
    summary = {'profiles': report, 'pairs': 16, 'ablation_profile_version': 2,
               'fixture_sha256': hashlib.sha256((ROOT / 'benchmark/ablation/context-levels/src/main.rs').read_bytes()).hexdigest(),
               'symbolic_definition': 'Full = C plus symbolic execution/composition of other callchain candidates, exercised by the ordinary setter. All profiles use the same execution models.'}
    (ROOT / '.local/ablation/summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
