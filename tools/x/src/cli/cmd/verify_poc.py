import json
import hashlib
import logging
import re
import tomllib
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

from llm.gen_example import build_testcase_prompt, build_witness_guided_rust_context
from llm.openai_llm import OpenAILLM


logger = logging.getLogger(__name__)
INJECTION_STATE_FILE = ".unsat-test-injections.json"
BEGIN_MARKER = "// UNSAT-GENERATED-TESTCASE-BEGIN:"
END_MARKER = "// UNSAT-GENERATED-TESTCASE-END:"


@dataclass(frozen=True)
class TestcaseInjection:
    key: str
    callsite: str
    rule: str
    feature: str
    function: str
    source_path: str = ""
    line: int = 0


def _is_ident_continue(ch: str) -> bool:
    return ch == "_" or ch.isalnum()


def _raw_string_len(text: str, pos: int) -> int:
    start = pos
    if pos < len(text) and text[pos] == "b":
        pos += 1
    if pos >= len(text) or text[pos] != "r":
        return 0
    pos += 1
    hashes = 0
    while pos < len(text) and text[pos] == "#":
        hashes += 1
        pos += 1
    if pos >= len(text) or text[pos] != '"':
        return 0
    pos += 1
    end = text.find('"' + ("#" * hashes), pos)
    if end < 0:
        return len(text) - start
    return end + 1 + hashes - start


def _quoted_literal_len(text: str, pos: int, quote: str) -> int:
    i = pos + 1
    while i < len(text):
        if text[i] == "\\":
            i += 2
            continue
        if text[i] == quote:
            return i + 1 - pos
        if text[i] == "\n":
            return i - pos
        i += 1
    return len(text) - pos


def _line_comment_len(text: str, pos: int) -> int:
    end = text.find("\n", pos)
    return len(text) - pos if end < 0 else end - pos


def _block_comment_len(text: str, pos: int) -> int:
    depth = 1
    i = pos + 2
    while i < len(text) and depth > 0:
        if text.startswith("/*", i):
            depth += 1
            i += 2
        elif text.startswith("*/", i):
            depth -= 1
            i += 2
        else:
            i += 1
    return i - pos


def _number_literal_len(text: str, pos: int) -> int:
    i = pos
    if text.startswith(("0x", "0X", "0o", "0O", "0b", "0B"), pos):
        i += 2
        while i < len(text) and (text[i].isalnum() or text[i] == "_"):
            i += 1
        return i - pos

    while i < len(text) and (text[i].isdigit() or text[i] == "_"):
        i += 1
    if (
        i + 1 < len(text)
        and text[i] == "."
        and text[i + 1] != "."
        and text[i + 1].isdigit()
    ):
        i += 1
        while i < len(text) and (text[i].isdigit() or text[i] == "_"):
            i += 1
    if i < len(text) and text[i] in "eE":
        j = i + 1
        if j < len(text) and text[j] in "+-":
            j += 1
        if j < len(text) and text[j].isdigit():
            i = j + 1
            while i < len(text) and (text[i].isdigit() or text[i] == "_"):
                i += 1
    while i < len(text) and (text[i].isalnum() or text[i] == "_"):
        i += 1
    return i - pos


def _integer_literal_value(literal: str) -> int | None:
    match = re.match(
        r"(0[xX][0-9A-Fa-f_]+|0[oO][0-7_]+|0[bB][01_]+|[0-9][0-9_]*)(.*)\Z",
        literal,
    )
    if not match:
        return None
    suffix = match.group(2)
    if suffix.startswith((".", "e", "E", "f")):
        return None
    digits = match.group(1).replace("_", "")
    try:
        return int(digits, 0)
    except ValueError:
        return None


def _rerun_sym_integer_upper_bound(literal: str) -> int | None:
    value = _integer_literal_value(literal)
    if value is None:
        return None
    if value > 1_000_000:
        return None
    # Literals such as `0xE6` are often inferred as `u8` when placed in bytecode
    # vectors. Keeping small generated bounds within the byte range avoids
    # creating overflowing literals after symbolization, while still letting
    # usize offsets escape tiny allocations during rerun-sym.
    if value <= 0xFF:
        return 0xFF
    return max(16, value * 16)


def _array_const_ranges(text: str) -> list[tuple[int, int]]:
    """Return array-length regions, excluding macro brackets such as vec![...]."""
    ranges: list[tuple[int, int]] = []
    stack: list[dict[str, Any]] = []
    pairs = {")": "(", "]": "[", "}": "{"}
    i = 0
    while i < len(text):
        raw_len = _raw_string_len(text, i)
        if raw_len:
            i += raw_len
            continue
        if text.startswith("//", i):
            i += _line_comment_len(text, i)
            continue
        if text.startswith("/*", i):
            i += _block_comment_len(text, i)
            continue
        if text.startswith('b"', i):
            i += 1 + _quoted_literal_len(text, i + 1, '"')
            continue
        if text[i] == '"':
            i += _quoted_literal_len(text, i, '"')
            continue
        if text.startswith("b'", i):
            i += 1 + _quoted_literal_len(text, i + 1, "'")
            continue
        if text[i] == "'":
            i += _quoted_literal_len(text, i, "'")
            continue
        if text[i] in "([{":
            j = i - 1
            while j >= 0 and text[j].isspace():
                j -= 1
            stack.append(
                {
                    "delimiter": text[i],
                    "macro": text[i] == "[" and j >= 0 and text[j] == "!",
                    "semicolon": None,
                }
            )
        elif text[i] == ";" and stack and stack[-1]["delimiter"] == "[":
            stack[-1]["semicolon"] = i
        elif text[i] in pairs and stack and stack[-1]["delimiter"] == pairs[text[i]]:
            opened = stack.pop()
            semicolon = opened["semicolon"]
            if text[i] == "]" and semicolon is not None and not opened["macro"]:
                ranges.append((semicolon + 1, i))
        i += 1
    return ranges


def _find_function_body_span(testcase: str, function_name: str) -> tuple[int, int]:
    match = re.search(rf"\bfn\s+{re.escape(function_name)}\s*\(\s*\)", testcase)
    if not match:
        raise RuntimeError(f"could not find generated testcase function {function_name}()")
    open_brace = testcase.find("{", match.end())
    if open_brace < 0:
        raise RuntimeError(f"generated testcase function {function_name}() has no body")

    depth = 0
    i = open_brace
    while i < len(testcase):
        raw_len = _raw_string_len(testcase, i)
        if raw_len:
            i += raw_len
            continue
        if testcase.startswith("//", i):
            i += _line_comment_len(testcase, i)
            continue
        if testcase.startswith("/*", i):
            i += _block_comment_len(testcase, i)
            continue
        if testcase.startswith('b"', i):
            i += 1 + _quoted_literal_len(testcase, i + 1, '"')
            continue
        if testcase[i] == '"':
            i += _quoted_literal_len(testcase, i, '"')
            continue
        if testcase.startswith("b'", i):
            i += 1 + _quoted_literal_len(testcase, i + 1, "'")
            continue
        if testcase[i] == "'":
            i += _quoted_literal_len(testcase, i, "'")
            continue
        if testcase[i] == "{":
            depth += 1
        elif testcase[i] == "}":
            depth -= 1
            if depth == 0:
                return open_brace, i
        i += 1
    raise RuntimeError(f"generated testcase function {function_name}() body is unterminated")


_RERUN_SYM_FOCUS_STOPWORDS = {
    "as",
    "at",
    "bool",
    "call",
    "caller",
    "construct",
    "crate",
    "false",
    "field",
    "get_arg",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "len",
    "length",
    "mut",
    "rule",
    "self",
    "source",
    "target",
    "true",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "value",
}


def _rerun_sym_focus_terms(focus_text: str | None) -> set[str]:
    if not focus_text:
        return set()
    terms: set[str] = set()
    relation_match = re.search(
        r"Derived source-level target relation.*?so `([^`]+)` is true",
        focus_text,
        re.S,
    )
    relation = relation_match.group(1) if relation_match else ""
    relation_idents = {
        ident
        for ident in re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", relation)
        if ident not in _RERUN_SYM_FOCUS_STOPWORDS and not ident.startswith("get_arg")
    }
    terms.update(relation_idents)
    for field in re.findall(r"\.([A-Za-z_][A-Za-z0-9_]*)\b", relation):
        if field not in _RERUN_SYM_FOCUS_STOPWORDS:
            terms.add(field)

    prefix_text = focus_text
    prefix_match = re.search(
        r"Actual unsafe-call containing function prefix.*?```rust\n(.*?)\n```",
        focus_text,
        re.S,
    )
    if prefix_match:
        prefix_text = prefix_match.group(1)
    prefix_fields = {
        field
        for field in re.findall(
            r"\.([A-Za-z_][A-Za-z0-9_]*)\b(?!\s*\()",
            prefix_text,
        )
        if field not in _RERUN_SYM_FOCUS_STOPWORDS
    }
    terms.update(prefix_fields)
    # Prefix type information matters for rerun-sym because the generated
    # testcase may construct a safe actual-containing function's receiver and
    # parameters under different local names than the source prefix uses. Keep
    # this syntactic and generic: use Rust signature/annotation types, plus the
    # common convention that a `.point` field is constructed by a `*Point` type.
    for typ in re.findall(
        r"\b[A-Za-z_][A-Za-z0-9_]*\s*:\s*([A-Za-z_][A-Za-z0-9_:<>]*)",
        prefix_text,
    ):
        base_typ = typ.rsplit("::", 1)[-1].split("<", 1)[0]
        if base_typ and base_typ[0].isupper():
            terms.add(base_typ)
    if "point" in prefix_fields:
        terms.update({"Point", "IntPoint"})

    # Candidate reproduction plans are part of the testcase prompt and name the
    # visible types/constructors/fields used to build the target caller
    # receiver/arguments.  If the generated testcase reaches the callsite but
    # keeps those construction arguments concrete, rerun-sym must be allowed to
    # perturb them.  Keep this syntactic and source-agnostic: add identifiers
    # that appear inside backticks in plan lines, including public field names
    # from snippets like ``field: Ty`` and constructor/type names such as
    # ``Foo::new``.
    for ticked in re.findall(r"`([^`]+)`", focus_text):
        for ident in re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", ticked):
            if ident in _RERUN_SYM_FOCUS_STOPWORDS:
                continue
            terms.add(ident)
        terminal = _terminal_name(ticked)
        if terminal and terminal not in _RERUN_SYM_FOCUS_STOPWORDS:
            terms.add(terminal)

    # Add the concrete Rust type names for relation variables when the target
    # context shows a function signature, e.g. `abc: DTriangle`. Generated
    # harnesses often construct a relation variable through a constructor whose
    # line contains the type name but not the source variable name.
    for params in re.findall(r"\(([^)]*)\)", focus_text):
        for name, typ in re.findall(
            r"\b([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_][A-Za-z0-9_:<>]*)",
            params,
        ):
            if name not in relation_idents:
                continue
            base_typ = typ.rsplit("::", 1)[-1].split("<", 1)[0]
            if base_typ and base_typ[0].isupper():
                terms.add(base_typ)
    return {term for term in terms if len(term) >= 2}


def _focused_constructed_vars(lines: list[str], focus_terms: set[str]) -> set[str]:
    type_terms = {term for term in focus_terms if term[:1].isupper()}
    if not type_terms:
        return set()
    vars: set[str] = set()
    for line in lines:
        if not any(
            re.search(rf"\b{re.escape(term)}\s*(?:::|\{{)", line)
            for term in type_terms
        ):
            continue
        match = re.search(
            r"\blet\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*[:=]",
            line,
        )
        if match:
            vars.add(match.group(1))
    return vars


def _line_has_focus_term(line: str, focus_terms: set[str]) -> bool:
    if not focus_terms:
        return True
    return any(re.search(rf"\b{re.escape(term)}\b", line) for term in focus_terms)


def _line_bounds(text: str, pos: int) -> tuple[int, int]:
    start = text.rfind("\n", 0, pos) + 1
    end = text.find("\n", pos)
    if end < 0:
        end = len(text)
    return start, end


def _simple_literal_argument_line(line: str) -> bool:
    stripped = line.strip().rstrip(",").strip()
    if not stripped:
        return False
    if stripped in {"true", "false"}:
        return True
    if re.fullmatch(r"b?'(?:\\.|[^'\\])+'", stripped):
        return True
    return _integer_literal_value(stripped) is not None


def _top_level_literal_spans_for_focused_line(
    line: str, focus_terms: set[str]
) -> list[tuple[int, int]]:
    spans: list[tuple[int, int]] = []
    seen: set[tuple[int, int]] = set()

    def add_span(begin: int, end: int) -> None:
        span = (begin, end)
        if span not in seen:
            seen.add(span)
            spans.append(span)

    focus_positions = sorted(
        {
            match.start()
            for term in focus_terms
            for match in re.finditer(rf"\b{re.escape(term)}\b", line)
        }
    )
    if not focus_positions:
        return []

    def scan_call_at(focus_pos: int) -> None:
        open_paren = line.find("(", focus_pos)
        if open_paren < 0:
            add_span(0, len(line))
            return
        depth = 0
        i = open_paren
        while i < len(line):
            raw_len = _raw_string_len(line, i)
            if raw_len:
                i += raw_len
                continue
            if line.startswith("//", i):
                break
            if line.startswith("/*", i):
                i += _block_comment_len(line, i)
                continue
            if line.startswith('b"', i):
                i += 1 + _quoted_literal_len(line, i + 1, '"')
                continue
            if line[i] == '"':
                i += _quoted_literal_len(line, i, '"')
                continue
            if line.startswith("b'", i):
                n = 1 + _quoted_literal_len(line, i + 1, "'")
                if depth == 1:
                    add_span(i, i + n)
                i += n
                continue
            if line[i] == "'":
                n = _quoted_literal_len(line, i, "'")
                if depth == 1:
                    add_span(i, i + n)
                i += n
                continue
            if line[i] in "([{":
                depth += 1
                i += 1
                continue
            if line[i] in ")]}":
                depth -= 1
                i += 1
                if depth <= 0:
                    break
                continue
            if (
                depth == 1
                and line[i].isdigit()
                and (i == 0 or not _is_ident_continue(line[i - 1]))
                and (i == 0 or line[i - 1] != ".")
            ):
                n = _number_literal_len(line, i)
                add_span(i, i + n)
                i += n
                continue
            if depth == 1 and (line[i].isalpha() or line[i] == "_"):
                j = i + 1
                while j < len(line) and _is_ident_continue(line[j]):
                    j += 1
                if line[i:j] in {"true", "false"}:
                    add_span(i, j)
                i = j
                continue
            i += 1

    for focus_pos in focus_positions:
        scan_call_at(focus_pos)

    return spans


