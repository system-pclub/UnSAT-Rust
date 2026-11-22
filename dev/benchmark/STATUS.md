# TASK completed — 2026-11-22T03:50:09.163353+00:00

- Three-crate statistics: [crates.md](crates.md), [crates.tsv](crates.tsv), with saved upstream responses and counting definitions.
- 32 requested published versions are present in all four corpora. Original snapshots match their archives; checksums and tool-copy differences are recorded in [sources.json](sources.json).
- RuKLEE, CULPA and Safe4U library compilation: **32/32 each, 96/96 total**, default features. Selected commands, actual compiler versions and evidence paths are in [compilation-matrix.json](compilation-matrix.json).
- Five Claude runner contract tests passed inside the root Docker image. Final source-only layout and original archive checks passed after compatibility repairs.
- Whole-crate Claude now covers all **32/32** requested crates: the original three runs are retained and the other 29 were added with the same task configuration. Total: **90 unreviewed candidate violations**, zero errors/timeouts. All runs report claude-sonnet-5-5. See [claude-results.md](claude-results.md).
- Whole-crate mode copies core/std/alloc into the image, adds /libraries to Claude's readable directories, and saves structured findings under eval/claude-crate/<crate>/<attempt>.
- All new evaluation outputs follow eval/<tool>/<crate>/<attempt>; the environment skill documents this policy. Historical referenced experiments retain their paths.

[Compatibility notes](compatibility-notes.md) document the new patches and dependency locks. Failed attempts are retained as evidence; the matrix selects the final successful check per crate/tool.

Host Docker encountered filesystem writeback stalls. Recovery used dedicated Docker and containerd daemons with tmpfs storage and preserved original images, build caches and patched toolchain outputs. Stopped containers auto-started by a service restart were stopped again, restoring the empty running-container baseline. See [execution-environment.json](execution-environment.json) and [images.json](images.json) for runtime details.

benchmark/rustsec remains because active allowlists and historical replay/Claude tooling still reference its snapshots; deletion was conditional in TASK. Extra pre-existing fast_id_map snapshots are retained.
