"""Translate unresolved documentation rules, retaining reviewed correct DSL verbatim.

Requests, raw responses, source bindings, repair attempts and provenance are saved.
Review labels select work but are never treated as proof of a new translation.
"""

from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from dataclasses import fields, is_dataclass
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import sys
import time

from openai import OpenAI

from dsl.parser import _GRAMMAR, parse_dsl
from dsl.ast import Identifier, QuantifiedExpression, Literal, CollectionExpression, CallExpression
from llm.openai_llm import _load_openai_api_key_from_dotenv

ROOT = next(p for p in Path(__file__).resolve().parents if (p / 'rules.csv').is_file() and (p / 'tools/x').is_dir())
SYSTEM = """You are a Rust code analysis assistant specialising in formal safety rules.
Translate the supplied atomic natural-language rule into the supplied project DSL.
Return exactly one DSL expression as plain text, without Markdown or explanation.
Translate every supplied rule by composing the supplied operators and grammar.
Never substitute an opaque Boolean field, guessed operator, or unrelated condition.
Preserve the complete atomic requirement; do not import adjacent obligations.
Translate undefined-behavior triggers into the positive condition that avoids that
trigger. Preserve explicit panic/return effects as effects, not prohibitions on safe
panics. Keep sufficient safety guarantees conditional; do not turn them into exclusive
preconditions. Permissions and descriptive observations are not extra caller duties.
Use the exact modeled source fields and metadata supplied here; check representation,
snapshot time, causal/thread scope, interval endpoints and every exception before replying.
"""

BINDING = """Documentation-stage source-level binding convention:
- get_arg(i) indexes source arguments in declaration order; a receiver self is index 0.
- Compound source values remain structured: use get_field(layout, "size") etc.
- Fat pointers remain source pointer values; LLVM metadata/sret operands are not arguments here.
- get_type_at_func(i) indexes enclosing impl type parameters, then method type parameters.
  Type indexes exclude lifetimes and const parameters. For unsafe traits, Self is type 0, followed by trait type parameters.
  Do not request a type parameter absent from the source; use known concrete sizes/alignment for u8, bool, etc.
- Const generics are available by source name through get_field(get_field(get_field(get_result(), "call"), "consts"), "N").
  Arrays, slices and fat pointers expose length in elements and element_type; use those instead of guessing N or dropping lengths.
- References and pointer wrappers expose the logical referenced source object (including real source fields)
  and pointer address/provenance, not an extra LLVM reference indirection.
- get_result() is the source API result. Actual backend ABI lowering is a separate integration concern.
- Use only declared properties and real fields documented by the source. Do not invent summary flags.
- Conditions are positive: true when the atomic safety requirement holds. Preserve implications,
  exceptions, quantification, units, and temporal scope. Do not silently add other atomic rules.
- Use the supplied specification semantics. Integration limits are not expressibility gates.
- get_result().call is the current source invocation context (also for unit-returning functions).
  Its events and states are modeled observations, not per-rule truth flags. At a trait contract,
  quantify over call.invocations and bind each member's arguments/result from that invocation;
  get_result() by itself is not the result of an unnamed trait method.
- When restricting events caused by an invocation, include its nested helpers using the
  event's directly owning invocation ID OR membership in causal_ancestors. Thread/time
  coincidence alone is not a causal relation. Check intervals only for the duration stated.
- Use current call.time/state or a value's explicitly time-indexed states for temporal
  conditions. A historical return-value snapshot is not proof of present validity.
- OS resource openness/ownership comes from the defined resource state, not a list of
  observed creator calls or the existence of an OwnedFd/OwnedHandle source object.
  Require exclusive ownership only when the atomic clause actually requires uniqueness.
- Region membership is MemoryRange.region, independent of single-allocation containment.
  Compare fully qualified type item_path when nominal identity matters; item_name alone
  can also name an unrelated user-defined type. Preserve zero-size exceptions from source.
- Source fields and the explicitly defined metadata fields may be projected using get_field.
  Do not invent fields such as is_safe, contract_holds or satisfies_rule.
- Expressions at this documentation stage use mathematical integer arithmetic. Backend
  machine-width lowering is a separate concern; an explicit integer-fit clause still requires mul_fits.
- Compose ordinary arithmetic and field access rather than requiring a dedicated operator
  for each property. For example, with pointer p at arg0 and pointee T at type0,
  proper alignment is expressible as
  get_field(get_arg(0), "address") % get_field(get_type_at_func(0), "align") == 0.
  A structured Layout argument's size is get_field(get_arg(i), "size").
"""


