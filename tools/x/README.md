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
