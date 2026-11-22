import json
from pathlib import Path
import subprocess
import sys


SCRIPT = Path(__file__).with_name('plan_ruklee_ablation.py')


def test_plan_union_and_resume_rejects_changed_catalog(tmp_path):
    roster = tmp_path / 'roster.json'
    roster.write_text(json.dumps([{'directory': 'one-1.0'}, {'directory': 'two-2.0'}]))
    catalog = tmp_path / 'catalog'
    catalog.mkdir()
    (catalog / 'rules.json').write_text('{"rules":{}}')
    (catalog / 'studied_rules').write_text('# no legacy rules\n')
    metadata = tmp_path / 'metadata'
    results = tmp_path / 'eval'
    command = [sys.executable, str(SCRIPT), '--suite', 'all', '--roster', str(roster),
               '--catalog', str(catalog), '--metadata-root', str(metadata), '--results-root', str(results)]
    result = subprocess.run(command, capture_output=True, text=True)
    assert result.returncode == 0, result.stderr
    plan = json.loads((metadata / 'plan.json').read_text())
    assert len(plan['commands']) == 18
    assert len({row['setting'] for row in plan['commands']}) == 9
    assert sum(row['setting'] == 'full' for row in plan['commands']) == 2
    assert not results.exists()
    assert not (metadata / 'jobs').exists()
    result = subprocess.run(command + ['--resume'], capture_output=True, text=True)
    assert result.returncode == 0, result.stderr
    (catalog / 'rules.json').write_text('{"rules":{"changed":{}}}')
    result = subprocess.run(command + ['--resume'], capture_output=True, text=True)
    assert result.returncode != 0
    assert 'different inputs/settings' in result.stderr
