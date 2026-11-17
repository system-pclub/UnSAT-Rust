---
name: ruklee-fix
description: Diagnose and patch RuKLEE, DSL rules, KLEE, MIRScan, or PoC slicing and replay failures. Use when investigating incorrect verification behavior or implementing fixes.
---

# Diagnose and fix RuKLEE

Read the [environment skill](../ruklee-environment/SKILL.md) before reproducing or testing a failure. Use the affected benchmark's maintenance skill when changing its corpus or configuration.

Locate the failing layer with its actual inputs and retained reports: the selected source version and callsite, extracted caller/callee and call chain, rule metadata and DSL semantics, generated IR, KLEE output, or PoC replay. A compile failure, missing callsite, `not-in-ir`, a satisfiable DSL and a reproduced vulnerability are different outcomes; report the evidence for the outcome reached.

- If a DSL rule is wrong, fix the DSL. Match the contract to the actual unsafe API; do not substitute an unrelated rule merely to obtain a result or suppress a configuration error.
- If KLEE lacks required functionality or has a bug, fix KLEE, recompile the affected component and rerun the reproduction.
- If extraction or slicing drops a function used in the call chain, fix that omission generally. Keep the original failure as a regression case where it meaningfully exercises the fix.
- Fix the responsible layer generally. Avoid crate-name branches, benchmark-specific symbolic assumptions, and changes that mask a failure in one example.
- The PoC prompt already has the information it needs. Do not add new categories of prompt information without checking with the maintainer first. Fixing existing bugs, such as an omitted function used in a call chain, is allowed.

Keep vendored vulnerable sources intact. Put compatibility adjustments and generic instantiation harnesses in isolated experiment copies. Preserve unrelated local edits. Rebuild only the affected components with existing caches, run the targeted reproduction and appropriate regression checks in Docker, and clean obsolete run outputs using the environment skill.

Before reporting a fix, state what changed, the evidence that the original failure is resolved, and any remaining limitations. Do not describe a missing contract or unsupported operation as verified. Changes to `x` or the build/test workflow follow the maintainer review requirement in the environment skill.
