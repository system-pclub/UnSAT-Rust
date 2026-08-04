import argparse
import json
import logging
import os
from pathlib import Path
import re
import subprocess
import tomllib

logger = logging.getLogger(__name__)

def _emit_llvm_rustflags(
    *,
    panic_abort: bool = False,
    panic_abort_tests: bool = False,
    overflow_checks: bool = False,
) -> str:
    flags = [
        "-Zinline-mir=no",
        "--emit=llvm-ir",
        "-Cllvm-args=--inline-threshold=0",
        "-Copt-level=0",
        "-Ccodegen-units=1",
        f"-Coverflow-checks={'on' if overflow_checks else 'off'}",
    ]
    if panic_abort:
        flags.append("-Cpanic=abort")
    if panic_abort_tests:
        flags.append("-Zpanic_abort_tests")
    return " ".join(flags)


def _build_std_args(*, test: bool = False, panic_abort: bool = False) -> list[str]:
    if test and panic_abort:
        return [
            "-Zbuild-std=std,panic_abort,test",
            (
                "-Zbuild-std-features="
                "std_detect_file_io,std_detect_dlsym_getauxval"
            ),
        ]
    if panic_abort:
        return [
            "-Zbuild-std=std,panic_abort",
            (
                "-Zbuild-std-features="
                "std_detect_file_io,std_detect_dlsym_getauxval"
            ),
        ]
    return ["-Zbuild-std"]


def compile_with_emit_llvm(
    cargo_dir: Path,
    custom_rustc: str = None,
    build_std: bool = False,
    panic_abort: bool = False,
    overflow_checks: bool = False,
    features: list[str] | None = None,
    log_path: Path | None = None,
) -> None:
    """Compile the crate at *cargo_dir* and emit LLVM IR (.ll) files.
    """
    import platform
    env = os.environ.copy()
    env["CARGO_INCREMENTAL"] = "0" 
    if custom_rustc:
        env["RUSTC"] = custom_rustc
    env["RUSTFLAGS"] = _emit_llvm_rustflags(
        panic_abort=panic_abort,
        overflow_checks=overflow_checks,
    )

    cmd = ["cargo", "build"]
    if (cargo_dir / "src" / "lib.rs").is_file():
        cmd.append("--lib")
    if features:
        cmd += ["--features", ",".join(features)]
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
    if log_path is not None:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        log_path.write_text(
            f"command: {cmd!r}\nreturncode: {result.returncode}\n"
            f"\n[stdout]\n{result.stdout}\n[stderr]\n{result.stderr}",
            encoding="utf-8",
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
    overflow_checks: bool = False,
    features: list[str] | None = None,
    log_path: Path | None = None,
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
        overflow_checks=overflow_checks,
    )

    # Only the library unit-test harness is part of verify's execution model.
    # Building every test target can both introduce duplicate mains and make
    # LLVM IR generation depend on unrelated, broken integration tests.
    cmd = ["cargo", "test", "--lib", "--no-run"]
    if features:
        cmd += ["--features", ",".join(features)]
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
    if log_path is not None:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        log_path.write_text(
            f"command: {cmd!r}\nreturncode: {result.returncode}\n"
            f"\n[stdout]\n{result.stdout}\n[stderr]\n{result.stderr}",
            encoding="utf-8",
        )

    if result.returncode != 0:
        raise RuntimeError(
            f"Failed to compile test in '{cargo_dir}':\n"
            f"stdout: {result.stdout}\n"
            f"stderr: {result.stderr}"
        )
        
def _get_crate_name(cargo_dir: Path) -> str | None:
    """Return the linkable crate name from Cargo.toml, or None."""
    toml_path = cargo_dir / "Cargo.toml"
    try:
        with open(toml_path, "rb") as f:
            data = tomllib.load(f)
        lib_name = data.get("lib", {}).get("name")
        if lib_name:
            return lib_name
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


def _find_library_test_harness_llvm_ir(deps_dir: Path, crate_name: str) -> Path:
    """Find the package's library unit-test harness IR.

    ``cargo test --no-run`` may emit several independent test executables.  The
    library unit-test harness keeps the package crate name and defines ``main``;
    integration-test and binary-test harnesses use other target names.
    """
    matches = [
        entry
        for entry in deps_dir.iterdir()
        if entry.suffix == ".ll"
        and entry.name.startswith(crate_name + "-")
        and _llvm_ir_defines_main(entry)
    ]
    if matches:
        return max(matches, key=lambda path: path.stat().st_mtime)
    raise FileNotFoundError(
        f"Library unit-test harness LLVM IR not found for crate '{crate_name}' "
        f"in {deps_dir}"
    )


def _is_incompatible_panic_runtime(path: Path, *, panic_abort: bool) -> bool:
    if panic_abort:
        return path.name.startswith("panic_unwind-")
    return path.name.startswith("panic_abort-")


_LLVM_ARTIFACT_RE = re.compile(r"^(?P<name>.+)-[0-9a-f]{8,}$")


