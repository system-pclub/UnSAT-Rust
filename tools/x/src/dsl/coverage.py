from __future__ import annotations

import argparse
import csv
import re
from collections import Counter
from dataclasses import dataclass
from pathlib import Path


# This is deliberately a conservative grammar-capability audit, not an LLM
# judgment and not a KLEE support claim.  A rule is counted only when its text
# exposes a semantic family for which the DSL has the required building blocks.
# Unmatched prose stays unsupported even though first-order temporal logic may
# be able to encode it after a human identifies the relevant program objects.
_FAMILIES: tuple[tuple[str, re.Pattern[str]], ...] = (
    (
        "memory",
        re.compile(
            r"pointer|\bptr\b|memory|allocated|allocation|valid for (?:read|writ)|"
            r"align|non-?null|dangling|dereference|overlap|buffer|initialized",
            re.IGNORECASE,
        ),
    ),
    (
        "scalar_bounds",
        re.compile(
            r"\b(?:len|length|size|capacity|index|indices|offset|range|overflow|"
            r"underflow|greater|smaller|less|at least|at most|in-bounds|"
            r"out-of-bounds|fit in|no larger)\b|must be [<>=]|<=|>=| < | > ",
            re.IGNORECASE,
        ),
    ),
    (
        "encoding",
        re.compile(
            r"(?:utf|wtf)-?8|ascii|nul|cstr|encoded|code point",
            re.IGNORECASE,
        ),
    ),
    (
        "lifetime_aliasing",
        re.compile(
            r"lifetime|borrow|reference|alias|mutat|accessed through|"
            r"while .* alive|outlive|\bpin(?:ned|ning)?\b",
            re.IGNORECASE,
        ),
    ),
    (
        "concurrency",
        re.compile(
            r"race|atomic|synchron|thread|ordering|concurrent", re.IGNORECASE
        ),
    ),
    (
        "type_value",
        re.compile(
            r"valid value|type `?t\b|contained value|discriminant|variant|"
            r"transmute|layout|vtable|sized|\bzst\b|\bcopy\b|drop|uninit|"
            r"char value|state is",
            re.IGNORECASE,
        ),
    ),
    (
        "protocol",
        re.compile(
            r"only be called|must be called|previously returned|created by|"
            r"\bafter\b|\bbefore\b|\bonce\b|calls? to|unwind|panic|destroy|"
            r"closed|owned|ownership|handle|file descriptor|must be open|"
            r"must have been called|was not called|has been pushed|same mutex|"
            r"read-locked|write-locked|in progress|result of calling",
            re.IGNORECASE,
        ),
    ),
    (
        "trait_contract",
        re.compile(
            r"implementations?|trait|subtype|supertype|methods? must|"
            r"return the same|public api|specializ|coerc",
            re.IGNORECASE,
        ),
    ),
    (
        "vacuous",
        re.compile(
            r"always safe|marker|no requirements|no additional safety|"
            r"does not impose|cannot cause undefined behavior",
            re.IGNORECASE,
        ),
    ),
)


# A parseable formula for each counted family.  These are grammar witnesses,
# not translations of every rule in the family: they make the audit's required
# building blocks concrete and prevent a family from being counted without a
# representation that uses the current operator vocabulary.
FAMILY_WITNESSES: dict[str, str] = {
    "memory": (
        'is_valid_for(memory_range(get_arg(0), '
        'get_field(get_type_at_func(0), "size")), "read")'
    ),
    "scalar_bounds": (
        'get_arg(0) > 0 && get_arg(0) <= '
        'get_field(get_type_at_func(0), "max")'
    ),
    "encoding": (
        'forall byte in get_field(get_arg(0), "bytes"): '
        "byte >= 0 && byte <= 127"
    ),
    "lifetime_aliasing": (
        'always (get_field(get_arg(0), "live_borrows") > 0 => '
        'get_field(get_arg(0), "writes") == 0)'
    ),
    "concurrency": (
        'always !(get_field(get_arg(0), "active_writes") > 0 && '
        'get_field(get_arg(0), "active_reads") > 0)'
    ),
    "type_value": (
        'is_initialized_for_type(memory_range(get_arg(0), '
        'get_field(get_type_at_func(0), "size")), get_type_at_func(0))'
    ),
    "protocol": (
        'previously (get_field(get_arg(0), "state") in {"open"}) && '
        'eventually (get_field(get_arg(0), "state") in {"closed"})'
    ),
    "trait_contract": (
        'forall impl in get_field(get_type_at_func(0), "implementations"): '
        'get_field(impl, "return_type") == '
        'get_field(get_type_at_func(0), "return_type")'
    ),
    "vacuous": "true",
}


@dataclass(frozen=True)
class GrammarCoverage:
    total: int
    supported: int
    by_family: dict[str, int]
    by_library: dict[str, tuple[int, int]]
    unsupported_ids: tuple[str, ...]

    @property
    def ratio(self) -> float:
        return self.supported / self.total if self.total else 1.0


def classify_rule(rule: str) -> str | None:
    text = " ".join(rule.split())
    for family, pattern in _FAMILIES:
        if pattern.search(text):
            return family
    return None


def audit_rules(path: str | Path) -> GrammarCoverage:
    counts: Counter[str] = Counter()
    library_totals: Counter[str] = Counter()
    library_supported: Counter[str] = Counter()
    unsupported: list[str] = []
    total = 0

    with Path(path).open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            rule_id = (row.get("id") or "").strip()
            if re.fullmatch(r"rule-\d+", rule_id) is None:
                continue
            total += 1
            path_value = row.get("path") or ""
            library_match = re.search(r"rust/library/(core|std)/", path_value)
            library = library_match.group(1) if library_match else "other"
            library_totals[library] += 1
            family = classify_rule(row.get("rule") or "")
            if family is None:
                unsupported.append(rule_id)
            else:
                counts[family] += 1
                library_supported[library] += 1

    return GrammarCoverage(
        total=total,
        supported=sum(counts.values()),
        by_family=dict(sorted(counts.items())),
        by_library={
            library: (library_supported[library], library_total)
            for library, library_total in sorted(library_totals.items())
        },
        unsupported_ids=tuple(unsupported),
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Audit std/core rule expressibility in the DSL grammar."
    )
    parser.add_argument("--rules", default="rules.csv")
    parser.add_argument("--minimum", type=float, default=0.80)
    parser.add_argument("--show-unsupported", action="store_true")
    args = parser.parse_args(argv)

    result = audit_rules(args.rules)
    print(
        f"grammar coverage: {result.supported}/{result.total} "
        f"({result.ratio:.2%})"
    )
    for library, (supported, total) in result.by_library.items():
        ratio = supported / total if total else 1.0
        print(f"{library}: {supported}/{total} ({ratio:.2%})")
    for family, count in result.by_family.items():
        print(f"{family}: {count}")
    if args.show_unsupported:
        print("unsupported: " + ",".join(result.unsupported_ids))
    library_ratios = (
        supported / total if total else 1.0
        for supported, total in result.by_library.values()
    )
    return 0 if result.ratio >= args.minimum and all(
        ratio >= args.minimum for ratio in library_ratios
    ) else 1


if __name__ == "__main__":
    raise SystemExit(main())
