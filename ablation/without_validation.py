#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import add_common_arguments, run_verify_variant


parser = argparse.ArgumentParser(description="Ablate LLM testcase generation and same-callsite rerun validation.")
add_common_arguments(parser)
raise SystemExit(
    run_verify_variant(
        "without-validation",
        parser.parse_args(),
        # This ablation is scored at callsite granularity. Once the initial
        # KLEE pass finds one violation candidate, later obligations at the
        # same callsite cannot change that callsite-level prediction.
        extra_verify_args=("--skip-llm-testcase", "--stop-callsite-if-violated"),
    )
)
