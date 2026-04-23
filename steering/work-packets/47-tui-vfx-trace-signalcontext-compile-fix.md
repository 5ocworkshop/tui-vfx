# Packet 47 — tui-vfx-trace SignalContext compile fix

## Task first
Fix the stale `SignalContext` initializer in `tui-vfx-trace` so the trace proof surface compiles again.

## Why this matters
The audit found that one adjacent proof surface is red for a simple reason: `SignalContext` initializers in `tui-vfx-trace` are stale after `cell_x`/`cell_y` were added.

## Success condition
- `tui-vfx-trace` builds again
- the targeted trace test or command that was previously blocked now runs
- no broad trace redesign

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/src/orc_run_trace.rs`
- `/usr/projects/tui-vfx-recipes/tools/tui-vfx-trace/tests/test_orc_run_trace.rs`
- `/usr/projects/mixed-signals/src/traits/signal.rs`

## Exact write scope
- the stale initializer seam in `orc_run_trace.rs`
- the narrowest trace test file(s) needed to prove the fix

## Out of scope
- trace feature expansion
- replay/runtime redesign
- packet/library work

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- targeted `tui-vfx-trace` compile/test command that previously failed

## Task reminder
Your task is still: repair the stale compile seam, not expand trace behavior.
