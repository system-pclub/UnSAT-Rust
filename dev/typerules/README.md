# Existing-DSL KLEE regressions

`bit-reverse-initialization` exercises `reverse_bits` through the normal full,
skip-LLM pipeline, with initialized/missing selected halves at 16/32/64/128
bits, complete/missing u8 values, and 32/64-bit bit-order controls. Generate the
unchanged rules using `bit_reverse_initialization.py` in Docker, then run
`./x verify benchmark/typerules/bit-reverse-initialization --rule-dsl
dev/typerules/bit-reverse-initialization-rules.json --studied-rules
dev/typerules/bit-reverse-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/bit-reverse-initialization-ir
--results-json <evidence>/fixed-results/matrix.json`. Build the root image before
every container invocation. Preserve all baseline reports and replay inputs
before reusing IR. `check_bit_reverse_initialization.py <evidence>` checks the
eight-case unsupported-intrinsic baseline and expanded twelve-case fixed run,
actual bitreverse IR, generated constraints, exact DSL/text/IDs and absent LLM
stages. Partial values and null NonNull are verifier inputs only, never native
executions. Fixed-vector bit reversal, nested constants, snapshot-root
initialization and allocation contracts/provenance remain unestablished.

`byte-swap-initialization` checks byte-initialization after scalar `swap_bytes`
for u16/u32/u64/u128 selected halves and a corresponding missing-byte control.
Validate the unchanged rules with `byte_swap_initialization.py` in Docker, then
run `./x verify benchmark/typerules/byte-swap-initialization --rule-dsl
dev/typerules/byte-swap-initialization-rules.json --studied-rules
dev/typerules/byte-swap-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/byte-swap-initialization-ir
--results-json <evidence>/fixed-results/matrix.json`. Build the root image before
each container invocation. Retain baseline reports and replay inputs before
reusing IR. `check_byte_swap_initialization.py <evidence>` checks the original
eight cases and expanded ten-case fixed run, actual bswap IR, constraints,
exact DSL/text/IDs and absent LLM stages. Value controls check 32-bit byte order
and a deliberate 64-bit null NonNull. Partial values/null construction are
verifier inputs only. Fixed-vector byte swaps, nested constants, snapshot-root
initialization and allocation contracts/provenance remain unestablished.

`bitwise-initialization` checks initialized low subwords through scalar And/Or/Xor
and truncation, including a symbolic left operand with a partial RHS. Validate
the unchanged synthetic observation using `bitwise_initialization.py` in Docker,
then run normal `./x verify benchmark/typerules/bitwise-initialization --rule-dsl
dev/typerules/bitwise-initialization-rules.json --studied-rules
dev/typerules/bitwise-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/bitwise-initialization-ir
--results-json <evidence>/fixed-results/matrix.json`. Build the root image before
each container invocation. Preserve baseline replay inputs before reusing IR.
`check_bitwise_initialization.py <evidence>` checks baseline/fixed verdicts, actual
bitwise IR, generated constraints, unchanged expression/sentence/ID and skipped
LLM stages. Partial values are verifier inputs only, never native executions.
Value-dependent masking, nested constant shadows, snapshot-root initialization
and allocation provenance/contracts remain incomplete.

`shift-widths` extends scalar logical-right-shift initialization coverage to
64-bit values, 8-bit fills and bounded symbolic amounts. Validate the existing
observation with `shift_initialization.py` in Docker, then use normal `./x verify
benchmark/typerules/shift-widths --rule-dsl dev/typerules/shift-initialization-rules.json
--studied-rules dev/typerules/shift-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/shift-widths-maintenance-ir
--results-json <evidence>/results/matrix.json`. Build the root image before each
container invocation. `check_shift_widths.py <evidence>` checks actual shift IR,
constraints, exact DSL/text/ID and absent LLM stages. Expected outcomes: three
verified, two low-confidence SAT and one synthetic observation violation. None
is a confirmed vulnerability. Keep partial values as verifier inputs only.
These cases do not establish arithmetic/left shifts at other widths, missing
amounts, allocation contracts/provenance or general initialization transfers.