def _focused_line_ranges(body: str, focus_terms: set[str]) -> list[tuple[int, int]]:
    if not focus_terms:
        return [(0, len(body))]
    ranges: list[tuple[int, int]] = []
    lines = body.splitlines(keepends=True)
    focus_terms = set(focus_terms)
    focus_terms.update(_focused_constructed_vars(lines, focus_terms))
    starts: list[int] = []
    pos = 0
    for line in lines:
        starts.append(pos)
        pos += len(line)
    i = 0
    while i < len(lines):
        line = lines[i]
        start = starts[i]
        end = start + len(line)
        if not _line_has_focus_term(line, focus_terms):
            i += 1
            continue
        line_spans = _top_level_literal_spans_for_focused_line(line, focus_terms)
        ranges.extend((start + begin, start + span_end) for begin, span_end in line_spans)
        # If a focused constructor/wrapper call spans multiple lines, the
        # source value we care about is often supplied as plain literal
        # argument lines (`index,`, `len,`, ...). Keep those symbolic while
        # avoiding nested constructor calls such as `Point::new(0, 0)`.
        depth = line.count("(") + line.count("[") + line.count("{")
        depth -= line.count(")") + line.count("]") + line.count("}")
        j = i + 1
        while depth > 0 and j < len(lines):
            cont = lines[j]
            cont_start = starts[j]
            cont_end = cont_start + len(cont)
            if _simple_literal_argument_line(cont):
                ranges.append((cont_start, cont_end))
            depth += cont.count("(") + cont.count("[") + cont.count("{")
            depth -= cont.count(")") + cont.count("]") + cont.count("}")
            j += 1
        i += 1
    return ranges


def _transform_body_constants(
    body: str, *, symbol_prefix: str, focus_terms: set[str] | None = None
) -> tuple[str, list[dict[str, Any]]]:
    out: list[str] = []
    symbols: list[dict[str, Any]] = []
    array_const_ranges = _array_const_ranges(body)
    focus_terms = focus_terms or set()
    focus_ranges = _focused_line_ranges(body, focus_terms)
    i = 0

    def replace(literal: str, kind: str, start: int) -> str:
        name = f"{symbol_prefix}_{len(symbols):03d}"
        line = body.count("\n", 0, start) + 1
        last_newline = body.rfind("\n", 0, start)
        col = start + 1 if last_newline < 0 else start - last_newline
        symbols.append(
            {
                "name": name,
                "literal": literal,
                "kind": kind,
                "body_line": line,
                "body_col": col,
                "upper_bound": _rerun_sym_integer_upper_bound(literal)
                if kind == "number"
                else None,
            }
        )
        return name

    def should_symbolize_at(start: int) -> bool:
        if not focus_terms:
            return True
        return any(begin <= start < end for begin, end in focus_ranges)

    while i < len(body):
        raw_len = _raw_string_len(body, i)
        if raw_len:
            out.append(body[i : i + raw_len])
            i += raw_len
            continue
        if body.startswith("//", i):
            n = _line_comment_len(body, i)
            out.append(body[i : i + n])
            i += n
            continue
        if body.startswith("/*", i):
            n = _block_comment_len(body, i)
            out.append(body[i : i + n])
            i += n
            continue
        if body.startswith('b"', i):
            n = 1 + _quoted_literal_len(body, i + 1, '"')
            out.append(body[i : i + n])
            i += n
            continue
        if body[i] == '"':
            n = _quoted_literal_len(body, i, '"')
            out.append(body[i : i + n])
            i += n
            continue
        if body.startswith("b'", i):
            n = 1 + _quoted_literal_len(body, i + 1, "'")
            literal = body[i : i + n]
            out.append(
                replace(literal, "byte-char", i) if should_symbolize_at(i) else literal
            )
            i += n
            continue
        if body[i] == "'":
            n = _quoted_literal_len(body, i, "'")
            literal = body[i : i + n]
            if len(literal) > 1 and literal.endswith("'") and should_symbolize_at(i):
                out.append(replace(literal, "char", i))
                i += n
                continue
        if (
            body[i].isdigit()
            and (i == 0 or not _is_ident_continue(body[i - 1]))
            and (i == 0 or body[i - 1] != ".")
        ):
            n = _number_literal_len(body, i)
            literal = body[i : i + n]
            if any(start <= i < end for start, end in array_const_ranges):
                out.append(literal)
            elif not should_symbolize_at(i):
                out.append(literal)
            else:
                out.append(replace(literal, "number", i))
            i += n
            continue
        if body[i].isalpha() or body[i] == "_":
            j = i + 1
            while j < len(body) and _is_ident_continue(body[j]):
                j += 1
            word = body[i:j]
            if word in {"true", "false"} and should_symbolize_at(i):
                out.append(replace(word, "bool", i))
            else:
                out.append(word)
            i = j
            continue
        out.append(body[i])
        i += 1
    return "".join(out), symbols


def _stabilize_static_symbolic_slice_literals(body: str, symbol_prefix: str) -> str:
    """Keep rerun-sym rewrites compilable for generated `'static` slice inputs.

    LLM-generated harnesses often write `let data: &'static [T] = &[...]` as a
    convenient way to satisfy receiver/argument lifetimes.  After rerun-sym
    replaces an element literal with a local symbolic variable, the borrowed
    array literal is no longer a promotable `'static` constant.  Preserve the
    requested `'static` shape by leaking a boxed array whenever the RHS contains
    one of our symbolic values.

    Byte container helpers such as `Bytes::from_static(&[...])` have the same
    lifetime requirement.  Once their argument contains local symbolic values,
    switch to the equivalent runtime-copy constructor when the receiver is a
    `Bytes` type.  This keeps rerun-sym source-level and generic: it does not
    alter the chosen testcase shape, it only replaces a static-slice constructor
    with the matching owned-slice constructor after symbolization.
    """

    pattern = re.compile(
        rf"(?m)^([ \t]*let\s+(?:mut\s+)?[A-Za-z_][A-Za-z0-9_]*\s*:"
        rf"\s*&'static\s*\[[^\]\n]+\]\s*=\s*)&\[(.*?)\](\s*;[ \t]*(?://[^\n]*)?)$"
    )

    def replace(match: re.Match[str]) -> str:
        elements = match.group(2)
        if symbol_prefix not in elements:
            return match.group(0)
        return f"{match.group(1)}&*Box::leak(Box::new([{elements}])){match.group(3)}"

    body = pattern.sub(replace, body)

    from_static_pattern = re.compile(
        r"\b(?P<path>(?:[A-Za-z_][A-Za-z0-9_]*::)*Bytes)::from_static"
        r"\s*\(\s*&\[(?P<elements>.*?)\]\s*\)",
        re.S,
    )

    def replace_from_static(match: re.Match[str]) -> str:
        elements = match.group("elements")
        if symbol_prefix not in elements:
            return match.group(0)
        return f"{match.group('path')}::copy_from_slice(&[{elements}])"

    return from_static_pattern.sub(replace_from_static, body)


def symbolize_testcase_constants(
    *, testcase: str, injection: TestcaseInjection, focus_text: str | None = None
) -> tuple[str, dict[str, Any]]:
    open_brace, close_brace = _find_function_body_span(testcase, injection.function)
    body = testcase[open_brace + 1 : close_brace]
    symbol_prefix = "__unsat_rerun_sym"
    focus_terms = _rerun_sym_focus_terms(focus_text)
    transformed_body, symbols = _transform_body_constants(
        body, symbol_prefix=symbol_prefix, focus_terms=focus_terms
    )
    if focus_terms and not symbols:
        transformed_body, symbols = _transform_body_constants(
            body, symbol_prefix=symbol_prefix
        )
    transformed_body = _stabilize_static_symbolic_slice_literals(
        transformed_body, symbol_prefix
    )
    indent_match = re.search(r"\n([ \t]*)\S", body)
    indent = indent_match.group(1) if indent_match else "    "
    declarations = "".join(
        f"\n{indent}let mut {item['name']} = {item['literal']};"
        f"\n{indent}klee_ext_bind::make_symbolic!(&mut {item['name']}, \"{item['name']}\");"
        for item in symbols
    )
    updated = (
        testcase[: open_brace + 1]
        + declarations
        + transformed_body
        + testcase[close_brace:]
    )
    return updated, {
        "schema_version": 1,
        "mode": "rerun-sym",
        "function": injection.function,
        "feature": injection.feature,
        "symbol_count": len(symbols),
        "focus_terms": sorted(focus_terms),
        "focused": bool(focus_terms),
        "symbols": symbols,
    }


def testcase_injection(callsite: str, rule: str) -> TestcaseInjection:
    key = f"{callsite}::{rule}"
    slug = re.sub(r"[^a-z0-9]+", "-", f"{callsite}-{rule}".lower()).strip("-")
    slug = slug[:80].rstrip("-")
    digest = hashlib.sha256(key.encode()).hexdigest()[:10]
    feature = f"unsat-poc-{slug}-{digest}"
    function_slug = slug.replace("-", "_")
    return TestcaseInjection(
        key=key,
        callsite=callsite,
        rule=rule,
        feature=feature,
        function=f"__unsat_poc_{function_slug}_{digest}",
    )


def ensure_cargo_feature(crate_dir: Path, feature: str) -> Path:
    manifest = crate_dir / "Cargo.toml"
    text = manifest.read_text(encoding="utf-8")
    document = tomllib.loads(text)
    features = document.get("features")
    if isinstance(features, dict) and feature in features:
        return manifest

    feature_line = f"{feature} = []\n"
    header = re.search(r"(?m)^\[features\]\s*(?:#.*)?$", text)
    if header:
        insert_at = text.find("\n", header.end())
        insert_at = len(text) if insert_at < 0 else insert_at + 1
        updated = text[:insert_at] + feature_line + text[insert_at:]
    else:
        updated = text.rstrip() + f"\n\n[features]\n{feature_line}"
    # Parse before writing so malformed edits never damage the manifest.
    tomllib.loads(updated)
    manifest.write_text(updated, encoding="utf-8")
    logger.info("Registered testcase feature %s in %s", feature, manifest)
    return manifest


def write_certainty_chain_json(
    *, info_path: Path, output_path: Path, callsite: str, rule: str
) -> dict[str, Any]:
    text = info_path.read_text(encoding="utf-8", errors="replace")
    klee_chain_path = info_path.parent.parent / "klee-control-chains.json"
    klee_chains: dict[str, Any] = {}
    if klee_chain_path.is_file():
        value = json.loads(klee_chain_path.read_text(encoding="utf-8"))
        if isinstance(value, dict):
            klee_chains = value
    accepted = re.findall(
        r"compose-verify: accepted (?:aggregate escape|constructor|mutator) '([^']+)'",
        text,
    )
    selected = re.findall(
        r"compose-verify: selected .*? '([^']+)'", text
    )
    evidence_match = re.search(
        r"compose-verify: certainty evidence ledger: (.+)", text
    )
    progress_match = re.search(
        r"compose-verify: final (?:tracked DSL )?progress: (.+)", text
    )
    result_match = re.search(r"compose-verify: final result: (.+)", text)
    structured_symbols = klee_chains.get("symbols", [])
    structured_steps = [
        step
        for symbol in structured_symbols
        if isinstance(symbol, dict)
        for step in symbol.get("steps", [])
        if isinstance(step, dict)
    ] if isinstance(structured_symbols, list) else []
    structured_functions = [
        step["function"]
        for step in structured_steps
        if isinstance(step.get("function"), str) and step["function"]
    ]
    payload: dict[str, Any] = {
        "schema_version": 1,
        "callsite": callsite,
        "rule": rule,
        "status": (
            "control-chain-complete"
            if klee_chains.get("all_certain") is True
            else (result_match.group(1).strip() if result_match else "unknown")
        ),
        "call_chain": list(
            dict.fromkeys([*structured_functions, *accepted, *selected])
        ),
        "control_chains": structured_symbols,
        "certainty_evidence": evidence_match.group(1).strip()
        if evidence_match
        else "",
        "dsl_progress": progress_match.group(1).strip()
        if progress_match
        else "",
        "klee_info": str(info_path.resolve()),
        "klee_control_chain_json": str(klee_chain_path.resolve()),
    }
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    logger.info("Wrote certainty call-chain JSON: %s", output_path)
    return payload


def select_testcase_control_chains(chain: dict[str, Any]) -> list[dict[str, Any]]:
    raw = chain.get("control_chains")
    if not isinstance(raw, list):
        return []

    def score(symbol: dict[str, Any]) -> tuple[int, int]:
        text = json.dumps(symbol, ensure_ascii=False).lower()
        points = 0
        for needle, weight in (
            ("location.index", 100),
            ("entities::entities::get_mut", 50),
            ("world::world::spawn_batch", 50),
            ("get_arg(1)", 25),
        ):
            if needle in text:
                points += weight
        steps = symbol.get("steps")
        return points, len(steps) if isinstance(steps, list) else 0

    candidates = [item for item in raw if isinstance(item, dict)]
    candidates.sort(key=score, reverse=True)
    return candidates[:5]


def _terminal_name(value: object) -> str | None:
    if not isinstance(value, str) or not value.strip():
        return None
    parts = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", value)
    return parts[-1] if parts else None


def _qualified_type_names(type_name: str) -> set[str]:
    terminal = _terminal_name(type_name)
    names = {type_name}
    if terminal:
        names.add(terminal)
    return names


