# Safe4U experiment summary

The experiments use the Rust 1.83-compatible crate copies under
`benchmark/safe4u`.

| Crate | Candidates / context | Evaluated | GPT-4o S/U/? | GPT-5.4-mini S/U/? |
| --- | ---: | ---: | ---: | ---: |
| bitm-0.4.2 | 5 / 5 | 3 | 1 / 2 / 0 | 1 / 2 / 0 |
| jh-0.1.0 | 3 / 3 | 1 | 0 / 1 / 0 | 0 / 1 / 0 |
| revm-interpreter-10.0.3 | 30 / 30 | 25 | 17 / 8 / 0 | 13 / 12 / 0 |
| i_triangle-0.26.0 | 7 / 7 | 7 | 4 / 3 / 0 | 0 / 7 / 0 |
| aegis-0.9.12 | 102 / 102 | 13 | 13 / 0 / 0 | 13 / 0 / 0 |
| jomini-0.27.0 | 17 / 17 | 14 | 10 / 4 / 0 | 8 / 6 / 0 |
| i_tree-0.9.0 | 2 / 2 | 2 | 0 / 2 / 0 | 0 / 2 / 0 |
| splay_tree-0.3.1 | 3 / 3 | 2 | 0 / 2 / 0 | 0 / 2 / 0 |
| hugepage-rs-0.1.0 | 3 / 3 | 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| **Total** | **172 / 172** | **67** | **45 / 22 / 0** | **35 / 32 / 0** |

`S/U/?` means sound, unsound, and unknown. `hugepage-rs` produced three
candidates but no safety constraints to evaluate, so it made no model
requests.

## Runs

- `gpt-4o-2024-05-13`: `eval/safe4u-gpt4o`
  - 680 requests, 2,510,232 tokens
- `gpt-5.4-mini-2026-03-17`: `eval/safe4u-gpt5mini`
  - 600 requests, 2,154,849 tokens

Each crate directory contains candidate items, retrieved context,
`samples_with_constraints.json`, `results.json`, and `run.log`.
