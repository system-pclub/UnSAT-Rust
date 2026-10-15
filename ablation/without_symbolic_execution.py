#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import add_common_arguments, run_without_symbolic_execution


parser = argparse.ArgumentParser(description="Replace KLEE with an LLM judgment over each DSL rule and related Rust code.")
add_common_arguments(parser)
raise SystemExit(run_without_symbolic_execution(parser.parse_args()))
