import argparse
import logging
def _run_view(args: argparse.Namespace) -> int:
    from cli.cmd.view import run

    return run(args)


def _run_compare(args: argparse.Namespace) -> int:
    from cli.cmd.compare import run

    return run(args)


def _run_generate(args: argparse.Namespace) -> int:
    from cli.cmd.generate import run

    return run(args)


def _run_get_batch_task(args: argparse.Namespace) -> int:
    from cli.cmd.get_batch_task import run

    return run(args)


def _run_submit_batch_task(args: argparse.Namespace) -> int:
    from cli.cmd.submit_batch_task import run

    return run(args)


def _run_cancel_batch_task(args: argparse.Namespace) -> int:
    from cli.cmd.cancel_batch_task import run

    return run(args)


def _run_process(args: argparse.Namespace) -> int:
    from cli.cmd.process import run

    return run(args)


def _run_sync(args: argparse.Namespace) -> int:
    from cli.cmd.sync import run

    return run(args)


def _run_merge_rules(args: argparse.Namespace) -> int:
    from cli.cmd.merge_rules import run

    return run(args)


def _run_eval(args: argparse.Namespace) -> int:
    from cli.cmd.eval import run

    return run(args)


def _run_summary(args: argparse.Namespace) -> int:
    from cli.cmd.summary import run

    return run(args)

def _run_llvmir(args: argparse.Namespace) -> int:
    from cli.cmd.llvmir import run

    return run(args)

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="x",
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
    view_parser.set_defaults(func=_run_view)

    compare_parser = subparsers.add_parser(
        "compare",
        help="Compare task1 constraints between a crate meta JSON and another result JSON.",
    )
    compare_parser.add_argument(
        "cargo_dir",
        help="Path to the Rust crate directory (for ensuring crates/<crate>.json and LLVM IR).",
    )
    compare_parser.add_argument(
        "--other",
        required=True,
        help="Path to another JSON file (e.g., eval/<crate>.json) containing task1 results.",
    )
    compare_parser.add_argument(
        "--studied-rules",
        default="studied_rules",
        help="Path to studied_rules used when compare needs to sync metadata.",
    )
    compare_parser.add_argument(
        "--ir-output-dir",
        default=".local/irs",
        help="Directory where linked LLVM IR is expected/written (default: .local/irs).",
    )
    compare_parser.add_argument(
        "--work-dir",
        default=".local/compare",
        help="Directory to write intermediate SMT2 files (default: .local/compare).",
    )
    compare_parser.add_argument(
        "--output",
        default=".local/compare/report.json",
        help="Path to JSON compare report output.",
    )
    compare_parser.add_argument(
        "--klee-bin",
        default="klee",
        help="KLEE executable to use (default: klee).",
    )
    compare_parser.add_argument(
        "--rustc",
        help="Optional custom rustc for llvmir generation.",
    )
    compare_parser.add_argument(
        "--test",
        action="store_true",
        help="Compile tests for llvmir generation instead of the main crate.",
    )
    compare_parser.add_argument(
        "--build-std",
        action="store_true",
        help="Pass -Zbuild-std while generating llvmir for compare.",
    )
    
    compare_parser.set_defaults(func=_run_compare)
    
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
    generate_parser.set_defaults(func=_run_generate)

    get_batch_task_parser = subparsers.add_parser(
        "get-batch-task",
        help="Poll an OpenAI batch task and write output JSONL when complete.",
    )
    get_batch_task_parser.add_argument(
        "batch_id",
        help="OpenAI batch task id.",
    )
    get_batch_task_parser.set_defaults(func=_run_get_batch_task)

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
    submit_batch_task_parser.set_defaults(func=_run_submit_batch_task)

    cancel_batch_task_parser = subparsers.add_parser(
        "cancel-batch-task",
        help="Cancel an in-progress OpenAI batch task.",
    )
    cancel_batch_task_parser.add_argument(
        "batch_id",
        help="OpenAI batch task id to cancel.",
    )
    cancel_batch_task_parser.set_defaults(func=_run_cancel_batch_task)

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
    process_parser.set_defaults(func=_run_process)

    sync_parser = subparsers.add_parser(
        "sync",
        help="Compile crates with MIR scan rustc and persist crate metadata + report.",
    )
    sync_parser.add_argument(
        "--studied-rules",
        default="studied_rules",
        help="Path to file listing allowed rule ids (one per line). Defaults to studied_rules.",
    )
    sync_parser.add_argument(
        "--cargo-dir",
        help="Path to only one Rust crate to sync. If not provided, all crates in crates/ will be synced.",
    )
    sync_parser.add_argument(
        "--strict",
        action="store_true",
        help="When writing human/<crate>.json, remove old callsites/rules that are not present in current sync output.",
    )
    sync_parser.set_defaults(func=_run_sync)

    merge_rules_parser = subparsers.add_parser(
        "merge-rules",
        help="Merge TP=1 rows from all CSV rule files into a single rules.csv.",
    )
    merge_rules_parser.add_argument(
        "--rules-dir",
        default=".local/rules",
        help="Directory containing rule CSV files to merge.",
    )
    merge_rules_parser.add_argument(
        "--output",
        default="rules.csv",
        help="Path to merged CSV output file.",
    )
    merge_rules_parser.set_defaults(func=_run_merge_rules)

    eval_parser = subparsers.add_parser(
        "eval",
        help="Run LLM prompts (task1/2/3) against each crate meta JSON and save results to eval/.",
    )
    eval_parser.add_argument(
        "--model",
        default="gpt-4o",
        help="OpenAI model to use (default: gpt-4o).",
    )
    eval_parser.add_argument(
        "--verbose",
        action="store_true",
        help="Print per-rule progress.",
    )
    eval_parser.set_defaults(func=_run_eval)

    summary_parser = subparsers.add_parser(
        "summary",
        help="Show crate/target/task summary counts from crates/*.json.",
    )
    summary_parser.set_defaults(func=_run_summary)
    
    llvmir_parser = subparsers.add_parser(
        "llvmir",
        help="Compile a Rust crate with --emit=llvm-ir and collect the emitted .ll files.",
    )
    llvmir_parser.add_argument(
        "cargo_dir",
        help="Path to the Rust crate to compile.",
    )
    llvmir_parser.add_argument(
        "--test",
        action="store_true",
        help="Compile tests instead of the main crate.",
    )
    llvmir_parser.add_argument(
        "--output-dir",
        help="Directory to copy the collected .ll files to.",
    )
    llvmir_parser.add_argument(
        "--rustc",
        help="Path to a custom rustc executable to use for compilation.",
    )
    llvmir_parser.add_argument(
        "--build-std",
        action="store_true",
        help="Pass -Zbuild-std=core,alloc,std to cargo so that std crates also emit LLVM IR.",
    )
    llvmir_parser.set_defaults(func=_run_llvmir)

    return parser


def main(argv: list[str] | None = None) -> int:
    logging.basicConfig(level=logging.INFO, format="%(message)s")
    parser = build_parser()
    args = parser.parse_args(argv)

    try:
        return args.func(args)
    except Exception as e:
        print(f"Error: {e}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())