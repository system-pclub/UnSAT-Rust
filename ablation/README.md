# RuKLEE ablation experiments

The benchmark is read from `raudit.xlsx`, sheet `top10 github bugs`. Its ten
crates are fixed to `hftbacktest`, `jh`, `revm`, `rust-crypto`, `lurk-cli`,
`arrow2`, `blade`, `memmap`, `fast-float`, and `meshopt`; the scripts map those
names to the versioned directories under `benchmark/ruklee/`.

The experiments follow SymGPT's rule for ablations: remove a stage directly
when the pipeline remains executable, and replace an indispensable stage with
an LLM judgment when simply deleting it would make the experiment undefined.

| Entry | Variant | Implementation |
|---|---|---|
| `full.py` | RuKLEE | Complete pipeline. |
| `without_rule_extraction.py` | W.O. E | Translate each complete unsafe API documentation block directly to DSL, without first extracting atomic natural-language rules, then run the normal backend. |
| `without_mut_ref_recovery.py` | W.O. M | Ban direct and aggregate/wrapper `&mut` escapes during safe-context recovery; constructors and direct mutators remain enabled. |
| `without_symbolic_execution.py` | W.O. S | Ask the LLM to judge each DSL rule against related Rust code; KLEE is not run. |
| `without_validation.py` | W.O. V | Stop after initial KLEE checking; do not generate or rerun a same-callsite testcase. |

All runners accept `--crate NAME` (repeatable), `--resume`, `--dry-run`,
`--timeout-sec`, `--model`, `--rustc`, `--klee-bin`, and `--results-root`.
They default to the repository's patched stage-1 rustc and KLEE build; run
`./build_rust.sh`, `./build_mirscan.sh`, and `./build_klee.sh` first on a fresh
checkout. Examples:

```bash
python3 ablation/full.py --resume
python3 ablation/without_mut_ref_recovery.py --resume
python3 ablation/without_validation.py --resume
python3 ablation/without_rule_extraction.py --resume
python3 ablation/without_symbolic_execution.py --resume
```

The two LLM replacement experiments require `OPENAI_API_KEY` and should be run
with `tools/x/.venv/bin/python`, which contains the existing OpenAI dependency.
Prompts and raw responses are retained with the results. The direct-document
rules are cached in
`ablation/results/without-rule-extraction/direct-document-rules.json`.

Score and plot only after the desired runs finish:

```bash
python3 -m pip install -r ablation/requirements.txt
python3 ablation/summarize_results.py
python3 ablation/plot_results.py --png ablation/results/ablation.png
```

`summarize_results.py` parses the workbook directly with Python's standard
library. A ground-truth row is a TP when at least one positive callsite matches
its crate, path, and annotated line/range. An unmatched positive callsite is an
FP, and an unmatched workbook row is an FN. For W.O. Validation, an initial
KLEE violation candidate counts as positive; the other KLEE variants require a
successful full rerun. W.O. Symbolic Execution uses the LLM's direct Boolean
prediction. These are preliminary location-level scores: when multiple safety
obligations share one callsite, final paper-grade TP/FP/FN still requires
rule-level manual adjudication. The plot uses the same unfilled green dotted TP
bars, red hatched FP bars, value annotations, and dashed total line as SymGPT
Figure 11.
