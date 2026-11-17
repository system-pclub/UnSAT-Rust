# Source-derived Rust rule pipeline

This run uses `./x rule` on std, core and alloc in that order, including private
and restricted documented unsafe APIs. Step 1 has completed for all 591 APIs,
yielding 1,427 natural-language clauses. The [extraction quality review](quality/README.md)
records the source-based gate before starting full DSL translation.

Step 2 uses the new `tools/x/src/dsl*.py` implementation, model `gpt-6.1-sol`,
direct OpenAI API transport with eight workers and an initial attempt plus at most three repair rounds.
Each global round finishes across all three libraries before failed formulas
enter the next round. The exact Starlark-style expression is parsed with its
Lark grammar and checked against the generated type system. This is not KLEE
execution, and accepted grammar/types do not prove semantic equivalence.
The prompt requests only a complete DSL expression. Refusal/status labels fail DSL validation and enter the repair queue. No expressibility decision is requested.

Exact step 1 inputs/modes and Batch IDs are in each library's `step1-input.json`
and `requests/`; targeted repairs and their raw request caches are retained.
Step 2 inputs, jobs, raw replies, retry queues, per-rule grammar and validation
results are in `step2-input.json` and `translations/`. The old interrupted
public-only run is under `history/public-scope/`; the original three-rule smoke
is retained under `validation/`.

Always build the root Docker image before invoking the wrapper. Resume the
current translation without resubmitting extraction:

```bash
./x rule --output eval/rule --step 2 --include-private \
  --step2-mode llm --workers 8 --retries 3 \
  tools/rust/library/std tools/rust/library/core tools/rust/library/alloc
```

Exit 75 means pending; repeat the same command. Completion and final translation
counts must be read from `std/step2.json`, `core/step2.json` and `alloc/step2.json`.
The generation catalogs here have separate IDs from the root historical catalog.

After the required image build, run `tools/x/scripts/validate_rule_run.py`
inside Docker with `--run-root eval/rule --output eval/rule/run-summary.json`
to check full source/rule inventories, global IDs, retry budgets and retained
grammar/type-check evidence. Its exit 75 means incomplete cloud work, not success.

## Expression-only step2 rerun (2026-11-17)

The previous step2 outputs and request caches were moved to `history/step2-gated-20261117/`. Step0, step1, the quality review and rule IDs are unchanged. All 1,427 clauses are regenerated with the expression-only prompt and direct API transport with eight workers and the same three-repair budget. New results will appear in each library's `step2.json`; errors remaining after the configured repair budget remain errors, not expressibility labels.

The queued expression-only Batch jobs were cancelled on 2026-11-17 at the user's request; their IDs and cancellation responses are under `history/step2-batch-cancelled-20261117/`. Active step2 uses `--step2-mode llm --workers 8`; only step2 is regenerated.

## Expression-only step2 outcomes

| Library | Clauses | Accepted grammar/types | Validation errors after repairs |
| --- | ---: | ---: | ---: |
| std | 214 | 55 | 159 |
| core | 891 | 569 | 322 |
| alloc | 322 | 124 | 198 |

All 1,427 clauses received expression-only DSL-generation attempts through the direct API with eight concurrent workers. The queued Batch jobs were cancelled before this run. No model expressibility classification is accepted. Invalid replies are retained with diagnostics; only failed rules receive up to three repair rounds after the initial attempt. Remaining validation errors are reported as errors, not translations. Grammar/type acceptance does not establish semantic equivalence.

The current change passed 56 relevant unit tests; three real smoke rules retained their raw outputs and failure diagnostics under `validation/expression-only-20261117/`.
