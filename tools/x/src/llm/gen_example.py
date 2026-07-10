import asyncio
from pathlib import Path


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


def read_rust_files_as_context(root: str | Path) -> str:
    root = Path(root).resolve()
    files = get_rust_file_paths(root)

    chunks: list[str] = ["<rust context>"]
    for path in files:
        rel = path.relative_to(root)
        content = path.read_text(encoding="utf-8", errors="replace")
        chunks.append(
            f"\n<file path=\"{rel}\">\n"
            f"```rust\n{content}\n```\n"
            f"</file>"
        )

    chunks.append("</rust context>")
    return "\n".join(chunks)


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
soundness violation found by KLEE.

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

<klee witness and reproduction target>
{klee_witness or "No additional KLEE witness was available."}
</klee witness and reproduction target>

Important:
- The testcase must make the target unsafe callee arguments violate the rule
  above in the same way KLEE did.
- If the witness says a relation such as `index < len` was violated, choose
  concrete inputs that make `index >= len` at the unsafe callsite.
- If the witness says two pointers must be from different allocations, do not
  use `ptr.wrapping_add(...)` from the same allocation; construct the pointer
  from a different safe allocation or safe API state if possible.
- Prefer reproducing the concrete argument relation shown in the KLEE witness
  over writing a merely plausible API call.

{feedback_block}

{rust_context}
""".strip()
