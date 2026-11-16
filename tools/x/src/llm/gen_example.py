import asyncio
from dataclasses import dataclass
import json
import math
import os
from pathlib import Path
import re
import subprocess
from typing import Any


GENERATED_TESTCASE_BEGIN = "// UNSAT-GENERATED-TESTCASE-BEGIN:"
GENERATED_TESTCASE_END = "// UNSAT-GENERATED-TESTCASE-END:"


@dataclass(frozen=True)
class RustContext:
    text: str
    stats: dict[str, Any]


@dataclass(frozen=True)
class _FunctionSpan:
    path: Path
    name: str
    start: int
    body_start: int
    body_end: int
    start_line: int
    end_line: int


def get_rust_file_paths(root: str | Path) -> list[Path]:
    root = Path(root).resolve()
    if not root.is_dir():
        raise ValueError(f"Not a directory: {root}")

    return sorted(
        p for p in root.rglob("*.rs")
        if p.is_file()
        and "target" not in p.parts
        and ".git" not in p.parts
    )


def read_rust_files_as_context(
    root: str | Path, *, exclude_generated_testcases: bool = False
) -> str:
    root = Path(root).resolve()
    files = get_rust_file_paths(root)

    chunks: list[str] = ["<rust context>"]
    for path in files:
        rel = path.relative_to(root)
        content = path.read_text(encoding="utf-8", errors="replace")
        if exclude_generated_testcases:
            content = _without_generated_testcases(content)
        content = _without_cfg_test_items(content)
        chunks.append(
            f"\n<file path=\"{rel}\">\n"
            f"```rust\n{content}\n```\n"
            f"</file>"
        )

    chunks.append("</rust context>")
    return "\n".join(chunks)


def _mask_rust_non_code(source: str) -> str:
    """Replace comments and literals with spaces while retaining byte offsets."""
    masked = list(source)
    length = len(source)

    def blank(start: int, end: int) -> None:
        for offset in range(start, end):
            if masked[offset] != "\n":
                masked[offset] = " "

    i = 0
    while i < length:
        if source.startswith("//", i):
            end = source.find("\n", i + 2)
            end = length if end < 0 else end
            blank(i, end)
            i = end
            continue
        if source.startswith("/*", i):
            depth = 1
            end = i + 2
            while end < length and depth:
                if source.startswith("/*", end):
                    depth += 1
                    end += 2
                elif source.startswith("*/", end):
                    depth -= 1
                    end += 2
                else:
                    end += 1
            blank(i, end)
            i = end
            continue

        raw = re.match(r"(?:br|r)(?P<hashes>#{0,255})\"", source[i:])
        if raw and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")):
            terminator = '"' + raw.group("hashes")
            content_start = i + raw.end()
            found = source.find(terminator, content_start)
            end = length if found < 0 else found + len(terminator)
            blank(i, end)
            i = end
            continue

        if source[i] == '"':
            end = i + 1
            escaped = False
            while end < length:
                char = source[end]
                end += 1
                if char == '"' and not escaped:
                    break
                if char == "\n" and not escaped:
                    break
                escaped = char == "\\" and not escaped
                if char != "\\":
                    escaped = False
            blank(i, end)
            i = end
            continue

        # Mask character/byte-character literals, but leave lifetimes such as
        # `'a` and `'static` visible to the item scanner.
        quote = i + 1 if source.startswith("b'", i) else i
        if quote < length and source[quote] == "'":
            end = quote + 1
            escaped = False
            closing = -1
            while end < min(length, quote + 12) and source[end] != "\n":
                char = source[end]
                if char == "'" and not escaped:
                    closing = end + 1
                    break
                escaped = char == "\\" and not escaped
                if char != "\\":
                    escaped = False
                end += 1
            if closing > 0:
                blank(i, closing)
                i = closing
                continue
        i += 1
    return "".join(masked)


