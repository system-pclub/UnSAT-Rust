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
