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
) -> str:
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
	- Use only safe Rust. Do not use unsafe blocks, unsafe functions, or unsafe
	  operations. If the crate uses Rust 2024 and the compiler requires it, the
	  attribute `#[unsafe(no_mangle)]` is allowed; otherwise the token `unsafe`
	  must not occur in the output.
	- Do not call any function whose signature is declared `unsafe fn` or any
	  external FFI function that rustc requires to be called from an unsafe
	  block. If a previous compiler diagnostic says a call "is unsafe and
	  requires unsafe function or block", remove that call and construct the
	  target caller's receiver/arguments with safe visible fields, safe local
	  values, safe pointer-producing methods such as `as_ptr`/`as_mut_ptr`, or a
	  small local safe helper instead.
	- Unless the output needs `#[unsafe(no_mangle)]` for Rust 2024, the generated
	  code must not contain the token `unsafe` anywhere, including comments,
	  explanations, strings, or block bodies.
	- The testcase is compiled as part of the library with
	  `cargo build --lib --features {feature_name}`. It is not compiled with
	  `cargo test`.
	- Do not call, reference, copy, or depend on any item gated by `#[cfg(test)]`
	  or located inside a test-only module, even if such an item appeared in a
	  previous attempt or compiler diagnostic. If a test-only helper would have
	  been convenient, recreate the necessary state using non-test library
	  constructors, public fields visible from the injection module, or a small
	  local safe helper inside `{function_name}`.
	- The testcase is injected into the module containing the target callsite, so
	  it may use fields and types visible from that module. Reproduce the supplied
	  control-chain steps literally; do not replace them with unrelated APIs.
	- Because the testcase is compiled inside the library crate, do not import
	  the current crate through its package name (for example, avoid
	  `use package_name::...`). Prefer `crate::...`, `super::...`, or names
	  already visible from the injection module.
	- Preserve nested module paths for dependency or re-exported types. If the
	  context or compiler feedback shows `foo::bar::Type`,
	  `foo::{{bar::Type, Other}}`, or suggests importing `crate::x::bar::Type`,
	  use that exact nested path instead of guessing that `Type` is re-exported
	  from `foo` or `crate::x`. If the surrounding source file refers to a
	  dependency module directly as `foo::Type`, and the testcase is injected
	  into that same module, use that visible `foo::Type` path instead of
	  inventing `crate::foo::Type`.
	- If the target context lists visible `use` statements or direct dependency
	  paths from the injection module, those names are already in scope for the
	  generated function. Use the visible path exactly; for an external crate
	  dependency visible as `dep::Type`, do not write `crate::dep::Type`.
	- Candidate constructors are only suggestions, but a candidate direct
	  target-caller skeleton is the primary path when it type-checks. If a
	  constructor or wrapper can call external FFI, validate inputs away, panic,
	  or return before the target callsite, construct the target caller's
	  receiver/arguments from visible fields instead.
- If the target caller receiver or its nested fields contain raw pointers that
  are read or written before the target callsite, do not leave those prefix
  pointer fields as `null()`, `null_mut()`, or dangling placeholders. Use tiny
  local arrays, Vecs, or other safe pointer-producing values to provide just
  enough backing storage for prefix reads/writes needed to reach the target.
  This prefix backing is separate from the target counterexample relation; keep
  prefix-only state valid while choosing the actual target caller field/index/
  length/value that makes the target callsite a counterexample.
  If the same caller argument/index/count is used both by prefix pointer
  operations and by the target unsafe call, pick a small violating value for the
  target and make only the prefix backing large enough for that value. Do not
  make every raw-pointer field share the same backing size: a prefix field may
  need two elements to reach the target while the actual target field may need
  only one element so the target relation is false. For pointer arithmetic
  targets such as `add`/`offset`, an offset/count of zero often makes the target
  safety relation trivially true; if a previous observation showed the selected
  offset/count was `0`, use a concrete small non-zero value such as `1`, keep
  any earlier prefix receiver valid for that value, and make the target
  receiver/buffer relation false at that same value.
  If a visible receiver field has a pointer-to-pointer type such as
  `*mut *mut T`, construct it as an array/Vec of row/element pointers whose
  elements point at separate safe backing arrays/Vecs. Do not cast a `Vec<T>` or
  `[T; N]` element buffer directly to `*mut *mut T`; that creates pointer values
  from data bytes instead of a valid pointer array.
- The KLEE init witness and target context override examples in the Rust
  context. Do not imitate an unrelated example's container size, element count,
  indexes, or constructor choices when they do not reproduce the target
  counterexample.
- Before writing code, work backward from the target call arguments and the
  target caller's source expressions. If a witness argument comes from a `.len`, `.capacity`, slice
  length, pointer base, or receiver field, construct that field/receiver state
  directly at a small boundary value instead of adding elements just because an
  example did.
- Read the target caller's guards and arithmetic before choosing values. Try
  concrete edge values such as `0`, `1`, `len - 1`, `len`, `len + 1`,
  `usize::MAX`, and `usize::MAX - k` when they can still reach the target
  callsite. In optimized library builds, unsigned arithmetic may wrap; if a
  guard checks an expression like `a + b`, a wraparound pair can pass the guard
  while a later target expression using `a`, `b`, or the wrapped result becomes
  the counterexample.
- Do not stop at the first ordinary in-bounds value that reaches the target
  callsite. If a previous attempt reached the target but did not reproduce the
  counterexample, change the caller parameters or receiver fields that feed the
  selected target expression, especially values involved in earlier arithmetic
  or comparisons.
