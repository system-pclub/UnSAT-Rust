# Roster ablation preparation

No ablation has been launched by this preparation. The settings and output
format still require the owner's answer. Original stage JSON remains unchanged.

`plan_ruklee_ablation.py` defaults to writing a plan only. `--suite pipeline`
selects six pipeline variants; `--suite profiles` selects full/a/b/c; `--suite all`
selects their union of nine variants. It accepts an optional `--setting` subset.
Each setting writes to `eval/ruklee-<setting>/<versioned-crate>/`. Plans, commands,
exit statuses and runner logs stay beneath the requested `--metadata-root`.
`--execute` is a separate explicit switch. `--resume` skips successful scheduled
jobs and resumes incomplete matrices; existing failed matrix rows are preserved
by the verification frontend and require deliberate retries when appropriate.

Example review-only plan after freezing the catalog:

```bash
python3 dev/benchmark/plan_ruklee_ablation.py --suite pipeline \
  --catalog dev/benchmark/ruklee-ablation-prep-20261122/catalog \
  --metadata-root dev/benchmark/ruklee-ablation-prep-20261122/pipeline-plan
```

The default roster is `dev/benchmark/sources.json`, preserving its order and all
32 `directory` entries. Roster mode uses the package root and default features.
In particular, it does not silently substitute blade-graphics for blade or add
arrow2's workbook-specific compute_sort feature. `--package-scope workbook`
explicitly opts into those older package/feature choices, which must be disclosed
when comparing against Claude's original published package analysis.

The legacy `ablation/*.py` entrypoints keep their default top-ten input. Their
new optional arguments are `--roster`, `--rule-dsl`, `--studied-rules`,
`--metadata-root`, `--setting-name`, `--ablation-level`, and `--package-scope`.
Roster mode requires metadata outside eval. The schedule runs serially and
shares `.local/run-all-crates/runner.lock`: source/report.json, source target,
crates metadata, human metadata and crates_inj are keyed by package directory
basename, so separate IR directories alone do not make same-crate runs safe.

Freeze current step2 input locally inside the root image, after the required
normal cached Docker build:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 \
  -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  /opt/x/.venv/bin/python tools/x/scripts/prepare_step2_catalog.py \
  --source /unsat-rust/eval/rule \
  --output /unsat-rust/dev/benchmark/ruklee-ablation-prep-20261122/catalog \
  --supplement ''
```

The empty supplement argument deliberately disables the old supplement: its
rule numbers refer to the earlier 748-rule catalog, and now point at different
APIs or unexpressible rows. The current source has 381 translated rules
(44 alloc, 326 core, 11 std) and 781 unexpressible rows. The preparation preserves
translated expressions, validates and lowers them without repair, enables the
built-in raw-pointer check separately, and writes excluded rows and source hashes.
The comment-only generated studied_rules file prevents legacy rule tasks from
contaminating rule selection; every MIRScan target is retained, and rules are
matched later by unsafe callee path/line, with path/function-name fallback.

Remaining review before execution: the existing E/T preparation prompts still
use the historical operator DSL/selector table. Roster and explicit catalog
support do not establish that those LLM-backed ablations use the current DSL
generation protocol. Their semantics and resume provenance need review before
launch; no paid generation has been used to test them. No compatibility summary
has been enabled while the requested output format is pending.
