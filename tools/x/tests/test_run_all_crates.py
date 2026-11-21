import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tarfile
import unittest

ROOT = Path(__file__).resolve().parents[3]


class RunnerChecks(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.repo = Path(self.temporary.name)
        shutil.copy(ROOT / 'run-all-crates', self.repo / 'run-all-crates')
        self.bin = self.repo / 'bin'
        self.bin.mkdir()
        (self.repo / 'tools/klee/build/bin').mkdir(parents=True)
        (self.repo / 'tools/klee/build/bin/klee').write_text('fixture binary')
        (self.repo / 'tools/x/scripts').mkdir(parents=True)
        shutil.copy(ROOT / 'tools/x/scripts/index_crate_reports.py', self.repo / 'tools/x/scripts')
        self.write_executable('docker', '''#!/bin/bash
if [[ $1 == run ]]; then
  output=${@: -1}
  mkdir -p "$output"
  printf '{"rules":{}}' > "$output/rules.json"
  touch "$output/studied_rules"
fi
''')
        self.write_executable('df', '''#!/bin/bash
if [[ -n ${MOCK_DISK_STATE:-} ]]; then
  n=0; [[ ! -f $MOCK_DISK_STATE ]] || n=$(cat "$MOCK_DISK_STATE")
  n=$((n+1)); printf '%s' "$n" > "$MOCK_DISK_STATE"
  if (( n >= 3 )); then MOCK_FREE_KIB=1; fi
fi
printf 'Filesystem 1024-blocks Used Available Capacity Mounted\\nmock 1000000000 1 %s 1%% /\\n' "${MOCK_FREE_KIB:-900000000}"
''')
        self.write_executable('x', '''#!/bin/bash
[[ -z ${OPENAI_API_KEY+x} ]] || exit 99
printf '%s\\n' "$@" >> commands.txt
crate=$2
while [[ $# -gt 0 ]]; do
  if [[ $1 == --results-json ]]; then result=$2; shift; fi
  if [[ $1 == --ir-output-dir ]]; then ir=$2; shift; fi
  shift
done
mkdir -p "${result%/*}/site/rule/klee-out" "$crate/target" "crates_inj/${crate##*/}"
printf klee-ext-bind > "crates_inj/${crate##*/}/Cargo.toml"
mkdir -p "crates_inj/${crate##*/}/target" "crates_inj/${crate##*/}/src/target"
printf rebuildable > "crates_inj/${crate##*/}/target/cache"
printf source-not-cache > "crates_inj/${crate##*/}/src/target/evidence.rs"
printf '{}' > "${result%/*}/site/rule/result.json"
printf evidence > "${result%/*}/site/rule/klee.log"
printf scratch > "${result%/*}/site/rule/klee-out/large.smt2"
printf first-witness > "${result%/*}/site/rule/klee-out/test000001.ktest"
printf second-witness > "${result%/*}/site/rule/klee-out/test000002.ktest"
printf error-details > "${result%/*}/site/rule/klee-out/test000002.ptr.err"
mkdir -p "$ir"
printf linked-input > "$ir/input.ll"
printf '{"results":[{"status":"low-confidence-sat","caller":"probe","rule":"fixture","callsite":"site"}]}' > "$result"
[[ ${MOCK_HANG:-0} != 1 ]] || sleep 30
[[ $crate != *first* ]]
''', in_repo=True)
        self.sources = self.repo / 'inputs with spaces'
        for name in ('first crate', 'second crate'):
            source = self.sources / name
            source.mkdir(parents=True)
            (source / 'Cargo.toml').write_text('[package]\nname="fixture"\nversion="0.1.0"\n')

    def write_executable(self, name, body, in_repo=False):
        path = (self.repo if in_repo else self.bin) / name
        path.write_text(body)
        path.chmod(0o755)

    def run_runner(self, **settings):
        environment = {**os.environ, 'PATH': str(self.bin) + ':' + os.environ['PATH'],
                       'DISK_CHECK_SECONDS': '1', **settings}
        result = subprocess.run([str(self.repo / 'run-all-crates'), str(self.sources)],
                                env=environment, capture_output=True, text=True, timeout=30)
        latest_path = self.repo / '.local/run-all-crates/latest'
        self.assertTrue(latest_path.exists(), result.stdout + result.stderr)
        latest = latest_path.read_text().strip()
        return result, Path(latest)

    def test_serial_no_llm_flags_failure_isolation_and_owned_cleanup(self):
        result, run = self.run_runner(OPENAI_API_KEY='fixture-must-not-reach-x')
        self.assertEqual(result.returncode, 0, result.stderr)
        progress = (run / 'progress.tsv').read_text()
        self.assertIn('1\t1\tfailed', progress)
        self.assertIn('2\t0\tfinished', progress)
        commands = (self.repo / 'commands.txt').read_text()
        self.assertEqual(commands.count('--skip-llm-testcase'), 2)
        self.assertEqual(commands.count('--ablation-level\nfull'), 2)
        for job in (run / 'jobs').iterdir():
            pair = job / 'site/rule'
            self.assertTrue((pair / 'result.json').exists())
            self.assertTrue((pair / 'klee.log.gz').exists())
            self.assertFalse((pair / 'klee-out').exists())
            with tarfile.open(pair / 'klee-out.tar.gz') as archive:
                self.assertEqual(archive.extractfile('klee-out/test000002.ktest').read(), b'second-witness')
                self.assertEqual(archive.extractfile('klee-out/test000002.ptr.err').read(), b'error-details')
                self.assertEqual(archive.extractfile('klee-out/large.smt2').read(), b'scratch')
            with tarfile.open(job / 'replay-inputs.tar.gz') as archive:
                names = archive.getnames()
                self.assertTrue(any(name.endswith('/input.ll') for name in names))
                slug = Path((job / 'source.path').read_text().strip()).name
                self.assertFalse(any(name.startswith(f'crates_inj/{slug}/target') for name in names))
                self.assertEqual(archive.extractfile(f'crates_inj/{slug}/src/target/evidence.rs').read(), b'source-not-cache')
        self.assertEqual(len((run / 'findings.jsonl').read_text().splitlines()), 2)
        self.assertFalse(any((self.repo / 'crates_inj').iterdir()))
        self.assertTrue(all((source / 'Cargo.toml').exists() and not (source / 'target').exists()
                            for source in self.sources.iterdir()))

    def test_low_disk_stops_before_build_or_analysis(self):
        result, run = self.run_runner(MOCK_FREE_KIB='1')
        self.assertEqual(result.returncode, 75)
        self.assertEqual((run / 'status').read_text().strip(), 'disk-low')
        self.assertFalse((self.repo / 'commands.txt').exists())

    def test_start_index_and_limit(self):
        result, run = self.run_runner(START_INDEX='2', MAX_CRATES='1')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('2\t0\tfinished', (run / 'progress.tsv').read_text())
        self.assertNotIn('first crate', (self.repo / 'commands.txt').read_text())

    def test_mid_crate_disk_watchdog_cleans_completed_pairs_and_stops(self):
        result, run = self.run_runner(MOCK_DISK_STATE=str(self.repo / 'disk-counter'), MOCK_HANG='1')
        self.assertEqual(result.returncode, 75, result.stderr)
        self.assertEqual((run / 'status').read_text().strip(), 'disk-low')
        job = run / 'jobs/000001'
        self.assertTrue((job / 'disk-stop').exists())
        self.assertFalse((job / 'site/rule/klee-out').exists())
        self.assertTrue((job / 'site/rule/result.json').exists())
        self.assertFalse((run / 'jobs/000002').exists())

    def mock_old_image_worker(self):
        self.write_executable('docker', '''#!/bin/bash
case $1 in
  run)
    output=${@: -1}; mkdir -p "$output"
    printf '{"rules":{}}' > "$output/rules.json"; touch "$output/studied_rules" ;;
  ps)
    # A tag refresh makes this worker invisible to an ancestor filter.
    [[ "$*" == *ancestor=* ]] || printf 'unrelated\\nstale-image-worker\\n' ;;
  inspect)
    if [[ ${@: -1} == unrelated ]]; then
      printf '["verify", "other-run/jobs/000001/matrix.json"]'
    else
      run=$(cat .local/run-all-crates/latest)
      printf '["verify", "%s/jobs/000001/matrix.json"]' "${run#"$PWD/"}"
    fi ;;
  stop)
    printf '%s\\n' "${@: -1}" >> stopped-workers
    [[ ${MOCK_STOP_FAIL:-0} != 1 ]] || exit 1
    run=$(cat .local/run-all-crates/latest)
    printf final-witness > "$run/jobs/000001/site/rule/klee-out/final.ktest" ;;
esac
''')

    def test_timeout_stops_old_image_worker_before_evidence_retention(self):
        self.mock_old_image_worker()
        result, run = self.run_runner(MOCK_HANG='1', CRATE_TIMEOUT_SEC='1', MAX_CRATES='1')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue((self.repo / 'stopped-workers').exists(), 'old-image worker was never stopped')
        self.assertEqual((self.repo / 'stopped-workers').read_text(), 'stale-image-worker\n')
        self.assertIn('1\t124\tfailed', (run / 'progress.tsv').read_text())
        with tarfile.open(run / 'jobs/000001/site/rule/klee-out.tar.gz') as archive:
            self.assertEqual(archive.extractfile('klee-out/final.ktest').read(), b'final-witness')

    def test_worker_stop_failure_preserves_inputs_and_prevents_next_job(self):
        self.mock_old_image_worker()
        result, run = self.run_runner(MOCK_HANG='1', CRATE_TIMEOUT_SEC='1', MOCK_STOP_FAIL='1')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((run / 'status').read_text().strip(), 'container-stop-failed')
        self.assertTrue((run / 'work/ir/input.ll').exists())
        self.assertTrue((run / 'jobs/000001/site/rule/klee-out/test000002.ktest').exists())
        self.assertTrue((self.repo / 'crates_inj/first crate/src/target/evidence.rs').exists())
        self.assertFalse((run / 'jobs/000002').exists())
        self.assertFalse((run / 'jobs/000001/cleanup.txt').exists())