def _llvm_artifact_key(path: Path) -> str:
    """Return a stable crate-artifact key for a Cargo-emitted LLVM IR path."""
    match = _LLVM_ARTIFACT_RE.match(path.stem)
    if match:
        return match.group("name")
    return path.stem


def _dedupe_latest_llvm_irs(paths: list[Path]) -> list[Path]:
    """Keep only the newest LLVM IR for each Cargo artifact key.

    ``target/*/debug/deps`` is sticky across multiple ``cargo build`` and
    ``cargo test`` invocations.  A later feature-specific rerun can therefore
    leave several ``alloc-<hash>.ll``/``core-<hash>.ll``/crate ``.ll`` files in
    the directory.  Linking all of them creates duplicate definitions such as
    ``alloc::sync::STATIC_INNER_SLICE``.  Cargo artifact hashes are not semantic
    dependencies for llvm-link, so select the newest file per logical artifact.
    """
    latest_by_key: dict[str, Path] = {}
    for path in paths:
        key = _llvm_artifact_key(path)
        current = latest_by_key.get(key)
        if current is None or path.stat().st_mtime >= current.stat().st_mtime:
            latest_by_key[key] = path
    selected = set(latest_by_key.values())
    return [path for path in paths if path in selected]


def _collect_test_link_llvm_irs(
    deps_dir: Path,
    *,
    harness_ir: Path,
    panic_abort: bool,
) -> list[Path]:
    """Collect one library unit-test harness and every emitted library IR.

    Other unit/integration-test harnesses are separate executables and therefore
    bring another ``main``.  All non-harness IRs are linked, including build-std
    support libraries: those modules can own generic Rust monomorphizations
    referenced by the package.  Only the panic runtime incompatible with the
    selected compilation strategy is excluded.
    """
    selected = [harness_ir]
    candidates: list[Path] = []
    for path in sorted(deps_dir.glob("*.ll")):
        if path == harness_ir:
            continue
        if _llvm_ir_defines_main(path):
            continue
        if _is_incompatible_panic_runtime(path, panic_abort=panic_abort):
            continue
        candidates.append(path)
    selected.extend(_dedupe_latest_llvm_irs(candidates))

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
    overflow_checks: bool = False,
    force: bool = False,
    features: list[str] | None = None,
    build_log_path: Path | None = None,
) -> Path:
    cargo_dir = cargo_dir.resolve()
    output_dir = output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)

    members = _get_crate_name_or_workspace_members(cargo_dir)
    if not members:
        raise RuntimeError(f"Could not determine crate name(s) from {cargo_dir}")
    crate_name = members[0][0].replace("-", "_")
    output_path = output_dir / f"{crate_name}.ll"
    features = sorted(set(features or []))
    build_config = {
        "cargo_dir": str(cargo_dir),
        "rustflags": _emit_llvm_rustflags(
            panic_abort=panic_abort,
            panic_abort_tests=test and panic_abort,
            overflow_checks=overflow_checks,
        ),
        "rustc": rustc,
        "test": test,
        "build_std": build_std,
        "build_std_args": _build_std_args(test=test, panic_abort=panic_abort)
        if build_std
        else [],
        "panic_abort_tests": test and panic_abort,
        "overflow_checks": overflow_checks,
        "features": features,
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
            overflow_checks=overflow_checks,
            features=features,
            log_path=build_log_path,
        )
    else:
        compile_with_emit_llvm(
            cargo_dir,
            custom_rustc=rustc,
            build_std=build_std,
            panic_abort=panic_abort,
            overflow_checks=overflow_checks,
            features=features,
            log_path=build_log_path,
        )

    if target_triple:
        all_deps_dir = cargo_dir / "target" / target_triple / "debug" / "deps"
    else:
        all_deps_dir = cargo_dir / "target" / "debug" / "deps"

    lls: list[Path] = []
    if test:
        harness_ir = _find_library_test_harness_llvm_ir(all_deps_dir, crate_name)
        lls.extend(
            _collect_test_link_llvm_irs(
                all_deps_dir,
                harness_ir=harness_ir,
                panic_abort=panic_abort,
            )
        )
    else:
        main_ir = _find_llvm_ir(all_deps_dir, crate_name)
        # A library IR can contain calls to non-inlined dependency
        # monomorphizations (for example hashbrown helpers). Linking only the
        # package plus core/alloc/std leaves those as declarations and makes a
        # concrete KLEE rerun stop before reaching the target callsite.
        lls.append(main_ir)
        candidates: list[Path] = []
        for path in sorted(all_deps_dir.glob("*.ll")):
            if path == main_ir or _llvm_ir_defines_main(path):
                continue
            if _is_incompatible_panic_runtime(path, panic_abort=panic_abort):
                continue
            candidates.append(path)
        for path in _dedupe_latest_llvm_irs(candidates):
            if _llvm_artifact_key(path) == _llvm_artifact_key(main_ir):
                continue
            lls.append(path)

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
