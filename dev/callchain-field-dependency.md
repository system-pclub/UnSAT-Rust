# Field dependency call-chain reproduction

Source: `benchmark/typerules/callchain-field-dependency/`.
The supplied Stack example is unchanged apart from formatting and `mod retain`.
`src/retain.rs` uses `#[used]` function pointers to emit candidate method bodies;
it does not call methods or prescribe their order.

Run after the image build required by `.agents/skills/ruklee-environment/SKILL.md`:

```sh
./x verify benchmark/typerules/callchain-field-dependency \
  --callsite src-main-rs-40-22 --rule rule-958 --skip-llm-testcase \
  --ir-output-dir .local/callchain-stack-ir
```

Rule 958 is the existing copy_nonoverlapping non-overlap contract. KLEE identifies
both private branch-control fields f1 and f2 as uncertain at the target callsite.

## Result

The search now keeps a mutator whose post-state target field is derived from other
receiver fields. `verifySnapshotFieldSymbol` reports those source fields by matching
the post-state reads against the receiver's pre-state bytes. The queue in
`tools/klee/lib/Snapshot/StateOrchestrator.cpp` (`resolveFieldDependency`) then
searches backward for a constructor or mutator that makes each source controllable,
recursively and with a depth bound. The calls are committed in execution order
(`commitDependencySteps`), restoring composition state on failure; a constructor is
always composed first and only once per receiver.

Evidence (`.local/verify/callchain-field-dependency/src-main-rs-40-22/rule-958/klee-compose/info`):

- f1 <- new, f1 <- fun1: rejected. f3 has no controllable definition (`new` only
  stores a constant).
- f1 <- fun2 derives f1 from f4; f4 <- new is a constant, f4 <- fun3 is controllable.
  Chain accepted: `Stack::fun3, Stack::fun2` (`field-dependency:Stack::fun2<-f4`).
- f2 <- new is controllable, so `new` is composed first: final path
  `new -> fun3 -> fun2 -> dup` (snapshotsComposed=3).
- Final result: `DSL status=violation-sat`.

## Multi-field search

`benchmark/typerules/callchain-multi-field/` holds nine callers (padded to 20 bytes
so constructors return through memory); `dev/callchain-multi-field/run.sh` diffs the
verdict per caller against `expected.txt`:

| caller | scenario | verdict |
| --- | --- | --- |
| TwoSetters | two guarded fields, one setter each | violation |
| Nested | f1 <- f4 <- f5 <- input (depth 2) | violation |
| SharedSource | f1 and f2 both derived from f4 | violation |
| PartialSource | f1 = f3 + f4, only f3 has a setter, f4 is a constant | violation |
| ConstOk / ConstBad | constructor stores 5 / 100 in the guarded field | violation / none |
| DerivedConst / DerivedConstBad | f1 = f4, constructor stores 5 / 100 in f4 | violation / none |
| Cycle | f1 = f4, f4 = f1, no input | none (terminates) |

Search order for a field: controllable definitions first; if none, constant
definitions are tried one by one (`tryDependencyCandidate`). Each complete call
sequence is checked with the callsite's path constraints (`pathPhiFeasible`): the
receiver fields built by the composed sequence replace the entry receiver fields
in the ending snapshot's constraints, and an unsatisfiable result drops the
sequence, so the next definition is tried. This is the `Phi = false, drop` step.
Backward resolution of source fields is recursive (depth <= 4, cycle-safe).

Limitations:

- Alternatives are backtracked only for the target field's own definitions. Within
  a derived definition each source field takes its first resolution.
- Constructors that return a small ADT in registers (up to 16 bytes, e.g. two
  `i32` fields) are not supported: the final receiver binding treats the returned
  value as a pointer (`writeArgumentCell`) and enabling field evidence for them
  produced a false violation and a KLEE crash for `&mut self` receivers. They stay
  unresolved, as before; the test structs are padded for this reason.
- Dependency search applies to the final field of an access path.

## Expected backward-search result

Continuing the supplied S6 with distinct input symbols a and b:

1. f4@fun2 <- fun3(a): substitute a for f4@fun2; Q=[f2@dup],
   Phi=(a<=10 && f2@dup<=10), partial chains [fun3,fun2,dup] and [dup].
2. f2@dup <- new(b): substitute b; Q=[], Phi=(a<=10 && b<=10).
3. Merge the partial chains on the same receiver, using a single constructor:
   new(b) -> fun3(a) -> fun2() -> dup(n).

The f4@fun2 <- new branch substitutes 100 and is infeasible. The f3 branch is
likewise infeasible. A concrete expected witness is b=0, a=0, n=1: before dup,
f1=f2=0, so execution reaches copy_nonoverlapping with identical source and
destination and a positive byte count. This witness follows from the source;
the current search did not discover or replay it.

An implementation of the proposed search must retain the candidate snapshot,
substitute its field expressions into Phi, enqueue remaining uncertain pre-state
fields at the front of Q, prune unsatisfiable states, and merge partial sequences
with receiver identity, shared constructor and call order preserved. A concrete
constant must be checked against Phi; absence of a controllable symbol alone is
not a reason to discard a satisfiable constant definition.
