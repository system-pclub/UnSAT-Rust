#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools" / "x" / "src"))
from ablation_runner import add_common_arguments, run_verify_variant


parser = argparse.ArgumentParser(description="Ablate safe-context recovery through returned &mut references.")
add_common_arguments(parser)
raise SystemExit(
    run_verify_variant(
        "without-mut-ref-recovery",
        parser.parse_args(),
        extra_verify_args=("--disable-mut-ref-recovery",),
    )
)
