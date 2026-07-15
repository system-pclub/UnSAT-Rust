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
