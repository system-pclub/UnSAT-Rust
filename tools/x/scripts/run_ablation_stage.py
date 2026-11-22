"""Run only a dependency-bearing preparation/LLM stage in the root image."""
import argparse
from contextlib import nullcontext
import fcntl
import json
from pathlib import Path

from ablation_runner import prepare_without_rule_extraction, prepare_without_rule_translation, run_without_symbolic_execution, _translation_root, repo_root


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--stage', required=True)
    parser.add_argument('--arguments', type=Path, required=True)
    options = parser.parse_args()
    data = json.loads(options.arguments.read_text())
    path_fields = ('results_root', 'roster', 'rule_dsl', 'studied_rules', 'metadata_root',
                   'rustc', 'klee_bin', 'translation_cache_root')
    for key in path_fields:
        if data.get(key) is not None:
            data[key] = Path(data[key])
    args = argparse.Namespace(**data)
    if options.stage == 'without-symbolic-execution':
        return run_without_symbolic_execution(args)
    output = _translation_root(repo_root(), args, options.stage)
    output.mkdir(parents=True, exist_ok=True)
    with (output / 'preparation.lock').open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        prepare = prepare_without_rule_extraction if options.stage == 'without-rule-extraction' else prepare_without_rule_translation
        prepare(args)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
