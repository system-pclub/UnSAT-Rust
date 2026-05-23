import argparse
import json
from pathlib import Path

from verirule.dsl import translate_buggy_violation
from verirule.equivalence import compare_buggy_violation_equivalence
from verirule.utils import load_json



def run(args: argparse.Namespace) -> int:
    rule_a = load_json(args.json_a)
    rule_b = load_json(args.json_b)

    result = compare_buggy_violation_equivalence(
        rule_a,
        rule_b,
        use_context=not args.no_context,
    )

    print(json.dumps(result, indent=2, ensure_ascii=False))

    if result["equivalent"] is True:
        return 0

    if result["equivalent"] is False:
        return 1

    return 2