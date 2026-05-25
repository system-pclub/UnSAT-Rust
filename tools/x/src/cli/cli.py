import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tomllib

from cli.cmd.compare import run as compare_run
from cli.cmd.view import run as view_run
from cli.cmd.generate import run as generate_run
from cli.cmd.get_batch_task import run as get_batch_task_run
from cli.cmd.submit_batch_task import run as submit_batch_task_run
from cli.cmd.cancel_batch_task import run as cancel_batch_task_run
from cli.cmd.process import run as process_run


def _find_repo_root() -> Path:
    start = Path.cwd().resolve()
    for directory in [start, *start.parents]:
        if (directory / "crates").is_dir():
            return directory
    raise RuntimeError(
        f"could not locate repository root from {start} (expected an ancestor containing crates/)"
    )


def _find_crates(crates_dir: Path) -> list[Path]:
    if not crates_dir.is_dir():
        raise RuntimeError(f"failed to read crates directory {crates_dir}")

    crate_dirs = [
        child
        for child in crates_dir.iterdir()
        if child.is_dir() and (child / "Cargo.toml").is_file()
    ]
    return sorted(crate_dirs)


def _build_local_mirscan(repo_root: Path) -> None:
    mirscan_manifest = repo_root / "tools" / "mirscan" / "Cargo.toml"
    if not mirscan_manifest.is_file():
        return

    result = subprocess.run(
        ["cargo", "build", "--manifest-path", str(mirscan_manifest)],
        cwd=repo_root,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            "building local mirscan failed "
            f"(manifest: {mirscan_manifest}, exit: {result.returncode})"
        )


def _resolve_mirscan_rustc(repo_root: Path) -> str:
    configured = os.environ.get("MIRSCAN_RUSTC")
    if configured:
        return configured

    local_raudit = repo_root / "tools" / "mirscan" / "target" / "release" / "raudit"
    if local_raudit.is_file():
        return str(local_raudit)
    local_raudit_exe = local_raudit.with_suffix(".exe")
    if local_raudit_exe.is_file():
        return str(local_raudit_exe)

    _build_local_mirscan(repo_root)
    if local_raudit.is_file():
        return str(local_raudit)
    if local_raudit_exe.is_file():
        return str(local_raudit_exe)

    path_raudit = shutil.which("raudit")
    if path_raudit:
        return path_raudit
    path_mirscan = shutil.which("mirscan")
    if path_mirscan:
        return path_mirscan

    raise RuntimeError(
        "could not find mirscan rustc binary; set MIRSCAN_RUSTC or build tools/mirscan "
        "(expected tools/mirscan/target/release/raudit)"
    )


def _compile_crate(crate_dir: Path, mirscan_rustc: str, report_path: Path) -> None:
    if report_path.exists():
        report_path.unlink()

    clean_result = subprocess.run(["cargo", "clean"], cwd=crate_dir, check=False)
    if clean_result.returncode != 0:
        raise RuntimeError(f"cargo clean failed in {crate_dir} (exit: {clean_result.returncode})")

    env = os.environ.copy()
    env["RUSTC"] = mirscan_rustc
    env["ANALYSIS_OUT"] = str(report_path)
    check_result = subprocess.run(["cargo", "check"], cwd=crate_dir, env=env, check=False)
    if check_result.returncode != 0:
        raise RuntimeError(
            f"cargo check failed in {crate_dir} with RUSTC={mirscan_rustc} "
            f"and ANALYSIS_OUT={report_path} (exit: {check_result.returncode})"
        )


def _parse_package_name_and_description(cargo_toml_content: str) -> tuple[str | None, str | None]:
    manifest = tomllib.loads(cargo_toml_content)
    package = manifest.get("package")
    if not isinstance(package, dict):
        return None, None

    name = package.get("name")
    description = package.get("description")

    return (
        name if isinstance(name, str) else None,
        description if isinstance(description, str) else None,
    )


def run_sync(args: argparse.Namespace) -> int:
    _ = args
    repo_root = _find_repo_root()
    crates_dir = repo_root / "crates"
    meta_dir = repo_root / "meta"
    meta_dir.mkdir(parents=True, exist_ok=True)

    mirscan_rustc = _resolve_mirscan_rustc(repo_root)
    crate_dirs = _find_crates(crates_dir)

    for crate_dir in crate_dirs:
        crate_name = crate_dir.name or "unknown"
        crate_dir_relative = crate_dir.relative_to(repo_root).as_posix()

        print(f"syncing crate {crate_name} with rustc={mirscan_rustc}")

        report_path = crate_dir / "report.json"
        _compile_crate(crate_dir, mirscan_rustc, report_path)

        try:
            report = json.loads(report_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            raise RuntimeError(
                f"report.json is not valid JSON for crate {crate_name} at {report_path}"
            ) from exc

        cargo_toml_path = crate_dir / "Cargo.toml"
        cargo_toml_content = cargo_toml_path.read_text(encoding="utf-8")
        parsed_name, parsed_description = _parse_package_name_and_description(cargo_toml_content)

        out = {
            "crate_dir": crate_dir_relative,
            "crate_name": parsed_name,
            "description": parsed_description,
            "report": report,
        }

        out_path = meta_dir / f"{crate_name}.json"
        out_path.write_text(json.dumps(out, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    return 0

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

    sync_parser = subparsers.add_parser(
        "sync",
        help="Compile crates with MIR scan rustc and persist crate metadata + report.",
    )
    sync_parser.set_defaults(func=run_sync)

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