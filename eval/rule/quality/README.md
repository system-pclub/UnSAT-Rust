# Extraction quality gate (2026-11-16)

The full extraction contains 591 source APIs: std 140, core 294, alloc 157.
The effective catalog has 1,427 clauses: std 214, core 891, alloc 322.
The prompt was kept concise, with source-backed delegation context rather than
API-specific instructions. Trials selected a few functions with the largest
missing contracts before expanding the run.

| Review | Eligible legacy clauses | Covered | Missed | Coverage | Strictly source-supported generated clauses |
| --- | ---: | ---: | ---: | ---: | ---: |
| std | 212 | 209 | 3 | 98.58% | 214/214 |
| core | 687 | 681 | 6 | 99.13% | 890/891 |
| combined | 899 | 890 | 9 | 99.00% | 1104/1105 (99.91%) |

These are local Codex semantic/source review labels, with no external judge API
calls. Coverage matches clauses within the same source API, preserving the
responsible party, conditions, exceptions, units and time scope. Split/merged
clauses and duplicates can share a match. It is not a sentence-count ratio or a
fuzzy-text score, and does not establish DSL semantic correctness or KLEE
execution. The gate covers eligible **legacy std/core clauses**, not every
contract absent from that incomplete catalog. Alloc has no comparable complete
legacy gold catalog, so it has no claimed recall score here.

`baseline-rules.csv` preserves all 962 original root rows. Of 958 std/core rows,
59 were ineligible, each with a reason in `std-judgment.json` or
`core-judgment.json`: marker/reference placeholders, checked panic/error behavior,
descriptive rationale, or an obligation copied to the wrong API/reference kind.
Explicit requirements remain eligible even when a later error check handles
violations. The judgment files also retain all nine conservative misses. These
include `_mm_prefetch` outside the three physical source folders, omitted
sufficient routes/trait-access conditions, and two overstrong legacy
same-allocation clauses whose same-address exception the new extraction
correctly preserves. One async no-move clause remains uncertain under a strict
explicit-source criterion and receives no precision credit.

Review packets retain actual request context and provenance. Structural scope
comparison is in `comparison.json`: the original public-only run excluded 176
legacy clauses by visibility and failed to extract another 116 clauses in
included APIs. These are structural categories, not semantic FN counts.
The original output and raw requests remain in `../history/public-scope/`.

Targeted repairs covered safety delegation written in bare backticks,
reference definitions after examples, `#method.*` anchors, generic `Self`
owners, allocator definitions and module memory-layout anchors. Small trials
also caught a checked DWARF requirement being dropped and a `not both` ownership
condition being strengthened incorrectly. Repaired APIs retain their original
Batch results in `step1-base.json`, their overlay in `step1-repairs.json`, and
all exact request bodies and raw responses in each library's `requests/`.
An additional iterator trial was not promoted because its receiver/trait
conditions needed further review; trial outputs are not silently substituted
for the reviewed corpus.

The root `rules.csv` is a separate, stable-ID catalog: its original std/core rows
are unchanged, existing alloc IDs are preserved, four alloc clauses are corrected
against source, and 318 alloc clauses are appended (1,280 total rows).
`alloc-root-catalog-map.json` records the mapping and previous text, including
the Box zero-sized-type exception. Generated `../rules.csv` has its own IDs and
must not replace root IDs or `ptr_rule_dsl.json`.

After the root Docker build, reconstruct review packets with:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 \
  -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  python tools/x/scripts/prepare_rule_review.py --library core \
    --run-root eval/rule --output .local/core-review-packets.json
```

Frozen resume uses `./x rule --output eval/rule --step 1 --include-private
--resume-step1-input` with std/core/alloc input directories in that order.
Step 2 then uses the saved extraction, Batch mode and three configurable repair
rounds after the initial attempt. See the parent README for final status counts.
