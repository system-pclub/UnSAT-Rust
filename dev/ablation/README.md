# ruKLEE ablation profiles

KLEE accepts `--ruklee-ablation=a|b|c|full` together with
`--compose-verify`. The default is `full`, preserving the existing behavior.
The verification frontend accepts the same values with `--ablation-level`.

| Profile | Functions/access paths considered by the callchain algorithm |
|---|---|
| `a` | Buggy function and public fields |
| `b` | A plus constructors |
| `c` | B plus functions returning mutable references, including supported escapes through wrappers |
| `full` | C plus symbolic execution and state composition of other candidate functions |

**Full is C + symbolic**, not a fifth profile. Symbolic means the callchain
algorithm may execute other report-backed candidates and check their symbolic
field/state effects; the ordinary setter in the fixture exercises this path.
All four profiles share the same underlying symbolic inputs, execution engine,
trait-method summaries, allocation models, DSL evaluation and loop bounds.
These switches restrict the candidate functions used to recover safe calling
contexts. Calls made by the buggy function itself still execute normally.
Public-field traversal remains available in every profile.

The existing `--disable-mut-ref-recovery` override remains supported and disables
that feature even with `full`. Logs print all effective switches. Control-chain
JSON records `ablation_level` and effective `mut_ref_recovery`; matrix JSON and
per-pair initialization stages record the selected level. Resuming a matrix from
a different profile, mutable-reference override or older profile definition
fails instead of reusing its rows. `ablation_profile_version: 2` identifies the
corrected four-profile definition; earlier experimental five-profile results
cannot be resumed. Use separate result and artifact directories for comparisons.

For example, after the normal Docker image refresh required by the environment
skill:

```bash
./x verify benchmark/ablation/context-levels \
  --rule-dsl dev/ablation/rules.json --studied-rules dev/ablation/studied_rules \
  --ablation-level b --skip-llm-testcase --compose-loop-bound 1 \
  --results-json .local/ablation/b/matrix.json \
  --artifacts-dir .local/ablation/b/artifacts
```

Run each profile serially for this crate; verification regenerates the source
report, injected crate and IR. `--ablation-level` applies to the initial compose
analysis. Concrete PoC replay and `compose-rerun-sym` keep their existing separate
controls, so the replay does not inherit a different execution semantics.

`benchmark/ablation/context-levels/{Cargo.toml,src/main.rs}` provides independent
public-field, constructor, mutable-reference getter and ordinary-setter probes.
The constructor probe uses a memory-returned aggregate; these tests do not
establish coverage of every constructor ABI. A safe Rust test root retains all
context methods in the emitted test IR for every profile.
Its DSL is the actual `ptr::read` readable precondition, revalidated by
`prepare.py` with the existing Python grammar/type checker. The four probes
exercise each successive callchain capability without changing execution models.
The expected actionable counterexample counts for A/B/C/full are 1/2/3/4; other
probes remain uncertain SAT outcomes rather than being counted as detected bugs.

Before each container run, build the image as directed by
`.agents/skills/ruklee-environment/SKILL.md`. Run `prepare.py` and
`check_results.py` inside that image, mounted at `/unsat-rust`. Results are
retained under `.local/ablation/<profile>/`; the check writes
`.local/ablation/summary.json`. Evaluation errors, timeouts and uncertain SAT
outcomes remain distinct from verified rules and actionable counterexamples.
