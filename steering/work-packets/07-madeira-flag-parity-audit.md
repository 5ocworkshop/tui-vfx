# Packet 07 — Madeira flag parity audit

## Task first
Audit the exact remaining gap between today’s Madeira success state and true intended V3 scene/effect parity.

## Objective
Determine the exact remaining gap between the current “loads/compiles/validator passes” state of `madeira_flag.json` and the eventual full V3 realized scene/effect parity target.

## Why this matters
Madeira is the showcase target. We should not jump into implementation without a precise gap list.

## Mode
BLOCKER_MODE

## Success condition
- a concrete gap list exists
- each gap is tied to a specific seam or file area
- one next implementation slice is suggested

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- V3 scene/compile/preview/render seams in `/usr/projects/tui-vfx-recipes/src/v3/`
- validator/probe surfaces only as evidence

## Exact write scope
- none by default; this is a blocker audit packet
- if you find a tiny broken proof command or comment that must be corrected just to complete the audit truthfully, stop and report first

## Out of scope
- implementing all Madeira gaps
- broad scene-engine redesign
- unrelated recipe cleanup

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
4. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`

## Repo-boundary guardrails
- Keep this audit in `/usr/projects/tui-vfx-recipes` proof surfaces unless the evidence clearly points lower or higher.
- Do not begin implementation from this packet.

## First steps
1. Confirm current pass state:
   - loads
   - compiles
   - validator output-stage pass
2. Identify what that does **not** guarantee.
3. Compare expected scene/effect semantics against what current bridge/native paths actually exercise.

## Verification required
- exact commands showing current pass state
- evidence-backed list of unproven or unsupported semantics

## Reporting format
Report:
- what is already proven
- what is not yet proven
- exact remaining gaps
- recommended next implementation slice

## Task reminder
Your task is still: audit the Madeira gap precisely, not solve it end-to-end in one packet.
