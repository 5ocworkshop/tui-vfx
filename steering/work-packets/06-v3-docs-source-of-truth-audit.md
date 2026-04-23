# Packet 06 — V3 docs source-of-truth audit

## Task first
Audit the V3 docs/schema source-of-truth surfaces and identify one concrete next cleanup lane without fixing everything.

## Objective
Audit the remaining V3 schema/docs surfaces and identify where the source of truth is still ambiguous, stale, duplicated, or misleading.

## Why this matters
Generated docs freshness passing once is good, but we still need to know whether any parallel handwritten/schema surfaces remain likely to drift.

## Mode
BLOCKER_MODE

## Success condition
- one clear audit report of which V3 docs/surfaces are canonical
- identified stale or duplicate sources
- one recommended next cleanup lane, not a giant rewrite

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/`
- `/usr/projects/tui-vfx-recipes/docs/generated/`
- V3 schema/export helpers
- any doc generator entrypoints directly involved

## Exact write scope
- none by default; this is a read-heavy audit packet
- if you discover a tiny clarification edit is absolutely necessary for the audit to be truthful, stop and report before making it

## Out of scope
- runtime changes
- broad docs rewrite across all repos
- recipe content

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- Keep the audit in `/usr/projects/tui-vfx-recipes`.
- This packet is for identifying truth boundaries, not editing runtime behavior.

## Deliverable shape
Produce a shortlist of surfaces grouped by:
- canonical
- derived/generated
- stale/risky duplicate
- unclear ownership

## Verification required
- evidence from file inspection
- generator/test commands if relevant
- no speculative claims without a cited file basis

## Reporting format
Report:
- canonical surfaces
- risky duplicates
- recommended next cleanup packet
- exact files inspected

## Task reminder
Your task is still: audit source-of-truth boundaries for V3 docs, not fix every docs issue in one pass.
