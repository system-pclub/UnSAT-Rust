# `x` developer commands

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

Print one result per crate callsite. A callsite is `BUG` when any matched rule
has `status=violation` and `full_rerun_passed=true`; otherwise it is `OK`:

```bash
./x result
./x result -dir .local/verify/jomini-0.27.0
./x result -dirdir .local/verify
```

`-dir` scans one verification result tree. `-dirdir` scans each child result
tree in a directory. With neither option, `result` uses `.local/verify` as the
directory of result trees.
