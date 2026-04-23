# Packet 12 — native replay hot-path audit

## Objective
Identify the next meaningful performance cleanup opportunity in the compiled/native replay seam after the elapsed-time normalization work.

## Why this matters
We improved timing correctness. The next step is to spot the highest-value hot-path inefficiencies without speculative premature optimization.

## Mode
BLOCKER_MODE

## Success condition
- one evidence-backed hot-path issue is identified
- it is ranked by likely payoff and risk
- exact files/functions are named for a later fix packet

## In scope
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_apply_compiled_pipeline_replay_to_scene.rs`
- nearby deterministic/native replay seams if needed for context

## Out of scope
- implementing the optimization
- speculative math extraction into `mixed-signals` unless the 3+ use-case rule is clearly satisfied
- broad render-pipeline redesign

## What to inspect
- repeated context rebuilding
- repeated per-cell cloning/allocation
- recomputation that can be hoisted once per replay/sample
- elapsed-time vs normalized-time misuse that may also cost work

## Verification required
- evidence from exact code locations
- explain why the issue is likely hot-path relevant
- suggest the exact proof command or benchmark/test to use in a future implementation lane

## Reporting format
Report:
- top hot-path issue
- exact file/function/line area
- likely optimization shape
- proof approach for next implementation packet
- boundary note about whether it belongs in `mixed-signals` or not

## Task reminder
Your task is still: audit and rank the next hot-path issue, not implement performance changes.
