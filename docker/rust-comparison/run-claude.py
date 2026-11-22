#!/usr/bin/env python3
"""Host Docker wrapper and container entrypoint for Claude source judgments."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import uuid
import hashlib
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parent
if ROOT.name == "rust-comparison":
    ROOT = ROOT.parent.parent
IMAGE = "uspec-rust-comparison-claude:local"
SCHEMA = {"type": "object", "properties": {"violation": {"type": "boolean"}},
          "required": ["violation"], "additionalProperties": False}
CRATE_SCHEMA = {
    "type": "object", "properties": {"violations": {"type": "array", "items": {
        "type": "object", "properties": {key: {"type": "string"} for key in
            ("file", "function", "unsafe_api", "requirement", "explanation")},
        "required": ["file", "function", "unsafe_api", "requirement", "explanation"],
        "additionalProperties": False}}},
    "required": ["violations"], "additionalProperties": False}


def crate_prompt():
    return ("Read the Rust crate at /work and the Rust libraries core, std, and alloc "
            "at /libraries. List the violations of unsafe fn requirements in this crate.")


def api_key(explicit=None):
    if explicit:
        return explicit
    if os.environ.get("CLAUDE_API_KEY"):
        return os.environ["CLAUDE_API_KEY"]
    dotenv = ROOT / ".env"
    for line in dotenv.read_text().splitlines() if dotenv.exists() else []:
        line = line.strip()
        if line.startswith("export "):
            line = line[7:].strip()
        key, sep, value = line.partition("=")
        if sep and key.strip() == "CLAUDE_API_KEY":
            value = value.strip()
            if value[:1] in ("'", '"'):
                value = value[1:value.index(value[0], 1)]
            else:
                value = value.split(" #", 1)[0].strip()
            if value:
                return value
    raise ValueError("Set CLAUDE_API_KEY in the environment or repository .env")


def build_image(libraries=None):
    import tempfile
    import shutil
    libraries = Path(libraries or ROOT / "tools/rust/library").resolve()
    for name in ("core", "std", "alloc"):
        if not (libraries / name / "Cargo.toml").is_file():
            raise ValueError(f"Missing Rust library: {libraries / name}")
    with tempfile.TemporaryDirectory(prefix="claude-build-") as tmp:
        context = Path(tmp)
        for name in ("Dockerfile.claude", "run-claude.py"):
            shutil.copy2(ROOT / "docker/rust-comparison" / name, context / name)
        for name in ("core", "std", "alloc"):
            shutil.copytree(libraries / name, context / "libraries" / name)
        subprocess.run(["docker", "build", *platform_args(), "-f",
                        str(context / "Dockerfile.claude"), "-t", IMAGE, str(context)],
                       check=True, stdout=sys.stderr)


def platform_args():
    return ["--platform", "linux/amd64"] if sys.platform == "darwin" else []


def judge(crate, rule, target_function, unsafe_api, token=None, model="sonnet", timeout=600):
    crate = Path(crate).resolve()
    source_file = crate.is_file()
    if not source_file and not (crate / "Cargo.toml").is_file():
        raise ValueError(f"Not a Rust source file or crate directory: {crate}")
    mount = f"{crate}:/work/input.rs:ro" if source_file else f"{crate}:/work:ro"
    env = dict(os.environ, ANTHROPIC_API_KEY=api_key(token))
    name = "rustsec-claude-" + uuid.uuid4().hex
    command = ["docker", "run", "--rm", "--name", name, *platform_args(), "-i",
               "--env", "ANTHROPIC_API_KEY", "--volume", mount, IMAGE]
    request = dict(rule=rule, target_function=target_function, unsafe_api=unsafe_api, model=model, source_file=source_file)
    try:
        result = subprocess.run(command, input=json.dumps(request), text=True,
                                capture_output=True, env=env, timeout=timeout)
    except subprocess.TimeoutExpired:
        subprocess.run(["docker", "rm", "-f", name], capture_output=True)
        raise
    if result.returncode:
        raise RuntimeError(f"Claude container failed ({result.returncode}): {result.stderr}\n{result.stdout}")
    value = json.loads(result.stdout)
    if type(value.get("violation")) is not bool:
        raise ValueError("Claude did not return a boolean violation")
    return value


def inside():
    request = json.load(sys.stdin)
    source_instruction = ("Read the Rust source file at /work/input.rs. "
                          if request.get("source_file") else "Read the Rust crate at /work. ")
    whole_crate = request.get("mode") == "crate"
    prompt = crate_prompt() if whole_crate else (
        source_instruction + "Determine whether the supplied rule is "
        "violated at the specified unsafe API call in the target function. "
        "Return {\"violation\": true} if the rule is violated, "
        "or {\"violation\": false} if it is satisfied.\n" +
        json.dumps({key: request[key] for key in ("rule", "target_function", "unsafe_api")})
    )
    command = [
        "claude", "--bare", "-p", "--model", request["model"],
        "--tools", "Read,Glob,Grep", "--allowedTools", "Read,Glob,Grep",
        "--permission-mode", "dontAsk", "--output-format", "json",
        "--json-schema", json.dumps(CRATE_SCHEMA if whole_crate else SCHEMA), "--no-session-persistence",
    ]
    if whole_crate:
        command.extend(["--add-dir", "/libraries"])
    result = subprocess.run(command, input=prompt, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(result.stderr + "\n" + result.stdout)
    response = json.loads(result.stdout)
    if response.get("is_error") or response.get("subtype") != "success":
        raise ValueError(f"Claude failed: {result.stdout}")
    value = response.get("structured_output")
    if whole_crate:
        validate_crate_result(value)
        print(json.dumps({**value, "execution": {key: response.get(key) for key in
            ("modelUsage", "usage", "total_cost_usd", "duration_ms", "num_turns")}}))
        return
    if not isinstance(value, dict) or type(value.get("violation")) is not bool:
        raise ValueError(f"Missing structured boolean: {result.stdout}")
    print(json.dumps({"violation": value["violation"]}))


def validate_crate_result(value):
    if not isinstance(value, dict) or not isinstance(value.get("violations"), list):
        raise ValueError("Missing structured violations list")
    for row in value["violations"]:
        if not isinstance(row, dict) or any(not isinstance(row.get(k), str)
                for k in ("file", "function", "unsafe_api", "requirement", "explanation")):
            raise ValueError("Malformed violation record")


def judge_crate(crate, output, token=None, model="sonnet", timeout=1800):
    crate = Path(crate).resolve()
    if not (crate / "Cargo.toml").is_file():
        raise ValueError(f"Not a crate directory: {crate}")
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    request = {"mode": "crate", "model": model}
    hashes = {str(p.relative_to(crate)): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in sorted(crate.rglob("*")) if p.is_file()
              and not {"target", ".git"}.intersection(p.relative_to(crate).parts)}
    metadata = dict(crate=str(crate), request=request, prompt=crate_prompt(), source_hashes=hashes,
                    started_at=datetime.now(timezone.utc).isoformat())
    metadata["image_id"] = subprocess.check_output(
        ["docker", "image", "inspect", "--format", "{{.Id}}", IMAGE], text=True).strip()
    (output / "request.json").write_text(json.dumps(metadata, indent=2) + "\n")
    name = "claude-crate-" + uuid.uuid4().hex
    command = ["docker", "run", "--rm", "--name", name, *platform_args(), "-i",
               "--env", "ANTHROPIC_API_KEY", "--volume", f"{crate}:/work:ro", IMAGE]
    try:
        result = subprocess.run(command, input=json.dumps(request), text=True, capture_output=True,
                                env=dict(os.environ, ANTHROPIC_API_KEY=api_key(token)), timeout=timeout)
        (output / "stdout.json").write_text(result.stdout)
        (output / "stderr.log").write_text(result.stderr)
        if result.returncode:
            raise RuntimeError(f"Claude container exited {result.returncode}; see {output}")
        value = json.loads(result.stdout)
        validate_crate_result(value)
        (output / "result.json").write_text(json.dumps(value, indent=2) + "\n")
        return value
    except subprocess.TimeoutExpired as exc:
        subprocess.run(["docker", "rm", "-f", name], capture_output=True)
        (output / "error.json").write_text(json.dumps({"status": "timeout", "timeout": timeout}))
        for filename, data in (("stdout.json", exc.stdout), ("stderr.log", exc.stderr)):
            (output / filename).write_text(data.decode() if isinstance(data, bytes) else data or "")
        raise
    except (ValueError, RuntimeError) as exc:
        (output / "error.json").write_text(json.dumps({"status": "error", "message": str(exc)}))
        raise


def main():
    if sys.argv[1:] == ["--inside"]:
        inside()
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=Path)
    parser.add_argument("--mode", choices=("callsite", "crate"), default="callsite")
    parser.add_argument("--libraries", type=Path, default=ROOT / "tools/rust/library")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--rule")
    parser.add_argument("--target-function")
    parser.add_argument("--unsafe-api")
    parser.add_argument("--api-token", help="Defaults to CLAUDE_API_KEY in environment or .env")
    parser.add_argument("--model", default="sonnet")
    parser.add_argument("--timeout", type=int, default=600)
    args = parser.parse_args()
    if args.mode == "callsite" and not all((args.rule, args.target_function, args.unsafe_api)):
        parser.error("callsite mode requires --rule, --target-function and --unsafe-api")
    api_key(args.api_token)
    build_image(args.libraries)
    if args.mode == "crate":
        output = args.output or ROOT / "eval/claude-crate" / args.crate.name / datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
        print(json.dumps(judge_crate(args.crate, output, args.api_token, args.model, args.timeout)))
        return
    print(json.dumps(judge(args.crate, args.rule, args.target_function, args.unsafe_api,
                           args.api_token, args.model, args.timeout)))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, RuntimeError, subprocess.SubprocessError) as exc:
        print(str(exc), file=sys.stderr)
        sys.exit(1)
