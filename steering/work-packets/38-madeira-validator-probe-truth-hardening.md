# Packet 38 — Madeira validator/probe truth hardening

## Task first
Make the validator/probe surfaces tell the truth about Madeira’s current support level after implementation slices land.

## Objective
Make sure the validator/probe surfaces tell the truth about Madeira’s current support level as implementation slices land.

## Why this matters
A recipe can appear “supported” too early if the diagnostic surfaces do not distinguish between parse/compile/bridge success and actual operational parity.

## Mode
BLOCKER_MODE

## Prerequisites
- at least one Madeira implementation slice has landed

## Success condition
- validator/probe surfaces accurately communicate Madeira’s current state
- no misleading pass status remains for known unimplemented semantics
- the reporting is evidence-backed

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/`
- `/usr/projects/tui-vfx-recipes/src/probe/`
- Madeira-specific proof commands/tests

## Exact write scope
- the smallest validator/probe truth-reporting seam involved
- the narrowest related tests

## Out of scope
- new Madeira runtime features unless strictly necessary for truthful reporting
- broad validator redesign

## Must-read docs in order
1. current Madeira implementation slice outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- Keep the packet on diagnostic truth surfaces only.
- Do not implement new Madeira features from this lane unless a tiny proof fix is absolutely necessary.

## Verification required
- representative Madeira validator run(s)
- representative Madeira probe run(s)
- tests for any new diagnostic wording or classification logic

## Reporting format
Report:
- prior misleading truth surface
- new truthful surface
- exact files changed
- exact commands/tests
- any remaining caveat still not expressible cleanly

## Task reminder
Your task is still: harden diagnostic truth for Madeira, not broaden feature implementation.