`vector-shifts` exercises selected initialized bits and logical/arithmetic fill
in LLVM fixed-vector Shl/LShr/AShr. Generate the existing observation with
`vector_shifts.py` inside Docker, then use normal `./x verify
benchmark/typerules/vector-shifts --rule-dsl dev/typerules/vector-shifts-rules.json
--studied-rules dev/typerules/vector-shifts.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/vector-shifts-ir
--results-json <evidence>/fixed-results/matrix.json`. Build the root image before
each container invocation. Preserve baseline IR, injected source and all reports
before reusing the IR directory. `check_vector_shifts.py <evidence>` checks the
six-case baseline and expanded eight-case regression, actual vector shift IR,
generated constraints, unchanged DSL/original sentence/rule ID and skipped LLM
stages. Three false-positive SATs are corrected; final outcomes are four verified
and four unconfirmed SAT, including second-lane isolation. Partial SIMD values
are verifier inputs, never native executions. Symbolic/uninitialized shift amounts,
other lane widths, allocation provenance/contracts and general initialization
remain unestablished by this fixture.

`vector-select` reproduces the SIMD blend vector-predicate SelectExpr assertion.
Generate its existing initialization and NonNull rules using `vector_select.py`
inside Docker, then run normal `./x verify benchmark/typerules/vector-select
--rule-dsl dev/typerules/vector-select-rules.json --studied-rules
dev/typerules/vector-select.studied --ablation-level full --skip-llm-testcase
--timeout-sec 60`. Preserve baseline and fixed reports as
`<evidence>/{baseline,fixed}-results/matrix.json`, then run
`check_vector_select.py <evidence>` inside Docker. The six baseline pairs abort;
the expanded fixture checks per-lane initialization and both selection arms with
different predicate signs in adjacent lanes. Deliberately incomplete vectors
are verifier inputs, never native executions. Blend exercises signed-less-than,
sext, lshr, trunc and vector-condition select. Other implemented predicates,
shifts, zext and scalar-condition vector select lack end-to-end coverage here;
general vector arithmetic remains incomplete. Build the root image before every
container invocation and archive IR/injection/reports before another default-IR run.

`vector-bitwise` checks initialization of independent LLVM And/Or/Xor vector
lanes, including an uninitialized right-hand operand. Generate its unchanged
synthetic ptr::read observation with `vector_bitwise.py` in Docker, then run
`./x verify benchmark/typerules/vector-bitwise --rule-dsl
dev/typerules/vector-bitwise-rules.json --studied-rules
dev/typerules/vector-bitwise.studied --ablation-level full --skip-llm-testcase
--timeout-sec 60 --results-json <evidence>/fixed-results/matrix.json`.
Build the root image before each container run. Preserve earlier replay inputs
before the default IR directory is reused. `check_vector_bitwise.py <evidence>`
checks the retained five-case baseline and expanded seven-case fixed run, exact
DSL/original sentence, actual vector instructions, generated constraints and
skipped LLM stages. Deliberate partial-vector inputs must never run natively.
These bitwise cases do not establish general vector arithmetic semantics.

The source fixtures are `benchmark/typerules/<case>/Cargo.toml` and `src/main.rs`.
Their manifests point the library target at `main.rs` because MIRScan selects
library targets. Unsafe negative functions are verification inputs; `main` calls
the satisfying examples. Do not execute the negative functions natively.

Read the repository's `ruklee-environment` and `ruklee-fix` skills first. Before
**each** container invocation below, refresh the image with its normal cache:

```bash
docker build --build-arg USER_ID=$(id -u) --build-arg GROUP_ID=$(id -g) -t unsat-rust:local .
```

After editing KLEE, run `dev/typerules/build-klee.sh` for the existing incremental
build. Finish that build before starting verification: linking replaces the
executable that the verification workers launch.

Generate and validate the DSL metadata with the existing Python syntax/type
checker; this does not extend the grammar or memory schema:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  sh -c '/opt/x/.venv/bin/python dev/typerules/build_rules.py && /opt/x/.venv/bin/python dev/typerules/combinations.py && /opt/x/.venv/bin/python dev/typerules/classify_contracts.py'
```

`cases.json` retains the source-derived API rule and generated DSL provenance.
Additional observations are explicitly labelled: byte initialization is separate
from typed validity; returned-str validity is separate from an input byte slice's
validity. `combination-rules.json` contains synthetic evaluator observations at
the real `std::ptr::write` call, with its concrete tuple argument type. These
observations are not presented as that API's safety contract.

For each distinct `case` in `cases.json`, build the image and run:

```bash
dev/typerules/matrix.sh <case> --timeout-sec 60 --ir-output-dir .local/<case>-ir > .local/typerules-<case>.actual
```

Build the image again and run the combination fixtures:

```bash
./x verify benchmark/typerules/dsl-combinations \
  --rule-dsl dev/typerules/combination-rules.json \
  --studied-rules dev/typerules/combinations.studied \
  --skip-llm-testcase --compose-loop-bound 1 --timeout-sec 60 \
  --ir-output-dir .local/combinations-ir
