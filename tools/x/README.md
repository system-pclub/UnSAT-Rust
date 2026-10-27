# `x` developer commands

## Translate and review the current core/std catalog

The dissertation uses the frozen selective audit in
`eval/rule-processing/2026-10-16-gpt61-final/`: 958 rules (708 core, 250 std),
15 reusable call operators, and 879 semantically correct translations under
local Codex specification review. Full-catalog success is 879/958 = 91.8%;
among 953 produced expressions it is 92.2%. These are AI review labels, not
independent human annotations. Extraction retains 948 TP, 38 FP and six FN
(96.1% precision, 99.4% recall).

`src/llm/translate_rules.py` translates every submitted rule directly using
`gpt-6.1-sol`, with source argument/type bindings and frozen definitions.
`--retain-correct-from RUN` preserves reviewed correct DSL byte-for-byte;
only unresolved rules are submitted, and review feedback guides revisions.
No support classification or `UNSUPPORTED` response is used. See the final
audit's `README.md` for provenance and reproducible aggregation. Its
`translations.json` and `accepted-translations.json` are the evaluated
catalog outputs. The legacy root `ptr_rule_dsl.json` below is a separate
historical artifact.

## Demonstrated DSL expressibility coverage

`eval/rule-processing/2026-10-16-dsl-coverage/` evaluates the same 958 rules under
exactly the same frozen 15-operator specification. It retains 879 accepted model
translations as witnesses and accepts six additional manually constructed,
separately reviewed capacity witnesses. Demonstrated expressibility coverage
is at least 885/958=92.4% (core 94.5%, std 86.4%); 73 rules remain unresolved.
This is an AI-review-based lower bound, not proof that the remainder is
inexpressible. Manual capacity witnesses do not change the original model
translation result (879 TP,0 FP,79 FN; 91.8% success).

The older keyword-family classifier below does not establish a true
expressibility upper bound or a complete per-rule coverage assessment.

## Historical std/core DSL grammar-family audit

The grammar supports first-order quantification, finite collection literals,
membership, and temporal predicates without adding domain-specific operators.
Run the conservative rule-family audit with an enforced threshold for both
`core` and `std` (as well as the combined inventory):

```bash
PYTHONPATH=tools/x/src tools/x/.venv/bin/python -m dsl.coverage \
  --rules rules.csv --minimum 0.80
```

This reports grammar expressibility only. It does not claim that KLEE can lower
quantified or temporal expressions. Rules that cannot be classified from their
text remain unsupported rather than being counted through an opaque predicate.

The current catalog has 962 canonical `rule-N` entries: 958 in `core`/`std`
(708/250), plus four supplemental `alloc` rules. The inventory
uses only `core` and `std`: 365 API source items and 958 rules. The historical
grammar-family classifier selects 832 of those rules; this classifier is not
used by the current translation workflow. The original extraction audit accepted 948
rules; ten later `core` supplements bring that cohort to 958 without changing
the original extraction TP/FP/FN labels.

All 836 rules selected by the classifier over the complete 962-rule catalog
have parseable expressions in `ptr_rule_dsl.json`. The file has 854 entries:
850 for `core`/`std` and four for `alloc`; 18 `core` translations fall outside
the conservative classifier. `./x
lint-rule-dsl` checks that every translated ID is canonical, exists in
`rules.csv`, and has matching path/name/rule metadata. This is translation
coverage, not a claim that KLEE can prove 80% of the rules at every callsite:
summary-state fields still require corresponding program-state bindings.

The frozen `core`/`std` semantic review in
`eval/rule-processing/2026-10-15/` separately reviews all 850 retained
translations against their rules and current operator/binding semantics.
Local Codex adjudication records 158 correct, 681 incorrect, and 11
unverifiable translations (18.6% success among produced expressions), with
108 untranslated catalog rules. These are AI semantic-review labels, not
human-author annotations or a fresh prompt-run accuracy estimate. Regenerate
the summary with:

```bash
tools/x/.venv/bin/python eval/audit_rule_processing.py
```

## Generate a Rust exploit example

Set `OPENAI_API_KEY`, then select a callsite from the crate's `report.json` and
a safety rule from `ptr_rule_dsl.json`:

```bash
./x gen-exp crates/jh-0.1.0 \
  --callsite src-compressor-rs-112-16 \
  --rule rule-323
```

Unless `--output` is supplied, the generated program is written to the crate's
`examples/` directory as `<callsite>-<rule>.rs`. Use `--model` to override the
default model, or `--report-json` and `--rule-dsl` to select alternate metadata
files. The exact system prompt, user prompt, metadata, and raw model response
are retained under `.local/gen-exp/<crate>/<callsite>/<rule>/`.

## Show verification results

Print one result per `(crate, callsite, rule)`. A rule is `BUG` when it has
`status=violation` and `full_rerun_passed=true`; otherwise it is `OK`:

```bash
./x result
./x result -dir .local/verify/jomini-0.27.0
./x result -dirdir .local/verify
./x result -dirdir .local/verify --granularity caller
./x result -dirdir .local/verify --unsafe-api
```

```text
jomini-0.27.0 src-buffer-rs-67-18 rule-292 BUG
jomini-0.27.0 src-buffer-rs-67-18 rule-294 BUG
jomini-0.27.0 src-buffer-rs-74-28 rule-292 OK
```

`-dir` scans one verification result tree. `-dirdir` scans each child result
tree in a directory. With neither option, `result` uses `.local/verify` as the
directory of result trees. `--granularity rule` is the default and prints one
row per `(callsite, rule)`. `--granularity caller` groups all rules and
callsites in the same caller function into one row; the caller is `BUG` if any
grouped result is `BUG`.

`--unsafe-api` (also available as `--unsafe-api-summary`) groups callsites by
unsafe API. For each API it prints the number of callsites with at least one
confirmed bug over the total number of callsites, first per crate and then
across all crates:

```text
alpha-1.0.0:
core::ptr::offset 2/3
core::slice::get_unchecked 4/5
all:
core::ptr::offset 6/9
core::slice::get_unchecked 7/10
how many unique unsafe api: 2
how many buggy/total unsafe api: 2/2 100.00%
how many buggy/total unsafe api callsites: 13/19 68.42%
```

## Preserve `unreachable_unchecked` verification targets

AutoInj replaces a direct `unreachable_unchecked()` with
`klee_ext_bind::unreachable_unchecked!("<original-callsite-id>")` at the same
expression position. The shim records the callsite and calls KLEE's opaque,
non-returning `klee_silent_exit`. This retains the source branch and its never
type while preventing LLVM from erasing the observation because of subsequent
undefined behavior. A path that reaches an earlier, unselected unchecked target
also terminates there; it cannot continue to a later selected target.

For unambiguous local `macro_rules!` definitions, AutoInj specializes the macro
and its local helper macros for that invocation. It changes only unchecked calls
in expansion bodies. Matchers, argument tokens, conditions, debug/release branches,
and single evaluation of arguments are retained. For example, REVM's
`assume! -> debug_unreachable! -> unreachable_unchecked()` is handled without
editing the benchmark source or inserting a marker before the assumption guard.
External, ambiguous, and recursive macro definitions remain explicitly skipped.

Release-only unchecked branches require `./x verify ... --no-debug-assertions`.
Generic callers still need a compile-only instantiation or concrete harness to
emit their body. Marker preservation fixes target availability; a preserved
marker alone does not establish a DSL violation. Rule-89 (`false`) is violated
only when KLEE actually reaches that original branch.

Rebuild `tools/autoinj/target/release/autoinj` after source changes: `x sync`
prefers this binary. The end-to-end regression checks O0/O3, valid/invalid
branches, debug configuration, and termination at an earlier unchecked target:

```bash
PYTHONPATH=tools/x/src tools/x/.venv/bin/python \
  dev/check_unreachable_instrumentation.py
```

Add `--real` with Cargo/custom-rustc configured to reinject the five original
REVM macro targets and Arrow2 `OffsetsBuffer::<i32>::first`. Each run writes a
fresh evidence tree under `.local/unreachable-instrumentation/`. Native fallback
stubs panic if an instrumented unchecked branch is reached; this shim is for
verification builds.
