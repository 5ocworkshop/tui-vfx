# Packet 40 — Madeira end-to-end operational check

## Task first
Run one bounded end-to-end Madeira operational check and identify the next blocker if it is not yet truly operational.

## Objective
Run a bounded end-to-end operational check for Madeira across authoring, loading, validation, deterministic rendering, and any current probe/preview surfaces.

## Why this matters
After several slices, we need a concrete checkpoint that says what Madeira can actually do end-to-end today.

## Mode
BLOCKER_MODE

## Prerequisites
- relevant Madeira implementation slices complete
- validator/probe truth surfaces reasonably current

## Success condition
- one end-to-end checklist run is performed
- every stage is marked pass/fail/unproven with evidence
- one next blocker is identified if not yet fully operational

## Task-scope paths for grounding
- Madeira recipe
- current supporting loader/validator/probe/deterministic-render surfaces
- no broad new implementation unless a trivial proof fix is necessary

## Exact write scope
- none by default; this is an operational audit packet
- if a tiny proof fix is required, stop and report before widening

## Out of scope
- fixing every discovered issue in the same packet
- broad docs rewrite

## Must-read docs in order
1. current Madeira implementation and baseline outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`

## Repo-boundary guardrails
- This is an operational check, not a grab-bag implementation packet.
- Gather evidence first; do not start fixing everything you find.

## Verification required
At minimum, gather fresh evidence for:
- load/parse
- normalize/validate/compile
- validator output truth
- deterministic render/probe
- any Madeira-specific baseline tests already created

## Reporting format
Report an end-to-end matrix with:
- stage
- command
- result
- evidence
- next blocker if failed or degraded

## Task reminder
Your task is still: assess end-to-end operational state, not immediately fix everything discovered.
