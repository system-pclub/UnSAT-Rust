import argparse
import json
from pathlib import Path

from verirule.dsl import translate_buggy_violation
from verirule.equivalence import compare_buggy_violation_equivalence
from verirule.utils import load_json




def run(args: argparse.Namespace) -> int:
    rule = load_json(args.json_path)
    dsl = translate_buggy_violation(rule)
    print(rule["rule"])
    print(rule["description"])
    print("buggy_violation:")
    print(dsl)

    return 0