```

Parallel verifications need distinct IR directories because verification cleans
its selected IR directory. Keep each crate's verification output independent.

Build the image again for the Python regressions:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -e PYTHONPATH=/unsat-rust/tools/x/src \
  -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  /opt/x/.venv/bin/python -m unittest discover -s tools/x/tests > .local/typerules-python-tests.log 2>&1
```

After all runs finish, build the image again and run `check_results.py` inside it.
It checks the full result matrices against `expected.txt` and the source
expectations for each synthetic observation, verifies that timed-out division
queries actually produced constraints, and writes
`.local/typerules-validation-summary.json` and `.local/typerules-all.actual`.
An expected `eval-error` or timeout remains **unproved**. Pointer-receiver
uncertainty is reported as `low-confidence-sat`, according to the existing
compose policy. None of these outcomes is counted as verified.

The report `contract-only.json` classifies the inventory's 80 trait obligations;
it makes no DSL translation or SAT/UNSAT claim. Missing catalog matches stay
explicitly empty. The remaining backend work and the maintainer's preserve-DSL
decision are recorded in the root `TODO`. Allocation generation/provenance,
bodyless targets, general DSTs and unbounded quantification are not established
by this regression suite.

The hourly-maintenance additions have their own targeted check:
`dev/typerules/check_memory_maintenance.py`. `scalar-initialization` uses the
existing Python-validated `rules.json` ptr::read observations to check ordinary
loads/stores, calls/returns, conditional initialization, partial-word truncation,
aggregate padding and invalid indirect calls. Deliberate invalid reads/calls are
verification inputs only. Raw-pointer sentinels that injection cannot locate stay
`not-in-ir`; they are not counted as proofs.

Run `dst_layout.py` inside the image to validate and generate
`dst-layout-rules.json` and `dst-layout.studied`, then use normal `./x verify
benchmark/typerules/dst-layout --rule-dsl dev/typerules/dst-layout-rules.json
--studied-rules dev/typerules/dst-layout.studied --skip-llm-testcase
--ablation-level full --timeout-sec 60 --ir-output-dir .local/dst-layout-ir`.
The synthetic observations check the actual CStr return layout and its `inner`
field. Three-byte data satisfies the size-three assertion; four-byte data reports
SAT. This does not assert that the latter violates CStr's API contract. The
separate `dst-validity` fixture below checks bounded trailing-DST typed validity;
vtable-backed layouts and general zero-sized tail-element metadata remain unsupported.

Rerun `matrix.sh scalar-initialization` and `matrix.sh memory-transfers`, using
distinct IR directories if concurrent, then run the targeted checker in Docker.
Build the image before every container run. Preserve earlier matrices and logs
before rerunning because `matrix.sh` replaces a case's output directory.

`dst-validity` checks length-metadata aggregate DSTs, including sized prefixes,
empty bool tails and nested DST tails. Invalid bool bytes and missing prefix
initialization are deliberate verification inputs. Typed validity permits padding;
the separate byte-initialization observation detects it. Generate its synthetic
ptr::write observations using `dst_validity.py` inside Docker, then build the
image again and run `./x verify benchmark/typerules/dst-validity --rule-dsl
dev/typerules/dst-validity-rules.json --studied-rules
dev/typerules/dst-validity.studied --ablation-level full --skip-llm-testcase
--timeout-sec 60`. Run `matrix.sh memory-state` afterward, preserving the DST
IR/injected sources and any previous memory-state output first. Also rerun the
existing `dst-layout` command above to check CStr field projections. Build the
image before each container invocation. `check_dst_validity.py
<evidence-directory>` checks all three matrices against reviewed outcomes and
the retained baseline.
These observations do not establish allocation provenance or library invariants.
Vtable-backed DSTs and general zero-sized tail-element metadata remain unsupported;
symbolic tails still require the existing proven quantifier bound. The existing
grammar also excludes arbitrary field projections on opaque generic T; do not
extend it to make these synthetic tests pass.

