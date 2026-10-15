#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import add_common_arguments, run_verify_variant


parser = argparse.ArgumentParser(description="Run the full RuKLEE pipeline on the raudit top ten crates.")
add_common_arguments(parser)
raise SystemExit(run_verify_variant("full", parser.parse_args()))
