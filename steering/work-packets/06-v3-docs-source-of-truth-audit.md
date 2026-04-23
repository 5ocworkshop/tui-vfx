# Packet 06 — V3 docs source-of-truth audit

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

## In scope
- `/usr/projects/tui-vfx-recipes/src/v3/`
- `/usr/projects/tui-vfx-recipes/docs/generated/`
- V3 schema/export helpers
- any doc generator entrypoints directly involved

## Out of scope
- runtime changes
- broad docs rewrite across all repos
- recipe content

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
