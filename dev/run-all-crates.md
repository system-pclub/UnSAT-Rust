# Serial full-mode crate run without LLM calls

From the repository root:

```bash
./run-all-crates .local/crates
```

The argument is a directory whose immediate children contain `Cargo.toml`.
It must be inside this checkout, which `./x` mounts into Docker. The runner uses
a Bash `for` loop, in C-locale path order, with one crate active at a time.
Another runner is excluded by a lock. `./x verify <crate>` scans the crate's
MIRScan callsites and matches rules by the unsafe callee's source identity.
This is not a claim that every function or generic instantiation is analyzable.
Build failures, missing bodies, unmatched APIs, KLEE evaluation errors, uncertain
SAT and timeouts remain explicit outcomes; a finished crate is not necessarily
a verified crate.

The run reads only `status == "translated"` rows from the existing
`eval/rule/{alloc,core,std}/step2.json` files. `expression` is kept unchanged in
the catalog, DSL input and backend AST's source field. The existing Python
parser validates and serializes it; no expression repair or alternative rule is
substituted. Original file hashes, IDs and non-translated exclusions are retained.
Generated IDs are namespaced as `step2-core-rule-...` to avoid the unrelated
legacy catalog's special rule IDs. The root catalogs and eval outputs are not
overwritten. In addition to these catalog rules, new runs enable the frontend's
built-in raw-pointer dereference check (`__raw_ptr_deref__`), as requested by the
owner. This check is independent of the step2 DSL rules. Its enabled state is
recorded in `rules.json` and `catalog-manifest.json`. Existing run catalogs retain
their original settings; the next new run or maintenance restart enables it.

Each invocation uses `--ablation-level full --skip-llm-testcase`, and stops
after KLEE's matrix results. It does not run `./x rule`, regenerate translations,
generate LLM testcases, or invoke LLM validation/replay. Dependency downloads and
Rust builds may still use the network. `prepare_step2_catalog.py` is entirely
local; its tests forbid OpenAILLM construction.
The runner removes `OPENAI_API_KEY` from the environment passed to `./x`.

`./x verify` always stops after the KLEE matrix (the LLM PoC and rerun stages are
disabled) and writes `violations.json` beside `matrix.json`: one entry per
(callsite, rule) with the rule's DSL, `violation` true/false, the compose
`call_chain` and the `callchain` of certain symbols (how safe code controls the
violating input). `violation` is true for status `violation`, and for
`violation-timeout` only when the witness depends on certain symbols. A witness
found while "continuing under optimistic uncertainty" is never certain, so it is
reported as `low-confidence-sat`, not as a violation.

The image is rebuilt with the normal Docker cache before every container run.
KLEE/rustc/LLVM/MIRScan build trees, Cargo registry caches, source code and other
experiments are preserved. The runner clears only its own linked IR and newly
created injection/source build outputs after each crate. Completed KLEE pairs
retain result/control-chain JSON and compressed full logs/info. Every KLEE output
file, including all ktests, errors and queries, is preserved losslessly in a
verified `klee-out.tar.gz` with a SHA-256 checksum before the working directory
is removed. The exact linked IR, injection source/manifest/lockfile and metadata
are archived in `replay-inputs.tar.gz`; only the root Cargo `target` cache is
excluded, never source subdirectories with that name. Paths in the original matrix
still refer to the original log: append `.gz` after cleanup. A removed injection
directory and IR can be restored from the replay archive, whose entries preserve
repository-relative paths. Extract into a separate inspection directory first;
do not overwrite an active run. Protected archives/results are never deleted by
the disk watchdog: the batch stops if preserving them leaves insufficient space.

Disk space is checked before each crate and every 15 seconds during a crate.
Below 100 GiB, the watcher prunes completed pairs from the active run. Below
50 GiB, it stops the active crate/container and preserves partial results. If
100 GiB cannot be recovered before the next crate, the batch stops with code 75.
It never prunes Docker images, shared toolchain/registry caches, other runs or
source crates to force progress. Cleanup only applies to outputs actually
created/recreated by this run; unchanged pre-existing work is left alone on
early failures.

Configurable environment variables:

| Variable | Default | Meaning |
|---|---:|---|
| `MIN_FREE_GIB` | 100 | Required space before the next crate; active cleanup threshold |
| `STOP_FREE_GIB` | 50 | Stop threshold during a crate |
| `DISK_CHECK_SECONDS` | 15 | Active-run disk check interval |
| `KLEE_TIMEOUT_SEC` | 180 | Per callsite/rule KLEE timeout |
| `CRATE_TIMEOUT_SEC` | 3600 | Whole-crate timeout; 0 disables this timeout |
| `MAX_CRATES` | 0 | Limit for smoke runs; 0 means all crates |
| `START_INDEX` | 1 | Restart from this 1-based crate index in a new run |

Per-crate failures are recorded and the next crate starts. Results live under
`.local/run-all-crates/run-<UTC>.<unique>/`; `.local/run-all-crates/latest`
contains the current run path. `runner.pid`, `status`, `current-index`,
`progress.tsv`, `disk.log`, `catalog/` and `jobs/<index>/` provide progress,
source provenance, commands, exit codes and retained verification evidence.
`findings.jsonl` aggregates SAT/uncertain-SAT candidates when each crate finishes;
`jobs/<index>/findings.json` is the per-crate report list. `analysis-errors.jsonl`
records other non-verified outcomes separately. New records include `dsl` (the
exact original expression) and `original_rule` (the original safety-rule sentence),
alongside the existing `rule` ID. Metadata comes from the retained pair result,
with the run catalog as fallback. Built-in raw-pointer checks have no DSL and
therefore report `dsl: null`; their original check description is retained.
Matrices and per-pair results
remain available during an active crate. A SAT report is not labelled a confirmed
vulnerability without further validation.
Stop the active batch before rebuilding KLEE; its executable is shared by the
verification workers. The run retains the starting executable hash.
Check the last completed index in `progress.tsv` before using `START_INDEX`;
use the same unchanged input directory and ordering. A restart creates fresh
outputs and never combines stale partial pairs with an active pair.

For a persistent launch:

```bash
tmux new-session -d -s ruklee-all-crates -c "$PWD" \
  './run-all-crates .local/crates > .local/run-all-crates-launch.log 2>&1'
```

Terminate the runner PID with SIGTERM to stop its active crate and matching
container. Containers are selected only by this run's unique result path.

The startup regression also exposed and fixed a KLEE result-scope bug: an
optimistic private-state probe's SAT could override a subsequent composed-state
UNSAT. KLEE now marks the final query phase explicitly and the frontend reports
only that phase once reached. The unchanged step2 `core:rule-657` regression is
retained in `.local/runner-final-query-regression/`: two constrained safe cases
are verified and the two genuine counterexamples remain violations.
