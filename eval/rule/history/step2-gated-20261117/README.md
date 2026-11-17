Archived step2 run with the historical model expressibility gate. Active step2 is being regenerated with expression-only output.

# Source-derived Rust rule pipeline

This run uses `./x rule` on std, core and alloc in that order, including private
and restricted documented unsafe APIs. Step 1 has completed for all 591 APIs,
yielding 1,427 natural-language clauses. The [extraction quality review](../../quality/README.md)
records the source-based gate before starting full DSL translation.

Step 2 uses the new `tools/x/src/dsl*.py` implementation, model `gpt-6.1-sol`,
OpenAI Batch transport and an initial attempt plus at most three repair rounds.
Each global round finishes across all three libraries before failed formulas
enter the next round. The exact Starlark-style expression is parsed with its
Lark grammar and checked against the generated type system. This is not KLEE
execution, and accepted grammar/types do not prove semantic equivalence.
`UNEXPRESSIBLE` is a terminal model judgment, not proof of inexpressibility.

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
  --step2-mode batch --retries 3 --batch-no-wait \
  tools/rust/library/std tools/rust/library/core tools/rust/library/alloc
```

Exit 75 means pending; repeat the same command. Completion and final translation
counts must be read from `std/step2.json`, `core/step2.json` and `alloc/step2.json`.
The generation catalogs here have separate IDs from the root historical catalog.

After the required image build, run `tools/x/scripts/validate_rule_run.py`
inside Docker with `--run-root eval/rule --output eval/rule/run-summary.json`
to check full source/rule inventories, global IDs, retry budgets and retained
grammar/type-check evidence. Its exit 75 means incomplete cloud work, not success.

## Completed translation run

| Library | Clauses | Accepted grammar/types | UNEXPRESSIBLE |
| --- | ---: | ---: | ---: |
| std | 214 | 16 | 198 |
| core | 891 | 352 | 539 |
| alloc | 322 | 60 | 262 |

All requested clauses have terminal results with no preparation, request or validation errors remaining. The retry budget was the initial attempt plus up to three repairs. Accepted grammar/types do not establish semantic equivalence; UNEXPRESSIBLE remains a model judgment.

Validation: 85 relevant tests passed, plus the RustSec source/evidence checker and dry-run. See `quality/validation-tests.log` and `run-summary.json`.
