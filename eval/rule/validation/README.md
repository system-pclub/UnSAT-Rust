# Bounded validation before the full run

The final compact prompts were exercised on three real safety rules before the
full std/core/alloc Batch submission:

| Rule | Transport | Extraction / translation review |
| --- | --- | --- |
| `HashMap::get_disjoint_unchecked_mut` | direct LLM | Preserved the no-overlap condition even when returned references are unused; `UNEXPRESSIBLE` in the supplied source/memory vocabulary. |
| `GlobalAlloc` allocation occurrence obligation | direct LLM | Preserved the prohibition on relying on removable heap allocations; `UNEXPRESSIBLE` for this execution/optimization contract. |
| `AsciiChar::from_u8_unchecked` | OpenAI Batch | Extracted `b in 0..=127`; translated to `0 <= $1 <= 127`; generated grammar and type checks passed. |

Artifacts retain source documents, exact prompts, raw replies, mode and Batch job
IDs. The `alloc::alloc` API in the direct smoke fixture yielded no explicit rule:
its Safety text only links to `GlobalAlloc::alloc`, without stating its contract.
This is a supplied-document scope limitation, not evidence that it has no safety
obligations. No precision/recall score is inferred from this bounded sample.

Local Docker tests covered pending-job resume, reordered results, partial batch
failure, failed-rule-only repair, bounded retries, extended-budget resume,
const-generic brace preservation, and the global round barrier. The shared DSL
checks and dslgen tests also passed. Grammar/type acceptance does not establish
semantic fidelity or KLEE execution. The global barrier was added after the live
sample, then checked locally; it does not change either model request body.

Cache paths in copied manifests retain their original `/unsat-rust/.local/...`
locations. These are evidence snapshots; the full run's active caches are under
`eval/rule/{std,core,alloc}/`.