def _matching_brace(masked: str, opening: int) -> int | None:
    depth = 0
    for offset in range(opening, len(masked)):
        if masked[offset] == "{":
            depth += 1
        elif masked[offset] == "}":
            depth -= 1
            if depth == 0:
                return offset
    return None


def _function_spans(path: Path, source: str) -> list[_FunctionSpan]:
    masked = _mask_rust_non_code(source)
    spans: list[_FunctionSpan] = []
    for match in re.finditer(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)", masked):
        cursor = match.end()
        body_start: int | None = None
        paren_depth = 0
        bracket_depth = 0
        while cursor < len(masked):
            char = masked[cursor]
            if char == "(":
                paren_depth += 1
            elif char == ")":
                paren_depth = max(0, paren_depth - 1)
            elif char == "[":
                bracket_depth += 1
            elif char == "]":
                bracket_depth = max(0, bracket_depth - 1)
            elif char == ";" and paren_depth == 0 and bracket_depth == 0:
                break
            elif char == "{" and paren_depth == 0 and bracket_depth == 0:
                body_start = cursor
                break
            cursor += 1
        if body_start is None:
            continue
        body_end = _matching_brace(masked, body_start)
        if body_end is None:
            continue
        spans.append(
            _FunctionSpan(
                path=path,
                name=match.group(1),
                start=match.start(),
                body_start=body_start,
                body_end=body_end,
                start_line=source.count("\n", 0, match.start()) + 1,
                end_line=source.count("\n", 0, body_end) + 1,
            )
        )
    return spans


def _without_generated_testcases(source: str) -> str:
    pattern = re.compile(
        rf"(?m)^\s*{re.escape(GENERATED_TESTCASE_BEGIN)}.*?"
        rf"^\s*{re.escape(GENERATED_TESTCASE_END)}[^\n]*(?:\n|$)",
        re.S,
    )
    return pattern.sub("", source)


def _without_cfg_test_items(source: str) -> str:
    """Remove `#[cfg(test)]` items from prompt context.

    Testcase POCs are compiled with `cargo build --lib --features ...`, not with
    `cargo test`, so test-only helpers in the prompt are attractive but
    unavailable. Keep the generator honest by hiding those items from context.
    """
    masked = _mask_rust_non_code(source)
    ranges: list[tuple[int, int]] = []
    attr_pattern = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")
    for match in attr_pattern.finditer(masked):
        start = source.rfind("\n", 0, match.start()) + 1
        cursor = match.end()
        while cursor < len(masked) and masked[cursor].isspace():
            cursor += 1
        if cursor >= len(masked):
            ranges.append((start, len(source)))
            continue
        brace = masked.find("{", cursor)
        semi = masked.find(";", cursor)
        if semi != -1 and (brace == -1 or semi < brace):
            end = semi + 1
        elif brace != -1:
            close = _matching_brace(masked, brace)
            end = len(source) if close is None else close + 1
        else:
            end = len(source)
        newline = source.find("\n", end)
        ranges.append((start, len(source) if newline < 0 else newline + 1))

    result = source
    for start, end in sorted(ranges, reverse=True):
        result = result[:start] + result[end:]
    return result


def _terminal_rust_name(value: object) -> str | None:
    if not isinstance(value, str):
        return None
    identifiers = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", value)
    return identifiers[-1] if identifiers else None


def _semantic_seed_names(target: dict[str, Any], control_chains: list[dict[str, Any]]) -> set[str]:
    names: set[str] = set()
    for key in ("caller", "caller_parent"):
        entry = target.get(key)
        if isinstance(entry, dict):
            name = _terminal_rust_name(entry.get("name"))
            if name:
                names.add(name)
    for chain in control_chains:
        steps = chain.get("steps")
        if not isinstance(steps, list):
            continue
        for step in steps:
            if not isinstance(step, dict):
                continue
            name = _terminal_rust_name(step.get("function"))
            if name:
                names.add(name)
    return names