def digest(data: bytes):
    return hashlib.sha256(data).hexdigest()


def write_json(path: Path, value):
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
    temporary.replace(path)


def context_for(row, units):
    filename, _, line = row["path"].rpartition(":")
    source = ROOT / "tools" / filename
    match = re.search(r"\b(?:fn|trait)\s+(\w+)", row["name"])
    if not match:
        match = re.search(r"^\s*(\w+)\s*\(", row["name"])
    name = match.group(1) if match else None
    candidates = [u for u in units.get("tools/" + filename, []) if u["name"] == name]
    if candidates:
        unit = min(candidates, key=lambda u: abs(u["line_number_start"] - int(line)))
        return {"source_file": str(source.relative_to(ROOT)), "catalog_line": int(line),
                "source_line": unit["line_number_start"], "parent": unit["parent_struct_or_trait"],
                "declaration": unit["declaration_text"], "documentation": unit["comment"],
                **{k: unit.get(k) for k in ("enclosing_declaration", "enclosing_generics", "item_generics", "members", "source_body")}}
    if source.is_file():
        lines = source.read_text().splitlines()
        pattern = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?unsafe\s+(?:extern\s+\"[^\"]+\"\s+)?fn\s+" + re.escape(name or "__missing__") + r"\b")
        indices = [i for i, text in enumerate(lines) if pattern.search(text)]
        index = min(indices, key=lambda i: abs(i + 1 - int(line))) if indices else max(0, int(line) - 1)
        return {"source_file": str(source.relative_to(ROOT)), "catalog_line": int(line),
                "source_line": index + 1, "declaration": row["name"],
                "source_excerpt": "\n".join(f"{i+1}: {lines[i]}" for i in range(max(0, index-90), min(len(lines), index+35)))}
    return {"source_file": filename, "declaration": row["name"], "source_missing": True}


def prompt_for(row, context, operators):
    return (f"Unsafe API / source item: {row['name']}\nSource location: {row['path']}\n"
            f"Atomic requirement:\n{row['rule']}\n\n"
            "Explicit source argument index table:\n" + json.dumps(source_arguments(context.get("declaration", row["name"]))) + "\n"
            "Source declaration and documentation:\n" + json.dumps(context, indent=2) +
            "\nWrite the complete positive expression for this atomic requirement.")


def source_arguments(declaration):
    match = re.search(r"\bfn\s+\w+", declaration)
    if not match:
        return []
    angle = 0
    start = None
    for i in range(match.end(), len(declaration)):
        char = declaration[i]
        if char == "<": angle += 1
        elif char == ">": angle = max(0, angle - 1)
        elif char == "(" and angle == 0:
            start = i + 1
            break
    if start is None:
        return []
    nesting = 0
    result, current = [], []
    for char in declaration[start:]:
        if char == ")" and nesting == 0:
            if "".join(current).strip(): result.append("".join(current).strip())
            break
        if char in "([<{": nesting += 1
        elif char in ")]>}": nesting = max(0, nesting - 1)
        if char == "," and nesting == 0:
            result.append("".join(current).strip()); current = []
        else: current.append(char)
    return [{"index": i, "source_argument": text} for i, text in enumerate(result)]


def validate_closed_expression(text, operators_path):
    ast = parse_dsl(text, operators_path)
    definitions = json.loads(operators_path.read_text())
    if isinstance(ast, Literal) and not isinstance(ast.value, bool):
        raise ValueError('The rule must be Boolean, not a literal value or diagnostic message')
    if isinstance(ast, CollectionExpression):
        raise ValueError('The rule must be Boolean, not a collection')
    signatures = {o['name']: o for o in definitions['operators']}
    constants = {item["name"] for item in definitions.get("constants", [])}
    unknown = set()
    def visit(node, bound):
        if isinstance(node, CallExpression):
            inputs = signatures[node.name]['input']
            assert sum(not p.get('optional', False) for p in inputs) <= len(node.args) <= len(inputs), f'Wrong argument count for {node.name}'
            if node.name == 'is_valid_for' and isinstance(node.args[1], Literal):
                allowed = signatures[node.name].get('access_modes')
                if allowed and node.args[1].value not in allowed:
                    raise ValueError(f'Undefined access mode {node.args[1].value!r}; choose a defined property and compose the remaining constraints')
        if isinstance(node, Identifier):
            if node.name not in bound: unknown.add(node.name)
        elif isinstance(node, QuantifiedExpression):
            visit(node.domain, bound)
            visit(node.predicate, bound | {node.variable})
        elif is_dataclass(node):
            for field in fields(node):
                value = getattr(node, field.name)
                if isinstance(value, tuple):
                    for item in value:
                        if is_dataclass(item): visit(item, bound)
                elif is_dataclass(value): visit(value, bound)
    visit(ast, constants)
    if unknown:
        raise ValueError("Undeclared free identifiers: " + ", ".join(sorted(unknown)))


