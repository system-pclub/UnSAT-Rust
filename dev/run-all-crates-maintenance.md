# Hourly crate maintenance

The owner requested hourly inspection, error diagnosis/repair, and implementation
of important missing existing DSL memory fields. The local tmux session
`ruklee-hourly-maintenance` waits one hour before each maintenance pass and invokes
`dev/run-all-crates-maintenance.sh`. It was started at 2026-11-19 07:37:40 UTC;
the first pass is due around 08:37:40 UTC. Each later pass starts one hour after
the preceding pass finishes. The session depends on this host and tmux staying up.

The scheduled development pass runs the installed Codex CLI with the existing
configured model and repository instructions. Verification itself still uses
`--ablation-level full --skip-llm-testcase`; its rule inputs come from the existing
step2 translations and it makes no LLM verification requests. The maintenance
agent's own reasoning is separate from that experiment.

Inspect `.local/klee-maintenance/timer.log`, `latest`, and each timestamped
directory's `status`, `agent.log`, `summary.md` and `exit-code`. An exclusive lock
prevents overlapping scheduled passes. The `manual-active` marker makes the timer
skip a pass while interactive maintenance is in progress; remove it when that
work has finished. Failed CLI/authentication runs remain visible as failed passes.

The prompt is retained in `dev/run-all-crates-maintenance-prompt.txt`. It requires
preserving DSL, reports and replay inputs, stopping workers before rebuilding
KLEE, Docker validation with source crates, accurate TODO updates, and resuming
from the interrupted crate. It must not create recursive timers or subagents.

Stop future passes with `tmux kill-session -t ruklee-hourly-maintenance`.
An already running pass can be inspected with `tmux attach -t ruklee-hourly-maintenance`.
Do not terminate unrelated tmux sessions or delete preserved experiment results.