def _semantic_function_qualifiers(
    target: dict[str, Any], control_chains: list[dict[str, Any]]
) -> dict[str, set[str]]:
    result: dict[str, set[str]] = {}

    def add(value: object) -> None:
        if not isinstance(value, str):
            return
        identifiers = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", value)
        if not identifiers:
            return
        result.setdefault(identifiers[-1], set()).update(identifiers[:-1])

    caller = target.get("caller")
    if isinstance(caller, dict):
        add(caller.get("name"))
    for chain in control_chains:
        steps = chain.get("steps")
        if isinstance(steps, list):
            for step in steps:
                if isinstance(step, dict):
                    add(step.get("function"))
    return result


def _function_seed_matches(
    path: Path, name: str, qualifiers: dict[str, set[str]]
) -> bool:
    if name not in qualifiers:
        return False
    expected = qualifiers[name]
    if not expected:
        return True
    module_qualifiers = {
        value
        for value in expected
        if value == value.lower() and value not in {"crate", "self", "super"}
    }
    if not module_qualifiers:
        return True
    path_qualifiers = {path.stem, *(parent.name for parent in path.parents)}
    return not module_qualifiers.isdisjoint(path_qualifiers)


def _semantic_identifiers(
    target: dict[str, Any], control_chains: list[dict[str, Any]]
) -> set[str]:
    identifiers = set(_semantic_seed_names(target, control_chains))

    def add(value: object) -> None:
        if isinstance(value, list):
            for nested in value:
                add(nested)
        elif isinstance(value, str):
            identifiers.update(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", value))

    # Deliberately exclude the external unsafe callee's signature: names such
    # as `Output` or `Item` are common associated types and otherwise create
    # false links to unrelated local modules.
    for key in ("caller_parent", "caller"):
        entry = target.get(key)
        if isinstance(entry, dict):
            add(entry.get("name"))
            add(entry.get("return_ty"))
    type_args = target.get("callee_type_args")
    if isinstance(type_args, list):
        for type_arg in type_args:
            if isinstance(type_arg, dict):
                add(type_arg.get("instantiated_ty"))
                add(type_arg.get("external_sources"))
    for chain in control_chains:
        steps = chain.get("steps")
        if isinstance(steps, list):
            for step in steps:
                if isinstance(step, dict):
                    add(step.get("root_type"))
                    add(step.get("function"))
    return identifiers


def _select_relevant_source_files(
    *, root: Path, candidates: list[Path], target_path: Path | None,
    target: dict[str, Any], control_chains: list[dict[str, Any]],
    retained_names: set[str], retained_function_qualifiers: dict[str, set[str]],
) -> tuple[list[Path], set[str]]:
    """Compute a file-level type-definition closure around the witness path."""
    sources = {
        path: _without_generated_testcases(
            path.read_text(encoding="utf-8", errors="replace")
        )
        for path in candidates
    }
    type_definitions: dict[str, set[Path]] = {}
    function_definitions: dict[str, set[Path]] = {}
    for path, source in sources.items():
        masked = _mask_rust_non_code(source)
        for match in re.finditer(
            r"\b(?:struct|enum|union|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)",
            masked,
        ):
            type_definitions.setdefault(match.group(1), set()).add(path)
        for span in _function_spans(path, source):
            function_definitions.setdefault(span.name, set()).add(path)

    selected: set[Path] = set()
    if target_path is not None and target_path in sources:
        selected.add(target_path)
    for root_name in ("lib.rs", "main.rs"):
        root_file = root / "src" / root_name
        if root_file in sources:
            selected.add(root_file)
    if target_path is not None:
        parent = target_path.parent
        while root in parent.parents or parent == root:
            module_file = parent / "mod.rs"
            if module_file in sources:
                selected.add(module_file)
            if parent == root:
                break
            parent = parent.parent

    identifiers = _semantic_identifiers(target, control_chains)
    for name in identifiers:
        selected.update(type_definitions.get(name, set()))
    for name in retained_names:
        definitions = function_definitions.get(name, set())
        matched = {
            path
            for path in definitions
            if _function_seed_matches(path, name, retained_function_qualifiers)
        }
        selected.update(matched or (definitions if len(definitions) == 1 else set()))

    # Close over user-defined type names appearing in retained files. Two
    # rounds capture common wrapper -> state -> leaf layouts without pulling
    # in unrelated modules through implementation-local identifiers.
    for _ in range(2):
        discovered: set[str] = set()
        qualified: set[tuple[str, str]] = set()
        for path in selected:
            if path != target_path and path.name in {"lib.rs", "main.rs", "mod.rs"}:
                continue
            source = sources[path]
            interface, _ = _slice_source(
                path=path,
                source=source,
                retained_names=retained_names,
                retained_function_qualifiers=retained_function_qualifiers,
                target_path=target_path,
                target_line=None,
            )
            masked_interface = _mask_rust_non_code(interface)
            discovered.update(
                re.findall(r"\b[A-Z][A-Za-z0-9_]*\b", masked_interface)
            )
            qualified.update(
                re.findall(
                    r"\b([a-z_][A-Za-z0-9_]*)\s*::\s*([A-Z][A-Za-z0-9_]*)\b",
                    masked_interface,
                )
            )
        before = len(selected)
        for name in discovered:
            definitions = type_definitions.get(name, set())
            if len(definitions) == 1:
                selected.update(definitions)
        for module, name in qualified:
            selected.update(
                path
                for path in type_definitions.get(name, set())
                if path.stem == module or path.parent.name == module
            )
        identifiers.update(discovered)
        if len(selected) == before:
            break

    # Missing metadata should degrade to the conservative source set rather
    # than silently producing an empty prompt.
    if not selected:
        selected.update(candidates)
    return sorted(selected), identifiers


def _slice_source(
    *, path: Path, source: str, retained_names: set[str], target_path: Path | None,
    target_line: int | None,
    retained_function_qualifiers: dict[str, set[str]] | None = None,
) -> tuple[str, list[str]]:
    spans = _function_spans(path, source)
    retained_bodies: list[str] = []
    omitted: list[_FunctionSpan] = []
    for span in spans:
        is_target = (
            target_path is not None
            and path == target_path
            and target_line is not None
            and span.start_line <= target_line <= span.end_line
        )
        named_seed = (
            _function_seed_matches(path, span.name, retained_function_qualifiers)
            if retained_function_qualifiers is not None
            else span.name in retained_names
        )
        if named_seed or is_target:
            retained_bodies.append(span.name)
        else:
            omitted.append(span)

    # Work backwards so offsets found in the original source remain valid.
    result = source
    for span in sorted(omitted, key=lambda item: item.body_start, reverse=True):
        line_start = result.rfind("\n", 0, span.body_start) + 1
        indent = re.match(r"[ \t]*", result[line_start:span.body_start]).group(0)
        replacement = "{\n" + indent + "    /* body omitted: interface retained */\n" + indent + "}"
        result = result[:span.body_start] + replacement + result[span.body_end + 1:]
    result = re.sub(r"\n{4,}", "\n\n\n", result).strip() + "\n"
    return result, sorted(set(retained_bodies))


def build_witness_guided_rust_context(
    root: str | Path,
    *,
    target: dict[str, Any],
    control_chains: list[dict[str, Any]],
    mode: str = "slice",
) -> RustContext:
    """Build the LLM context used for concrete witness realization.

    Slice mode keeps the target and certainty-chain function bodies, while all
    other functions are reduced to interfaces. Only library sources under
    ``src/`` are included. Full mode preserves the historical all-Rust-files
    behavior and is used as a conservative retry fallback.
    """
    root = Path(root).resolve()
    if mode not in {"slice", "full"}:
        raise ValueError(f"unsupported Rust context mode: {mode}")

    baseline_files = get_rust_file_paths(root)
    baseline_text = read_rust_files_as_context(
        root, exclude_generated_testcases=True
    )
    if mode == "full":
        chars = len(baseline_text)
        return RustContext(
            text=baseline_text,
            stats={
                "schema_version": 1,
                "mode": "full",
                "baseline_file_count": len(baseline_files),
                "retained_file_count": len(baseline_files),
                "retained_files": [str(path.relative_to(root)) for path in baseline_files],
                "baseline_chars": chars,
                "context_chars": chars,
                "char_reduction_ratio": 0.0,
                "estimated_baseline_tokens": math.ceil(chars / 4),
                "estimated_context_tokens": math.ceil(chars / 4),
                "retained_function_bodies": [],
            },
        )

    callsite = target.get("callsite")
    callsite = callsite if isinstance(callsite, dict) else {}
    raw_target_path = callsite.get("path")
    target_path = (
        (root / raw_target_path).resolve()
        if isinstance(raw_target_path, str)
        else None
    )
    target_line = callsite.get("line") if isinstance(callsite.get("line"), int) else None

    src_root = root / "src"
    retained_names = _semantic_seed_names(target, control_chains)
    retained_function_qualifiers = _semantic_function_qualifiers(
        target, control_chains
    )
    candidates = get_rust_file_paths(src_root) if src_root.is_dir() else baseline_files
    if target_path is not None and target_path.is_file() and target_path not in candidates:
        candidates.append(target_path)
        candidates.sort()
    source_files, semantic_identifiers = _select_relevant_source_files(
        root=root,
        candidates=candidates,
        target_path=target_path,
        target=target,
        control_chains=control_chains,
        retained_names=retained_names,
        retained_function_qualifiers=retained_function_qualifiers,
    )
    chunks = [
        '<rust context mode="witness-guided-semantic-slice">',
        "<context policy>",
        "Function bodies marked `body omitted: interface retained` are intentionally elided. ",
        "All type shapes and function signatures remain available. Bodies are retained for ",
        "the target callsite and the KLEE certainty-chain operations.",
        "</context policy>",
    ]
    retained_bodies: dict[str, list[str]] = {}
    for path in source_files:
        rel = path.relative_to(root)
        source = _without_generated_testcases(
            path.read_text(encoding="utf-8", errors="replace")
        )
        source = _without_cfg_test_items(source)
        sliced, retained = _slice_source(
            path=path,
            source=source,
            retained_names=retained_names,
            retained_function_qualifiers=retained_function_qualifiers,
            target_path=target_path,
            target_line=target_line,
        )
        if retained:
            retained_bodies[str(rel)] = retained
        chunks.append(
            f'\n<file path="{rel}">\n```rust\n{sliced}```\n</file>'
        )
    chunks.append("</rust context>")
    text = "\n".join(chunks)
    baseline_chars = len(baseline_text)
    context_chars = len(text)
    ratio = (
        1.0 - (context_chars / baseline_chars)
        if baseline_chars
        else 0.0
    )
    return RustContext(
        text=text,
        stats={
            "schema_version": 1,
            "mode": "slice",
            "baseline_file_count": len(baseline_files),
            "available_library_file_count": len(candidates),
            "retained_file_count": len(source_files),
            "retained_files": [str(path.relative_to(root)) for path in source_files],
            "baseline_chars": baseline_chars,
            "context_chars": context_chars,
            "char_reduction_ratio": round(ratio, 6),
            "estimated_baseline_tokens": math.ceil(baseline_chars / 4),
            "estimated_context_tokens": math.ceil(context_chars / 4),
            "semantic_seed_names": sorted(retained_names),
            "semantic_identifiers": sorted(semantic_identifiers),
            "retained_function_bodies": retained_bodies,
        },
    )


def crate_api_slice_function_names(
    target: dict[str, Any], control_chains: list[dict[str, Any]]
) -> list[str]:
    """Return the target/call-chain function names used to seed mirscan."""
    names: set[str] = set()
    for key in ("caller", "caller_parent"):
        value = target.get(key)
        if isinstance(value, dict) and isinstance(value.get("name"), str):
            names.add(value["name"])
    for chain in control_chains:
        steps = chain.get("steps")
        if not isinstance(steps, list):
            continue
        for step in steps:
            if isinstance(step, dict) and isinstance(step.get("function"), str):
                names.add(step["function"])
    return sorted(names)


def render_crate_api_slice(data: dict[str, Any]) -> RustContext:
    """Render mirscan's crate-local API/type closure for an LLM prompt."""
    functions = data.get("functions") if isinstance(data.get("functions"), list) else []
    types = data.get("types") if isinstance(data.get("types"), list) else []
    chunks = [
        '<rust context mode="crate-local-api-slice">',
        "<context policy>",
        "Only crate-local types and APIs related to the requested target/call-chain functions are shown. ",
        "Bodies are included only for target/call-chain functions; related-type APIs retain signatures only. ",
        "Standard-library APIs are intentionally omitted.",
        "</context policy>",
    ]
    for item in types:
        if not isinstance(item, dict) or not isinstance(item.get("definition"), str):
            continue
        chunks.append(
            f'\n<type name="{item.get("name", "")}" path="{item.get("path", "")}" '
            f'line="{item.get("line", "")}">\n```rust\n{item["definition"]}\n```\n</type>'
        )
    for item in functions:
        if not isinstance(item, dict):
            continue
        source = item.get("body") or item.get("signature")
        if not isinstance(source, str) or not source.strip():
            continue
        chunks.append(
            f'\n<function name="{item.get("name", "")}" role="{item.get("role", "")}" '
            f'path="{item.get("path", "")}" line="{item.get("line", "")}">\n'
            f'```rust\n{source}\n```\n</function>'
        )
    chunks.append("</rust context>")
    text = "\n".join(chunks)
    return RustContext(
        text=text,
        stats={
            "schema_version": 1,
            "mode": "crate-local-api-slice",
            "requested_functions": data.get("requested_functions", []),
            "unresolved_functions": data.get("unresolved_functions", []),
            "retained_function_count": len(functions),
            "retained_type_count": len(types),
            "context_chars": len(text),
            "estimated_context_tokens": math.ceil(len(text) / 4),
        },
    )


def build_crate_api_slice_context(
    root: str | Path,
    *,
    target: dict[str, Any],
    control_chains: list[dict[str, Any]],
    output_path: str | Path,
) -> RustContext:
    """Run mirscan's crate-local API slicer and render its cached JSON output."""
    root = Path(root).resolve()
    output_path = Path(output_path).resolve()
    requested = crate_api_slice_function_names(target, control_chains)
    if output_path.is_file():
        cached = json.loads(output_path.read_text(encoding="utf-8"))
        if cached.get("requested_functions") == requested:
            return render_crate_api_slice(cached)

    mirscan_rustc = os.environ.get("MIRSCAN_RUSTC")
    if not mirscan_rustc:
        raise RuntimeError("MIRSCAN_RUSTC is required for crate-local API slicing")
    output_path.parent.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env.pop("ANALYSIS_OUT", None)
    env["RUSTC"] = mirscan_rustc
    env["API_SLICE_OUT"] = str(output_path)
    env["API_SLICE_FUNCTIONS"] = json.dumps(requested)
    result = subprocess.run(
        ["cargo", "check", "--lib", "--quiet"],
        cwd=root,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0 or not output_path.is_file():
        detail = (result.stderr or result.stdout).strip()[-4000:]
        raise RuntimeError(
            f"crate-local API slicing failed (exit {result.returncode}): {detail}"
        )
    return render_crate_api_slice(json.loads(output_path.read_text(encoding="utf-8")))


def build_final_prompt(
    rust_context: str,
    struct_method: str,
    unsafe_api: str,
    safety_requirement: str,
) -> str:
    return f"""
You are analyzing Rust library soundness.

Given:
- <rust context>: the source code context
- <struct method>: a public safe struct method

Task:
Produce a runnable minimal Rust program, preferably a single `main.rs`, that
uses only public safe functions from the library API and exercises a concrete
counterexample through the target safe caller.

Rules:
- Do not call unsafe code in the generated example.
- Do not rely on modifying private fields directly.
- The program should be concrete and runnable.
- Explain briefly why the chosen caller arguments/state form the
  counterexample.
- If no exploit is possible from the provided public safe API, say so clearly and explain the blocker.

{rust_context}

<struct method>
{struct_method}
</struct method>
""".strip()


def build_testcase_prompt(
    rust_context: str,
    call_chain: str,
    callsite: str,
    safety_requirement: str,
    function_name: str,
    feature_name: str,
    klee_witness: str | None = None,
    retry_feedback: str | None = None,
    reproduction_plans: str | None = None,
    target_context: str | None = None,
    no_std: bool = False,
) -> str:
    no_std_block = ""
    if no_std:
        no_std_block = (
            "\n\t- The crate is `#![no_std]`: `Vec`, `vec!`, `String`, `Box`, and "
            "`format!` are not in scope. Use arrays and slices, or import `alloc` "
            "items only if the crate already declares `extern crate alloc`."
        )
    witness_block = ""
    if klee_witness and klee_witness.strip():
        witness_block = f"""

<klee init witness>
{klee_witness.strip()}
</klee init witness>
"""
    feedback_block = ""
    hard_feedback_block = ""
    if retry_feedback:
        external_calls = sorted(
            {
                match.group(1)
                for match in re.finditer(
                    r"(?:external call with symbolic argument|failed external call)\s*:\s*([A-Za-z_][A-Za-z0-9_:]*)",
                    retry_feedback,
                )
            }
        )
        external_guidance = ""
        if external_calls:
            names = ", ".join(f"`{name}`" for name in external_calls[:8])
            external_guidance = f"""

The previous attempt was blocked before the target by external function(s):
{names}. In this attempt, do not call constructors, wrappers, destructors, or
helpers that invoke those external symbols. Prefer constructing the target
caller receiver/arguments from fields visible in the injection module, or a
small local safe helper inside `{function_name}`. If a value's destructor would
call one of those external symbols after the target caller returns, use safe
`std::mem::forget` after exercising the target caller so cleanup does not
mask the target result.
"""
        feedback_block = f"""

<previous attempt feedback>
The previous testcase attempt failed. Fix the generated testcase using this
compiler/tool feedback. Do not repeat the same mistake.

{retry_feedback}
{external_guidance}
</previous attempt feedback>
"""
        hard_lines: list[str] = []
        for raw_line in retry_feedback.splitlines():
            line = " ".join(raw_line.strip().split())
            if not line:
                continue
            lower = line.lower()
            if (
                "hard retry constraint" in lower
                or "in the next testcase" in lower
                or "this failed observation had" in lower
                or "target counterexample query was unsat" in lower
                or "do not make all" in lower
                or "zero offset/count" in lower
            ):
                hard_lines.append(line)
            if len(hard_lines) >= 12:
                break
        if hard_lines:
            hard_feedback_block = (
                "\n\n<high priority retry constraints>\n"
                "These constraints come from the immediately previous failed "
                "compile/KLEE attempt and override examples or ordinary-looking "
                "safe values. The next testcase must visibly change these "
                "values in the generated Rust source:\n- "
                + "\n- ".join(hard_lines)
                + "\n</high priority retry constraints>"
            )
    plans_block = reproduction_plans or "No candidate safe-call plans were available."
    target_context_block = ""
    if target_context and target_context.strip():
        target_context_block = f"""

<target caller and target-call context>
{target_context.strip()}
</target caller and target-call context>
"""
    return f"""
Generate one small in-crate Rust testcase that concretely reproduces the
target counterexample.
{hard_feedback_block}
Requirements:
- Use only safe Rust: no unsafe blocks, `unsafe fn` calls, or FFI calls that
  need `unsafe`. The token `unsafe` must not appear anywhere in the output
  (comments and strings included), except `#[unsafe(no_mangle)]` when Rust 2024
  requires it. If a call is rejected as unsafe, build the receiver/arguments
  from visible fields, safe local values, or safe pointer-producing methods such
  as `as_ptr`/`as_mut_ptr` instead.{no_std_block}
- The testcase is appended to the module containing the target callsite and
  compiled with `cargo build --lib --features {feature_name}` (not `cargo test`).
  It may use fields and types visible from that module. Do not use items gated
  by `#[cfg(test)]`; recreate any needed state with non-test constructors,
  visible fields, or a small local safe helper.
- Paths must compile from that module: prefer `crate::...`, `super::...`, or
  names already visible there; never import the current crate by package name.
  Keep nested module paths exactly as shown in the context or compiler feedback,
  and use dependency paths as visible in the surrounding source.
- Gate the function with exactly `#[cfg(feature = "{feature_name}")]` and
  define `pub extern "C" fn {function_name}()` with no arguments, using
  `#[no_mangle]`. Do not define `main` or use `#[test]`.
- Reproduce the certainty call chain literally, with concrete values for every
  input (no KLEE helpers or symbolic inputs). Keep earlier steps valid so
  execution reaches the target callsite.
- Call the most direct safe function that contains the target call. If the
  target context lists a "Direct actual-containing safe function option" and it
  type-checks from the injection module, use that direct function.
  Do not fall back to the root wrapper merely because its location is shown;
  otherwise call the caller named in the callsite metadata. Candidate safe-call plans are
  suggestions; if a constructor or wrapper may call FFI, validate inputs away,
  panic, or return before the target, build the receiver from visible fields.

Choosing values:
- The KLEE witness and target context override examples in the Rust context.
  Work backward from the target call's arguments to the caller's source
  expressions, and set that receiver field/parameter directly at the boundary
  value the witness implies (empty, zero length, one element, index == length).
  If a bound-argument map is given (`bind_arg_*(1, X)` means `get_arg(1)` is
  `X`), reproduce the counterexample for those mapped expressions.
- Length, count, capacity, index, and pointer inputs are independent; do not
  assume they match the size of a local container, or that invariants such as
  `stored length == allocation length` or `field index == Vec slot` hold unless
  the caller enforces them. Deliberately inconsistent but type-correct state is
  often required.
- Read the caller's guards and arithmetic first. Try edge values (`0`, `1`,
  `len - 1`, `len`, `len + 1`, `usize::MAX`) that still pass the guards;
  unsigned arithmetic may wrap in optimized builds. If an earlier attempt
  reached the target without the counterexample, change the values feeding the
  target expression. For pointer `add`/`offset` targets, avoid a zero offset
  unless the witness requires it.
- If the witness requires a length/count/capacity to be `0` or another
  boundary, do not build a larger container for that field.
- Raw-pointer fields read or written before the target need small safe backing
  storage (local arrays/Vecs), not null or dangling placeholders. Size prefix
  backing only as needed to reach the target; keep it separate from the target
  field, which must violate the relation. A `*mut *mut T` field needs an array
  of pointers to separate backing buffers, not a cast of a data buffer.
- Values decoded from selected collection elements before the target must be
  valid for that decode and actually stored in the element the prefix reads;
  do not leave the collection empty or use placeholder bytes.
- `Vec::clear`, `reserve`, and `reserve_exact` never shrink capacity. To get a
  smaller capacity through an accessor returning `&mut Vec<T>`, replace the Vec
  (`*receiver.data_mut() = replacement`) and track final length and capacity.
- Return only the function in one Rust fenced code block.

The testcase passes only if KLEE reports the exact target callsite and the
counterexample is reproduced. A panic, `None`/`Err`, early return, or a
different target call is a failure. If the target was reached without the
counterexample, generate a different safe testcase for the same caller and
callsite.

<certainty call chain>
{call_chain}
</certainty call chain>

<target callsite metadata>
{callsite}
</target callsite metadata>

<candidate safe-call plans>
{plans_block}
</candidate safe-call plans>
{target_context_block}{witness_block}{feedback_block}
{rust_context}
""".strip()