def run_one(row, context, operators, args, output):
    directory = output / "requests" / row["id"]
    directory.mkdir(parents=True, exist_ok=True)
    user = prompt_for(row, context, operators)
    if args.draft_run:
        draft_path = args.draft_run / "requests" / row["id"] / "result.json"
        draft = (json.loads(draft_path.read_text()) if draft_path.exists() else
                 next(r for r in json.loads((args.draft_run / 'judgments.json').read_text()) if r['id'] == row['id']))
        user += ("\n\nExplicit source argument index table (authoritative):\n" +
                 json.dumps(source_arguments(context.get("declaration", row["name"])), indent=2) +
                 "\nAn earlier model produced this untrusted draft:\n" + (draft.get("dsl") or "No valid draft was produced.") +
                 "\nGenerate a faithful final translation from the requirement and definitions above. "
                 "Check every source argument index, each guard and exception actually present in the atomic rule, "
                 "units, allocation identity versus mere address bounds, and documented operator limits. "
                 "Do not import unrelated requirements from the surrounding documentation. "
                 "Use only declared constants, literals and bound quantified variables; no undeclared identifiers. "
                 "Return one corrected DSL expression.")
    if args.review_feedback:
        feedback = next((r for r in json.loads(args.review_feedback.read_text()) if r['id'] == row['id']), None)
        if feedback:
            user += ("\n\nPrior review identified a possible issue; verify it against the source and definitions, then correct the translation:\n"
                     + json.dumps({k: feedback[k] for k in ('reason', 'evidence')}, ensure_ascii=False))
    system = SYSTEM + "\n" + BINDING + "\nDSL grammar:\n" + _GRAMMAR + "\nOperator definitions and metadata:\n" + json.dumps(operators, indent=2)
    request = {"model": args.model, "messages": [{"role": "system", "content": system},
                {"role": "user", "content": user}], "max_completion_tokens": args.max_output_tokens,
                "reasoning_effort": args.reasoning_effort}
    write_json(directory / "request.json", request)
    write_json(directory / "context.json", context)
    key = os.environ.get("OPENAI_API_KEY") or _load_openai_api_key_from_dotenv()
    client = OpenAI(api_key=key, timeout=180, max_retries=1)
    errors = []
    for attempt in range(1, args.max_attempts + 1):
        started = time.monotonic()
        try:
            response = client.chat.completions.create(**request)
            raw = response.model_dump(mode="json")
            write_json(directory / f"response-{attempt}.json", raw)
            text = response.choices[0].message.content or ""
            (directory / f"response-{attempt}.txt").write_text(text)
            status, problem = "translated", None
            try:
                validate_closed_expression(text.strip(), output / "inputs/operators.json")
            except Exception as error:
                status, problem = "syntax_error", str(error)
            result = {**row, "status": status, "dsl": text.strip() if status == "translated" else None,
                      "attempt": attempt, "latency_seconds": round(time.monotonic()-started, 3),
                      "model": raw.get("model"), "usage": raw.get("usage"), "syntax_error": problem,
                      "response_id": raw.get("id"), "syntax_repairs": errors}
            write_json(directory / f"attempt-result-{attempt}.json", result)
            if status != "syntax_error" or attempt == args.max_attempts:
                write_json(directory / "result.json", result)
                return result
            errors.append(problem)
            request["messages"] += [{"role": "assistant", "content": text},
                {"role": "user", "content": "The DSL parser rejected the expression:\n" + problem +
                 "\nRepair using the supplied grammar. Translate the entire rule; return one expression."}]
            write_json(directory / f"repair-request-{attempt+1}.json", request)
        except Exception as error:
            result = {**row, "status": "request_error", "dsl": None, "error": type(error).__name__ + ": " + str(error)}
            write_json(directory / "result.json", result)
            return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--source-units", type=Path, required=True)
    parser.add_argument("--model", default="gpt-6.1-sol")
    parser.add_argument("--workers", type=int, default=8)
    parser.add_argument("--reasoning-effort", default="medium")
    parser.add_argument("--max-attempts", type=int, default=2)
    parser.add_argument("--max-output-tokens", type=int, default=6000)
    parser.add_argument("--rule-ids", default="")
    parser.add_argument("--draft-run", type=Path)
    parser.add_argument("--retain-correct-from", type=Path, action="append", default=[])
    parser.add_argument("--review-feedback", type=Path)
    parser.add_argument("--exclude-retained-ids", default="", help="Only specific prior labels overturned by a recorded semantic counterexample")
    parser.add_argument("--limit", type=int)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    (output / "inputs").mkdir(exist_ok=True)
    for filename in ("rules.csv", "operators.json"):
        destination = output / "inputs" / filename
        if not destination.exists(): shutil.copy2(ROOT / filename, destination)
    rows = [r for r in csv.DictReader((output / "inputs/rules.csv").open(newline=""))
            if "/library/core/" in r["path"] or "/library/std/" in r["path"]]
    retained = {}
    for run in args.retain_correct_from:
        for item in json.loads((run / "judgments.json").read_text()):
            if item["label"] == "correct":
                retained[item["id"]] = {**item, "retained_from": str(run.resolve()),
                    "retained_judgments_sha256": digest((run / "judgments.json").read_bytes())}
    for rule_id in filter(None, args.exclude_retained_ids.split(',')):
        assert rule_id in retained, f'No previously accepted translation to adjudicate: {rule_id}'
        retained.pop(rule_id)
    catalog = {r["id"]: r for r in rows}
    for i, item in retained.items():
        assert all(item[k] == catalog[i][k] for k in ("path", "name", "rule"))
        validate_closed_expression(item["dsl"], output / "inputs/operators.json")
    write_json(output / "retained-translations.json", list(retained.values()))
    rows = [r for r in rows if r["id"] not in retained]
    ids = set(args.rule_ids.split(",")) if args.rule_ids else None
    if ids: rows = [r for r in rows if r["id"] in ids]
    if args.limit: rows = rows[:args.limit]
    source_units = json.loads(args.source_units.read_text())
    units = source_units["files"]
    operators = json.loads((output / "inputs/operators.json").read_text())
    settings = {"model": args.model, "reasoning_effort": args.reasoning_effort,
                "max_attempts": args.max_attempts, "max_output_tokens": args.max_output_tokens,
                "system_prompt_sha256": digest(SYSTEM.encode()), "binding_sha256": digest(BINDING.encode()),
                "grammar_sha256": digest(_GRAMMAR.encode()), "source_units_sha256": digest(args.source_units.read_bytes()),
                "generator_sha256": digest(Path(__file__).read_bytes()),
                "draft_run": str(args.draft_run.resolve()) if args.draft_run else None,
                "retained_sha256": digest((output / "retained-translations.json").read_bytes()),
                "review_feedback_sha256": digest(args.review_feedback.read_bytes()) if args.review_feedback else None,
                "excluded_prior_labels": sorted(filter(None, args.exclude_retained_ids.split(','))),
                "inputs": {p.name: digest(p.read_bytes()) for p in (output / "inputs").iterdir()}}
    manifest_file = output / "manifest.json"
    if manifest_file.exists(): assert json.loads(manifest_file.read_text()) == settings, "Run inputs/settings changed; use a new output directory"
    else: write_json(manifest_file, settings)
    (output / "system-prompt.txt").write_text(SYSTEM)
    (output / "binding-convention.txt").write_text(BINDING)
    pending = [r for r in rows if not (output / "requests" / r["id"] / "result.json").exists()]
    print(f"cohort={len(rows)} pending={len(pending)} model={args.model}", flush=True)
    failures = 0
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = {}
        for r in pending:
            context = context_for(r, units)
            context["source_declarations"] = source_units.get("source_declarations", {}).get(context["source_file"], [])
            futures[pool.submit(run_one, r, context, operators, args, output)] = r["id"]
        for done, future in enumerate(as_completed(futures), 1):
            result = future.result()
            if result["status"] == "request_error": failures += 1
            print(f"{done}/{len(pending)} {result['id']} {result['status']}", flush=True)
            if failures >= 3 and done <= 8:
                for f in futures: f.cancel()
                raise SystemExit("Repeated provider errors; stopping this run. Inspect preserved result files.")
    results = [json.loads((output / "requests" / r["id"] / "result.json").read_text()) for r in rows]
    write_json(output / "generation-results.json", results)
    counts = {status: sum(r["status"] == status for r in results) for status in {r["status"] for r in results}}
    print(json.dumps(counts), flush=True)


if __name__ == "__main__":
    main()