- If the target context contains an instrumented bound-argument map, use that
  map before choosing values. For example, `bind_arg_*(1, X)` means target
  argument `get_arg(1)` is `X`, and `bind_arg_*_value(2, &Y)` means target
  argument `get_arg(2)` is the current value of `Y`; generate the
  counterexample for the mapped source expressions at the actual target call.
- If the prefix path decodes a value from bytes selected out of a collection,
  lookup, or field (for example, `Type::decode(selected.clone()).expect(...)`),
  the selected bytes must be valid for that decode before the target is
  reached. Do not use placeholder byte strings for such selected elements.
  Prefer constructing a separate value of the decoded type with visible safe
  constructors/mutators, then place bytes from a visible safe raw/encode/accessor
  for that value into the selected collection element.
  If the selected element came from a receiver field or nested receiver
  collection, build that receiver with the selected collection non-empty at the
  chosen selector/index. A standalone local byte value or a decoded top-level
  value is not enough unless it is actually stored into the selected collection
  element read by the prefix. Do not use `Default`, `Vec::new()`, or `vec![]` as
  the final state for a receiver whose prefix must read an existing selected
  element.
- When the target context shows an inner target-call source line, identify the
  receiver and argument expressions on that exact line and make those expression
  values reproduce the target counterexample. If the target-call index/count/length comes from
  a visible struct field, array element, method result, or caller parameter,
  construct that source value directly; do not assume it must equal the element's
  position in a Vec or an earlier guarded lookup index unless the source code
  forces equality.
- In particular, do not preserve common data-structure invariants such as
  `field named index == Vec slot`, `stored length == allocation length`, or
  `neighbor id == current object id` unless the target caller enforces them
  before the target call. Counterexamples often require safe construction of a
  deliberately inconsistent but type-correct receiver state: keep the earlier
  values needed to reach the callsite valid, and make the actual target
  expression's field/argument form the counterexample.
- If the witness-derived target relation says a length/count/capacity/index
  must be `0`, `<= 0`, equal to another boundary, or otherwise at a small
  boundary to reproduce the counterexample, do not build a non-empty/non-boundary
  container for that same field. A testcase that reaches the callsite while the
  target expression is still on the safe side is invalid.
- If the target context shows a root wrapper location and a different inner
  target-call containing function, prefer the most direct safe function that can
  reach the inner target call and is callable from the injected module. When
  the actual-containing function is safe and in the same module, constructing
  its receiver/parameters and calling it directly is valid and often better
  than routing through a root wrapper that performs earlier guarded lookups,
  loop traversal, validation, or panic-prone setup.
- If the target context contains a "Direct actual-containing safe function
  option" and it type-checks from the injection module, use that direct function
  as the primary target caller. Do not fall back to the root wrapper merely
  because the metadata caller/root source location is shown; the root wrapper is
  only a fallback when the direct actual-containing safe function is not visible
  or cannot be called safely.
- If there is no separate actual-containing safe function, the target callsite
  metadata names the caller function to exercise. If that caller is safe and
  visible, call it directly instead of a higher-level wrapper unless doing so
  fails to type-check from the injection module.
- Gate the function with exactly `#[cfg(feature = "{feature_name}")]`.
- Define `pub extern "C" fn {function_name}()` with no arguments under that
  cfg, using either `#[no_mangle]` or, only when required by Rust 2024,
  `#[unsafe(no_mangle)]`. Do not define `main` and do not use `#[test]`.
	- Copy the feature gate and function name exactly. The first lines should be
	  exactly this shape, with the closing quote and bracket intact:
	  `#[cfg(feature = "{feature_name}")]`
	  `#[no_mangle]`
	  `pub extern "C" fn {function_name}() {{`
	- Exercise the target callsite through the certainty call chain. Use concrete
	  values for every input; do not use KLEE helpers or symbolic inputs.
	- Do not assume that length/count/capacity/index arguments must equal the
	  actual size of a local container. When the target caller accepts a pointer,
	  slice, raw buffer, length, count, capacity, or index as separate inputs,
	  choose these inputs independently and try boundary relationships that
	  reproduce the target counterexample while still reaching that callsite.
	- The function will be appended to the Rust file containing the callsite, so
	  use paths that compile from that module.
	- Return only the function in one Rust fenced code block.

<certainty call chain>
{call_chain}
</certainty call chain>

<target callsite metadata>
{callsite}
</target callsite metadata>

<candidate safe-call plans>
{plans_block}
</candidate safe-call plans>

{target_context_block}

{witness_block}

Important:
- Generate a counterexample for the target callsite. If the context identifies
  a direct actual-containing safe function, call that function directly when it
  is visible/type-checks from the injected module; otherwise use the target
  caller named in the metadata.
- Use the KLEE init witness, when present, as the target shape for the concrete
  testcase. If a call argument in the witness comes from a receiver field,
  container length/capacity, slice length, pointer, index, or other caller state,
  construct that caller state so the target call's arguments reproduce the
  counterexample. Boundary states such as empty containers, zero lengths, one-element
  containers, and indexes equal to a length are often the smallest useful
  choices; use whichever one is implied by the witness and still reaches the
  target callsite.
- The testcase is judged by whether KLEE reports the exact target callsite id.
  A panic, `None`/`Err`, early return, or a different target call before that
  site is a failed testcase even if the API call looks plausible.
- If previous KLEE feedback says the target callsite was reported but the
  counterexample was not reproduced, generate a different safe testcase for the same
  target caller and target callsite.

{feedback_block}

{rust_context}
""".strip()
