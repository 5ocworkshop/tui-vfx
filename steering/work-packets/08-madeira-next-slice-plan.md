# Packet 08 — Madeira next-slice plan

## Task first
Choose the single best next Madeira implementation slice from the audit results and define it concretely.

## Objective
Turn the Madeira parity audit into one narrow next implementation slice that can be executed safely.

## Why this matters
The audit should feed a concrete next step, not a vague promise of future parity.

## Mode
BLOCKER_MODE

## Success condition
- one next slice is chosen
- exact files to touch are named
- exact proof commands are named
- risks are explicit

## Task-scope paths for grounding
- Madeira audit results
- the exact seam identified by the audit

## Exact write scope
- none by default; this is a planning packet
- if you need to add one planning artifact or note, keep it narrowly scoped and state why

## Out of scope
- implementing the slice itself
- widening into multiple Madeira gaps at once

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. relevant Madeira audit result(s)
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- This is slice planning, not execution.
- Choose one slice only; do not create a laundry list of loosely ranked tasks.

## Deliverable
Produce:
- slice name
- exact seam
- exact files
- why this slice is first
- what commands prove it when done

## Reporting format
Be concrete. If more than one slice seems possible, explain why the chosen one is best first.

## Task reminder
Your task is still: pick the next slice, not perform Madeira implementation.
