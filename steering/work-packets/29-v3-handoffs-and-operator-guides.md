# Packet 29 — V3 handoffs and operator guides

## Task first
Prepare one concise operator-facing handoff/guide artifact that helps humans or agents resume V3 work after crashes or context resets.

## Objective
Prepare concise operator-facing handoff notes for the V3 migration/tooling state so future humans or agents can resume quickly after crashes or context resets.

## Why this matters
We have already seen repeated crashes/resets. High-quality handoffs reduce restart cost.

## Mode
BLOCKER_MODE

## Success condition
- one durable handoff artifact exists or is improved
- it captures current truth, not stale plan text
- it points to canonical docs/commands

## Task-scope paths for grounding
- handoff docs in `docs/`, `.omx/context/`, or `steering/` as appropriate

## Exact write scope
- the single chosen handoff/operator guide artifact
- the smallest pointer/update nearby if needed

## Out of scope
- runtime code
- broad strategy changes

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. current handoff/context artifacts
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- Keep this lane on handoff/operator guidance only.
- Do not use it to rewrite general strategy unless the artifact directly requires a minimal correction.

## Verification required
- human-readable spot check: can a new reader understand current state, next blockers, and proof commands?

## Reporting format
Report the artifact path and what recovery/use case it supports.
