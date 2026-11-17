---
name: dsl-eval
description: Run or reproduce the eval/rule documentation-to-DSL pipeline through ./x rule, including stages, durable OpenAI Batch jobs, repair rounds, and result validation.
---

# Run DSL evaluation through x

Work from the repository root. Read [ruklee-environment](../ruklee-environment/SKILL.md)
first: build the root Dockerfile as `unsat-rust:local` before **every** container
run and validate inside Docker. Use `./x rule`, implemented by
`tools/x/src/cli/cmd/rule.py`, `llm/rule_pipeline.py`, and `llm/rule_jobs.py`.
Changes to the entrypoint/build environment follow that skill's review rule.

## Inputs and scope

Keep the input directories **and their order** fixed, including on resume:

```bash
rule_dirs=(tools/rust/library/std tools/rust/library/core tools/rust/library/alloc)
```

The default source scope is public APIs. Use `--include-private` for comparison
against the legacy std/core catalog, which includes restricted/private APIs.
Step 0 records the scope in `step0-input.json`. `--api-location PATH:LINE`
(repeatable, normalized `rust/library/...` paths) selects exact step 1 APIs for
targeted prompt trials; unknown locations fail before requests are submitted.

The output root records their order in `inputs.json`; each library's `input.json`
records its source identity. Do not run concurrent writers to one output.
Normal outputs belong under `eval/rule/`; smoke runs use a separate `.local/`
directory. Never let a limited smoke run replace a full catalog.

Uncached steps 1 and 2 send documentation/signatures and DSL definitions to OpenAI
and incur API charges. Run only the user-authorized scope. The key is loaded from
`OPENAI_API_KEY` or repository `.env`; never print or commit it. Documentation or
local-only checks do not authorize paid requests.

## Stages and modes

After the required Docker build:

```bash
./x rule --output eval/rule --model gpt-6.1-sol \
  --step1-mode batch --step2-mode batch --retries 3 "${rule_dirs[@]}"
```

Omitting `--step` runs all stages. `--step 0` refreshes source comments using
`tools/extract-comment`; `--step 1` requires `step0.json` and extracts atomic
natural-language obligations; `--step 2` requires `step0.json` and `step1.json`,
rebuilds the globally numbered catalog, then uses the **new source-derived DSL**.
Each LLM stage independently supports `--step1-mode llm|batch` and
`--step2-mode llm|batch` (defaults: `llm`). The two transports use identical
prompts and validation.

Step 2 uses `tools/x/src/dsltrans.py`, `dslapi.py`, `dsltypes.py`, `dslgrammar.py`,
and `dslmemory.{py,json}`. The Rust generator stays in `tools/dslgen`; a missing
`tools/x/generated/dsl/schema.json` is generated automatically. `--dslgen PATH`
selects a different catalog. After changing the Rust sources, regenerate with
`cargo run --locked --manifest-path tools/dslgen/Cargo.toml` in Docker, then
refresh step 0. The old operator DSL and `llm.translate_rules` remain legacy tools;
`x rule` step 2 does not use them.

`--retries 3` means **one initial attempt plus up to three repair rounds**.
A global barrier waits for every requested library in a round before any library
submits repairs. After every whole round completes, parse each raw Starlark-style expression,
compile its exact Lark grammar, and check its types. Only failed rules enter the
next round, with the prior output and diagnostic. `UNEXPRESSIBLE` is a terminal
model result, not a successful formula or proof of inexpressibility. Preparation
errors are reported separately; do not relabel them as unexpressible.

## Batch resume

Batch mode waits by default, polling every 30 seconds (`--batch-poll-seconds`).
For submission/polling without waiting:

```bash
./x rule --output eval/rule --step1-mode batch --step2-mode batch \
  --retries 3 --batch-no-wait "${rule_dirs[@]}"
```

