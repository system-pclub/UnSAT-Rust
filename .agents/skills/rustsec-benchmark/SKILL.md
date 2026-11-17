---
name: rustsec-benchmark
description: Maintain benchmark/rustsec crate snapshots, its exact allowlist and advisory evidence in dev/rustsec, selected callsites, and Rust 1.87.0 requirements.
---

# Maintain the RustSec benchmark

Read `dev/rustsec/allowlist.json` before changing the corpus. It is the authoritative list: **38 advisory records, 37 distinct crate versions**. `toodee-0.5.0` covers two advisories. Keep exact versions; do not expand the list without a maintainer instruction.

The lexical entry is **lexical-util 0.8.5**, as confirmed by the maintainer and the [original Bytes::read report](https://github.com/Alexhuszagh/rust-lexical/issues/102). Its RustSec advisory package is lexical-core; preserve that distinction in provenance.

## Source-only layout

`benchmark/<benchmark-name>/` must contain **only crate directories**, each containing its `Cargo.toml`. For RustSec this is `benchmark/rustsec/<crate>-<version>/`. Do not put runner scripts, tests, README files, allowlists, evidence directories, target lists, toolchain files, spreadsheets, logs or caches beside the crate directories.

Keep RustSec's runner (`run.py`), scheduling tests (`test_run.py`), allowlist, manifests, cases, toolchain pin, selection evidence, `targets/` and `evidence/` under **`dev/rustsec/`**. Generated experiment outputs belong in `eval/` or `.local/`. Leave vendored crate internals intact.

- Compare every immediate corpus entry against the allowlist, including package names and versions in Cargo.toml. Remove retired versions and obsolete supporting records; preserve unrelated local edits before cleanup.
- Fetch a missing exact version from the repository root with `python3 tools/crates.py download-one <crate> <version> --output-dir benchmark/rustsec`. If the download fails, report the missing pin; do not fabricate a directory or substitute another version or dependency.
- Keep `dev/rustsec/manifest.json`, `cases.json`, `targets/`, and `evidence/` consistent with all advisory tuples. Preserve checksums, archives and provenance only for their actual package versions.
- Preserve both toodee selections: insert_row and DrainCol's nested DropGuard. For lexical-util, Bytes::read reaches ptr::read_unaligned; select the actual unsafe callee rather than treating the table's shorthand as a direct call.
- Validate callsite IDs and required Cargo features against fresh MIRScan reports. Keep caller/callee locations and source hashes in `dev/rustsec/selection-evidence.json`. A successful compile without a fresh report is insufficient; the current extractor assumes the crate's default target directory.
- Match rule metadata and semantics to the selected unsafe API. Replace generic pending labels with validated selections or a precise limitation. Compilation, dry-run and not-in-ir results are not vulnerability detections.

## Rust and validation

Use **Rust 1.87.0** for analysis and replay. Keep `dev/rustsec/rust-toolchain.toml` pinned to 1.87.0 and copy it into isolated work copies. Check that the actual patched compiler reports 1.87.0 (a custom suffix is acceptable). Do not upgrade Rust to hide dependency failures; retain compatible dependency locks under dev/rustsec/evidence and apply them only to work copies.

Follow the [environment skill](../ruklee-environment/SKILL.md) for Docker, build-first execution, entrypoint review and disk cleanup. Follow the [fix skill](../ruklee-fix/SKILL.md) when patching the verifier or DSL.

After the required image build, run the reusable checker in Docker:

```bash
docker run --rm -e PYTHONDONTWRITEBYTECODE=1 -v "$PWD:/unsat-rust" -w /unsat-rust unsat-rust:local \
  python3 .agents/skills/rustsec-benchmark/scripts/check_corpus.py --check-selections \
  --rustc tools/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
```

The checker validates source-only benchmark layout, package identities, advisory tuples, provenance, source hashes, selected target/rule references and the actual compiler release. Also run `python3 dev/rustsec/run.py --dry-run` and `python3 -m unittest dev.rustsec.test_run` in Docker. They must not launch KLEE or contact an LLM. The batch runner invokes `./x verify` for real runs from the host.