def _load_mirscan_report(report_path: Path | None) -> dict[str, Any]:
    if report_path is None or not report_path.is_file():
        return {}
    try:
        value = json.loads(report_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    return value if isinstance(value, dict) else {}


def _mirscan_type_index(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    index: dict[str, dict[str, Any]] = {}
    raw_types = report.get("types")
    if not isinstance(raw_types, list):
        return index
    for entry in raw_types:
        if not isinstance(entry, dict):
            continue
        ty = entry.get("type")
        if not isinstance(ty, dict):
            continue
        name = ty.get("name")
        if not isinstance(name, str) or not name:
            continue
        for key in _qualified_type_names(name):
            index.setdefault(key, entry)
    return index


def _split_top_level_commas(text: str) -> list[str]:
    parts: list[str] = []
    start = 0
    depth_angle = depth_paren = depth_bracket = 0
    for index, char in enumerate(text):
        if char == "<":
            depth_angle += 1
        elif char == ">" and depth_angle:
            depth_angle -= 1
        elif char == "(":
            depth_paren += 1
        elif char == ")" and depth_paren:
            depth_paren -= 1
        elif char == "[":
            depth_bracket += 1
        elif char == "]" and depth_bracket:
            depth_bracket -= 1
        elif char == "," and not (depth_angle or depth_paren or depth_bracket):
            part = text[start:index].strip()
            if part:
                parts.append(part)
            start = index + 1
    tail = text[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def _extract_signature_params(crate_dir: Path, target: dict[str, Any]) -> list[dict[str, str]]:
    caller = target.get("caller")
    caller = caller if isinstance(caller, dict) else {}
    path = caller.get("path")
    line_start = caller.get("line_start")
    if not isinstance(path, str) or not isinstance(line_start, int):
        return []
    source_path = crate_dir / path
    if not source_path.is_file():
        return []
    source = source_path.read_text(encoding="utf-8", errors="replace")
    lines = source.splitlines()
    signature = "\n".join(lines[max(0, line_start - 12): min(len(lines), line_start + 20)])
    fn_match = re.search(r"\bfn\s+[A-Za-z_][A-Za-z0-9_]*", signature)
    if not fn_match:
        caller_name = caller.get("name")
        terminal = _terminal_name(caller_name) if isinstance(caller_name, str) else None
        if terminal:
            fallback = re.search(rf"\bfn\s+{re.escape(terminal)}\b", source)
            if fallback:
                signature = source[fallback.start(): fallback.start() + 4000]
                fn_match = re.search(r"\bfn\s+[A-Za-z_][A-Za-z0-9_]*", signature)
    if not fn_match:
        return []
    paren_start = signature.find("(", fn_match.end())
    if paren_start < 0:
        return []
    depth = 0
    paren_end = -1
    for index in range(paren_start, len(signature)):
        char = signature[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                paren_end = index
                break
    if paren_end < 0:
        return []
    params: list[dict[str, str]] = []
    for raw in _split_top_level_commas(signature[paren_start + 1:paren_end]):
        cleaned = re.sub(r"^\s*(?:mut\s+)?", "", raw.strip())
        if cleaned in {"self", "&self", "&mut self", "mut self"}:
            params.append({"name": "self", "ty": "Self", "raw": raw.strip()})
            continue
        if ":" not in cleaned:
            continue
        name_part, ty_part = cleaned.split(":", 1)
        name = _terminal_name(name_part) or name_part.strip()
        ty = ty_part.strip()
        if name and ty:
            params.append({"name": name, "ty": ty, "raw": raw.strip()})
    return params


def _type_names_in_text(text: str, type_index: dict[str, dict[str, Any]]) -> list[str]:
    names: list[str] = []
    tokens = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", text))
    for key, entry in type_index.items():
        terminal = _terminal_name(key)
        type_name = entry.get("type", {}).get("name") if isinstance(entry.get("type"), dict) else None
        if not isinstance(type_name, str):
            continue
        if key in text or (terminal and terminal in tokens):
            if type_name not in names:
                names.append(type_name)
    return names


def _called_functions_in_chain(testcase_chains: list[dict[str, Any]]) -> set[str]:
    called: set[str] = set()
    for symbol in testcase_chains:
        steps = symbol.get("steps")
        if not isinstance(steps, list):
            continue
        for step in steps:
            if not isinstance(step, dict):
                continue
            function = step.get("function")
            if isinstance(function, str) and function:
                called.add(function)
                terminal = _terminal_name(function)
                if terminal:
                    called.add(terminal)
    return called


def _constructors_for_type(entry: dict[str, Any], called_functions: set[str]) -> list[str]:
    constructors: list[str] = []
    raw = entry.get("constructors")
    if not isinstance(raw, list):
        return constructors
    for item in raw:
        if not isinstance(item, dict):
            continue
        name = item.get("name")
        if not isinstance(name, str) or not name:
            continue
        terminal = _terminal_name(name)
        if name in called_functions or (terminal and terminal in called_functions):
            continue
        constructors.append(name)
    return constructors[:3]


def _constructor_phrases(
    *,
    crate_dir: Path,
    target: dict[str, Any],
    report: dict[str, Any],
    testcase_chains: list[dict[str, Any]],
) -> list[str]:
    type_index = _mirscan_type_index(report)
    if not type_index:
        return []
    called_functions = _called_functions_in_chain(testcase_chains)
    wanted: list[tuple[str, str]] = []

    caller_parent = target.get("caller_parent")
    if isinstance(caller_parent, dict):
        parent_name = caller_parent.get("name")
        if isinstance(parent_name, str) and parent_name:
            wanted.append(("receiver", parent_name))

    for param in _extract_signature_params(crate_dir, target):
        if param["ty"] == "Self":
            continue
        for type_name in _type_names_in_text(param["ty"], type_index):
            wanted.append((param["name"], type_name))

    phrases: list[str] = []
    seen_types: set[str] = set()
    for role, type_name in wanted:
        if type_name in seen_types:
            continue
        seen_types.add(type_name)
        entry = type_index.get(type_name) or type_index.get(_terminal_name(type_name) or "")
        if not isinstance(entry, dict):
            continue
        constructors = _constructors_for_type(entry, called_functions)
        public_fields = entry.get("public_fields")
        field_names = [
            field.get("name")
            for field in public_fields
            if isinstance(field, dict) and isinstance(field.get("name"), str)
        ] if isinstance(public_fields, list) else []
        field_layouts = entry.get("field_layouts")
        field_types = {
            field.get("name"): field.get("ty")
            for field in field_layouts
            if isinstance(field, dict)
            and isinstance(field.get("name"), str)
            and isinstance(field.get("ty"), str)
        } if isinstance(field_layouts, list) else {}
        field_parts = [
            (
                f"`{name}: {field_types[name]}`"
                if name in field_types
                else f"`{name}`"
            )
            for name in field_names[:5]
        ]
        if field_parts:
            phrases.append(
                f"construct `{type_name}` for `{role}` using fields visible "
                "from the injection module when that better controls the "
                "target caller state: "
                + ", ".join(field_parts)
            )
        if constructors:
            if field_parts:
                phrases.append(
                    f"or construct `{type_name}` for `{role}` using "
                    "constructor(s): "
                    + ", ".join(f"`{name}`" for name in constructors)
                )
            else:
                phrases.append(
                    f"construct `{type_name}` for `{role}` using constructor(s): "
                    + ", ".join(f"`{name}`" for name in constructors)
                )
    return phrases


def _target_caller_name(target: dict[str, Any]) -> str | None:
    caller = target.get("caller")
    if isinstance(caller, dict):
        name = caller.get("name")
        if isinstance(name, str) and name.strip():
            return name
    caller_parent = target.get("caller_parent")
    if isinstance(caller_parent, dict):
        name = caller_parent.get("name")
        if isinstance(name, str) and name.strip():
            return name
    return None


def _step_phrase(step: dict[str, Any]) -> str | None:
    kind = step.get("kind")
    if kind == "root":
        root_type = step.get("root_type")
        return f"construct or obtain a `{root_type}` value" if root_type else None
    if kind == "public_field":
        field = step.get("field")
        root_type = step.get("root_type")
        if field and root_type:
            return f"set or populate public field `{root_type}.{field}`"
        if field:
            return f"set or populate public field `{field}`"
    if kind in {"mut_ref_escape", "mutator", "observer", "constructor"}:
        function = step.get("function")
        field = step.get("field")
        if function and field:
            return f"call safe `{function}` to affect `{field}`"
        if function:
            return f"call safe `{function}`"
    function = step.get("function")
    if isinstance(function, str) and function:
        return f"call safe `{function}`"
    field = step.get("field")
    if isinstance(field, str) and field:
        return f"prepare field `{field}`"
    return None


def build_candidate_reproduction_plans(
    *,
    crate_dir: Path,
    target: dict[str, Any],
    chain: dict[str, Any],
    testcase_chains: list[dict[str, Any]],
    report: dict[str, Any] | None = None,
    limit: int = 5,
) -> str:
    """Summarize safe constructor/mutator/target paths for the LLM.

    This deliberately does not synthesize concrete POC code or values. It only
    exposes the safe API sequence shapes already present in mirscan/KLEE
    metadata, leaving the LLM to choose values and write the testcase.
    """
    target_caller = _target_caller_name(target)
    plans: list[list[str]] = []
    constructor_steps = _constructor_phrases(
        crate_dir=crate_dir,
        target=target,
        report=report or {},
        testcase_chains=testcase_chains,
    )
    direct_actual_plan = _direct_actual_call_plan(crate_dir=crate_dir, target=target)
    if direct_actual_plan:
        plans.append([*constructor_steps, *direct_actual_plan])

    for symbol in testcase_chains:
        steps = symbol.get("steps")
        if not isinstance(steps, list):
            continue
        phrases = [
            phrase
            for step in steps
            if isinstance(step, dict)
            for phrase in [_step_phrase(step)]
            if phrase
        ]
        phrases = [*constructor_steps, *phrases]
        if target_caller:
            phrases.append(f"call target caller `{target_caller}`")
        if phrases:
            plans.append(phrases)

    flat_chain = chain.get("call_chain")
    if isinstance(flat_chain, list):
        names = [
            name
            for item in flat_chain
            for name in [_terminal_name(item)]
            if name
        ]
        if names:
            phrases = [*constructor_steps, *[f"call safe `{name}`" for name in dict.fromkeys(names)]]
            if target_caller and target_caller not in flat_chain:
                phrases.append(f"call target caller `{target_caller}`")
            plans.append(phrases)

    if target_caller:
        plans.append([
            *(constructor_steps or ["construct the minimal visible receiver/arguments needed by the target caller"]),
            f"call target caller `{target_caller}` directly",
        ])

    deduped: list[list[str]] = []
    seen: set[tuple[str, ...]] = set()
    for plan in plans:
        key = tuple(plan)
        if key in seen:
            continue
        seen.add(key)
        deduped.append(plan)
        if len(deduped) >= limit:
            break

    if not deduped:
        return (
            "1. Construct the smallest crate-visible state needed to reach the "
            "metadata caller, then call the target caller to generate a "
            "counterexample for the target callsite."
        )

    if direct_actual_plan or target_caller:
        lines = [
            "Safe-call skeletons. Prefer the first direct target-caller skeleton "
            "when it type-checks from the injection module; only fall back to "
            "later wrapper skeletons if the direct skeleton does not compile or "
            "cannot reach the target. Choose concrete values yourself to "
            "generate a counterexample for the target caller and target callsite:"
        ]
    else:
        lines = [
            "Use one of these safe-call skeletons if it type-checks; choose concrete "
            "values yourself to generate a counterexample for the target caller and "
            "target callsite:"
        ]
    for index, plan in enumerate(deduped, start=1):
        lines.append(f"{index}. " + " -> ".join(plan))
    return "\n".join(lines)


def _source_window_for_line(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
    radius: int = 10,
) -> str | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None
    start = max(1, line_value - radius)
    end = min(len(lines), line_value + radius)
    width = len(str(end))
    body = "\n".join(
        f"{line_no:>{width}}: {lines[line_no - 1]}"
        for line_no in range(start, end + 1)
    )
    return f"{path_value}:{line_value}\n```rust\n{body}\n```"


def _source_function_prefix_for_line(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
    max_lines: int = 120,
) -> str | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None

    fn_start = None
    for index in range(line_value - 1, -1, -1):
        if re.search(r"\bfn\s+[A-Za-z_][A-Za-z0-9_]*\b", lines[index]):
            fn_start = index + 1
            break
    if fn_start is None:
        return None

    end = min(len(lines), line_value, fn_start + max_lines - 1)
    width = len(str(end))
    body = "\n".join(
        f"{line_no:>{width}}: {lines[line_no - 1]}"
        for line_no in range(fn_start, end + 1)
    )
    return f"{path_value}:{fn_start}-{end}\n```rust\n{body}\n```"


def _called_helper_names_from_rust(code: str) -> list[str]:
    """Return small safe-prefix helper names in source order.

    This is intentionally syntactic.  The testcase prompt should expose
    source facts that help the LLM satisfy safe prefix path conditions, without
    synthesizing a POC or adding crate-specific knowledge.
    """

    names: list[str] = []
    seen: set[str] = set()

    def add(name: str) -> None:
        if name.startswith("__") or name in seen:
            return
        if name in {
            "as_ptr",
            "as_mut_ptr",
            "clone",
            "copy",
            "default",
            "expect",
            "is_empty",
            "iter",
            "len",
            "new",
            "push",
            "unwrap",
        }:
            return
        seen.add(name)
        names.append(name)

    for match in re.finditer(
        r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(",
        code,
    ):
        add(match.group(1))
    for match in re.finditer(
        r"\b(?:Self|[A-Z][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)::"
        r"([A-Za-z_][A-Za-z0-9_]*)\s*\(",
        code,
    ):
        add(match.group(1))
    return names


def _called_macro_names_from_rust(code: str) -> list[str]:
    names: list[str] = []
    seen: set[str] = set()
    for match in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*!", code):
        name = match.group(1)
        if name in seen:
            continue
        seen.add(name)
        names.append(name)
    return names


def _source_function_window_by_name(
    *,
    crate_dir: Path,
    function_name: str,
    max_lines: int = 48,
) -> str | None:
    if not function_name or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", function_name):
        return None
    root = (crate_dir / "src").resolve()
    if not root.is_dir():
        root = crate_dir.resolve()
    crate_root = crate_dir.resolve()
    pattern = re.compile(rf"\bfn\s+{re.escape(function_name)}\b")
    for path in sorted(root.rglob("*.rs")):
        try:
            path.relative_to(crate_root)
        except ValueError:
            continue
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue
        for index, line in enumerate(lines):
            if not pattern.search(line):
                continue
            start = index
            end = min(len(lines), start + max_lines)
            depth = 0
            saw_body = False
            for j in range(start, end):
                text = lines[j]
                depth += text.count("{")
                if "{" in text:
                    saw_body = True
                depth -= text.count("}")
                if saw_body and depth <= 0:
                    end = j + 1
                    break
            rel = str(path.relative_to(crate_root))
            width = len(str(end))
            body = "\n".join(
                f"{line_no:>{width}}: {lines[line_no - 1]}"
                for line_no in range(start + 1, end + 1)
            )
            return f"{rel}:{start + 1}-{end}\n```rust\n{body}\n```"
    return None


def _source_macro_window_by_name(
    *,
    crate_dir: Path,
    macro_name: str,
    max_lines: int = 80,
) -> str | None:
    if not macro_name or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", macro_name):
        return None
    root = (crate_dir / "src").resolve()
    if not root.is_dir():
        root = crate_dir.resolve()
    crate_root = crate_dir.resolve()
    pattern = re.compile(rf"\bmacro_rules!\s*{re.escape(macro_name)}\b")
    for path in sorted(root.rglob("*.rs")):
        try:
            path.relative_to(crate_root)
        except ValueError:
            continue
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue
        for index, line in enumerate(lines):
            if not pattern.search(line):
                continue
            start = index
            end = min(len(lines), start + max_lines)
            depth = 0
            saw_body = False
            for j in range(start, end):
                text = lines[j]
                depth += text.count("{")
                if "{" in text:
                    saw_body = True
                depth -= text.count("}")
                if saw_body and depth <= 0:
                    end = j + 1
                    break
            rel = str(path.relative_to(crate_root))
            width = len(str(end))
            body = "\n".join(
                f"{line_no:>{width}}: {lines[line_no - 1]}"
                for line_no in range(start + 1, end + 1)
            )
            return f"{rel}:{start + 1}-{end}\n```rust\n{body}\n```"
    return None


def _cargo_manifest_dependency_names(crate_dir: Path) -> set[str]:
    manifest_path = crate_dir / "Cargo.toml"
    if not manifest_path.is_file():
        return set()
    try:
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8", errors="replace"))
    except (tomllib.TOMLDecodeError, OSError):
        return set()
    names: set[str] = set()

    def add_deps(table: object) -> None:
        if not isinstance(table, dict):
            return
        for key, value in table.items():
            if isinstance(key, str):
                names.add(key)
            if isinstance(value, dict):
                package = value.get("package")
                if isinstance(package, str):
                    names.add(package)

    # Testcase source facts should explain the target crate's public runtime
    # types.  Dev/build dependencies tend to contribute derive/helper names
    # such as serde/bincode internals, which are rarely constructible state for
    # the target caller and can crowd out the relevant dependency structs.
    add_deps(manifest.get("dependencies"))
    target = manifest.get("target")
    if isinstance(target, dict):
        for target_table in target.values():
            if isinstance(target_table, dict):
                add_deps(target_table.get("dependencies"))
    return names


def _cargo_lock_registry_package_dirs(crate_dir: Path) -> list[Path]:
    lock_path = crate_dir / "Cargo.lock"
    if not lock_path.is_file():
        return []
    try:
        lock = tomllib.loads(lock_path.read_text(encoding="utf-8", errors="replace"))
    except (tomllib.TOMLDecodeError, OSError):
        return []
    packages = lock.get("package")
    if not isinstance(packages, list):
        return []
    direct_names = _cargo_manifest_dependency_names(crate_dir)
    registry_roots = [
        root
        for base in [Path.home() / ".cargo" / "registry" / "src"]
        if base.is_dir()
        for root in base.iterdir()
        if root.is_dir()
    ]
    dirs: list[Path] = []
    seen: set[Path] = set()
    for package in packages:
        if not isinstance(package, dict):
            continue
        name = package.get("name")
        version = package.get("version")
        source = package.get("source")
        if not (
            isinstance(name, str)
            and isinstance(version, str)
            and isinstance(source, str)
            and source.startswith("registry+")
        ):
            continue
        if direct_names and name not in direct_names:
            continue
        dirname = f"{name}-{version}"
        for root in registry_roots:
            candidate = root / dirname
            if candidate.is_dir() and candidate not in seen:
                seen.add(candidate)
                dirs.append(candidate)
    return dirs


def _source_type_window_in_file(path: Path, type_name: str, max_lines: int = 140) -> str | None:
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    pattern = re.compile(
        rf"\b(?:pub\s+)?(?:struct|enum|union|type)\s+{re.escape(type_name)}\b"
    )
    for index, line in enumerate(lines):
        if not pattern.search(line):
            continue
        start = index
        end = min(len(lines), start + max_lines)
        width = len(str(end))
        body = "\n".join(
            f"{line_no:>{width}}: {lines[line_no - 1]}"
            for line_no in range(start + 1, end + 1)
        )
        return f"{path}:{start + 1}-{end}\n```rust\n{body}\n```"
    return None


def _external_type_source_context(
    *,
    crate_dir: Path,
    seed_text: str,
    limit: int = 10,
) -> str | None:
    """Add small public type snippets from registry dependencies.

    The generated testcase is compiled inside the local crate, but its receiver
    state often contains public dependency types re-exported by the crate.  A
    semantic slice over only local `src/` files leaves the LLM guessing those
    public field names.  This helper exposes source facts for dependency types
    mentioned by the target context, plus one level of types mentioned by those
    snippets.
    """

    def seed_rust_code(text: str) -> str:
        blocks = re.findall(r"```rust\n(.*?)\n```", text, re.S)
        if not blocks:
            return text
        return "\n\n".join(
            "\n".join(
                re.sub(r"^\s*\d+:\s?", "", line)
                for line in block.splitlines()
            )
            for block in blocks
        )

    skip = {
        "Arc",
        "Address",
        "B256",
        "Box",
        "Bytes",
        "Clone",
        "Contract",
        "Deserialize",
        "Deserializer",
        "DummyHost",
        "Default",
        "Error",
        "FunctionStack",
        "Gas",
        "Host",
        "InstructionResult",
        "Interpreter",
        "InterpreterAction",
        "None",
        "Ok",
        "Option",
        "Result",
        "Self",
        "Serialize",
        "Serializer",
        "SharedMemory",
        "Some",
        "Stack",
        "String",
        "U256",
        "Vec",
    }
    code_seed = seed_rust_code(seed_text)
    queue: list[str] = [
        name
        for name in dict.fromkeys(re.findall(r"\b[A-Z][A-Za-z0-9_]*\b", code_seed))
        if len(name) > 1 and not name.isupper() and name not in skip
    ]
    package_dirs = _cargo_lock_registry_package_dirs(crate_dir)
    if not package_dirs or not queue:
        return None
    snippets: list[str] = []
    seen_names: set[str] = set()
    seen_paths: set[Path] = set()
    rounds = 0
    while queue and len(snippets) < limit and rounds < limit * 4:
        rounds += 1
        type_name = queue.pop(0)
        if (
            type_name in seen_names
            or type_name in skip
            or len(type_name) <= 1
            or type_name.isupper()
        ):
            continue
        seen_names.add(type_name)
        snippet = None
        snippet_path: Path | None = None
        for package_dir in package_dirs:
            for path in sorted((package_dir / "src").rglob("*.rs")):
                candidate = _source_type_window_in_file(path, type_name)
                if candidate:
                    snippet = candidate
                    snippet_path = path
                    break
            if snippet:
                break
        if not snippet or snippet_path is None or snippet_path in seen_paths:
            continue
        seen_paths.add(snippet_path)
        snippets.append(snippet)
        nested_names = []
        for nested in re.findall(
            r"\b[A-Z][A-Za-z0-9_]*\b", _rust_code_from_fenced_block(snippet)
        ):
            if (
                len(nested) > 1
                and not nested.isupper()
                and nested not in seen_names
                and nested not in skip
                and nested not in queue
            ):
                nested_names.append(nested)
        queue = list(dict.fromkeys(nested_names + queue))
    if not snippets:
        return None
    return (
        "Relevant public type definitions from registry dependencies. These "
        "are source facts for constructing the target caller state; do not call "
        "unsafe APIs from these snippets:\n\n"
        + "\n\n".join(snippets)
    )


def _module_visible_path_context(
    *,
    crate_dir: Path,
    path_value: object,
) -> str | None:
    if not isinstance(path_value, str) or not path_value:
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None

    use_lines: list[str] = []
    for line in lines[:160]:
        stripped = line.strip()
        if re.match(r"use\s+[^;]+;", stripped):
            use_lines.append(stripped)
        if len(use_lines) >= 20:
            break

    dependency_crates = {
        name.replace("-", "_")
        for name in _cargo_manifest_dependency_names(crate_dir)
        if isinstance(name, str)
    }
    head = "\n".join(lines[:260])
    visible_deps = [
        dep
        for dep in sorted(dependency_crates)
        if re.search(rf"\b{re.escape(dep)}::", head)
    ]
    if not use_lines and not visible_deps:
        return None

    pieces = [
        "Visible paths from the injection module. The generated testcase is "
        "appended to this same Rust file, so these imports/direct dependency "
        "paths are already in scope; use them exactly instead of inventing "
        "`crate::<dependency>::...` paths."
    ]
    if use_lines:
        pieces.append(
            "Top visible `use` statements:\n```rust\n"
            + "\n".join(use_lines)
            + "\n```"
        )
    if visible_deps:
        pieces.append(
            "Direct dependency crate/module prefixes visibly used in this file: "
            + ", ".join(f"`{dep}::...`" for dep in visible_deps)
            + "."
        )
    return "\n\n".join(pieces)


def _source_function_window_containing_line(
    *,
    path: Path,
    line_index: int,
    display_root: Path | None = None,
    max_lines: int = 80,
) -> str | None:
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_index < 0 or line_index >= len(lines):
        return None
    start = line_index
    for index in range(line_index, -1, -1):
        if re.search(r"\bfn\s+[A-Za-z_][A-Za-z0-9_]*\b", lines[index]):
            start = index
            break
    while start > 0 and (
        lines[start - 1].lstrip().startswith("#") or not lines[start - 1].strip()
    ):
        start -= 1
    end = min(len(lines), start + max_lines)
    depth = 0
    saw_body = False
    for index in range(start, end):
        text = lines[index]
        depth += text.count("{")
        if "{" in text:
            saw_body = True
        depth -= text.count("}")
        if saw_body and depth <= 0:
            end = index + 1
            break
    rel = str(path)
    if display_root is not None:
        try:
            rel = str(path.relative_to(display_root))
        except ValueError:
            pass
    width = len(str(end))
    body = "\n".join(
        f"{line_no:>{width}}: {lines[line_no - 1]}"
        for line_no in range(start + 1, end + 1)
    )
    return f"{rel}:{start + 1}-{end}\n```rust\n{body}\n```"


def _source_construction_example_context(
    *,
    crate_dir: Path,
    seed_text: str,
    limit: int = 8,
) -> str | None:
    """Expose existing safe construction idioms for target-state types.

    Type definitions alone often leave the testcase generator guessing how a
    dependency type expects its fields to be kept coherent.  This miner adds
    short source examples already present in the crate or its direct registry
    dependencies.  It does not synthesize values or point at a particular unsafe
    API; it only gives constructor/mutator skeletons the LLM may reuse.
    """

    blocks = re.findall(r"```rust\n(.*?)\n```", seed_text, re.S)
    code_seed = "\n\n".join(blocks) if blocks else seed_text
    skip = {
        "Address",
        "Arc",
        "Box",
        "Bytes",
        "Clone",
        "Default",
        "DummyHost",
        "Gas",
        "Host",
        "InstructionResult",
        "InterpreterAction",
        "None",
        "Ok",
        "Option",
        "Result",
        "Self",
        "Some",
        "String",
        "TestCase",
        "U256",
        "Vec",
    }
    tokens = [
        name
        for name in dict.fromkeys(re.findall(r"\b[A-Z][A-Za-z0-9_]*\b", code_seed))
        if len(name) > 1 and not name.isupper() and name not in skip
    ]
    if not tokens:
        return None

    crate_root = crate_dir.resolve()
    search_roots: list[tuple[Path, Path | None]] = []
    local_src = crate_root / "src"
    if local_src.is_dir():
        search_roots.append((local_src, crate_root))
    for package_dir in _cargo_lock_registry_package_dirs(crate_dir):
        src = package_dir / "src"
        if src.is_dir():
            search_roots.append((src, package_dir))

    candidates: list[tuple[int, int, str]] = []
    seen: set[str] = set()
    token_alt = "|".join(re.escape(token) for token in tokens[:24])
    construction_pattern = re.compile(
        rf"\b(?:{token_alt})\s*(?:::|\{{|\()|"
        rf"\b[A-Z][A-Za-z0-9_]*::(?:{token_alt})\b"
    )
    priority_tokens = {
        token
        for token in tokens
        if token
        not in {
            "Contract",
            "DummyHost",
            "FunctionStack",
            "Gas",
            "InstructionResult",
            "Interpreter",
            "SharedMemory",
            "Stack",
        }
    }
    for root_index, (root, display_root) in enumerate(search_roots):
        for path in sorted(root.rglob("*.rs")):
            try:
                lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
            except OSError:
                continue
            for index, line in enumerate(lines):
                if not construction_pattern.search(line):
                    continue
                if not any(
                    needle in line
                    for needle in (
                        "::new",
                        "::decode",
                        "::default",
                        "::from",
                        "Default::default",
                        "Bytecode::",
                        "container_section",
                        "code_section",
                        "types_section",
                    )
                ):
                    continue
                snippet = _source_function_window_containing_line(
                    path=path,
                    line_index=index,
                    display_root=display_root,
                )
                if not snippet or snippet in seen:
                    continue
                seen.add(snippet)
                code = _rust_code_from_fenced_block(snippet)
                if "unsafe {" in code or "klee_ext_bind::callsite!" in code:
                    # Construction examples should be safe state-building
                    # idioms.  Re-showing the target body or another unsafe
                    # implementation body tends to make the LLM copy the wrong
                    # layer instead of constructing a caller state.
                    continue
                score = 0
                for token in tokens:
                    if re.search(rf"\b{re.escape(token)}\b", code):
                        score += 4 if token in priority_tokens else 1
                for needle in (
                    "container_section",
                    "code_section",
                    "types_section",
                    "Bytecode::",
                    "::decode",
                ):
                    if needle in code:
                        score += 6
                if "#[test]" in code or "#[cfg(test)]" in code or re.search(r"\bfn\s+test", code):
                    score += 18
                if re.search(r"\bfn\s+[A-Za-z0-9_]*(?:dummy|setup|fixture|make|build)[A-Za-z0-9_]*\b", code):
                    score += 18
                if re.search(r"::decode\s*\([^)]*\)\s*\.\s*(?:unwrap|expect)\s*\(", code):
                    score += 18
                if re.search(r"\b(?:body|header)\s*\.\s*[A-Za-z0-9_]+\s*\.", code):
                    score += 10
                if re.search(r"\.(?:push|extend|extend_from_slice|clear)\s*\(", code):
                    score += 8
                if "::default" in code or "Default::default" in code:
                    score -= 18
                if re.search(r"\b(?:Vec::new|vec!\s*\[\s*\])", code):
                    score -= 18
                if re.search(r"\b(?:Err|assert_eq!)\s*\(", code) and re.search(
                    r"\bInvalid[A-Za-z0-9_]*|MissingInput|ShortInput|TooMany|Mismatch",
                    code,
                ):
                    score -= 48
                if re.search(
                    r"\b(?:pub\s+)?fn\s+(?:decode|new|default|encode|into_eof)\b",
                    code,
                ):
                    score -= 64
                candidates.append((score, root_index, snippet))
                if len(candidates) >= max(limit * 12, 32):
                    break
            if len(candidates) >= max(limit * 12, 32):
                break
        if len(candidates) >= max(limit * 12, 32):
            break

    snippets: list[str] = []
    for _score, _root_index, snippet in sorted(
        candidates,
        key=lambda item: (-item[0], item[1], item[2]),
    ):
        snippets.append(snippet)
        if len(snippets) >= limit:
            break
    if not snippets:
        return None
    return (
        "Existing safe construction examples for types mentioned above. Use "
        "these only as constructor/mutator/caller skeletons; choose your own "
        "values to build a target-caller counterexample, and do not call unsafe "
        "APIs from these examples. If a type's prefix path reads from a "
        "collection, decodes an element, or expects a branch guard to hold, do "
        "not rely on `Default`, `Vec::new()`, or `vec![]` as the complete "
        "receiver state unless the example also mutates the relevant fields "
        "into a non-empty, coherent state:\n\n"
        + "\n\n".join(snippets)
    )


def _source_function_source_by_name(
    *,
    crate_dir: Path,
    function_name: str,
    max_lines: int = 80,
) -> str | None:
    window = _source_function_window_by_name(
        crate_dir=crate_dir,
        function_name=function_name,
        max_lines=max_lines,
    )
    if not window:
        return None
    return _rust_code_from_fenced_block(window)


def _helper_source_context_for_prefix(
    *,
    crate_dir: Path,
    prefix_block: str,
    limit: int = 8,
) -> str | None:
    code = _rust_code_from_fenced_block(prefix_block)
    names = _called_helper_names_from_rust(code)
    snippets: list[str] = []
    for name in _called_macro_names_from_rust(code):
        snippet = _source_macro_window_by_name(
            crate_dir=crate_dir,
            macro_name=name,
        )
        if snippet:
            snippets.append(snippet)
        if len(snippets) >= limit:
            break
    for name in names:
        if len(snippets) >= limit:
            break
        snippet = _source_function_window_by_name(
            crate_dir=crate_dir,
            function_name=name,
        )
        if snippet:
            snippets.append(snippet)
    if not snippets:
        return None
    return (
        "Relevant macro/helper definitions used by the safe prefix before the target "
        "callsite. Use these source facts only to choose values that let the "
        "prefix return normally and reach the target; the counterexample still "
        "must be for the target caller and target callsite:\n\n"
        + "\n\n".join(snippets)
    )


def _rust_code_from_fenced_block(text: str) -> str:
    match = re.search(r"```rust\n(.*?)\n```", text, re.S)
    code = match.group(1) if match else text
    return "\n".join(
        re.sub(r"^\s*\d+:\s?", "", line)
        for line in code.splitlines()
    )


def _helper_lookup_obligation_items(
    *, crate_dir: Path | None, receiver: str, method: str, args: str
) -> list[str]:
    if crate_dir is None:
        return []
    body = _source_function_source_by_name(
        crate_dir=crate_dir,
        function_name=method,
    )
    if not body:
        return []
    first_arg = _split_top_level_commas(args)[0].strip() if args.strip() else ""
    if not first_arg:
        return []
    items: list[str] = []
    for field, param in re.findall(
        r"\bself\.([A-Za-z_][A-Za-z0-9_]*)\s*\[\s*i\s*\]\s*==\s*"
        r"([A-Za-z_][A-Za-z0-9_]*)",
        body,
    ):
        items.append(
            f"From helper `{method}` source, `self.{field}[i] == {param}` "
            f"is a successful lookup condition; for `{receiver}.{method}({args})`, "
            f"construct `{receiver}.{field}` so it contains `{first_arg}`."
        )
    for field, subfield, param in re.findall(
        r"\bself\.([A-Za-z_][A-Za-z0-9_]*)\s*\[\s*i\s*\]\s*"
        r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*==\s*"
        r"([A-Za-z_][A-Za-z0-9_]*)",
        body,
    ):
        items.append(
            f"From helper `{method}` source, `self.{field}[i].{subfield} == {param}` "
            f"is a successful lookup condition; for `{receiver}.{method}({args})`, "
            f"construct `{receiver}.{field}[*].{subfield}` so one element equals "
            f"`{first_arg}`."
        )
    return items


def _helper_prefix_call_obligation_items(
    *, crate_dir: Path | None, receiver: str, method: str, args: str
) -> list[str]:
    if crate_dir is None:
        return []
    body = _source_function_source_by_name(
        crate_dir=crate_dir,
        function_name=method,
    )
    if not body:
        return []
    first_arg = _split_top_level_commas(args)[0].strip() if args.strip() else ""
    if not first_arg:
        return []
    items: list[str] = []
    if (
        ".is_not_nil()" in body
        and (
            "get_unchecked" in body
            or re.search(r"\[[^\]]+\]", body)
            or ".opposite(" in body
            or ".neighbor(" in body
        )
    ):
        items.append(
            f"`{receiver}.{method}({args})` is a safe prefix helper that must "
            "return normally before the target. Its source has a nil-sentinel "
            f"guard before nested indexing/lookup; construct `{first_arg}` so "
            "the helper either takes the nil/sentinel skip path or uses an "
            "in-bounds existing element whose nested lookup will also succeed."
        )
    return items


def _prefix_path_obligation_items(
    prefix_block: str, *, crate_dir: Path | None = None
) -> list[str]:
    code = _rust_code_from_fenced_block(prefix_block)
    marker_index = code.find("klee_ext_bind::callsite!")
    if marker_index >= 0:
        code = code[:marker_index]
    obligations: list[str] = []
    seen: set[str] = set()

    def add(text: str) -> None:
        if text in seen:
            return
        seen.add(text)
        obligations.append(text)

    selected_gets: dict[str, tuple[str, str]] = {}
    for var, expr in re.findall(
        r"\blet\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"
        r"([^;]*?\.get\s*\(.+?\)\s*(?:\.\s*[A-Za-z_][A-Za-z0-9_]*"
        r"\s*\(\s*\)\s*)*\.expect\s*\([^;]*\))\s*;",
        code,
        re.S,
    ):
        get_match = re.search(r"\.get\s*\((.*?)\)", expr, re.S)
        get_pos = expr.rfind(".get")
        if get_match and get_pos >= 0:
            collection = " ".join(expr[:get_pos].split())
            selector = " ".join(get_match.group(1).split())
            selected_gets[var] = (collection, selector)
            add(
                f"`{var}` is selected by `{collection}.get({selector}).expect(...)` "
                "before the target; construct that collection with an element "
                "at the selected index/key. A default or empty receiver is not "
                "enough for this prefix path. If the collection expression is "
                "reached through helper methods or wrappers, use the helper "
                "source below to identify the backing receiver field(s), then "
                "initialize those backing fields so this exact expression reads "
                "the populated collection."
            )

    for receiver, method, args in re.findall(
        r"\b([A-Za-z_][A-Za-z0-9_\.]*)\s*\.\s*"
        r"(opposite|neighbor|vertex_by_order|neighbor_by_order|"
        r"get|lookup|find|index_of|position)"
        r"\s*\(([^)]*)\)",
        code,
    ):
        args = " ".join(args.split())
        if method == "opposite":
            detail = (
                "construct the receiver so the argument value is present in "
                "the opposite/neighbor/link collection it searches; for this "
                f"call, make `{receiver}` contain/reference `{args}` in the "
                "field that drives the lookup"
            )
        elif method == "neighbor":
            detail = (
                "construct the receiver so the searched vertex/index is present "
                "in the field collection it scans; for this call, make "
                f"`{receiver}` contain a vertex/key/index matching `{args}`"
            )
        elif method in {"vertex_by_order", "neighbor_by_order"}:
            detail = (
                "construct earlier prefix values so this order/index argument "
                "is within the receiver collection bounds on the path to the "
                "target"
            )
        else:
            detail = "construct the receiver and argument values so the lookup/index is accepted"
        add(
            f"`{receiver}.{method}({args})` must return normally on the path "
            f"to the target; {detail}."
        )
        for item in _helper_lookup_obligation_items(
            crate_dir=crate_dir,
            receiver=receiver,
            method=method,
            args=args,
        ):
            add(item)

    for receiver, args in re.findall(
        r"\b([A-Za-z_][A-Za-z0-9_\.]*)\s*\.\s*get\s*"
        r"\(([^)]*)\)\s*(?:\.\s*[A-Za-z_][A-Za-z0-9_]*\s*\(\s*\)\s*)*"
        r"\.\s*expect\s*\(",
        code,
        re.S,
    ):
        args = " ".join(args.split())
        add(
            f"`{receiver}.get({args}).expect(...)` must succeed before the "
            "target; construct the receiver collection so this index/key is "
            "present on the path to the target."
        )

    for receiver, method, args in re.findall(
        r"\b([A-Za-z_][A-Za-z0-9_\.]*)\s*\.\s*"
        r"([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*"
        r"\.\s*expect\s*\(",
        code,
        re.S,
    ):
        if method in {"get", "checked_add", "checked_sub", "checked_mul"}:
            continue
        args = " ".join(args.split())
        rendered_args = f"({args})" if args else "()"
        add(
            f"`{receiver}.{method}{rendered_args}.expect(...)` must return "
            "normally before the target; construct the receiver/input state so "
            "this Option/Result-producing prefix call succeeds instead of "
            "panicking or returning early."
        )

    for type_name, arg_expr in re.findall(
        r"\b([A-Z][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)"
        r"::decode\s*\(\s*(.+?)\s*\)\s*\.expect\s*\(",
        code,
        re.S,
    ):
        arg = " ".join(arg_expr.split())
        add(
            f"`{type_name}::decode({arg}).expect(...)` must return normally "
            "before the target; construct that decode input as valid encoded "
            f"`{type_name}` bytes. If `{arg}` is cloned from a container/slice "
            "element selected earlier in the prefix, put the valid encoded "
            "bytes in that selected element while keeping the selector/index "
            "in bounds. Prefer bytes produced by visible safe constructors, "
            "encoders, or raw/encode accessors for the same type instead of "
            "hand-writing format bytes."
        )
        for selected_var, (collection, selector) in selected_gets.items():
            if re.search(rf"\b{re.escape(selected_var)}\b", arg):
                add(
                    f"`{arg}` is derived from `{selected_var}`, which came from "
                    f"`{collection}.get({selector}).expect(...)`; populate that "
                    f"exact selected collection element with valid encoded "
                    f"`{type_name}` bytes so the decode succeeds before the "
                    "target. Do not construct the decoded bytes only in an "
                    "unrelated local variable, and do not rely on a `Default` "
                    "receiver if this collection would remain empty. If the "
                    "selected collection is nested behind a helper method, "
                    "construct or mutate the helper's backing field on the "
                    "actual target receiver so this same expression observes "
                    "the non-empty collection."
                )

    unsafe_reads: dict[str, str] = {}

    def add_unsafe_read(var: str, ptr_expr: str) -> None:
        ptr = " ".join(ptr_expr.split())
        if not ptr:
            return
        unsafe_reads[var] = ptr
        add(
            f"The prefix reads `*{ptr}` into `{var}` before the target; make "
            f"`{ptr}` a valid readable pointer at that earlier read. Do not "
            "leave any receiver/raw-pointer field feeding this expression as "
            "`null_mut()`/`null()`; use a tiny local backing array/Vec or a "
            "visible safe pointer-producing value for prefix-only storage. If "
            "this prefix pointer expression uses the same index/count argument "
            "as the target unsafe call, make this prefix backing large enough "
            "for the chosen target value while keeping the actual target "
            "receiver/buffer relation independent and violating. When separate "
            "raw-pointer receiver fields are used with the same index/count, "
            "their backing allocations do not need to have the same length: a "
            "prefix receiver may be larger so execution reaches the target, "
            "while the target receiver is shorter so the target relation is "
            "false."
        )

    for var, ptr_expr in re.findall(
        r"\blet\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*unsafe\s*\{\s*\*\s*"
        r"([^;}]+?)\s*\}\s*;",
        code,
        re.S,
    ):
        add_unsafe_read(var, ptr_expr)

    for unsafe_block in re.findall(r"\bunsafe\s*\{(.*?)\}", code, re.S):
        for var, ptr_expr in re.findall(
            r"\blet\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\*\s*([^;]+?)\s*;",
            unsafe_block,
            re.S,
        ):
            add_unsafe_read(var, ptr_expr)
    for var, ptr in unsafe_reads.items():
        if re.search(rf"\.get\s*\(\s*{re.escape(var)}\s+as\s+usize\s*\)", code):
            add(
                f"`{var}` is later used as a `.get({var} as usize)` selector; "
                f"choose the byte/value read from `*{ptr}` so that lookup is "
                "in bounds on the path to the target. This selector is a safe "
                "prefix-routing value, not automatically the target unsafe "
                "call's offset/count argument; do not make it out of bounds "
                "unless the source code explicitly uses the same expression as "
                "the target argument."
            )
        if re.search(rf"\b{re.escape(ptr)}\s*\.\s*(?:add|offset)\s*\(", code):
            add(
                f"The same pointer `{ptr}` is read earlier and then used again "
                "at the target callsite; its backing storage only needs to be "
                "large enough for the earlier read and any selector value "
                "needed to reach the target. Do not enlarge that backing "
                "storage merely to preserve extra pointer headroom unless the "
                "prefix code requires it. Keep any earlier selector/read value "
                "valid for prefix lookups, and use the actual target call "
                "argument expression shown on the unsafe-call line for the "
                "counterexample."
            )

    for var, expr in re.findall(
        r"\blet\s+Some\s*\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*=\s*"
        r"(.+?)\s+else\s*\{\s*(?:return|break|continue)\b",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        add(
            f"`let Some({var}) = {condensed} else {{ ... }}` is a safe-prefix "
            "gate before the target; construct inputs so it evaluates to "
            "`Some` and execution continues toward the target."
        )

    for expr in re.findall(
        r"\bif\s+!\s*([^{};]+?)\s*\{\s*[^{}]*\bpanic!\s*\(",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        if condensed:
            add(
                f"`if !{condensed} {{ panic!(...) }}` guards the prefix path; "
                f"construct state so `{condensed}` is true before the target."
            )

    for receiver, method, args in re.findall(
        r"\b([A-Za-z_][A-Za-z0-9_\.]*)\s*\.\s*"
        r"([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)",
        code,
    ):
        if method in {
            "as_ptr",
            "as_mut_ptr",
            "clone",
            "copy",
            "default",
            "expect",
            "is_empty",
            "iter",
            "len",
            "new",
            "push",
            "unwrap",
            "opposite",
            "neighbor",
            "vertex_by_order",
            "neighbor_by_order",
            "get",
            "lookup",
            "find",
            "index_of",
            "position",
        }:
            continue
        args = " ".join(args.split())
        for item in _helper_prefix_call_obligation_items(
            crate_dir=crate_dir,
            receiver=receiver,
            method=method,
            args=args,
        ):
            add(item)

    assigned_calls: dict[str, str] = {}
    for var, expr in re.findall(
        r"\blet\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([^;]*\([^;]*\))\s*;",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        if condensed:
            assigned_calls[var] = condensed

    for var, expr in assigned_calls.items():
        escaped = re.escape(var)
        if re.search(
            rf"return\s+if\s+{escaped}\s*\{{\s*false\s*\}}\s*else\s*\{{",
            code,
            re.S,
        ):
            add(
                f"The target is in the `else` branch controlled by `{var}`; "
                f"construct input/receiver fields so `{expr}` returns false."
            )
        if re.search(
            rf"\bif\s+{escaped}\s*\{{\s*(?:return\s+)?false\s*;?\s*\}}\s*else\s*\{{",
            code,
            re.S,
        ):
            add(
                f"The target is in the branch taken when `{var}` is false; "
                f"construct input/receiver fields so `{expr}` returns false."
            )
        if re.search(
            rf"\bif\s+{escaped}\s*\{{[^{{}}]*\b(?:return|break|continue)\b",
            code,
            re.S,
        ):
            add(
                f"`{expr}` controls an early-exit prefix branch; choose values "
                f"so `{var}` takes the path that continues to the target."
            )

    for expr in re.findall(
        r"\bif\s+([^{};]*\([^{};]*\))\s*\{\s*(?:return\s+)?false\s*;?\s*\}\s*else\s*\{",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        if condensed:
            add(
                f"The target is in the branch taken when `{condensed}` is false; "
                f"construct input/receiver fields so `{condensed}` returns false."
            )

    for expr in re.findall(
        r"\bif\s+([^{};]*\([^{};]*\))\s*\{\s*return\s+false\s*;?\s*\}",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        if condensed:
            add(
                f"The target is after a false-return early branch controlled "
                f"by `{condensed}`; construct input/receiver fields so "
                f"`{condensed}` returns false."
            )

    for expr in re.findall(
        r"\bif\s+([^{};]*\([^{};]*\))\s*\{\s*(?:return|break|continue)\b",
        code,
        re.S,
    ):
        condensed = " ".join(expr.split())
        if condensed:
            add(
                f"`{condensed}` controls an early-exit prefix branch; choose "
                "values so execution continues toward the target callsite."
            )

    if re.search(r"return\s+if\s+[^{]+\{\s*false\s*\}\s*else\s*\{", code, re.S):
        add(
            "The target is in the `else` branch of a `return if ... { false } "
            "else { ... }`; choose input/receiver fields so the condition is "
            "false and execution enters the target-containing branch."
        )
    elif re.search(r"\bif\s+[^{]+\{\s*(?:return\s+)?false\s*;?\s*\}\s*else\s*\{", code, re.S):
        add(
            "The target is after an `if` whose true branch returns/produces "
            "`false`; choose values so execution follows the branch containing "
            "the target."
        )

    return obligations


def _prefix_path_obligations(
    prefix_block: str, *, crate_dir: Path | None = None
) -> str | None:
    obligations = _prefix_path_obligation_items(prefix_block, crate_dir=crate_dir)
    if not obligations:
        return None
    lines = [
        "Inferred prefix path obligations from the actual-containing function. "
        "These are not the unsafe API rule; they are safe-path conditions needed "
        "to reach the target callsite:"
    ]
    lines.extend(f"- {item}" for item in obligations[:20])
    return "\n".join(lines)


def _source_function_signature_text_for_line(
    *, crate_dir: Path, path_value: object, line_value: object
) -> str | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None
    for index in range(line_value - 1, -1, -1):
        line = lines[index].strip()
        if re.search(r"\bunsafe\s+fn\b", line):
            return None
        match = re.search(
            r"((?:pub(?:\([^)]*\))?\s+)?fn\s+[A-Za-z_][A-Za-z0-9_]*\s*\([^)]*\))",
            line,
        )
        if match:
            return match.group(1)
    return None


def _direct_actual_function_hint(
    *,
    crate_dir: Path,
    root_callsite: dict[str, Any],
    unsafe_callsite: dict[str, Any],
) -> str | None:
    root_path = root_callsite.get("path")
    unsafe_path = unsafe_callsite.get("path")
    if root_path != unsafe_path:
        return None
    signature = _source_function_signature_text_for_line(
        crate_dir=crate_dir,
        path_value=unsafe_path,
        line_value=unsafe_callsite.get("line"),
    )
    if not signature:
        return None
    name_match = re.search(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", signature)
    name = name_match.group(1) if name_match else "<actual function>"
    call_shape = f"`receiver.{name}(...)`" if re.search(r"\bself\b", signature) else f"`{name}(...)`"
    return (
        "Direct inner-target-containing safe function option. The inner target call "
        f"is inside safe function `{signature}`. Because the generated testcase "
        "is injected into the same source file/module as this function, it may "
        f"call the safe actual-containing function directly, e.g. {call_shape}, "
        "if that type-checks. This is often better than calling the root wrapper "
        "when the wrapper performs earlier guarded lookups or loop traversal. "
        "Construct the receiver/parameters for this function, preserve the safe "
        "prefix conditions needed to execute from the function entry to the "
        "inner target call, and make the bound target arguments form the "
        "counterexample. If the prefix calls helpers such as `opposite`, "
        "`neighbor`, indexing, lookup, or branch-condition functions before the "
        "target, construct the involved fields so those helpers do not panic or "
        "return early. Do not reuse identical placeholder values for distinct "
        "semantic roles when the prefix computes an opposite element or branch "
        "condition from them; use distinct concrete values when needed to drive "
        "the path into the inner target call."
    )


def _direct_actual_call_plan(
    *, crate_dir: Path, target: dict[str, Any]
) -> list[str] | None:
    callsite = target.get("callsite")
    unsafe_callsite = target.get("unsafe_callsite")
    if not isinstance(callsite, dict) or not isinstance(unsafe_callsite, dict):
        return None
    if callsite.get("path") != unsafe_callsite.get("path"):
        return None
    signature = _source_function_signature_text_for_line(
        crate_dir=crate_dir,
        path_value=unsafe_callsite.get("path"),
        line_value=unsafe_callsite.get("line"),
    )
    if not signature:
        return None
    name_match = re.search(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", signature)
    name = name_match.group(1) if name_match else "<actual function>"
    call_phrase = (
        f"call safe actual-containing method `receiver.{name}(...)` directly"
        if re.search(r"\bself\b", signature)
        else f"call safe actual-containing function `{name}(...)` directly"
    )
    plan = [
        f"construct receiver and parameters for actual safe function `{signature}`",
        "preserve prefix helper lookups/branches before the target (for example, values used by `opposite`, `neighbor`, indexing, or condition helpers must be present/valid until the inner target call)",
        "make the bound target-call relation form the counterexample at the target, not merely at the root wrapper",
        call_phrase,
    ]
    prefix = _source_function_prefix_for_line(
        crate_dir=crate_dir,
        path_value=unsafe_callsite.get("path"),
        line_value=unsafe_callsite.get("line"),
    )
    if prefix:
        for item in _prefix_path_obligation_items(prefix, crate_dir=crate_dir)[:6]:
            plan.insert(-2, "satisfy prefix path obligation: " + item)
    return plan


def _source_line_for_line(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
) -> str | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None
    return lines[line_value - 1].strip()


def _raw_source_line_for_line(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
) -> str | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None
    return lines[line_value - 1]


def _target_expression_hint_for_callsite(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
    col_value: object,
) -> str | None:
    """Best-effort hint for the source expression selected by line/column.

    This is intentionally syntactic.  It is not a POC and does not encode any
    rule-specific knowledge; it just helps the testcase generator distinguish
    multiple target expressions that share one source line.
    """

    if not isinstance(col_value, int):
        return None
    # Callsite columns are reported against the original source line, including
    # indentation.  Do not use the display helper that strips whitespace here:
    # otherwise same-line calls can be shifted onto the wrong expression.
    source_line = _raw_source_line_for_line(
        crate_dir=crate_dir,
        path_value=path_value,
        line_value=line_value,
    )
    if not source_line:
        return None
    expression = _source_expression_at_callsite_column(
        source_line=source_line,
        col_value=col_value,
    )
    if expression is None:
        return None
    pieces = [
        "Target expression selected by the callsite line/column:",
        f"- column: `{col_value}`",
        f"- expression: `{expression}`",
    ]
    method = re.fullmatch(
        r"(?P<receiver>[A-Za-z_][A-Za-z0-9_]*)\."
        r"(?P<method>[A-Za-z_][A-Za-z0-9_]*)\((?P<args>[^()]*)\)",
        expression,
    )
    if method:
        pieces.append(f"- receiver expression: `{method.group('receiver')}`")
        pieces.append(f"- method/function: `{method.group('method')}`")
        args = [arg.strip() for arg in method.group("args").split(",") if arg.strip()]
        for index, arg in enumerate(args, start=1):
            pieces.append(f"- call argument {index}: `{arg}`")
    pieces.append(
        "Choose caller receiver/argument values so this expression at this "
        "target callsite becomes the counterexample, while all earlier guards "
        "needed to reach it still pass."
    )
    return "\n".join(pieces)


def _source_expression_at_callsite_column(
    *,
    source_line: str,
    col_value: int,
) -> str | None:
    col_index = max(0, col_value - 1)
    candidates: list[tuple[int, int, str]] = []
    patterns = [
        r"[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*\([^()]*\)",
        r"[A-Za-z_][A-Za-z0-9_:]*\([^()]*\)",
    ]
    for pattern in patterns:
        for match in re.finditer(pattern, source_line):
            start, end = match.span()
            if start <= col_index < end or abs(start - col_index) <= 1:
                candidates.append((start, end, match.group(0).strip()))
    if not candidates:
        return None
    _, _, expression = min(
        candidates,
        key=lambda item: (item[1] - item[0], abs(item[0] - col_index)),
    )
    return expression


def target_call_arg_source_map(
    *,
    crate_dir: Path,
    target: dict[str, Any],
) -> dict[str, str]:
    """Map KLEE get_arg(N) names to the selected source expression pieces.

    This is intentionally syntactic and best-effort.  It only helps the LLM
    relate observed rerun argument values back to source-level caller values;
    it does not encode rule-specific counterexample logic.
    """

    callsite = target.get("callsite") if isinstance(target, dict) else None
    if not isinstance(callsite, dict):
        return {}
    source_line = _raw_source_line_for_line(
        crate_dir=crate_dir,
        path_value=callsite.get("path"),
        line_value=callsite.get("line"),
    )
    col_value = callsite.get("col")
    if not source_line or not isinstance(col_value, int):
        return {}
    expression = _source_expression_at_callsite_column(
        source_line=source_line,
        col_value=col_value,
    )
    if not expression:
        return {}
    mapping: dict[str, str] = {}
    method = re.fullmatch(
        r"(?P<receiver>[A-Za-z_][A-Za-z0-9_]*)\."
        r"(?P<method>[A-Za-z_][A-Za-z0-9_]*)\((?P<args>[^()]*)\)",
        expression,
    )
    if method:
        mapping["get_arg(0)"] = method.group("receiver").strip()
        args = [arg.strip() for arg in method.group("args").split(",") if arg.strip()]
        for index, arg in enumerate(args, start=1):
            mapping[f"get_arg({index})"] = arg
        return mapping
    function = re.fullmatch(
        r"[A-Za-z_][A-Za-z0-9_:]*\((?P<args>[^()]*)\)",
        expression,
    )
    if function:
        args = [arg.strip() for arg in function.group("args").split(",") if arg.strip()]
        for index, arg in enumerate(args):
            mapping[f"get_arg({index})"] = arg
    return mapping


def _retry_forbidden_zero_sources(retry_feedback: str | None) -> set[str]:
    if not retry_feedback:
        return set()
    sources: set[str] = set()
    for match in re.finditer(
        r"HARD RETRY CONSTRAINT:\s*`([^`]+)`\s+must not be `0`",
        retry_feedback,
    ):
        source = match.group(1).strip()
        if source:
            sources.add(source)
    for match in re.finditer(
        r"this failed observation had `([^`]+)\s*=\s*0`",
        retry_feedback,
    ):
        source = match.group(1).strip()
        if source:
            sources.add(source)
    return sources


def _rust_args_for_simple_call(args_text: str) -> list[str]:
    args: list[str] = []
    current: list[str] = []
    depth = 0
    for ch in args_text:
        if ch in "([{":
            depth += 1
        elif ch in ")]}" and depth > 0:
            depth -= 1
        if ch == "," and depth == 0:
            arg = "".join(current).strip()
            if arg:
                args.append(arg)
            current = []
            continue
        current.append(ch)
    arg = "".join(current).strip()
    if arg:
        args.append(arg)
    return args


def _generated_expr_is_definitely_zero(code: str, expr: str) -> bool:
    stripped = expr.strip()
    if re.fullmatch(r"0(?:usize|u(?:8|16|32|64|128)|i(?:8|16|32|64|128))?", stripped):
        return True
    if re.fullmatch(r"0_(?:usize|u(?:8|16|32|64|128)|i(?:8|16|32|64|128))", stripped):
        return True
    if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", stripped):
        return False
    init = re.search(
        rf"\blet\s+(?:mut\s+)?{re.escape(stripped)}(?:\s*:\s*[^=;]+)?\s*=\s*"
        r"0(?:_usize|usize|u(?:8|16|32|64|128)|i(?:8|16|32|64|128))?\s*;",
        code,
    )
    return bool(init)


def _validate_retry_zero_constraints(
    *,
    code: str,
    crate_dir: Path,
    target: dict[str, Any],
    retry_feedback: str | None,
) -> None:
    forbidden_sources = _retry_forbidden_zero_sources(retry_feedback)
    if not forbidden_sources:
        return
    arg_sources = target_call_arg_source_map(crate_dir=crate_dir, target=target)
    if not arg_sources:
        return
    caller = target.get("caller") if isinstance(target, dict) else None
    caller_name = caller.get("name") if isinstance(caller, dict) else None
    if not isinstance(caller_name, str) or not caller_name:
        return
    leaf = caller_name.rsplit("::", 1)[-1]
    if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", leaf):
        return
    for arg_name, source_expr in sorted(arg_sources.items()):
        if source_expr not in forbidden_sources:
            continue
        match = re.fullmatch(r"get_arg\((\d+)\)", arg_name)
        if not match:
            continue
        klee_arg_index = int(match.group(1))
        if klee_arg_index == 0:
            continue
        rust_arg_index = klee_arg_index - 1
        call_pattern = re.compile(
            rf"(?:\.|::|\b){re.escape(leaf)}\s*\((?P<args>[^;{{}}]*)\)",
            re.S,
        )
        for call in call_pattern.finditer(code):
            args = _rust_args_for_simple_call(call.group("args"))
            if rust_arg_index >= len(args):
                continue
            actual = args[rust_arg_index]
            if _generated_expr_is_definitely_zero(code, actual):
                raise RuntimeError(
                    "generated testcase repeats previous KLEE non-SAT value: "
                    f"target argument `{arg_name}` maps to source expression "
                    f"`{source_expr}`, previous feedback requires it to be "
                    f"non-zero, but generated call `{leaf}(...)` passes "
                    f"`{actual}`. Generate a new testcase that passes a "
                    "concrete non-zero value for this argument while keeping "
                    "prefix operations valid."
                )


def _validate_visible_dependency_paths(
    *,
    code: str,
    crate_dir: Path,
    target: dict[str, Any],
) -> None:
    callsite = target.get("callsite") if isinstance(target, dict) else None
    if not isinstance(callsite, dict):
        return
    path_value = callsite.get("path")
    if not isinstance(path_value, str) or not path_value:
        return
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return
    if not path.is_file():
        return
    try:
        source = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return
    dependency_crates = {
        name.replace("-", "_")
        for name in _cargo_manifest_dependency_names(crate_dir)
        if isinstance(name, str)
    }
    for dep in sorted(dependency_crates):
        if not re.search(rf"\b{re.escape(dep)}::", source):
            continue
        invented = f"crate::{dep}::"
        if invented in code:
            raise RuntimeError(
                "generated testcase invented an in-crate path for an external "
                f"dependency: `{invented}...`. The target module visibly uses "
                f"`{dep}::...`; because the testcase is injected into that same "
                f"module, use `{dep}::...` directly."
            )


def _validate_pointer_to_pointer_casts(code: str) -> None:
    """Reject obvious element-buffer-to-pointer-array casts in generated POCs."""

    for match in re.finditer(
        r"\b(?P<base>[A-Za-z_][A-Za-z0-9_]*)\s*"
        r"\.\s*as_(?:mut_)?ptr\s*\(\s*\)\s*as\s*\*mut\s+\*mut\b",
        code,
    ):
        base = match.group("base")
        init = re.search(
            rf"\blet\s+(?:mut\s+)?{re.escape(base)}(?:\s*:\s*[^=;]+)?"
            r"\s*=\s*(?P<expr>[^;]+);",
            code,
            re.S,
        )
        if not init:
            continue
        expr = init.group("expr")
        if re.search(r"\.\s*as_(?:mut_)?ptr\s*\(\s*\)", expr):
            continue
        raise RuntimeError(
            "generated testcase casts an ordinary element buffer directly to a "
            f"pointer-to-pointer using `{match.group(0)}`. For `*mut *mut T` "
            "fields, build separate safe row/element backing arrays or Vecs, "
            "then build an array/Vec of their pointers and pass that pointer "
            "array instead."
        )


def _method_call_hint_for_line(source_line: str) -> str | None:
    """Best-effort human hint for the receiver/arguments in a Rust method call.

    This is intentionally syntactic and advisory only. It helps the LLM focus
    on the expression that feeds the target call without baking in any
    crate-specific POC logic.
    """

    matches = list(
        re.finditer(
            r"(?P<receiver>[A-Za-z_][A-Za-z0-9_:]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)"
            r"\.(?P<method>[A-Za-z_][A-Za-z0-9_]*)\((?P<args>[^()]*)\)",
            source_line,
        )
    )
    if not matches:
        return None
    match = matches[-1]
    receiver = match.group("receiver").strip()
    method = match.group("method").strip()
    args = [arg.strip() for arg in match.group("args").split(",") if arg.strip()]
    pieces = [
        "Target expression checklist from the actual call line:",
        f"- receiver expression: `{receiver}`",
        f"- method/function: `{method}`",
    ]
    if args:
        for index, arg in enumerate(args, start=1):
            pieces.append(f"- call argument {index}: `{arg}`")
    else:
        pieces.append("- call arguments: none")
    pieces.append(
        "Choose caller receiver/argument values so these expressions at the "
        "target call become the counterexample."
    )
    return "\n".join(pieces)


def _source_function_signature_for_line(
    *,
    crate_dir: Path,
    path_value: object,
    line_value: object,
) -> tuple[str, list[dict[str, str]], str] | None:
    if not isinstance(path_value, str) or not isinstance(line_value, int):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None
    if line_value < 1 or line_value > len(lines):
        return None

    fn_start = None
    for index in range(line_value - 1, -1, -1):
        if re.search(r"\bfn\s+[A-Za-z_][A-Za-z0-9_]*\b", lines[index]):
            fn_start = index
            break
    if fn_start is None:
        return None

    signature_lines: list[str] = []
    paren_depth = 0
    saw_paren = False
    for line in lines[fn_start : min(len(lines), fn_start + 30)]:
        signature_lines.append(line.strip())
        for char in line:
            if char == "(":
                paren_depth += 1
                saw_paren = True
            elif char == ")" and paren_depth:
                paren_depth -= 1
        if saw_paren and paren_depth == 0:
            break
    signature = " ".join(part for part in signature_lines if part).strip()
    fn_match = re.search(r"\bfn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\b", signature)
    if not fn_match:
        return None
    paren_start = signature.find("(", fn_match.end())
    if paren_start < 0:
        return None
    depth = 0
    paren_end = -1
    for index in range(paren_start, len(signature)):
        char = signature[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                paren_end = index
                break
    if paren_end < 0:
        return None

    params: list[dict[str, str]] = []
    for raw in _split_top_level_commas(signature[paren_start + 1 : paren_end]):
        cleaned = re.sub(r"^\s*(?:mut\s+)?", "", raw.strip())
        if cleaned in {"self", "&self", "&mut self", "mut self"}:
            params.append({"name": "self", "ty": "Self", "raw": raw.strip()})
            continue
        if ":" not in cleaned:
            continue
        name_part, ty_part = cleaned.split(":", 1)
        name = _terminal_name(name_part) or name_part.strip()
        ty = ty_part.strip()
        if name and ty:
            params.append({"name": name, "ty": ty, "raw": raw.strip()})
    return fn_match.group("name"), params, signature


def _method_calls_for_line(source_line: str) -> list[dict[str, Any]]:
    calls: list[dict[str, Any]] = []
    for match in re.finditer(
        r"(?P<receiver>[A-Za-z_][A-Za-z0-9_:]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)"
        r"\.(?P<method>[A-Za-z_][A-Za-z0-9_]*)\(",
        source_line,
    ):
        paren_start = match.end() - 1
        depth = 0
        paren_end = -1
        for index in range(paren_start, len(source_line)):
            char = source_line[index]
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
                if depth == 0:
                    paren_end = index
                    break
        if paren_end < 0:
            continue
        args = _split_top_level_commas(source_line[paren_start + 1 : paren_end])
        calls.append(
            {
                "receiver": match.group("receiver").strip(),
                "method": match.group("method").strip(),
                "args": args,
                "text": source_line[match.start() : paren_end + 1].strip(),
            }
        )
    return calls


def _wrapper_argument_bridge_hint(
    *,
    crate_dir: Path,
    root_callsite: dict[str, Any],
    unsafe_callsite: dict[str, Any],
) -> str | None:
    root_line = _source_line_for_line(
        crate_dir=crate_dir,
        path_value=root_callsite.get("path"),
        line_value=root_callsite.get("line"),
    )
    unsafe_line = _source_line_for_line(
        crate_dir=crate_dir,
        path_value=unsafe_callsite.get("path"),
        line_value=unsafe_callsite.get("line"),
    )
    signature = _source_function_signature_for_line(
        crate_dir=crate_dir,
        path_value=unsafe_callsite.get("path"),
        line_value=unsafe_callsite.get("line"),
    )
    if not root_line or not unsafe_line or not signature:
        return None
    fn_name, params, signature_text = signature
    calls = [
        call
        for call in _method_calls_for_line(root_line)
        if call.get("method") == fn_name
    ]
    if not calls:
        return None
    call = calls[-1]
    non_self_params = [param for param in params if param.get("name") != "self"]
    args = call.get("args") if isinstance(call.get("args"), list) else []
    mappings = [
        (param.get("name", ""), args[index])
        for index, param in enumerate(non_self_params)
        if index < len(args) and param.get("name")
    ]
    if not mappings:
        return None

    lines = [
        "Wrapper-to-actual argument bridge inferred from source syntax. The "
        "root call reaches the inner target-containing function through this "
        "call, so mutate the root-side value that maps to the inner target "
        "argument rather than a different object:",
        f"- root call expression: `{call.get('text')}`",
        f"- actual function signature: `{signature_text}`",
    ]
    for param_name, arg_expr in mappings:
        lines.append(f"- actual parameter `{param_name}` comes from root call argument `{arg_expr}`")
    field_bridges = []
    for param_name, arg_expr in mappings:
        for field_match in re.finditer(
            rf"\b{re.escape(param_name)}\.([A-Za-z_][A-Za-z0-9_]*)\b",
            unsafe_line,
        ):
            field = field_match.group(1)
            field_bridges.append(
                f"`{param_name}.{field}` at the inner target call is the "
                f"`{field}` field of root-side `{arg_expr}`"
            )
    if field_bridges:
        lines.append("- field bridge: " + "; ".join(dict.fromkeys(field_bridges)))
    lines.append(
        "If that root-side value was itself loaded from a container element or "
        "local variable before the root call, set that same element/local field "
        "in the constructed receiver state; changing a sibling element does not "
        "change the inner target argument."
    )
    return "\n".join(lines)


def _instrumented_bound_arg_window(
    *,
    crate_dir: Path,
    path_value: object,
    callsite_id: object,
    radius: int = 6,
) -> str | None:
    if (
        not isinstance(path_value, str)
        or not isinstance(callsite_id, str)
        or not callsite_id
    ):
        return None
    path = (crate_dir / path_value).resolve()
    try:
        path.relative_to(crate_dir.resolve())
    except ValueError:
        return None
    if not path.is_file():
        return None
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return None

    literal = f'callsite!("{callsite_id}")'
    candidates: list[tuple[int, int]] = []
    for index, line in enumerate(lines):
        if literal not in line:
            continue
        start = max(0, index - radius)
        end = min(len(lines), index + radius + 1)
        score = sum("bind_arg" in item for item in lines[start:end])
        candidates.append((score, index))
    if not candidates:
        return None
    _, selected = max(candidates, key=lambda item: (item[0], item[1]))
    start = max(0, selected - radius)
    end = min(len(lines), selected + radius + 1)
    if not any("bind_arg" in item for item in lines[start:end]):
        return None
    width = len(str(end))
    body = "\n".join(
        f"{line_no:>{width}}: {lines[line_no - 1]}"
        for line_no in range(start + 1, end + 1)
    )
    return f"{path_value}:{selected + 1}\n```rust\n{body}\n```"


def build_target_context_block(
    *,
    crate_dir: Path,
    target: dict[str, Any],
    instrumented_crate_dir: Path | None = None,
) -> str:
    """Describe the root callsite and, when distinct, the inner target call.

    MIRScan may report a source-level safe wrapper call as the comparable
    callsite while the innermost target operation is in a nested callee.
    The testcase generator needs both: the root location tells it which caller
    path to exercise, and the inner target-call window shows which receiver
    field/index/length eventually becomes the target argument.
    """

    if not isinstance(target, dict):
        return ""
    pieces: list[str] = []
    caller = target.get("caller") if isinstance(target.get("caller"), dict) else {}
    if isinstance(caller, dict):
        caller_name = caller.get("name")
        if isinstance(caller_name, str) and caller_name:
            pieces.append(f"Target caller: `{caller_name}`.")

    callsite = target.get("callsite") if isinstance(target.get("callsite"), dict) else {}
    visible_path_context = _module_visible_path_context(
        crate_dir=crate_dir,
        path_value=callsite.get("path") if isinstance(callsite, dict) else None,
    )
    if visible_path_context:
        pieces.append(visible_path_context)
    target_expression_hint = _target_expression_hint_for_callsite(
        crate_dir=crate_dir,
        path_value=callsite.get("path") if isinstance(callsite, dict) else None,
        line_value=callsite.get("line") if isinstance(callsite, dict) else None,
        col_value=callsite.get("col") if isinstance(callsite, dict) else None,
    )
    if target_expression_hint:
        pieces.append(target_expression_hint)
    root_window = _source_window_for_line(
        crate_dir=crate_dir,
        path_value=callsite.get("path") if isinstance(callsite, dict) else None,
        line_value=callsite.get("line") if isinstance(callsite, dict) else None,
        radius=8,
    )
    if root_window:
        pieces.append(
            "Root callsite/source location to reach. This can be a wrapper "
            "expression rather than the innermost target expression:\n" + root_window
        )

    unsafe_callsite = (
        target.get("unsafe_callsite")
        if isinstance(target.get("unsafe_callsite"), dict)
        else None
    )
    if unsafe_callsite:
        unsafe_line = _source_line_for_line(
            crate_dir=crate_dir,
            path_value=unsafe_callsite.get("path"),
            line_value=unsafe_callsite.get("line"),
        )
        if unsafe_line:
            pieces.append(
                "Exact inner target-call source line. The receiver and "
                "argument expressions on this line determine the target "
                "counterexample shape; if an argument expression is a visible "
                "struct field, array element, method result, or caller "
                "parameter, construct that source "
                f"value directly:\n`{unsafe_line}`"
            )
            method_hint = _method_call_hint_for_line(unsafe_line)
            if method_hint:
                pieces.append(method_hint)
        unsafe_window = _source_window_for_line(
            crate_dir=crate_dir,
            path_value=unsafe_callsite.get("path"),
            line_value=unsafe_callsite.get("line"),
            radius=12,
        )
        if unsafe_window:
            pieces.append(
                "Inner target callsite/source location whose call arguments "
                "must be a counterexample. Work backward from this expression "
                "to construct the target caller receiver/arguments:\n"
                + unsafe_window
            )
        unsafe_prefix = _source_function_prefix_for_line(
            crate_dir=crate_dir,
            path_value=unsafe_callsite.get("path"),
            line_value=unsafe_callsite.get("line"),
        )
        if unsafe_prefix:
            pieces.append(
                "Inner target-call containing function prefix. Preserve the "
                "safe preconditions needed to execute from the start of this "
                "function to the target call, while making the target call "
                "a counterexample:\n"
                + unsafe_prefix
            )
            prefix_obligations = _prefix_path_obligations(
                unsafe_prefix,
                crate_dir=crate_dir,
            )
            if prefix_obligations:
                pieces.append(prefix_obligations)
            helper_context = _helper_source_context_for_prefix(
                crate_dir=crate_dir,
                prefix_block=unsafe_prefix,
            )
            if helper_context:
                pieces.append(helper_context)
        direct_actual_hint = _direct_actual_function_hint(
            crate_dir=crate_dir,
            root_callsite=callsite,
            unsafe_callsite=unsafe_callsite,
        )
        if direct_actual_hint:
            pieces.append(direct_actual_hint)
        bridge_hint = _wrapper_argument_bridge_hint(
            crate_dir=crate_dir,
            root_callsite=callsite,
            unsafe_callsite=unsafe_callsite,
        )
        if bridge_hint:
            pieces.append(bridge_hint)
        bound_window = _instrumented_bound_arg_window(
            crate_dir=instrumented_crate_dir or crate_dir,
            path_value=unsafe_callsite.get("path") or callsite.get("path"),
            callsite_id=callsite.get("id"),
        )
        if bound_window:
            pieces.append(
                "Instrumented bound-argument map from autoinj. This is the "
                "authoritative mapping for target argument `get_arg(N)`: "
                "`bind_arg_*(N, expr)` means `get_arg(N)` is `expr`, and "
                "`bind_arg_*_value(N, &tmp)` means `get_arg(N)` is the current "
                "value of `tmp`. Generate the counterexample for these bound "
                "expressions, not just for Rust positional arguments:\n"
                + bound_window
            )
    else:
        root_prefix = _source_function_prefix_for_line(
            crate_dir=crate_dir,
            path_value=callsite.get("path"),
            line_value=callsite.get("line"),
        )
        if root_prefix:
            pieces.append(
                "Target caller prefix. Preserve the safe preconditions needed "
                "to execute from the function entry to the target callsite, "
                "while making the target callsite a counterexample:\n"
                + root_prefix
            )
            prefix_obligations = _prefix_path_obligations(
                root_prefix,
                crate_dir=crate_dir,
            )
            if prefix_obligations:
                pieces.append(prefix_obligations)
            helper_context = _helper_source_context_for_prefix(
                crate_dir=crate_dir,
                prefix_block=root_prefix,
            )
            if helper_context:
                pieces.append(helper_context)

    dependency_context = _external_type_source_context(
        crate_dir=crate_dir,
        seed_text="\n\n".join(pieces),
    )
    if dependency_context:
        pieces.append(dependency_context)
    construction_context = _source_construction_example_context(
        crate_dir=crate_dir,
        seed_text="\n\n".join(pieces),
    )
    if construction_context:
        pieces.append(construction_context)

    if len(pieces) <= 1:
        return "\n".join(pieces)
    if unsafe_callsite:
        pieces.append(
            "When the root location and inner target location differ, do not stop "
            "at merely reaching the root wrapper. Construct caller state so the "
            "inner target call's bound arguments form the target counterexample. "
            "A value used earlier to reach the root wrapper may need to stay valid; "
            "only reuse it for the inner target-call argument when the source code "
            "forces both expressions to be equal."
        )
    return "\n\n".join(pieces)


def _source_level_bound_relation_hint(
    *, target_context: str, klee_witness: str | None
) -> str | None:
    if not klee_witness or "bind_arg" not in target_context:
        return None
    relation = _counterexample_get_arg_relation_from_witness(klee_witness)
    if relation is None:
        return None
    left, op, right = relation

    temp_values: dict[str, str] = {}
    bound_values: dict[str, str] = {}
    for raw_line in target_context.splitlines():
        line = re.sub(r"^\s*\d+:\s*", "", raw_line).strip()
        assign = re.search(r"\blet\s+(__klee_arg\d+)\s*=\s*(.+?);", line)
        if assign:
            temp_values[assign.group(1)] = assign.group(2).strip()
            continue
        by_ref = re.search(
            r"\bbind_arg_[A-Za-z0-9_]+_value\(\s*(\d+)\s*,\s*&"
            r"([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*;",
            line,
        )
        if by_ref:
            arg_index, temp = by_ref.groups()
            bound_values[f"get_arg({arg_index})"] = temp_values.get(temp, temp)
            continue
        by_expr = re.search(
            r"\bbind_arg_[A-Za-z0-9_]+\(\s*(\d+)\s*,\s*(.+)\)\s*;",
            line,
        )
        if by_expr:
            arg_index, expr = by_expr.groups()
            bound_values[f"get_arg({arg_index})"] = expr.strip()

    left_expr = bound_values.get(left)
    right_expr = bound_values.get(right)
    if not left_expr or not right_expr:
        return None
    return (
        "Derived source-level target relation from autoinj "
        f"bound-argument map: construct caller state so `{left_expr} {op} "
        f"{right_expr}` is true at the actual target call. A testcase that "
        f"reaches the call with `{left_expr}` still on the wrong side of "
        f"`{right_expr}` is invalid."
    )


def _counterexample_get_arg_relation_from_witness(
    klee_witness: str,
) -> tuple[str, str, str] | None:
    relation_match = re.search(
        r"testcase should make `(get_arg\(\d+\))\s*(>=|<=|>|<|==|!=)\s*"
        r"(get_arg\(\d+\))` true",
        klee_witness,
    )
    if relation_match:
        return relation_match.groups()  # type: ignore[return-value]

    ast = _rule_ast_from_witness(klee_witness)
    if ast is None:
        return None
    for key in ("simplified", "original"):
        expr = ast.get(key) if isinstance(ast, dict) else None
        relation = _counterexample_get_arg_relation_from_ast(expr)
        if relation is not None:
            return relation
    return _counterexample_get_arg_relation_from_ast(ast)


def _rule_ast_from_witness(klee_witness: str) -> dict[str, Any] | None:
    marker = "Rule DSL AST"
    marker_pos = klee_witness.find(marker)
    if marker_pos < 0:
        return None
    json_pos = klee_witness.find("{", marker_pos)
    if json_pos < 0:
        return None
    try:
        value, _ = json.JSONDecoder().raw_decode(klee_witness[json_pos:])
    except json.JSONDecodeError:
        return None
    return value if isinstance(value, dict) else None


def _counterexample_get_arg_relation_from_ast(
    expr: Any,
) -> tuple[str, str, str] | None:
    if not isinstance(expr, dict) or expr.get("type") != "binary":
        return None
    op = expr.get("op")
    inverse = {
        "<": ">=",
        "<=": ">",
        ">": "<=",
        ">=": "<",
        "==": "!=",
        "!=": "==",
    }.get(op)
    if inverse is None:
        return None
    left = _get_arg_expr_name(expr.get("left"))
    right = _get_arg_expr_name(expr.get("right"))
    if left is None or right is None:
        return None
    return left, inverse, right


def _get_arg_expr_name(expr: Any) -> str | None:
    if not isinstance(expr, dict):
        return None
    if expr.get("type") == "simplified_var":
        name = expr.get("name")
        return name if isinstance(name, str) and re.fullmatch(r"get_arg\(\d+\)", name) else None
    if expr.get("type") != "call" or expr.get("name") != "get_arg":
        return None
    args = expr.get("args")
    if (
        not isinstance(args, list)
        or len(args) != 1
        or not isinstance(args[0], dict)
        or args[0].get("type") != "literal"
    ):
        return None
    value = args[0].get("value")
    if isinstance(value, int):
        return f"get_arg({value})"
    if isinstance(value, str) and value.isdigit():
        return f"get_arg({value})"
    return None


def generate_safe_testcase(
    *,
    crate_dir: Path,
    source_crate_dir: Path | None = None,
    target: dict[str, Any],
    rule: dict[str, Any],
    chain: dict[str, Any],
    report_path: Path | None = None,
    model: str,
    artifacts_dir: Path,
    injection: TestcaseInjection,
    llm: Any | None = None,
    klee_witness: str | None = None,
    retry_feedback: str | None = None,
    attempt: int = 1,
    context_mode: str = "slice",
) -> str:
    testcase_chains = select_testcase_control_chains(chain)
    reproduction_plans = build_candidate_reproduction_plans(
        crate_dir=crate_dir,
        target=target,
        chain=chain,
        testcase_chains=testcase_chains,
        report=_load_mirscan_report(report_path),
    )
    target_context = build_target_context_block(
        crate_dir=source_crate_dir or crate_dir,
        target=target,
        instrumented_crate_dir=crate_dir,
    )
    relation_hint = _source_level_bound_relation_hint(
        target_context=target_context,
        klee_witness=klee_witness,
    )
    if relation_hint:
        target_context = target_context + "\n\n" + relation_hint
    rust_context = build_witness_guided_rust_context(
        crate_dir,
        target=target,
        control_chains=testcase_chains,
        mode=context_mode,
    )
    prompt = build_testcase_prompt(
        rust_context=rust_context.text,
        call_chain=json.dumps(testcase_chains, indent=2),
        callsite=json.dumps(target, indent=2),
        safety_requirement=str(rule.get("rule", "")),
        function_name=injection.function,
        feature_name=injection.feature,
        klee_witness=klee_witness,
        retry_feedback=retry_feedback,
        reproduction_plans=reproduction_plans,
        target_context=target_context,
    )
    artifacts_dir.mkdir(parents=True, exist_ok=True)
    (artifacts_dir / f"testcase-context-attempt-{attempt}.txt").write_text(
        rust_context.text + "\n", encoding="utf-8"
    )
    (artifacts_dir / f"testcase-context-stats-attempt-{attempt}.json").write_text(
        json.dumps(rust_context.stats, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    (artifacts_dir / "testcase-context-stats.json").write_text(
        json.dumps(rust_context.stats, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    (artifacts_dir / "testcase-prompt.txt").write_text(prompt + "\n", encoding="utf-8")
    (artifacts_dir / f"testcase-prompt-attempt-{attempt}.txt").write_text(
        prompt + "\n", encoding="utf-8"
    )
    (artifacts_dir / "testcase-injection.json").write_text(
        json.dumps(asdict(injection), indent=2) + "\n", encoding="utf-8"
    )
    llm = llm or OpenAILLM(model=model)
    response = llm.complete(
        "You are a Rust soundness researcher. Return only one ```rust code block.",
        prompt,
    )
    (artifacts_dir / "testcase-response.txt").write_text(
        response + "\n", encoding="utf-8"
    )
    (artifacts_dir / f"testcase-response-attempt-{attempt}.txt").write_text(
        response + "\n", encoding="utf-8"
    )
    blocks = re.findall(r"```(?:rust|rs)\s*\n(.*?)```", response, re.S | re.I)
    if not blocks:
        raise RuntimeError("testcase generator returned no Rust code block")
    code = blocks[0].strip()
    unsafe_scan = re.sub(
        r"#\s*\[\s*unsafe\s*\(\s*no_mangle\s*\)\s*\]",
        "#[no_mangle]",
        code,
    )
    if re.search(r"\bunsafe\b", unsafe_scan):
        raise RuntimeError("generated testcase contains `unsafe`; refusing to inject")
    if re.search(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]", code):
        raise RuntimeError(
            "generated testcase contains `#[cfg(test)]`; POCs must compile under lib build"
        )
    if not re.search(rf"\bfn\s+{re.escape(injection.function)}\s*\(\s*\)", code):
        raise RuntimeError(
            f"generated testcase must define {injection.function}()"
        )
    cfg = rf"#\s*\[\s*cfg\s*\(\s*feature\s*=\s*\"{re.escape(injection.feature)}\"\s*\)\s*\]"
    if not re.search(cfg, code):
        raise RuntimeError(
            f"generated testcase must be gated by feature {injection.feature}"
        )
    _validate_retry_zero_constraints(
        code=code,
        crate_dir=source_crate_dir or crate_dir,
        target=target,
        retry_feedback=retry_feedback,
    )
    _validate_visible_dependency_paths(
        code=code,
        crate_dir=source_crate_dir or crate_dir,
        target=target,
    )
    _validate_pointer_to_pointer_casts(code)
    if "missing_docs" not in code:
        code = re.sub(
            cfg,
            lambda match: "#[allow(missing_docs)]\n" + match.group(0),
            code,
            count=1,
        )
    return code + "\n"


def inject_testcase_at_callsite(
    *, crate_dir: Path, target: dict[str, Any], testcase: str,
    injection: TestcaseInjection,
) -> TestcaseInjection:
    callsite = target.get("callsite")
    if not isinstance(callsite, dict) or not isinstance(callsite.get("path"), str):
        raise RuntimeError("target has no callsite source path")
    source_path = (crate_dir / callsite["path"]).resolve()
    if not source_path.is_file() or crate_dir.resolve() not in source_path.parents:
        raise RuntimeError(f"invalid callsite source path: {source_path}")
    source = source_path.read_text(encoding="utf-8")
    manifest_path = crate_dir / "Cargo.toml"
    if manifest_path.is_file():
        manifest = manifest_path.read_text(encoding="utf-8")
        if re.search(r'^\s*edition\s*=\s*["\']2024["\']', manifest, re.M):
            testcase = re.sub(
                r"#\s*\[\s*no_mangle\s*\]",
                "#[unsafe(no_mangle)]",
                testcase,
            )
    begin = BEGIN_MARKER + injection.key
    end = END_MARKER + injection.key
    generated = f"{begin}\n{testcase.rstrip()}\n{end}"
    pattern = re.compile(
        rf"\n?{re.escape(begin)}.*?{re.escape(end)}\n?", re.S
    )
    if pattern.search(source):
        updated = pattern.sub("\n" + generated + "\n", source)
    else:
        updated = source.rstrip() + "\n\n" + generated + "\n"
    line = updated[: updated.index(begin)].count("\n") + 1
    source_path.write_text(updated, encoding="utf-8")
    logger.info("Injected generated testcase into callsite file: %s", source_path)
    result = TestcaseInjection(
        **{
            **asdict(injection),
            "source_path": str(source_path.relative_to(crate_dir)),
            "line": line,
        }
    )
    update_injection_state(crate_dir, result)
    return result


def update_injection_state(
    crate_dir: Path, injection: TestcaseInjection
) -> Path:
    state_path = crate_dir / INJECTION_STATE_FILE
    if state_path.is_file():
        try:
            state = json.loads(state_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            raise RuntimeError(f"invalid injection state {state_path}: {exc}") from exc
    else:
        state = {"schema_version": 1, "injections": {}}
    if not isinstance(state, dict) or not isinstance(state.get("injections"), dict):
        raise RuntimeError(f"invalid injection state shape: {state_path}")
    state["schema_version"] = 1
    state["injections"][injection.key] = asdict(injection)
    state["injections"] = dict(sorted(state["injections"].items()))
    state_path.write_text(
        json.dumps(state, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    logger.info("Updated testcase injection state: %s", state_path)
    return state_path