Exit **75** means jobs are pending. Repeat the exact command to resume; exit 0
means requested stages finished, while other nonzero statuses are errors. All
requested libraries/shards are submitted before returning pending. Never treat
elapsed time or a submitted job as completion. Match output by `custom_id`, not
line order. Completed output/error files and terminal partial results are retained.
Successful requests are reused. Step 1 errors can be retried by rerunning;
step 2's repair budget survives restart and can be extended with `--retries`.
Do not silently switch model/mode to bypass an error. Changed request settings or
DSL implementation select a new translation fingerprint; an existing round with
changed mode/input is rejected. Inspect retained inputs before changing settings.

After a prompt edit, `--step 1 --resume-step1-input` resumes the saved model,
mode, exact request bodies and job IDs instead of regenerating requests with the
new prompt. It rejects changed source snapshots and changed API selection.
For a targeted successful repair, use `tools/x/scripts/apply_rule_repairs.py`
with `--dataset`, `--repairs`, and `--libraries`. It checks source identity,
retains `step1-base.json`, and records a `step1-repairs.json` overlay with each
repair's request-cache provenance. Frozen resume preserves those repairs.

For a quality gate, compare source APIs before scheduling a corpus rerun. Use
`tools/x/scripts/compare_catalogs.py` for structural differences and
`prepare_rule_review.py` for source/request review packets. Counts alone are not
precision or recall: review responsible parties, guards, exceptions, units and
lifetimes against source, recording every excluded legacy clause and every FN.
Test the worst few APIs first, then expand. The current extraction review and
its limitations live in `eval/rule/quality/`; its scores do not establish DSL
translation correctness or recall of contracts absent from the legacy catalog.

## Artifacts

```text
eval/rule/
  inputs.json, rules.csv
  std/                              also core/ and alloc/
    input.json, step0.json
    step1-input.json, step1.json, rules.csv
    requests/<fingerprint>/round-NN/
      input.json                    full request bodies + mode + endpoint
      batch-NNNN/
        input.jsonl, job.json        upload file ID, job ID, status, remote metadata
        output_file_id.jsonl, error_file_id.jsonl, results.json
      <request-id>/response.json, result.json
    step2-input.json, step2.json
    translations/<fingerprint>/
      <rule-id>/                    source schema, fixed API variables, prompt EBNF,
                                    memory semantics, result, accepted grammar.lark
      round-NN/                     input.json, raw replies, per-rule result,
                                    retry-queue.json, Batch artifacts
      generation-results.json
```

Direct mode retains `response.json` under each round's request ID. Batch raw
replies are retained in downloaded JSONL/`results.json`; `response.txt` stores each
translation's unchanged model text. Input manifests point to the active cache.
Fingerprints include rules, source context, schema, DSL implementation, model,
reasoning effort and output-token limit. Preserve old successful caches.

## Bounded checks

Before a full paid run, validate with a separate output:

```bash
./x rule --output .local/rule-smoke-compact --limit 1 --rule-limit 1 \
  --step1-mode batch --step2-mode batch "${rule_dirs[@]}"
```

`--limit` bounds source APIs **per library** at step 1 (each may yield several
rules). `--rule-limit` bounds saved rules **per library** at step 2. Neither
establishes full corpus completeness. Inspect the actual number of rules.

After the required image build, local tests without paid calls:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 \
  -e PYTHONPATH=/unsat-rust/tools/x/src -v "$PWD:/unsat-rust" \
  -w /unsat-rust unsat-rust:local bash -c \
  'python -m unittest cli.cmd.test_rule llm.test_rule_pipeline && python -m unittest discover -s tools/x/tests -p "test_dsl*.py"'
```

Check real run exit status, source identity/order, all expected step 1 items marked
`extracted`, catalog IDs and every step 2 status. Report counts of `translated`,
`unexpressible`, preparation and request/validation errors separately. Grammar/type
acceptance proves neither semantic fidelity nor KLEE execution. Do not overwrite
`ptr_rule_dsl.json`, root `rules.csv`, or benchmark rule IDs with generated output.
An explicitly requested root-catalog supplement must preserve existing IDs and
record any source corrections and the mapping separately; never replace it with
the generated eval IDs. Follow the environment skill's post-run disk checks and
preserve current evidence.
