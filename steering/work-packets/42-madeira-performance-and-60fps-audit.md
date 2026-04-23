# Packet 42 — Madeira performance and 60 FPS audit

## Task first
Audit Madeira against the 16.7 ms / 60 FPS target once enough behavior exists to make that audit meaningful.

## Objective
Audit Madeira-specific performance risk against the 16.7 ms / 60 FPS goal once enough of the recipe is operational to make the audit meaningful.

## Why this matters
A showcase recipe that is visually correct but too slow is not truly production-worthy.

## Mode
BLOCKER_MODE

## Prerequisites
- enough Madeira behavior is implemented to exercise realistic paths

## Success condition
- one evidence-backed Madeira performance audit exists
- major hot paths and likely bottlenecks are identified
- one next optimization slice is recommended if needed

## Task-scope paths for grounding
- Madeira execution path and its immediate runtime/probe surfaces
- no speculative global performance work beyond what Madeira reveals

## Exact write scope
- none by default; this is a performance audit packet
- if you need to add one small measurement helper or note, stop and justify it first

## Out of scope
- broad engine-wide optimization campaign
- unrelated performance tuning elsewhere in the repo

## Must-read docs in order
1. current Madeira operational outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Repo-boundary guardrails
- Keep the audit Madeira-specific.
- Do not generalize into whole-engine performance work from this packet.

## Verification required
- exact commands/measurements used
- clear explanation of whether the 60 FPS target looks safe, borderline, or at risk

## Reporting format
Report:
- measured or inferred hot spots
- risk level against 16.7 ms/frame
- recommended next optimization packet if needed

## Task reminder
Your task is still: audit Madeira performance readiness, not optimize the whole engine in one go.
