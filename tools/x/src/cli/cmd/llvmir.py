import argparse
import json
import logging
import os
from pathlib import Path
import subprocess
import tomllib

logger = logging.getLogger(__name__)

def _emit_llvm_rustflags(
    *,
    panic_abort: bool = False,
    panic_abort_tests: bool = False,
) -> str:
    flags = [
        "-Zinline-mir=no",
        "--emit=llvm-ir",
        "-Cllvm-args=--inline-threshold=0",
        "-Copt-level=0",
        "-Ccodegen-units=1",
    ]
    if panic_abort:
        flags.append("-Cpanic=abort")
    if panic_abort_tests:
        flags.append("-Zpanic_abort_tests")
    return " ".join(flags)


def _build_std_args(*, test: bool = False, panic_abort: bool = False) -> list[str]:
    if test and panic_abort:
        return ["-Zbuild-std=std,panic_abort,test"]
    return ["-Zbuild-std"]


def compile_with_emit_llvm(
    cargo_dir: Path,
    custom_rustc: str = None,
    build_std: bool = False,
    panic_abort: bool = False,
) -> None:
    """Compile the crate at *cargo_dir* and emit LLVM IR (.ll) files.
    """
    import platform
    env = os.environ.copy()
    env["CARGO_INCREMENTAL"] = "0" 
    if custom_rustc:
        env["RUSTC"] = custom_rustc
    env["RUSTFLAGS"] = _emit_llvm_rustflags(panic_abort=panic_abort)

    cmd = ["cargo", "build"]
    if build_std:
        cmd += _build_std_args(test=False, panic_abort=panic_abort)
        # cargo -Zbuild-std requires an explicit --target
        machine = platform.machine().lower()
        arch = "x86_64" if machine in ("x86_64", "amd64") else machine
        target = f"{arch}-unknown-linux-gnu"
        cmd += ["--target", target]
    logger.info(f"Running command: {cmd}  (cwd={cargo_dir})")

    result = subprocess.run(
        cmd,
        cwd=cargo_dir,
        env=env,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        raise RuntimeError(
            f"Failed to compile '{cargo_dir}':\n"
            f"stdout: {result.stdout}\n"
            f"stderr: {result.stderr}"
        )

def compile_test_with_emit_llvm(
    cargo_dir: Path,
    custom_rustc: str = None,
    build_std: bool = False,
    panic_abort: bool = False,
) -> None:
    """Compile the test in the crate at *cargo_dir* and emit LLVM IR (.ll) files.
    """
    import platform
    env = os.environ.copy()
    env["CARGO_INCREMENTAL"] = "0" 
    if custom_rustc:
        env["RUSTC"] = custom_rustc
    env["RUSTFLAGS"] = _emit_llvm_rustflags(
        panic_abort=panic_abort,
        panic_abort_tests=panic_abort,
    )

    cmd = ["cargo", "test", "--no-run"]
    if panic_abort:
        cmd.append("-Zpanic-abort-tests")
    if build_std:
        cmd += _build_std_args(test=True, panic_abort=panic_abort)
        machine = platform.machine().lower()
        arch = "x86_64" if machine in ("x86_64", "amd64") else machine
        target = f"{arch}-unknown-linux-gnu"
        cmd += ["--target", target]
    logger.info(f"Running command: {cmd}  (cwd={cargo_dir})")

    result = subprocess.run(
        cmd,
        cwd=cargo_dir,
        env=env,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        raise RuntimeError(
            f"Failed to compile test in '{cargo_dir}':\n"
            f"stdout: {result.stdout}\n"
            f"stderr: {result.stderr}"
        )
        
def _get_crate_name(cargo_dir: Path) -> str | None:
    """Return the [package] name from Cargo.toml, or None."""
    toml_path = cargo_dir / "Cargo.toml"
    try:
        with open(toml_path, "rb") as f:
            data = tomllib.load(f)
        return data.get("package", {}).get("name")
    except (OSError, tomllib.TOMLDecodeError):
        return None


def _get_workspace_members(workspace_dir: Path) -> list[tuple[str, Path]] | None:
    """Return [(crate_name, crate_dir), ...] for a workspace, or None."""
    toml_path = workspace_dir / "Cargo.toml"
    try:
        with open(toml_path, "rb") as f:
            data = tomllib.load(f)
    except (OSError, tomllib.TOMLDecodeError):
        return None

    members = data.get("workspace", {}).get("members")
    if members is None:
        return None

    result = []
    for member_path in members:
        member_dir = workspace_dir / member_path
        member_toml = member_dir / "Cargo.toml"
        try:
            with open(member_toml, "rb") as f:
                mdata = tomllib.load(f)
            name = mdata.get("package", {}).get("name")
            if name:
                result.append((name, member_dir))
        except (OSError, tomllib.TOMLDecodeError):
            continue

    return result or None


def _get_crate_name_or_workspace_members(cargo_dir: Path) -> list[tuple[str, Path]]:
    """Return all (crate_name, crate_dir) pairs for a single crate or workspace."""
    ws = _get_workspace_members(cargo_dir)
    if ws:
        return ws
    name = _get_crate_name(cargo_dir)
    if name:
        return [(name, cargo_dir)]
    return []


def _find_llvm_ir(deps_dir: Path, crate_name: str) -> Path:
    """Find the newest .ll file for *crate_name* inside *deps_dir*."""
    matches = [
        entry
        for entry in deps_dir.iterdir()
        if entry.suffix == ".ll" and entry.name.startswith(crate_name + "-")
    ]
    if matches:
        return max(matches, key=lambda path: path.stat().st_mtime)
    raise FileNotFoundError(f"LLVM IR file not found for crate '{crate_name}' in {deps_dir}")


def _find_test_llvm_irs(deps_dir: Path, main_ir: Path, crate_name: str) -> list[Path]:
    """Find integration/unit test harness IR files in *deps_dir*.

    Cargo names integration test crates after the test target, not the package,
    so package-name lookup misses them. Keep this conservative: include .ll
    files that are newer than or as new as the selected package IR and are not
    obvious dependency/runtime crates.
    """
    skip_prefixes = {
        "alloc-",
        "compiler_builtins-",
        "core-",
        "klee_ext_bind-",
        "libc-",
        "std-",
        "test-",
    }
    found: list[Path] = []
    main_mtime = main_ir.stat().st_mtime
    for entry in deps_dir.iterdir():
        if entry.suffix != ".ll" or entry == main_ir:
            continue
        if entry.name.startswith(crate_name + "-"):
            continue
        if any(entry.name.startswith(prefix) for prefix in skip_prefixes):
            continue
        if entry.stat().st_mtime + 1 < main_mtime:
            continue
        found.append(entry)
    return sorted(found)

def _llvm_ir_defines_main(path: Path) -> bool:
    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            for line in f:
                stripped = line.lstrip()
                if stripped.startswith("define ") and " @main(" in stripped:
                    return True
    except OSError:
        return False
    return False


def _collect_test_link_llvm_irs(
    deps_dir: Path,
    *,
    test_irs: list[Path],
    main_ir: Path,
    crate_name: str,
    build_std: bool,
) -> list[Path]:
    """Collect IR needed by a test harness without linking duplicate mains.

    Cargo emits the test harness as its own crate. It references the package
    rlib and dependency crates, but the old test-mode linker only used the
    harness plus std/core, which leaves dependency globals unresolved (for
    example ahash's random-state OnceBox). Link the harness, the package lib
    IR, and ordinary dependency IRs; skip alternate executable harnesses and
    build-std support crates that duplicate panic/runtime symbols.
    """
    selected: list[Path] = []
    selected_set: set[Path] = set()

    def add(path: Path) -> None:
        if path not in selected_set:
            selected.append(path)
            selected_set.add(path)

    for path in test_irs:
        add(path)

    skip_prefixes = {
        "addr2line-",
        "adler2-",
        "bencher-",
        "getopts-",
        "gimli-",
        "memchr-",
        "miniz_oxide-",
        "object-",
        "panic_abort-",
        "panic_unwind-",
        "proc_macro-",
        "rustc_demangle-",
        "rustc_std_workspace_",
        "std_detect-",
        "unicode_width-",
        "unwind-",
    }

    for path in sorted(deps_dir.glob("*.ll")):
        if path in selected_set:
            continue
        name = path.name
        if any(name.startswith(prefix) for prefix in skip_prefixes):
            continue
        if not build_std and name.startswith(("alloc-", "compiler_builtins-", "core-", "std-", "test-")):
            continue
        if _llvm_ir_defines_main(path):
            continue
        add(path)

    if main_ir not in selected_set and not _llvm_ir_defines_main(main_ir):
        add(main_ir)

    logger.info(
        "Selected test LLVM IR paths: %s",
        [path.name for path in selected],
    )
    return selected

def _link_llvm_irs(llvm_ir_paths: list[Path], output_path: Path, bitcode: bool = False) -> None:
    """Link multiple LLVM IR files into one using llvm-link."""
    cmd = ["llvm-link-20", "-o", str(output_path)] + [str(p) for p in llvm_ir_paths]
    if not bitcode:
        cmd.append("-S")
    logger.info(f"Running command: {cmd}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(
            f"Failed to link LLVM IR files:\n"
            f"stdout: {result.stdout}\n"
            f"stderr: {result.stderr}"
        )


def collect_llvm_irs(cargo_dir: Path, target_triple: str | None = None) -> list[Path]:
    """Collect all emitted .ll files for a crate or workspace rooted at *cargo_dir*.

    When *target_triple* is given (e.g. "aarch64-unknown-linux-gnu") the search
    uses ``target/<triple>/debug/deps/`` instead of ``target/debug/deps/``,
    which is where cargo places outputs when ``--target`` is specified.
    """
    targets = _get_crate_name_or_workspace_members(cargo_dir)
    logger.info(f"Found target names: {[n for n, _ in targets]}")

    if target_triple:
        deps_dir = cargo_dir / "target" / target_triple / "debug" / "deps"
    else:
        deps_dir = cargo_dir / "target" / "debug" / "deps"
    llvm_ir_paths: list[Path] = []

    for crate_name, _ in targets:
        actual_name = crate_name.replace("-", "_")
        logger.info(f"Searching for LLVM IR for crate '{actual_name}'")
        path = _find_llvm_ir(deps_dir, actual_name)
        llvm_ir_paths.append(path)

    logger.info(f"Found LLVM IR paths: {llvm_ir_paths}")
    return llvm_ir_paths


def _resolve_target_triple(build_std: bool) -> str | None:
    if not build_std:
        return None
    import platform

    machine = platform.machine().lower()
    arch = "x86_64" if machine in ("x86_64", "amd64") else machine
    return f"{arch}-unknown-linux-gnu"


def ensure_linked_llvm_ir_file(
    *,
    cargo_dir: Path,
    output_dir: Path,
    rustc: str | None = None,
    test: bool = False,
    build_std: bool = True,
    panic_abort: bool = False,
    force: bool = False,
) -> Path:
    cargo_dir = cargo_dir.resolve()
    output_dir = output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)

    members = _get_crate_name_or_workspace_members(cargo_dir)
    if not members:
        raise RuntimeError(f"Could not determine crate name(s) from {cargo_dir}")
    crate_name = members[0][0].replace("-", "_")
    output_path = output_dir / f"{crate_name}.ll"
    build_config = {
        "rustflags": _emit_llvm_rustflags(
            panic_abort=panic_abort,
            panic_abort_tests=test and panic_abort,
        ),
        "rustc": rustc,
        "test": test,
        "build_std": build_std,
        "build_std_args": _build_std_args(test=test, panic_abort=panic_abort)
        if build_std
        else [],
        "panic_abort_tests": test and panic_abort,
    }
    metadata_path = output_dir / f"{crate_name}.ll.meta.json"
    if output_path.is_file() and not force:
        try:
            cached_config = json.loads(metadata_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            cached_config = None
        if cached_config == build_config:
            return output_path

    if output_path.is_file() and not force and not panic_abort:
        return output_path

    target_triple = _resolve_target_triple(build_std)

    if test:
        compile_test_with_emit_llvm(
            cargo_dir,
            custom_rustc=rustc,
            build_std=build_std,
            panic_abort=panic_abort,
        )
    else:
        compile_with_emit_llvm(
            cargo_dir,
            custom_rustc=rustc,
            build_std=build_std,
            panic_abort=panic_abort,
        )

    if target_triple:
        all_deps_dir = cargo_dir / "target" / target_triple / "debug" / "deps"
    else:
        all_deps_dir = cargo_dir / "target" / "debug" / "deps"

    lls: list[Path] = []
    main_ir = _find_llvm_ir(all_deps_dir, crate_name)
    if test:
        test_irs = _find_test_llvm_irs(all_deps_dir, main_ir, crate_name)
        if test_irs:
            lls.extend(
                _collect_test_link_llvm_irs(
                    all_deps_dir,
                    test_irs=test_irs,
                    main_ir=main_ir,
                    crate_name=crate_name,
                    build_std=build_std,
                )
            )
        else:
            lls.append(main_ir)
    else:
        lls.append(main_ir)
    if build_std:
        for link in ("core", "alloc", "std", "compiler_builtins"):
            try:
                link_path = _find_llvm_ir(all_deps_dir, link)
                if link_path not in lls:
                    lls.append(link_path)
            except FileNotFoundError:
                logger.warning(f"Could not find LLVM IR for '{link}' in {all_deps_dir}, skipping it.")

    _link_llvm_irs(lls, output_path)
    metadata_path.write_text(json.dumps(build_config, sort_keys=True), encoding="utf-8")
    return output_path

def run(args: argparse.Namespace) -> int:
    cargo_dir = Path(args.cargo_dir).resolve()
    if not cargo_dir.is_dir():
        print(f"Error: '{cargo_dir}' is not a directory.")
        return 1
    out_dir = Path(args.output_dir).resolve() if args.output_dir else None
    if out_dir and not out_dir.exists():
        try:
            out_dir.mkdir(parents=True)
            logger.info(f"Created output directory '{out_dir}'")
        except OSError as e:
            print(f"Error: Failed to create output directory '{out_dir}': {e}")
            return 1


    build_std = getattr(args, "build_std", True)
    try:
        members = _get_crate_name_or_workspace_members(cargo_dir)
        if not members:
            raise RuntimeError(f"Could not determine crate name(s) from {cargo_dir}")

        for member_name, member_dir in members:
            output_root = out_dir if out_dir else cargo_dir
            output_path = ensure_linked_llvm_ir_file(
                cargo_dir=member_dir,
                output_dir=output_root,
                rustc=args.rustc,
                test=args.test,
                build_std=build_std,
                force=True,
            )
            logger.info(f"Wrote linked LLVM IR for '{member_name}' to {output_path}")
    except Exception as e:
        print(f"Error: {e}")
        return 1

    return 0
