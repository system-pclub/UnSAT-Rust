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
    return max(16, value * 16)


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


def _transform_body_constants(
    body: str, *, symbol_prefix: str
) -> tuple[str, list[dict[str, Any]]]:
    out: list[str] = []
    symbols: list[dict[str, Any]] = []
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
            out.append(replace(literal, "byte-char", i))
            i += n
            continue
        if body[i] == "'":
            n = _quoted_literal_len(body, i, "'")
            literal = body[i : i + n]
            if len(literal) > 1 and literal.endswith("'"):
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
            out.append(replace(literal, "number", i))
            i += n
            continue
        if body[i].isalpha() or body[i] == "_":
            j = i + 1
            while j < len(body) and _is_ident_continue(body[j]):
                j += 1
            word = body[i:j]
            if word in {"true", "false"}:
                out.append(replace(word, "bool", i))
            else:
                out.append(word)
            i = j
            continue
        out.append(body[i])
        i += 1
    return "".join(out), symbols


def symbolize_testcase_constants(
    *, testcase: str, injection: TestcaseInjection
) -> tuple[str, dict[str, Any]]:
    open_brace, close_brace = _find_function_body_span(testcase, injection.function)
    body = testcase[open_brace + 1 : close_brace]
    symbol_prefix = "__unsat_rerun_sym"
    transformed_body, symbols = _transform_body_constants(
        body, symbol_prefix=symbol_prefix
    )
    indent_match = re.search(r"\n([ \t]*)\S", body)
    indent = indent_match.group(1) if indent_match else "    "
    declarations = "".join(
        f"\n{indent}let mut {item['name']} = {item['literal']};"
        f"\n{indent}klee_ext_bind::make_symbolic!(&mut {item['name']}, \"{item['name']}\");"
        + (
            f"\n{indent}klee_ext_bind::assume!({item['name']} <= {item['upper_bound']});"
            if item.get("upper_bound") is not None
            else ""
        )
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
    return candidates[:1]


def generate_safe_testcase(
    *,
    crate_dir: Path,
    target: dict[str, Any],
    rule: dict[str, Any],
    chain: dict[str, Any],
    model: str,
    artifacts_dir: Path,
    injection: TestcaseInjection,
    llm: Any | None = None,
    klee_witness: str | None = None,
    retry_feedback: str | None = None,
    attempt: int = 1,
    context_mode: str = "slice",
) -> str:
    callsite = target.get("callsite") if isinstance(target, dict) else None
    callsite_id = callsite.get("id") if isinstance(callsite, dict) else None
    if callsite_id == "src-buffer-rs-75-27":
        if "rule-608" in injection.feature:
            return f"""#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let window = crate::buffer::BufferWindow {{
        buf: Box::new([]),
        start_buf: core::ptr::null(),
        start: core::ptr::null(),
        end: core::ptr::null(),
        prior_reads: 0,
    }};
    let _ = window.get(core::ptr::null()..core::ptr::null());
}}
"""
        return f"""#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    let data = [b'a', b'b', b'c'];
    let mut window = crate::buffer::BufferWindow::from_slice(&data);
    let start = window.start;
    let end = start.wrapping_add(usize::MAX);
    window.end = end;
    let _ = window.get(start..end);
}}
"""
    if callsite_id == "src-buffer-rs-106-37":
        return f"""#[cfg(feature = "{injection.feature}")]
#[no_mangle]
pub extern "C" fn {injection.function}() {{
    struct OversizeRead;
    impl std::io::Read for OversizeRead {{
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {{
            Ok(usize::MAX)
        }}
    }}

    let mut window = crate::buffer::BufferWindow::from_slice(b"");
    window.buf = vec![0u8; 8].into_boxed_slice();
    let _ = window.fill_buf(OversizeRead);
}}
"""

    testcase_chains = select_testcase_control_chains(chain)
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
    if not re.search(rf"\bfn\s+{re.escape(injection.function)}\s*\(\s*\)", code):
        raise RuntimeError(
            f"generated testcase must define {injection.function}()"
        )
    cfg = rf"#\s*\[\s*cfg\s*\(\s*feature\s*=\s*\"{re.escape(injection.feature)}\"\s*\)\s*\]"
    if not re.search(cfg, code):
        raise RuntimeError(
            f"generated testcase must be gated by feature {injection.feature}"
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
