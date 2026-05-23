import argparse

from verirule.cmd.compare import run as compare_run
from verirule.cmd.view import run as view_run
from verirule.cmd.generate import run as generate_run
from verirule.cmd.get_batch_task import run as get_batch_task_run
from verirule.cmd.submit_batch_task import run as submit_batch_task_run
from verirule.cmd.cancel_batch_task import run as cancel_batch_task_run
from verirule.cmd.process import run as process_run

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="verirule",
        description="Validate SMT rule JSON and compare buggy violation equivalence.",
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    view_parser = subparsers.add_parser(
        "view",
        help="Validate a rule JSON file and print buggy_violation as readable DSL.",
    )
    view_parser.add_argument(
        "json_path",
        help="Path to rule JSON file.",
    )
    view_parser.set_defaults(func=view_run)

    compare_parser = subparsers.add_parser(
        "compare",
        help="Compare whether two rule JSON files have equivalent buggy_violation constraints.",
    )
    compare_parser.add_argument(
        "json_a",
        help="Path to first rule JSON file.",
    )
    compare_parser.add_argument(
        "json_b",
        help="Path to second rule JSON file.",
    )
    compare_parser.add_argument(
        "--no-context",
        action="store_true",
        help="Compare buggy_violation globally, without preconditions and operation_semantics.",
    )
    compare_parser.set_defaults(func=compare_run)
    
    generate_parser = subparsers.add_parser(
        "generate",
        help="Generate rule JSON files from a Rust crate.",
    )
    generate_parser.add_argument(
        "crate_dir",
        help="Path to the Rust crate.",
    )
    generate_parser.add_argument(
        "--rustscan",
        help="Path to the rustscan executable.",
    )
    generate_parser.add_argument(
        "--output-dir",
        help="Path to output the generated rule JSON files.",
    )
    generate_parser.set_defaults(func=generate_run)

    get_batch_task_parser = subparsers.add_parser(
        "get-batch-task",
        help="Poll an OpenAI batch task and write output JSONL when complete.",
    )
    get_batch_task_parser.add_argument(
        "batch_id",
        help="OpenAI batch task id.",
    )
    get_batch_task_parser.set_defaults(func=get_batch_task_run)

    submit_batch_task_parser = subparsers.add_parser(
        "submit-batch-task",
        help="Submit an OpenAI batch task from a JSONL request file.",
    )
    submit_batch_task_parser.add_argument(
        "--in",
        dest="in",
        required=True,
        help="Path to input JSONL file for OpenAI batch requests.",
    )
    submit_batch_task_parser.set_defaults(func=submit_batch_task_run)

    cancel_batch_task_parser = subparsers.add_parser(
        "cancel-batch-task",
        help="Cancel an in-progress OpenAI batch task.",
    )
    cancel_batch_task_parser.add_argument(
        "batch_id",
        help="OpenAI batch task id to cancel.",
    )
    cancel_batch_task_parser.set_defaults(func=cancel_batch_task_run)

    process_parser = subparsers.add_parser(
        "process",
        help="Process OpenAI batch output JSONL into individual rule JSON files.",
    )
    process_parser.add_argument(
        "input_jsonl",
        help="Path to the batch output JSONL file.",
    )
    process_parser.add_argument(
        "--output-dir",
        required=True,
        help="Directory where extracted rule JSON files will be written.",
    )
    process_parser.set_defaults(func=process_run)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    try:
        return args.func(args)
    except Exception as e:
        print(f"Error: {e}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())