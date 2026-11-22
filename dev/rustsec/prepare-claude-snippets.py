#!/usr/bin/env python3
"""Extract source-exact functions/structs for the 38 Claude advisory inputs.

Run in the repository Docker image with tree-sitter 0.25.2 and
 tree-sitter-rust 0.24.0 (the experiment venv is under .local/).
The output is source excerpts, not standalone buildable Rust crates.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path

from tree_sitter import Language, Parser
import tree_sitter_rust

ROOT = Path(__file__).resolve().parents[2]
PARSER = Parser(Language(tree_sitter_rust.language()))
# These defects span the named public function and its unsafe helper/later drop.
EXTRA_FUNCTIONS = {
    "RUSTSEC-2023-0080": ["transpose"],
    "RUSTSEC-2023-0016": ["partial_sort", "make_heap", "sort_heap"],
    "RUSTSEC-2019-0012": ["grow", "spilled", "triple_mut", "deallocate"],
    "RUSTSEC-2023-0086": ["read"],
}


def walk(node):
    yield node
    for child in node.named_children:
        yield from walk(child)


def name(node, data):
    field = node.child_by_field_name("name")
    return data[field.start_byte:field.end_byte].decode() if field else ""


def outer_function(node):
    """Retain the enclosing function when the unsafe call is in a local Drop impl."""
    result = node
    parent = node.parent
    while parent:
        if parent.type == "function_item":
            result = parent
        parent = parent.parent
    return result


def render(node, data):
    code = data[node.start_byte:node.end_byte].decode()
    parent = node.parent
    if parent and parent.type == "declaration_list" and parent.parent.type in {"impl_item", "trait_item"}:
        impl = parent.parent
        header = data[impl.start_byte:parent.start_byte].decode().rstrip()
        associated = ""
        if impl.type == "trait_item":
            associated = "\n".join(data[n.start_byte:n.end_byte].decode()
                                     for n in parent.named_children if n.type == "associated_type")
        code = header + " {\n" + (associated + "\n" if associated else "") + code + "\n}"
    return code


def extract(case):
    crate = ROOT / case["directory"]
    cache = {}

    def parse(path):
        if path not in cache:
            data = path.read_bytes()
            tree = PARSER.parse(data)
            cache[path] = (data, tree)
        return cache[path]

    path = crate / case["callsite"]["path"]
    data, tree = parse(path)
    line = case["callsite"]["line"] - 1
    matches = [n for n in walk(tree.root_node)
               if n.type == "function_item" and n.start_point.row <= line <= n.end_point.row]
    if not matches:
        raise ValueError(f"No containing function for {case['advisory']}")
    node = outer_function(min(matches, key=lambda n: n.end_byte - n.start_byte))
    selected = {(path, node.start_byte): node}
    for extra in EXTRA_FUNCTIONS.get(case["advisory"], []):
        candidates = [n for n in walk(tree.root_node)
                      if n.type == "function_item" and name(n, data) == extra]
        # Trait forwarding methods named partial_sort are not the free entrypoint.
        if extra == "partial_sort":
            candidates = [n for n in candidates if n.parent.type == "source_file"]
        if not candidates:
            raise ValueError(f"Missing related function {extra} for {case['advisory']}")
        for n in candidates:
            n = outer_function(n)
            selected[path, n.start_byte] = n

    needed = {"ArrayVec"} if case["advisory"] == "RUSTSEC-2020-0042" else set()
    for (path, _), node in selected.items():
        data, _ = parse(path)
        for n in walk(node):
            if n.type == "type_identifier":
                needed.add(data[n.start_byte:n.end_byte].decode())
        parent = node.parent
        if parent.type == "declaration_list" and parent.parent.type == "impl_item":
            receiver = parent.parent.child_by_field_name("type")
            if receiver:
                needed.update(data[n.start_byte:n.end_byte].decode()
                              for n in walk(receiver) if n.type == "type_identifier")

    # Include the original declarations for referenced in-crate structs, including their field types.
    # Avoid pulling implementations, tests, imports or every transitive field type.
    declarations = {}
    searched = set()
    sources = [source for source in sorted(crate.rglob("*.rs"))
               if not any(part in {"target", ".git", "tests", "benches", "examples"}
                          for part in source.relative_to(crate).parts)]
    while needed - searched:
        pending = needed - searched
        searched.update(pending)
        for source in sources:
            if not any(("struct " + candidate).encode() in source.read_bytes()
                       for candidate in pending):
                continue
            source_data, source_tree = parse(source)
            for n in walk(source_tree.root_node):
                if n.type == "struct_item" and name(n, source_data) in needed:
                    if any(parent.type == "function_item" for parent in ancestors(n)):
                        continue
                    key = name(n, source_data)
                    if key not in declarations or source == path:
                        declarations[key] = (source, n)
                        needed.update(source_data[child.start_byte:child.end_byte].decode()
                                      for child in walk(n) if child.type == "type_identifier")

    items = list(declarations.values()) + [(p, n) for (p, _), n in selected.items()]
    # Stable source ordering within each file; structs first for readability.
    items.sort(key=lambda pair: (pair[1].type != "struct_item", str(pair[0]), pair[1].start_byte))
    pieces, spans = [], []
    for source, n in items:
        source_data, _ = parse(source)
        pieces.append(render(n, source_data))
        spans.append({"path": str(source.relative_to(crate)), "start_line": n.start_point.row + 1,
                      "end_line": n.end_point.row + 1, "kind": n.type, "name": name(n, source_data),
                      "source_sha256": hashlib.sha256(source_data).hexdigest(),
                      "excerpt_sha256": hashlib.sha256(source_data[n.start_byte:n.end_byte]).hexdigest()})
    result = "\n\n".join(pieces) + "\n"
    expression = case["callsite"]["source_line"]
    locations = [i + 1 for i, text in enumerate(result.splitlines()) if text.strip() == expression]
    if not locations:
        raise ValueError(f"Selected unsafe expression lost: {case['advisory']}")
    return result, spans, locations[0]


def ancestors(node):
    while node.parent:
        node = node.parent
        yield node


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    cases = json.loads((ROOT / "dev/rustsec/claude-cases.json").read_text())["cases"]
    allowed = json.loads((ROOT / "dev/rustsec/allowlist.json").read_text())["cases"]
    assert {(c["advisory"], c["directory"]) for c in cases} == {(c["advisory"], c["directory"]) for c in allowed}
    output = args.output or ROOT / "eval/rustsec-claude-snippets/inputs" / datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = output.resolve()
    (output / "files").mkdir(parents=True, exist_ok=False)
    records = []
    for case in cases:
        code, spans, line = extract(case)
        file = output / "files" / f"{case['advisory']}-{Path(case['directory']).name}.rs"
        file.write_text(code)
        records.append({"advisory": case["advisory"], "file": str(file.relative_to(ROOT)),
                        "sha256": hashlib.sha256(file.read_bytes()).hexdigest(),
                        "line": line, "source_spans": spans})
        print(f"{case['advisory']}: {len(code.splitlines())} lines, {len(spans)} items", flush=True)
    (output / "manifest.json").write_text(json.dumps({"policy": "Original function bodies and required struct declarations; only one source file is mounted per request.", "cases": records}, indent=2) + "\n")
    print(f"Manifest: {output / 'manifest.json'}")


if __name__ == "__main__":
    main()
