# Source-derived Rust rule pipeline

This run uses `./x rule` on std, core and alloc in that order, restricted to
public documented unsafe APIs (`step0-input.json` has `include_private: false`).
Step 1 holds the reviewed extraction filtered to those APIs: 79 std, 240 core and
59 alloc APIs, yielding 1,162 natural-language clauses (131, 814 and 217). The
[extraction quality review](quality/README.md) covers the earlier, larger
private-inclusive corpus; its counts do not describe this run.

Step 2 uses the `tools/x/src/dsl*.py` implementation, model `gpt-6.1-sol`, OpenAI
Batch transport and an initial attempt plus at most three repair rounds. Each
global round finishes across all three libraries before failed formulas enter the
next round. The prompt asks for a complete DSL expression, or exactly
`UNEXPRESSIBLE` when a required observation is unavailable; that label is a
terminal `unexpressible` status. Other expressions are parsed with the Lark
grammar and type-checked. This is not KLEE execution, and accepted
grammar/types do not prove semantic equivalence.

Exact step 1 inputs/modes and Batch IDs are in each library's `step1-input.json`
and `requests/`; targeted repairs and their raw request caches are retained.
Step 2 inputs, jobs, raw replies, retry queues, per-rule grammar and validation
results are in `step2-input.json` and `translations/`. Earlier runs are under
`history/`; the original three-rule smoke is retained under `validation/`.

Always build the root Docker image before invoking the wrapper. Resume the
current translation without resubmitting extraction:

```bash
./x rule --output eval/rule --step 2 --model gpt-6.1-sol \
  --step2-mode batch --retries 3 \
  tools/rust/library/std tools/rust/library/core tools/rust/library/alloc
```

Exit 75 means pending; repeat the same command. Completion and final translation
counts must be read from `std/step2.json`, `core/step2.json` and `alloc/step2.json`.
The generation catalogs here have separate IDs from the root historical catalog.

After the required image build, run `tools/x/scripts/validate_rule_run.py`
inside Docker with `--run-root eval/rule --output eval/rule/run-summary.json`
to check full source/rule inventories, global IDs, retry budgets and retained
grammar/type-check evidence. Its exit 75 means incomplete cloud work, not success.

## Public-scope step2 rerun (2026-11-21)

Private and restricted APIs were removed from step 0/1, the rule translation
prompt was reverted to allow `UNEXPRESSIBLE`, and all previous step 2 outputs were
deleted (they remain in git history and `history/`). Step 2 was rerun through
OpenAI Batch.

| Library | APIs | Clauses | Translated | Unexpressible | Errors |
| --- | ---: | ---: | ---: | ---: | ---: |
| std | 79 | 131 | 11 | 120 | 0 |
| core | 240 | 814 | 326 | 488 | 0 |
| alloc | 59 | 217 | 44 | 173 | 0 |

Rule IDs are renumbered for this catalog. Grammar/type acceptance does not
establish semantic equivalence, and `unexpressible` is a model judgment that has
not been audited against the DSL.
