# Repository instructions

- Keep `benchmark/<benchmark-name>/` source-only: its immediate children must be crate directories containing `Cargo.toml`. Store scripts, metadata, documentation and outputs elsewhere.
- Before building, testing, validating, running RuKLEE or changing its execution environment, read [ruklee-environment](.agents/skills/ruklee-environment/SKILL.md).
- For RustSec corpus, versions, callsites and evidence maintenance, read [rustsec-benchmark](.agents/skills/rustsec-benchmark/SKILL.md).
- For diagnosis and patches to DSL, KLEE, MIRScan or PoC behavior, read [ruklee-fix](.agents/skills/ruklee-fix/SKILL.md).
- For `eval/rule` generation, stage commands, caches and documentation-to-DSL evaluation, read [dsl-eval](.agents/skills/dsl-eval/SKILL.md).
