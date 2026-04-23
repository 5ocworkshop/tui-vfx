# Packet 43 — Madeira release readiness checklist

## Task first
Assemble the concrete release-readiness checklist for Madeira using evidence from the operational, visual, performance, and baseline work.

## Objective
Assemble the concrete release-readiness checklist for Madeira once it approaches operational parity.

## Why this matters
We need a final go/no-go checklist that combines correctness, tooling truth, regression anchors, visual vetting, and performance.

## Mode
BLOCKER_MODE

## Prerequisites
- multiple Madeira implementation slices complete
- operational and visual/performance audits available

## Success condition
- a checklist exists with concrete proof items
- each item has a clear pass/fail condition
- must-haves are separated from nice-to-haves

## Task-scope paths for grounding
- Madeira-related proof surfaces and docs only

## Exact write scope
- the checklist artifact only
- the smallest supporting note/reference if needed

## Out of scope
- implementing checklist items in this packet
- broad feature work

## Must-read docs in order
1. current Madeira operational/visual/performance audit outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- Keep this packet on readiness criteria only.
- Separate must-haves from nice-to-haves explicitly.

## Verification required
- checklist items must be tied to exact commands, artifacts, or human review steps

## Reporting format
Report:
- must-have checklist
- nice-to-have checklist
- current pass/fail status if already known

## Task reminder
Your task is still: prepare Madeira release-readiness criteria, not ship it in the same packet.
