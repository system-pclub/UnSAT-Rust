import asyncio
from dataclasses import dataclass
import json
import math
from pathlib import Path
import re
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
- <unsafe api>: an unsafe API used internally
- <safety requirement>: the requirement that must hold for the unsafe API to be sound

Task:
Produce a runnable minimal Rust program, preferably a single `main.rs`, that uses only public safe functions from the library API and violates the safety requirement.

Rules:
- Do not call unsafe code in the generated example.
- Do not rely on modifying private fields directly.
- The program should be concrete and runnable.
- Explain briefly why the safety requirement is violated.
- If no exploit is possible from the provided public safe API, say so clearly and explain the blocker.

{rust_context}

<struct method>
{struct_method}
</struct method>

<unsafe api>
{unsafe_api}
</unsafe api>

<safety requirement>
{safety_requirement}
</safety requirement>
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
) -> str:
    feedback_block = ""
    if retry_feedback:
        feedback_block = f"""

<previous attempt feedback>
The previous testcase attempt failed. Fix the generated testcase using this
compiler/tool feedback. Do not repeat the same mistake.

{retry_feedback}
</previous attempt feedback>
"""
    return f"""
Generate one small in-crate Rust testcase that concretely reproduces the
reported soundness violation.

Requirements:
- Use only safe Rust. Do not use unsafe blocks, unsafe functions, or unsafe
  operations. If the crate uses Rust 2024 and the compiler requires it, the
  attribute `#[unsafe(no_mangle)]` is allowed; otherwise the token `unsafe`
  must not occur in the output.
- The testcase is injected into the module containing the target callsite, so
  it may use fields and types visible from that module. Reproduce the supplied
  control-chain steps literally; do not replace them with unrelated APIs.
- Gate the function with exactly `#[cfg(feature = "{feature_name}")]`.
- Define `pub extern "C" fn {function_name}()` with no arguments under that
  cfg, using either `#[no_mangle]` or, only when required by Rust 2024,
  `#[unsafe(no_mangle)]`. Do not define `main` and do not use `#[test]`.
- Exercise the target callsite through the certainty call chain. Use concrete
  values for every input; do not use KLEE helpers or symbolic inputs.
- For a writable integer field, assign a concrete value that violates the
  safety requirement (for example an out-of-bounds index), then invoke the
  safe caller containing the target callsite.
- The function will be appended to the Rust file containing the callsite, so
  use paths that compile from that module.
- Return only the function in one Rust fenced code block.

<certainty call chain>
{call_chain}
</certainty call chain>

<target callsite metadata>
{callsite}
</target callsite metadata>

<safety requirement>
{safety_requirement}
</safety requirement>

<structured reproduction target>
{klee_witness or "No additional structured target was available."}
</structured reproduction target>

Important:
- The testcase must make the target unsafe callee arguments violate the rule
  above.
- If the target says a relation such as `index < len` must be violated, choose
  concrete inputs that make `index >= len` at the unsafe callsite.
- If the target says two pointers must be from different allocations, do not
  use `ptr.wrapping_add(...)` from the same allocation; construct the pointer
  from a different safe allocation or safe API state if possible.
- Prefer reproducing the concrete argument relation described by the rule over
  writing a merely plausible API call.

{feedback_block}

{rust_context}
""".strip()
