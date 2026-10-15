#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import (
    add_common_arguments,
    prepare_without_rule_extraction,
    run_verify_variant,
)


parser = argparse.ArgumentParser(description="Translate each complete Safety document directly to DSL, then run RuKLEE.")
add_common_arguments(parser)
parser.add_argument("--prepare-only", action="store_true")
args = parser.parse_args()
rule_dsl = prepare_without_rule_extraction(args)
if args.prepare_only or args.dry_run:
    raise SystemExit(0)
raise SystemExit(run_verify_variant("without-rule-extraction", args, rule_dsl=rule_dsl))
