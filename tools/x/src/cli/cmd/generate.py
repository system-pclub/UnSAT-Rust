
import argparse
import json
from pathlib import Path
import subprocess
from typing import Any

from verirule.schema import RULE_SCHEMA
from verirule.utils import load_json


def run(args: argparse.Namespace) -> int:
    crate_path = Path(args.crate_dir)
    rustscan_path = Path(args.rustscan)
    output_path = Path(args.output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    units_file = output_path / "units.json"
    jsonl_file = output_path / "prompts.jsonl"
    run_rustscan(crate_path, rustscan_path, units_file)
    units = read_units(units_file)
    prompts = []
    for unit in units:
        prompt = build_rule_schema_prompt(crate_path, unit)
        if prompt:
            prompts.append(prompt)

    batch_requests = prompts_to_openai_batch_requests(prompts[:10])
    prompts_to_jsonl(batch_requests, jsonl_file)
    return 0


    
    
def run_rustscan(crate_path: Path, rustscan_path: Path, output_file: Path):
    subprocess.run([rustscan_path, str(crate_path), str(output_file)], check=True)
 

class Unit:
    def __init__(self, type: str, name: str, relative_file_path: str, body_range: dict, children: list['Unit']):
        self.type = type
        self.name = name
        self.relative_file_path = relative_file_path
        self.body_range = body_range
        self.children = children

    @staticmethod
    def from_dict(d: dict) -> 'Unit':
        return Unit(
            type=d['type'],
            name=d['name'],
            relative_file_path=d['relative_file_path'],
            body_range=d['body_range'],
            children=[Unit.from_dict(c) for c in d.get('children', [])],
        )


def read_units(units_file: Path) -> list[Unit]:
    return [Unit.from_dict(d) for d in load_json(units_file)]


INSTRUCTIONS = f"""
## Role
You are a Rust code analysis assistant that generates formal rules based on Rust source code. The rules should capture the semantics of the code in a structured format defined by the provided JSON Schema. 

## Task
Your goal is to produce accurate and complete rules that reflect the behavior of the Rust code excerpts.

## Output Format
Your output must be a JSON object that conforms to the provided JSON Schema.
Schema: ```
{json.dumps(RULE_SCHEMA, indent=2)}
```
"""
def build_rule_schema_prompt(crate_path: Path, unit: Unit) -> str:
    source_file = crate_path / unit.relative_file_path
    source_lines = source_file.read_text(encoding="utf-8").splitlines()

    start = int(unit.body_range["start"])
    end = int(unit.body_range["end"])
    lines = end - start
    if lines >= 100:
        print(f"Warning: skipping unit with body range > 100 lines: {unit.name} ({start}-{end})")
        return None 
    if lines <= 3:
        print(f"Warning: skipping unit with body range <= 3 lines: {unit.name} ({start}-{end})")
        return None
    snippet = "\n".join(source_lines[start - 1 : end])
    
    return snippet


def prompts_to_openai_batch_requests(
    prompts: list[str], model: str = "gpt-5.4-mini"
) -> list[dict[str, Any]]:
    requests: list[dict[str, Any]] = []
    for index, prompt in enumerate(prompts):
        requests.append(
            {
                "custom_id": f"req-{index}",
                "method": "POST",
                "url": "/v1/responses",
                "body": {
                    "model": model,
                    "instructions": INSTRUCTIONS,
                    "input": prompt,
                },
            }
        )
    return requests


def prompts_to_jsonl(requests: list[dict[str, Any]], output_jsonl_file: Path) -> None:
    output_jsonl_file.parent.mkdir(parents=True, exist_ok=True)
    with output_jsonl_file.open("w", encoding="utf-8") as f:
        for request in requests:
            f.write(json.dumps(request, ensure_ascii=False) + "\n")


def generate_with_openai(client: Any, prompt: str, model: str = "gpt-5.4-mini") -> str:
    response = client.responses.create(model=model, input=prompt)
    if hasattr(response, "output_text"):
        return response.output_text
    return str(response)