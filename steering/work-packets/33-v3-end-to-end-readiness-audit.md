# Packet 33 — V3 end-to-end readiness audit

## Task first
Audit current V3 end-to-end readiness across the major surfaces and rank the top remaining blockers.

## Objective
Conduct a bounded audit of how close the current V3 pipeline is to being operational end-to-end across authoring, loading, validation, preview, probe, and representative showcase recipes.

## Why this matters
We need a periodically refreshed view of overall readiness so we know whether we are converging or just fixing isolated seams.

## Mode
BLOCKER_MODE

## Success condition
- one readiness matrix exists
- top remaining blockers are ranked
- evidence is tied to actual commands/tests, not intuition

## Task-scope paths for grounding
- all major V3 surfaces in `/usr/projects/tui-vfx-recipes`
- representative showcase checks (including Madeira where appropriate)

## Exact write scope
- none by default; this is an audit packet
- if you need a checklist artifact, keep it minimal and justify it

## Out of scope
- solving all blockers
- broad implementation

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`

## Repo-boundary guardrails
- This is a readiness audit, not an implementation packet.
- Use exact commands and evidence rather than intuition.

## Verification required
- representative commands for each major surface
- exact pass/fail/readiness notes

## Reporting format
Report a readiness matrix and the next 3 highest-value blockers.

## Task reminder
Your task is still: assess end-to-end readiness and rank blockers, not start fixing them.
