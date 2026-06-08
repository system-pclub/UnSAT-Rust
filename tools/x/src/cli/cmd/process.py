import argparse
import json
from pathlib import Path
from typing import Any


def _extract_text_from_response_body(body: dict[str, Any]) -> str:
	output = body.get("output", [])
	for item in output:
		if item.get("type") != "message":
			continue
		for content in item.get("content", []):
			if content.get("type") == "output_text" and isinstance(content.get("text"), str):
				return content["text"].strip()
	return ""


def _parse_rule_json(text: str) -> dict[str, Any]:
	stripped = text.strip()
	if stripped.startswith("```"):
		lines = stripped.splitlines()
		if len(lines) >= 3 and lines[-1].strip() == "```":
			stripped = "\n".join(lines[1:-1]).strip()
	return json.loads(stripped)


def run(args: argparse.Namespace) -> int:
	input_jsonl = Path(args.input_jsonl)
	output_dir = Path(args.output_dir)
	output_dir.mkdir(parents=True, exist_ok=True)

	written = 0
	skipped = 0

	with input_jsonl.open("r", encoding="utf-8") as f:
		for line_no, line in enumerate(f, start=1):
			if not line.strip():
				continue

			row = json.loads(line)
			custom_id = row.get("custom_id", f"line-{line_no}")
			response = row.get("response")
			if not isinstance(response, dict):
				skipped += 1
				continue

			body = response.get("body")
			if not isinstance(body, dict):
				skipped += 1
				continue

			text = _extract_text_from_response_body(body)
			if not text:
				skipped += 1
				continue

			try:
				rule_json = _parse_rule_json(text)
			except json.JSONDecodeError:
				skipped += 1
				continue

			out_file = output_dir / f"{custom_id}.json"
			out_file.write_text(json.dumps(rule_json, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
			written += 1

	print(f"processed={written} skipped={skipped} output_dir={output_dir}")
	return 0