After verification, move generated source `target/` and `report.json` to a
retained `.local/typerules-source-artifacts.<run>/` directory. Preserve compiler,
injection and incremental-build caches; keep `benchmark/typerules` source-only.


`dst-zero-tail` exercises bounded DST tails whose elements are complete empty
structures, including `repr(align(16))`. Generate its five existing-DSL
observations with `dst_zero_tail.py` inside Docker, then use `./x verify
benchmark/typerules/dst-zero-tail --rule-dsl dev/typerules/dst-zero-tail-rules.json
--studied-rules dev/typerules/dst-zero-tail.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60`. Size remains 16 for both empty and nonempty
tails, and typed validity still rejects invalid/uninitialized prefix fields.
Byte initialization separately detects padding. Other zero-sized element types
and absent metadata remain unsupported; empty structure recognition uses DI,
without type-name branches. `check_dst_zero_tail.py <evidence-directory>` checks
the retained baseline, generated constraints, skipped LLM stage and the existing
`dst-validity`/`dst-layout` regressions.

`symbolic-allocation` reproduces a crash after a suppressed allocation model
failure. Generate `symbolic-allocation-rules.json` with `symbolic_allocation.py`
in Docker and verify this fixture using that catalog and its `.studied` file,
full mode and `--skip-llm-testcase`. The unchanged byte-initialization expression
is checked alongside an explicit allocation alignment observation. Two possible
allocation sizes retain the requested alignment; initialized and uninitialized
reads remain distinct. More than two possible bounded sizes still require a
backend model and must report evaluation error, including across the final-query
boundary. Terminating that unsupported branch is not a proof of its safety.
Build the root image before every container run and retain replay inputs before
another default-IR verification, as above.

`allocation_contract.py` generates a syntax/type-validated observation of
`allocation.minimum_size <= allocation.size` for the same `symbolic-allocation`
source fixture. Use `allocation-contract-rules.json` and
`allocation-contract.studied` with normal full/no-LLM verification. The expected
outcome is an explicit evaluation error: live allocation capacity does not
establish the allocation request's minimum fitting-layout size. This regression
must stay unproved until request metadata is retained by the backend.

`vector-initialization` isolates the existing byte-initialization observation at
`ptr::read` after SIMD extraction, insertion and shuffling. Generate its catalog
with `vector_initialization.py` inside Docker, then run normal `./x verify
benchmark/typerules/vector-initialization --rule-dsl
dev/typerules/vector-initialization-rules.json --studied-rules
dev/typerules/vector-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/vector-initialization-ir`.
Build the root image before every container invocation. Deliberately partially
initialized SIMD inputs exercise the verifier shadow and must not be run natively.
The public x86 helpers avoid the frontend's direct-intrinsic constant-argument
injection limitation. Keep baseline results under `<evidence>/baseline-results`
and run `check_vector_initialization.py <evidence>` in Docker after verification.
It checks generated constraints, actual LLVM lane instructions, skipped LLM
stages and all seven expected outcomes: four verified, three unconfirmed SAT.
Poison shuffle-mask branches and symbolic lane indices are not exercised by this
fixture; vector arithmetic and nested-undef constants remain incomplete.


`intrinsic-initialization` exercises byte-initialization after LLVM fabs and
funnel shifts (Rust abs/rotate_left/rotate_right), including rotation of a
partially initialized word followed by truncation. Population count already
lowers to ordinary instructions and is retained as a regression. Generate the
existing observation with `intrinsic_initialization.py` inside Docker, then use
`./x verify benchmark/typerules/intrinsic-initialization --rule-dsl
dev/typerules/intrinsic-initialization-rules.json --studied-rules
dev/typerules/intrinsic-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/intrinsic-initialization-ir`.
Build the root image before every container invocation. Preserve baseline
results under `<evidence>/baseline-results` before rerunning; then run
`check_intrinsic_initialization.py <evidence>` inside Docker. Expected final
outcomes are four verified and four unconfirmed low-confidence SAT. Deliberate
uninitialized values are verifier fixtures, never native executions. Symbolic
shift amounts and missing initialization in the shift amount are not exercised
by this source fixture. Other intrinsic/opaque summaries remain incomplete.

