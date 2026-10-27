#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import (
    add_common_arguments,
    prepare_without_rule_translation,
    run_verify_variant,
)


parser = argparse.ArgumentParser(
    description="Translate extracted rules directly to SMT-LIB plus variable selectors, then run KLEE."
)
add_common_arguments(parser)
parser.add_argument("--prepare-only", action="store_true")
args = parser.parse_args()
rule_file = prepare_without_rule_translation(args)
if args.prepare_only or args.dry_run:
    raise SystemExit(0)
raise SystemExit(
    run_verify_variant("without-rule-translation", args, rule_dsl=rule_file)
)
