# Packet 61 — Madeira reference-repo faithfulness audit

## Task first
Audit the current Madeira V3 recipe and its rendered output against the reference implementation in `/usr/projects/madeira-flag` to determine how faithfully the recipe recreates the original animation.

## Why this matters
The current Madeira path is operational enough to render and inspect, but final confidence requires an explicit comparison against the reference repo rather than memory or indirect tests. This packet creates the evidence lane that says what matches, what is approximated, and what is still missing.

## Success condition
- one bounded evidence-backed comparison against `/usr/projects/madeira-flag` exists
- the audit distinguishes faithful recreation, approximation, and known divergence
- one next corrective slice is recommended if the recipe is not yet faithful enough

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/madeira-flag/`
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- current Madeira baseline / probe / validator / preview evidence
- current visual-vetting protocol and release-readiness checklist

## Exact write scope
- none by default; this is an audit packet
- if one tiny comparison artifact is needed, keep it narrowly scoped and explain it

## Out of scope
- broad implementation changes during the audit
- aesthetic retuning in the same packet
- non-Madeira work

## Must-read docs in order
1. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
2. `/usr/projects/tui-vfx/steering/work-packets/41-madeira-visual-vetting-protocol.md`
3. `/usr/projects/tui-vfx/steering/work-packets/43-madeira-release-readiness-checklist.md`
4. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- exact commands/evidence used to inspect the reference repo
- exact commands/evidence used to inspect the current V3 Madeira recipe/output
- a comparison matrix covering at least:
  - backdrop
  - fireworks
  - flag motion / shading / displacement intent
  - text stack / choreography
  - timing / cadence behavior

## Reporting format
Report:
- evidence bundle used
- faithful recreations
- approximations still present
- known divergences from `/usr/projects/madeira-flag`
- one single next corrective slice if not yet faithful enough

## Task reminder
Your task is still: audit faithfulness against the reference repo, not fix every divergence in the same packet.