`snapshot-layout` reproduces an out-of-bounds snapshot write when an Arc<str>
static prefix is used as storage for its unsized field. Generate its unchanged
NonNull rule with `snapshot_layout.py` inside Docker, then run `./x verify
benchmark/typerules/snapshot-layout --rule-dsl dev/typerules/snapshot-layout-rules.json
--studied-rules dev/typerules/snapshot-layout.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60`. Preserve baseline reports as
`<evidence>/layout-baseline-results` before rerunning. The expected final outcomes
are two verified, one unconfirmed SAT and one explicit layout evaluation error.
`check_snapshot_layout.py <evidence>` checks constraints, the retained crash and
skipped LLM stages, together with the `constant-initialization` audit. The latter
does not establish nested poison/undef constant propagation. Dynamic snapshot
storage and extern-return shaping remain incomplete; Option<Arc<str>> does not
exercise inner Arc shaping under the current niche model. Build the root image
before every container invocation and preserve replay inputs and source artifacts.

`vector-arithmetic` exercises fixed-vector integer Add/Sub/Mul lane values and
initialization. Generate its two unchanged observations with
`vector_arithmetic.py` inside Docker, then use normal `./x verify
benchmark/typerules/vector-arithmetic --rule-dsl
dev/typerules/vector-arithmetic-rules.json --studied-rules
dev/typerules/vector-arithmetic.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --ir-output-dir .local/vector-arithmetic-ir`.
Build the root image before every container invocation. Retain baseline/fixed
outputs in distinct artifact directories with matrix.json, then run
`check_vector_arithmetic.py <evidence-directory>` in Docker. The old backend
uses packed arithmetic, causing three initialized-lane false-positive SATs and
three incorrect non-null proofs from cross-lane carry, borrow and multiplication.
The fixture checks wrapping i32 SIMD operations; it does not establish vector
division, floating arithmetic or LLVM overflow flags. Partially initialized
vectors and null NonNull construction are verifier inputs only, never native
executions. Preserve IR, injection, reports and generated compiler IR before
rerunning; keep generated source target/report artifacts outside benchmark/.

`snapshot-unit` checks zero-sized debug basic layouts nested in
UnsafeCell, fixed arrays and Arc, alongside sized, null and unsized controls.
Generate its unchanged rule-504 with `snapshot_unit.py` in Docker, then use
normal `./x verify benchmark/typerules/snapshot-unit --rule-dsl
dev/typerules/snapshot-unit-rules.json --studied-rules
dev/typerules/snapshot-unit.studied --ablation-level full --skip-llm-testcase
--timeout-sec 60 --ir-output-dir .local/snapshot-unit-ir --results-json
<evidence>/fixed-results/matrix.json`. Preserve baseline reports under
`<evidence>/baseline-results/matrix.json` and archive replay inputs before
reusing IR. Run `check_snapshot_unit.py <evidence>` inside Docker to check all
six outcomes, exact rule text, generated constraints and absent LLM stages.
Build the root image before each container run. Arc<str> remains an explicit
layout error; the deliberate null case is a candidate SAT, not confirmation.

`shift-initialization` exercises scalar Shl/LShr/AShr bit-initialization shadows
through selected partial words and logical/arithmetic fill bits. Generate the
unchanged synthetic byte-initialization observation with `shift_initialization.py`
inside Docker, then run normal `./x verify benchmark/typerules/shift-initialization
--rule-dsl dev/typerules/shift-initialization-rules.json --studied-rules
dev/typerules/shift-initialization.studied --ablation-level full
--skip-llm-testcase --timeout-sec 60 --results-json
<evidence>/fixed-results/matrix.json`. Retain the original six-case baseline in
`<evidence>/baseline-results/matrix.json`, and run
`check_shift_initialization.py <evidence>` in Docker. It checks exact rule ID,
DSL, observation sentence, actual shift instructions, constraints and skipped
LLM stages: three incorrect baseline SATs become verified; the expanded fixture
has five verified and three unconfirmed SAT outcomes. Symbolic/uninitialized
shift amounts and vector shifts are not established by this source regression.
Partial-value functions are verifier inputs only; never execute them natively.
Build the root image before every container invocation and preserve all replay
inputs before reusing default IR. Keep generated source artifacts outside benchmark/.
