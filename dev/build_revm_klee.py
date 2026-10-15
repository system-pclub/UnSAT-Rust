"""Rebuild changed KLEE units against the existing local LLVM 20 build.

This supports the relocated workspace without rewriting CMake's old absolute
paths. The original libraries/executable remain available until --install.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import shutil
import subprocess


ROOT = Path(__file__).resolve().parents[1]
KLEE = ROOT / "tools/klee"
BUILD = KLEE / "build"
OUT = ROOT / "eval/revm-formal-replay/klee-build"


def run(command, log):
    process = subprocess.run([str(c) for c in command], capture_output=True, text=True)
    log.write_text(repr(command) + "\n" + process.stdout + process.stderr)
    if process.returncode:
        raise RuntimeError(f"Command failed; see {log}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--install", action="store_true")
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    sources = ["lib/Core/Executor.cpp", "lib/Ext/ExtInfoManager.cpp", "lib/Ext/Dsl.cpp", "tools/klee/main.cpp"]
    flags = [
        "-D_GNU_SOURCE", "-D__STDC_CONSTANT_MACROS", "-D__STDC_FORMAT_MACROS", "-D__STDC_LIMIT_MACROS",
        f"-I{KLEE / 'include'}", f"-I{BUILD / 'include'}", "-I/usr/lib/llvm-20/include",
        f"-I{KLEE / 'lib/Core'}", f"-I{KLEE / 'lib/Snapshot'}",
        "-O3", "-DNDEBUG", "-std=gnu++17", "-Wall", "-Wextra", "-Wno-unused-parameter", "-UNDEBUG", "-fno-exceptions",
    ]

    def compile(source):
        path = KLEE / source
        obj = OUT / (path.name + ".o")
        if not obj.exists() or obj.stat().st_mtime < path.stat().st_mtime:
            run(["/usr/bin/c++", *flags, "-c", path, "-o", obj], OUT / (path.name + ".build.log"))
        return obj

    with ThreadPoolExecutor(max_workers=2) as pool:
        executor, manager, dsl, driver = list(pool.map(compile, sources))
    for library, objects in (("libkleeCore.a", [executor]), ("libkleeExt.a", [manager, dsl])):
        shutil.copy2(BUILD / "lib" / library, OUT / library)
        run(["ar", "r", OUT / library, *objects], OUT / (library + ".build.log"))
    core = OUT / "libkleeCore.a"
    run([
        "/usr/bin/c++", "-O3", "-DNDEBUG", driver, "-o", OUT / "klee",
        "-Wl,-rpath,/usr/lib/llvm-20/lib", core, OUT / "libkleeExt.a",
        BUILD / "lib/libkleeSnapshot.a", core, BUILD / "lib/libkleeModule.a",
        "/usr/lib/x86_64-linux-gnu/libsqlite3.so", BUILD / "lib/libkleaverSolver.a",
        BUILD / "lib/libkleeBasic.a", "/usr/lib/x86_64-linux-gnu/libz3.so",
        BUILD / "lib/libkleaverExpr.a", BUILD / "lib/libkleeSupport.a",
        "/usr/lib/llvm-20/lib/libLLVM.so.1", "/usr/lib/x86_64-linux-gnu/libz.so",
    ], OUT / "link.log")
    if args.install:
        backup = OUT / "klee-before-revm-fix"
        if not backup.exists():
            shutil.copy2(BUILD / "bin/klee", backup)
        shutil.copy2(OUT / "klee", BUILD / "bin/klee")
        print(f"Installed {BUILD / 'bin/klee'}; previous executable: {backup}")
    print(OUT / "klee")


if __name__ == "__main__":
    main()
